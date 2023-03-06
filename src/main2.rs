fn main() {
    let proyect : &str = "PC time with Rust";
    let create_age : u16 = 2023;

    println!("Por favor ingresa tu nombre");
    let mut nombre : String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    println!("Por favor ingresa tu edad");
    let mut edad : String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();
    let edad_init : u8 = edad.trim().parse().unwrap();

    if edad_init >= 18 {
        println!("Puedes entrar a la discoteca");
    } else {
        println!("Eres menor de edad, no puedes entrar a la discoteca");
    }

    println!("El nombre del proyecto es '{}' y fue creado en el año {}", proyect, create_age);
    println!("Tu nombre es '{}' y tu edad es {} años", nombre, edad_init);
}
