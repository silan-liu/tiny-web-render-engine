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

    let css_source = "div.note {background-color:#332234;margin-top:10.2px;postion:absolute;}";
    let css_rules = css::parse(css_source.to_string());
    println!("{:?}", css_rules);
}
