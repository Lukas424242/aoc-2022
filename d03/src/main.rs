
use std::{fs::read_to_string};
fn main() {
    let input: String = read_to_string("demo.txt").unwrap();
    let tokens: Vec<String> = input.split('\n').map(String::from).collect();

    let daten:Vec<Vec<(i32,i32)>> = tokens.iter().filter(|x:&&String| !x.is_empty()).map(|x|x.split(",").map(|y|{

        let k = y.to_owned();
        let d:Vec<i32> = k.split("-").map(|c| c.parse().unwrap()).collect();
        (d[0], d[1])

    }).collect()
).collect();

// PART 1
let score:i32 = daten.iter().map(|x|{
    let e1 = x[0];
    let e2 =x[1];

    // ist e1 in e2 enthalten
    if (e1.0 >= e2.0 && e1.1 <= e2.1) {
        return 1;
    }
    else if e1.0 <= e2.0 && e1.1 >= e2.1  {
        return 1;
    }

    0  

}).sum();

println!("{}", score);

// Part2
// 5 7
// 7 9

// 7 9
// 5 7
let score2:i32 = daten.iter().map(|x|{
    let e1 = x[0];
    let e2 =x[1];

    if e1.0 >= e2.0{
        return 1;
    }
    else if  {
        
    }
   
   0
    

}).sum();
println!("{}", score2);

}
