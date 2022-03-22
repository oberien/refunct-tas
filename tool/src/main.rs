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

