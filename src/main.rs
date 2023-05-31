use rand::thread_rng;
use rand::Rng;
use crossterm::style::{Attribute, Color, Stylize};
use std::io;

fn main() {

    welcome();
    let random=randomnum();

    let random_str=random.to_string();
    let random_chars: Vec<char> = random_str.chars().collect();
    println!("{}",random);

loop{
    let mut guess_str = String::new();
    io::stdin().read_line(&mut guess_str).expect("Failed to read line");
    let guess_chars: Vec<char> = guess_str.chars().collect();
    let guess: Result<u32, _> = guess_str.trim().parse();


    if let Ok(num) = guess {
        loopfn(num,guess_chars,guess_str,random,random_str.clone(),random_chars.clone());
    } else {
        println!("Give proper number");
    }


fn loopfn(mut my_int:u32,my_int_chars:Vec<char>,my_int_str:String,r:i32,r_str: String,r_chars:Vec<char>){

    loop {
        if my_int > 99999 {
            my_int = my_int / 10;
        } else if my_int < 1000 {
            my_int = my_int * 10;
        } else {
            break;
        }
    }




    for n in 0..my_int_chars.len()
    {
        if my_int_chars.get(n) == r_chars.get(n)
        {
            let styledint = my_int_chars[n].with(Color::Green).attribute(Attribute::Bold);
            print!("{}",styledint);
        }
        else if  r_chars.iter().any(|x| *x==my_int_chars[n]){
            let styledint = my_int_chars[n].with(Color::Yellow).attribute(Attribute::Bold);
            print!("{}",styledint);   
        }
        else{
            print!("{}",my_int_chars[n]);   
        }
    }


}
}
}




fn welcome() {
    print!("\x1B[2J\x1B[1;1H");

    println!("Welcome to guessing game!");
    println!("I have a 4 digit number for you. Can you guess what it is?");
}

fn randomnum()->i32
{
    return thread_rng(). gen_range(1_00_00..=9_99_99);
}