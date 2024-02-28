use std::io;

struct Player {
    name: String,
    pos: i64,
    item: Option<PlayerItem>,
}

struct PlayerItem {
    name: String,
    item_type: PlayerItemQtyType,
}

enum PlayerItemQtyType {
    Once,
    Consumable(u64),
}

enum PlayerAction {
    left,
    right,
    uitem,
}

impl Player {
    fn new(name: String, item: Option<PlayerItem>) -> Self {
        Self { name, item, pos: 0 }
    }

    fn action(&mut self, action_type: PlayerAction) -> String {
        match action_type {
            PlayerAction::left => {
                self.pos -= 1;
                format!("New position: {}", self.pos)
            }
            PlayerAction::right => {
                self.pos += 1;
                format!("New position: {}", self.pos)
            }
            PlayerAction::uitem => {
                if let None = self.item {
                    return "Player has no item to use".to_string();
                }

                let item = self.item.as_mut().unwrap();

                let mut item_is_empty: bool = false;

                let result = match item.item_type {
                    PlayerItemQtyType::Consumable(ref mut count) => {
                        *count -= 1;
                        if *count == 0 {
                            item_is_empty = true;
                            format!("Player used <{}>. It is now gone", item.name)
                        } else {
                            format!(
                                "Player used <{}>. {}x of <{}> remains.",
                                item.name, count, item.name
                            )
                        }
                    }
                    PlayerItemQtyType::Once => {
                        item_is_empty = true;
                        format!("Player used <{}>. It is now gone", item.name)
                    }
                };

                if item_is_empty {
                    self.item = None;
                }
                result
            }
        }
    }
}

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in).expect("Invalid input!");

    let n_players: usize = str_in.trim().parse().expect("Invalid number!");

    let mut output_strings: Vec<String> = Vec::new();

    for i in 1..=n_players {
        let mut str_in = String::new();

        io::stdin().read_line(&mut str_in).expect("Invalid input!");

        let player_info: Vec<&str> = str_in.split_whitespace().collect();
        let item_amount = player_info[2].parse::<u64>().unwrap();

        let item = match item_amount {
            0 => None,
            _ => Some(PlayerItem {
                name: player_info[1].to_string(),
                item_type: if item_amount == 1 {
                    PlayerItemQtyType::Once
                } else {
                    PlayerItemQtyType::Consumable(item_amount)
                },
            }),
        };
        let mut player = Player::new(player_info[0].to_string(), item);

        let mut str_in = String::new();

        io::stdin().read_line(&mut str_in).expect("Invalid input!");

        let n_actions: usize = str_in.trim().parse().expect("Invalid number!");

        let mut input_actions = Vec::<PlayerAction>::with_capacity(n_actions);

        for _i in 0..n_actions {
            let mut str_in = String::new();

            io::stdin().read_line(&mut str_in).expect("Invalid input!");
            input_actions.push(string_to_action(str_in.trim()));
        }

        if item_amount > 0 {
            output_strings.push(format!(
                "Player #{}:\nName: {}\nItem: {}x {}",
                i,
                player.name,
                item_amount,
                player_info[1].to_string()
            ));
        } else {
            output_strings.push(format!("Player #{}:\nName: {}\nItem: NONE", i, player.name));
        }

        output_strings.push(format!("----------LOG----------"));

        for action in input_actions {
            output_strings.push(format!("{}", player.action(action)));
        }
    }

    for output in output_strings {
        println!("{}", output);
    }
}

fn string_to_action(action: &str) -> PlayerAction {
    match action {
        "left" => PlayerAction::left,
        "right" => PlayerAction::right,
        "uitem" => PlayerAction::uitem,
        _ => unreachable!(),
    }
}
