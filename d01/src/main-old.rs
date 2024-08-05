use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    // man nimmt den string als input
    // und speichert in dem slice dann die referenzen
    // auf den string, wo was drinne steht
    //let tokens: Vec<&str> = input.split(' ').collect();
    let tokens: Vec<String> = input.split('\n').map(String::from).collect();
    let mut werte: Vec<i32> = Vec::new();

    for token in tokens {

        if token.len() ==0{
            werte.push(0);
        }
        else {
            let vorheriges = werte.pop().unwrap();
            let my_int = token.parse::<i32>().expect("you are an idiot");

            let neu = my_int + vorheriges;
            werte.push(neu);
        }


    }

    //println!("Input {}", input)

}
