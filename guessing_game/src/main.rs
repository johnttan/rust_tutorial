use std::old_io;
use std::rand;
use std::cmp::Ordering;

fn main() {

    let secret_number = (rand::random::<u32>() % 100) + 1;

    println!("Guess the number!");

    loop {
      println!("Please input your guess");

      let input = old_io::stdin().read_line()
      .ok()
      .expect("Failed to read line");

      let input_num: Option<u32> = input.trim().parse().ok();

      let num = match input_num {
        Some(num) => num,
        None => {
          println!("Please input a number!");
          return;
        }
      };

      println!("You guessed: {}", input);

      match cmp(num, secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
          println!("You win!");
          return ;
        }
      };
    }

}


fn cmp(a: u32, b: u32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}
