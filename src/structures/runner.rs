use std::{process::Command, ffi::OsStr};

use crate::{structures::{
    self,
    Options,
}, utils::Color};

impl structures::Runner {
    pub fn run(opt: Options) -> Options {
        let mut cmd = Command::new(opt.compiler.compiled_path.clone());
        // TODO: Add command arg passthrough
        // cmd.arg("com");
        // cmd.arg("-o");
        // cmd.arg(opt.compiler.compiled_path.clone());
        // cmd.arg(opt.compiler.src_code_path.clone());
        
        for inc_path in opt.compiler.include_paths.clone() {
            cmd.args(["-I", inc_path.as_str()]);
        }
        let args: Vec<&OsStr> = cmd.get_args().collect::<Vec<&OsStr>>().clone();
        
        println!("{bl}{gn}Running{rs} {projectName} {projectVersion} ({execPath} {execArgs})",
            execArgs=args.iter().map(|x| x.to_str().unwrap()).collect::<Vec<&str>>().join(" "),
            execPath=cmd.get_program().to_string_lossy(),
            projectName=opt.project.name,
            projectVersion=opt.project.version,
            gn=Color::GREEN,
            rs=Color::RESET,
            bl=Color::BOLD
        );

            match cmd.output() {
                Ok(o) => {
                    
                        let stdout = String::from_utf8_lossy(&o.stdout);
                        let stdoutarr = stdout.split("\n").map(|x| x.trim());
                        let stderr = String::from_utf8_lossy(&o.stderr);
                        let stderrarr = stderr.split("\n").map(|x| x.trim());
                        
                        for msg in stdoutarr {
                            if msg == "" {
                                continue;
                            }
                            println!("{}",
                                msg,
                                // gn=Color::GREEN,
                                // bl=Color::BLUE,
                                // bd=Color::BOLD,
                                // vl=Color::VIOLET,
                                // rs=Color::RESET
                            );
                        }
                        for msg in stderrarr {
                            if msg == "" {
                                continue;
                            }
                            println!("{}",
                                msg,
                                // bl=Color::BLUE,
                                // rd=Color::RED,
                                // vl=Color::VIOLET,
                                // bd=Color::BOLD,
                                // rs=Color::RESET
                            );
                        }
                },
                Err(e) => {
                    // println!("{}",String::from_utf8_lossy(&e.stderr));
                    println!("{:?}",e.to_string());
                }
            }
        opt
    }
}