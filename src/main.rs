// Guess number

use std::{io,io::Write};
use rand::Rng;

fn gen_rand_num() -> i32{
    return rand::thread_rng().gen_range(1..100);
}

fn start_round(secect_num:i32){
    loop{
        let guess = get_guess();
        println!("Your guess: {}", guess);
        if guess == secect_num{
            println!("You win!\n");
            break;
        }else if guess < secect_num{
            println!("Too small!\n");
        }else if guess > secect_num{
            println!("Too big!\n");
        }
    }
}

fn get_guess() -> i32{
    let guess = get_input("Guess a number: ");
    match guess.trim().parse::<i32>() {
        Ok(num) => {
            return num;
        },
        Err(_) => {
            println!("Invalid input!");
            return get_guess();
        }
    }
}

fn get_input(msg: &str) -> String{
    print!("{}", msg);
    io::stdout().flush().expect("Flush err!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read err!");
    return input;
}

fn main(){
    println!("Guess number!\n");
    loop {
        let secect_num = gen_rand_num();
        start_round(secect_num);
        
        //is_contine
        let is_contine = get_input("Contine? (Y/n) ");
        if is_contine.trim().to_ascii_lowercase() == "y"{
            continue;
        }else if is_contine.trim().to_ascii_lowercase() == "n"{
            break;
        }else {
            continue;
        }
    }
}