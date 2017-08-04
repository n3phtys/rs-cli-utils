extern crate time;

mod execute;

use std::env;

fn main() {
    //loop <n> <COMMAND>



    let cmds = env::args_os();

    let mut cmd = "".to_string();

    let n: u32 = env::args_os().nth(1).map(|x| x.to_str().unwrap().to_string().parse::<u32>().unwrap()).unwrap();

    for k in cmds.skip(2) {
        cmd = cmd + k.to_str().unwrap() + " ";
    }


    for i in 0..n {
        let complete_cmd = cmd.to_string();
        let res = execute::execute(complete_cmd);
        println!("{}\nIteration={} finished", res.output, (i + 1));
    }

    std::process::exit(0);
}
