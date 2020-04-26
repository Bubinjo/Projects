use std::io; //standardna knjižnica(std). Uporabi, če hočeš dobit input/output
use std::cmp::Ordering;
use rand::Rng; //Vključi, če hočeš uporabljati  rng generator

fn main() {

    println!("GUESS THE NUMBER");

    let secret_number = rand::thread_rng().gen_range(1, 101); //Generiranje naključnega števila od 1 do 100

    loop {
        println!("Please input your guess!");

        let mut guess = String::new(); //Shranjevanje inputa v 'mut' spremeljivko. 'Mut' pomeni, da se lahko vrednost spreminja. Po defaultu je immutable

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!"); //Branje vnešenega inputa

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number)  {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}