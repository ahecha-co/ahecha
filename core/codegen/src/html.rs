use std::str;

use nom::{
  branch::alt,
  bytes::complete::{tag, take_till, take_while},
  combinator::opt,
  error::{context, ContextError, ParseError},
  multi::{many0, many_till},
  sequence::{delimited, preceded},
  AsChar, IResult,
};

use node::*;

use crate::html::children::Children;

use self::attributes::Attribute;

mod attributes;
mod children;
mod node;

/// parser combinators are constructed from the bottom up:
/// first we write parsers for the smallest elements (here a space character),
/// then we'll combine them in larger parsers
fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
  let chars = " \t\r\n";

  // nom combinators like `take_while` return a function. That function is the
  // parser,to which we can pass the input
  take_while(move |c| chars.contains(c))(i)
}

fn parse_tag_name<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, String, E> {
  let (input, name) = context(
    "parse_tag_name",
    take_while(|c: char| c.is_alphanum() || c == '-' || c == '_'),
  )(input)?;
  Ok((input, name.to_string()))
}

fn parse_attribute<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, Attribute, E> {
  let (input, _) = take_while(|c| c == ' ')(input)?;
  let (input, key) = take_while(|c: char| c.is_alphanumeric())(input)?;
  let (input, value) = delimited(tag("=\""), take_till(|c| c == '"'), tag("\""))(input)?;
  Ok((input, (key.to_string(), value.to_string())))
}

fn parse_tag_name_attributes0<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, HtmlElement, E> {
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

fn parse_self_closing_tag<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, HtmlNode, E> {
  let (input, html_tag) = delimited(tag("<"), parse_tag_name_attributes0, tag("/>"))(input)?;
  Ok((input, html_tag.into()))
}

fn parse_tag_with_children0<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, HtmlNode, E> {
  let (input, mut html_tag) = delimited(tag("<"), parse_tag_name_attributes0, tag(">"))(input)?;
  let closing_tag = format!("</{}>", &html_tag.name);
  let (input, (children, _)) = many_till(parse_node, tag(closing_tag.as_str()))(input)?;

  html_tag.children = Children { nodes: children };

  Ok((input, html_tag.into()))
}

fn parse_text<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, HtmlNode, E> {
  let (input, text) = take_while(|c| c != '<' && c != '{')(input)?;
  Ok((
    input,
    HtmlNode::Text(HtmlText {
      text: text.to_string(),
    }),
  ))
}

fn parse_block<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, HtmlNode, E> {
  let (input, text) = delimited(tag("{"), take_while(|c| c != '}'), tag("}"))(input)?;
  Ok((
    input,
    HtmlNode::Block(HtmlBlock {
      block: text.to_string(),
    }),
  ))
}

fn parse_doctype<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, HtmlNode, E> {
  let (input, _) = alt((tag("<!DOCTYPE html>"), tag("<!doctype html>")))(input)?;
  Ok((input, HtmlNode::Doctype(HtmlDoctype::Html5)))
}

fn parse_node<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, HtmlNode, E> {
  let res = context(
    "parse",
    preceded(
      opt(sp),
      alt((
        parse_doctype,
        parse_self_closing_tag,
        parse_tag_with_children0,
        parse_block,
        parse_text,
        // parse_empty,
      )),
    ),
  )(input);

  res
}

pub fn parse<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, Vec<HtmlNode>, E> {
  let (input, nodes) = delimited(opt(sp), parse_node, opt(sp))(input)?;
  Ok((input, vec![nodes]))
}

#[cfg(test)]
mod test {
  use nom::error::ErrorKind;

  use super::*;

  #[test]
  fn test_parse_doctype() {
    let input = "<!doctype html>";
    match parse_doctype::<(&str, ErrorKind)>(input) {
      Ok((remainder, node)) => {
        assert_eq!(remainder, "");
        if let HtmlNode::Doctype(HtmlDoctype::Html5) = node {
          assert!(true);
        } else {
          assert!(false);
        }
      }
      Err(e) => panic!("{:?}", e),
    }
  }

  #[test]
  fn test_parse_tag_name() {
    let input = "div attr=\"value\"";
    let (remainder, name) = parse_tag_name::<(&str, ErrorKind)>(input).unwrap();
    assert_eq!(remainder, " attr=\"value\"");
    assert_eq!(&name, "div");
  }

  #[test]
  fn test_parse_attribute() {
    let input = "attr=\"value\" attr2=\"value2\"";
    let (remainder, attribute) = parse_attribute::<(&str, ErrorKind)>(input).unwrap();
    assert_eq!(remainder, " attr2=\"value2\"");
    assert_eq!(attribute, ("attr".into(), "value".into()));
  }

  #[test]
  fn test_self_closing_tag() {
    let input = "<div/>";
    let (remainder, node) = parse_self_closing_tag::<(&str, ErrorKind)>(input).unwrap();
    assert_eq!(remainder, "");
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
    let input = "<div></div>";
    let (remainder, node) = parse_tag_with_children0::<(&str, ErrorKind)>(input).unwrap();
    assert_eq!(remainder, "");
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
    let input = "<div><h1>Hello</h1></div>";
    let (remainder, node) = parse_tag_with_children0::<(&str, ErrorKind)>(input).unwrap();
    assert_eq!(remainder, "");
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
    let input = "<div><h1>Hello</h1><p>World</p></div>";
    let (remainder, node) = parse_node::<(&str, ErrorKind)>(input).unwrap();
    assert_eq!(remainder, "");
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
    let input = "<div><h1><h2><h3><h4>Hello</h4></h3></h2></h1><p>World</p></div>";
    let (remainder, node) = parse_tag_with_children0::<(&str, ErrorKind)>(input).unwrap();
    assert_eq!(remainder, "");
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
    let input = "Hello World</div>";
    let (remainder, node) = parse_text::<(&str, ErrorKind)>(input).unwrap();
    assert_eq!(remainder, "</div>");
    match node {
      HtmlNode::Text(text) => assert_eq!(text.text, "Hello World"),
      _ => panic!("Expected text"),
    }
  }

  #[test]
  fn test_parse_block() {
    let input = "{ a block } Hello World</div>";
    let (remainder, node) = parse_block::<(&str, ErrorKind)>(input).unwrap();
    assert_eq!(remainder, " Hello World</div>");
    match node {
      HtmlNode::Block(block) => assert_eq!(block.block, " a block "),
      _ => panic!("Expected block"),
    }
  }

  #[test]
  fn test_parse_custom_element() {
    let input = "<custom-element attr1=\"v1\" attr2=\"v2\">Content</custom-element>";
    let (remainder, node) = parse_node::<(&str, ErrorKind)>(input).unwrap();
    assert_eq!(remainder, "");
    match node {
      HtmlNode::Element(tag) => {
        assert_eq!(tag.name, "custom-element")
      }
      _ => panic!("Expected block"),
    }
  }

  #[test]
  fn test_parse_component() {
    let input = "<Component attr1=\"v1\" attr2=\"v2\">Content</Component>";
    let (remainder, node) = parse_node::<(&str, ErrorKind)>(input).unwrap();
    assert_eq!(remainder, "");
    match node {
      HtmlNode::Component(tag) => {
        assert_eq!(tag.name, "Component")
      }
      _ => panic!("Expected block"),
    }
  }
}