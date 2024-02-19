use std::env;
use std::fs::{write, File};
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    if let Ok(lines) = read_lines(config.input_path) {
        let rows = lines.flatten().collect::<Vec<String>>();
        let network_values = NetworkValues::calculate_values(rows);

        let output_string = format!(
            "{:.4}\n{:.4}",
            network_values.series, network_values.parallel
        );

        write_to_output(config.output_path, output_string);
    }
}

fn write_to_output(path: String, data: String) {
    write(path, data).expect("Error writing to file");
}

struct NetworkValues {
    series: f32,
    parallel: f32,
}

impl NetworkValues {
    fn calculate_values(rows: Vec<String>) -> NetworkValues {
        let mut series: f32 = 0.0;
        let mut parallel: f32 = 0.0;
        let mut n: usize = 0;

        for (index, line) in rows.iter().enumerate() {
            if index == 0 {
                n = line.parse().unwrap();
                continue;
            // ensure to use next n-lines
            } else if index <= n {
                let value: f32 = line.parse().unwrap();
                series += &value;
                parallel += 1.0 / &value;
            }
        }

        parallel = 1.0 / parallel;

        NetworkValues { series, parallel }
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
