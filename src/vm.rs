use mlua::{Lua, LuaOptions, StdLib as MluaStdLib};

use crate::config::{LuaConfig, LuaStdLib};
use crate::error::LuaError;
use crate::value::LuaValue;

#[derive(uniffi::Object)]
pub struct LuaVm {
    lua: Lua,
}

#[uniffi::export]
impl LuaVm {
    /// Create a VM with all standard libraries loaded.
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self { lua: Lua::new() }
    }

    /// Create a VM with explicit stdlib and options.
    #[uniffi::constructor]
    pub fn with_config(config: LuaConfig) -> Result<Self, LuaError> {
        let stdlib = match config.stdlib {
            LuaStdLib::All => MluaStdLib::ALL_SAFE,
            LuaStdLib::Safe => {
                MluaStdLib::TABLE
                    | MluaStdLib::STRING
                    | MluaStdLib::MATH
                    | MluaStdLib::COROUTINE
            }
            LuaStdLib::None => MluaStdLib::NONE,
        };
        let lua = Lua::new_with(stdlib, LuaOptions::default())
            .map_err(LuaError::from)?;
        Ok(Self { lua })
    }

    /// Execute a script, discarding any return values.
    pub fn exec(&self, script: String) -> Result<(), LuaError> {
        self.lua.load(&script).exec().map_err(LuaError::from)
    }

    /// Execute a script and return the first value.
    pub fn eval(&self, script: String) -> Result<LuaValue, LuaError> {
        let v: mlua::Value = self.lua.load(&script).eval().map_err(LuaError::from)?;
        Ok(LuaValue::from(v))
    }

    /// Set a global variable.
    pub fn set_global(&self, name: String, value: LuaValue) -> Result<(), LuaError> {
        let globals = self.lua.globals();
        match value {
            LuaValue::Nil => globals.set(name, mlua::Value::Nil),
            LuaValue::Boolean(b) => globals.set(name, b),
            LuaValue::Integer(i) => globals.set(name, i),
            LuaValue::Number(n) => globals.set(name, n),
            LuaValue::LuaString(s) => globals.set(name, s),
        }
        .map_err(LuaError::from)
    }

    /// Get a global variable.
    pub fn get_global(&self, name: String) -> Result<LuaValue, LuaError> {
        let v: mlua::Value = self.lua.globals().get(name).map_err(LuaError::from)?;
        Ok(LuaValue::from(v))
    }

    /// Return the Lua version string, e.g. `"Lua 5.4"`.
    pub fn version(&self) -> String {
        self.lua
            .globals()
            .get::<String>("_VERSION")
            .unwrap_or_else(|_| "unknown".to_string())
    }
}