use sqlx::postgres::PgArguments;

pub trait ToSql {
  fn to_sql(&self) -> String;
}

pub trait ToArray<T> {
  fn to_array(&self) -> Vec<T>;
}

pub trait ToArguments {
  fn to_arguments(&self) -> PgArguments;
}

impl<T> ToSql for T
where
  T: ToString,
{
  fn to_sql(&self) -> String {
    self.to_string()
  }
}

// impl<T> ToSql for Option<T>
// where
//   T: ToString,
// {
//   fn to_sql(&self) -> String {
//     match self {
//       Some(value) => value.to_string(),
//       None => String::new(),
//     }
//   }
// }

// macro_rules! impl_to_sql {
//   ($($t:ty),*) => {
//     $(
//       impl ToSql for $t {
//         fn to_sql(&self) -> String {
//           format!("{}", self)
//         }
//       }

//       impl ToSql for Option<$t> {
//         fn to_sql(&self) -> String {
//           match self {
//             Some(value) => format!("{}", value),
//             None => String::new(),
//           }
//         }
//       }
//     )*
//   };
// }

// impl_to_sql!(
//   u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, &str, String
// );

// macro_rules! impl_tuples {
//   () => {
//     impl ToSql for () {
//       fn to_sql(&self) -> String {
//         String::new()
//       }
//     }
//   };
//   ($($t:ident),*) => {
//     #[allow(non_snake_case)]
//     impl<$($t),*> ToSql for ($($t),*,) where $($t : ToSql),* {
//       fn to_sql(&self) -> String {
//         let ($($t),*) = self;
//         let res: Vec<String> = vec![
//           $($t .to_sql()),*
//         ];
//         res.join(",")
//       }
//     }

//     #[allow(non_snake_case)]
//     impl<$($t),*> ToArray<String> for ($($t),*) where $($t : ToSql),* {
//       fn to_array(&self) -> Vec<String> {
//         let ($($t),*) = self;
//         vec![
//           $($t .to_sql()),*
//         ]
//       }
//     }
//   };
// }

// impl_tuples!();
// impl_tuples!(T1);
// impl_tuples!(T1, T2);
// impl_tuples!(T1, T2, T3);
// impl_tuples!(T1, T2, T3, T4);
// impl_tuples!(T1, T2, T3, T4, T5);
// impl_tuples!(T1, T2, T3, T4, T5, T6);
// impl_tuples!(T1, T2, T3, T4, T5, T6, T7);
// impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8);
// impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
// impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
// impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
// impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
// impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
// impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
// impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
// impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
// impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
// impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
// impl_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
// impl_tuples!(
//   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20
// );
// impl_tuples!(
//   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21
// );
// impl_tuples!(
//   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
//   T22
// );
// impl_tuples!(
//   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
//   T22, T23
// );
// impl_tuples!(
//   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
//   T22, T23, T24
// );

macro_rules! impl_to_argument_tuples {
  () => {
    impl ToArguments for () {
      fn to_arguments(&self) -> PgArguments {
        PgArguments::default()
      }
    }
  };
  ($($t:ident),*) => {
    #[allow(non_snake_case, unused_parens)]
    impl<'a, $($t),*> ToArguments for ($($t),*,) where $($t : 'a + sqlx::Type<sqlx::Postgres> + std::marker::Sync + sqlx::Encode<'a, sqlx::Postgres> + Clone + std::marker::Send),* {
      fn to_arguments(&self) -> PgArguments {
        use sqlx::Arguments;
        let mut args = PgArguments::default();
        let ($($t),*) = self.clone();
        $(
          args.add( $t );
        )*
        args
      }
    }
  };
}

impl<'a, T> ToArguments for (T,)
where
  T: 'a
    + sqlx::Type<sqlx::Postgres>
    + std::marker::Sync
    + sqlx::Encode<'a, sqlx::Postgres>
    + Clone
    + std::marker::Send,
{
  fn to_arguments(&self) -> PgArguments {
    use sqlx::Arguments;
    let mut args = PgArguments::default();
    args.add(self.0.clone());
    args
  }
}

impl_to_argument_tuples!();
// impl_to_argument_tuples!(T1);
impl_to_argument_tuples!(T1, T2);
impl_to_argument_tuples!(T1, T2, T3);
impl_to_argument_tuples!(T1, T2, T3, T4);
impl_to_argument_tuples!(T1, T2, T3, T4, T5);
impl_to_argument_tuples!(T1, T2, T3, T4, T5, T6);
impl_to_argument_tuples!(T1, T2, T3, T4, T5, T6, T7);
impl_to_argument_tuples!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_to_argument_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_to_argument_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_to_argument_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_to_argument_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_to_argument_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_to_argument_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_to_argument_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_to_argument_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
// impl_to_argument_tuples!(
//   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17
// );
// impl_to_argument_tuples!(
//   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18
// );
// impl_to_argument_tuples!(
//   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19
// );
// impl_to_argument_tuples!(
//   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20
// );
// impl_to_argument_tuples!(
//   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21
// );
// impl_to_argument_tuples!(
//   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
//   T22
// );
// impl_to_argument_tuples!(
//   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
//   T22, T23
// );
// impl_to_argument_tuples!(
//   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21,
//   T22, T23, T24
// );
