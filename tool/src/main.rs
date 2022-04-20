#[cfg(all(target_os = "linux", not(target_pointer_width = "64")))]
compile_error!("must be compiled as 64bit on Linux (e.g. with `--target x86_64-unknown-linux-gnu`");
#[cfg(all(target_os = "windows", not(target_pointer_width = "32")))]
compile_error!("must be compiled as 32bit on Windows (e.g. with `--target i686-pc-windows-msvc`)");
#[cfg(all(target_os = "macos", not(target_pointer_width = "64")))]
compile_error!("must be compiled as 64bit on macOS (e.g. with `--target x86_64-apple-darwin`");

mod error;
mod tas;
#[cfg(windows)] mod inject;

use std::env;
use std::path::{Path, PathBuf};

use crate::tas::Tas;

fn main() {
    let mut tas;
    #[cfg(windows)]
    {
        // inject dll
        println!("Testing if DLL is already injected");
        match Tas::new() {
            Ok(val) => {
                tas = val;
                println!("DLL already injected.");
            },
            Err(error::Error::CantConnectToRtil) => {
                println!("DLL has not been injected yet, injecting...");
                inject::inject();
                println!("DLL Injected");
                println!("Create tas...");
                tas = Tas::new().unwrap();
                println!("TAS created successfully.");
            }
        }
    }
    #[cfg(unix)]
    {
        println!("Create tas...");
        tas = Tas::new().unwrap();
        println!("TAS created successfully.");
    }
    let script_file = if env::args().len() == 1 {
        if !Path::new("main.re").is_file() {
            panic!("No tas file specified. Usage: refunct-tas <file.re>");
        } else {
            PathBuf::from("main.re")
        }
    } else {
        PathBuf::from(env::args().nth(1).unwrap())
    };
    println!("Executing Script {} ...", script_file.display());
    tas.execute(script_file);
    println!("Script Executed.");
    println!("Finished");
}

