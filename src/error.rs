use mlua::Error as MluaError;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum LuaError {
    #[error("{msg}")]
    Syntax { msg: String },
    #[error("{msg}")]
    Runtime { msg: String },
    #[error("{msg}")]
    Other { msg: String },
}

impl From<MluaError> for LuaError {
    fn from(e: MluaError) -> Self {
        match e {
            MluaError::SyntaxError { message, .. } => LuaError::Syntax { msg: message },
            MluaError::RuntimeError(msg) => LuaError::Runtime { msg },
            other => LuaError::Other { msg: other.to_string() },
        }
    }
}