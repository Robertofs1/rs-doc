use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Advinhe o numero!");

    let secret_number = rand::thread_rng()
    .gen_range(1..=100);

    println!("O número secreto é: {secret_number}");

    println!("Por favor Insira seu palpite");
        
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
               .expect("Falha ao ler a linha");

    let guess: u32 = guess.trim().parse().expect("Por favor Insira seu palpite");
    
    println!("Seu palpite é: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("palpite baixo bro"),
        Ordering::Greater => println!("palpite muito alto bro"),
        Ordering::Equal => println!("acerto mizeravi"),
    }
}
