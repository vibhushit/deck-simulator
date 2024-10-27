use rand::{thread_rng, seq::SliceRandom}

#[derive(Debug)]
struct Deck {
    cards : Vec<String>
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamongs", "Clubs"];
        // let values = ["A", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
        let values = ["A", "1", "2"];
        let mut cards = Vec::new();
    
        for suit in suits {
            for value in values {
                let card =  format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }
}

fn main() {
    let cards = Deck::new();
    println!("Deck is {:#?}", cards);
}
