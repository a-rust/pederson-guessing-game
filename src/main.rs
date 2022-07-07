mod commitment;
mod group;
mod interactive;
use crate::commitment::*;
use crate::group::Group;
use crate::interactive::*;

fn main() {
    game_setup();

    // Letting the user choose which prime p to set for the
    // multiplicative group Z_p^*
    let prime: i128 = choose_ring();

    // Setting the elements of the group = {1, ... , p-1}, since p is
    // prime
    let elem: Vec<i128> = (1..prime).collect();

    // Initializing the group for our Pederson Commitment Scheme
    let x = Group {
        elements: elem,
        subscript: prime,
    };

    // Finding divisors of p-1 to apply theorem for finding generators
    let divisors = x.find_divisors();

    // Finding generators of Z_p^*
    let _generators = x.find_generator(divisors);

    // Initializing the Pederson Commitment Scheme
    let guess_my_number = PedersonCommitmentScheme { group: x };

    // Generating public and private values
    let gen_one = guess_my_number.choose_random_generator();
    let gen_two = guess_my_number.choose_random_generator();
    let key_one = guess_my_number.secret_key();
    let key_two = guess_my_number.secret_key();

    // Generating my random message (number)
    let my_guess = choose_my_number();

    // Taking the user's guess
    let user_guess = user_guess();

    // Gathers public and private information (secret keys and my
    // number) to verify the commit was truthful
    let public_info = guess_my_number.public_info(gen_one, gen_two, key_one, key_two, my_guess);

    // Giving the user the option to change guesses after seeing the
    // commitment (did they break the scheme?)
    let _changing_guess = option_to_change();

    // Checks user's answer against mine
    did_user_win(user_guess, my_guess);

    let private_info = guess_my_number.private_info(key_one, key_two, my_guess);

    // Verifies commit
    guess_my_number.verify_commitment(public_info, private_info, prime);
}
