use std::cmp::min;

const SPACE_CHARS: &str = " \t\r\n";

fn slice_to_string(bytes: &[u8], start: usize, end: usize) -> String {
    String::from_utf8(bytes[start..end].to_vec()).expect("input is valid UTF-8")
}

fn find_subslice(haystack: &[u8], start: usize, needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(start);
    }

    haystack
        .get(start..)?
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|offset| start + offset)
}

fn find_byte(haystack: &[u8], start: usize, value: u8) -> Option<usize> {
    haystack
        .get(start..)?
        .iter()
        .position(|&b| b == value)
        .map(|offset| start + offset)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tokenizer<'a> {
    input: &'a str,
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            bytes: input.as_bytes(),
            position: 0,
        }
    }

    fn shift(&mut self, end: &str) -> String {
        if let Some(pattern_pos) = find_subslice(self.bytes, self.position, end.as_bytes()) {
            let end_pos = pattern_pos + end.len();
            let start = self.position;
            self.position = end_pos;
            return slice_to_string(self.bytes, start, end_pos);
        }

        self.shift_until('<')
    }

    fn shift_to_tag_end(&mut self) -> String {
        #[derive(Clone, Copy)]
        enum State {
            TagName,
            TagContent,
            AttributeName,
            AttributeValue,
            DoubleQuote,
            SingleQuote,
        }

        let mut state = State::TagName;
        let start = self.position;
        let len = self.bytes.len();
        let mut i = self.position + 1;

        while i < len {
            let curr = self.bytes[i];
            match state {
                State::DoubleQuote => {
                    if curr == b'"' {
                        state = State::TagName;
                    }
                    i += 1;
                    continue;
                }
                State::SingleQuote => {
                    if curr == b'\'' {
                        state = State::TagName;
                    }
                    i += 1;
                    continue;
                }
                _ => {}
            }

            match curr {
                b' ' | b'\t' | b'\r' | b'\n' => match state {
                    State::TagName => state = State::TagContent,
                    State::AttributeValue => state = State::TagContent,
                    _ => {}
                },
                b'=' => {
                    if let State::AttributeName = state {
                        state = State::AttributeValue;
                    }
                }
                b'<' => {
                    let result = slice_to_string(self.bytes, start, i);
                    self.position = i;
                    return result;
                }
                b'>' => {
                    let end_idx = min(i + 1, len);
                    let result = slice_to_string(self.bytes, start, end_idx);
                    self.position = end_idx;
                    return result;
                }
                b'"' => {
                    if let State::AttributeValue = state {
                        state = State::DoubleQuote;
                    }
                }
                b'\'' => {
                    if let State::AttributeValue = state {
                        state = State::SingleQuote;
                    }
                }
                _ => {
                    if let State::TagContent = state {
                        state = State::AttributeName;
                    }
                }
            }

            i += 1;
        }

        self.position = len;
        slice_to_string(self.bytes, start, len)
    }

    fn shift_until(&mut self, next: char) -> String {
        let len = self.bytes.len();
        if self.position < len {
            let search_start = (self.position + 1).min(len);
            if let Some(pos) = find_byte(self.bytes, search_start, next as u8) {
                let start = self.position;
                self.position = pos;
                return slice_to_string(self.bytes, start, pos);
            }
        }

        let start = self.position;
        self.position = len;
        slice_to_string(self.bytes, start, len)
    }

    fn has(&self, next: &str) -> bool {
        let start = self.position;
        let end = start + next.len();
        end <= self.bytes.len() && &self.bytes[start..end] == next.as_bytes()
    }

    pub fn next_token(&mut self) -> Result<Token, TokenizerError> {
        let len = self.bytes.len();
        if self.position >= len {
            return Err(TokenizerError::Eof);
        }

        let bytes = self.bytes;

        if self.position.saturating_add(3) < len
            && bytes[self.position] == b'<'
            && self.position + 1 < len
        {
            match bytes[self.position + 1] {
                b'?' => {
                    let raw = self.shift("?>");
                    return Ok(Token::ProcInst(raw));
                }
                b'!' => {
                    if self.has("<!--") {
                        let raw = self.shift("-->");
                        return Ok(Token::Comment(raw));
                    }
                    if self.has("<![CDATA[") {
                        let raw = self.shift("]]>");
                        return Ok(Token::CData(raw));
                    }
                    let mut raw = self.shift(">");
                    if raw.starts_with("<!DOCTYPE") && raw.contains('[') {
                        raw.push_str(&self.shift("]"));
                        raw.push_str(&self.shift(">"));
                    }
                    return Ok(Token::Directive(raw));
                }
                b'/' => {
                    let raw = self.shift(">");
                    return Ok(Token::EndElement(EndElementToken::new(raw)));
                }
                _ => {
                    let raw = self.shift_to_tag_end();
                    if raw.len() >= 3 && raw.as_bytes()[raw.len() - 2] == b'/' {
                        return Ok(Token::EmptyElement(EmptyElementToken::new(raw)));
                    }
                    return Ok(Token::StartElement(StartElementToken::new(raw)));
                }
            }
        }

        let raw = self.shift_until('<');
        if raw.is_empty() {
            return Err(TokenizerError::Eof);
        }
        Ok(Token::Text(raw))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Text(String),
    CData(String),
    Comment(String),
    Directive(String),
    ProcInst(String),
    StartElement(StartElementToken),
    EndElement(EndElementToken),
    EmptyElement(EmptyElementToken),
}

