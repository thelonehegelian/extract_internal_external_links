use char;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashSet;
use std::error::Error;
use tokio;

fn extract_internal_external_links(page_content: &str) -> HashSet<Cow<str>> {
    // only extracts external links
    lazy_static! {
        static ref WIKI_REGEX: Regex = Regex::new(r"\[((?!http)[-a-zA-Z0-9@:%._+~#=]+\.[a-z]{2,6}[-a-zA-Z0-9@:%_+.~#?&//=*])\]|\[((?!www)[-a-zA-Z0-9@:%._+~#=]+\.[a-z]{2,6}[-a-zA-Z0-9@:%_+.~#?&//=*])\]").unwrap();
    // static ref WIKI_REGEX: Regex = Regex::new(&format!("[http{}s]?://(www.)?[-a-zA-Z0-9@:%._+~#={}]{{2,256}}.[a-z]{{2,6}}\\b([-a-zA-Z0-9@:%_+.~#?&//=*)])",
    // char::escape_default('/'), char::escape_default(']'))).unwrap();
    }
    // iterate WIKI_REGEX and collect valid links into a HashSet
    let itr = WIKI_REGEX
        .captures_iter(page_content)
        .map(|cap| cap.get(0).unwrap());

    let mut links = HashSet::new();
    for link in itr {
        // add each link to the HashSet
        links.insert(link.as_str().into());
        println!("{}", link.as_str());
    }

    links
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://en.wikipedia.org/wiki/Shannon_Lucid";
    // make a request to url and get the page content
    let page_content = reqwest::get(url).await?.text().await?;
    let links = extract_internal_external_links(page_content.as_str());
    Ok(())
}
