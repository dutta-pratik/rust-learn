#[derive(Debug)] //calls the debug trait of Struct Deck attribute
struct Deck {
    cards: Vec<String>
}

impl Deck { //we can implement function like this
    fn new() -> Deck { //Deck is showing the return type, we can use 'Self' instead of Deck(Self is an alias for Deck which is mentioned in impl)
        //this function is an associative function, means it is not tied to a specific instance of the struct, howver it is tied to struct
    }
    fn shuffle(&mut self) {//this is a method which is tied to a specific instance of the struct
        
    }
}

fn main() {

    let suits = ["hearts", "diamonds", "clubs", "spades"];//for string we use "" instead of '', '' uses for char
    let values = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];

    //let cards = vec![]; // this binding is immutable, we can't reassign it(eg. cards = vec![]) or we can't change the value of it

    let mut cards = vec![]; // this binding is mutable

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);//format macro is used to create a string
            cards.push(card);
        }
    }

    // let deck = Deck { cards: vec![] }; //variables are called bindings in rust
    // let deck = Deck { cards: cards }; //since bindings and fields have the same name, we can use the same name
    let mut deck = Deck { cards };
    
    // println!("here's our deck: {:?}", deck); // :? is used to print the debug trait of the struct in normal format
    println!("here's our deck: {:#?}", deck); // :#? is used to print the debug trait of the struct in pretty format
}
