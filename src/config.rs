#[derive(Debug, Clone, uniffi::Enum)]
pub enum LuaStdLib {
    /// Full standard library: io, os, string, math, table, coroutine, utf8, package.
    /// Does not include the `debug` library (mlua refuses to load it via the safe API).
    All,
    /// Sandboxed subset: base, string, math, table, coroutine only.
    /// No io, os, package, or debug.
    Safe,
    /// Bare VM with no standard library
    None,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct LuaConfig {
    pub stdlib: LuaStdLib,
}