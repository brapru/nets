#[cfg(any(target_os = "macos"))]
use crate::os::macos::libproc::*;

pub fn get_proc_name(pid: u32) -> String {
    let retval = get_os_proc_name(i32::try_from(pid).unwrap());

    match retval {
        Ok(name) => name,
        Err(_) => String::from("-"),
    }
}
