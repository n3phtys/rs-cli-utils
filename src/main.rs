
extern crate time;

use std::env;
use std::process::Command;

fn main() {

    let cmds = env::args_os();

    let mut cmd = "".to_string();

    for k in cmds.skip(1) {
        //println!("{}", k.to_str().unwrap());
        cmd = cmd + k.to_str().unwrap() + " ";
    }



    //println!("Hello, world!");
    //println!("Read args: {}", cmd);

    let begin_ns :u64 = time::precise_time_ns();

    let status_raw = Command::new(cmd)
        .status();

    let time_ns :u64 = time::precise_time_ns() - begin_ns;

    let status = status_raw.map(|x|x.code().unwrap_or(-1)).unwrap_or(-1);

    let time_ms :u64 = time_ns / 1000;

    println!("Command took {} ms (+ {} nanos)", time_ms, (time_ns % 1000));

    std::process::exit(status);
}
