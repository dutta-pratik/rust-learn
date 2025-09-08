#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

impl Deck {
    fn new() -> Self {
        let suits = ["hearts", "diamonds", "clubs", "spades"];
        let values = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];
        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        let deck = Deck { cards };
        // return deck;
        deck //this is the same as return deck; this is called implicit return. keep in mind we have to drop semicolon at the end and return keyword to make it implicit return    }
    }

}

//borrowing
fn print_cards(deck: &Deck) {
    println!("deck: {:#?}", deck);
}


fn main() {
    let deck = Deck::new();
    // let suit = deck.cards;
    // println!("suit: {:#?}", deck.cards);

    //borrowing
    let deck_ref = &deck;
    print_cards(deck_ref);
    print_cards(deck_ref);

    println!("here's our deck: {:#?}", deck);
    println!("here's our deck: {:#?}", deck);
}
