use time;
use std;
use std::process::Command;

pub struct CmdResult {
    pub time_begin_ns: u64,
    pub time_end_ns: u64,
    pub output: String,
}

pub fn execute(cmd: String) -> CmdResult {
    if cfg!(target_os = "windows") {
        let argsarr = ["/C", &cmd];
        let mut raw_cmd = Command::new("cmd");
        let command = raw_cmd
            .args(&argsarr);
        let begin_ns: u64 = time::precise_time_ns();
        let output = command
            .output()
            .expect("failed to execute process");
        let end_ns: u64 = time::precise_time_ns();

        let stdout_of_child = output.stdout;

        return CmdResult { time_begin_ns: begin_ns, time_end_ns: end_ns, output: std::str::from_utf8(&stdout_of_child).unwrap().to_string() };
    } else {
        let mut raw_cmd = Command::new("sh");
        let command = raw_cmd
            .arg("-c")
            .arg(&cmd);
        let begin_ns: u64 = time::precise_time_ns();
        let output = command
            .output()
            .expect("failed to execute process");
        let end_ns: u64 = time::precise_time_ns();


        let stdout_of_child = output.stdout;


        return CmdResult { time_begin_ns: begin_ns, time_end_ns: end_ns, output: std::str::from_utf8(&stdout_of_child).unwrap().to_string() };
    }
}