use std::path::Path;
// use std::{process::Command, ffi::OsStr};
use crate::error::Error;
use crate::utils::Color;
use crate::structures::{
    // self,
    Options,
};
#[allow(dead_code)]
pub fn init(opt: Options) -> Options {
    println!("{gn}{bd}Created {rs}new MClang project{rs}", 
                gn=Color::GREEN,
                bd=Color::BOLD,
                rs=Color::RESET
            );
    if !Path::new("./src").is_dir() {
        if let Err(_e) = std::fs::create_dir("./src".clone()) {
            Error::new(Error::UNABLE_TO_MAKE_DIR, Some("./src".into()));
        };
    }
    let _ = std::fs::write("./mclpm.toml", crate::constants::Constants::get_default_config_file());
    let _ = std::fs::write("./src/main.mcl", crate::constants::Constants::get_default_main_file());
    
    opt
}