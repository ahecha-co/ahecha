use std::fmt::{Result, Write};

mod attributes;
mod doctype;
mod elements;
mod node;
mod numbers;
mod text;
// mod tuples;

pub trait RenderString: Sized {
  /// Render the component to a writer.
  /// Make sure you escape html correctly using the `render::html_escaping` module
  fn render_into<W: Write>(self, writer: &mut W) -> Result;

  /// Render the component to string
  fn render(self) -> String {
    let mut buf = String::new();
    self.render_into(&mut buf).unwrap();
    buf
  }
}

// Renders `()` or nothing
impl RenderString for () {
  fn render_into<W: Write>(self, _writer: &mut W) -> Result {
    Ok(())
  }
}

/// Renders `T` or nothing
impl<T: RenderString> RenderString for Option<T> {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    match self {
      None => Ok(()),
      Some(x) => x.render_into(writer),
    }
  }
}

/// Renders a list of `T`
impl<T: RenderString> RenderString for Vec<T> {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    for elem in self {
      elem.render_into(writer)?;
    }
    Ok(())
  }
}

/// Renders `O` or `E`
impl<O: RenderString, E: RenderString> RenderString for std::result::Result<O, E> {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    match self {
      Ok(o) => o.render_into(writer),
      Err(e) => e.render_into(writer),
    }
  }
}

/// Renders `bool`
impl RenderString for bool {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    if self {
      write!(writer, "true")?;
    } else {
      write!(writer, "false")?;
    }
    Ok(())
  }
}

// TODO: Play with the idea of removing the tuple_list and instead generate the renderer for each case?
/*
macro_rules! impl_render {(
    $( $n:ident $(, $k:ident)* $(,)? )?
) => (
    impl<$($n : Render, $($k : Render),*)?> Render for ( $($n, $($k),*)? ) {
        fn render (self: Self)
        {
            let ( $($n, $($k),*)? ) = self;
         $( $n.render();
         $( $k.render(); )*)?
        }
    }

    $(
        impl_render! { $($k),* }
    )?
)}
impl_render!(_6, _5, _4, _3, _2, _1);

trait Render {
  fn render(self: Self);
}
*/
