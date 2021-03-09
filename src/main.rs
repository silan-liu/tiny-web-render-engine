pub mod dom;
pub mod html;

fn main() {
    println!("Hello, world!");
    let node = dom::text(String::from("hello"));
    println!("{:?}", node);

    let html = "<html></html>";
    let root = html::parse(html.to_string());
    println!("{}", root);
}
