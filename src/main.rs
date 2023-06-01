use rand::thread_rng;
use rand::Rng;
use crossterm::style::{ Color, Attribute, Stylize};
use std::io;

fn main() {

    welcome();
    let random=randomnum();

    let random_str=random.to_string();
    let random_chars: Vec<char> = random_str.chars().collect();

loop{
    let mut guess_str = String::new();
    print!("\x1b[0m");
    io::stdin().read_line(&mut guess_str).expect("Failed to read line");
    let guess: Result<u32, _> = guess_str.trim().parse();


    if let Ok(num) = guess 
    {
       let i= loopfn(num,random,random_chars.clone());
        if i
        { continue;}
        else {break;}
        
    } 
    else {
        println!("Give proper number");
    }


fn loopfn(mut my_int:u32,r:i32,r_chars:Vec<char>)->bool{

    while my_int>99999||my_int<10000 {
        if my_int > 99999 {
            my_int = my_int / 10;
        } 
        else {
            my_int = my_int * 10;
        } 
    }

    let my_int_str=my_int.to_string();
    let my_int_chars: Vec<char>=my_int_str.chars().collect();

    let mut k="".to_string();
    let mut close=0;

    for n in 0..my_int_chars.len()
    {
        if my_int_chars.get(n) == r_chars.get(n)
        {
            k=format!("{}\x1b[32m{a}\x1b[0m",k,a=my_int_chars[n]);
            close+=3;
        }
        else if  r_chars.iter().any(|x| *x==my_int_chars[n]){
            k=format!("{}\x1b[33m{a}\x1b[0m",k,a=my_int_chars[n]);       
            close+=1;
        }
        else{
            k=format!("{}\x1b[31m{a}\x1b[0m",k,a=my_int_chars[n]);       
        }
        }
    
    println!("{}",k);

    if r.to_string()==my_int.to_string()
    {
        println!("You guessed it correct. Great job");
        io::stdin().read_line(&mut String::new()).unwrap();
        return false;
    }
    else if close==7{
        println!("So close, Try again:");
        return true;
    }
    else if close>9{
        println!("Nearly there, Try again:");
        return true;
    }
    else{
        println!("Try again:");
        return true;
    }


}
}
}




fn welcome() {
    print!("\x1B[2J\x1B[1;1H");
    let styled = "Welcome to guessing game"
    .with(Color::Blue)
    .on(Color::Black)
    .attribute(Attribute::Bold);

    println!("{}", styled);
    println!("I have a 5 digit number for you. Can you guess what it is?");
}

fn randomnum()->i32
{
    return thread_rng(). gen_range(1_00_00..=9_99_99);
}