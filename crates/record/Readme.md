# Ahecha Record

Ahecha Record is heavily inspired in [SQLx](https://github.com/launchbadge/sqlx). Tell me the features:

- Asynchronous library.
- Check at compile time the queries.
- Support for most common databases (Postgres, MySQL/MariaDB, SQLite).
- Works with multiple runtimes ( `async-std` / `tokio` / `actix`) and TLS backend (`native-tls` / `rustls`).

This seems a copy pasta from `SQLx`, right? Yes, that's because as stated above it's inspired by it, then why not use SQLx instead of Ahecha Record? Good question, and the answer is that we do not intent to support raw/pure SQL, but rather a SQL like language.

## Concepts

### Record

With the `record!` macro we define our models and check them against the database (or offline schema), and write our own metadata to use later. For example we define our records like:

```rust
record!("users", {
  pub struct User {
    id, // infers type and primary key from the schema
    name,
    email, // infers from the schema that is indexed
    password: Password, // You can still specify the type if you want, for cases like this
    created_at: DateTime<Utc>, // infers from the schema that has default
    updated_at: DateTime<Utc>, // infers from the schema that has default
  }

  // This are called variants, and are checked all with the same rule as the first one
  struct UserSignup {
    name,
    email,
    password,
  }

  pub(crate) struct UserSession {
    id,
    name,
    email,
  }

  struct Author {
    id,
    name,
    posts: Vec<Post>,
  }
});

record!("posts", {
  pub struct Post {
    id,
    title,
    content,
    author_id,
    created_at,
    updated_at,
  }

  struct PostAuthor {
    id,
    title,
    content,
    author: UserSession,
  }
});
```

And this expands to:

```rust
pub struct User {
  id: Uuid,
  name: String,
  email: String,
  password: Password,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

impl Record for User {}

struct UserSignup {
  name: String,
  email: String,
  password: Password,
}

impl Record for UserSignup {}

pub(crate) struct UserSession {
  id: Uuid,
  name: String,
  email: String,
  password: Password,
}

impl Record for UserSession {}

struct Author {
  id: Uuid,
  name: String
  posts: Vec<Post>
}

impl Record for Author {}

pub struct Post {
  id,
  title,
  content,
  author_id,
  created_at,
  updated_at,
}

impl Record for Post {}

struct PostAuthor {
  id,
  title,
  content,
  author: UserSession,
}

impl Record for PostAuthor {}
```

With derive:

```rust
#[derive(Record)]
#[record(table = "users")]
pub struct User {
  id: Uuid,
  name: String,
  email: String,
  password: Password,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

// Checks the struct integrity against the database/schema and implements the trait

impl Record for User {}
```

### Queries

The `query!` macro parses the query, check the fields, generate the SQL query and returns an executor. There are few concepts to learn here:

- SQL keywords are reserved, you can't name your struct/variables to use inside the macro if is a reserved keyword
- Capitalized tokens are interpreted as strucs
- A struct token followed by `::` indicates the the following token is a field of the struct
- Lower case tokens (not part of the SQL reserved keywords) are interpreted as variables

Will be ideal that the `query!` macro generates 100% of the time valid SQL, in case of error return a nice helpful error message with suggestions.

#### Select

```rust
query!(SELECT User WHERE User::id = 'abc')
// `SELECT User.id, User.name, User.email, password, created_at, updated_at FROM users as User WHERE User.id = 'abc'`
```

It already knows that Author uses the "users" table

```rust
query!(SELECT Author WHERE Author::id = 'abc')
// `SELECT Author.id, Author.name, json_agg(Post.id, Post.title, ...) FROM users as Author WHERE Author.id = 'abc';
// > :warning check if json_agg can do what I expect

query!(SELECT Author LEFT JOIN Post ON Post::author_id = Author::id) WHERE Author::id = 'abc' 
// `SELECT Author.id, Author.name, json_agg(Post.id, Post.title, ...) FROM users as Author LEFT JOIN posts AS Post ON Post.author_id = Author.id WHERE Author.id = 'abc';
```

#### Insert/Update

There are few new concepts here:

- Spread operator, it tells the macro to destructure the variable.
- Fields used in the WHERE statement will be excluded from the SET statement (in the UPDATE query).
- You can insert multiple rows using the `Vec<T>` type where `T` is the type being inserted/updated.

```rust
let post = Post { ... };
query!(INSERT Post VALUES (...post))
// `INSERT posts as Post (Post.id, Post.title, Post.content, ...) VALUES ($1, $2, ...)`, ...

let posts = vec![post { ... }, post { ... }];
query!(INSERT Vec<Post> VALUES (...post))
// `INSERT posts as Post (Post.id, Post.title, Post.content, ...) VALUES ($1, $2, ...), ($n1, $n2, ...)`, ...

query!(UPDATE Post SET ...post WHERE Post::id)
query!(UPDATE Post SET ...post WHERE Post::id = post.id) // Same as above
// `UPDATE posts as Post SET post.title = $1, post.content = $2, post.author_id = $3, post.created_at = $4, post.updated_at = $5 WHERE Post.id = $6`, post.title, ..., post.id
// `id` is automatically excluded from the SET statement because is used in the WHERE statement

query!(UPDATE Post SET ...post WHERE Post::id AND Post::author_id)
// `UPDATE posts as Post SET post.title = $1, post.content = $2, post.created_at = $3, post.updated_at = $4 WHERE Post.id = $5 AND Post.author_id = $6`, post.title, ..., post.id, post.author_id
// `id` and `author_id` are automatically excluded from the SET statement because is used in the WHERE statement

let posts = vec![post { ... }, post { ... }];
query!(UPDATE Vec<Post> SET ...post WHERE Post::id AND Post::author_id)
// `UPDATE posts as Post SET Post.title = PostRow.title, ... FROM (VALUES('Post title', ...), ($1, ...)) AS PostRow ($n, ...) WHERE Post.id = PostRow.id AND Post.author_id = PostRow.author_id
```

#### Delete

In this statement you need to always define the field and the value.

```rust
query!(DELETE Post WHERE Post::id = post.id)
// `DELETE posts as Post WHERE Post.id = $1`, post.id
```

### Issues and limitations

If multiple you define multiple structs with the same name in the `record!` macro they will override the metadata of the previous one, this is a limitation of this approach, the macro doesn't not know from where comes the struct being used (if they live in different modules).

Another issue is that sometimes will happen that the metadata is not in sync with the struct, if the `query!` macro is parsed before the `record!` macro, and the last one changed, then the metadata will be out of sync, one possible fix is to run a build script to parse all the `record!` macros contents in the project and generate the metadata, so the macro itself just generates rust code and doesn't write any file.

An alternative to the build script issue, we can have a cli tool that is needed in order to use the library, the only issue I have with that makes more hard to use the library, it requires more steps, etc, the build script might have the limitation that it does not know in which files are located the macro content that it needs to parse, so it will run every time you build your project, slowing your build times.

```rust
query!(SELECT NotARecord)
// Will throw a rust error complaining about the `Record` trait not being implemented, if you do so manually it will complain about
// the missing entry in the offline schema, if it has the same name as an existing one it might work if the fields are the same, if not,
// you will get some kind of error, this is a limitation of the system, because we don't have all the information about the struct it could
// leat to some issues if multiple structs share the same name.
```

### Database drivers

What is the best option, reuse existing database drivers or develop our own drivers?

If we develop our own we can share a lot of code between them and the usability will be similar, reusing existing ones might add a lot of extra complexity to make them all work the same. Or can we reuse the SQLx ones that already uses most probably the same interfaces.
