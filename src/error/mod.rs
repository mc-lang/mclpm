#![allow(dead_code)]
use crate::utils::Color;
mod types;
pub struct ErrorType<'a>{
    message: &'a str,
    code: i32,
    unrecoverable: bool
}
impl ErrorType<'_> {
    pub const fn new(message: &str, code: i32, unrecoverable: bool) -> ErrorType{
        ErrorType{
            message: message,
            code: code,
            unrecoverable: unrecoverable
        }
    }
}

pub struct Error;
impl Error {
    pub fn new(err: ErrorType, extra: std::option::Option<String>) {
        let mut msg: String = err.message.to_string();
        if extra != None {
            msg = err.message.replace("{extra}", extra.unwrap().as_str());
        }
        // [{ErrCode}] Error: {errMessage}
        println!("{rs}{gr}[{purp}{errCode}{rs}{gr}] {rd}{bd}error{rs}: {errMessage}",
                rs = Color::RESET,
                rd = Color::RED,
                // ul = Color::UNDERLINE,
                bd = Color::BOLD,
                gr = Color::GREEN,
                purp = Color::VIOLET,
                errCode=err.code,
                errMessage=msg
            );

        if err.unrecoverable {
            std::process::exit(err.code);
        }
    }
}
