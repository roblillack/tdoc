use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
        MouseButton, MouseEvent, MouseEventKind,
    },
    execute, queue,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::collections::HashMap;
use std::io::{self, Stdout, Write};
use std::ops::Range;
use std::sync::Arc;
use unicode_width::UnicodeWidthChar;

/// ANSI-aware segment ready for rendering.
#[derive(Clone, Debug)]
struct ParsedLineSegment {
    text: String,
    range: Range<usize>,
    style: AnsiStyle,
    hyperlink: Option<String>,
}

#[derive(Clone, Debug)]
struct ParsedLine {
    segments: Vec<ParsedLineSegment>,
    plain: String,
}

#[derive(Clone, Debug, Default)]
struct TextAttributes {
    bold: bool,
    dim: bool,
    italic: bool,
    underlined: bool,
    slow_blink: bool,
    rapid_blink: bool,
    reversed: bool,
    hidden: bool,
    crossed_out: bool,
}

impl TextAttributes {
    fn reset(&mut self) {
        *self = TextAttributes::default();
    }

    fn attribute_list(&self) -> impl Iterator<Item = Attribute> {
        let mut attrs = Vec::new();
        if self.bold {
            attrs.push(Attribute::Bold);
        }
        if self.dim {
            attrs.push(Attribute::Dim);
        }
        if self.italic {
            attrs.push(Attribute::Italic);
        }
        if self.underlined {
            attrs.push(Attribute::Underlined);
        }
        if self.slow_blink {
            attrs.push(Attribute::SlowBlink);
        }
        if self.rapid_blink {
            attrs.push(Attribute::RapidBlink);
        }
        if self.reversed {
            attrs.push(Attribute::Reverse);
        }
        if self.hidden {
            attrs.push(Attribute::Hidden);
        }
        if self.crossed_out {
            attrs.push(Attribute::CrossedOut);
        }
        attrs.into_iter()
    }
}

#[derive(Clone, Debug, Default)]
struct AnsiStyleState {
    fg: Option<Color>,
    bg: Option<Color>,
    attributes: TextAttributes,
    hyperlink: Option<String>,
}

impl AnsiStyleState {
    fn reset(&mut self) {
        self.fg = None;
        self.bg = None;
        self.attributes.reset();
    }

