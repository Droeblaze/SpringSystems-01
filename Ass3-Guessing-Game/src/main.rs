fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret: i32 = 25;
    let mut guess_count = 0;
    let mut guess;

    loop {
        if guess_count == 0 {
            guess = 10;
        } else if guess_count == 1 {
            guess = 30;
        } else {
            guess = 25;
        }

        guess_count += 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct");
            break;
        } else if result == 1 {
            println!("Too high");
        } else {
            println!("Too low");
        }
    }

    println!("Guesses {}", guess_count);
}