use std::env;

use xcraper::{dom::DomBuilder, xpath::XPathBuilder};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let xpath = XPathBuilder::parse("//p:nth=-1").unwrap();
    let html = r#"
        <html>
        <p size="10"> </p>
        <p href="sad">
        </p>
        <p href="last">
        </p>
        </html>
    "#;
    let doc = DomBuilder::parse(&html).unwrap();
    let mut filter = doc.query(&xpath);
    let node = filter.next();
    // println!("{:#?}", doc);
    println!("{:#?}", node);
    // println!("{:#?}", xpath);
}
