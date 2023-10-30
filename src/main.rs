mod lib;

fn main() {
    // Query
    println!("Querying data...");
    if let Err(err) = lib::query() {
        eprintln!("Error: {:?}", err);
    }
}
