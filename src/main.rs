mod create;  // Import the create module
mod read;    // Import the read module
mod delete;  // Import the delete module
mod update;  // Import the update module

fn main() {
    // CREATE: Call the function to create the database from a CSV file
    let db_file = "data/weweoweo.db";

    if let Err(err) = create::create_database_from_csv("data/WorldSmall.csv", db_file) {
        eprintln!("Error creating the database: {}", err);
    } else {
        println!("Database created successfully.");
    }

    // READ:  Call the function to perform the read query
    if let Err(err) = read::query(db_file) {
       eprintln!("Error: {:?}", err);
    }

    // UPDATE: Call the update function to transform the table (e.g., converting 'country' to uppercase)
    if let Err(err) = update::update_table(db_file) {
        eprintln!("Error updating the table: {}", err);
    } else {
        println!("Table updated successfully.");
    }

    // DELETE:  Delete the database if needed
    if let Err(err) = delete::delete_database(db_file) {
        eprintln!("Error deleting the database: {}", err);
    } else {
        println!("\n\nDatabase deleted successfully.");
    }

}
