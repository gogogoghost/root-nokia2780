#[cfg(not(target_os = "android"))]
pub mod env {
    #[macro_export]
    macro_rules! socket_path {
        () => {
            "/tmp/su-daemon"
        };
    }
    #[macro_export]
    macro_rules! shell_path {
        () => {
            "/usr/bin/sh"
        };
    }
}

#[cfg(target_os = "android")]
pub mod env {
    #[macro_export]
    macro_rules! socket_path {
        () => {
            "/dev/socket/su-daemon"
        };
    }
    #[macro_export]
    macro_rules! shell_path {
        () => {
            "/system/bin/sh"
        };
    }
}

pub const PROCESS_EXIT:u8=0x01;
pub const PROCESS_EXIT_BY_SIGNAL:u8=0x02;

pub mod client;
pub mod server;
