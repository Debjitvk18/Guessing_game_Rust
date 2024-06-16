use std::io;
use rand::prelude::*;

fn main() {
    let guest_list = ["grapes", "banana", "orange","mango","litchi","goava",
    "peach","pineappple","palm","lemon","date","apple"];

    let mut rng = thread_rng();
    let index = rng.gen_range(0..guest_list.len());
    let random_fruit = guest_list[index];
    // println!("Random fruit:{} ",random_fruit);        
    let mut input = String::new();
    
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let fruit_selected = input.trim().to_lowercase();
                println!("Fruit selected:{} ",fruit_selected);
                if !guest_list.contains(&&fruit_selected.as_str()) {
                    println!("Fruit entered does not found");
                    continue;
                }
                if guess_checker(&fruit_selected, random_fruit) {
                    println!("You are winner");
                    break;
                } else {
                    println!("Retry");
                }
            }
            Err(error) => {
                println!("Error:{}", error);
            }
        }
    }
}

fn guess_checker(fruit_selected: &str, random_fruit: &str) -> bool {
    return fruit_selected == random_fruit;
}
