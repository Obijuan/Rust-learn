fn main() {

    //-- Borrar el terminal
    print!("{esc}[2J", esc = 27 as char);

    //-- Situar cursor en posicion 1,1
    print!("{esc}[1;1H", esc = 27 as char);

    //-- Imprimir un mensaje de prueba
    println!("Hello, world!");
}
