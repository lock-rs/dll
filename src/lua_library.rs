//let lua = Lua::new();

/*       lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();

        let lua_fpslimit = lua_ctx.create_function(|lua_ctx, (fps): (i32) | {
            setfpslimit(fps);
            Ok(())
        })?;

        let lua_print = lua_ctx.create_function(|lua_ctx, text: String | {
          print(text.as_str());
          Ok(())
        })?;
        
        globals.set("setfpslimit", lua_fpslimit)?;
        globals.set("r_print", lua_print)?;

        lua_ctx.load("
        
        setfpslimit(150)
        for i=1,10 do
          r_print('Hello from rlua!')
        end

        ").exec()?;

        Ok(())
      })?;
  */