    fn to_style(&self) -> AnsiStyle {
        AnsiStyle {
            fg: self.fg,
            bg: self.bg,
            attributes: self.attributes.clone(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct AnsiStyle {
    fg: Option<Color>,
    bg: Option<Color>,
    attributes: TextAttributes,
}

impl AnsiStyle {
    fn with_highlight(&self, fg: Color, bg: Color, emphasize: bool) -> Self {
        let mut style = self.clone();
        style.fg = Some(fg);
        style.bg = Some(bg);
        if emphasize {
            style.attributes.bold = true;
        }
        style
    }

    fn with_link_style(&self, focused: bool) -> Self {
        let mut style = self.clone();
        style.attributes.underlined = true;
        if focused {
            style.fg = Some(Color::White);
            style.bg = Some(Color::Blue);
        } else {
            style.fg = Some(Color::Blue);
            style.bg = None;
        }
        style
    }

    fn apply(&self, stdout: &mut Stdout) -> io::Result<()> {
        queue!(
            stdout,
            SetAttribute(Attribute::Reset),
            ResetColor,
            SetForegroundColor(self.fg.unwrap_or(Color::Reset)),
            SetBackgroundColor(self.bg.unwrap_or(Color::Reset))
        )?;

        for attr in self.attributes.attribute_list() {
            queue!(stdout, SetAttribute(attr))?;
        }

        Ok(())
    }
}

#[derive(Clone)]
struct RenderChunk {
    text: String,
    style: AnsiStyle,
    hyperlink: Option<String>,
}

#[derive(Clone)]
struct SearchMatch {
    line_idx: usize,
    start: usize,
    end: usize,
}

#[derive(Clone)]
enum SearchMode {
    Normal,
    EnteringQuery,
    Active {
        query: String,
        matches: Vec<SearchMatch>,
        current_match: usize,
    },
}

#[derive(Clone)]
struct LinkInfo {
    url: String,
    line_idx: usize,
    start_char: usize,
    end_char: usize,
    start_col: usize,
    end_col: usize,
}

#[derive(Clone)]
pub struct PagerOptions {
    pub enable_mouse_capture: bool,
    pub link_callback: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl Default for PagerOptions {
    fn default() -> Self {
        Self {
            enable_mouse_capture: true,
            link_callback: Some(default_link_callback()),
        }
    }
}

fn default_link_callback() -> Arc<dyn Fn(&str) + Send + Sync> {
    Arc::new(|target: &str| {
        println!("{}", target);
        std::process::exit(0);
    })
}

struct PagerState {
    scroll_offset: usize,
    total_lines: usize,
    viewport_height: usize,
    search_mode: SearchMode,
    search_input: String,
    last_terminal_height: usize,
    links: Vec<LinkInfo>,
    focused_link: Option<usize>,
}

impl PagerState {
    fn new(total_lines: usize, viewport_height: usize) -> Self {
        Self {
            scroll_offset: 0,
            total_lines,
            viewport_height,
            search_mode: SearchMode::Normal,
            search_input: String::new(),
            last_terminal_height: 0,
            links: Vec::new(),
            focused_link: None,
        }
    }

    fn update_viewport_height(&mut self, height: usize) {
        self.viewport_height = height;
        self.clamp_scroll();
    }

    fn max_scroll(&self) -> usize {
        if self.viewport_height == 0 {
            self.total_lines.saturating_sub(1)
        } else {
            self.total_lines.saturating_sub(self.viewport_height)
        }
    }

    fn clamp_scroll(&mut self) {
        let max_scroll = self.max_scroll();
        if self.scroll_offset > max_scroll {
            self.scroll_offset = max_scroll;
        }
    }

    fn scroll_down(&mut self) {
        if self.scroll_offset < self.max_scroll() {
            self.scroll_offset += 1;
        }
    }

    fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    fn page_down(&mut self) {
        let max_scroll = self.max_scroll();
        self.scroll_offset = (self.scroll_offset + self.viewport_height).min(max_scroll);
    }

    fn page_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(self.viewport_height);
    }

    fn jump_to_start(&mut self) {
        self.scroll_offset = 0;
    }

    fn jump_to_end(&mut self) {
        self.scroll_offset = self.max_scroll();
    }

    fn start_search(&mut self) {
        self.search_mode = SearchMode::EnteringQuery;
        self.search_input.clear();
    }

    fn perform_search(&mut self, content: &[ParsedLine]) {
        if self.search_input.is_empty() {
            self.search_mode = SearchMode::Normal;
            return;
        }

        let query = self.search_input.clone();
        let matches = find_search_matches(&query, content);

        if matches.is_empty() {
            self.search_mode = SearchMode::Normal;
        } else {
            let first_line = matches[0].line_idx;
            self.scroll_offset = first_line.saturating_sub(self.viewport_height / 2);
            self.clamp_scroll();
            self.search_mode = SearchMode::Active {
                query,
                matches,
                current_match: 0,
            };
        }
    }

    fn rebuild_search_results(&mut self, content: &[ParsedLine], target_line: Option<usize>) {
        let mut reset_to_normal = false;

        if let SearchMode::Active {
            query,
            matches,
            current_match,
            ..
        } = &mut self.search_mode
        {
            let new_matches = find_search_matches(query, content);
            if new_matches.is_empty() {
                reset_to_normal = true;
            } else {
                let desired_index = target_line.and_then(|line| {
                    new_matches
                        .iter()
                        .enumerate()
                        .min_by_key(|(_, m)| m.line_idx.abs_diff(line))
                        .map(|(idx, _)| idx)
                });

                let new_index = desired_index
                    .unwrap_or_else(|| (*current_match).min(new_matches.len().saturating_sub(1)));
                *current_match = new_index;
                *matches = new_matches;
            }
        }

        if reset_to_normal {
            self.search_mode = SearchMode::Normal;
        }
    }

    fn rebuild_links(&mut self, content: &[ParsedLine]) {
        let previous = self
            .focused_link
            .and_then(|idx| self.links.get(idx).cloned());
        self.links = collect_links(content);
        if let Some(prev_link) = previous {
            self.focused_link = self.links.iter().position(|link| {
                link.url == prev_link.url
                    && link.line_idx == prev_link.line_idx
                    && link.start_char == prev_link.start_char
            });
        } else {
            self.focused_link = None;
        }
        if let Some(idx) = self.focused_link {
            if idx >= self.links.len() {
                self.focused_link = None;
            }
        }
    }

    fn focus_next_link(&mut self) -> bool {
        if self.links.is_empty() {
            return false;
        }
        let next_index = match self.focused_link {
            Some(idx) => (idx + 1) % self.links.len(),
            None => 0,
        };
        let changed = self.focused_link != Some(next_index);
        self.focused_link = Some(next_index);
        self.ensure_link_visible(next_index);
        changed
    }

    fn focus_prev_link(&mut self) -> bool {
        if self.links.is_empty() {
            return false;
        }
        let prev_index = match self.focused_link {
            Some(0) => self.links.len().saturating_sub(1),
            Some(idx) => idx.saturating_sub(1),
            None => self.links.len().saturating_sub(1),
        };
        let changed = self.focused_link != Some(prev_index);
        self.focused_link = Some(prev_index);
        self.ensure_link_visible(prev_index);
        changed
    }

    fn ensure_link_visible(&mut self, index: usize) {
        if let Some(link) = self.links.get(index) {
            if link.line_idx < self.scroll_offset {
                self.scroll_offset = link.line_idx;
            } else if self.viewport_height > 0
                && link.line_idx >= self.scroll_offset + self.viewport_height
            {
                let desired = link
                    .line_idx
                    .saturating_sub(self.viewport_height.saturating_sub(1));
                self.scroll_offset = desired;
            }
            self.clamp_scroll();
        }
    }

    fn focus_link_at(&mut self, line_idx: usize, column: usize) -> Option<usize> {
        if let Some(idx) = self.links.iter().position(|link| {
            link.line_idx == line_idx
                && column >= link.start_col
                && column < link.end_col.max(link.start_col + 1)
        }) {
            let changed = self.focused_link != Some(idx);
            self.focused_link = Some(idx);
            if changed {
                self.ensure_link_visible(idx);
            }
            Some(idx)
        } else {
            None
        }
    }

    fn focused_link(&self) -> Option<&LinkInfo> {
        self.focused_link.and_then(|idx| self.links.get(idx))
    }

    fn current_link_target(&self) -> Option<&str> {
        self.focused_link().map(|link| link.url.as_str())
    }

    fn next_match(&mut self) {
        if let SearchMode::Active {
            matches,
            current_match,
            ..
        } = &mut self.search_mode
        {
            if matches.is_empty() {
                return;
            }
            *current_match = (*current_match + 1) % matches.len();
            let line = matches[*current_match].line_idx;
            self.scroll_offset = line.saturating_sub(self.viewport_height / 2);
            self.clamp_scroll();
        }
    }

    fn prev_match(&mut self) {
        if let SearchMode::Active {
            matches,
            current_match,
            ..
        } = &mut self.search_mode
        {
            if matches.is_empty() {
                return;
            }
            *current_match = if *current_match == 0 {
                matches.len() - 1
            } else {
                *current_match - 1
            };
            let line = matches[*current_match].line_idx;
            self.scroll_offset = line.saturating_sub(self.viewport_height / 2);
            self.clamp_scroll();
        }
    }

    fn clear_search(&mut self) {
        self.search_mode = SearchMode::Normal;
        self.search_input.clear();
    }
}

fn find_search_matches(query: &str, content: &[ParsedLine]) -> Vec<SearchMatch> {
    if query.is_empty() {
        return Vec::new();
    }

    let query_len = query.len();
    let query_lower = query.to_lowercase();
    let mut matches = Vec::new();

    for (line_idx, line) in content.iter().enumerate() {
        let line_lower = line.plain.to_lowercase();
        let mut start = 0;
        while let Some(pos) = line_lower[start..].find(&query_lower) {
            let match_start = start + pos;
            matches.push(SearchMatch {
                line_idx,
                start: match_start,
                end: match_start + query_len,
            });
            start += pos + 1;
        }
    }

    matches
}

fn collect_links(content: &[ParsedLine]) -> Vec<LinkInfo> {
    let mut links = Vec::new();

    for (line_idx, line) in content.iter().enumerate() {
        let mut current: Option<LinkInfo> = None;
        let mut char_index = 0usize;
        let mut col_index = 0usize;

        for segment in &line.segments {
            for ch in segment.text.chars() {
                let width = UnicodeWidthChar::width(ch).unwrap_or(0);

                if let Some(url) = &segment.hyperlink {
                    let is_continuation = current
                        .as_ref()
                        .map(|link| {
                            link.url == *url
                                && link.line_idx == line_idx
                                && link.end_char == char_index
                        })
                        .unwrap_or(false);

                    if !is_continuation {
                        if let Some(mut link) = current.take() {
                            if link.end_col == link.start_col {
                                link.end_col = link.start_col + 1;
                            }
                            links.push(link);
                        }
                        current = Some(LinkInfo {
                            url: url.clone(),
                            line_idx,
                            start_char: char_index,
                            end_char: char_index,
                            start_col: col_index,
                            end_col: col_index,
                        });
                    }

                    if let Some(link) = current.as_mut() {
                        link.end_char = char_index + 1;
                        if width > 0 {
                            link.end_col = col_index + width;
                        } else if link.end_col == link.start_col {
                            link.end_col = link.start_col + 1;
                        }
                    }
                } else if let Some(mut link) = current.take() {
                    if link.end_col == link.start_col {
                        link.end_col = link.start_col + 1;
                    }
                    links.push(link);
                }

                col_index += width;
                char_index += 1;
            }

            if segment.hyperlink.is_none() {
                if let Some(mut link) = current.take() {
                    if link.end_col == link.start_col {
                        link.end_col = link.start_col + 1;
                    }
                    links.push(link);
                }
            }
        }

        if let Some(mut link) = current.take() {
            if link.end_col == link.start_col {
                link.end_col = link.start_col + 1;
            }
            links.push(link);
        }
    }

    links
}

impl ParsedLine {
    fn from_ansi(line: &str) -> Self {
        let mut plain = String::new();
        let mut segments = Vec::new();
        let mut current_text = String::new();
        let mut style_state = AnsiStyleState::default();
        let mut current_style = style_state.to_style();
        let mut segment_start = 0usize;

        let bytes = line.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'\x1b' => {
                    flush_segment(
                        &mut segments,
                        &mut current_text,
                        &current_style,
                        &mut segment_start,
                        plain.len(),
                        style_state.hyperlink.clone(),
                    );
                    i += 1;
                    if i >= bytes.len() {
                        break;
                    }
                    match bytes[i] {
                        b'[' => {
                            i += 1;
                            i += parse_csi_sequence(line, i, &mut style_state);
                            current_style = style_state.to_style();
                        }
                        b']' => {
                            i += 1;
                            i += parse_osc_sequence(line, i, &mut style_state);
                            current_style = style_state.to_style();
                        }
                        _ => {
                            i += 1;
                        }
                    }
                }
                b'\r' | b'\x07' => {
                    i += 1;
                }
                _ => {
                    if current_text.is_empty() {
                        segment_start = plain.len();
                    }
                    let ch = line[i..].chars().next().unwrap();
                    let len = ch.len_utf8();
                    current_text.push(ch);
                    plain.push(ch);
                    i += len;
                }
            }
        }

        flush_segment(
            &mut segments,
            &mut current_text,
            &current_style,
            &mut segment_start,
            plain.len(),
            style_state.hyperlink,
        );

        Self { segments, plain }
    }

    fn to_render_chunks(&self, highlights: &[(usize, usize, bool)]) -> Vec<RenderChunk> {
        let mut chunks = Vec::new();
        let mut highlight_iter = highlights.iter().cloned().peekable();

        for segment in &self.segments {
            let mut cursor = segment.range.start;
            while cursor < segment.range.end {
                let (end, style) =
                    if let Some(&(hl_start, hl_end, is_current)) = highlight_iter.peek() {
                        if hl_end <= cursor {
                            highlight_iter.next();
                            continue;
                        }

                        if hl_start > cursor {
                            (hl_start.min(segment.range.end), segment.style.clone())
                        } else {
                            let end = hl_end.min(segment.range.end);
                            let highlight_style = if is_current {
                                segment
                                    .style
                                    .with_highlight(Color::Black, Color::Yellow, true)
                            } else {
                                segment
                                    .style
                                    .with_highlight(Color::Black, Color::Cyan, false)
                            };
                            if end >= hl_end {
                                highlight_iter.next();
                            }
                            (end, highlight_style)
                        }
                    } else {
                        (segment.range.end, segment.style.clone())
                    };

                if cursor >= end {
                    continue;
                }

                let rel_start = cursor - segment.range.start;
                let rel_end = end - segment.range.start;
                let slice = segment.text[rel_start..rel_end].to_string();
                if slice.is_empty() {
                    cursor = end;
                    continue;
                }

                chunks.push(RenderChunk {
                    text: slice,
                    style: style.clone(),
                    hyperlink: segment.hyperlink.clone(),
                });
                cursor = end;
            }
        }

        chunks
    }
}

fn flush_segment(
    segments: &mut Vec<ParsedLineSegment>,
    current_text: &mut String,
    current_style: &AnsiStyle,
    segment_start: &mut usize,
    plain_len: usize,
    hyperlink: Option<String>,
) {
    if current_text.is_empty() {
        return;
    }

    let text = std::mem::take(current_text);
    let start = *segment_start;
    let end = start + text.len();
    segments.push(ParsedLineSegment {
        text,
        range: start..end,
        style: current_style.clone(),
        hyperlink,
    });
    *segment_start = plain_len;
}

fn parse_csi_sequence(line: &str, start: usize, style_state: &mut AnsiStyleState) -> usize {
    let bytes = line.as_bytes();
    let mut i = start;
    while i < bytes.len() {
        let b = bytes[i];
        if (0x40..=0x7e).contains(&b) {
            if b == b'm' {
                apply_sgr(&line[start..i], style_state);
            }
            return i + 1 - start;
        }
        i += 1;
    }
    bytes.len().saturating_sub(start)
}

fn parse_osc_sequence(line: &str, start: usize, style_state: &mut AnsiStyleState) -> usize {
    let bytes = line.as_bytes();
    let mut i = start;
    while i < bytes.len() {
        match bytes[i] {
            b'\x07' => {
                apply_osc(&line[start..i], style_state);
                return i + 1 - start;
            }
            b'\x1b' if i + 1 < bytes.len() && bytes[i + 1] == b'\\' => {
                apply_osc(&line[start..i], style_state);
                return i + 2 - start;
            }
            _ => {}
        }
        i += 1;
    }
    apply_osc(&line[start..], style_state);
    bytes.len().saturating_sub(start)
}

fn apply_osc(content: &str, style_state: &mut AnsiStyleState) {
    if let Some(rest) = content.strip_prefix('8') {
        let rest = rest.strip_prefix(';').unwrap_or(rest);
        let mut parts = rest.splitn(2, ';');
        let _params = parts.next();
        if let Some(url) = parts.next() {
            if url.is_empty() {
                style_state.hyperlink = None;
            } else {
                style_state.hyperlink = Some(url.to_string());
            }
        }
    }
}

fn apply_sgr(params: &str, style_state: &mut AnsiStyleState) {
    let mut numbers: Vec<i64> = if params.is_empty() {
        vec![0]
    } else {
        params
            .split(';')
            .filter_map(|part| part.parse::<i64>().ok())
            .collect()
    };

    if numbers.is_empty() {
        numbers.push(0);
    }

    let mut iter = numbers.into_iter();
    while let Some(code) = iter.next() {
        match code {
            0 => style_state.reset(),
            1 | 21 => style_state.attributes.bold = true,
            2 => style_state.attributes.dim = true,
            3 => style_state.attributes.italic = true,
            4 => style_state.attributes.underlined = true,
            5 => style_state.attributes.slow_blink = true,
            6 => style_state.attributes.rapid_blink = true,
            7 => style_state.attributes.reversed = true,
            8 => style_state.attributes.hidden = true,
            9 => style_state.attributes.crossed_out = true,
            22 => {
                style_state.attributes.bold = false;
                style_state.attributes.dim = false;
            }
            23 => style_state.attributes.italic = false,
            24 => style_state.attributes.underlined = false,
            25 => {
                style_state.attributes.slow_blink = false;
                style_state.attributes.rapid_blink = false;
            }
            27 => style_state.attributes.reversed = false,
            28 => style_state.attributes.hidden = false,
            29 => style_state.attributes.crossed_out = false,
            30..=37 => style_state.fg = Some(map_basic_color((code - 30) as u8, false)),
            38 => apply_extended_color(&mut iter, style_state, true),
            39 => style_state.fg = None,
            40..=47 => style_state.bg = Some(map_basic_color((code - 40) as u8, false)),
            48 => apply_extended_color(&mut iter, style_state, false),
            49 => style_state.bg = None,
            90..=97 => style_state.fg = Some(map_basic_color((code - 90) as u8, true)),
            100..=107 => style_state.bg = Some(map_basic_color((code - 100) as u8, true)),
            _ => {}
        }
    }
}

fn apply_extended_color(
    iter: &mut impl Iterator<Item = i64>,
    style_state: &mut AnsiStyleState,
    is_fg: bool,
) {
    match iter.next() {
        Some(5) => {
            if let Some(value) = iter.next() {
                let color = Color::AnsiValue(value as u8);
                if is_fg {
                    style_state.fg = Some(color);
                } else {
                    style_state.bg = Some(color);
                }
            }
        }
        Some(2) => {
            let r = iter.next().unwrap_or(0).clamp(0, 255) as u8;
            let g = iter.next().unwrap_or(0).clamp(0, 255) as u8;
            let b = iter.next().unwrap_or(0).clamp(0, 255) as u8;
            let color = Color::Rgb { r, g, b };
            if is_fg {
                style_state.fg = Some(color);
            } else {
                style_state.bg = Some(color);
            }
        }
        _ => {}
    }
}

fn map_basic_color(index: u8, bright: bool) -> Color {
    match (index, bright) {
        (0, false) => Color::Black,
        (1, false) => Color::DarkRed,
        (2, false) => Color::DarkGreen,
        (3, false) => Color::DarkYellow,
        (4, false) => Color::DarkBlue,
        (5, false) => Color::DarkMagenta,
        (6, false) => Color::DarkCyan,
        (7, false) => Color::Grey,
        (0, true) => Color::DarkGrey,
        (1, true) => Color::Red,
        (2, true) => Color::Green,
        (3, true) => Color::Yellow,
        (4, true) => Color::Blue,
        (5, true) => Color::Magenta,
        (6, true) => Color::Cyan,
        (7, true) => Color::White,
        _ => Color::Reset,
    }
}

fn render_pager(
    stdout: &mut Stdout,
    content: &[ParsedLine],
    state: &mut PagerState,
) -> io::Result<()> {
    let (terminal_width, terminal_height) = terminal::size()?;
    if terminal_height == 0 {
        return Ok(());
    }

    let terminal_height_usize = terminal_height as usize;
    let previous_height = state.last_terminal_height;
    state.last_terminal_height = terminal_height_usize;

    state.total_lines = content.len();

    state.update_viewport_height(terminal_height_usize.saturating_sub(1));
    let content_width = terminal_width.saturating_sub(1) as usize;

    let mut highlight_map: HashMap<usize, Vec<(usize, usize, bool)>> = HashMap::new();
    if let SearchMode::Active {
        matches,
        current_match,
        ..
    } = &state.search_mode
    {
        for (idx, search_match) in matches.iter().enumerate() {
            if search_match.line_idx >= state.scroll_offset
                && search_match.line_idx < state.scroll_offset + state.viewport_height
            {
                highlight_map
                    .entry(search_match.line_idx)
                    .or_default()
                    .push((search_match.start, search_match.end, idx == *current_match));
            }
        }

        for ranges in highlight_map.values_mut() {
            ranges.sort_by_key(|(start, _, _)| *start);
        }
    }

    for row in 0..state.viewport_height {
        let line_idx = state.scroll_offset + row;
        queue!(stdout, MoveTo(0, row as u16), Clear(ClearType::CurrentLine))?;
        if let Some(line) = content.get(line_idx) {
            let highlights = highlight_map.get(&line_idx).cloned().unwrap_or_default();
            let focused_link = state
                .focused_link()
                .filter(|link| link.line_idx == line_idx);
            render_line(stdout, line, &highlights, content_width, focused_link)?;
        }
    }

    if state.total_lines > state.viewport_height && state.viewport_height > 0 {
        draw_scrollbar(
            stdout,
            state.scroll_offset,
            state.total_lines,
            state.viewport_height,
            terminal_width.saturating_sub(1),
        )?;
    }

    let status_row = state.viewport_height as u16;
    draw_status_line(stdout, state, terminal_width, status_row)?;

    if previous_height > terminal_height_usize {
        for row in terminal_height_usize..previous_height {
            queue!(stdout, MoveTo(0, row as u16), Clear(ClearType::CurrentLine))?;
        }
    }

    stdout.flush()
}

fn render_line(
    stdout: &mut Stdout,
    line: &ParsedLine,
    highlights: &[(usize, usize, bool)],
    width: usize,
    focused_link: Option<&LinkInfo>,
) -> io::Result<()> {
    if width == 0 {
        return Ok(());
    }

    let chunks = line.to_render_chunks(highlights);
    let mut remaining = width;
    let mut char_cursor = 0usize;

    for chunk in chunks {
        if remaining == 0 {
            break;
        }

        let (render_text, used_width, complete) = clip_to_width(chunk.text.as_str(), remaining);

        if render_text.is_empty() && used_width == 0 && !complete {
            break;
        }

        let render_char_count = render_text.chars().count();
        let chunk_start_char = char_cursor;
        let chunk_end_char = char_cursor + render_char_count;

        let mut style = chunk.style.clone();
        if chunk.hyperlink.is_some() {
            let is_focused = focused_link
                .map(|link| link.start_char < chunk_end_char && link.end_char > chunk_start_char)
                .unwrap_or(false);
            style = style.with_link_style(is_focused);
        }

        style.apply(stdout)?;
        queue!(stdout, Print(render_text.as_str()))?;

        char_cursor = chunk_end_char;

        if used_width >= remaining || !complete {
            break;
        }

        remaining = remaining.saturating_sub(used_width);
    }

    queue!(stdout, SetAttribute(Attribute::Reset), ResetColor)?;
    Ok(())
}

fn clip_to_width(text: &str, max_width: usize) -> (String, usize, bool) {
    if max_width == 0 {
        return (String::new(), 0, false);
    }

    if text.is_empty() {
        return (String::new(), 0, true);
    }

    let mut width = 0usize;
    let mut end = 0usize;
    for (idx, ch) in text.char_indices() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);
        if width + ch_width > max_width {
            break;
        }
        width += ch_width;
        end = idx + ch.len_utf8();
    }

