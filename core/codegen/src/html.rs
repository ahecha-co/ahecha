use std::str;

use nom::{
  branch::alt,
  bytes::complete::{tag, take_till, take_while},
  character::is_alphanumeric,
  combinator::opt,
  error::context,
  multi::{many0, many_till},
  sequence::{delimited, preceded},
  AsChar, IResult,
};

use node::*;

use crate::html::children::Children;

use self::attributes::{Attribute, Attributes};

mod attributes;
mod children;
mod node;

fn parse_tag_name(input: &[u8]) -> IResult<&[u8], String> {
  let (input, name) = context(
    "parse_tag_name",
    take_while(|c: u8| c.is_alphanum() || c == b'-' || c == b'_'),
  )(input)?;
  Ok((input, str::from_utf8(name).unwrap().to_string()))
}

fn parse_attribute(input: &[u8]) -> IResult<&[u8], Attribute> {
  let (input, _) = take_while(|c| c == b' ')(input)?;
  let (input, key) = take_while(is_alphanumeric)(input)?;
  let (input, value) = delimited(tag("=\""), take_till(|c| c == b'"'), tag("\""))(input)?;
  Ok((
    input,
    (
      str::from_utf8(key).unwrap().to_string(),
      str::from_utf8(value).unwrap().to_string(),
    ),
  ))
}

fn parse_attributes(input: &[u8]) -> IResult<&[u8], Attributes> {
  let (input, attributes) = context(
    "parse_attributes",
    many0(preceded(many0(tag(" ")), parse_attribute)),
  )(input)?;
  Ok((input, attributes.into()))
}

fn parse_tag_name_attributes0(input: &[u8]) -> IResult<&[u8], HtmlElement> {
  let (input, name) = parse_tag_name(input)?;
  let (input, attributes) = opt(many0(parse_attribute))(input)?;
  Ok((
    input,
    HtmlElement {
      attributes: attributes.into(),
      children: Default::default(),
      name,
    },
  ))
}

fn parse_self_closing_tag(input: &[u8]) -> IResult<&[u8], HtmlNode> {
  let (input, html_tag) = delimited(tag("<"), parse_tag_name_attributes0, tag("/>"))(input)?;
  Ok((input, html_tag.into()))
}

fn parse_tag_with_children0(input: &[u8]) -> IResult<&[u8], HtmlNode> {
  let (input, mut html_tag) = delimited(
    tag("<"),
    nom::error::dbg_dmp(parse_tag_name_attributes0, "parse_tag_with_children0"),
    tag(">"),
  )(input)?;
  let closing_tag = format!("</{}>", &html_tag.name);
  let (input, (children, _)) = many_till(parse, tag(closing_tag.as_str()))(input)?;

  html_tag.children = Children { nodes: children };

  Ok((input, html_tag.into()))
}

fn parse_text(input: &[u8]) -> IResult<&[u8], HtmlNode> {
  let (input, text) = take_while(|c| c != b'<' && c != b'{')(input)?;
  Ok((
    input,
    HtmlNode::Text(HtmlText {
      text: str::from_utf8(text).unwrap().to_string(),
    }),
  ))
}

fn parse_block(input: &[u8]) -> IResult<&[u8], HtmlNode> {
  let (input, text) = delimited(tag("{"), take_while(|c| c != b'}'), tag("}"))(input)?;
  Ok((
    input,
    HtmlNode::Block(HtmlBlock {
      block: str::from_utf8(text).unwrap().to_string(),
    }),
  ))
}

fn parse_empty(input: &[u8]) -> IResult<&[u8], HtmlNode> {
  Ok((input, HtmlNode::None))
}

