pub mod css;
pub mod dom;
pub mod html;
pub mod source;
pub mod style;

fn main() {
    println!("Hello, world!");
    let node = dom::text(String::from("hello"));
    println!("{:?}", node);

    let html = "<html><p>hello</p></html>";
    let root = html::parse(html.to_string());
    println!("{:?}", root);

    let css_source = "div.note {background-color:#332234;margin-top:10.2px;postion:absolute;}";
    let stylesheet = css::parse(css_source.to_string());
    println!("{:?}", stylesheet);

    style::style_tree(&root, &stylesheet);
}
