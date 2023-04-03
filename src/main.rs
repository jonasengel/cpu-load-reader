use std::{thread, time};
use subprocess::{Popen, PopenConfig, Redirection};

fn main() {
    let proc = Popen::create(&["/home/jonas/rust_projects/proc_stat/src/child_task"], PopenConfig {
        stdout: Redirection::None, stdin: Redirection::Pipe, ..Default::default()
    });
    let five_sec = time::Duration::from_secs(60);
    thread::sleep(five_sec);
    println!("this is the parent process ");
    thread::sleep(five_sec);
    proc.expect("REASON").kill().expect("kill didn't work");
}

