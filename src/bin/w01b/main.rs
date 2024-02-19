use std::io;

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

    let n_testcases: u64 = str_in.trim().parse().expect("Not an integer!"); 

    for _ in 1..n_testcases + 1{
        str_in.clear();
        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");

        let split_in: Vec <&str> = str_in.splitn(3, ' ').collect();

        let cmd = split_in[0];
        let r_desired: f64 = split_in[1].trim().parse().expect("Not a float!");
        let x: f64 = split_in[2].trim().parse().expect("Not a number!");
        
        let mut temp: f64 = 0.0;
        let mut factorial: f64 = 1.0; 
        let mut n: f64 = 0.0; 

        // TODO: Find the desired number of iterations n. Make sure to get its
        //       floored value

        // TODO: ... then compute the Taylor approximation of a function
        //       depending on the command.
    }
}