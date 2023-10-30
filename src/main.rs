mod mylib; // Importa el módulo mylib

fn main() {
    if let Err(err) = mylib::query() {
        eprintln!("Error: {:?}", err);
    }
}
