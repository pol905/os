// https://www.microsoft.com/en-us/research/uploads/prod/2019/04/fork-hotos19.pdf

use nix::{sys::wait,unistd::{self, ForkResult}, libc};
use std::process::exit;
fn main() {
    let mut fds = [0;2];
    let buf = b"Ayyyyyyyy yo boi";
    let length: libc::size_t = buf.len();
    unsafe {
        if libc::pipe(fds.as_mut_ptr()) == -1 {
            println!("Failed to create pipe...");
            exit(1)
        }

        match unistd::fork() {
            Ok(ForkResult::Parent { child }) => {
                println!("In parent process. Sending data to child...");
                libc::close(fds[0]);
                unistd::write(fds[1], buf).unwrap();
                wait::waitpid(child, None).unwrap();
            },
            Ok(ForkResult::Child) => {
                println!("In child process. Parent process says...");
                let mut buf = vec![0; length];
                libc::close(fds[1]);
                let _ = unistd::read(fds[0], &mut buf).unwrap();
                println!("{:?}", String::from_utf8(buf));
            },
            Err(err) => {
                println!("Failed to spawn process due to error: {}", err);
            }
        }
    }
}
