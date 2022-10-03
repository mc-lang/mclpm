use crate::error::{
    Error,
    ErrorType,
};

impl Error {
    pub const NOT_ENOUGH_ARGUMENTS: ErrorType<'static> = ErrorType::new("Not enough arguments", 1, true);
    pub const UNKNOWN_ARGUMENT: ErrorType<'static> = ErrorType::new("Unknown argument '{extra}'", 2, true);
    pub const UNABLE_TO_MAKE_DIR: ErrorType<'static> = ErrorType::new("Unable to create directory '{extra}'", 3, true);
    pub const NO_COMPILER: ErrorType<'static> = ErrorType::new("Unable to find the compiler 'mclangc' (Formerly called 'mclang') '{extra}'. Make sure mclangc is installed and in the correct path", 4, true);
    pub const NO_CFG_FILE: ErrorType<'static> = ErrorType::new("Unable to find config file'{extra}'", 5, true);
    pub const CFG_NO_MAIN: ErrorType<'static> = ErrorType::new("Unable to find 'build.main' in config file 'mclpm.toml'", 6, true);
    pub const CFG_NO_NAME: ErrorType<'static> = ErrorType::new("Unable to find 'project.name' in config file 'mclpm.toml'", 7, true);
    pub const CFG_NO_VER: ErrorType<'static> = ErrorType::new("Unable to find 'project.version' in config file 'mclpm.toml'", 8, true);
    pub const CFG_NO_AUTHOR: ErrorType<'static> = ErrorType::new("Unable to find 'project.author' in config file 'mclpm.toml'", 9, true);
}