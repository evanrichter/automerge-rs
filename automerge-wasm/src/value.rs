use std::borrow::Cow;

use automerge as am;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct ScalarValue<'a>(pub(crate) Cow<'a, am::ScalarValue>);

impl<'a> From<ScalarValue<'a>> for JsValue {
    fn from(val: ScalarValue<'a>) -> Self {
        match &*val.0 {
            am::ScalarValue::Bytes(v) => Uint8Array::from(v.as_slice()).into(),
            am::ScalarValue::Str(v) => v.to_string().into(),
            am::ScalarValue::Int(v) => (*v as f64).into(),
            am::ScalarValue::Uint(v) => (*v as f64).into(),
            am::ScalarValue::F64(v) => (*v).into(),
            am::ScalarValue::Counter(v) => (f64::from(v)).into(),
            am::ScalarValue::Timestamp(v) => js_sys::Date::new(&(*v as f64).into()).into(),
            am::ScalarValue::Boolean(v) => (*v).into(),
            am::ScalarValue::Null => JsValue::null(),
            am::ScalarValue::Unknown { bytes, .. } => Uint8Array::from(bytes.as_slice()).into(),
        }
    }
}

pub(crate) fn datatype(s: &am::ScalarValue) -> String {
    match s {
        am::ScalarValue::Bytes(_) => "bytes".into(),
        am::ScalarValue::Str(_) => "str".into(),
        am::ScalarValue::Int(_) => "int".into(),
        am::ScalarValue::Uint(_) => "uint".into(),
        am::ScalarValue::F64(_) => "f64".into(),
        am::ScalarValue::Counter(_) => "counter".into(),
        am::ScalarValue::Timestamp(_) => "timestamp".into(),
        am::ScalarValue::Boolean(_) => "boolean".into(),
        am::ScalarValue::Null => "null".into(),
        am::ScalarValue::Unknown { type_code, .. } => format!("unknown{}", type_code),
    }
}
