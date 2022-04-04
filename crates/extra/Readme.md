# Ahecha extra

Experimental utilities for ahecha.

## Experimental/in development features

### Page

The idea around pages is that when you define a page, it will have a route, handler and a view, so a build script or maybe a macro could
register into a file all the routes that then will be automounted via a macro, maybe a macro derive over a function that will receive the
routes as argument.

### Partial

A partial is a component that could be rendered and served from the backend to the frontend, only rendering the component, without the app
layout. This will work similar to pages, they need to be also mounted in the router with a prefix that could be
`/__ahecha/partials/:partial_id` they could also custom route params.

An open question is that how to handle cases where it, when rendered in the server, received props from a sql query and then when doing a
partial request we need to execute that query again, so it means that we need to dup the queries? This will make it more cumbersome and not
so good UX.

### Custom elements

This is an open question still, but the idea is that the component could be rendered server side and also compiled via wasm to register in
the frontend the wasm custom elements.

The only downside I could find to the custom elements is that AFAIK it doesn't fulfill the same role as React or similar frameworks, so it
seems harder to build an SPA around them. The main idea on using custom elements was to remove the need to build a framework to deal with
rendering the DOM, React/Yew uses VDOM.

This needs more testing and research.

### Record

The record will be an SQL-ish syntax, ideally it should improve the sqlx ergonomics with a slightly
different approach.

```rust
#[derive(ahecha::Table)]
#[table(name = "users")]
struct User {
  id: i32,
  name: String,
  age: i32,
}

let id = 5;

// we can infer the field name from the variable name, only if the expression is like the following
let user = ahecha::query!(select #User where {id}).first(pool).await;
// Alternative
let user = ahecha::query!(select #User where id = {id}).first(pool).await;
// sqlx equivalent
let user = sqlx::query_as!(User, "SELECT id, name, age FROM users WHERE id = $1", id).fetch_one(pool).await;
```

#### Open questions

##### Should we generate a metadata file for each struct that derives `ahecha::Table`?

Ideally write to the target directory

##### How to improve build times without being annoying?

sqlx is playing with the idea of having a daeamon that runs in the background and do the heavy lifting,
the macro just pass the sql string to the daemon, somehow, and then it generates the code with the cli
output? Is this something similar to the LSP?

Could we have the daemon running and monitoring the database, so when the macro call the cli it will use
the in memory metadata or just have the daemon update some files and the macro read those files? Ideally
those files should be commited to the repository so the entire process will be executed always offline,
if the daemon isn't running the metadata could get out of sync.

Other approach could be that when running the migrations we generate the metadata, this should be much
more effective, so we store the metadata for each table in an easy to deserialize format that is still
readable?

##### There are some downsides or blockers?

One that comes to mind is that we need somehow parse the sql query, so we mihgt need to maintain a parser
for each engine, that could make this project bigger/complex that we want. This could lead to support
less databases, if so I would think that mysql/postgresql would be the most supported. OTOH we could have
a core crate and implement each engine in a separate crate, this will allow to support different engines
within the community.

##### If we go full greed, do we want to replicate somehow prisma?

This, but without the need to have a `.prisma` file, maybe we can collect all the metadata of all the
declarations from the `ahecha::Table` derive for the same table and then generate the `.sql` file for the
migration, this will require us to have a diff system to check what changed between our current offline
schema and the online schema. Because we generate a `.sql` file the user will be able to do the changes
needed to fix what is missing, this will be a lot easier to do than to do the migration by hand.

#### Can we fix a downside of sqlx that has to do with types?

One thing that throws me off is that sqlx and I think other drivers doesn't support for example `u32`
with postgres, and postgres doesn't support it natively, it needs some workarouds, but we with all the
metadata could generate the necessary code to support it.
