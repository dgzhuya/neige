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

impl From<i8> for LuaType {
    fn from(value: i8) -> Self {
        match value {
            -1 => LuaType::None,
            _ => panic!("type error"),
        }
    }
}
