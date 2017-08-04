
extern crate time;

use std::env;
use std::process::Command;
//use std::error::Error;

fn main() {
    //timer <COMMAND>



    let cmds = env::args_os();

    let mut cmd = "".to_string();

    let n: u32 = env::args_os().nth(1).map(|x|x.to_str().unwrap().to_string().parse::<u32>().unwrap()).unwrap();

    for k in cmds.skip(2) {
        cmd = cmd + k.to_str().unwrap() + " ";
    }


    let mut status: i32 = 0;


    for i in 0..n {
        let mut command = Command::new(cmd.clone());

        let status_raw = command.status();

        /*
        {
            match ref status_raw {
                ref Ok(ok) => println!("Result: {}", ok.code().unwrap_or(42)),
                ref Err(err) => println!("Result: {}", err.description()),
            }
        }*/

        {
            status = status_raw.map(|x| x.code().unwrap_or(-1)).unwrap_or(-1);
        }

        println!("i={}, Return Status = {}", i, status);
    }

    std::process::exit(status);
}