    if end == 0 {
        return (String::new(), 0, false);
    }

    let complete = end == text.len();
    (text[..end].to_string(), width, complete)
}

fn draw_scrollbar(
    stdout: &mut Stdout,
    scroll_offset: usize,
    total_lines: usize,
    viewport_height: usize,
    column: u16,
) -> io::Result<()> {
    if viewport_height == 0 || total_lines <= viewport_height {
        return Ok(());
    }

    let track_height = viewport_height as u16;
    let knob_size = ((viewport_height * viewport_height) / total_lines).max(1);
    let knob_size = knob_size.min(viewport_height);
    let max_scroll = total_lines - viewport_height;
    let knob_start = if max_scroll == 0 {
        0
    } else {
        (scroll_offset * (viewport_height - knob_size)) / max_scroll
    };
    let knob_end = knob_start + knob_size;

    for row in 0..viewport_height {
        queue!(stdout, MoveTo(column, row as u16))?;
        if row >= knob_start && row < knob_end {
            queue!(
                stdout,
                SetAttribute(Attribute::Reverse),
                Print(" "),
                SetAttribute(Attribute::NoReverse)
            )?;
        } else {
            queue!(stdout, Print(" "))?;
        }
    }

    queue!(stdout, MoveTo(column, track_height), Print(" "))?;
    Ok(())
}

