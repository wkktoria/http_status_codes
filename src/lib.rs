use colored::Colorize;
use reqwest::Error;
use scraper::{Html, Selector};

const DESCRIPTION_URL: &str = "https://developer.mozilla.org/en-US/docs/Web/HTTP/Status";
const IMAGE_URL: &str = "https://http.dog";

const NAME_SELECTOR: &str = "header>h1";
const DESCRIPTION_SELECTOR: &str = "div.section-content>p:first-of-type";

const NON_EXISTING_STATUS_TEXT_CONTENT: &str = "Page not found";

struct Status {
    name: String,
    description: String,
    image_link: String,
}

impl Status {
    fn print_info(&self) {
        println!(
            "\n{}\n\n{}\nLink to image: {}\n",
            self.name.green(),
            self.description,
            self.image_link.blue()
        );
    }
}

pub fn display_info(status_code: &String) {
    let status = create_status(&status_code);

    match status {
        Ok(s) => s.print_info(),
        Err(e) => println!("\n{}\n", e.red()),
    }
}

fn create_status(status_code: &String) -> Result<Status, &'static str> {
    let url = format!("{}/{}", DESCRIPTION_URL, &status_code);
    let html = get_html(&url);

    let document: Result<Html, &'static str> = match html {
        Ok(s) => {
            if status_exists(&s) {
                Ok(get_document(&s))
            } else {
                Err("Status code doesn't exist")
            }
        }
        Err(_) => Err("Failed to get document."),
    };

    match document {
        Ok(h) => {
            let name = get_by_selector(&h, &NAME_SELECTOR);
            let description = get_by_selector(&h, &DESCRIPTION_SELECTOR);

            Ok(Status {
                name: name,
                description: description,
                image_link: format!("{}/{}.jpg", IMAGE_URL, &status_code.trim())
                    .trim()
                    .to_string(),
            })
        }
        Err(e) => Err(e),
    }
}

fn get_html(url: &String) -> Result<String, Error> {
    let response = reqwest::blocking::get(url)?;
    response.text()
}

fn get_document(html: &String) -> Html {
    Html::parse_document(html)
}

fn status_exists(text_content: &String) -> bool {
    !text_content.contains(&NON_EXISTING_STATUS_TEXT_CONTENT)
}

fn get_by_selector(document: &Html, selector: &str) -> String {
    let mut result = String::new();
    let selector = Selector::parse(&selector).unwrap();

    for element in document.select(&selector) {
        result = element.text().into_iter().collect();
    }

    result.trim().to_string()
}
