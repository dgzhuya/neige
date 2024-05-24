use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Display for LuaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LuaType::None => write!(f, "-1"),
            LuaType::Nil => write!(f, "0"),
            LuaType::Boolean => write!(f, "1"),
            LuaType::LightUserData => write!(f, "2"),
            LuaType::Number => write!(f, "3"),
            LuaType::String => write!(f, "4"),
            LuaType::Table => write!(f, "5"),
            LuaType::Function => write!(f, "6"),
            LuaType::UserData => write!(f, "7"),
            LuaType::Thread => write!(f, "8"),
            LuaType::Integer => write!(f, "19"),
        }
    }
}
