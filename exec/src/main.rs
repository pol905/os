// https://www.microsoft.com/en-us/research/uploads/prod/2019/04/fork-hotos19.pdf

use std::ffi::CString;

use nix::{sys::wait::waitpid,unistd::{self, ForkResult}, libc::sleep};
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
            println!("Executing ls in child...");
            unistd::execv(&CString::new("/bin/ls").unwrap(), &[CString::new("-l").unwrap()]).unwrap();
        },
        Err(err) => {
            println!("Failed to spawn process due to error: {}", err);
        }
    }
}