fn draw_status_line(
    stdout: &mut Stdout,
    state: &PagerState,
    width: u16,
    row: u16,
) -> io::Result<()> {
    let status_text = match &state.search_mode {
        SearchMode::EnteringQuery => format!("/{}", state.search_input),
        SearchMode::Active {
            query,
            matches,
            current_match,
        } => {
            let position_text = if state.total_lines == 0 {
                " (empty)".to_string()
            } else {
                let percentage = if state.max_scroll() == 0 {
                    100
                } else {
                    (state.scroll_offset * 100) / state.max_scroll()
                };
                format!(
                    " Line {}-{}/{} ({}%)",
                    state.scroll_offset + 1,
                    (state.scroll_offset + state.viewport_height).min(state.total_lines),
                    state.total_lines,
                    percentage
                )
            };
            format!(
                "{} -- Searching: '{}' ({}/{} matches) -- n: next, N: prev, Esc: clear",
                position_text,
                query,
                current_match + 1,
                matches.len()
            )
        }
        SearchMode::Normal => {
            if state.total_lines == 0 {
                " (empty) -- q: quit, ↑/↓ j/k: scroll, PgUp/PgDn, Home/End, /: search, Tab: next link, Shift-Tab: prev, Enter: open".to_string()
            } else {
                let percentage = if state.max_scroll() == 0 {
                    100
                } else {
                    (state.scroll_offset * 100) / state.max_scroll()
                };
                format!(
                    " Line {}-{}/{} ({}%) -- q: quit, ↑/↓ j/k: scroll, PgUp/PgDn, Home/End, /: search, Tab/Shift-Tab: links, Enter: open",
                    state.scroll_offset + 1,
                    (state.scroll_offset + state.viewport_height)
                        .min(state.total_lines),
                    state.total_lines,
                    percentage
                )
            }
        }
    };

    queue!(
        stdout,
        MoveTo(0, row),
        Clear(ClearType::CurrentLine),
        SetAttribute(Attribute::Reverse),
        // SetAttribute(Attribute::Dim),
        // SetBackgroundColor(Color::DarkGrey),
        // SetForegroundColor(Color::White),
        Print(truncate_with_padding(status_text.as_str(), width as usize)),
        SetAttribute(Attribute::Reset),
        ResetColor
    )?;

    Ok(())
}

