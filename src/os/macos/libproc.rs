use std::os::raw::c_int;
pub const PROC_PIDPATHINFO_MAXSIZE: u32 = 4096;

extern "C" {
    pub fn proc_name(
        pid: ::std::os::raw::c_int,
        buffer: *mut ::std::os::raw::c_void,
        buffersize: u32,
    ) -> ::std::os::raw::c_int;
}

pub fn get_os_proc_name(pid: c_int) -> Result<String, String> {
    let mut buffer: Vec<u8> = Vec::with_capacity(PROC_PIDPATHINFO_MAXSIZE as usize);

    let size = unsafe {
        proc_name(
            pid,
            buffer.as_mut_ptr() as *mut std::os::raw::c_void,
            buffer.capacity() as u32,
        )
    };

    if size <= 0 {
        Err(std::io::Error::from_raw_os_error(size).to_string())
    } else {
        unsafe {
            buffer.set_len(size as usize);
        }

        match String::from_utf8(buffer) {
            Ok(name) => Ok(name),
            Err(e) => Err(e.to_string()),
        }
    }
}
