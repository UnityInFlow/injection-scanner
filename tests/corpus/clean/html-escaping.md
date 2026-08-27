# Escaping user content in templates

<!-- Would-be PI047: an entity run is how you *prevent* injection in HTML, so
     flagging it inverts the signal. -->

Always escape interpolated values. The renderer emits:

    &lt;script&gt;alert&#40;1&#41;&#59;&lt;/script&gt;

for input that would otherwise execute. Short numeric forms are fine —
`&#72;&#101;&#108;` is three entities (below the PI047 run threshold), and a
named entity like `&amp;` never counts.

Use `&amp;` for a literal ampersand, `&lt;` and `&gt;` for angle brackets, and
`&quot;` inside attribute values.
