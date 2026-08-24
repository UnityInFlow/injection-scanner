# Escaping user content in templates

<!-- Would-be PI047: an entity run is how you *prevent* injection in HTML, so
     flagging it inverts the signal. -->

Always escape interpolated values. The renderer emits:

    &lt;script&gt;alert&#40;1&#41;&#59;&lt;/script&gt;

for input that would otherwise execute. Numeric forms are equivalent —
`&#72;&#101;&#108;&#108;&#111;` renders as `Hello`, and `&#x48;&#x65;&#x6C;&#x6C;&#x6F;`
is the hexadecimal spelling of the same thing.

Use `&amp;` for a literal ampersand, `&lt;` and `&gt;` for angle brackets, and
`&quot;` inside attribute values.
