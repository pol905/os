// https://www.microsoft.com/en-us/research/uploads/prod/2019/04/fork-hotos19.pdf

use std::{ffi::CString};

use nix::libc::c_uint;
use nix::{sys::wait::waitpid,unistd::{self,ForkResult}, libc::{self, sleep, STDOUT_FILENO}};
fn main() {

    match unsafe {unistd::fork()} {
        Ok(ForkResult::Parent { child }) => {
            // Add line below to see child execute first
            // unsafe {sleep(1)};
            println!("Executing in parent process...");
            println!("Wating for child with id: {:?} to exit before exiting", child);
            waitpid(child, None).unwrap();
        },
        Ok(ForkResult::Child) => {
            println!("Executing in child process...");
            println!("Counting words in multi_process.log file from prog 2");
            println!("Replacing stdout fd with a file fd...");
            // unistd::close(io::stdout().as_raw_fd().into_raw_fd()).unwrap();
            unistd::close(STDOUT_FILENO).unwrap();
            let _ = unsafe { libc::open(CString::new("p4.output").unwrap().as_ptr(), libc::O_RDWR | libc::O_CREAT | libc::O_TRUNC, libc::S_IRUSR as c_uint) };
            unistd::execvp(&CString::new("/usr/bin/wc").unwrap(), &[CString::new("wc").unwrap(), CString::new("/Users/siddhanth/Documents/os/multi_process.log").unwrap()]).unwrap();
        },
        Err(err) => {
            println!("Failed to spawn process due to error: {}", err);
        }
    }
}
