#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>
}

#[derive(Debug)]
struct Account {
    id: i32,
    name: String,
}

impl Bank {
    fn new() -> Self {
        let accounts = vec![];
        Bank { accounts }
    }
}
impl Account {
    fn new(id: i32, name: String) -> Account {
        Account { id, name }
    }
}

fn change_account(account: &mut Account) {
    account.balance = 200.0;

    println!("account owner: {:#?}", account.owner);
    //mutable reference can be used to change or read the value 
}

fn print_account(account: &Account) {
    // account.owner = String::from("Jane"); // will throw error because we are trying to change the value of the account
    // account.owner = String::from("Jane"); //even if we make account binding mutable, we can't change the value of the account if we pass just the reference, to make it work we need to pass the mutable reference
    println!("account: {:#?}", account.name);
}

fn make_and_print_account() -> &Account {
    let account = Account::new(100.0, String::from("John"));
    println!("account: {:#?}", account);

    // &account//will throuw error since we are sending the reference of the account binding, not the account binding itself
    //rust will delete the account binding after the function call, so the reference will be invalid
}

fn main() {

    /*
    reference
    */
    // let mut account = Account::new(100.0, String::from("John"));

    // // print_account(&account);sent reference

    // println!("account: {:#?}", account);



    /*
    Mutable reference
    */
    let mut account = Account::new(100.0, String::from("John"));

    change_account(&mut account);

    println!("account: {:#?}", account);


    /*some type of bindings like numbers, char, booleans, arrays(if everything is copyable), tuples(if everything is copyable), references(both readable and writable). 
    their values can be copied into another binding without any issues like we have in other languages.
    The above mentioned bindings are called Copyable types. They gets copied instead of being moved.
    Following is the example of copying a value into another binding:
    */
    let x = 5;
    let y = x;
    println!("x: {}", x);
    println!("y: {}", y);

    /*
    */



    /* Rule 9
    */
    let account = make_and_print_account();
    println!("account: {:#?}", account);

}
