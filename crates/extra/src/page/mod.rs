use crate::{
  view::{Json, View},
  Layout, Mountable,
};

// The havy lifting will do the macros, they should generate all the needed boilerplate,
// Page and Partial differs one of each other only by the Layout, a Page is also a Partial
// but not vice versa.

pub trait Page<'a, L>: Json<'a> + Mountable + View
where
  L: Layout,
{
  fn slots(&self) -> L::Slot;
}

pub trait Partial<'a>: Json<'a> + Mountable + View {}
