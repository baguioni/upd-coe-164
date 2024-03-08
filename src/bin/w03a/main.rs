use std::io;
use std::collections::HashMap;

fn main() {
    // this is for printing in order 
    let alphabet_with_space: [char; 27] = [
        'a', 'b', 'c', 'd', 'e', 
        'f', 'g', 'h', 'i', 'j', 
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 
        'u', 'v', 'w', 'x', 'y', 
        'z', ' ',
    ];

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

struct LetterFreq {
    dictionary: HashMap<char, u64>
}

impl LetterFreq {

    fn new() -> LetterFreq {
        LetterFreq {
            dictionary: HashMap::new()
        }
    }

    fn count(&mut self, input: char) {
        *self.dictionary.entry(input.to_ascii_lowercase()).or_insert(0) += 1;
    }

    fn current_counter(&mut self, input: char) {
            if let Some(value) = self.dictionary.get(&input) {
                println!("{}: {}", input, value);
            }
    }
}
