[![lint](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/lint.yml)
[![build](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/build.yml/badge.svg)](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/build.yml)
[![test](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/test.yml/badge.svg)](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/test.yml)
[![format](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/format.yml/badge.svg)](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/actions/workflows/format.yml)


IDS706_DataEngineering_BarbaraFlores_Project2


## 📂 Rust CLI Binary with SQLite

The objective of this project is to develop a Rust command-line application (CLI) that interacts with an SQLite database. This involves creating and manipulating the database, using GitHub Copilot, generating an optimized binary as a GitHub Actions artifact, properly configuring GitHub Actions to test, build, and lint the Rust code, and producing a high-quality demonstration video explaining and showcasing the project's functionality.


### 📊 Database

In particular, the [world-small.csv](https://raw.githubusercontent.com/sejdemyr/sejdemyr.github.io/master/r-tutorials/basics/data/world-small.csv) database was used, which was employed in the `"Practical Data Science"` class taught by Nick Eubank. This database contains information about some countries, their regions, and their values for `Polity IV` and `gdppcap08`.

- The `polityIV` variable in this dataset is an expert score for a country's authoritarianism. Traditionally, values of -10 represent extreme autocracies, while values of 10 denote consolidated liberal democracies. However, in this dataset, they have been rescaled to range from 0 to 20, where 0 represents an extreme autocracy, and 20 represents a consolidated liberal democracy.

- The variable `gdppcap08` represents the GDP per Capita values for countries in the year 2008.

### 🌳 Directory Tree
This repository includes the following directory and file structure:

```bash
.
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── LICENSE
├── Makefile
├── README.md
├── data
│   └── WorldSmall.csv
├── images
├── setup.sh
└── src
    ├── create.rs
    ├── delete.rs
    ├── main.rs
    ├── read.rs
    └── update.rs
```

- [`src/`](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/tree/main/src): This folder contains the Rust application source code with CRUD operations.
- [`data/`](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/tree/main/src) : Here you can find the CSV database file with input data.
- [`images/`](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/tree/main/images): In this directory, you'll discover visual aids, examples of program usage, and tutorials to help you better understand and utilize the functionality of the Rust CLI application. 


#### User Instructions

1. Clone this repository: Go to the [repository page](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/), then click on "Use this template," and then "Create New Repository."

![Step 1: Clone the repository](https://raw.githubusercontent.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/main/images/Step01.png)


2. Create a Codespace
3. Install Rust using:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
1
source "$HOME/.cargo/env"
```

Make sure you have Rust installed on your system, using:
```bash
rustc --version
cargo --version
```


### Dependencies

This project uses the following dependencies:

- `rusqlite`: For SQLite database interaction.
- `csv`: To work with CSV files.
- `reqwest`: To make HTTP requests.
- `assert_cmd`: For testing command-line applications.
- `predicates`: For testing command-line output.


To install the Rust dependencies, add them to your `Cargo.toml` file and run `cargo build`.

### CRUD Operations

**CREATE**
In the main.rs file, the CREATE operation involves creating a database from a CSV file. It accomplishes this by using the create::create_database_from_csv function. This function connects to an SQLite database, creates a table, and then reads data from the provided CSV file, inserting it into the table.

**UPDATE**
The UPDATE operation is carried out using the update::update_table function. It transforms the database table, converting specific columns, such as 'country' and 'region,' to uppercase.

**READ**
The READ operation, defined in the read::query function, performs queries on the database. In the project, two sample queries are demonstrated. The first query selects a random sample of five records from the database and displays them. The second query groups records by the 'region' column, counting the number of records in each region, and then displays the results.

**DELETE**
The DELETE operation, managed by the delete::delete_database function, is responsible for deleting the SQLite database file. It waits for a duration before performing the deletion. If the database file exists, it is removed.

You can use these descriptions to further enhance your README, providing a clear understanding of each CRUD operation within your Rust CLI application.

![Screen Shot 2023-10-30 at 08 51 54](https://github.com/nogibjj/IDS706_DataEngineering_BarbaraFlores_Project2/assets/143648839/ad2dcb2c-b081-4803-9afe-7e0602171ae9)

### 🤖 GitHub Copilot Assistance
GitHub Copilot, an AI-powered coding assistant, played a significant role in the development of this Rust CLI application. It assisted in generating code snippets, suggesting best practices, and improving the overall code quality. Specifically, it provided valuable support in the following areas:

Code Generation: GitHub Copilot helped generate boilerplate code for creating, updating, reading, and deleting operations, making the development process more efficient.

Error Handling: It offered suggestions for effective error handling, ensuring that the application could gracefully handle potential issues.

Documentation: GitHub Copilot assisted in writing clear and concise comments, making the codebase more understandable for developers.

Testing: It provided guidance on writing unit tests to ensure the reliability of the application.

Optimization: Copilot suggested optimizations to enhance the performance of the Rust binary generated by the project.

By leveraging GitHub Copilot's capabilities, the development process was streamlined, resulting in a well-structured and efficient Rust CLI application that interacts with an SQLite database.

### 📊 Video Tutorial

