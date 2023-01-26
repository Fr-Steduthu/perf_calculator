use std::io::Write;
use std::str::FromStr;

pub mod types;
pub mod io;
pub mod exec;

fn parse_args() -> Result<(String, u32, bool, bool, bool, Vec<String>), String> {

    let args = std::env::args();
    if std::env::args().nth(1).unwrap() == "-h".to_string() {
        println!(
            "perf_calculator <exePath> -- <exe arguments>\n\r\
             \toptions:\n\r\
             \t\t-h\t\t\t| Displays this list\n\r\
             \t\t--iterations <integer>\t| Set the number of iterations to be performed (default: 10)\n\r\
             \t\t--threaded\t\t| Gives each iteration a separate thread to run on (de-activated for accuracy)\n\r\
             \t\t--md\t\t\t| Outputs as markdown-formatted string representing a table of the results\n\r\
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
    let mut md = false;

    let mut _iterations_found = false;
    let mut _iterations_complete = false;

    for (arg, index) in std::env::args().zip(0..args.len()) {

        if index == 0 { let _ = arg; continue; }

        if index == 1 { exe_name = arg.clone(); continue; } // TODO : Verifier que le chemin existe bien

        if *arg == "--iterations".to_string() { _iterations_found = true; continue; }
        if _iterations_found && !_iterations_complete { _iterations_complete = true; iterations = i32::from_str(arg.as_str()).unwrap() as u32 }

        if *arg == "--threaded".to_string() { threaded = true; continue; }

        if *arg == "--no-capture".to_string() { no_capture = true; continue; }

        if *arg == "--md".to_string() { md = true; continue;}

        if *arg == "--".to_string() { exec_args = args.step_by(index+1).collect::<Vec<String>>(); exec_args.remove(0); break; }

        panic!("Argument n°{index}: {arg} not recognized!")
    }

    Ok((exe_name, iterations, md, false, no_capture, exec_args)) // --threaded desactive
}

fn main() {
    if std::env::args().len() >= 2 { // Entree automatique

        let (exe_name, nb_iter, as_md, threaded, no_capture, exec_args) = match parse_args() {
            Ok(v) => v,
            Err(_) => return,
        };

        let mut durations = vec![];

        //Measurements

        if !threaded {

            for _ in 0..nb_iter {
                durations.push(exec::measure(exe_name.clone(), exec_args.clone(), !no_capture));
            }

        } else {

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

            for thread_handle in threads {
                durations.push(thread_handle.join().unwrap());
            }
        }

        // Display

        if !as_md {
            write!(std::io::stdout(), "\n\r{}", types::MathObject::new(durations).unwrap()).unwrap();
        } else {
            let (result, math) = types::MathObject::new(durations).unwrap().to_table(exec_args);
            let l0 = result.0.join("|");
            let l1 = {
                let mut res = vec!["|".to_string()];
                for _ in 0..result.1.len() {
                    res.push(":---:|".to_string());
                }
                res.join("")
            };
            let l2 = result.1.join("|");

            let l3 = math.0.join("|");
            let l4 = {
                let mut res = vec!["|".to_string()];
                for _ in 0..math.1.len() {
                    res.push(":---:|".to_string());
                }
                res.join("")
            };
            let l5 = math.1.join("|");

            write!(std::io::stdout(),
                   "\n\r|{l0}|\n\r\
                   {l1}\n\r\
                   |{l2}|\n\r\
                   \n\r\
                   |{l3}|\n\r\
                   {l4}\n\r\
                   |{l5}|\
            ").unwrap();
        }

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