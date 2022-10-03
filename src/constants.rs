#![allow(dead_code)]
pub struct Constants;
use crate::{
    green,
    blue,
    reset,
    violet,
    red,
    
};

impl Constants {
    const VERSION: &'static str = "0.0.1-DEV";
    const AUTHORS: [&'static str; 1] = [
        "MCorange <mcorangecodes@gmail.com>"
    ];
    const HELP: &'static str = concat!(
        green!(), "Usage:", reset!(), " mclpm [subcommand] [flags]\n",
        green!(), "Subcommands:\n", reset!(),
        "    help                     ", violet!(), "=>", reset!(), "    Print this help message\n",
        "    build, b                 ", violet!(), "=>", reset!(), "    Compile the current project\n",
        "    sim, s                   ", violet!(), "=>", reset!(), "    Simulate the current project\n",
        "    run, r                   ", violet!(), "=>", reset!(), "    Compile and run the current project\n",
        "    clean                    ", violet!(), "=>", reset!(), "    Clean the project directory from build artifacts\n",
        "    version                  ", violet!(), "=>", reset!(), "    Print version information\n",
        "    install (",red!(),"UNIMPLEMENTED",reset!(),")  ", violet!(), "=>", reset!(), "    Install a dependency\n",
        "    remove  (",red!(),"UNIMPLEMENTED",reset!(),")  ", violet!(), "=>", reset!(), "    Remove a dependency\n",
        "    update  (",red!(),"UNIMPLEMENTED",reset!(),")  ", violet!(), "=>", reset!(), "    Update a dependency\n",
        "    init                     ", violet!(), "=>", reset!(), "    Create a new mclang project\n",
        blue!(), "\nNote: ", "Replace `[type]` with the provided type\n", reset!(),
        green!(), "Flags:\n", reset!(),
        "    -h, --help               ", violet!(), "=>", reset!(), "    Print this help message\n",
        "    --verbose                ", violet!(), "=>", reset!(), "    Print more to STDOUT\n",
        "    -v, --version            ", violet!(), "=>", reset!(), "    Print the version of mclpm and mclangc(", red!(), "TBD", reset!(), ")\n",
        "    -q, --quiet              ", violet!(), "=>", reset!(), "    MCLPM and mclangc wont print anything, unless an error occurs\n",
        "    -dm, -dump-memory [int]  ", violet!(), "=>", reset!(), "    Dumps the virtual machine memory contents. Only works in simulation mode\n",
        "    --unsafe                 ", violet!(), "=>", reset!(), "    Disables type checking\n",
        ""
    );

    const DEFAULT_CONF: &'static str = concat!(
        "[project]\n",
        "name=\"hello_world\"\n",
        "version=\"0.0.1\"\n",
        "author=\"The MClang team.\"\n\n",
        "[build]\n",
        "main=\"./src/main.mcl\"\n",
        "build_dir=\"./build\"\n",
        "compiler=\"~/.mclang/mclang\"\n",
    );

    const DEFAULT_MAIN: &'static str = concat!(
        "include \"std.mcl\"\n",
        "\"Henlo, world!\" puts"
    );

    pub fn get_version() -> &'static str { Constants::VERSION }
    pub fn get_authors() -> [&'static str; 1] { Constants::AUTHORS }
    pub fn get_help() -> &'static str { Constants::HELP }
    pub fn get_default_config_file() -> &'static str { Constants::DEFAULT_CONF }
    pub fn get_default_main_file() -> &'static str { Constants::DEFAULT_MAIN }
}

