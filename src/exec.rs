

pub fn measure(exe_path : String, args : Vec<String>, hide_stdout : bool) -> u32 {

    let mut cmd = std::process::Command::new(format!("{exe_path}"));
    cmd.args(args);
    if hide_stdout { cmd.stdout(std::process::Stdio::null()); }

    let start = std::time::Instant::now();
    let _ = cmd.spawn().unwrap().wait_with_output();
    start.elapsed().as_millis() as u32
}