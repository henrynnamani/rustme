use rand::{thread_rng, seq::SliceRandom}

#[derive(Debug)]


struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let mut cards = vec![];

        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);

                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle()

    println!("My deck: {:#?}", deck)
}
