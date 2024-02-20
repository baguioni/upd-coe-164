use core::panic;
use std::i64;
use std::io;

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

    let n_testcases: u64 = str_in.trim().parse().expect("Not an integer!"); 
    let mut taylor_series = TaylorSeries::new();

    for _ in 1..n_testcases + 1{
        str_in.clear();
        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");

        let split_in: Vec <&str> = str_in.splitn(3, ' ').collect();

        let cmd = split_in[0].chars().next().unwrap_or('\0');
        let r_desired: f64 = split_in[1].trim().parse().expect("Not a float!");
        let x: f64 = split_in[2].trim().parse().expect("Not a number!");

        println!("{}", taylor_series.estimate(cmd, x, r_desired))
    }
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
