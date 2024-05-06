#[allow(dead_code)]
pub enum LuaType {
    None,
    Nil,
    Boolean,
    LightUserData,
    Number,
    String,
    Table,
    Function,
    UserData,
    Thread,
    Integer,
}

impl LuaType {
    pub fn type_to_str(&self) -> &str {
        match self {
            LuaType::None => "no value",
            LuaType::Nil => "nil",
            LuaType::Boolean => "boolean",
            LuaType::LightUserData => "light user data",
            LuaType::Number => "number",
            LuaType::String => "string",
            LuaType::Table => "table",
            LuaType::Function => "function",
            LuaType::UserData => "user data",
            LuaType::Thread => "thread",
            LuaType::Integer => "integer",
        }
    }
}
