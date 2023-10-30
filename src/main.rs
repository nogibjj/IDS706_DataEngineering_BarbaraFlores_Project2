// src/main.rs
extern crate ids706_data_engineering_barbara_flores_project2; 
use ids706_data_engineering_barbara_flores_project2::my_library_function;
use ids706_data_engineering_barbara_flores_project2::query;



fn main() {
    println!("¡Hola desde el programa principal!");
    my_library_function();
    // Query
    println!("Querying data...");
    if let Err(err) = query() {
        eprintln!("Error: {:?}", err);
    

        
    }
}
