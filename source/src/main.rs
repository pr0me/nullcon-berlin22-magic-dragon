pub mod puf;
pub mod utils;

use std::io;
use std::io::prelude::*;
use std::fs;
use rand::Rng;
use ndarray::s;

const LINES: [&str; 6] = ["Come on!", "Come on, come on!", "You nearly caught me!", "So close, hihi", "Catch me!", "You almost got me"];

fn greet() {
    println!("{{ Greetings, traveler, come catch me, hehe! }}");
    let d = "
                       ^     ^                  
                      / \\  / \\                 
                     /.  \\/   \\      |\\___/|   
  *----*           / / |  \\    \\  __/  O  O\\   
  |   /          /  /  |   \\    \\_\\/  \\     \\     
 / /\\/         /   /   |    \\   _\\/    '@___@      
/  /         /    /    |     \\ _\\/       |U
|  |       /     /     |      \\\\/        |
\\  |     /_     /      |       \\  )   \\ _|_
\\   \\       ~-./_ _    |    .- ; (  \\_ _ _,\\'
~    ~.           .-~-.|.-*      _        {-,
 \\      ~-. _ .-~                 \\      /\'
  \\                   }            {   .*
   ~.                 '-/        /.-~----.
     ~- _             /        >..----.\\\\\\
         ~ - - - - ^}_ _ _ _ _ _ _.-\\\\\\
    ";
    println!("{}", d);
}

fn motivate() {
    let d = r#"
    ,{
        '_}
     ;._'.             .-'
     (             \-./ ;.-;
      ; _      |'--,| `  '< ___,
        __)     \`-.__.   /`.-'/
      {;         `/o(o \ | / ,'
        ;  _  __.-'-'`-'  \'`
        ' (-,`  .-.     _ -.      
           /     _))  _/    |        
    "#;
    println!("{}\n{{ Keep trying, you can do it, hihi! }}", d);
}

fn win() {
    let d = r#"
                 __/>^^^;:,
    __  __      /-.       :,/|/|
   /  \/  \  __/ ^         :,/ \__
  |        |(~             ;/ /  /
  \       {  `-'--._       / / ,<  ___
   \      /,__.   /=\     /  _/  >|_'.
    \    /  `_ `--------'    __ / ',\ \
     \  / ,_// ,---_____,   ,_  \_  ,| |
      \/   `--' |=|           \._/ ,/  |
                 \=\            `,,/   |
                  \=\            ||    /
                   \=\____       |\    \
                  / \/    `     <__)    \
                  | |                    |
                ,__\,\                   /
               ,--____>    /\.         ./
               '-__________>  \.______/
    "#;
    println!("{}", d);
    println!("{{ Yay, you caught me! }}");
    let flag = fs::read_to_string("./flag.txt").unwrap();
    println!("{}", flag);
}

fn speak() {
    let mut rng = rand::os::OsRng::new().unwrap();
    let line = *rng.choose(&LINES).unwrap();
    println!("{{ {} }}", line);
}

fn main() {
    let arbiter = puf::XORArbiterPUF::new(64, 4, 0.0, false).unwrap();
    let mut buffer = String::new();
    // let mut buffer: Vec<u8> = Vec::new();
    let mut correct_counter = 0;

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    greet();
    loop {
        let challenges = utils::get_challenges(64, 64);
        let response = arbiter.eval(&challenges);
        for i in 0..64 {
            println!(
                "{:?}",
                challenges.slice(s![i, ..]).to_vec()
            );

            stdin.read_line(&mut buffer).unwrap();
            if buffer.eq(&format!("{}\n", response[i])) {
                correct_counter += 1;
                speak();
            } else {
                correct_counter = 0;
                println!("{{ So close, hehe! Maybe try {} next time! }}", response[i]);
            }

            if correct_counter >= 64 {
                win();
            }
            buffer.clear();
        }
        motivate();
    }
}
