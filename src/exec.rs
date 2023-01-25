
pub fn measure(exe_path : String, args : Vec<String>, hide_stdout : bool) -> u32 {

    let mut cmd = std::process::Command::new(format!("{exe_path}"));
    cmd.args(args);
    if hide_stdout { cmd.stdout(std::process::Stdio::null()); }

    let process = cmd.spawn().unwrap();

    let mut _result = Err(std::io::Error::from(std::io::ErrorKind::Interrupted)); //Initialisation sans importance

    let start = std::time::Instant::now();
    _result = process.wait_with_output();
    start.elapsed().subsec_millis()
}