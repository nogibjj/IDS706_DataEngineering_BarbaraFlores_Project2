// read.rs

// Importa el módulo main
mod read;



fn main() {
    // Query
    read::mi_funcion_principal();
    println!("Querying data...");
    if let Err(err) = read::query() {
        eprintln!("Error: {:?}", err);
    }
}


