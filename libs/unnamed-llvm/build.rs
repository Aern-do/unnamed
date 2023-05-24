use std::{env, path::PathBuf, process::Command};

use cc::Build;

fn main() -> anyhow::Result<()> {
    let config_path = env::var("DEP_LLVM_16_CONFIG_PATH").expect("llvm-sys failed");
    let config_path = PathBuf::from(config_path);

    let llvm_config = LLVMConfig::new(config_path);

    let include_dir = llvm_config.get_include_dir()?;

    let cxx_flags = llvm_config.get_cxx_flags()?;
    let cxx_flags = cxx_flags.split_ascii_whitespace();

    let mut cc = Build::new();

    for cxx_flag in cxx_flags {
        cc.flag(cxx_flag);
    }

    cc.cpp(true).warnings(false).include(include_dir).file("src/llvm.cpp").compile("llvm");

    println!("cargo:rerun-if-changed=src/llvm.cpp");

    Ok(())
}

struct LLVMConfig {
    path: PathBuf,
}

impl LLVMConfig {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn run(&self, arg: &'static str) -> anyhow::Result<String> {
        let output = Command::new(&self.path).arg("--link-static").arg(arg).output()?;

        Ok(String::from_utf8(output.stdout)?)
    }

    fn get_include_dir(&self) -> anyhow::Result<PathBuf> {
        Ok(self.run("--includedir")?.into())
    }

    fn get_cxx_flags(&self) -> anyhow::Result<String> {
        self.run("--cxxflags")
    }
}
