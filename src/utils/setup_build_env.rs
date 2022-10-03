use std::path::Path;

use crate::structures::{
    Options
};

use crate::error::Error;

pub fn start(opt: Options) -> Options {
    let path = opt.compiler.build_dir.clone();
    if !Path::new(path.as_str()).is_dir() {
        if let Err(_e) = std::fs::create_dir(path.clone()) {
            Error::new(Error::UNABLE_TO_MAKE_DIR, Some(path.into()));
        };
    }
    opt
}