mod mylib;
use mylib::query; // Importa el módulo query desde mylib

fn main() {
    if let Err(err) = query::query() {
        // Manejar el error aquí
    }
}
