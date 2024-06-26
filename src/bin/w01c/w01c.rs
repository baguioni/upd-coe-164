use std::io;

fn main() {
    let mut str_in = String::new();

    str_in.clear();
    io::stdin()
        .read_line(&mut str_in)
        .expect("Failed to read input");

    let n_testcases: u64 = str_in.trim().parse().expect("Input is not an integer!");

    for t in 1..=n_testcases {
        str_in.clear();
        io::stdin()
            .read_line(&mut str_in)
            .expect("Failed to read input");

        let phone_as_str: String = str_in.trim().to_string();

        let phone_as_int: u64 = str_in.trim().parse().expect("Input is not an integer!");

        let prefix: u64 = (&phone_as_str[0..4]).parse().unwrap();
        let last_digit: u64 = (&phone_as_str[4..5]).parse().unwrap();

        let network_type: u64 = get_network_from_prefix(prefix, last_digit);

        if network_type == 0 {
            println!("Case #{}: {}", t, display_type(network_type));
        } else {
            println!(
                "Case #{}: {} | {}",
                t,
                display_type(network_type),
                display_number(phone_as_int.to_string())
            )
        }
    }
}

// prefix length 4
// last_digit length 1
fn get_network_from_prefix(prefix: u64, last_digit: u64) -> u64 {
    const GLOBE_TM: [u64; 27] = [
        817, 905, 906, 915, 916, 917, 926, 927, 935, 936, 937, 945, 953, 954, 955, 956, 965, 966,
        967, 975, 976, 977, 978, 979, 995, 996, 997,
    ];
    const GLOBE_TM_LONG: [u64; 9] = [9173, 9175, 9176, 9178, 9253, 9255, 9256, 9257, 9258];
    const SMART_SUN_TNT: [u64; 38] = [
        922, 923, 924, 925, 931, 932, 933, 934, 940, 941, 942, 943, 973, 974, 907, 909, 910, 912,
        930, 938, 946, 948, 950, 908, 918, 919, 920, 921, 928, 929, 939, 946, 947, 949, 951, 961,
        998, 999,
    ];
    const DITO: [u64; 8] = [895, 896, 897, 898, 991, 992, 993, 994];

    if GLOBE_TM_LONG.contains(&(last_digit + prefix * 10)) {
        return 1;
    }

    if GLOBE_TM.contains(&prefix) {
        return 1;
    }

    if SMART_SUN_TNT.contains(&prefix) {
        return 2;
    }

    if DITO.contains(&prefix) {
        return 3;
    }

    return 0;
}

fn display_type(network_type: u64) -> String {
    match network_type {
        1 => "Globe/TM".to_string(),
        2 => "Smart/Sun/TNT".to_string(),
        3 => "DITO".to_string(),
        _ => "Invalid".to_string(),
    }
}

fn display_number(phone_as_str: String) -> String {
    let prefix: u64 = (&phone_as_str[..3]).parse().unwrap();
    let uid_left: u64 = (&phone_as_str[3..6]).parse().unwrap();
    let uid_right: u64 = (&phone_as_str[6..]).parse().unwrap();

    format!("+63 {} {} {}", prefix, uid_left, uid_right)
}
