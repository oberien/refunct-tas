error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Recv(::std::sync::mpsc::RecvError);
    }
}
