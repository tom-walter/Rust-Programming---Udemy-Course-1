#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_must_use)]

use std::io::stdin;
use rand::Rng; // version = "0.7.3"


fn main()
{
    // generate a random number between 1 and 100
    let number: i64 = rand::thread_rng().gen_range(1, 101); // exlusive

    // create the guessing loop
    loop {
        // ask user for input
        println!("Enter you guess: ");
        // save user input
        let mut buffer = String::new();
        
        match stdin().read_line(&mut buffer)
        {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed
                {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Your guess is out of range.");
                        } else if  guess < number {
                            println!("Your guess is too low.");
                        } else if guess > number {
                            println!("Your guess is too high.");
                        } else {
                            println!("Your guess is correct!");
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Could not read your number: {}. Try again.", e);
                    }
                }
            }
            Err(_) => continue,
        }

    }
}