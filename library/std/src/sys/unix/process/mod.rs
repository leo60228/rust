pub use self::process_common::{Command, ExitCode, Stdio, StdioPipes};
pub use self::process_inner::{ExitStatus, Process};
pub use crate::ffi::OsString as EnvKey;

mod process_common;
#[cfg(not(any(target_os = "fuchsia", target_env = "devkita64")))]
#[path = "process_unix.rs"]
mod process_inner;
#[cfg(target_os = "fuchsia")]
#[path = "process_fuchsia.rs"]
mod process_inner;
#[cfg(target_env = "devkita64")]
#[path = "process_dummy.rs"]
mod process_inner;
#[cfg(target_os = "fuchsia")]
mod zircon;
