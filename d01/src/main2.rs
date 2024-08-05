use std::fs::read_to_string;

fn main() {
    // let input = read_to_string("input.txt").unwrap();
    //
    // let tokens: Vec<&str> = input.split('\n').collect();
    //
    // let mut sum: i32 = 0;
    // let mut sumMax: i32 = 0;
    //
    // for token in tokens {
    //     // leere Zeile
    //     if token.len() == 0 {
    //         println!("Sum: {}", sum);
    //
    //         if sum > sumMax {
    //             sumMax = sum;
    //         }
    //
    //         sum = 0;
    //         continue;
    //     }
    //
    //     let value = token.parse::<i32>().expect("idiot made invalid input");
    //     sum = sum + value;
    // }
    //
    // if sum > sumMax {
    //     sumMax = sum;
    // }
    //
    // println!("Max Sum: {}", sumMax)

    let input = read_to_string("input.txt").unwrap();

    // man nimmt den string als input
    // und speichert in dem slice dann die referenzen
    // auf den string, wo was drinne steht
    //let tokens: Vec<&str> = input.split(' ').collect();
    let tokens: Vec<String> = input.split('\n').map(String::from).collect();
    let mut werte: Vec<i32> = Vec::new();
    werte.push(0);

    for token in tokens {

        if token.len() ==0{
            werte.push(0);
        }
        else {
            let vorheriges = werte.pop().expect("not available");
            let my_int = token.parse::<i32>().expect("you are an idiot");

            let neu = my_int + vorheriges;
            werte.push(neu);
        }


    }
    werte.sort();

    println!("{:?}" ,werte);
}
