use std::path::Path;

use serde_derive::{
    Serialize,
    Deserialize
};
use toml;

use crate::error::{Error};
// use crate::utils::Color;
use crate::structures::{
    // self,
    Options,
};

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    version: String,
    author: String
}
#[derive(Serialize, Deserialize)]
struct Build {
    main: String,
    compiler: String,
    build_dir: String
}


#[derive(Serialize, Deserialize)]
struct Config {
    project: Project,
    build: Build
}


pub fn parse(mut opt: Options) -> Options {
    
    let file = match std::fs::read_to_string("./mclpm.toml") {
        Ok(o) => {
            o
        }
        Err(_e) => {
            Error::new(Error::NO_CFG_FILE, Some("./mclpm.toml".into()));
            String::new()
        }
    };
    
    let config: Config = toml::from_str(file.as_str()).unwrap();
    if config.build.main == "".to_string() {
        Error::new(Error::CFG_NO_MAIN, None);
    } else {
        opt.compiler.main_path = config.build.main;
    }

    if config.build.compiler != String::new() {
        let path = config.build.compiler;
        opt.compiler.path = path.replace("~", Path::new(&dirs::home_dir().unwrap()).to_str().unwrap())
    }

    if config.build.build_dir != String::new() {
        opt.compiler.build_dir = config.build.build_dir;
    }

    if config.project.name != String::new() {
        opt.project.name = config.project.name;
    } else {
        Error::new(Error::CFG_NO_NAME, None);
    }

    if config.project.version != String::new() {
        opt.project.version = config.project.version;
    } else {
        Error::new(Error::CFG_NO_VER, None);
    }

    if config.project.author != String::new() {
        opt.project.author = config.project.author;
    } else {
        Error::new(Error::CFG_NO_AUTHOR, None);
    }

    opt
}