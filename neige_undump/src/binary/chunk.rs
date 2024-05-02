use neige_infra::Prototype;

#[allow(dead_code)]
struct Header {
    signature: [u8; 4],
    version: u8,
    format: u8,
    luac_data: [u8; 6],
    c_int_size: u8,
    c_size_t_size: u8,
    instruction_size: u8,
    lua_integer_size: u8,
    lua_number_size: u8,
    lua_int: i64,
    lua_num: f64,
}

#[allow(dead_code)]
struct BinaryChunk {
    header: Header,
    size_upvalue: u8,
    main_func: Prototype,
}
