// Standard Uses
use mlua::prelude::*;
use mlua::Table;

// Crate Uses

// External Uses


// Strategy 1: `parse_table` with `_parsing_metatable` attached, set on globals

pub fn ensured_parse_table(lua: &Lua) -> LuaResult<Table> {
    let parse_table = lua.globals().get::<_, Table>(DEFAULT_TABLE_NAME);

    match parse_table {
        Ok(table) => Ok(table),
        Err(_) => {
            let parse_table = lua.create_table().unwrap();
            parse_table.set_metatable(Some(create_parsing_metatable(lua)?));

            lua.globals().set(DEFAULT_TABLE_NAME, parse_table)?;

            lua.globals().raw_get(DEFAULT_TABLE_NAME)
        }
    }
}


// Strategy 2: `_parsing_metatable` only on globals which a `parse_table` should refer to

pub fn register_global_parsing_metatable(lua: &Lua) -> LuaResult<()> {
    if lua.globals().get::<_, Table>("_parsing_metatable").is_ok() {
        return Ok(())
    }

    lua.globals().set("_parsing_metatable", create_parsing_metatable(lua)?)
}


// Common functions and items
pub const DEFAULT_TABLE_NAME: &str = "_parse_table";
pub const DEFAULT_TYPE_CTOR_NAME: &str = "new";
pub const DEFAULT_CTOR_PROXY_TABLE_NAME: &str = "ctors";
pub const DEFAULT_HASH_TYPE_FN_NAME: &str = "_type";


fn create_parsing_metatable(lua: &Lua) -> LuaResult<Table> {
    let parsing_metatable = lua.create_table().unwrap();

    parsing_metatable
        .set(
            "__index",
            lua.load(include_str!("_scripts/parsing_metatable.lua"))
                .eval::<mlua::Function>()
                .unwrap(),
        )
        .unwrap();

    Ok(parsing_metatable)
}
