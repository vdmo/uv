//! Host services: environment, process identity, terminal queries.

use std::io::IsTerminal;

pub fn host_get_env_utf8(name: &str) -> Option<String> {
    std::env::var(name).ok()
}

pub fn current_host_process_id() -> u64 {
    std::process::id() as u64
}

#[cfg(target_os = "linux")]
pub fn current_host_thread_id() -> u64 {
    // SAFETY: gettid takes no arguments and cannot fail.
    unsafe { libc::syscall(libc::SYS_gettid) as u64 }
}

#[cfg(target_os = "macos")]
pub fn current_host_thread_id() -> u64 {
    let mut tid: u64 = 0;
    // SAFETY: a null thread handle selects the calling thread; `tid` is a valid out pointer.
    unsafe { libc::pthread_threadid_np(0, &mut tid) };
    tid
}

#[cfg(windows)]
pub fn current_host_thread_id() -> u64 {
    extern "system" {
        fn GetCurrentThreadId() -> u32;
    }
    // SAFETY: GetCurrentThreadId has no preconditions.
    unsafe { GetCurrentThreadId() as u64 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostStream {
    Stdout,
    Stderr,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HostTerminalInfo {
    pub is_tty: bool,
    pub ansi_enabled: bool,
    pub width: i32,
}

#[cfg(unix)]
fn terminal_width(stream: HostStream) -> i32 {
    let fd = match stream {
        HostStream::Stdout => libc::STDOUT_FILENO,
        HostStream::Stderr => libc::STDERR_FILENO,
    };
    // SAFETY: `size` is a valid winsize out pointer for TIOCGWINSZ.
    unsafe {
        let mut size: libc::winsize = std::mem::zeroed();
        if libc::ioctl(fd, libc::TIOCGWINSZ, &mut size) == 0 && size.ws_col > 0 {
            return size.ws_col as i32;
        }
    }
    0
}

#[cfg(not(unix))]
fn terminal_width(_stream: HostStream) -> i32 {
    0
}

pub fn query_host_terminal(stream: HostStream) -> HostTerminalInfo {
    let is_tty = match stream {
        HostStream::Stdout => std::io::stdout().is_terminal(),
        HostStream::Stderr => std::io::stderr().is_terminal(),
    };
    if !is_tty {
        return HostTerminalInfo::default();
    }
    HostTerminalInfo { is_tty: true, ansi_enabled: true, width: terminal_width(stream) }
}
