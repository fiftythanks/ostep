use std::io::stdout;

use nix::{
    sys,
    unistd::{self, ForkResult},
};

fn main() {
    // Execv
    match unsafe { unistd::fork() } {
        Ok(ForkResult::Parent { .. }) => {
            sys::wait::wait().unwrap();
        }
        Ok(ForkResult::Child) => {
            let msg = format!("`ls` call from child {}:\n", std::process::id());
            unistd::write(stdout(), msg.as_bytes()).ok();
            unistd::execv(c"/bin/ls", &[c"/bin/ls", c"-l", c"-a"]).ok();

            // If this line is reached, something went wrong.
            std::process::exit(1);
        }
        Err(_) => eprintln!("Fork failed! (execv)"),
    }

    // Execve
    match unsafe { unistd::fork() } {
        Ok(ForkResult::Parent { .. }) => {
            sys::wait::wait().unwrap();
        }
        Ok(ForkResult::Child) => {
            let msg = format!("\n`ls` call from child {}:\n", std::process::id());
            unistd::write(stdout(), msg.as_bytes()).ok();
            unistd::execve(
                c"/bin/ls",
                &[c"/bin/ls", c"-l", c"-a"],
                &[c"HOME=/home/saprilonty/"],
            )
            .ok();

            // If this line is reached, something went wrong.
            std::process::exit(1);
        }
        Err(_) => eprintln!("Fork failed!"),
    }
}
