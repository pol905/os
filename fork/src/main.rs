// https://www.microsoft.com/en-us/research/uploads/prod/2019/04/fork-hotos19.pdf

use nix::{sys::wait::waitpid,unistd::{fork, ForkResult}, libc::sleep};
fn main() {
    let mut x = 100;

    match unsafe {fork()} {
        Ok(ForkResult::Parent { child }) => {
            // Add line below to see child execute first
            // unsafe {sleep(1)};
            println!("Executing in parent process...");
            println!("Parents' current x value: {x}");
            println!("Updating x value in parent...");
            x = 105;
            println!("Updated x in parent, {x}");
            println!("Wating for child with id: {:?} to exit before exiting", child);
            waitpid(child, None).unwrap();
        },
        Ok(ForkResult::Child) => {
            println!("Childs' current x value: {x}");
            println!("Executing in child process...");
            println!("Updating x value in child...");
            x = 109;
            println!("Updated x in child, {x}");
        },
        Err(err) => {
            println!("Failed to spawn process due to error: {}", err);
        }
    }
}
