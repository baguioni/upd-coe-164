use core::fmt;
use std::{borrow::Borrow, collections::HashMap, fmt::write, io, num::NonZeroI8};

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in).expect("Invalid input!");

    let num_borrowers: u64 = str_in.trim().parse().expect("Not an integer!");

    let mut borrowers_hash: HashMap<String, Borrower>= HashMap::new();

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

        borrowers_hash.insert(name, borrower);
    }

    str_in.clear();
    io::stdin().read_line(&mut str_in).expect("Invalid input!");
    let num_items: u64 = str_in.trim().parse().expect("Not an integer!");

    let mut lent_items: Vec<LentItem> = Vec::new();
    let mut lent_items_hash: HashMap<String, LentItem>= HashMap::new();

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

        lent_items.push(item);
        lent_items_hash.insert(name, item);
    }

    let lent_items_references: Vec<&LentItem> = lent_items.iter().collect();

    str_in.clear();
    io::stdin().read_line(&mut str_in).expect("Invalid input!");
    let num_commands: u64 = str_in.trim().parse().expect("Not an integer!");

    for _i in 1..=num_commands {
        str_in.clear();
        io::stdin().read_line(&mut str_in).expect("Invalid input!");

        let commands: Vec<&str> = str_in.trim().split(' ').collect();

        let cmd = commands[0];

        if commands.len() == 2 {
            let b_name = commands[1];
            match cmd {
                "bi" => {
                    match borrowers_hash.get(b_name) {
                        Some(person) => {
                            println!("[BINFO] {}", person);

                            let borrowed_items = person.borrowed_items(&lent_items_references);

                            if !borrowed_items.is_empty() {
                                println!("-----BORROWED ITEMS-----"); 
                                for item in borrowed_items {
                                    println!("{}", item);
                                }
                            }
                        },
                        None => println!("[BINFO] Borrower \"{}\" not found!", b_name)
                    };
                }, 
                "ii" =>println!("{}", commands[1]),
                "u" => println!("{}", commands[1]),
                _ => unreachable!(),
            }
        } else if commands.len() == 3 {
            let i_name = commands[1];
            let b_name = commands[2];
            match cmd {
                "b" => {
                    let borrower = borrowers_hash.get(b_name);

                    if borrower.is_none() {
                        println!("[BORROW] Borrower \"{}\" not found!", b_name)
                    }

                    

                    let mut item = lent_items_hash.get(i_name);

                    if item.is_none() {
                        println!("[BORROW] Item \"{}\" not found!", i_name);
                    }

                    LentItem::borrow(item, &borrower);
                },
                _ => unreachable!(),
            }
        } else if commands.len() == 4 {
            match cmd {
                "t" => println!("{}", commands[1]),
                _ => unreachable!(),
                
            }
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

    fn borrowed_items<'a>(&self, items: &'a Vec<&'a LentItem<'a>>) -> Vec<&'a LentItem<'a>> {
        let mut borrowed: Vec<&LentItem> = Vec::new();
        for item in items {
            if let Some(borrowed_by) = item.borrowed_by {
                if self.name == borrowed_by.name {
                    borrowed.push(item);
                }
            }
        }

        borrowed
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
