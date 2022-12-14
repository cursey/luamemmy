use memmy::*;
use mlua::prelude::*;

#[mlua::lua_module]
fn luamemmy(lua: &Lua) -> LuaResult<LuaTable> {
    let mem = lua.create_table()?;

    mem.set(
        "abs",
        lua.create_function(|_, address: usize| Ok(abs(address)))?,
    )?;

    mem.set(
        "xref",
        lua.create_function(|_, (start, len, address): (usize, usize, usize)| {
            Ok(xref(span(start, len), address))
        })?,
    )?;

    mem.set(
        "xrefs",
        lua.create_function(|_, (start, len, address): (usize, usize, usize)| {
            Ok(xrefs(span(start, len), address))
        })?,
    )?;

    mem.set(
        "scan",
        lua.create_function(|_, (start, len, pat): (usize, usize, String)| {
            Ok(scan(span(start, len), pat.as_str()))
        })?,
    )?;

    // rscan
    mem.set(
        "rscan",
        lua.create_function(|_, (start, len, pat): (usize, usize, String)| {
            Ok(rscan(span(start, len), pat.as_str()))
        })?,
    )?;

    mem.set(
        "scan_all",
        lua.create_function(|_, (start, len, pat): (usize, usize, String)| {
            Ok(scan_all(span(start, len), pat.as_str()))
        })?,
    )?;

    mem.set(
        "scan_str",
        lua.create_function(|_, (start, len, pat): (usize, usize, String)| {
            Ok(scan_str(span(start, len), pat.as_str()))
        })?,
    )?;

    // rscan_str
    mem.set(
        "rscan_str",
        lua.create_function(|_, (start, len, pat): (usize, usize, String)| {
            Ok(rscan_str(span(start, len), pat.as_str()))
        })?,
    )?;

    mem.set(
        "scan_all_str",
        lua.create_function(|_, (start, len, pat): (usize, usize, String)| {
            Ok(scan_all_str(span(start, len), pat.as_str()))
        })?,
    )?;

    // read_i8
    mem.set(
        "read_i8",
        lua.create_function(|_, address: usize| Ok(read_i8(address)))?,
    )?;

    // read_i16
    mem.set(
        "read_i16",
        lua.create_function(|_, address: usize| Ok(read_i16(address)))?,
    )?;

    // read_i32
    mem.set(
        "read_i32",
        lua.create_function(|_, address: usize| Ok(read_i32(address)))?,
    )?;

    // read_i64
    mem.set(
        "read_i64",
        lua.create_function(|_, address: usize| Ok(read_i64(address)))?,
    )?;

    // read_f32
    mem.set(
        "read_f32",
        lua.create_function(|_, address: usize| Ok(read_f32(address)))?,
    )?;

    // read_f64
    mem.set(
        "read_f64",
        lua.create_function(|_, address: usize| Ok(read_f64(address)))?,
    )?;

    // read_u8
    mem.set(
        "read_u8",
        lua.create_function(|_, address: usize| Ok(read_u8(address)))?,
    )?;

    // read_u16
    mem.set(
        "read_u16",
        lua.create_function(|_, address: usize| Ok(read_u16(address)))?,
    )?;

    // read_u32
    mem.set(
        "read_u32",
        lua.create_function(|_, address: usize| Ok(read_u32(address)))?,
    )?;

    // read_u64
    mem.set(
        "read_u64",
        lua.create_function(|_, address: usize| Ok(read_u64(address)))?,
    )?;

    // write_i8
    mem.set(
        "write_i8",
        lua.create_function(|_, (address, value): (usize, i8)| Ok(write_i8(address, value)))?,
    )?;

    // write_i16
    mem.set(
        "write_i16",
        lua.create_function(|_, (address, value): (usize, i16)| Ok(write_i16(address, value)))?,
    )?;

    // write_i32
    mem.set(
        "write_i32",
        lua.create_function(|_, (address, value): (usize, i32)| Ok(write_i32(address, value)))?,
    )?;

    // write_i64
    mem.set(
        "write_i64",
        lua.create_function(|_, (address, value): (usize, i64)| Ok(write_i64(address, value)))?,
    )?;

    // write_f32
    mem.set(
        "write_f32",
        lua.create_function(|_, (address, value): (usize, f32)| Ok(write_f32(address, value)))?,
    )?;

    // write_f64
    mem.set(
        "write_f64",
        lua.create_function(|_, (address, value): (usize, f64)| Ok(write_f64(address, value)))?,
    )?;

    // write_u8
    mem.set(
        "write_u8",
        lua.create_function(|_, (address, value): (usize, u8)| Ok(write_u8(address, value)))?,
    )?;

    // write_u16
    mem.set(
        "write_u16",
        lua.create_function(|_, (address, value): (usize, u16)| Ok(write_u16(address, value)))?,
    )?;

    // write_u32
    mem.set(
        "write_u32",
        lua.create_function(|_, (address, value): (usize, u32)| Ok(write_u32(address, value)))?,
    )?;

    // write_u64
    mem.set(
        "write_u64",
        lua.create_function(|_, (address, value): (usize, u64)| Ok(write_u64(address, value)))?,
    )?;

    // module
    mem.set(
        "module",
        lua.create_function(|_, name: String| Ok(module(name.as_str())))?,
    )?;

    // module_span
    mem.set(
        "module_span",
        lua.create_function(|lua, name: String| match module_span(name.as_str()) {
            Some(span) => (span.as_ptr() as usize, span.len()).to_lua_multi(lua),
            None => Ok(mlua::MultiValue::new()),
        })?,
    )?;

    // module_len
    mem.set(
        "module_len",
        lua.create_function(|_, name: String| match module_span(name.as_str()) {
            Some(m) => Ok(Some(m.len())),
            None => Ok(None),
        })?,
    )?;

    // module_section
    mem.set(
        "module_section",
        lua.create_function(|lua, (name, section): (String, String)| match module_section(name.as_str(), section.as_str()) {
            Some(section) => (section.as_ptr() as usize, section.len()).to_lua_multi(lua),
            None => Ok(mlua::MultiValue::new()),
        })?,
    )?;

    Ok(mem)
}

