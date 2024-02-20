use std::io;

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

    let n_resistors: usize = str_in.trim()
        .parse()
        .expect("Invalid number!");

    let mut r_series = 0.0;
    let mut r_parallel = 0.0;

    for _i in 0..n_resistors {
        let mut str_in = String::new();

        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");

        let value: f64 = str_in.trim()
            .parse()
            .expect("Invalid number!");

        r_series += value;
        r_parallel += 1.0/value;
    }

    r_parallel = 1.0/r_parallel;

    println!("{:.4}", r_series);
    println!("{:.4}", r_parallel);
}
