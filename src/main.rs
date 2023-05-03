use std::io::Write;
use std::str::FromStr;

pub mod types;
pub mod io;
pub mod exec;

fn parse_args() -> Result<(String, u32, bool, bool, bool, Vec<Vec<String>>), String> {

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
    let mut _threaded = false;
    let mut no_capture = false;
    let mut exec_args = vec![];
    let mut md = false;

    let mut _iterations_found = false;
    let mut _iterations_complete = false;

    for (arg, index) in std::env::args().zip(0..args.len()) {

        if exec_args.len() != 0 { // Capture les arguments
            if *arg == "--".to_string() { exec_args.push(vec![]); continue; }
            exec_args.last_mut().unwrap().push(arg);
            continue;
        }
        
        if index == 0 { let _ = arg; continue; }

        if index == 1 { exe_name = arg.clone(); continue; } // TODO : Verifier que le chemin existe bien

        if *arg == "--iterations".to_string() { _iterations_found = true; continue; }
        if _iterations_found && !_iterations_complete { _iterations_complete = true; iterations = i32::from_str(arg.as_str()).unwrap() as u32 }

        if *arg == "--threaded".to_string() { panic!("De-activated argument --threaded for inaccuracy"); /*threaded = true; continue;*/ }

        if *arg == "--no-capture".to_string() { no_capture = true; continue; }

        if *arg == "--md".to_string() { md = true; continue;}

        /* Exec_args collecting */

        if *arg == "--".to_string() { exec_args.push(vec![]); continue; }


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

        let mut global_durations : Vec<Vec<u32>> = Vec::with_capacity(exec_args.len());


        //Measurements

        if !threaded {
            for (args, i) in exec_args.iter().zip(0..exec_args.len()){
                global_durations.push(vec![]);
                for _ in 0..nb_iter {
                    global_durations[i].push(exec::measure(exe_name.clone(), args.clone(), !no_capture));
                }
            }
        }

        // Display

        if !as_md {
            for iteration_durations in global_durations {
                write!(std::io::stdout(), "\n\r{}", types::MathObject::new(iteration_durations).unwrap()).unwrap();
            }

        } else {
            let (mut header_results, mut header_stats) = ("".to_string(), "".to_string());
            let mut __header_initialized = false;

            let mut count_results = 0;
            let mut count_stats = 0;

            let (mut separator_results, mut separator_stats) = (vec!["|".to_string()], vec!["|".to_string()]);
            let (mut results, mut stats) = (vec![], vec![]);

            /* Results collecting */

            for (iteration_durations, arguments) in global_durations.into_iter().zip(exec_args) {

                let (result, math) = types::MathObject::new(iteration_durations).unwrap().to_table(arguments);

                if !__header_initialized { // Headers collecting
                    __header_initialized = true;
                    header_results = result.0.join("|");
                    header_stats = math.0.join("|");

                    count_results = result.0.len();
                    count_stats = math.0.len();
                }

                results.push(format!("|{}|", result.1.join("|")));
                stats.push(format!("|{}|", math.1.join("|")));
            }

            for _ in 0..count_results {
                separator_results.push(":---:|".to_string());
            }

            for _ in 0..count_stats {
                separator_stats.push(":---:|".to_string());
            }

            let results = results.join("\n\r");
            let stats = stats.join("\n\r");

            let separator_results = separator_results.join("");
            let separator_stats = separator_stats.join("");

            println!("\
            |{header_results}|\n\r\
            {separator_results}\n\r\
            {results}\n\r\
            \n\r\
            |{header_stats}|\n\r\
            {separator_stats}\n\r\
            {stats}\
            ")
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