pub mod css;
pub mod dom;
pub mod html;
pub mod source;

fn main() {
    println!("Hello, world!");
    let node = dom::text(String::from("hello"));
    println!("{:?}", node);

    let html = "<html><p>hello</p></html>";
    let root = html::parse(html.to_string());
    println!("{:?}", root);

    let css_source = "";
    css::parse(css_source.to_string());
}
