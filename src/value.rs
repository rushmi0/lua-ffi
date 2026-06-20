#[derive(Debug, Clone, uniffi::Enum)]
pub enum LuaValue {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    LuaString(String),
}

impl From<mlua::Value> for LuaValue {
    fn from(v: mlua::Value) -> Self {
        match v {
            mlua::Value::Nil => LuaValue::Nil,
            mlua::Value::Boolean(b) => LuaValue::Boolean(b),
            mlua::Value::Integer(i) => LuaValue::Integer(i),
            mlua::Value::Number(n) => LuaValue::Number(n),
            mlua::Value::String(s) => LuaValue::LuaString(
                s.to_str().map(|bs| bs.to_string()).unwrap_or_default(),
            ),
            _ => LuaValue::Nil,
        }
    }
}