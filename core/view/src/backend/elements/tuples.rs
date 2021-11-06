use std::fmt::{Result, Write};

use crate::backend::render::Render;

// TODO: Write a macro to implement the Render trait for all tuples.
// macro_rules! expr {
//   ($x:expr) => {
//     $x
//   };
// } // HACK
// macro_rules! tuple_index {
//   ($tuple:expr, $idx:tt) => {
//     expr!($tuple.$idx)
//   };
// }

// macro_rules! impl_tuple_render {
//   (@step $idx:expr, $tuple:expr,) => {};

//   (@step $idx:expr, $head:ident, $($tail:ident,)*) => {
//     tuple_index!($tuple, $idx);
//     impl_tuple_render!(@step $idx + 1i32, $tuple, $($tail,)*);
//   };

//   ($($t:ident),*) => {
//     impl<$($t),*> Render for ($($t),*)
//     where
//       $($t: Render),*
//     {
//       fn render_into<WW: Write>(self, writer: &mut WW) -> Result {
//         vec![impl_tuple_render!(@step 0i32, self, $($t,)*)];
//         Ok(())
//       }
//     }
//   };
// }

// impl_tuple_render!(A, B);
// impl_tuple_render!(A, B, C);
// impl_tuple_render!(A, B, C, D);
// impl_tuple_render!(A, B, C, D, E);
// impl_tuple_render!(A, B, C, D, E, F);
// impl_tuple_render!(A, B, C, D, E, F, G);
// impl_tuple_render!(A, B, C, D, E, F, G, H);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K, L);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K, L, M);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
// impl_tuple_render!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

impl<A: Render, B: Render> Render for (A, B) {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    self.0.render_into(writer)?;
    self.1.render_into(writer)
  }
}

impl<A: Render, B: Render, C: Render> Render for (A, B, C) {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    self.0.render_into(writer)?;
    self.1.render_into(writer)?;
    self.2.render_into(writer)
  }
}

impl<A: Render, B: Render, C: Render, D: Render> Render for (A, B, C, D) {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    self.0.render_into(writer)?;
    self.1.render_into(writer)?;
    self.2.render_into(writer)?;
    self.3.render_into(writer)
  }
}

impl<A: Render, B: Render, C: Render, D: Render, E: Render> Render for (A, B, C, D, E) {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    self.0.render_into(writer)?;
    self.1.render_into(writer)?;
    self.2.render_into(writer)?;
    self.3.render_into(writer)?;
    self.4.render_into(writer)
  }
}

impl<A: Render, B: Render, C: Render, D: Render, E: Render, F: Render> Render
  for (A, B, C, D, E, F)
{
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    self.0.render_into(writer)?;
    self.1.render_into(writer)?;
    self.2.render_into(writer)?;
    self.3.render_into(writer)?;
    self.4.render_into(writer)?;
    self.5.render_into(writer)
  }
}

impl<A: Render, B: Render, C: Render, D: Render, E: Render, F: Render, G: Render> Render
  for (A, B, C, D, E, F, G)
{
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    self.0.render_into(writer)?;
    self.1.render_into(writer)?;
    self.2.render_into(writer)?;
    self.3.render_into(writer)?;
    self.4.render_into(writer)?;
    self.5.render_into(writer)?;
    self.6.render_into(writer)
  }
}

impl<A: Render, B: Render, C: Render, D: Render, E: Render, F: Render, G: Render, H: Render> Render
  for (A, B, C, D, E, F, G, H)
{
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    self.0.render_into(writer)?;
    self.1.render_into(writer)?;
    self.2.render_into(writer)?;
    self.3.render_into(writer)?;
    self.4.render_into(writer)?;
    self.5.render_into(writer)?;
    self.6.render_into(writer)?;
    self.7.render_into(writer)
  }
}
