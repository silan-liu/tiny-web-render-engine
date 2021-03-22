use crate::css::Unit::Px;
use crate::css::Value::{Keyword, Length};
use crate::style::{Display, StyleNode};
use std::default::Default;

pub use self::BoxType::{AnonymousBlock, BlockNode, InlineNode};

#[derive(Debug, Clone, Default, Copy)]
pub struct Rect {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
}

#[derive(Debug, Clone, Default, Copy)]
pub struct Dimensions {
  pub content: Rect,
  pub padding: EdgeSizes,
  pub margin: EdgeSizes,
  pub border: EdgeSizes,
}

#[derive(Debug, Clone, Default, Copy)]
pub struct EdgeSizes {
  pub left: f32,
  pub right: f32,
  pub top: f32,
  pub bottom: f32,
}

#[derive(Debug)]
pub struct LayoutBox<'a> {
  pub dimensions: Dimensions,
  pub box_type: BoxType<'a>,
  pub children: Vec<LayoutBox<'a>>,
}

#[derive(Debug)]
pub enum BoxType<'a> {
  BlockNode(&'a StyleNode<'a>),
  InlineNode(&'a StyleNode<'a>),
  AnonymousBlock,
}

impl<'a> LayoutBox<'a> {
  fn new(box_type: BoxType) -> LayoutBox {
    LayoutBox {
      box_type: box_type,
      dimensions: Default::default(),
      children: Vec::new(),
    }
  }

  fn get_style_node(&self) -> &'a StyleNode<'a> {
    match self.box_type {
      BlockNode(node) | InlineNode(node) => node,
      AnonymousBlock => panic!("AnonymousBlock block box has no style node!"),
    }
  }
}

pub fn layout_tree<'a>(node: &'a StyleNode<'a>, mut containing_block: Dimensions) -> LayoutBox<'a> {
  containing_block.content.height = 0.0;
  let mut root_box = build_layout_tree(node);
  root_box.layout(containing_block);
  root_box
}

fn build_layout_tree<'a>(style_node: &'a StyleNode<'a>) -> LayoutBox<'a> {
  let mut root = LayoutBox::new(match style_node.display() {
    Display::Block => BlockNode(style_node),
    Display::Inline => InlineNode(style_node),
    Display::None => panic!("Root node has display: none"),
  });

  for child in &style_node.children {
    match child.display() {
      Display::Block => root.children.push(build_layout_tree(child)),
      Display::Inline => root
        .get_inline_container()
        .children
        .push(build_layout_tree(child)),
      Display::None => {}
    }
  }

  root
}

impl<'a> LayoutBox<'a> {
  fn layout(&mut self, containing_block: Dimensions) {
    match self.box_type {
      BlockNode(_) => self.layout_block(containing_block),
      InlineNode(_) | AnonymousBlock => {}
    }
  }

  fn layout_block(&mut self, containing_block: Dimensions) {
    // 根据 containing_block 计算宽度
    self.calculate_block_width(containing_block);

    // 计算位置
    self.calculate_block_position(containing_block);

    // 对子节点布局
    self.layout_block_children();

    // 计算整体高度
    self.calculate_block_height();
  }

