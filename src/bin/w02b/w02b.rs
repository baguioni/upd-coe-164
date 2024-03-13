use std::io;

struct StudentEnrollInfo {
    sn: u64,
    is_eligible: bool,
    has_accountables: bool,
    has_taken_ge2017: [bool; 10],
}
impl StudentEnrollInfo {
    fn new() -> Self {
        Self {
            sn: 0,
            is_eligible: false,
            has_accountables: true,
            has_taken_ge2017: [false; 10],
        }
    }

    fn check_ge2017(&mut self, course: String) -> bool {
        // HINT: Create a routine that edits self.has_taken_ge2017
        //       depending on the course argument
        let ge_2017 = [
            "arts1", "fil40", "kas1", "philo1", "eng13", "speech30", "sts1", "drmaps", "socsci2",
            "socsci1",
        ];

        // get the position of course in ge_2017
        // should handle case course is not in ge_2017

        let pos = ge_2017.iter().position(|&x| x == course);

        if let Some(pos) = pos {
            // check if course has been taken
            if self.has_taken_ge2017[pos] {
                return false;
            }
            self.has_taken_ge2017[pos] = true;

            if course == "eng13" {
                self.has_taken_ge2017[5] = true;
            } else if course == "speech30" {
                self.has_taken_ge2017[4] = true;
            }

            if course == "drmaps" {
                self.has_taken_ge2017[6] = true;
            } else if course == "sts1" {
                self.has_taken_ge2017[7] = true;
            }

            if course == "socsci2" {
                self.has_taken_ge2017[8] = true;
            } else if course == "socsci1" {
                self.has_taken_ge2017[9] = true;
            }

            return true;
        }
        return false;
    }

    fn print_unsatisfied_ge2017(&self) -> bool {
        // HINT: Create a routine that prints the state of self.has_taken_ge2017
        //       according to the specs
        let ge_2017 = [
            "arts1", "fil40", "kas1", "philo1", "eng13", "speech30", "sts1", "drmaps", "socsci2",
            "socsci1",
        ];

        let mut checklist_satistfied = true;

        let mut output_string = Vec::new();
        for (i, course) in ge_2017.iter().enumerate() {
            if i == 4 || i == 6 || i == 8 {
                continue;
            }

            if !self.has_taken_ge2017[i] {
                if i == 5 {
                    //print in all caps
                    output_string.push("ENG13/SPEECH30 ".to_string());
                } else if i == 7 {
                    output_string.push("STS1/DRMAPS ".to_string());
                } else if i == 9 {
                    output_string.push("SOCSCI2/SOCSCI1 ".to_string());
                } else {
                    output_string.push(format!("{} ", course.to_uppercase()));
                }
                checklist_satistfied = false;
            }
        }

        if !checklist_satistfied {
            println!("{}", output_string.join(""));
        } else {
            println!("NONE");
        }

        checklist_satistfied
    }
}

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in).expect("Invalid input!");

    let n_students: usize = str_in.trim().parse().expect("Invalid number!");

    for t in 1..=n_students {
        str_in.clear();
        io::stdin().read_line(&mut str_in).expect("Invalid input!");

        let mut a_student = StudentEnrollInfo::new();

        let split_in: Vec<&str> = str_in.splitn(2, ' ').collect();

        let n_cmd: u64 = split_in[0].trim().parse().expect("Not a number!");
        let sn: u64 = split_in[1].trim().parse().expect("Not a number!");

        a_student.sn = sn;

        for _ in 0..n_cmd {
            str_in.clear();
            io::stdin().read_line(&mut str_in).expect("Invalid input!");

            let split_in: Vec<&str> = str_in.splitn(2, ' ').collect();

            let cmd: char = split_in[0].trim().parse().expect("Not a character!");
            let cmd_arg = split_in[1].trim();

            match cmd {
                'e' => {
                    a_student.is_eligible = if cmd_arg == "y" { true } else { false };
                }
                'a' => {
                    a_student.has_accountables = if cmd_arg == "y" { true } else { false };
                }
                'c' => {
                    let course = cmd_arg.to_string();
                    a_student.check_ge2017(course);
                }
                _ => {
                    println!("Invalid command!");
                }
            }
        }
        print!("Student #{}:\nRecord for SN {}\n    Is eligible? {}\n    Is accountable? {}\n    Unsatisfied GE2017 Courses:\n    ", t, a_student.sn, print_yes_or_no(a_student.is_eligible), print_yes_or_no(a_student.has_accountables));
        a_student.print_unsatisfied_ge2017();
    }
}

fn print_yes_or_no(b: bool) -> String {
    if b {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}
