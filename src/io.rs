use std::io::Write;
use std::str::FromStr;

const IS_EOL: fn(char) -> bool = |c| {c=='\n' || c == '\r'};
const SEPARATORS: fn(char) -> bool = |c| { c == '\n' || c == '\r' || c == ';' || c== '|' || c == ',' };

pub fn get_line(message : &str) -> String {
    std::io::stdout().write(message.as_bytes()).unwrap();
    std::io::stdout().flush().unwrap();

    let mut s = "".to_string();
    std::io::stdin().read_line(&mut s).unwrap();

    s
}

fn is_end_of_input(s: &String) -> bool {
    return match s.split(IS_EOL)
        .collect::<String>()
        .split_whitespace()
        .collect::<String>()
        .as_ref()
    {
        "s" | "stop" | "end" | "yes" | "Yes" | "Y" | "y" | "" => true,
        _ => false,
    };
}

pub fn read_vec() -> Vec<u32> {
    let mut values = vec![];
    loop {
        let s = get_line("Enter values: ");
        let is_md = s.contains('|');

        if is_end_of_input(&s) { break; }

        match &mut parse_as_vec(s) {
            Ok(v) => values.append(v),
            Err(e) => {
                std::io::stdout()
                    .write(e.as_bytes())
                    .unwrap()
                ;
                if is_md {
                    break;
                }
                continue;
            }
        };
    }

    values
}

fn parse_as_vec(s: String) -> Result<Vec<u32>, String> {
    let mut temp = vec![];

    for num in s
        .split(SEPARATORS)
        .collect::<String>()
        .split_whitespace()
    {
        if num == "".to_string() {
            continue;
        }
        temp.push(u32::from_str(num).unwrap());
    }

    return Ok(temp);
}

