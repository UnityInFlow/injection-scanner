//! Source positions only. The existing parsers remain authoritative for values.
use std::collections::HashMap;

use yaml_rust2::parser::{Event, MarkedEventReceiver, Parser};
use yaml_rust2::scanner::Marker;

use super::{relax_jsonc, ConfigBlock, ConfigSyntax, MAX_DEPTH, MAX_NODES};

// Projection counts leaves; the position tree also needs their container nodes.
const SOURCE_NODE_LIMIT: usize = MAX_NODES * (MAX_DEPTH + 1);

#[derive(Clone, Default)]
pub(super) struct Source {
    pub line: usize,
    key_text: String,
    keys: HashMap<String, Source>,
    items: Vec<Source>,
}

impl Source {
    pub fn key(&self, key: &str) -> Option<&Self> {
        self.keys.get(key)
    }

    pub fn index(&self, index: usize) -> Option<&Self> {
        self.items.get(index)
    }
}

pub(super) fn positions(block: &ConfigBlock) -> Option<Source> {
    match block.syntax {
        ConfigSyntax::Json | ConfigSyntax::Yaml => {
            let body = if block.syntax == ConfigSyntax::Json {
                json_source(&relax_jsonc(&block.body))?
            } else {
                block.body.clone()
            };
            let mut receiver = Locations::default();
            Parser::new_from_str(&body)
                .load(&mut receiver, false)
                .ok()?;
            receiver.root
        }
        ConfigSyntax::Toml => {
            let document = toml_edit::ImDocument::parse(&block.body).ok()?;
            let newlines: Vec<_> = block
                .body
                .match_indices('\n')
                .map(|(index, _)| index)
                .collect();
            let mut remaining = SOURCE_NODE_LIMIT;
            Some(toml_item(document.as_item(), &newlines, 0, &mut remaining))
        }
    }
}

// YAML's quoted-string grammar does not accept JSON surrogate-pair escapes.
// Decode/re-encode just strings for the position parser. This preserves physical
// newlines, including escaped newlines, while the semantic JSON tree is untouched.
fn json_source(body: &str) -> Option<String> {
    let bytes = body.as_bytes();
    let mut out = String::with_capacity(body.len());
    let mut copied = 0;
    let mut cursor = 0;
    while cursor < bytes.len() {
        if bytes[cursor] != b'"' {
            cursor += 1;
            continue;
        }
        let start = cursor;
        cursor += 1;
        while cursor < bytes.len() {
            match bytes[cursor] {
                b'\\' => cursor += 2,
                b'"' => break,
                _ => cursor += 1,
            }
        }
        cursor += 1;
        let text: String = serde_json::from_str(body.get(start..cursor)?).ok()?;
        let encoded = serde_json::to_string(&text).ok()?;
        out.push_str(&body[copied..start]);
        out.push_str(
            &encoded
                .replace('\u{85}', "\\u0085")
                .replace('\u{2028}', "\\u2028")
                .replace('\u{2029}', "\\u2029"),
        );
        copied = cursor;
    }
    out.push_str(&body[copied..]);
    Some(out)
}

struct Frame {
    node: Source,
    mapping: bool,
    key: Option<(String, usize)>,
    anchor: usize,
}

#[derive(Default)]
struct Locations {
    root: Option<Source>,
    stack: Vec<Frame>,
    anchors: HashMap<usize, Source>,
    nodes: usize,
    skipped: usize,
}

impl Locations {
    fn attach(&mut self, mut node: Source) {
        if let Some(parent) = self.stack.last_mut() {
            if parent.mapping {
                if let Some((key, line)) = parent.key.take() {
                    node.line = line;
                    parent.node.keys.insert(key, node);
                } else {
                    parent.key = Some((node.key_text, node.line));
                }
            } else {
                parent.node.items.push(node);
            }
        } else {
            self.root = Some(node);
        }
    }
}

impl MarkedEventReceiver for Locations {
    fn on_event(&mut self, event: Event, mark: Marker) {
        // Keep the auxiliary position tree within the projection's own bounds.
        // Once a container is skipped, consume its balanced events, not siblings.
        if self.skipped > 0 {
            match event {
                Event::MappingStart(..) | Event::SequenceStart(..) => self.skipped += 1,
                Event::MappingEnd | Event::SequenceEnd => self.skipped -= 1,
                Event::Nothing
                | Event::StreamStart
                | Event::StreamEnd
                | Event::DocumentStart
                | Event::DocumentEnd
                | Event::Scalar(..)
                | Event::Alias(_) => {}
            }
            return;
        }
        match event {
            Event::MappingStart(anchor, _) | Event::SequenceStart(anchor, _) => {
                if self.stack.len() > MAX_DEPTH || self.nodes >= SOURCE_NODE_LIMIT {
                    self.skipped = 1;
                    // Consume the map's pending key even when its value is skipped.
                    self.attach(Source {
                        line: mark.line(),
                        ..Source::default()
                    });
                    return;
                }
                self.nodes += 1;
                self.stack.push(Frame {
                    node: Source {
                        line: mark.line(),
                        ..Source::default()
                    },
                    mapping: matches!(event, Event::MappingStart(..)),
                    key: None,
                    anchor,
                });
            }
            Event::MappingEnd | Event::SequenceEnd => {
                if let Some(frame) = self.stack.pop() {
                    if frame.anchor != 0 {
                        self.anchors.insert(frame.anchor, frame.node.clone());
                    }
                    self.attach(frame.node);
                }
            }
            Event::Scalar(text, _, anchor, _) => {
                let is_key = self
                    .stack
                    .last()
                    .is_some_and(|frame| frame.mapping && frame.key.is_none());
                if self.nodes >= SOURCE_NODE_LIMIT {
                    return;
                }
                if !is_key {
                    self.nodes += 1;
                }
                let node = Source {
                    line: mark.line(),
                    key_text: if is_key || anchor != 0 {
                        text
                    } else {
                        String::new()
                    },
                    ..Source::default()
                };
                if anchor != 0 {
                    self.anchors.insert(anchor, node.clone());
                }
                self.attach(node);
            }
            Event::Alias(anchor) => {
                if let Some(node) = self.anchors.get(&anchor) {
                    let mut remaining = SOURCE_NODE_LIMIT.saturating_sub(self.nodes);
                    if let Some(copy) = alias_at(node, mark.line(), &mut remaining) {
                        self.nodes = SOURCE_NODE_LIMIT - remaining;
                        self.attach(copy);
                    }
                }
            }
            Event::Nothing
            | Event::StreamStart
            | Event::StreamEnd
            | Event::DocumentStart
            | Event::DocumentEnd => {}
        }
    }
}

