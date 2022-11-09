use std::os::raw::c_int;

use std::{error::Error, fmt};

#[derive(Debug)]
struct ProcNameFail;

impl Error for ProcNameFail {}

impl fmt::Display for ProcNameFail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to fetch the process name")
    }
}

pub fn get_os_proc_name(pid: c_int) -> Result<String, String> {
    let process = procfs::process::Process::new(pid).unwrap();

    match process.stat() {
        Ok(stat) => Ok(stat.comm),
        _ => Err(ProcNameFail.to_string()),
    }
}
