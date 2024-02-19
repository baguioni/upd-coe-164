use std::io;

fn get_network_from_prefix(prefix: u64, last_digit: u64) -> u64 {
    // TODO: Write a routine that will output the following numbers
    //       given the prefix and the last digit (i.e. first digit
    //       of the unique id)
    // 0 - Invalid prefix
    // 1 - Globe/TM
    // 2 - Smart/Sun/TNT
    // 3 - DITO
    0
}

fn main() {
    let mut str_in = String::new();

    str_in.clear();
    io::stdin()
        .read_line(&mut str_in)
        .expect("Failed to read input");

    let n_testcases: u64 = str_in.trim()
        .parse()
        .expect("Input is not an integer!");

    for t in 1..=n_testcases {
        str_in.clear();
        io::stdin()
            .read_line(&mut str_in)
            .expect("Failed to read input");
        
        let phone_as_int: u64 = str_in.trim()
            .parse()
            .expect("Input is not an integer!");

        // TODO: Write a routine that will print out the mobile number and
        //       network provider according to the specs. Don't forget to use
        //       the get_network_from_prefix() function above!
    }
}