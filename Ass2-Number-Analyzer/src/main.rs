fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers: [i32; 10] = [3, 8, 15, 22, 5, 30, 11, 18, 7, 10];

    for n in numbers {
        if n % 3 == 0 && n % 5 == 0 {
            println!("{} FizzBuzz", n);
        } else if n % 3 == 0 {
            println!("{} Fizz", n);
        } else if n % 5 == 0 {
            println!("{} Buzz", n);
        } else {
            if is_even(n) {
                println!("{} even", n);
            } else {
                println!("{} odd", n);
            }
        }
    }

    let mut i = 0;
    let mut sum = 0;

    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }

    println!("Sum {}", sum);

    let mut index = 0;
    let mut largest = numbers[0];

    loop {
        if numbers[index] > largest {
            largest = numbers[index];
        }

        index += 1;

        if index >= numbers.len() {
            break;
        }
    }

    println!("Largest {}", largest);
}