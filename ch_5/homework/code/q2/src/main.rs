use std::{
    fs::File,
    os::fd::{BorrowedFd, IntoRawFd},
};

use nix::unistd::{self, ForkResult, fork};

fn main() {
    let fd = File::create("target_file").unwrap().into_raw_fd();
    println!("{fd}");

    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => unsafe {
            let borrowed_fd = BorrowedFd::borrow_raw(fd);
            unistd::write(borrowed_fd, b"TEXT WRITTEN BY PARENT PROCESS").unwrap();
            unistd::close(fd).unwrap();
        },
        Ok(ForkResult::Child) => unsafe {
            let borrowed_fd = BorrowedFd::borrow_raw(fd);
            unistd::write(borrowed_fd, b"text written by child process").unwrap();
            unistd::close(fd).unwrap();
        },
        Err(_) => eprintln!("Forking failed!"),
    }

    // 1. Can both the child and parent access the file descriptor returned by
    //    `open()` system call?
    // — Yes, they can.
    // 2. What happens when they are writing to the file concurrently?
    // — When a process calls `write()`, the shared file offset is increased by
    // the kernel by as much as needed to write the specified bytes. Since the
    // offset is increased atomically, the entire byte sequences of singular
    // `write()` calls are preserved. It is guaranteed that both “TEXT WRITTEN
    // BY PARENT PROCESS” and “text written by child process” will be written
    // fully and properly to the file. What is not guaranteed, though, is the
    // order of the strings themselves, meaning which one will be first.
    // Whichever process calls `write()` sooner writes to the file sooner.
}
