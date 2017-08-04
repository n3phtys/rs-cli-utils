
extern crate time;

use std::env;

mod execute;


fn main() {
    //loop <n> <COMMAND>

    let cmds = env::args_os();

    let mut cmd = "".to_string();

    for k in cmds.skip(1) {
        cmd = cmd + k.to_str().unwrap() + " ";
    }


    let res = execute::execute(cmd);

    let begin_ns :u64 = res.time_begin_ns;



    let time_ns :u64 = res.time_end_ns - begin_ns;


    println!("{}", res.output);

    let time_ms :u64 = time_ns / 1000;

    println!("Command took {} ms (+ {} nanos)", time_ms, (time_ns % 1000));

    std::process::exit(0);
}
