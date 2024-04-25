
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

    }else {
        print!("No check")
    }
}


fn arrays() {
    let mut my_array: Vec<&str> = Vec::new();
    my_array.push("Est");
    print!("{:?}", my_array);

}