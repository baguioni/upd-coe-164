use core::panic;
use std::env;
use std::fs::{write, File};
use std::i64;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("Input path: {}", config.input_path);
    let mut taylor_series = TaylorSeries::new();

    if let Ok(lines) = read_lines(config.input_path) {
        let rows = lines.flatten().collect::<Vec<String>>();
        let mut output_string = String::new();
        let mut n: usize = 0;

        for (index, line) in rows.iter().enumerate() {
            if index == 0 {
                n = line.parse().unwrap();
                continue;
            // ensure to use next n-lines
            } else if index <= n {
                let text: String = line.parse().unwrap();
                let values: Vec<&str> = text.split(" ").collect();
                let (fn_type, r, x) = (
                    values[0].chars().next().unwrap_or('\0'),
                    values[1].parse::<f64>(),
                    values[2].parse::<f64>(),
                );

                output_string.push_str(&format!(
                    "Function #{}: {:.2}\n",
                    index,
                    taylor_series.estimate(fn_type, x.unwrap(), r.unwrap())
                ));
            }
        }

        write_to_output(config.output_path, output_string);
    }
}

fn write_to_output(path: String, data: String) {
    write(path, data).expect("Error writing to file");
}

struct TaylorSeries {}

impl TaylorSeries {
    fn new() -> TaylorSeries {
        TaylorSeries {}
    }

    fn natural_exponent(&mut self, x: f64, n: i64) -> f64 {
        f64::powf(x, n as f64) / factorial(n)
    }

    fn sin(&mut self, x: f64, n: i64) -> f64 {
        f64::powf(x, (2 * n + 1) as f64) * f64::powf(-1.0, n as f64) / factorial(2 * n + 1)
    }

    fn cos(&mut self, x: f64, n: i64) -> f64 {
        f64::powf(x, (2 * n) as f64) * f64::powf(-1.0, n as f64) / factorial(2 * n)
    }

    fn estimate(&mut self, fn_type: char, x: f64, r: f64) -> f64 {
        let mut n = 0;
        let mut sum: f64 = 0.0;

        // From constraints
        while n < 21 {
            let value: f64 = match fn_type {
                'e' => self.natural_exponent(x, n),
                's' => self.sin(x, n),
                'c' => self.cos(x, n),
                _ => panic!("Function type was not indicated"),
            };

            sum += value;

            if value.abs() <= r {
                return sum;
            }

            n += 1;
        }
        sum
    }
}

fn factorial(n: i64) -> f64 {
    match n {
        0 => 1.0,
        _ => (1..=n).map(|x| x as f64).product(),
    }
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#creating-a-constructor-for-config
struct Config {
    input_path: String,
    output_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        // we can add error handling here next time
        let input_path = args[1].clone();
        let output_path = args[2].clone();

        Config {
            input_path,
            output_path,
        }
    }
}
