pub mod cleaner;
pub mod compiler;
pub mod runner;
pub mod simulator;
pub mod init;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mode {
    None,
    Build,
    Simulate,
    Clean,
    Init
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Runner {
    pub run: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Compiler {
    pub path: String,
    pub compiled_path: String,
    pub main_path: String,
    pub include_paths: Vec<String>,
    pub build_dir: String,
    pub dump_mem: i32,
    pub unsaf: bool
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub author: String,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Options {
    pub mode: Mode,
    pub runner: Runner,
    pub compiler: Compiler,
    pub project: Project,
    pub silent: bool,
    pub verbose: bool,
}



impl Options {
    pub fn new() -> Options {
        Options{
            mode: Mode::None,
            runner: Runner{
                run: false
            },
            compiler: Compiler{
                path: String::new(),
                compiled_path: String::new(),
                main_path: String::new(),
                include_paths: vec![],
                build_dir: String::new(),
                dump_mem: 0,
                unsaf: false
            },
            project: Project {
                name: String::new(),
                version: String::new(),
                author: String::new(),
            },
            silent: false,
            verbose: false
        }
    }
}
