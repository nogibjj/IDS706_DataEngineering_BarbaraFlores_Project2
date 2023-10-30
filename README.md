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


### Prerequisites

- Rust programming environment: Make sure you have Rust installed on your system.

### Dependencies

This project uses the following dependencies:

- `rusqlite`: For SQLite database interaction.
- `csv`: To work with CSV files.
- `reqwest`: To make HTTP requests.
- `assert_cmd`: For testing command-line applications.
- `predicates`: For testing command-line output.


To install the Rust dependencies, add them to your `Cargo.toml` file and run `cargo build`.



