use std::{process::Command, ffi::OsStr};

use crate::{
    structures::{
        self,
        Options,
    },
    error::Error,
    utils::Color
};

impl structures::Compiler {
    pub fn simulate(opt: Options) -> Options {
        println!("{yl}{bd}warning{rs}: Using sim mode is not supported, you should use the 'run' subcommand instead", 
                yl=Color::YELLOW,
                bd=Color::BOLD,
                rs=Color::RESET
            );
        let c_path = opt.compiler.path.as_str().clone();
        let mut cmd = Command::new(c_path);
            cmd.arg("sim");
            if opt.compiler.unsaf {
                cmd.arg("--unsafe");
            }
            if opt.verbose == false {
                cmd.arg("-s");
            }
            cmd.arg(opt.compiler.main_path.clone());


            for inc_path in opt.compiler.include_paths.clone() {
                cmd.args(["-I", inc_path.as_str()]);
            }
            let args: Vec<&OsStr> = cmd.get_args().collect::<Vec<&OsStr>>().clone();
            println!("{gn}{bd}running {rs}{prog} {args}", 
                prog=cmd.get_program().to_string_lossy(), 
                args=args.iter().map(|x| x.to_str().unwrap()).collect::<Vec<&str>>().join(" "),
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
                            if msg == "" {
                                continue;
                            }
                            println!("{bl}[{gn}STDOUT{bl}]{rs} {vl}{bd}mclangc{rs}: {}",
                                msg,
                                gn=Color::GREEN,
                                bl=Color::BLUE,
                                bd=Color::BOLD,
                                vl=Color::VIOLET,
                                rs=Color::RESET
                            );
                        }
                        for msg in stderrarr {
                            if msg == "" {
                                continue;
                            }
                            println!("{bl}[{rd}STDERR{bl}]{rs} {vl}{bd}mclangc{rs}: {}",
                                msg,
                                bl=Color::BLUE,
                                rd=Color::RED,
                                vl=Color::VIOLET,
                                bd=Color::BOLD,
                                rs=Color::RESET
                            );
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