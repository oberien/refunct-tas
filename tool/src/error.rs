error_chain! {
    foreign_links {
        Gdb(::gdb::Error);
    }
}

macro_rules! handle_err {
    ($e: expr) => {{
        match $e {
            Err(ref e) => {
                println!("error: {}", e);

                for e in e.iter().skip(1) {
                    println!("caused by: {}", e);
                }

                if let Some(backtrace) = e.backtrace() {
                    println!("backtrace: {:?}", backtrace);
                }

                print!("\x1b[0m");
                std::process::exit(1);
            },
            Ok(e) => e
        }
    }}
}
