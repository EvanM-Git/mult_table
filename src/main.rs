use std::io;

fn main() {
    let mul = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut input = String::new();

    let numero: i64 = loop {
        println!("Por favor, ingresa un número para generar su tabla de multiplicar:");

        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada del usuario");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Por favor, ingresa un número válido.");
                continue;
            }
        };
    };

    println!("\nAquí está tu tabla de multiplicar:\n");

    for number in mul {
        println!("{} x {} = {}", numero, number, numero * number);
    }
}
