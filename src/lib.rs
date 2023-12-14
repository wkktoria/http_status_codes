use colored::Colorize;
use reqwest::Error;
use scraper::{Html, Selector};

const DESCRIPTION_URL: &str = "https://developer.mozilla.org/en-US/docs/Web/HTTP/Status";
const IMAGE_URL: &str = "https://http.dog";

const CODE_SELECTOR: &str = "header>h1";
const DESCRIPTION_SELECTOR: &str = "div.section-content>p:first-of-type";

const NON_EXISTING_STATUS_TEXT: &str = "Page not found";

struct HttpStatusCode {
    code: String,
    description: String,
    image_link: String,
}

impl HttpStatusCode {
    fn print_info(&self) {
        println!(
            "\n{}\n\n{}\nLink to image: {}\n",
            self.code.green(),
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

fn create_status(status_code: &String) -> Result<HttpStatusCode, &'static str> {
    let url = format!("{}/{}", DESCRIPTION_URL, &status_code);
    let response_text = get_response_text(&url);

    let html: Result<Html, &'static str> = match response_text {
        Ok(s) => {
            if status_exists(&s) {
                Ok(get_html(&s))
            } else {
                Err("Supplied http status code doesn't exist!")
            }
        }
        Err(_) => Err("Failed to get html document."),
    };

    match html {
        Ok(h) => {
            let code = get_by_selector(&h, &CODE_SELECTOR);
            let description = get_by_selector(&h, &DESCRIPTION_SELECTOR);

            Ok(HttpStatusCode {
                code,
                description: description,
                image_link: format!("{}/{}.jpg", IMAGE_URL, &status_code.trim())
                    .trim()
                    .to_string(),
            })
        }
        Err(e) => Err(e),
    }
}

fn get_response_text(url: &String) -> Result<String, Error> {
    let response = reqwest::blocking::get(url)?;
    response.text()
}

fn get_html(html: &String) -> Html {
    Html::parse_document(html)
}

fn status_exists(text_content: &String) -> bool {
    !text_content.contains(&NON_EXISTING_STATUS_TEXT)
}

fn get_by_selector(document: &Html, selector: &str) -> String {
    let mut result = String::new();
    let selector = Selector::parse(&selector).unwrap();

    for element in document.select(&selector) {
        result = element.text().into_iter().collect();
    }

    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use crate::get_by_selector;
    use scraper::Html;

    #[test]
    fn test_get_by_selector() {
        let html = r#"
        <ul>
            <li>First</li>
            <li>Second</li>
            <li>Third</li>
        </ul>
        "#;
        let fragment = Html::parse_fragment(html);
        let first_li_selector = "li:first-of-type";

        assert_eq!("First", get_by_selector(&fragment, first_li_selector))
    }
}
