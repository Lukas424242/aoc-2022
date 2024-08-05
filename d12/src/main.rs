use std::fs::read_to_string;
fn main() {
	let input = read_to_string("input.txt").unwrap();
    let tokens: Vec<String> = input.split('\n').map(String::from).collect();

    let mut daten:Vec<daten> = Vec::new();

    for i in 0..tokens.len(){
        let mut zeile:Vec<String> = (tokens[i].clone()).split(' ').map(String::from).collect();

        let mut zeichen:Vec<char> = Vec::new();
        let mut zahlen:Vec<i32> = Vec::new();

        for i in zeile[0].chars(){
            zeichen.push(i);
        }

        for i in zeile[1].chars(){

            if i!=','{
                zahlen.push(String::from(i).clone().parse().unwrap());
            }
        }
        let daten1 = daten{
            listezeichen:zeichen,
            listezahlen:zahlen
        };
        daten.push(daten1);
    }

    let mut gesamt=0;
    for i in 0..daten.len(){

        let mut teil=0;
        let mut zeile:Vec<char> = daten[i].listezeichen.clone();
        let mut zahlen:Vec<i32> = daten[i].listezahlen.clone();

        let mut index=0;
        part1BF(&mut zeile, &mut zahlen, &mut teil, index);
        gesamt = gesamt + teil;


    }
    println!("{}", gesamt);

}

struct daten{
    listezeichen: Vec<char>,
    listezahlen: Vec<i32>
}

fn part1BF(zeile:&mut Vec<char>, zahlen: &Vec<i32>, teil:&mut i32, mut index:usize){

    if index == zeile.len() {
        // valide
        
        if isvalid(zeile, zahlen) {
            *teil = *teil+1;
        }
        else { 
            //Stop here
        }

    }
    else{

        // wenn fragezeichen, dann weiter andernfalls weitergehen //zähler
        
        if zeile[index] =='?' {

            // nachher frageziechen ergänzen wichtig, da das rückgängig gemacht werden müssen

            zeile[index] = '.';
            part1BF(zeile, zahlen, teil, index+1);
            zeile[index] ='?';


            zeile[index] ='#';
            part1BF(zeile, zahlen, teil, index+1);
            zeile[index] ='?';

        }
        else {
            part1BF(zeile, zahlen, teil, index+1);

        }

    }

}

fn isvalid(zeile: &Vec<char>, zahlen:&Vec<i32>) ->bool{

    let zeil:Vec<char> = zeile.clone();

    let mut laengewirklichkeit:Vec<Vec<char>> =Vec::new();

    let mut zeichen:Vec<char> = Vec::new();

    for i in 0..zeil.len() {


        if zeil[i] =='#' {
            zeichen.push('#');
        }
        else {

            if !zeichen.is_empty() {
                laengewirklichkeit.push(zeichen.clone());
                zeichen.clear();
            }

        }
        
        if i == zeil.len()-1 && !zeichen.is_empty()  {
            laengewirklichkeit.push(zeichen.clone());

        }


    }

    if laengewirklichkeit.len() !=zahlen.len() {

        return false;
    }
    else {


        for i in 0..zahlen.len(){

            if zahlen[i] != laengewirklichkeit[i].len() as i32 {
                return false;
            }

        }

        return true;

    }



    true
}

//brauchen wir eigentlich gar nicht
fn boolarray(zeile:&Vec<char>) -> Vec<bool>{

    //let mut boolarray:Vec<bool> = Vec::new();
    let mut boolarray: Vec<bool> = vec![false; zeile.len()];

    for i in 0..zeile.len(){

        if zeile[i] == '?' {
            boolarray[i] = true;
        }

    }

    return  boolarray;

}