pub fn parse(input: &[u8]) -> IResult<&[u8], HtmlNode> {
  let res = context(
    "parse",
    alt((
      parse_self_closing_tag,
      nom::error::dbg_dmp(parse_tag_with_children0, "parse::parse_tag_with_children0"),
      parse_block,
      parse_text,
      parse_empty,
    )),
  )(input);
  res
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse_tag_name() {
    let input = b"div attr=\"value\"";
    let (remainder, name) = parse_tag_name(input).unwrap();
    assert_eq!(str::from_utf8(remainder).unwrap(), " attr=\"value\"");
    assert_eq!(&name, "div");
  }

  #[test]
  fn test_parse_attribute() {
    let input = b"attr=\"value\" attr2=\"value2\"";
    let (remainder, attribute) = parse_attribute(input).unwrap();
    assert_eq!(str::from_utf8(remainder).unwrap(), " attr2=\"value2\"");
    assert_eq!(attribute, ("attr".into(), "value".into()));
  }

  #[test]
  fn test_parse_attributes() {
    let input = b"attr=\"value\" attr2=\"value2\"";
    let (remainder, attributes) = parse_attributes(input).unwrap();
    assert_eq!(str::from_utf8(remainder).unwrap(), "");
    assert_eq!(
      attributes.attrs,
      vec![
        ("attr".into(), "value".into()),
        ("attr2".into(), "value2".into())
      ]
    );
  }

  #[test]
  fn test_self_closing_tag() {
    let input = b"<div/>";
    let (remainder, node) = parse_self_closing_tag(input).unwrap();
    assert_eq!(remainder, b"");
    match node {
      HtmlNode::Element(tag) => {
        assert_eq!(tag.name, "div");
        assert_eq!(tag.attributes.attrs, vec![]);
      }
      _ => panic!("Expected tag"),
    }
  }

  #[test]
  fn test_tag() {
    let input = b"<div></div>";
    let (remainder, node) = parse_tag_with_children0(input).unwrap();
    assert_eq!(remainder, b"");
    match node {
      HtmlNode::Element(tag) => {
        assert_eq!(tag.name, "div");
        assert_eq!(tag.attributes.attrs, vec![]);
      }
      _ => panic!("Expected tag"),
    }
  }

  #[test]
  fn test_tag_with_children() {
    let input = b"<div><h1>Hello</h1></div>";
    let (remainder, node) = parse_tag_with_children0(input).unwrap();
    assert_eq!(str::from_utf8(remainder).unwrap(), "");
    match node {
      HtmlNode::Element(tag) => {
        assert_eq!(tag.name, "div");
        assert_eq!(tag.children.nodes.len(), 1);
      }
      _ => panic!("Expected tag"),
    }
  }

  #[test]
  fn test_parse() {
    let input = b"<div><h1>Hello</h1><p>World</p></div>";
    let (remainder, node) = parse(input).unwrap();
    assert_eq!(str::from_utf8(remainder).unwrap(), "");
    match node {
      HtmlNode::Element(tag) => {
        assert_eq!(tag.name, "div");
        assert_eq!(tag.children.nodes.len(), 2);
      }
      _ => panic!("Expected tag"),
    }
  }

  #[test]
  fn test_tag_deep_nested() {
    let input = b"<div><h1><h2><h3><h4>Hello</h4></h3></h2></h1><p>World</p></div>";
    let (remainder, node) = parse_tag_with_children0(input).unwrap();
    assert_eq!(str::from_utf8(remainder).unwrap(), "");
    match node {
      HtmlNode::Element(tag) => {
        assert_eq!(tag.name, "div");
        assert_eq!(tag.children.nodes.len(), 2);
      }
      _ => panic!("Expected tag"),
    }
  }

  #[test]
  fn test_parse_text() {
    let input = b"Hello World</div>";
    let (remainder, node) = parse_text(input).unwrap();
    assert_eq!(remainder, b"</div>");
    match node {
      HtmlNode::Text(text) => assert_eq!(text.text, "Hello World"),
      _ => panic!("Expected text"),
    }
  }

  #[test]
  fn test_parse_block() {
    let input = b"{ a block } Hello World</div>";
    let (remainder, node) = parse_block(input).unwrap();
    assert_eq!(remainder, b" Hello World</div>");
    match node {
      HtmlNode::Block(block) => assert_eq!(block.block, " a block "),
      _ => panic!("Expected block"),
    }
  }

  #[test]
  fn test_parse_custom_element() {
    let input = b"<custom-element attr1=\"v1\" attr2=\"v2\">Content</custom-element>";
    let (remainder, node) = parse(input).unwrap();
    assert_eq!(str::from_utf8(remainder).unwrap(), "");
    match node {
      HtmlNode::Element(tag) => {
        assert_eq!(tag.name, "custom-element")
      }
      _ => panic!("Expected block"),
    }
  }

  #[test]
  fn test_parse_component() {
    let input = b"<Component attr1=\"v1\" attr2=\"v2\">Content</Component>";
    let (remainder, node) = parse(input).unwrap();
    assert_eq!(str::from_utf8(remainder).unwrap(), "");
    match node {
      HtmlNode::Component(tag) => {
        assert_eq!(tag.name, "Component")
      }
      _ => panic!("Expected block"),
    }
  }
}
