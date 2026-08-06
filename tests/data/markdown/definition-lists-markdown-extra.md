# Definition Lists

Markdown Extra implements definition lists. Definition lists are made of terms
and definitions of these terms, much like in a dictionary.

A simple definition list in Markdown Extra is made of a single-line term
followed by a colon and the definition for that term.

Apple
:   Pomaceous fruit of plants of the genus Malus in
    the family Rosaceae.

Orange
:   The fruit of an evergreen tree of the genus Citrus.

## Lazy continuation

Terms must be separated from the previous definition by a blank line.
Definitions can span on multiple lines, in which case they should be indented.
But you don't really have to: if you want to be lazy, you could forget to indent
a definition that span on multiple lines and it will still work:

Apple
:   Pomaceous fruit of plants of the genus Malus in
the family Rosaceae.

Orange
:   The fruit of an evergreen tree of the genus Citrus.

## Several definitions for one term

Colons as definition markers typically start at the left margin, but may be
indented by up to three spaces. Definition markers must be followed by one or
more spaces or a tab.

Definition lists can have more than one definition associated with one term:

Apple
:   Pomaceous fruit of plants of the genus Malus in
    the family Rosaceae.
:   An American computer company.

Orange
:   The fruit of an evergreen tree of the genus Citrus.

## Several terms for one definition

You can also associate more than one term to a definition:

Term 1
Term 2
:   Definition a

Term 3
:   Definition b

## Loose definitions

If a definition is preceded by a blank line, Markdown Extra will wrap the
definition in `<p>` tags in the HTML output. For example, this:

Apple

:   Pomaceous fruit of plants of the genus Malus in
    the family Rosaceae.

Orange

:    The fruit of an evergreen tree of the genus Citrus.

## Block-level content in definitions

And just like regular list items, definitions can contain multiple paragraphs,
and include other block-level elements such as blockquotes, code blocks, lists,
and even other definition lists.

Term 1

:   This is a definition with two paragraphs. Lorem ipsum
    dolor sit amet, consectetuer adipiscing elit. Aliquam
    hendrerit mi posuere lectus.

    Vestibulum enim wisi, viverra nec, fringilla in, laoreet
    vitae, risus.

:   Second definition for term 1, also wrapped in a paragraph
    because of the blank line preceding it.

Term 2

:   This definition has a code block, a blockquote and a list.

        code block.

    > block quote
    > on two lines.

    1.  first list item
    2.  second list item
