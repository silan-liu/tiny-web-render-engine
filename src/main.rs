extern crate image;

use std::default::Default;
use std::fs::File;
use std::io::{BufWriter, Read};

pub mod css;
pub mod dom;
pub mod html;
pub mod layout;
pub mod painting;
pub mod source;
pub mod style;

fn main() {
    println!("Hello, world!");
    let node = dom::text(String::from("hello"));
    println!("{:?}", node);

    let html = read_source("example/test.html".to_string());
    let root = html::parse(html.to_string());
    println!("{:?}", root);

    let css = read_source("example/test.css".to_string());
    let stylesheet = css::parse(css.to_string());
    println!("{:?}", stylesheet);

    let style_tree = style::style_tree(&root, &stylesheet);
    println!("{:?}", style_tree);

    let layout_tree = layout::layout_tree(&style_tree, Default::default());
    println!("{:?}", layout_tree);

    let filename = "output.png";
    let mut file = BufWriter::new(File::create(&filename).unwrap());

    // 定义默认视口，800*600
    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;

    // 生成像素点
    let canvas = painting::paint(&layout_tree, viewport.content);
    let (w, h) = (canvas.width as u32, canvas.height as u32);

    // 生成图片
    let img = image::ImageBuffer::from_fn(w, h, move |x, y| {
        let index = (y * w + x) as usize;
        let color = canvas.pixels[index];
        image::Pixel::from_channels(color.r, color.g, color.b, color.a)
    });

    let ok = image::ImageRgba8(img).save(&mut file, image::PNG).is_ok();
    if ok {
        println!("Saved output as {}", filename);
    } else {
        println!("Error save output as {}", filename);
    }
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut str)
        .unwrap();
    str
}
