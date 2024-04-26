use std::collections::{HashMap, HashSet};
use rand::distributions::{Alphanumeric, DistString};

fn main() {
    let palabra = "Esto es un texto";
     
    println!("Ejecutando: {palabra}" );

    let palabra2 = "Esto";
    if palabra.contains(palabra2) {
        println!("Checkea por palabras");
        let mut my_float: f64 = 56.1;
        my_float = my_float.atan2(1.1);
        println!("{my_float}");
        arrays();
        mapas();

    }else {
        print!("No check")
    }
}


fn arrays() {
    let mut my_array: Vec<&str> = Vec::new();
    my_array.push("Est");
    println!("{:?}", my_array);
    let mut my_array2: HashSet<&str> = vec!["A","B","C","D"].into_iter().collect();
    my_array2.insert("element");
    println!("{:?}", my_array2);


}


fn mapas() {

    //let mut my_map: HashMap<String, i32> = vec![("Pos1", 23)].into_iter().collect();
    let mut my_map: HashMap<String, i32> = HashMap::new();
    for x in 1..=10 {
        let palabra = random_words();
        //let palabra2: &str = &palabra;
        my_map.insert(palabra, x);
        
    }
    println!("{:?}", my_map);
    
}

fn random_words() -> String {
    let _random_letters: String = Alphanumeric.sample_string(&mut rand::thread_rng(),16);
    //let palabra: &str = &*_random_letters;
    return _random_letters;

}

