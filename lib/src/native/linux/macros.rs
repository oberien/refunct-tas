macro_rules! alignstack_pre {
    () => {{
        asm!(r"
            push rbp
            mov rbp, rsp
            and rsp, 0xfffffffffffffff0
        " :::: "intel");
    }}
}
macro_rules! alignstack_post {
    () => {{
        asm!(r"
            mov rsp, rbp
            pop rbp
        " :::: "intel");
    }}
}
