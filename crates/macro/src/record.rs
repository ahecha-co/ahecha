/*
model!("users", {
  pub model User {
    id, // infers type and primary key from the schema
    name,
    email, // infers from the schema that is indexed
    password: Password, // You can still specify the type if you want, for cases like this
    created_at: DateTime<Utc>, // infers from the schema that has default
    updated_at: DateTime<Utc>, // infers from the schema that has default
  }

  // one_to_many {
  //   table: "posts",
  //   fields: [id],
  //   references: [author_id],
  //   name: Some("user_posts")
  // }

  // The variants can be skipped completely the types, they will be inferred from the model and fallback to the schema
  variant UserSignup {
    name,
    email,
    password,
  }

  pub(crate) variant UserSession {
    id,
    name,
    email,
  }

  variant Author {
    id,
    name,
    // Relations need to be specified, not all databases support references
    @relation("posts", fields: [id], references: [author_id]) posts: Vec<Post>,
  }

  variant LazyAuthor {
    id,
    name,
    @lazy @relation("posts", fields: [id], references: [author_id]) posts: Vec<Post>,
  }
});

// -- Expands to

pub enum UserPrimaryKey {
  Id
}

pub enum UserColumn {
  Name,
  Email,
  Password,
  CreatedAt,
  UpdatedAt,
}

impl ToString for UserColumn {
  fn to_string(&self) -> String {
    match self {
      UserColumn::Id => "id".to_owned(),
      //...
      UserColumn::CreatedAt => "created_at".to_owned(),
    }
  }
}

pub struct User {
  id: Uuid,
  name: String,
  email: String,
  password: Password,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

impl User {
  pub fn new(name: String, email: String, password: Password) -> Self {
    // ...
  }

  pub async fn create<R: Record>(record: R) -> Result<R> {
    // ...
  }

  pub async fn find_by_id<R: Record>(id: Uuid) -> Result<R> {
    // ...
  }

  pub async fn find_by_email<T: Record>(id: Uuid) -> Result<T> {
    // ...
  }

  pub async fn save(&self) -> Result<()> {
    // ...
  }

  pub async fn update<R: Record>(record: R) -> Result<()> {
    // ...
  }

  pub fn query() -> UserQuery {
    // ...
  }
}

pub enum UserQueryField {
  Id(Uuid),
  Name(String),
  Email(String),
  CreatedAt(DateTime<Utc>),
  UpdatedAt(DateTime<Utc>),
}

pub enum Op {
  Equal,
  GreaterThan,
  GreaterThan,
  LessOrEqualThan,
  LessOrEqualThan,
}

pub enum Order {
  Asc,
  Desc,
}

pub struct UserQuery {
  // ...
}

impl UserQuery {
  pub fn where(field: UserField, op: Op) -> Self {
    // ...
  }

  pub fn order(field: UserField, order: Order) -> Self {
    // ...
  }
}

impl Table for User {
  fn table_name() -> &'static str {
    "users"
  }
}

impl Record for User {
  fn record_columns() -> Vec<UserColumn> {
    vec![
      UserColumn::Name,
      // ...
      UserColumn::UpdatedAt,
    ]
  }

  fn record_primary_key_values() -> Arguments {
    args![self.id.clone()]
  }

  fn record_values() -> Arguments {
    args![
      self.name.clone(),
      // ...
      self.updated_at.clone(),
    ]
  }
  // ...
}

struct UserSignup {
  name: String,
  email: String,
  password: Password,
}

impl Record for UserSignup {
  // ...
}

pub(crate) struct UserSession {
  id: Uuid,
  name: String,
  email: String,
  password: Password,
}

impl Record for UserSession {
  // ...
}

enum AuthorField {
  Id
  Name,
  Posts,
}

impl ToString for AuthorField {
  fn to_string() -> String {
    // ...
  }
}

enum AuthorRelationReference {
  AuthorId
}

impl ToString for AuthorRelationReference {
  fn to_string() -> String {
    // ...
  }
}

enum AuthorRelation {
  Posts
}

impl AuthorRelation {
  fn fields() -> Vec<AuthorField> {
    vec![AuthorField::Id]
  }

  fn references() -> Vec<AuthorRelationReference> {
    vec![AuthorRelationReference::AuthorId]
  }
}

struct Author {
  id: Uuid,
  name: String
  posts: Vec<Post>
}

impl Record for Author {
  // ...
  fn record_relations() -> Vec<AuthorRelation> {
    vec![AuthorRelation::Posts]
  }

  fn record_relations_values() -> Arguments {
    args![self.id.clone()]
  }
}

struct LazyAuthor {
  id: Uuid,
  name: String
  posts: Vec<Post>
}

impl LazyAuthor {
  pub async fn posts() -> Vec<Post {
    // ...
  }
}

impl Record for Author {
  // ...
}
*/
