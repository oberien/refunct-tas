error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Recv(::std::sync::mpsc::RecvError);
    }

    errors {
        UnknownCommand {
            description("Unknown Command")
            display("Unknown Command")
        }
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
