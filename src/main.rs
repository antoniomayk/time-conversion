use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'time_conversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn time_conversion(s: &str) -> String {
    let arr = s.split(":").collect::<Vec<&str>>();

    let mut hour = arr[0].parse::<i32>().unwrap();
    let min = arr[1];
    let (secs, fmt) = arr[2].split_at(2);

    hour = match fmt {
        "PM" => {
            if hour == 12 {
                hour
            } else {
                hour + 12
            }
        }
        "AM" => {
            if hour == 12 {
                0
            } else {
                hour
            }
        }
        _ => {
            panic!("Invalid time format")
        }
    };

    format!("{hour:0>2}:{min}:{secs}")
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = time_conversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