#[cfg(test)]
mod test {
    use super::*;
    use mlua::Lua;

    fn register(lua: &Lua) -> mlua::Result<()> {
        lua.globals().set("mem", luamemmy(lua)?)
    }

    #[test]
    fn mem_abs() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let data: &[u8] = &[0x12, 0x34, 0x56, 0x78];
        let a = data.as_ptr() as usize;
        let script = format!("return mem.abs(0x{:x})", a);

        assert_eq!(
            lua.load(script.as_str()).eval::<usize>().unwrap(),
            a + 0x78563416
        );
    }

    #[test]
    fn mem_xref() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let data: &[u8] = &[1, 2, 3, 4, 0x12, 0x34, 0x56, 0x78];
        let a = data.as_ptr() as usize;
        let script = format!(
            "return mem.xref(0x{:x}, {}, 0x{:x} + 4 + 0x78563416)",
            a,
            data.len(),
            a,
        );

        assert_eq!(
            lua.load(script.as_str()).eval::<Option<usize>>().unwrap(),
            Some(a + 4)
        );
    }

    #[test]
    fn mem_xrefs() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let data: &[u8] = &[1, 2, 3, 4, 0x12, 0x34, 0x56, 0x78];
        let a = data.as_ptr() as usize;
        let script = format!(
            "return mem.xrefs(0x{:x}, {}, 0x{:x} + 4 + 0x78563416)",
            a,
            data.len(),
            a,
        );

        assert_eq!(
            lua.load(script.as_str()).eval::<Vec<usize>>().unwrap(),
            vec![a + 4]
        );
    }

    #[test]
    fn mem_scan() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let data: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf];
        let script = format!(
            r#"return mem.scan(0x{:x}, {}, "0a ? 0C")"#,
            data.as_ptr() as usize,
            data.len()
        );

        assert_eq!(
            lua.load(script.as_str()).eval::<Option<usize>>().unwrap(),
            Some(data.as_ptr() as usize + 10)
        );
    }

    #[test]
    fn mem_scan_all() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let data: &[u8] = &[0, 1, 2, 3, 42, 77, 0, 1, 2, 3, 5, 6, 7];
        let script = format!(
            r#"return mem.scan_all(0x{:x}, {}, "00 ? ? 03")"#,
            data.as_ptr() as usize,
            data.len()
        );

        assert_eq!(
            lua.load(script.as_str()).eval::<Vec<usize>>().unwrap(),
            vec![data.as_ptr() as usize + 0, data.as_ptr() as usize + 6]
        );
    }

    #[test]
    fn mem_scan_str() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let data = "Hello, world!";
        let script = format!(
            r#"return mem.scan_str(0x{:x}, {}, "world")"#,
            data.as_ptr() as usize,
            data.len()
        );

        assert_eq!(
            lua.load(script.as_str()).eval::<Option<usize>>().unwrap(),
            Some(data.as_ptr() as usize + 7)
        );
    }

    #[test]
    fn mem_scan_all_str() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let data = "Hello, world! Hello, moon!";
        let script = format!(
            r#"return mem.scan_all_str(0x{:x}, {}, "Hello")"#,
            data.as_ptr() as usize,
            data.len()
        );

        assert_eq!(
            lua.load(script.as_str()).eval::<Vec<usize>>().unwrap(),
            vec![data.as_ptr() as usize + 0, data.as_ptr() as usize + 14]
        );
    }

    #[test]
    fn mem_module() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let script = r#"return mem.module("kernel32")"#;

        assert!(lua.load(script).eval::<Option<usize>>().unwrap().is_some());

        let script = r#"return mem.module("nonexistent")"#;

        assert!(lua.load(script).eval::<Option<usize>>().unwrap().is_none());
    }

    #[test]
    fn mem_module_span() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let script = r#"return mem.module_span("kernel32")"#;
        let result = lua.load(script).eval::<(usize, usize)>();

        assert!(result.is_ok());

        let (start, len) = result.unwrap();

        assert!(start > 0);
        assert!(len > 0);

        let script = r#"return mem.module_span("nonexistent")"#;

        assert!(lua.load(script).eval::<(usize, usize)>().is_err());
    }

    #[test]
    fn mem_module_len() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let script = r#"return mem.module_len("kernel32")"#;

        assert!(lua.load(script).eval::<Option<usize>>().unwrap().is_some());

        let script = r#"return mem.module_len("nonexistent")"#;

        assert!(lua.load(script).eval::<Option<usize>>().unwrap().is_none());
    }

    #[test]
    fn mem_module_section() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let script = r#"return mem.module_section("kernel32", ".text")"#;
        let result = lua.load(script).eval::<(usize, usize)>();

        assert!(result.is_ok());

        let (start, len) = result.unwrap();

        assert!(start > 0);
        assert!(len > 0);

        let script = r#"return mem.module_section("kernel32", ".asdf")"#;

        assert!(lua.load(script).eval::<(usize, usize)>().is_err());

        let script = r#"return mem.module_section("nonexistent", ".text")"#;

        assert!(lua.load(script).eval::<(usize, usize)>().is_err());
    }
}
