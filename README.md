# Lua编译器结构

## infra基建信息
- 字节码枚举
- Lua值枚举
- 代码关键字信息
- 函数原型信息
- State trait信息
## compiler编译器
- lexer解析
- ast解析
- 可执行信息解析
## undump解析器
- 将二进制文件解析成Proto
## runtime运行时
- 设计LuaState
- 执行ByteCode字节码