  // 根据父容器宽度计算节点 x 方向的布局数据
  fn calculate_block_width(&mut self, containing_block: Dimensions) {
    let style = self.get_style_node();

    let auto = Keyword("auto".to_string());
    let mut width = style.value("width").unwrap_or(auto.clone());

    let zero = Length(0.0, Px);

    let mut margin_left = style.lookup("margin-left", "margin", &zero);
    let mut margin_right = style.lookup("margin-right", "margin", &zero);

    let border_left = style.lookup("border-left-width", "border-width", &zero);
    let border_right = style.lookup("border-right-width", "border-width", &zero);

    let padding_left = style.lookup("padding-left", "padding", &zero);
    let padding_right = style.lookup("padding-right", "padding", &zero);

    let total = sum(
      [
        &margin_left,
        &margin_right,
        &border_left,
        &border_right,
        &width,
        &padding_left,
        &padding_right,
      ]
      .iter()
      .map(|v| v.to_px()),
    );

    // 大于父容器宽度，修改 margin-left，margin-right
    if width != auto && total > containing_block.content.width {
      if margin_left == auto {
        margin_left = Length(0.0, Px);
      }

      if margin_right == auto {
        margin_right = Length(0.0, Px);
      }
    }

    let underflow = containing_block.content.width - total;
    match (width == auto, margin_left == auto, margin_right == auto) {
      // 全部都不为 auto
      (false, false, false) => {
        // 修改 margin-right
        margin_right = Length(margin_right.to_px() + underflow, Px)
      }
      // margin-right 为 auto，设置为剩余空间
      (false, false, true) => margin_right = Length(underflow, Px),

      // margin-left 为 auto，设置为剩余空间
      (false, true, false) => margin_left = Length(underflow, Px),

      // width 为 auto，自适应 width
      (true, _, _) => {
        if margin_left == auto {
          margin_left = Length(0.0, Px);
        }

        if margin_right == auto {
          margin_right = Length(0.0, Px);
        }

        if underflow >= 0.0 {
          // 设置宽度为剩余空间
          width = Length(underflow, Px)
        } else {
          // 超出宽度，减小 margin-right
          width = Length(0.0, Px);
          margin_right = Length(margin_right.to_px() + underflow, Px);
        }
      }

      // margin-left，margin-right 平分剩余空间
      (false, true, true) => {
        margin_left = Length(underflow / 2.0, Px);
        margin_right = Length(underflow / 2.0, Px);
      }
    }

    // 设置盒子模型部分数据
    let d = &mut self.dimensions;
    d.content.width = width.to_px();

    d.padding.left = padding_left.to_px();
    d.padding.right = padding_right.to_px();

    d.border.left = border_left.to_px();
    d.border.right = border_right.to_px();

    d.margin.left = margin_left.to_px();
    d.margin.right = margin_right.to_px();
  }

  fn calculate_block_position(&mut self, containing_block: Dimensions) {
    // 计算 x，y，竖直方向间距
    let style = self.get_style_node();
    let d = &mut self.dimensions;

    let zero = Length(0.0, Px);

    d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
    d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

    d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
    d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

    d.border.top = style
      .lookup("border-top-width", "border-width", &zero)
      .to_px();
    d.border.bottom = style
      .lookup("border-bottom-width", "border-width", &zero)
      .to_px();

    d.content.x = containing_block.content.x + d.margin.left + d.border.left + d.padding.left;
    d.content.y = containing_block.content.y
      + containing_block.content.height
      + d.margin.top
      + d.border.top
      + d.padding.top;
  }

  fn layout_block_children(&mut self) {
    let d = &mut self.dimensions;
    for child in &mut self.children {
      child.layout(*d);

      d.content.height += child.dimensions.margin_box().height;
    }
  }

  // 如果设置了 height，则取该值
  fn calculate_block_height(&mut self) {
    if let Some(Length(h, Px)) = self.get_style_node().value("height") {
      self.dimensions.content.height = h
    }
  }

  // if a block node has inline child，simply create a anonymous block wrapping the inline node
  fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
    match self.box_type {
      InlineNode(_) | AnonymousBlock => self,
      BlockNode(_) => {
        match self.children.last() {
          Some(&LayoutBox {
            box_type: AnonymousBlock,
            ..
          }) => {}
          _ => self.children.push(LayoutBox::new(AnonymousBlock)),
        }

        self.children.last_mut().unwrap()
      }
    }
  }
}

fn sum<I>(iter: I) -> f32
where
  I: Iterator<Item = f32>,
{
  iter.fold(0.0, |a, b| a + b)
}

impl Rect {
  pub fn expanded_by(self, edge: EdgeSizes) -> Rect {
    Rect {
      x: self.x - edge.left,
      y: self.y - edge.top,
      width: self.width + edge.left + edge.right,
      height: self.height + edge.top + edge.bottom,
    }
  }
}

impl Dimensions {
  // content + paddding
  pub fn padding_box(self) -> Rect {
    self.content.expanded_by(self.padding)
  }

  // content + border + padding
  pub fn border_box(self) -> Rect {
    self.padding_box().expanded_by(self.border)
  }

  pub fn margin_box(self) -> Rect {
    self.border_box().expanded_by(self.margin)
  }
}
