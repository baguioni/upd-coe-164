use core::fmt;
use std::{borrow::Borrow, fmt::write, io};

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in).expect("Invalid input!");

    let num_borrowers: u64 = str_in.trim().parse().expect("Not an integer!");

    for _i in 1..=num_borrowers {
        str_in.clear();
        io::stdin().read_line(&mut str_in).expect("Invalid input!");

        let date_borrower: Vec<_> = str_in.trim().splitn(2, ' ').collect();
        
        let name = date_borrower[1].to_string();

        let dates: Vec<_> = date_borrower[0].trim().split("-").collect();
        let year: u64 = dates[0].parse().expect("Failed to parse u64");
        let month: u8 = dates[1].parse().expect("Failed to parse u8");
        let day: u8 = dates[2].parse().expect("Failed to parse u8");

        let reg_date = SplitDate{
            year,
            month,
            day
        };

        let borrower = Borrower { name, reg_date };

        println!("{}", borrower);
    }

    str_in.clear();
    io::stdin().read_line(&mut str_in).expect("Invalid input!");
    let num_items: u64 = str_in.trim().parse().expect("Not an integer!");

    for _i in 1..=num_items {
        str_in.clear();
        io::stdin().read_line(&mut str_in).expect("Invalid input!");


        let date_item: Vec<_> = str_in.trim().splitn(2, ' ').collect();
        
        let dates: Vec<_> = date_item[0].trim().split("-").collect();
        let year: u64 = dates[0].parse().expect("Failed to parse u64");
        let month: u8 = dates[1].parse().expect("Failed to parse u8");
        let day: u8 = dates[2].parse().expect("Failed to parse u8");

        let name = date_item[1].to_string();

        let item = LentItem::new(name, year, month, day);

        println!("{}", item);

    }

    let num_commands: u64 = str_in.trim().parse().expect("Not an integer!");

    for _i in 1..=num_commands {
        str_in.clear();
        io::stdin().read_line(&mut str_in).expect("Invalid input!");

        let commands: Vec<&str> = str_in.trim().split(' ').collect();

        if commands.len() == 2 {
        } else if commands.len() == 3 {
        } else if commands.len() == 4 {
        }
    }
}

struct SplitDate {
    year: u64,
    month: u8,
    day: u8,
}

struct Borrower {
    name: String,
    reg_date: SplitDate,
}

struct LentItem<'a> {
    name: String,
    acquire_date: SplitDate,
    borrowed_by: Option<&'a Borrower>,
}

// impl SplitDate {
//     fn new(input: &str) -> Option<SplitDate> {
//         let parts: Vec<&str> = input.trim().split('-').collect();
//         if let (Ok(year), Ok(month), Ok(day)) = (
//             parts[0].parse::<u64>(),
//             parts[1].parse::<u8>(),
//             parts[2].parse::<u8>(),
//         ) {
//             return Some(SplitDate { year, month, day });
//         }
//         None
//     }
// }

impl<'a> LentItem<'a> {
    fn new(name: String, year: u64, month: u8, day: u8) -> Self {
        Self {
            name,
            acquire_date: SplitDate { year, month, day },
            borrowed_by: None,
        }
    }

    fn borrow(&'a mut self, to: &'a Borrower) -> Option<&'a Borrower> {
        if self.borrowed_by.is_none() {
            self.borrowed_by = Some(to);
            None
        } else {
            self.borrowed_by
        }
    }

    fn unborrow(&mut self) -> Option<&Borrower> {
        if self.borrowed_by.is_none() {
            None
        } else {
            return self.borrowed_by;
        }
    }

    fn transfer(
        &mut self,
        from: &'a Borrower,
        to: &'a Borrower,
    ) -> (Option<&Borrower>, Option<&Borrower>) {
        let current_borrower_name = &self.borrowed_by.as_ref().unwrap().name;
        if from.name != *current_borrower_name || to.name == *current_borrower_name {
            return (Some(from), None);
        }

        self.borrowed_by = Some(to);
        return (Some(from), Some(to));
    }
}

impl Borrower {
    fn new(name: String, year: u64, month: u8, day: u8) -> Self {
        Self {
            name,
            reg_date: SplitDate { year, month, day },
        }
    }

    fn borrowed_items(&self, items: &Vec<&LentItem>) -> Vec<&LentItem> {
        todo!()
    }
}

impl fmt::Display for SplitDate {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl<'a> fmt::Display for LentItem<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(borrowed_by) = self.borrowed_by {
            write!(
                fmt,
                "LentItem({}) [Acquired {}] [Borrowed by {}]",
                self.name, self.acquire_date, borrowed_by.name
            )
        } else {
            write!(
                fmt,
                "LentItem({}) [Acquired {}]",
                self.name, self.acquire_date
            )
        }
    }
}

impl std::fmt::Display for Borrower {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "Borrower({}) [Registered {}]",
            self.name, self.reg_date
        )
    }
}
