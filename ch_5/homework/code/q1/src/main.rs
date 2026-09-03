use nix::unistd::{ForkResult, fork};

// Questions:
// 1) What value is `x` in the child process?
// 2) What happens to `x` if both the child and parent change its value?

fn main() {
    let mut x = 100;

    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            // 2) Each process has its own virtual memory, so both of them have
            //    their own independent `x`es.
            x = 88;
            assert!(x == 88);
        }
        Ok(ForkResult::Child) => {
            // 1) It’s 100, obviously.
            assert!(x == 100);

            // 2) The value I assign here have no influence on the variable that
            //    belongs to the parent process, and vice versa.
            x = 98;
            assert!(x == 98);
        }
        Err(_) => println!("Fork failed!"),
    }
}
