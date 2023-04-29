// https://www.microsoft.com/en-us/research/uploads/prod/2019/04/fork-hotos19.pdf

use nix::{sys::wait::waitpid,unistd::{fork, ForkResult}, libc::sleep};
use std::{fs::File, io::Write};
fn main() {
    let mut file = 
            File::options()
            .append(true)
            .create(true)
            .open("./multi_process.log").unwrap();

    match unsafe {fork()} {
        Ok(ForkResult::Parent { child }) => {
            // Add line below to see child execute first
            // unsafe {sleep(1)};
            println!("In parent process...");
            println!("PID of child: {:?}", child);
            for i in 0..=10000 {
                write!(&mut file, "parent {i}\n").unwrap();
            }
            // view fd's for the process by running lsof -p comma_separated_pids
            // unsafe {sleep(100)};
            waitpid(child, None).unwrap();
        },
        Ok(ForkResult::Child) => {
            println!("In child process...");
            for i in 0..=10000 {
                write!(&mut file, "child {i}\n").unwrap();
            }
            // view fd's for the process by running lsof -p comma_separated_pids
            // unsafe {sleep(100)};
        },
        Err(err) => {
            println!("Failed to spawn process due to error: {}", err);
        }
    }
}
