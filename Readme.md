# Achecha

[![Clippy + fmt](https://github.com/norman784/ahecha/actions/workflows/check-and-lint.yaml/badge.svg)](https://github.com/norman784/ahecha/actions/workflows/check-and-lint.yaml)
[![Crate](https://img.shields.io/crates/v/ahecha.svg?color=brightgreen&style=flat-square)](https://crates.io/crates/ahecha)
[![Tests](https://github.com/norman784/ahecha/actions/workflows/test.yml/badge.svg)](https://github.com/norman784/ahecha/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A View Model framework written in rust, inspired by [next.js](https://nextjs.org).

## Motivation

I like the idea behind [next.js](https://nextjs.org), it's heavily oriented in building interactive
UI's that can be rendered in both client/server side, while I feel that the experience with
[next.js](https://nextjs.org) is ok, you can use Typescript to try to improve the experience/safety
but still isn't enough, so my goal is to try to port a similar experience into Rust.

## Goals

- Build component based webs
- Solid web app foundations
- Fun to work with
- Flexible
- Eliminate the need to maintain routes
- ~~Brainless ORM/migration system~~

## Features

- [x] Html templating
- [ ] Custom elements SSR
- [ ] Custom elements hydrate
- [ ] No more string routes references
- [ ] Mutation API (POST, PUT, PATCH, DELETE requests) with server and client side support
- [ ] Query API (GET request) with server and client side support
- [ ] Rest API support
- [ ] Page support
- [ ] Warp integration
- [ ] Rocket integration
- [ ] Actix-web integration

### Build component based webs

I like the idea of [next.js](https://nextjs.org), where the development feels seamsless while working
in the front or backend. Going with custom elements I believe is the best option, is supported
natively by the browsers so no need to write any extra dom manipulation framework for the frontend,
it might make the apps lighter and faster than other popular frameworks (react, vue, svelte, etc).

### Solid web app foundations

Web development for me feels so fragile, where you have your backend in X language and your frontend
in another, this most of the time brings friction and also is error prone, changing a property in
the backend could break the frontend, so you need to careful test all the app and write tests to
cover everything, this is somewhere where Rust shines, you can easily write tests but you don't need
to write too much extensive tests because the rust compiler do a lot of checks for you.

If you use something like [next.js](https://nextjs.org) you have one language for both, but the issue
for me is that is written in Javascript/Typescript, with the later you have type checks at compile
time but you lose that benefit when transpiling to Javascript to deploy your app, also another issue
I have with the Javascript ecosystem is the tooling, if you don't use vanilla JS you are mostly
doomed to setup a lot of different tools, with rust you have cargo that is the official tooling so
most if not  all project use them (I know that there are some improvements in that regard with
[esbuild](https://esbuild.github.io) and [swc](https://swc.rs)).

### Fun to work with

For me [next.js](https://nextjs.org) approach feels the same when I worked with php/html back in the
late 2000, was pretty straigh forward to setup your project, no fancy tooling, just one language
(back then you didn't use js too often, just small amount and that was the pre jquery era), also I
like how you can craft components and reuse in your project, sharing even more code in your project,
and with web components we can now doit natively (so no need to build a framework for that), also I
feel that the days where you wrote large stylesheet files are over, it's easier to write the style
where you want to use or have a small file shared by some components.

### Flexible

One thing I loved a while back from node was Express, it was so simple to work with and to extend it,
I like the middleware architecture, this makes easier to build a flexible system. You can just ship
the basic server and the developer can integrate his preferred crates.

### Eliminate the need to maintain routes

Why you need to manually define routes, you can easily infer the routes from file/folder structure,
also this forces you to adopt a solid structure for your project (in most mvc project you already do
this, but you are not forced to), also hardcoding your routes is another weak link in web
development, if a route changes then you suddenly your app breaks but you could not notice it until
is late, also helps with the url parameters and potentially with the types too (
[rocket](https://rocket.rs) has the uri! macro for this and I like it).

### Brainless ORM/migration system

There a few crates that are insteresting, [diesel](https://diesel.rs) is a full featured ORM, but the
ergnomics are not there for my taste. If you come from Ruby, NodeJS, etc you are used to good
ergonomics, maybe it can't be accomplish with Rust yet.

But [sqlx](https://crates.io/crates/sqlx) is an amazing crate, forget about all the complexity and
abstractions that an ORM bring to the table and write plain old sql, you can opt in for runtime or
compile time sql checking (for me the later is the best feature).

## What's the plan to achive this?

Build as much as possible from scratch (for reasons[1]) as a proof of concept, then if it feasible
with the ergonomics I wanted build a community to help and share ideas/experiences.

There are few important parts in the app:

- Server: [rocket](https://rocket.rs) and [actix-web](http://actix.rs).
- Client: here we need a lot of experimentation and research, need to check WASM/WASI capabilities
to build a thin layer over web components if possible and let the doors open to others to build
something more powerful over this foundation.
- Routing: using macros maybe? is an open question.
- ORM: [sqlx](https://crates.io/crates/sqlx)

[1] Learning, experimentation and research.

## What we need

> A way to parse the directory and generate the necessary code.

### Pages

- Get all the Pages
- Define the route based on the folder structure starting from src/pages
- Register the routes in the server and generate the PageRoutes enum holding all the routes with
their respective params

### API

- Get all the APIs
- Define the route based on the folder structure starting from src/api
- Register the routes in the server and generate the ApiRoutes enum holding all the routes with
their respective params

We will have special functions that will receive the  ApiRoutes that will help you to call those
functions in a seamless way (like react mutators / queries), the helpers will have all the
information to make the corresponding call (GET, POST, PUT, PATCH, DELETE)

### Tooling

> TBD

## Similar projects

- [Perseus](https://github.com/arctic-hen7/perseus)
