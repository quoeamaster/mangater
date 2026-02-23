use scraper::{node::Node, ElementRef, Html, Selector};

use crate::entity::{HtmlImage, HtmlPlainTextAndImages};

fn clean_text(element: &ElementRef) -> String {
    let mut text = String::new();

    for node in element.children() {
        match node.value() {
            Node::Text(t) => {
                text.push_str(t); // don't trim/insert space for inline text
            }
            Node::Element(e) if e.name() != "style" && e.name() != "script" => {
                if let Some(child_elem) = ElementRef::wrap(node) {
                    let child_text = clean_text(&child_elem);

                    // Insert space only for block elements
                    //println!("*Child element: {}", e.name());
                    match e.name() {
                        "p" | "div" | "li" | "h1" | "h2" | "h3" => {
                            text.push(' ');
                            text.push_str(&child_text);
                            text.push(' ');
                        }
                        _ => {
                            text.push_str(&child_text);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    text
}

/// # Example
///
/// ```
/// use scraper::Html;
/// use mangater_sdk::util::html_parsing::parse_images;
///
/// const CONTENT: &str = r#"<img src="cat.jpg"><img src="dog.png">"#;
/// let images = parse_images(CONTENT.to_string());
/// assert_eq!(images.len(), 2);
/// assert_eq!(images[0].src, "cat.jpg");
/// assert_eq!(images[1].src, "dog.png");
/// ```
pub fn parse_images(content: String) -> Vec<HtmlImage> {
    let document = Html::parse_document(&content);
    parse_images_through_html(&document)
}

/// Parses the provided HTML document and extracts all `<img>` elements as a collection of `HtmlImage`.
///
/// This utility function takes a reference to a [`scraper::Html`] document and returns a vector of
/// [`HtmlImage`] structs, each containing the `src` attribute and the inner HTML of the image element.
///
/// # Arguments
///
/// * `document` - A reference to the parsed HTML content.
///
/// # Returns
///
/// A vector of [`HtmlImage`] structs representing all images found in the document.
///
fn parse_images_through_html(document: &Html) -> Vec<HtmlImage> {
    let selector = Selector::parse("img").unwrap();
    let images = document.select(&selector).collect::<Vec<_>>();
    let mut html_images = Vec::new();
    for image in images {
        if let Some(src) = image.value().attr("src") {
            let inner_html = image.html();
            html_images.push(HtmlImage {
                src: src.to_string(),
                inner_html: inner_html.to_string(),
            });
        }
    }
    html_images
}

pub fn parse_plain_text_and_images(content: String) -> HtmlPlainTextAndImages {
    let document = Html::parse_document(&content);
    let selector = Selector::parse("#mw-content-text").unwrap();
    let content = document.select(&selector).next().unwrap();

    let text = clean_text(&content);

    HtmlPlainTextAndImages {
        text: text,
        images: parse_images_through_html(&document),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    use scraper::{Html, Selector};
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_parse_html_01() -> Result<(), Box<dyn std::error::Error>> {
        //let client = Client::new(); // without user agent, the request will be blocked by the server
        let client = reqwest::Client::builder()
            .user_agent("mangater-sdk/0.1 (+https://github.com/quoeamaster/mangater)")
            .build()?;

        let response = client
            .get("https://en.wikipedia.org/wiki/NoSQL")
            .send()
            .await?
            .error_for_status()?;

        let body = response.text().await?;
        assert!(!body.is_empty());

        let document = Html::parse_document(&body);
        let mut file = File::create("testdata/wikipedia_nosql_scrap_result.txt").await?;

        // parse only images
        let selector = Selector::parse("img").unwrap();
        let images = document.select(&selector).collect::<Vec<_>>();

        for image in images {
            let src = image.value().attr("src").unwrap();
            let class = image.value().attr("class");
            if class.is_some() {
                file.write_all(
                    format!("Image src: {}, class: {}\n", src, class.unwrap()).as_bytes(),
                )
                .await?;
                println!("Image src: {}, class: {}", src, class.unwrap());
            } else {
                file.write_all(format!("Image src: {}\n", src).as_bytes())
                    .await?;
                println!("Image src: {}", src);
            }
        }

        // parse plain text
        let selector = Selector::parse("#mw-content-text").unwrap();
        let content = document.select(&selector).next().unwrap();
        // recursive way to clean the text
        let text = clean_text(&content);
        file.write_all(text.as_bytes()).await?;

        // brute force way... not too accurate...
        // let text = content.text().map(str::trim)
        //     .filter(|s| !s.is_empty())
        //     .collect::<Vec<_>>()
        //     .join(" ");
        // file.write_all(format!("Content: {}\n", text).as_bytes()).await?;

        // works if no parsing required...
        // let mut stream = response.bytes_stream();
        //
        // while let Some(chunk) = stream.next().await {
        //     let chunk = chunk?;
        //     file.write_all(&chunk).await?;
        // }

        file.flush().await?;

        Ok(())
    }

    #[test]
    fn test_parse_images() {
        let content = fs::read_to_string("testdata/wikipedia_nosql_local.html.txt").unwrap();
        let images = parse_images(content);

        assert!(!images.is_empty());
        for image in images {
            println!(
                "* Image src: {}\r\n - inner_html: {}",
                image.src, image.inner_html
            );
        }
    }

    #[tokio::test]
    async fn test_parse_plain_text_and_images() -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string("testdata/wikipedia_nosql_local.html.txt").unwrap();
        let plain_text_and_images = parse_plain_text_and_images(content);

        assert!(!plain_text_and_images.text.is_empty());
        assert!(!plain_text_and_images.images.is_empty());

        // write it out to a file for debugging... etc
        let mut file = File::create("testdata/wikipedia_nosql_local.html.result.txt").await?;

        file.write_all(format!("plain-text content:\r\n").as_bytes())
            .await?;
        file.write_all(plain_text_and_images.text.as_bytes())
            .await?;

        file.write_all("\r\n\r\nImages:\r\n".as_bytes()).await?;
        for image in plain_text_and_images.images {
            file.write_all(
                format!(
                    "src: {}\r\n - html: {}\r\n\r\n",
                    image.src, image.inner_html
                )
                .as_bytes(),
            )
            .await?;
        }
        file.flush().await?;

        Ok(())
    }
}
