use std::{io::stdout, process};

use nix::{
    libc,
    unistd::{self, ForkResult},
};

fn main() {
    // To make sure the child process prints “hello” first without calling
    // `wait()`, we can use a pipe.
    let (read_end, write_end) = unistd::pipe().unwrap();

    match unsafe { unistd::fork() } {
        Ok(ForkResult::Parent { .. }) => {
            let mut buf = [0u8; 7];
            unistd::read(&read_end, &mut buf).unwrap();
            unistd::close(read_end).unwrap();
            let message = str::from_utf8(&buf).unwrap();
            println!("{message} ({})", process::id());
        }
        Ok(ForkResult::Child) => {
            let message = format!("hello ({})\n", process::id());
            unistd::write(stdout(), message.as_bytes()).ok();
            unistd::write(&write_end, b"goodbye").unwrap();
            unistd::close(write_end).unwrap();
            unsafe { libc::_exit(0) };
        }
        Err(_) => eprintln!("Fork failed!"),
    }
}
