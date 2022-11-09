use std::os::raw::c_int;
// use procfs::process::all_processes;

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
    // FIXME: We already know the pid, and all we need is /proc/pid/comm so this
    // adds unnecessary overhead by iterating through all_processes
    for process in procfs::process::all_processes().unwrap() {
        let this_one = process.unwrap();
        if this_one.pid == pid {
            if let Ok(stat) = this_one.stat() {
                return Ok(stat.comm);
            }    
        }
    }

    Err(ProcNameFail.to_string())
}