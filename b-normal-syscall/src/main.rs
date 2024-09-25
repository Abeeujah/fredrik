use std::io;

fn main() {
    let msg = "Hello from normal syscall!\n".to_string();
    let is_windows = cfg!(target_os = "windows");
    match is_windows {
        true => win_syscall(msg).unwrap(),
        false => syscall(msg).unwrap(),
    }
}

#[cfg(target_family = "unix")]
#[link(name = "c")]
extern "C" {
    fn write(fd: u32, buf: *const u8, count: usize) -> i32;
}
fn syscall(msg: String) -> io::Result<()> {
    let msg_ptr = msg.as_ptr();
    let len = msg.len();
    let res = unsafe { write(1, msg_ptr, len) };
    if res == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[link_name = "kernel32"]
extern "system" {
    fn GetStdHandle(nStdHandle: i32) -> i32;
    fn WriteConsoleW(
        hConsoleOutput: i32,
        lpBuffer: *const u16,
        numberOfCharsToWrite: u32,
        lpNumberOfCharsWritten: *mut u32,
        lpReserved: *const std::ffi::c_void,
    ) -> i32;
}
fn win_syscall(msg: String) -> io::Result<()> {
    let msg: Vec<u16> = msg.encode_utf16().collect();
    let msg_ptr = msg.as_ptr();
    let len = msg.len() as u32;
    let mut output: u32 = 0;
    let handle = unsafe { GetStdHandle(-11) };
    if handle == -1 {
        return Err(io::Error::last_os_error());
    }
    let res = unsafe { WriteConsoleW(handle, msg_ptr, len, &mut output, std::ptr::null()) };
    if res == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}
