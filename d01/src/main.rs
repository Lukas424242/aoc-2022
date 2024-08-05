use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let tokens: Vec<String> = input.split("\n\n").map(String::from).collect();


    let mut liste: Vec<i32> = tokens.into_iter().map(|x| x.split("\n").map(|xd| xd.parse::<i32>().unwrap()).sum()).collect();


    liste.sort();
    liste.reverse();

    println!("{}", liste[0]);

    // Part 2
    println!("{}", liste[0] + liste[1]+liste[2]);

    let mut d = String::from("a");
     let d1 = &d;

     let d2 = &mut d;




}
