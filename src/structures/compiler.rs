use std::{process::Command, ffi::OsStr};
use crate::error::Error;
use crate::utils::Color;
use crate::structures::{
    self,
    Options,
};

impl structures::Compiler {
    pub fn compile(opt: Options) -> Options {
        let c_path = opt.compiler.path.as_str().clone();
        let mut cmd = Command::new(c_path);
            cmd.arg("com");
            if opt.compiler.unsaf {
                cmd.arg("--unsafe");
            }
            cmd.arg("-o");
            cmd.arg(opt.compiler.compiled_path.clone());
            cmd.arg(opt.compiler.main_path.clone());

            for inc_path in opt.compiler.include_paths.clone() {
                cmd.args(["-I", inc_path.as_str()]);
            }
            let args: Vec<&OsStr> = cmd.get_args().collect::<Vec<&OsStr>>().clone();
            println!("{gn}{bd}Compiling{rs} {projectName} {projectVersion} ({prog} {args})", 
                prog=cmd.get_program().to_string_lossy(), 
                args=args.iter().map(|x| x.to_str().unwrap()).collect::<Vec<&str>>().join(" "),
                projectName=opt.project.name,
                projectVersion=opt.project.version,
                gn=Color::GREEN,
                bd=Color::BOLD,
                rs=Color::RESET
            );

            match cmd.output() {
                Ok(o) => {

                        let stdout = String::from_utf8_lossy(&o.stdout);
                        let stdoutarr = stdout.split("\n").map(|x| x.trim());
                        let stderr = String::from_utf8_lossy(&o.stderr);
                        let stderrarr = stderr.split("\n").map(|x| x.trim());
                        
                        for msg in stdoutarr {
                            if opt.verbose == true && msg != "" {
                                println!("{bl}[{gn}STDOUT{bl}]{rs} {vl}{bd}mclangc{rs}: {}",
                                    msg,
                                    gn=Color::GREEN,
                                    bl=Color::BLUE,
                                    bd=Color::BOLD,
                                    vl=Color::VIOLET,
                                    rs=Color::RESET
                                );
                            }
                        }
                        for msg in stderrarr {
                            if opt.silent != true &&  msg != "" {
                                println!("{bl}[{rd}STDERR{bl}]{rs} {vl}{bd}mclangc{rs}: {}",
                                    msg,
                                    bl=Color::BLUE,
                                    rd=Color::RED,
                                    vl=Color::VIOLET,
                                    bd=Color::BOLD,
                                    rs=Color::RESET
                                );
                            }
                        
                    }
                },
                Err(e) => {
                    // println!("{}",String::from_utf8_lossy(&e.stderr));
                    if e.to_string() == "No such file or directory (os error 2)".to_string() {
                        Error::new(Error::NO_COMPILER, Some(c_path.to_string()));
                    }
                    println!("{:?}",e.to_string());
                }
            }
        opt
    }
}