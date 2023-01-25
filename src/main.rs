use std::io::Write;
use std::str::FromStr;

pub mod types;
pub mod io;
pub mod exec;

fn parse_args() -> Result<(String, u32, bool, bool, Vec<String>), String> {

    let args = std::env::args();
    if std::env::args().nth(1).unwrap() == "-h".to_string() {
        println!(
            "perf_calculator <exePath> -- <exe arguments>\n\r\
             \toptions:\n\r\
             \t\t-h\t\t\t| Displays this list\n\r\
             \t\t--iterations <integer>\t| Set the number of iterations to be performed (default: 10)\n\r\
             \t\t--threaded\t\t| Gives each iteration a separate thread to run on\n\r\
             \t\t--no-capture\t\t| Displays the stdout of the target program\n\r\
             \tother:\n\r\
             \t\tAdd -- <executable arguments> if your program takes specific arguments when invoked\n\r\
             "
        );
        return Err("Displayed the help command".to_string());
    }

    let mut exe_name = "".to_string();
    let mut iterations = 10u32;
    let mut threaded = false;
    let mut no_capture = false;
    let mut exec_args = vec![];

    let mut _iterations_found = false;
    let mut _iterations_complete = false;

    for (arg, index) in std::env::args().zip(0..args.len()) {

        if index == 0 { continue; }

        if index == 1 { exe_name = arg.clone(); continue; }

        if *arg == "--iterations".to_string() { _iterations_found = true; continue; }
        if _iterations_found && !_iterations_complete { _iterations_complete = true; iterations = i32::from_str(arg.as_str()).unwrap() as u32 }

        if *arg == "--threaded".to_string() { threaded = true; continue; }

        if *arg == "--no-capture".to_string() { no_capture = true; continue; }

        if *arg == "--".to_string() { exec_args = args.step_by(index).collect::<Vec<String>>(); break; }
    }

    Ok((exe_name, iterations, threaded, no_capture, exec_args))
}

fn main() {
    if std::env::args().len() >= 2 {

        let (exe_name, nb_iter, threaded, no_capture, exec_args) = match parse_args() {
            Ok(v) => v,
            Err(_) => return,
        };

        if !threaded {
            let mut durations = vec![];
            for _ in 0..nb_iter {
                durations.push(exec::measure(exe_name.clone(), exec_args.clone(), !no_capture));
            }

            write!(std::io::stdout(), "\n\rResults -> \n{}", types::MathObject::new(durations).unwrap()).unwrap();
            return;
        }

        let mut threads = vec![];
        for _ in 0..nb_iter {
            threads.push(std::thread::spawn(
                {
                    let (arg1, arg2, arg3) = (exe_name.clone(), exec_args.clone(), !no_capture);
                    move || {
                        exec::measure(arg1, arg2, arg3)
                    }
                }));
        }

        let mut durations = vec![];
        for thread_handle in threads {
            durations.push(thread_handle.join().unwrap());
        }

        write!(std::io::stdout(), "\n\r{}", types::MathObject::new(durations).unwrap()).unwrap();
        return;

    } else {
        manual_entry();
    }

    io::get_line("\n\rPress any key to continue...");

}

pub fn manual_entry() {
    loop {
        println!("\n\r> New data series input");
        match types::MathObject::new(io::read_vec()) {
            Ok(v) => write!(std::io::stdout(), "\n\r{v}"),
            Err(_) => break,
        }.unwrap();
    }
}