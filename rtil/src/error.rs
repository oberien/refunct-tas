error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Recv(::crossbeam_channel::RecvError);
        FromUtf8(::std::string::FromUtf8Error);
    }
}
