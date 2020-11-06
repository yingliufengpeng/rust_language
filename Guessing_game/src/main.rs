use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input you guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Faild to read line");

        let guess: i32 = guess.trim().parse()
            .expect("请输入数字");

        println!("You gussed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }



}
