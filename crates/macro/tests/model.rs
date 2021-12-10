// use ahecha_macro::*;

// #[Derive(Table)]
// #[table(name = "users", insert, update, delete)]
// struct User {
//   #[primary_key]
//   id: i32,
//   firstname: String,
//   lastname: String,
//   email: String,
//   #[one_to_one(table="user_settings", fields=["id"], references=["user_id"])]
//   settings: Settings,
//   #[one_to_many(table="posts", fields=["id"], references=["author_id"])]
//   posts: Vec<Post>,
//   #[one_to_one(table="tenants", fields=["tenant_id"], references=["id"])]
//   tenant: Tenant,
//   // This field will be required to be filled on every SELECT query
//   #[multitenancy]
//   tenant_id: i32,
// }

// #[derive(Table)]
// #[table(name = "posts", insert, update)]
// struct Post {
//   #[primary_key]
//   id: i32,
//   author_id: i32,
//   #[one_to_one(table="users", fields=["user_id"], references=["id"])]
//   author: User,
// }

// #[derive(Table)]
// #[table(name = "users", update)]
// struct Profile {
//   #[primary_key]
//   id: i32,
//   firstname: String,
//   lastname: String,
// }

// #[derive(Table)]
// #[table(name = "posts")]
// struct LatestPost {
//   #[primary_key]
//   id: i32,
//   title: String,
//   content: String,
//   author_id: i32,
//   #[one_to_one(table="users", fields=["user_id"], references=["id"])]
//   author: User,
// }