impl Token {
    pub fn raw(&self) -> &str {
        match self {
            Token::Text(raw)
            | Token::CData(raw)
            | Token::Comment(raw)
            | Token::Directive(raw)
            | Token::ProcInst(raw) => raw,
            Token::StartElement(tok) => tok.raw(),
            Token::EndElement(tok) => tok.raw(),
            Token::EmptyElement(tok) => tok.raw(),
        }
    }

    pub fn as_element(&self) -> Option<ElementToken<'_>> {
        match self {
            Token::StartElement(tok) => Some(ElementToken::Start(tok)),
            Token::EmptyElement(tok) => Some(ElementToken::Empty(tok)),
            Token::EndElement(tok) => Some(ElementToken::End(tok)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ElementToken<'a> {
    Start(&'a StartElementToken),
    Empty(&'a EmptyElementToken),
    End(&'a EndElementToken),
}

impl<'a> ElementToken<'a> {
    pub fn name(&self) -> &str {
        match self {
            ElementToken::Start(tok) => tok.name(),
            ElementToken::Empty(tok) => tok.name(),
            ElementToken::End(tok) => tok.name(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartElementToken {
    raw: String,
}

impl StartElementToken {
    pub fn new<S: Into<String>>(raw: S) -> Self {
        Self { raw: raw.into() }
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn name(&self) -> &str {
        if self.raw.len() <= 1 {
            return "";
        }
        let body = &self.raw[1..];
        if let Some(idx) = body.find([' ', '\t', '\r', '\n', '>', '/']) {
            &body[..idx]
        } else {
            body
        }
    }

    pub fn attributes(&self) -> Vec<Attribute> {
        if self.raw.len() <= 1 {
            return Vec::new();
        }
        let content = if self.raw.ends_with('>') && self.raw.len() >= 2 {
            &self.raw[1..self.raw.len() - 1]
        } else {
            &self.raw[1..]
        };
        get_attributes(content)
    }

    pub fn attribute(&self, name: &str) -> Option<String> {
        if self.raw.len() < 2 {
            return None;
        }
        let content = if self.raw.ends_with('>') && self.raw.len() >= 2 {
            &self.raw[1..self.raw.len() - 1]
        } else {
            &self.raw[1..]
        };
        get_attribute(content, name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndElementToken {
    raw: String,
}

impl EndElementToken {
    pub fn new<S: Into<String>>(raw: S) -> Self {
        Self { raw: raw.into() }
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn name(&self) -> &str {
        if self.raw.len() <= 2 {
            ""
        } else {
            let raw_end = if self.raw.ends_with('>') {
                self.raw.len().saturating_sub(1)
            } else {
                self.raw.len()
            };

            if raw_end <= 2 {
                ""
            } else {
                let body = &self.raw[2..raw_end];
                let trimmed = body.trim();
                if trimmed.is_empty() {
                    ""
                } else {
                    trimmed
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyElementToken {
    raw: String,
}

impl EmptyElementToken {
    pub fn new<S: Into<String>>(raw: S) -> Self {
        Self { raw: raw.into() }
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn name(&self) -> &str {
        if self.raw.len() <= 1 {
            return "";
        }
        let body = if self.raw.ends_with("/>") && self.raw.len() >= 2 {
            &self.raw[1..self.raw.len() - 2]
        } else if self.raw.ends_with('>') && !self.raw.is_empty() {
            &self.raw[1..self.raw.len() - 1]
        } else {
            &self.raw[1..]
        };
        if let Some(idx) = body.find([' ', '\t', '\r', '\n', '>', '/']) {
            &body[..idx]
        } else {
            body
        }
    }

    pub fn attributes(&self) -> Vec<Attribute> {
        if self.raw.len() <= 2 {
            return Vec::new();
        }
        let end = self.raw.len().saturating_sub(2);
        let content = &self.raw[1..end];
        get_attributes(content)
    }

    pub fn attribute(&self, name: &str) -> Option<String> {
        if self.raw.len() <= 2 {
            return None;
        }
        let end = self.raw.len().saturating_sub(2);
        let content = &self.raw[1..end];
        get_attribute(content, name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub content: String,
}

fn get_attribute(raw_input: &str, name: &str) -> Option<String> {
    let mut tokenizer = AttributeTokenizer::new(raw_input);
    tokenizer.shift_until_space();
    while let Some(attribute) = tokenizer.next_attribute() {
        if attribute.name.eq_ignore_ascii_case(name) {
            return Some(attribute.content);
        }
    }
    None
}

fn get_attributes(raw_input: &str) -> Vec<Attribute> {
    let mut tokenizer = AttributeTokenizer::new(raw_input);
    tokenizer.shift_until_space();
    let mut list = Vec::new();
    while let Some(attribute) = tokenizer.next_attribute() {
        list.push(attribute);
    }
    list
}

struct AttributeTokenizer<'a> {
    input: &'a str,
    bytes: &'a [u8],
    position: usize,
}

impl<'a> AttributeTokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            bytes: input.as_bytes(),
            position: 0,
        }
    }

    fn shift_until(&mut self, next: &str) -> String {
        if self.position >= self.bytes.len() {
            return String::new();
        }

        if let Some(pos) = find_subslice(self.bytes, self.position + 1, next.as_bytes()) {
            let start = self.position;
            self.position = pos;
            return slice_to_string(self.bytes, start, pos);
        }

        let start = self.position;
        self.position = self.bytes.len();
        slice_to_string(self.bytes, start, self.bytes.len())
    }

    fn shift_until_space(&mut self) -> String {
        if self.position >= self.bytes.len() {
            return String::new();
        }

        if self.position + 1 >= self.bytes.len() {
            let start = self.position;
            self.position = self.bytes.len();
            return slice_to_string(self.bytes, start, self.bytes.len());
        }

        if let Some(pos) = self.bytes[self.position + 1..]
            .iter()
            .position(|&b| SPACE_CHARS.as_bytes().contains(&b))
        {
            let start = self.position;
            let end_idx = start + pos + 1;
            self.position = end_idx;
            return slice_to_string(self.bytes, start, end_idx);
        }

        let start = self.position;
        self.position = self.bytes.len();
        slice_to_string(self.bytes, start, self.bytes.len())
    }

    fn eat_space(&mut self) -> String {
        if self.position >= self.bytes.len() {
            return String::new();
        }

        let start = self.position;
        while self.position < self.bytes.len() {
            let ch = self.bytes[self.position];
            if !SPACE_CHARS.as_bytes().contains(&ch) {
                break;
            }
            self.position += 1;
        }

        slice_to_string(self.bytes, start, self.position)
    }

    fn shift_value(&mut self) -> String {
        let mut value = self.shift_until_space();
        if value.is_empty() {
            return value;
        }

        let first = value.chars().next().unwrap();
        if first != '"' && first != '\'' {
            return value;
        }

        let quote = first;
        while !value.ends_with(quote) {
            let part = format!("{}{}", self.eat_space(), self.shift_until_space());
            if part.is_empty() {
                break;
            }
            value.push_str(&part);
        }

        value.trim_matches(quote).to_string()
    }

    fn next_attribute(&mut self) -> Option<Attribute> {
        let _ = self.eat_space();
        if self.position >= self.input.len() {
            return None;
        }

        let key = self.shift_until("=");
        if self.position < self.input.len() {
            self.position += 1;
        }
        let _ = self.eat_space();

        if self.position >= self.input.len() {
            return Some(Attribute {
                name: key,
                content: String::new(),
            });
        }

        Some(Attribute {
            name: key,
            content: self.shift_value(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenizerError {
    Eof,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DocumentInfo {
        data: &'static str,
        element_names: &'static [&'static str],
    }

    static DOCUMENTS: &[(&str, DocumentInfo)] = &[
        (
            "doctype subset",
            DocumentInfo {
                data: r#"<?xml version="1.0" encoding="UTF-8" ?>
<!DOCTYPE doc [
    <!ELEMENT doc ANY>
]>
<doc>
</doc>"#,
                element_names: &["doc", "doc"],
            },
        ),
        (
            "empty start",
            DocumentInfo {
                data: "<<a",
                element_names: &[],
            },
        ),
        (
            "empty end",
            DocumentInfo {
                data: "</<a",
                element_names: &[""],
            },
        ),
        (
            "empty doctype",
            DocumentInfo {
                data: "<!DOCTYPE[",
                element_names: &[],
            },
        ),
        (
            "simple-svg",
            DocumentInfo {
                data: r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="100%" height="100%" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 1920 1080">
  <style>
/* This is a comment. */
.test {
	fill: 'black';
}
  </style>
  <rect width="1920" height="1080" class="test" fill="red"></rect>
  <defs>
    <linearGradient id="grad">
      <stop stop-color="white" offset="0"></stop>
      <stop stop-opacity="0" stop-color="white" offset="1"></stop>
    </linearGradient>
  </defs>
</svg>"#,
                element_names: &[
                    "svg",
                    "style",
                    "style",
                    "rect",
                    "rect",
                    "defs",
                    "linearGradient",
                    "stop",
                    "stop",
                    "stop",
                    "stop",
                    "linearGradient",
                    "defs",
                    "svg",
                ],
            },
        ),
    ];

    fn passthrough(data: &str) -> String {
        let mut buf = String::new();
        let mut tokenizer = Tokenizer::new(data);
        while let Ok(token) = tokenizer.next_token() {
            buf.push_str(token.raw());
        }
        buf
    }

    fn elements(data: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut tokenizer = Tokenizer::new(data);
        while let Ok(token) = tokenizer.next_token() {
            if let Some(element) = token.as_element() {
                result.push(element.name().to_string());
            }
        }
        result
    }

    #[test]
    fn test_no_change() {
        for (name, info) in DOCUMENTS.iter() {
            assert_eq!(
                info.data,
                passthrough(info.data),
                "document '{name}' changed"
            );
        }
    }

    #[test]
    fn test_element_names() {
        for (name, info) in DOCUMENTS.iter() {
            let actual = elements(info.data);
            for (pos, expected) in info.element_names.iter().enumerate() {
                if pos >= actual.len() {
                    panic!("Element pos {pos} not existing for document {name}");
                } else {
                    assert_eq!(
                        actual[pos], *expected,
                        "Element name mismatch at pos {pos} for document {name}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_broken_start_element() {
        let input = "<elem";
        let mut decoder = Tokenizer::new(input);
        let tok = decoder.next_token().expect("expected token");
        match tok {
            Token::StartElement(_) => {}
            other => panic!("Not a start element token: {input}, got {other:?}"),
        }
        assert_eq!(tok.raw(), input);
        assert_eq!(decoder.next_token(), Err(TokenizerError::Eof));
    }

    #[test]
    fn test_greater_symbol() {
        let cases = [
            (
                "<style>h1 > i { color: red !important; }</style>",
                "h1 > i { color: red !important; }",
            ),
            (
                "<button email='Someone <hello@example.example>'>Contact</button>",
                "Contact",
            ),
        ];

        for (input, content) in cases {
            let mut decoder = Tokenizer::new(input);
            let tok = decoder.next_token().expect("expected token");
            match tok {
                Token::StartElement(_) => {}
                other => panic!("Not a start element token: {input}, got {other:?}"),
            }

            let content_tok = decoder.next_token().expect("expected content token");
            match content_tok {
                Token::Text(ref raw) => assert_eq!(raw, content),
                other => panic!("Not a text token: {input}, got {other:?}"),
            }
            assert_eq!(content_tok.raw(), content);

            // skip end element
            let _ = decoder.next_token();

            assert_eq!(decoder.next_token(), Err(TokenizerError::Eof));
        }
    }

    #[test]
    fn test_newline_in_elements() {
        let cases = [
            ("<a\nb>", '<'),
            ("<a\nb\n>", '<'),
            ("<\na\nb=c\n>", '<'),
            ("</a>", '>'),
            ("</\na>", '>'),
            ("</a\n>", '>'),
            ("<a\n  name='b'\n  content='c'\n/>", '/'),
        ];

        for (input, typ) in cases {
            let mut decoder = Tokenizer::new(input);
            let tok = decoder.next_token().expect("expected token");
            match typ {
                '<' => match tok {
                    Token::StartElement(_) => {}
                    other => panic!("Not a start element token: {input}, got {other:?}"),
                },
                '>' => match tok {
                    Token::EndElement(_) => {}
                    other => panic!("Not an end element token: {input}, got {other:?}"),
                },
                '/' => match tok {
                    Token::EmptyElement(_) => {}
                    other => panic!("Not an empty element token: {input}, got {other:?}"),
                },
                _ => panic!("unexpected type marker"),
            }
            assert_eq!(tok.raw(), input);
            assert_eq!(decoder.next_token(), Err(TokenizerError::Eof));
        }
    }

    #[test]
    fn test_broken_text_element() {
        let input = "/asdkjlh";
        let mut decoder = Tokenizer::new(input);
        let tok = decoder.next_token().expect("expected token");
        match tok {
            Token::Text(ref raw) => assert_eq!(raw, input),
            other => panic!("Not a text token: {input}, got {other:?}"),
        }
        assert_eq!(tok.raw(), input);
        assert_eq!(decoder.next_token(), Err(TokenizerError::Eof));
    }

    #[test]
    fn test_attributes() {
        let svg = StartElementToken::new(
            r#"<svg xmlns="http://www.w3.org/2000/svg" version=1.1 width='100%' height='a + b' xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 1920 1080" bla=blub bla>"#,
        );
        let expected = [
            Attribute {
                name: "xmlns".into(),
                content: "http://www.w3.org/2000/svg".into(),
            },
            Attribute {
                name: "version".into(),
                content: "1.1".into(),
            },
            Attribute {
                name: "width".into(),
                content: "100%".into(),
            },
            Attribute {
                name: "height".into(),
                content: "a + b".into(),
            },
            Attribute {
                name: "xmlns:xlink".into(),
                content: "http://www.w3.org/1999/xlink".into(),
            },
            Attribute {
                name: "viewBox".into(),
                content: "0 0 1920 1080".into(),
            },
            Attribute {
                name: "bla".into(),
                content: "blub".into(),
            },
            Attribute {
                name: "bla".into(),
                content: String::new(),
            },
        ];

        let result = svg.attributes();
        assert_eq!(result.len(), expected.len(), "attribute count mismatch");
        for (actual, expected) in result.iter().zip(expected.iter()) {
            assert_eq!(actual, expected, "attribute mismatch");
        }
    }

    #[test]
    fn test_attributes_in_empty_elements() {
        let svg = EmptyElementToken::new(r#"<circle cx="50" cy="25" r="20" fill="yellow" />"#);
        let expected = [
            Attribute {
                name: "cx".into(),
                content: "50".into(),
            },
            Attribute {
                name: "cy".into(),
                content: "25".into(),
            },
            Attribute {
                name: "r".into(),
                content: "20".into(),
            },
            Attribute {
                name: "fill".into(),
                content: "yellow".into(),
            },
        ];

        assert_eq!(svg.name(), "circle");
        let result = svg.attributes();
        assert_eq!(result.len(), expected.len(), "attribute count mismatch");
        for (actual, expected) in result.iter().zip(expected.iter()) {
            assert_eq!(actual, expected, "attribute mismatch");
        }
    }

    #[test]
    fn test_getting_attributes_by_name() {
        struct AttribTest {
            token: Token,
            attributes: Vec<Attribute>,
        }

        type CaseFn = Box<dyn Fn(&str) -> String>;

        let tests = vec![
            AttribTest {
                token: Token::EmptyElement(EmptyElementToken::new(
                    r#"<circle cx="50" cy="25" r="20" fill="yellow" />"#,
                )),
                attributes: vec![
                    Attribute {
                        name: "cx".into(),
                        content: "50".into(),
                    },
                    Attribute {
                        name: "R".into(),
                        content: "20".into(),
                    },
                ],
            },
            AttribTest {
                token: Token::StartElement(StartElementToken::new(
                    r#"<group style="fill: none;" style="nope">"#,
                )),
                attributes: vec![Attribute {
                    name: "style".into(),
                    content: "fill: none;".into(),
                }],
            },
        ];

        let casing_variants: Vec<CaseFn> = vec![
            Box::new(|s| s.to_lowercase()),
            Box::new(|s| s.to_uppercase()),
            Box::new(|s| s.to_string()),
        ];

        for test in tests {
            let AttribTest { token, attributes } = test;
            let token = match token {
                Token::EmptyElement(tok) => StartOrEmpty::Empty(tok),
                Token::StartElement(tok) => StartOrEmpty::Start(tok),
                _ => panic!("unexpected token variant"),
            };

            for attribute in attributes.iter() {
                for casing in &casing_variants {
                    let name = casing(&attribute.name);
                    let value = token.attribute(&name).unwrap_or_else(|| {
                        panic!("Missing attribute {name} in token {:?}", token.raw())
                    });
                    assert_eq!(
                        value.as_str(),
                        attribute.content.as_str(),
                        "Wrong attribute {name} in token {:?}",
                        token.raw()
                    );
                }
            }
        }
    }

    #[derive(Debug, Clone)]
    enum StartOrEmpty {
        Start(StartElementToken),
        Empty(EmptyElementToken),
    }

    impl StartOrEmpty {
        fn attribute(&self, name: &str) -> Option<String> {
            match self {
                StartOrEmpty::Start(tok) => tok.attribute(name),
                StartOrEmpty::Empty(tok) => tok.attribute(name),
            }
        }

        fn raw(&self) -> &str {
            match self {
                StartOrEmpty::Start(tok) => tok.raw(),
                StartOrEmpty::Empty(tok) => tok.raw(),
            }
        }
    }

    #[test]
    fn test_cdata() {
        let mut doc = Tokenizer::new(r#"<p><![CDATA[</p>]]><!-- </p> --></p>"#);

        match doc.next_token() {
            Ok(Token::StartElement(_)) => {}
            other => panic!("Expected start element token, got {other:?}"),
        }

        match doc.next_token() {
            Ok(Token::CData(raw)) => assert_eq!(raw, "<![CDATA[</p>]]>"),
            other => panic!("Expected CDATA token, got {other:?}"),
        }

        match doc.next_token() {
            Ok(Token::Comment(raw)) => assert_eq!(raw, "<!-- </p> -->"),
            other => panic!("Expected comment token, got {other:?}"),
        }

        match doc.next_token() {
            Ok(Token::EndElement(tok)) => assert_eq!(tok.raw(), "</p>"),
            other => panic!("Expected end element token, got {other:?}"),
        }
    }

    fn get_all_tokens(data: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut tokenizer = Tokenizer::new(data);
        while let Ok(token) = tokenizer.next_token() {
            tokens.push(token);
        }
        tokens
    }

    #[test]
    fn test_parsing_fringe_cases() {
        struct DocInfo {
            data: &'static str,
            tokens: Vec<Token>,
        }

        let tests = [
            DocInfo {
                data: r#"<script =">alert(1)</script>"#,
                tokens: vec![
                    Token::StartElement(StartElementToken::new(r#"<script =">"#)),
                    Token::Text("alert(1)".into()),
                    Token::EndElement(EndElementToken::new("</script>")),
                ],
            },
            DocInfo {
                data: r#"<A/=">"#,
                tokens: vec![Token::StartElement(StartElementToken::new(r#"<A/=">"#))],
            },
        ];

        for (name, info) in ["dangling =", "= after /"].iter().zip(tests.iter()) {
            let tokens = get_all_tokens(info.data);
            for (pos, expected) in info.tokens.iter().enumerate() {
                if pos >= tokens.len() {
                    panic!("Token pos {pos} not existing for document {name}");
                }
                assert_eq!(
                    tokens[pos], *expected,
                    "Token mismatch at pos {pos} for input {name}"
                );
            }
        }
    }
}
