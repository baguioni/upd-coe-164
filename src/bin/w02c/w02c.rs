use std::io;

enum ItemType {
    Beverage,
    Cleaners,
    Electronics,
    Fruits,
    Meat,
}

fn display_item_type(item_type: &ItemType) -> String {
    match item_type {
        ItemType::Beverage => "Beverage".to_string(),
        ItemType::Cleaners => "Cleaners".to_string(),
        ItemType::Electronics => "Electronics".to_string(),
        ItemType::Fruits => "Fruits".to_string(),
        ItemType::Meat => "Meat".to_string(),
    }
}

fn map_to_item_type(input: &str) -> Option<ItemType> {
    match input.to_lowercase().as_str() {
        "coke" | "sprite" | "royal" => Some(ItemType::Beverage),
        "bleach" | "soap" => Some(ItemType::Cleaners),
        "battery" | "bulb" => Some(ItemType::Electronics),
        "banana" | "mango" | "strawberries" => Some(ItemType::Fruits),
        "beef" | "chicken" | "pork" => Some(ItemType::Meat),
        _ => None,
    }
}

struct GroceryItem {
    item: ItemType,
    price: f64,
    weight: f64,
}

struct SmartCart {
    items: [Option<GroceryItem>; 10],
    max_budget: f64,
    max_weight: f64,
    current_value: f64,
    current_weight: f64,
    current_size: usize,
}

impl SmartCart {
    fn new(max_budget: f64) -> Self {
        Self {
            items: [None, None, None, None, None, None, None, None, None, None],
            max_budget,
            max_weight: 12.0,
            current_value: 0.0,
            current_weight: 0.0,
            current_size: 0,
        }
    }

    fn add_item(&mut self, grocery_item: GroceryItem) {
        if self.current_weight + grocery_item.weight > self.max_weight {
            println!("[SYSTEM] Maximum weight reached! Item unsuccessfully added.");
            return;
        }

        if self.current_size + 1 > 10 {
            // !("{} + 1 > 10", self.current_size);
            println!("[SYSTEM] Maximum number of items reached! Item unsuccessfully added.");
            return;
        }

        if self.current_value + grocery_item.price > self.max_budget {
            // println!(" {} + {} > {}", self.current_value, grocery_item.price, self.max_budget);
            println!("[SYSTEM] Maximum budget reached! Item unsuccessfully added.");
            return;
        }

        self.current_size += 1;
        self.current_value += grocery_item.price;
        self.current_weight += grocery_item.weight;

        // Add grocery item to self.items array replacing the None value
        for i in 0..self.items.len() {
            if self.items[i].is_none() {
                self.items[i] = Some(grocery_item);
                break;
            }
        }

        println!("[SYSTEM] Item successfully added!")
    }

    fn remove_item(&mut self, index: usize) {
        if index >= self.items.len() {
            println!("[ERROR] Index does not exist!");
            return;
        }

        if self.items[index].is_none() {
            println!("[SYSTEM] No item removed!");
            return;
        }

        println!("[SYSTEM] Item removed!");

        if let Some(item) = self.items[index].take() {
            self.current_value -= item.price;
            self.current_weight -= item.weight;
        }

        for i in index..self.items.len() - 1 {
            self.items[i] = self.items[i + 1].take();
        }
        self.items[self.items.len() - 1] = None;
    }

    fn show_info(&self) {
        println!("------GROCERY CART------");
        for i in 0..self.items.len() {
            if self.items[i].is_none() {
                break;
            }
            let item_type = &self.items[i].as_ref().unwrap().item;

            println!("{}: {}", i + 1, display_item_type(item_type))
        }
        println!("Total price: Php {:.2}", self.current_value);
        println!("Total weight: {:.2} kg", self.current_weight);
        println!("------------------------");
    }
}

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in).expect("Invalid input!");

    let max_budget: f64 = str_in.trim().parse().expect("Not a Float!");

    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in).expect("Invalid input!");

    let n_commands: usize = str_in.trim().parse().expect("Invalid number!");

    let mut cart = SmartCart::new(max_budget);

    let mut valid_commands_ran = 0;

    while valid_commands_ran <= n_commands {
        let mut str_in = String::new();
        io::stdin().read_line(&mut str_in).expect("Invalid input!");

        let arg: Vec<&str> = str_in.split(' ').collect();

        let command = arg[0].trim();

        let valid_commands = ["add", "remove", "show_info"];

        if !valid_commands.contains(&command) {
            println!("[ERROR] Command not found!");
            continue;
        }

        if command == "add" {
            let grocery_item = arg[1].trim();

            if let None = map_to_item_type(grocery_item) {
                println!("[ERROR] Item not classified!");
                continue;
            }

            let price: f64 = match arg[2].trim().parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("[ERROR] Price error!");
                    continue;
                }
            };

            let weight: f64 = match arg[3].trim().parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("[ERROR] Weight error!");
                    continue;
                }
            };

            cart.add_item(GroceryItem {
                item: map_to_item_type(grocery_item).unwrap(),
                weight,
                price,
            });

            valid_commands_ran += 1;
            continue;
        }

        if command == "remove" {
            let index: usize = match arg[1].trim().parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("[ERROR] Index does not exist!");
                    continue;
                }
            };

            cart.remove_item(index - 1);

            valid_commands_ran += 1;
            continue;
        }

        if command == "show_info" {
            cart.show_info();

            valid_commands_ran += 1;
            continue;
        }
    }
}
