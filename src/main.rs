use maud::{html, Markup, PreEscaped, DOCTYPE};
use std::fs::{create_dir_all, File};
use std::io::Write;

/// The main function generates the Home and About pages
/// and saves them as HTML files in the `dist` directory.
///
/// It ensures that the necessary directories (`dist` and `dist/pages`)
/// are created before attempting to save the files.
fn main() {
    // Ensure the dist directory exists
    create_dir_all("dist").expect("Unable to create dist directory");

    // Generate and save the Home page
    let home_markup: PreEscaped<String> = generate_home_page();
    save_html_to_file(home_markup, "dist/index.html");

    // Ensure the dist/pages directory exists
    create_dir_all("dist/pages").expect("Unable to create dist/pages directory");

    // Generate and save the About page
    let about_markup: PreEscaped<String> = generate_about_page();
    save_html_to_file(about_markup, "dist/pages/about.html");

    println!("Home and About pages have been created successfully.");
}

/// Generates the HTML for the Home page.
///
/// # Returns
///
/// A `PreEscaped<String>` containing the HTML for the Home page.
fn generate_home_page() -> PreEscaped<String> {
    html!(
        (DOCTYPE)
        html {
            head {
                title { "Home" }
                script src="/main.js" {}
                link rel="stylesheet" href="/main.css" {}
            }
            body {
                (header())
                h1 { "Home" }
                p { "Welcome to the Home page!" }
                p id="counter" { "0" }
                button onclick="incrementCounter()" { "Increment" }
            }
        }
    )
}

/// Generates the HTML for the About page.
///
/// # Returns
///
/// A `PreEscaped<String>` containing the HTML for the About page.
fn generate_about_page() -> PreEscaped<String> {
    html!(
        (DOCTYPE)
        html {
            head {
                title { "About" }
                link rel="stylesheet" href="/main.css" {}
            }
            body {
                (header())
                h1 { "About" }
                p { "This is the About page." }
            }
        }
    )
}

/// Generates the HTML for the header navigation.
///
/// # Returns
///
/// A `Markup` containing the HTML for the header.
fn header() -> Markup {
    html! {
        nav {
            ul {
                li { a href="/" { "Home" } }
                li { a href="pages/about.html" { "About" } }
            }
        }
        div class="soldo-class" {
            p { "Ojla" }
        }
    }
}

/// Saves the generated HTML to a file.
///
/// # Arguments
///
/// * `markup` - A `PreEscaped<String>` containing the HTML to save.
/// * `file_path` - A `&str` specifying the path where the HTML file should be saved.
///
/// # Panics
///
/// This function will panic if the file cannot be created or written to.
fn save_html_to_file(markup: PreEscaped<String>, file_path: &str) {
    let html_string = markup.into_string();
    let mut file = File::create(file_path).expect("Unable to create file");
    file.write_all(html_string.as_bytes())
        .expect("Unable to write data");
}
