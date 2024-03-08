use std::io;
use std::collections::HashMap;

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in).expect("Invalid input!");

    let lines: u64 = str_in.trim().parse().expect("Not an integer!");

    for i in 1..=lines{
        str_in.clear();
        io::stdin().read_line(&mut str_in).expect("Invalid input!");

        let string_line = str_in.to_string();

        let mut char_frequency = LetterFreq::new();

        for ch in string_line.chars() {
            char_frequency.count(ch);
        }

        println!("---LETTER FREQUENCY of CASE #{}---", i);

        for letter in alphabet_with_space.iter(){
            char_frequency.current_counter(*letter);
        }

        println!("---------------------------------");
    }
}

trait Media {
    fn play(&self);
    fn title(&self) -> String;
    fn artist (&self) -> String;
}

struct Song {
    title: String,
    artist: String,
}

impl Media for Song {
    fn play(&self) {
        println!(" Now playing: {} by {}", self.title(), self.artist());
    }

    fn title(&self) -> String {
        self.title
    }

    fn artist (&self) -> String {
        match self.title().as_str() {
            "OMG" => "New Jeans",
            "Perfect Night" => "LE SSERAFIM",
            "Raining in Manila" => "Lola Amour",
            "Never Gonna Give You Up" => "Rick Astley",
            "Mananatili" => "Cup of Joe",
            "Aphrodite" => "The Ridleys",
            "Hanggang Sa Buwan" => "Kenaniah",
            "Dumaloy" => "SUD",
            _ => "Unknown",
        }.to_string()
    }
}

struct Queue<T> {
    list: Vec<T>
}

// how tf can this just be T
impl <T: Media> Queue<T> {
    fn new() -> Self{
        Self { list: Vec::new() }
    }

    fn play(&mut self) {
        let media  = self.list.remove(0);
        Media::play(&media);
    }

    fn add(&mut self, media: T) {
        self.list.push(media);
    }

    fn show_queue(&self) {
        if self.is_empty() {
            println("No media in queue.");
        } else {
            for (pos, val) in self.list.iter().enumerate() {
                println!("{}. {} by {}", pos + 1, Media::title(val), Media::artist(val));
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}
