use std::io;

// prefix length 4
// last_digit length 1
fn get_network_from_prefix(prefix: u64, last_digit: u64) -> u64 {
    let globe_tm: [u64; 27] = [ 817, 905, 906, 915, 916, 917, 926, 927, 935, 936, 937, 945, 953, 954,
        955, 956, 965, 966, 967, 975, 976, 977, 978, 979, 995, 996, 997 ];
    let globe_tm_long: [u64; 9] = [
        9173, 9175, 9176, 9178, 9253, 9255, 9256, 9257, 9258
    ];
    let smart_sun_tnt: [u64; 38] = [
        922, 923, 924, 925, 931, 932, 933, 934, 940, 941, 942, 943, 973,
        974, 907, 909, 910, 912, 930, 938, 946, 948, 950, 908, 918, 919,
        920, 921, 928, 929, 939, 946, 947, 949, 951, 961, 998, 999
    ];
    let dito: [u64; 8] = [
        895, 896, 897, 898, 991, 992, 993, 994
    ];

    if dito.contains(&prefix) {
        return 3;
    } 

    if smart_sun_tnt.contains(&prefix) {
        return 2; 
    } 

    if globe_tm.contains(&prefix) {
        return 1;
    } 

    if globe_tm_long.contains(&(last_digit+prefix*10)) {
        return 1;
    }

    return 0;
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

        let phone_as_str: String = phone_as_int.to_string();

        println!("{}", phone_as_int)

        // TODO: Write a routine that will print out the mobile number and
        //       network provider according to the specs. Don't forget to use
        //       the get_network_from_prefix() function above!
    }
}
