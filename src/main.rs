use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashSet;
use std::error::Error;
use tokio;

fn extract_internal_external_links(page_content: &str) -> HashSet<Cow<str>> {
    lazy_static! {
        static ref WIKI_REGEX: Regex = Regex::new(
            r"(?x)
        \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\]    # internal links
        |
        (url=|URL\||\[)(?P<external>http.*?)[ \|}] # external links
    "
        )
        .unwrap();
    }
    // iterate WIKI_REGEX and collect valid links into a HashSet
    let links: HashSet<Cow<str>> = WIKI_REGEX
        .captures_iter(page_content)
        .map(|c| match (c.name("internal"), c.name("external")) {
            (Some(val), None) => Cow::from(val.as_str().to_lowercase()),
            (None, Some(val)) => Cow::from(val.as_str()),
            _ => unreachable!(),
        })
        .collect();
    links
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw";
    // make a request to url and get the page content
    let page_content = reqwest::blocking::get(url)
        .and_then(|mut res| res.text())
        .unwrap();
    let links = extract_internal_external_links(page_content.as_str());
    println!("{:?}", links);
    Ok(())
}
