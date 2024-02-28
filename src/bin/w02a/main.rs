use std::io;

struct Player {
    name: String,
    pos: i64,
    item: Option<PlayerItem>
}

impl Player {
    fn new(name: String, item: Option<PlayerItem>) -> Self {}

    fn action(&self, action_type: PlayerAction) -> String {
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
                self.pos -= 1;
                let item = self.item.
                // check if item exists
                format!(" ")
            }
            _ => format!("Error has occured")
        }
    }


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

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in).expect("Invalid input!");

    let n_players: usize = str_in.trim().parse().expect("Invalid number!");

    for _i in 0..n_players{
        let mut str_in = String::new();

        io::stdin().read_line(&mut str_in).expect("Invalid input!");

        let player_info: Vec<&str> = str_in.split_whitespace().collect();

        let item_amount = player_info[2].parse::<i64>();

        let mut player = Player {
            name: player_info[0].to_string(),
            pos: 0,
            item: PlayerItem {
                name: player_info[1].to_string(),
                item_type: if item_amount > 1 {PlayerItemQtyType::Consumable(item_amount)} else {PlayerItemQtyType::Once},
            },
        };




    }
}
