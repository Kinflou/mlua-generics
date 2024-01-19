// Crate Uses
use crate::parse_table;

// External Uses
use mlua::prelude::*;


pub fn ensured_kind_table(lua: &Lua, kind_name: String) -> LuaResult<LuaTable> {
    let parse_table = parse_table::ensured_parse_table(lua)?;

    match parse_table.get(&*kind_name) {
        Ok(t) => Ok(t),
        Err(_) => {
            let kind_table = lua.create_table()?;

            let ctor_fn = format!(
                r#"
                return function(args)
                    local ctor_fn = {}.{}.{}[args]
                    return ctor_fn(args)
                end
                "#,
                parse_table::DEFAULT_TABLE_NAME, kind_name,
                parse_table::DEFAULT_CTOR_PROXY_TABLE_NAME
            );

            let ctor_fn = lua.load(ctor_fn).eval::<mlua::Function>()?;

            kind_table.set(parse_table::DEFAULT_TYPE_CTOR_NAME, ctor_fn)?;

            parse_table.set(&*kind_name, kind_table)?;
            parse_table.get(kind_name)
        }
    }
}


pub fn ensured_kind_ctor_table(lua: &Lua, kind_name: String) -> LuaResult<LuaTable> {
    let parse_table = ensured_kind_table(lua, kind_name)?;

    match parse_table.get(parse_table::DEFAULT_CTOR_PROXY_TABLE_NAME) {
        Ok(t) => Ok(t),
        Err(_) => {
            let ctor_table = lua.create_table()?;
            ctor_table.set_metatable(Some(parse_table::create_parsing_metatable(lua)?));

            parse_table.set(parse_table::DEFAULT_CTOR_PROXY_TABLE_NAME, ctor_table)?;

            parse_table.get(parse_table::DEFAULT_CTOR_PROXY_TABLE_NAME)
        }
    }
}
