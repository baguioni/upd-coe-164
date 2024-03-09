use std::io;

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in).expect("Invalid input!");

    let lines: u64 = str_in.trim().parse().expect("Not an integer!");

    let mut song_queue = Queue::<Song>::new();

    for _i in 1..=lines {
        str_in.clear();
        io::stdin().read_line(&mut str_in).expect("Invalid input!");

        let string_line: Vec<&str> = str_in.trim().splitn(2, ' ').collect();

        if string_line.len() == 1 {
            match string_line[0] {
                "play" => song_queue.play(),
                "show_queue" => song_queue.show_queue(),
                _ => println!("Invalid command!"),
            }
        } else {
            let title = string_line[1];
            let artist = Song::match_title_with_artist(&title);
            song_queue.add(Song {
                title: title.to_string(),
                artist,
            });
        }
    }
}


trait Media {
    fn play(&self);
    fn title(&self) -> String;
    fn artist(&self) -> String;
}

struct Song {
    title: String,
    artist: String,
}

impl Media for Song {
    fn play(&self) {
        println!("Now playing: {} by {}", self.title(), self.artist());
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn artist(&self) -> String {
        self.artist.clone()
    }
}

impl Song {
    fn match_title_with_artist(title: &str) -> String {
        match title {
            "OMG" => "New Jeans",
            "Perfect Night" => "LE SSERAFIM",
            "Raining in Manila" => "Lola Amour",
            "Never Gonna Give You Up" => "Rick Astley",
            "Mananatili" => "Cup of Joe",
            "Aphrodite" => "The Ridleys",
            "Hanggang sa Buwan" => "Kenaniah",
            "Dumaloy" => "SUD",
            _ => "Unknown",
        }
        .to_string()
    }
}


struct Queue<T> {
    list: Vec<T>,
}

// how tf can this just be T
impl<T: Media> Queue<T> {
    fn new() -> Self {
        Self { list: Vec::new() }
    }

    fn play(&mut self) {
        if self.is_empty() {
            println!("Queue is empty! No media to play...");
        } else {
            let media = self.list.remove(0);
            Media::play(&media);
        }
    }

    fn add(&mut self, media: T) {
        println!(
            "Successfully added {} by {} to the queue!",
            media.title(),
            media.artist()
        );
        self.list.push(media);
    }

    fn show_queue(&self) {
        if self.is_empty() {
            println!("No media in queue.");
        } else {
            println!("-----mEEEdia bot-----");
            for (pos, val) in self.list.iter().enumerate() {
                println!(
                    "{}. {} by {}",
                    pos + 1,
                    Media::title(val),
                    Media::artist(val)
                );
            }
            println!("---------------------");
        }
    }

    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}
