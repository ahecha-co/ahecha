pub use ahecha_macro as macros;

pub use sycamore;
pub use sycamore::builder::prelude::*;
pub use sycamore::render_to_string;

pub trait IntoView {
  fn into_view<G>(self) -> sycamore::view::View<G>
  where
    G: sycamore::generic_node::GenericNode;
}

macro_rules! impl_into_view {
  ($($t:ty),*) => {
    $(
      impl IntoView for $t {
        fn into_view<G>(self) -> sycamore::view::View<G> where G: sycamore::generic_node::GenericNode {
          t(format!("{self}"))
        }
      }

      impl IntoView for & $t {
        fn into_view<G>(self) -> sycamore::view::View<G> where G: sycamore::generic_node::GenericNode {
          t(format!("{self}"))
        }
      }

      impl IntoView for Option<$t> {
        fn into_view<G>(self) -> sycamore::view::View<G> where G: sycamore::generic_node::GenericNode {
          t(match self {
            Some(val) => format!("{val}"),
            None => "".to_owned(),
          })
        }
      }

      impl IntoView for Option<& $t> {
        fn into_view<G>(self) -> sycamore::view::View<G> where G: sycamore::generic_node::GenericNode {
          t(match self.as_ref() {
            Some(val) => format!("{val}"),
            None => "".to_owned(),
          })
        }
      }
    )*
  };
}

impl_into_view!(
  String, &str, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);