fn truncate_with_padding(text: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }

    let mut result = String::new();
    let mut used = 0usize;
    for ch in text.chars() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);
        if used + ch_width > width {
            break;
        }
        result.push(ch);
        used += ch_width;
    }

    if used < width {
        result.push_str(&" ".repeat(width - used));
    }

    result
}

fn handle_key_event(
    key_event: KeyEvent,
    state: &mut PagerState,
    content: &[ParsedLine],
    needs_redraw: &mut bool,
    link_to_open: &mut Option<String>,
) -> bool {
    if matches!(state.search_mode, SearchMode::EnteringQuery) {
        match key_event.code {
            KeyCode::Enter => {
                state.perform_search(content);
                *needs_redraw = true;
                return true;
            }
            KeyCode::Esc => {
                state.search_mode = SearchMode::Normal;
                state.search_input.clear();
                *needs_redraw = true;
                return true;
            }
            KeyCode::Backspace => {
                if state.search_input.pop().is_some() {
                    *needs_redraw = true;
                }
                return true;
            }
            KeyCode::Char(c) => {
                state.search_input.push(c);
                *needs_redraw = true;
                return true;
            }
            _ => return true,
        }
    }

    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
        match key_event.code {
            KeyCode::Char('c') => return false,
            KeyCode::Char('f') => {
                state.page_down();
                *needs_redraw = true;
            }
            KeyCode::Char('b') => {
                state.page_up();
                *needs_redraw = true;
            }
            _ => {}
        }
        return true;
    }

    match key_event.code {
        KeyCode::Char('q') => return false,
        KeyCode::Esc => {
            if matches!(state.search_mode, SearchMode::Active { .. }) {
                state.clear_search();
                *needs_redraw = true;
            } else {
                return false;
            }
        }
        KeyCode::Char('/') => {
            state.start_search();
            *needs_redraw = true;
        }
        KeyCode::Char('n') => {
            state.next_match();
            *needs_redraw = true;
        }
        KeyCode::Char('N') => {
            state.prev_match();
            *needs_redraw = true;
        }
        KeyCode::Tab => {
            let changed = if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                state.focus_prev_link()
            } else {
                state.focus_next_link()
            };
            if changed {
                *needs_redraw = true;
            }
        }
        KeyCode::BackTab => {
            if state.focus_prev_link() {
                *needs_redraw = true;
            }
        }
        KeyCode::Enter => {
            if let Some(target) = state.current_link_target() {
                *link_to_open = Some(target.to_string());
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            state.scroll_down();
            *needs_redraw = true;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            state.scroll_up();
            *needs_redraw = true;
        }
        KeyCode::PageDown | KeyCode::Char(' ') | KeyCode::Char('f') => {
            state.page_down();
            *needs_redraw = true;
        }
        KeyCode::PageUp => {
            state.page_up();
            *needs_redraw = true;
        }
        KeyCode::Home | KeyCode::Char('g') => {
            state.jump_to_start();
            *needs_redraw = true;
        }
        KeyCode::End | KeyCode::Char('G') => {
            state.jump_to_end();
            *needs_redraw = true;
        }
        _ => {}
    }

    true
}

