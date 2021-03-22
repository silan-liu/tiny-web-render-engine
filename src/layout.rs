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
  let mut root_box = build_layout_tree(node);
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
      Display::Inline => root.children.push(build_layout_tree(child)),
      Display::None => {}
    }
  }

  root
}
