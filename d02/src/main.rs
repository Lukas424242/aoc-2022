use std::fs::read_to_string;
fn main() {

    let input = read_to_string("input.txt").unwrap();
    let tokens: Vec<String> = input.split('\n').map(String::from).collect();




}

fn part2(tokens: Vec<String>){
    let daten:Vec<Runde2> = tokens.into_iter().map(|a|{

    let chars:Vec<String> = a.split(" ").map(|x| x.to_owned()).collect();


    let runde = Runde2{
            gegner: Wahl::parseOpponent(chars[0].to_owned()),
            ich: chars[1].to_owned()
        };
        runde
    }).collect();

    
    
    let score2:i32 = daten.iter().map(|x|{

        
        // y draw X lose Z win

        let mut score=0;
        if x.ich == "Y".to_string() {

            match x.gegner {
                Wahl::Rock=> score = score+1+3,
                Wahl::Paper=> score = score+2+3,
                Wahl::Scissor=> score = score+3+3,
                _=> unreachable!("Fuck")
            };
            
        }
        // Lose
        else if x.ich == "X".to_string() {

            match x.gegner {
                Wahl::Rock=> score = score+3+0 ,
                Wahl::Paper=> score = score+1+0 ,
                Wahl::Scissor=> score = score+2+0 ,
                _=> unreachable!("Fuck")

            
            };
            
        }
        else {
            // Z
            match x.gegner {
                Wahl::Rock=> score = score+2+6 ,
                Wahl::Paper=> score = score+3+6 ,
                Wahl::Scissor=> score = score+1+6 ,
                _=> unreachable!("Fuck")

            
            };
        }


        println!("{}", score);

        score
        
    }).sum();

    println!("{}", score2);


}

fn part1(tokens: Vec<String>){
    let daten:Vec<Runde> = tokens.into_iter().map(|a|{

        let chars:Vec<String> = a.split(" ").map(|x| x.to_owned()).collect();


        let runde = Runde{
            gegner: Wahl::parseOpponent(chars[0].to_owned()),
            ich: Wahl::parseI(chars[1].to_owned())
        };
        runde
    }).collect();

    let score:i32 = daten.iter().map(|x|{
        
        // 0 won 1 unentschieden 2 verloren
        let mut won = 0;

        let mut base = match x.ich {

            Wahl::Rock=> 1,
            Wahl::Paper=> 2,
            Wahl::Scissor=> 3,

            _=> panic!("")     
        };

        let base2 = match x.gegner {

            Wahl::Rock=> 1,
            Wahl::Paper=> 2,
            Wahl::Scissor=> 3,

            _=> panic!("")     
        };

        /* 
        won = if base == base2 {
            1
        } else if (base == 1 && base2 == 2) || (base == 2 && base2 == 3) || (base == 3 && base2 == 1) {
            2
        } else {
            0
        };
        */
        // Above refacored with help, but it works
        if base == base2{
            won =1;
        }
        else if base ==1 && base2 ==2{
            won =2;
        }
        else if base ==2 && base2 ==1 {
            won=0;
        }
        else if base ==3 && base2 ==2 {
            won =0;
        }
        else if base ==2 && base2 ==3 {
            won =2;   
        }
        else if base ==1 && base2 ==3 {
            won=0;
        }
        else if base ==3 && base2 ==1 {
            won=2;
        }


        if won == 0 {
            base+=6;
        }
        else if won==1  {
            base+=3;
        }
        else if won ==2 {
            base+=0;

        }
        base

    }).sum();

}

#[derive(Clone, Copy, Debug)]
struct Runde{
    gegner:Wahl,
    ich:Wahl
}

struct Runde2{
    gegner:Wahl,
    ich:String

}

#[derive(Clone, Copy, Debug)]
enum Wahl {
    Rock,
    Paper,
    Scissor,
    None
}

impl Wahl {

    fn parseOpponent(x:String)->Wahl{

        match x.as_str() {
            "A"=> Wahl::Rock,
            "B"=> Wahl::Paper,
            "C"=> Wahl::Scissor,
            _=> unreachable!("parsing Error")
        }
    }

    fn parseI(x:String)->Wahl{

        match x.as_str() {
            "Y"=> Wahl::Paper,
            "X"=> Wahl::Rock,
            "Z"=> Wahl::Scissor,
            _=> unreachable!("parsing Error")
        }
    }
    
}