fn handle_mouse_event(
    mouse_event: MouseEvent,
    state: &mut PagerState,
    needs_redraw: &mut bool,
    link_to_open: &mut Option<String>,
) {
    match mouse_event.kind {
        MouseEventKind::ScrollUp => {
            let previous = state.scroll_offset;
            state.scroll_up();
            if state.scroll_offset != previous {
                *needs_redraw = true;
            }
        }
        MouseEventKind::ScrollDown => {
            let previous = state.scroll_offset;
            state.scroll_down();
            if state.scroll_offset != previous {
                *needs_redraw = true;
            }
        }
        MouseEventKind::Down(MouseButton::Left) => {
            let row = mouse_event.row as usize;
            if row < state.viewport_height {
                let line_idx = state.scroll_offset + row;
                let column = mouse_event.column as usize;
                if state.focus_link_at(line_idx, column).is_some() {
                    *needs_redraw = true;
                    if let Some(link) = state.focused_link() {
                        *link_to_open = Some(link.url.clone());
                    }
                }
            }
        }
        _ => {}
    };
}

fn parse_content_to_lines(content: &str) -> Vec<ParsedLine> {
    content.lines().map(ParsedLine::from_ansi).collect()
}

fn run_interactive_pager<F>(
    mut content: Vec<ParsedLine>,
    mut regenerator: Option<F>,
    options: PagerOptions,
) -> io::Result<()>
where
    F: FnMut(u16, u16) -> Result<String, String>,
{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    if options.enable_mouse_capture {
        execute!(stdout, EnableMouseCapture)?;
    }
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    let (_, current_height) = terminal::size()?;
    let viewport_height = current_height.saturating_sub(1) as usize;
    let mut state = PagerState::new(content.len(), viewport_height);
    state.rebuild_links(&content);

    let mut result = Ok(());
    let mut needs_redraw = true;
    let mut pending_link: Option<String> = None;

    'outer: loop {
        if needs_redraw {
            if let Err(err) = render_pager(&mut stdout, &content, &mut state) {
                result = Err(err);
                break;
            }
            needs_redraw = false;
        }

        match event::read()? {
            Event::Key(key_event) => {
                let mut key_redraw = false;
                if !handle_key_event(
                    key_event,
                    &mut state,
                    &content,
                    &mut key_redraw,
                    &mut pending_link,
                ) {
                    break 'outer;
                }
                needs_redraw |= key_redraw;
            }
            Event::Mouse(mouse_event) => {
                if options.enable_mouse_capture {
                    handle_mouse_event(
                        mouse_event,
                        &mut state,
                        &mut needs_redraw,
                        &mut pending_link,
                    );
                }
            }
            Event::Resize(new_width, new_height) => {
                let new_viewport_height = new_height.saturating_sub(1) as usize;
                let center_line = state.scroll_offset + state.viewport_height / 2;
                let relative_position = if state.total_lines <= 1 {
                    0.0
                } else {
                    let denom = (state.total_lines.saturating_sub(1)) as f64;
                    (center_line as f64 / denom).clamp(0.0, 1.0)
                };
                let active_match_line = match &state.search_mode {
                    SearchMode::Active {
                        matches,
                        current_match,
                        ..
                    } => matches.get(*current_match).map(|m| m.line_idx),
                    _ => None,
                };

                let prev_total_lines = content.len();
                let half_viewport = new_viewport_height / 2;
                let target_center_old = if prev_total_lines <= 1 {
                    0
                } else {
                    let denom = (prev_total_lines.saturating_sub(1)) as f64;
                    (relative_position * denom).round() as usize
                };
                let old_max_scroll = if new_viewport_height == 0 {
                    prev_total_lines.saturating_sub(1)
                } else {
                    prev_total_lines.saturating_sub(new_viewport_height)
                };
                let mut old_scroll_offset = target_center_old.saturating_sub(half_viewport);
                if old_scroll_offset > old_max_scroll {
                    old_scroll_offset = old_max_scroll;
                }

                state.viewport_height = new_viewport_height;
                state.total_lines = prev_total_lines;
                state.scroll_offset = old_scroll_offset;

                let mut new_total_lines = prev_total_lines;
                if let Some(regen) = regenerator.as_mut() {
                    let regenerated = regen(new_width, new_height)
                        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
                    let regenerated_lines = parse_content_to_lines(&regenerated);
                    new_total_lines = regenerated_lines.len();
                    state.rebuild_search_results(&regenerated_lines, active_match_line);
                    content = regenerated_lines;
                    state.rebuild_links(&content);
                    needs_redraw = true;
                }

                state.total_lines = new_total_lines;

                let target_center_new = if new_total_lines <= 1 {
                    0
                } else {
                    let denom = (new_total_lines.saturating_sub(1)) as f64;
                    (relative_position * denom).round() as usize
                };
                let new_max_scroll = if new_viewport_height == 0 {
                    new_total_lines.saturating_sub(1)
                } else {
                    new_total_lines.saturating_sub(new_viewport_height)
                };
                let mut new_scroll_offset = target_center_new.saturating_sub(half_viewport);
                if new_scroll_offset > new_max_scroll {
                    new_scroll_offset = new_max_scroll;
                }
                state.scroll_offset = new_scroll_offset;
            }
            _ => {}
        }

        if pending_link.is_some() {
            break 'outer;
        }
    }

    if options.enable_mouse_capture {
        execute!(stdout, DisableMouseCapture)?;
    }
    execute!(stdout, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    if result.is_ok() {
        if let Some(link_target) = pending_link {
            if let Some(callback) = options.link_callback.clone() {
                callback(link_target.as_str());
            }
        }
    }

    result
}

