macro_rules! pushall {
    () => {{
        asm!(r"
            push eax
            push ebx
            push ecx
            push edx
            push esi
            push edi
            push ebp
            sub esp, 0x80
            movdqu [esp+0x70], xmm0
            movdqu [esp+0x60], xmm1
            movdqu [esp+0x50], xmm2
            movdqu [esp+0x40], xmm3
            movdqu [esp+0x30], xmm4
            movdqu [esp+0x20], xmm5
            movdqu [esp+0x10], xmm6
            movdqu [esp], xmm7
        " :::: "intel");
    }}
}
macro_rules! popall {
    () => {{
        asm!(r"
            movdqu xmm7, [esp]
            movdqu xmm6, [esp+0x10]
            movdqu xmm5, [esp+0x20]
            movdqu xmm4, [esp+0x30]
            movdqu xmm3, [esp+0x40]
            movdqu xmm2, [esp+0x50]
            movdqu xmm1, [esp+0x60]
            movdqu xmm0, [esp+0x70]
            add esp, 0x80
            pop ebp
            pop edi
            pop esi
            pop edx
            pop ecx
            pop ebx
            pop eax
        " :::: "intel");
    }}
}
