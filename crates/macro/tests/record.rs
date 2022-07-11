#[cfg(feature = "backend")]
mod backend {
  // use ahecha_macro::Record;

  // #[test]
  // fn test_insert_record() {
  //   #[derive(Record)]
  //   #[record(insertable, table = "users")]
  //   struct User {
  //     firstname: String,
  //     lastname: String,
  //   }
  // }

  // #[test]
  // fn test_insert_record_returning() {
  //   #[derive(Record)]
  //   #[record(insertable, table = "users", returning(id))]
  //   struct User {
  //     firstname: String,
  //     lastname: String,
  //   }
  // }

  // #[test]
  // fn test_update_record() {
  //   use sqlx::types::Uuid;

  //   #[derive(Record)]
  //   #[record(updateable, constraint(id: Uuid), table = "users")]
  //   struct User {
  //     firstname: String,
  //     lastname: String,
  //   }
  // }

  // #[test]
  // fn test_update_record_returning() {
  //   #[derive(Record)]
  //   #[record(updateable, constraint(id: Uuid), table = "users", returning(id))]
  //   struct User {
  //     firstname: String,
  //     lastname: String,
  //   }
  // }

  // #[test]
  // fn test_delete_record() {
  //   use sqlx::types::Uuid;

  //   #[derive(Record)]
  //   #[record(deleteable, constraint(id: Uuid), returning(id), table = "users")]
  //   struct User {
  //     firstname: String,
  //     lastname: String,
  //   }
  // }
}
