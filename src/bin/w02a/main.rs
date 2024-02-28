use std::io;

struct Player {
    name: String,
    pos: i64,
    item: Option<PlayerItem>
}

struct PlayerItem {
    name: String,
    item_type: PlayerItemQtyType,
}

enum PlayerItemQtyType {
    Once,
    Consumable(u64)
}

enum PlayerAction {
    left,
    right,
    uitem,
}

impl Player {
    fn new(name: String, item: Option<PlayerItem>) -> Self {
        Self {
            name,
            item,
            pos: 0,
        }
    }

    fn action(mut self, action_type: PlayerAction) -> String {
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
                let item: PlayerItem = self.item.unwrap();

                match item.item_type { 
                    PlayerItemQtyType::Consumable(mut count) => {
                        count -= 1;
                        if count < 1 {
                            self.item = None;
                            format!("Player used <{}>. It is now gone", item.name)
                        } else {
                            format!("Player used <{}>. {}x of <{}> remains.", item.name, count, item.name)
                        } 
                    }
                    PlayerItemQtyType::Once => {
                        self.item = None;
                        format!("Player used <{}>. It is now gone", item.name)
                    }
                }
            }
        }
    }


}


fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in).expect("Invalid input!");

    let n_players: usize = str_in.trim().parse().expect("Invalid number!");

    for _i in 0..n_players{
        let mut str_in = String::new();

        io::stdin().read_line(&mut str_in).expect("Invalid input!");

        let player_info: Vec<&str> = str_in.split_whitespace().collect();
        let item_amount = player_info[2].parse::<u64>().unwrap();

        let item = PlayerItem {
            name: player_info[1].to_string(),
            item_type: if item_amount == 1 {PlayerItemQtyType::Once} else {PlayerItemQtyType::Consumable(item_amount)},
        };

        let mut player = Player::new(player_info[0].to_string(), Some(item));



    }
}