fn is_interactive_terminal() -> bool {
    use std::io::IsTerminal;
    io::stdout().is_terminal()
}

/// Page ANSI content to the terminal if needed.
pub fn page_output(content: &str) -> Result<(), String> {
    page_output_with_options(content, PagerOptions::default())
}

pub fn page_output_with_regenerator<F>(content: &str, regenerator: Option<F>) -> Result<(), String>
where
    F: FnMut(u16, u16) -> Result<String, String>,
{
    page_output_with_options_and_regenerator(content, regenerator, PagerOptions::default())
}

pub fn page_output_with_options(content: &str, options: PagerOptions) -> Result<(), String> {
    page_output_with_options_and_regenerator(
        content,
        Option::<fn(u16, u16) -> Result<String, String>>::None,
        options,
    )
}

pub fn page_output_with_options_and_regenerator<F>(
    content: &str,
    regenerator: Option<F>,
    options: PagerOptions,
) -> Result<(), String>
where
    F: FnMut(u16, u16) -> Result<String, String>,
{
    let line_count = content.lines().count();

    let should_page = if !is_interactive_terminal() {
        false
    } else if let Ok((_, height)) = terminal::size() {
        let viewport_height = (height as usize).saturating_sub(3);
        line_count > viewport_height
    } else {
        false
    };

    if should_page {
        let parsed_lines = parse_content_to_lines(content);
        run_interactive_pager(parsed_lines, regenerator, options)
            .map_err(|e| format!("Pager error: {}", e))
    } else {
        print!("{}", content);
        Ok(())
    }
}
