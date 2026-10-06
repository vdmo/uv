//! Reading and building the JSON of the protocol.

use serde_json::{json, Value};
use uv_tooling::line_index::{LinePosition, LineRange};

pub fn get_str<'a>(object: &'a Value, key: &str) -> Option<&'a str> {
    object.get(key)?.as_str()
}

pub fn get_i64(object: &Value, key: &str) -> Option<i64> {
    object.get(key)?.as_i64()
}

pub fn get_bool(object: &Value, key: &str) -> Option<bool> {
    object.get(key)?.as_bool()
}

pub fn nested<'a>(root: Option<&'a Value>, path: &[&str]) -> Option<&'a Value> {
    let mut current = root?;
    for key in path {
        current = current.get(*key)?.as_object().and_then(|_| current.get(*key))?;
    }
    Some(current)
}

pub fn position_json(position: LinePosition) -> Value {
    json!({ "line": position.line as i64, "character": position.character as i64 })
}

pub fn range_json(range: LineRange) -> Value {
    json!({ "start": position_json(range.start), "end": position_json(range.end) })
}

pub fn text_document_uri(params: Option<&Value>) -> Option<String> {
    get_str(params?.get("textDocument")?, "uri").map(str::to_string)
}

pub fn position_param(params: Option<&Value>) -> Option<LinePosition> {
    let position = params?.get("position")?;
    let line = get_i64(position, "line")?;
    let character = get_i64(position, "character")?;
    if line < 0 || character < 0 {
        return None;
    }
    Some(LinePosition { line: line as usize, character: character as usize })
}
