macro_rules! impl_renderable {
  ($($t:ty),*) => {
    $(
      impl<const DINENSION: usize> crate::backend::attributes::RenderAttributes for [(&str, $t); DINENSION] {
        fn render_attributes_into<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
          self.iter().try_for_each(|(key, value)| {
            write!(writer, " {}=\"{}\"", key, value)
          })
        }
      }
    )*
  };
}

impl_renderable!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
