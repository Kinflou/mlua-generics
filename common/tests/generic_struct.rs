// Standard Uses
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

// External Uses
use mlua::{FromLua, Lua, UserData, UserDataMethods, Value};
use mlua::prelude::LuaResult;
use mlua_generics_common::parse_table;
use mlua_generics_common::proxy_table;


struct Foo<B> {
    bar: B
}

struct BarSquare {}
impl UserData for BarSquare {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method(parse_table::DEFAULT_HASH_TYPE_FN_NAME, |_, _, ()| {
            let mut hasher = DefaultHasher::new();
            std::hash::Hash::hash(&std::any::TypeId::of::<Self>(), &mut hasher);
            Ok(hasher.finish())
        });
    }
}
impl<'lua> FromLua<'lua> for BarSquare {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> LuaResult<Self> {

        todo!()
    }
}

// We annotate a UserData impl with the concrete types for each generic that shows up on the impl
// For example, it has one generic: `B`, so we declare the concrete types we want for `B`
// so we would do:
// #[lua_generic(BarSquare)]
//
// If we wanted an async implementation of the constructor for `B`, we would do:
// #[lua_generic(async BarSquare)]
impl<B: 'static> UserData for Foo<B> {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method(parse_table::DEFAULT_HASH_TYPE_FN_NAME, |_, _, ()| {
            let mut hasher = DefaultHasher::new();
            std::hash::Hash::hash(&std::any::TypeId::of::<B>(), &mut hasher);
            Ok(hasher.finish())
        });
    }
}

// Above we made a manual implementation, here is the rest of what it would generated
// by the macro
impl<'lua, B: FromLua<'lua> + 'static> Foo<B> {
    pub fn _lua_register_generic(lua: &'lua Lua) -> LuaResult<()> {
        let kind_name = "Foo";

        parse_table::ensured_parse_table(lua)?;

        // TODO: This key convention is prone to collision in case two types in
        //       different places/namespaces have the same name, find a better way
        //       std::any::type_name::<Self>()
        proxy_table::ensured_kind_table(lua, kind_name.to_owned())?;
        let ctor_table = proxy_table::ensured_kind_ctor_table(lua, kind_name.to_owned())?;

        let mut hasher = DefaultHasher::new();
        std::hash::Hash::hash(&std::any::TypeId::of::<B>(), &mut hasher);
        let hash = std::hash::Hasher::finish(&hasher);
        ctor_table.set(hash, lua.create_function(Self::_lua_generic_ctor).unwrap())?;

        Ok(())
    }

    fn _lua_generic_ctor(lua: &Lua, param0: B) -> LuaResult<Foo<B>> {
        todo!()
    }
}

fn register_foo_impls(lua: &Lua) -> LuaResult<()> {
    // We make a generics table of Foo and and it to globals, so we
    // can create generic instances of `Foo` directly in Lua
    Foo::<BarSquare>::_lua_register_generic(&lua)?;
    Foo::<String>::_lua_register_generic(&lua)?;

    Ok(())
}

#[test]
fn set_foo_constructor_in_globals() {
    let lua = Lua::new();

    // We pass Lua into a function that does all the concrete types registering for us
    register_foo_impls(&lua).unwrap();
}

#[test]
fn script_create_foo_with_concrete_bar_string() {
    let lua = Lua::new();

    // First register Foo into globals as normal
    register_foo_impls(&lua).unwrap();

    // Also register BarSquare
    lua.globals().set(
        "BarSquare", lua.create_function(|_, ()| Ok(BarSquare {})).unwrap()
    ).unwrap();

    // Now lets try to make a Foo instance with a concrete Bar
    lua.load("print(_parse_table.Foo)").exec().unwrap();
    lua.load("print(_parse_table.Foo.new)").exec().unwrap();
    lua.load(
        r#"
                    local square = BarSquare()
                    print(square)

                    local foo = _parse_table.Foo.new(square)
              "#
    ).exec().unwrap();

    // Done!
}
