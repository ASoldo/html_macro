# Rust HTML Generator with Maud

## Project Overview

This project demonstrates how to generate HTML files using Rust and the Maud templating library. It includes the generation of multiple pages (Home and About) and ensures the necessary directory structure is created.

## Steps and Explanation

### 1. Initialize Rust Project

We started by creating a new Rust project and adding the Maud templating library to generate HTML.

### 2. Generate HTML with Maud

The `main.rs` file contains functions to generate HTML for the Home and About pages and save them to the `dist` directory. It ensures that the necessary directories are created before saving the files.

#### Functions

- **main**: The entry point of the program. It ensures the `dist` and `dist/pages` directories exist and then generates and saves the Home and About pages.

- **generate_home_page**: Generates the HTML for the Home page using Maud.

- **generate_about_page**: Generates the HTML for the About page using Maud.

- **header**: Generates the HTML for the header navigation.

- **save_html_to_file**: Saves the generated HTML to a specified file path.

## How to Run

1. **Install Rust**:

   - Follow the instructions at [rustup.rs](https://www.rustup.rs/) to install Rust.

2. **Build and Run the Project**:

   - Navigate to the project directory and run the following commands:

   ```sh
   cargo build
   cargo run
   ```
