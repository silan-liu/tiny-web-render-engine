pub mod css;
pub mod dom;
pub mod html;
pub mod layout;
pub mod source;
pub mod style;

fn main() {
    println!("Hello, world!");
    let node = dom::text(String::from("hello"));
    println!("{:?}", node);

    let html = "<html><div classes=\"note\" id=\"test\"><p>hello</p></div></html>";
    let root = html::parse(html.to_string());
    println!("{:?}", root);

    let css_source =
        "#test {display:none;} p, div.note, #hello {background-color:#332234;margin-top:10.2px;postion:absolute;}";
    let stylesheet = css::parse(css_source.to_string());
    println!("{:?}", stylesheet);

    let style_tree = style::style_tree(&root, &stylesheet);
    println!("{:?}", style_tree);

    let layout_tree = layout::layout_tree(&style_tree, Default::default());
    println!("{:?}", layout_tree);
}
