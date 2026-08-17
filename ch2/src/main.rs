use std::io; //input-output library
use rand::Rng;
use std::cmp:: Ordering;
fn main(){
    println!("Guessing Game");
    let secret_number = rand::thread_rng().gen_range(1..=100);
   
    loop{
        println!("Please Enter your Guess:");
         
    
    let mut guess=String::new(); //mutable: value can be changed.
    io::stdin()
    .read_line(&mut guess).expect("Failed to Read line");
    let guess:u32 = guess.trim().parse().expect("Please Type a Number...");
    println!("You Guessed: {guess}");
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too Small..."),
        Ordering::Equal => {
            println!("You Win");
            break;
        },
        Ordering::Greater=> println!("Too Big...")
    }
    }
   
}
