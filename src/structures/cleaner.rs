use crate::structures::{
    Options,
};

pub fn clean(opt: Options) -> Options{
    let _ = std::fs::remove_dir_all(opt.compiler.build_dir.clone());
    // todo!();
    opt
}