// An alias is attributed to its reference, not the anchor's earlier declaration.
fn alias_at(node: &Source, line: usize, remaining: &mut usize) -> Option<Source> {
    *remaining = remaining.checked_sub(1)?;
    Some(Source {
        line,
        key_text: node.key_text.clone(),
        keys: node
            .keys
            .iter()
            .filter_map(|(key, child)| {
                alias_at(child, line, remaining).map(|copy| (key.clone(), copy))
            })
            .collect(),
        items: node
            .items
            .iter()
            .filter_map(|child| alias_at(child, line, remaining))
            .collect(),
    })
}

fn source_line(newlines: &[usize], offset: usize) -> usize {
    newlines.partition_point(|&newline| newline < offset) + 1
}

fn toml_item(
    item: &toml_edit::Item,
    newlines: &[usize],
    depth: usize,
    remaining: &mut usize,
) -> Source {
    if depth > MAX_DEPTH || *remaining == 0 {
        return Source::default();
    }
    *remaining -= 1;
    let mut node = Source {
        line: item.span().map_or(1, |s| source_line(newlines, s.start)),
        ..Source::default()
    };
    match item {
        toml_edit::Item::None => {}
        toml_edit::Item::Value(value) => return toml_value(value, newlines, depth, remaining),
        toml_edit::Item::Table(table) => {
            node.keys = toml_table(table, newlines, depth, remaining);
        }
        toml_edit::Item::ArrayOfTables(tables) => {
            for table in tables.iter().take(MAX_NODES) {
                if *remaining == 0 {
                    break;
                }
                *remaining -= 1;
                node.items.push(Source {
                    line: table.span().map_or(1, |s| source_line(newlines, s.start)),
                    keys: toml_table(table, newlines, depth + 1, remaining),
                    ..Source::default()
                });
            }
        }
    }
    node
}

fn toml_table(
    table: &toml_edit::Table,
    newlines: &[usize],
    depth: usize,
    remaining: &mut usize,
) -> HashMap<String, Source> {
    let mut keys = HashMap::new();
    if depth > MAX_DEPTH {
        return keys;
    }
    for (key, child) in table.iter().take(MAX_NODES) {
        if *remaining == 0 {
            break;
        }
        let mut source = toml_item(child, newlines, depth + 1, remaining);
        if let Some(span) = table.key(key).and_then(|key| key.span()) {
            source.line = source_line(newlines, span.start);
        }
        keys.insert(key.to_owned(), source);
    }
    keys
}

fn toml_value(
    value: &toml_edit::Value,
    newlines: &[usize],
    depth: usize,
    remaining: &mut usize,
) -> Source {
    let mut node = Source {
        line: value.span().map_or(1, |s| source_line(newlines, s.start)),
        ..Source::default()
    };
    if depth > MAX_DEPTH || *remaining == 0 {
        return node;
    }
    match value {
        toml_edit::Value::Array(array) => {
            for child in array.iter().take(MAX_NODES) {
                if *remaining == 0 {
                    break;
                }
                *remaining = remaining.saturating_sub(1);
                node.items
                    .push(toml_value(child, newlines, depth + 1, remaining));
            }
        }
        toml_edit::Value::InlineTable(table) => {
            for (key, child) in table.iter().take(MAX_NODES) {
                if *remaining == 0 {
                    break;
                }
                *remaining = remaining.saturating_sub(1);
                let mut source = toml_value(child, newlines, depth + 1, remaining);
                if let Some(span) = table.key(key).and_then(|key| key.span()) {
                    source.line = source_line(newlines, span.start);
                }
                node.keys.insert(key.to_owned(), source);
            }
        }
        toml_edit::Value::String(_)
        | toml_edit::Value::Integer(_)
        | toml_edit::Value::Float(_)
        | toml_edit::Value::Boolean(_)
        | toml_edit::Value::Datetime(_) => {}
    }
    node
}
