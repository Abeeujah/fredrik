use std::arch::asm;

fn main() {
    syscall("Hello world from raw syscall!\n".to_string());
}

#[inline(never)]
fn syscall(msg: String) {
    let msg_ptr = msg.as_ptr();
    let len = msg.len();
    unsafe {
        asm!(
            "mov rax, 1",
            "mov rdi, 1",
            "syscall",
            in("rsi") msg_ptr,
            in("rdx") len,
            out("rax") _,
            out("rdi") _,
            lateout("rsi") _,
            lateout("rdx") _
        );
    }
}
