use rand::prelude::SliceRandom;
use std::io;

pub fn game_setup() {
    println!(
        "Hello!
---
I will choose a number from 1-5. 
If you guess my number correctly in 1 attempt, I'll give you a prize!
---
To prevent myself from cheating I will give you a commitment c
from the Pederson Commitment scheme, which you can later verify,
after you make your guess.
---"
    );
    println!(
        "I will use the Pederson Commitment and show you all the 
corresponding public values. 
I'll even let you choose the group Z_p^* we work in
---"
    )
}

pub fn choose_ring() -> i128 {
    println!(
        "Please enter a prime p<=19 for the ring 
Z_P^* you want the commitment scheme to work in:"
    );
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Please enter a valid prime:");
    let prime: i128 = input.trim().parse().expect(
        "Please enter the prime p in the 
ring you want the commitment scheme to work in:",
    );
    println!(
        "You chose to work in the ring Z_{}^*
---",
        prime
    );
    //Return the first p-1 elements, since Z_p^* has the elements
    // {1, ... ,}
    prime
}

pub fn choose_my_number() -> i128 {
    let game_outcomes: Vec<i128> = (1..6).collect();
    let my_guess: Option<&i128> = game_outcomes.choose(&mut rand::thread_rng());
    *my_guess.unwrap()
}

pub fn user_guess() -> i128 {
    println!(
        "Alright, I have my number from 1-5. 
What's your guess?:"
    );
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Please enter a valid guess:");
    let user_guess: i128 = input
        .trim()
        .parse()
        .expect("Please enter a number from 1-5:");
    println!(
        "Great! You chose {}
---",
        user_guess
    );
    user_guess
}

pub fn option_to_change() {
    println!(
        "I'm feeling generous. Think you can figure out my 
my number, given [g_1, g_2, c]?
---
I'll give you the option of changing your guess.
Would you like to change your guess?
Enter y for yes, or n for no."
    );
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Please enter a valid option:");
    if input.trim() == "n" {
        println!(
            "---
Okay! Let's see if your original guess was correct
---"
        );
        return;
    } else {
        change();
    }
}

pub fn change() {
    let mut new_input = String::new();
    println!("Okay, what would you like your new guess to be?");
    io::stdin()
        .read_line(&mut new_input)
        .expect("Please enter a valid guess:");
    let user_guess: i128 = new_input
        .trim()
        .parse()
        .expect("Please enter a number from 1-5:");
    println!(
        "Okay, let's see if your new guess of {} is correct:
---",
        user_guess
    );
}

pub fn did_user_win(user_guess: i128, my_guess: i128) {
    if user_guess == my_guess {
        println!(
            "Congrats! Your guess was {}, and my guess was {}.
You either got lucky, or you cracked the code. 
Either way, here is your prize!!!
🏆",
            user_guess, my_guess
        );
    } else {
        println!(
            "Sorry, but your guess of {} 
was not the same as my guess of {}.",
            user_guess, my_guess
        );
    }
}
