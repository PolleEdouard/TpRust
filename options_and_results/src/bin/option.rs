fn main(){
    let sentence1: &str = "Bonjour Limoges";
    let sentence2: &str = "";
    print_first_word2(sentence1);
    //print_first_word2(sentence2);
    iterate_over_worlds(sentence1);
}

// fn print_first_word1(phrase: &str){
        
//         let mut words= phrase.split_whitespace();
//         let word1 = words.next()  ;
//         match word1 {
//             None => println!("la chaîne est vide"),
//             Some(w1) => println!("le Premier mot est {}",w1),
//         }
// }

fn print_first_word2(phrase: &str){
        
    let mut words= phrase.split_whitespace();
    let word1 = words.next()  ;
    word1.expect("la chaîne doit être non vide !");
}

fn iterate_over_worlds(phrase: &str){
    let mot: Vec<&str> = phrase.split_whitespace().collect();
    for mot in mot {
        println!("Mot : {}",mot);
    } 
}
