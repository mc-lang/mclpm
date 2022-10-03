#![allow(unused_assignments)]
use std::env;
use dirs;
use constants::Constants;
use std::path::Path;
use crate::{
    structures::{
        Compiler,
        Runner,
        // simulator,
        cleaner
    }, 
    // utils::Color
};

mod error;
#[macro_use]
mod utils;
mod constants;
mod structures;
mod config;

fn main() {
    let mut argv: Vec<String> = env::args().collect();
    let _exec = argv.remove(0);

    let mut options = structures::Options::new();
    // * default compiler path
    options.compiler.path = Path::new(&dirs::home_dir().unwrap()).join(".mclang/mclang").to_string_lossy().to_string();
    options.compiler.main_path = "./src/main.mcl".into();

    
    if argv.len() < 1 {
        // ? Possilbly install dependencies??
        println!("{}{rs}",
        Constants::get_help(),
        rs=utils::Color::RESET
    );
    error::Error::new(error::Error::NOT_ENOUGH_ARGUMENTS, None);
    }
    let mut argx = 0;
    while argx < argv.clone().len() {
        let arg = argv[argx].clone();
        match arg.clone().as_str() {
            "run" | "r" => {
                options.mode = structures::Mode::Build;
                options.runner.run = true;
            }
            "build" | "b" => {
                options.mode = structures::Mode::Build;
            }
            "sim" | "s" => {
                options.mode = structures::Mode::Simulate;
            }
            "clean" => {
                options.mode = structures::Mode::Clean;
            }
            "init" => {
                options.mode = structures::Mode::Init;
            }
            "-h" | "--help" | "help" => {
                println!("{}{rs}",
                    Constants::get_help(),
                    rs=utils::Color::RESET
                );
                println!("{gr}Authors: {rs}", 
                    gr = utils::Color::GREEN,
                    rs = utils::Color::RESET
                );
                for a in Constants::get_authors() {
                    println!("    {}", a);
                }
                return;
            }          
            "-v" | "--version" | "version" => {
                println!("MCLPM Version {}{rs}",
                Constants::get_version(),
                rs=utils::Color::RESET
            );
            return;
            }
            "--verbose" => {
                options.verbose = true;
            }
            "-dm" | "--dump-memory" => {
                argx += 1;
                options.compiler.dump_mem = argv[argx].parse().unwrap();
            }
            "--unsafe" => {
                options.compiler.unsaf = true;
            }
            _ => {
                error::Error::new(
                    error::Error::UNKNOWN_ARGUMENT,
                    Some(arg)
                );
            }
            
        }
    argx += 1;
    }   

    if options.mode.clone() == structures::Mode::Init {
        let _ =structures::init::init(options);
        return;
    }
    options = config::parse(options);

    options.compiler.compiled_path = format!("{}/{}", options.compiler.build_dir, options.project.name);

    
    if options.mode.clone() == structures::Mode::Build {
        options = crate::utils::setup_build_env::start(options);
        options = Compiler::compile(options);
        if options.runner.run {
            options = Runner::run(options);
        }
    }else
    if options.mode.clone() == structures::Mode::Simulate {
        options = Compiler::simulate(options);
    } else
    if options.mode.clone() == structures::Mode::Clean {
        options = cleaner::clean(options);
    }

    // dbg!(options);d
}
