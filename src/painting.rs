use crate::css::{Color, Value};
use crate::layout::{AnonymousBlock, BlockNode, InlineNode, LayoutBox, Rect};

pub struct Canvas {
  pub pixels: Vec<Color>,
  pub width: usize,
  pub height: usize,
}

#[derive(Debug)]
pub enum DisplayCommand {
  SolidColor(Color, Rect),
}

// 绘制命令列表
pub type DisplayList = Vec<DisplayCommand>;

pub fn paint(layout_root: &LayoutBox, bounds: Rect) -> Canvas {
  let display_list = build_display_list(layout_root);
  let mut canvas = Canvas::new(bounds.width as usize, bounds.height as usize);
  for item in display_list {
    canvas.paint_item(&item)
  }

  canvas
}

pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
  let mut list = Vec::new();
  render_layout_box(&mut list, layout_root);
  list
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
  render_background(list, layout_box);
  render_borders(list, layout_box);

  for child in &layout_box.children {
    render_layout_box(list, child);
  }
}

fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
  // bg:border+padding+content
  println!("render_background");

  get_color(layout_box, "background").map(|color| {
    println!("render_background, {:?}", color);
    list.push(DisplayCommand::SolidColor(
      color,
      layout_box.dimensions.border_box(),
    ))
  });
}

fn render_borders(list: &mut DisplayList, layout_box: &LayoutBox) {
  let color = match get_color(layout_box, "border-color") {
    Some(color) => color,
    _ => return,
  };

  let d = layout_box.dimensions;
  let border_box = d.border_box();

  // left rect
  let left_rect = Rect {
    x: border_box.x,
    y: border_box.y,
    width: d.border.left,
    height: border_box.height,
  };

  // left border
  list.push(DisplayCommand::SolidColor(color, left_rect));

  // right rect
  let right_rect = Rect {
    x: border_box.x + border_box.width - d.border.right,
    y: border_box.y,
    width: d.border.right,
    height: border_box.height,
  };

  // right border
  list.push(DisplayCommand::SolidColor(color, right_rect));

  // top rect
  let top_rect = Rect {
    x: border_box.x,
    y: border_box.y,
    width: border_box.width,
    height: d.border.top,
  };

  // top border
  list.push(DisplayCommand::SolidColor(color, top_rect));

  // bottom rect
  let bottom_rect = Rect {
    x: border_box.x,
    y: border_box.y + border_box.height - d.border.bottom,
    width: border_box.width,
    height: d.border.bottom,
  };

  // bottom border
  list.push(DisplayCommand::SolidColor(color, bottom_rect));
}

fn get_color(layout_box: &LayoutBox, name: &str) -> Option<Color> {
  match layout_box.box_type {
    BlockNode(style) | InlineNode(style) => match style.value(name) {
      Some(Value::ColorValue(color)) => Some(color),
      _ => None,
    },
    AnonymousBlock => None,
  }
}

impl Canvas {
  fn new(width: usize, height: usize) -> Canvas {
    let white = Color {
      r: 255,
      g: 255,
      b: 255,
      a: 255,
    };
    Canvas {
      pixels: vec![white; width * height],
      width: width,
      height: height,
    }
  }

  fn paint_item(&mut self, item: &DisplayCommand) {
    match *item {
      DisplayCommand::SolidColor(color, rect) => {
        let x0 = rect.x.clamp(0.0, self.width as f32) as usize;
        let x1 = (rect.x + rect.width).clamp(0.0, self.width as f32) as usize;

        let y0 = rect.y.clamp(0.0, self.height as f32) as usize;
        let y1 = (rect.y + rect.height).clamp(0.0, self.height as f32) as usize;

        println!("paint_item, {:?}, {:?}", color, rect);
        for y in y0..y1 {
          for x in x0..x1 {
            self.pixels[y * self.width + x] = color;
          }
        }
      }
    }
  }
}

trait Clamp {
  fn clamp(self, lower: Self, upper: Self) -> Self;
}

impl Clamp for f32 {
  fn clamp(self, lower: f32, upper: f32) -> f32 {
    self.max(lower).min(upper)
  }
}
