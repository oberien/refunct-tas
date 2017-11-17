error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Recv(::std::sync::mpsc::RecvError);
        FromUtf8(::std::string::FromUtf8Error);
    }
}
