# Achecha

[![Rust](https://github.com/ahecha-co/ahecha/actions/workflows/general.yml/badge.svg)](https://github.com/ahecha-co/ahecha/actions/workflows/general.yml)
[![Crate](https://img.shields.io/crates/v/ahecha.svg?color=brightgreen&style=flat-square)](https://crates.io/crates/ahecha)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# > ⚠️ Experimental, the readme might reflect the current state/goals of the project

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

## Features

- [x] Html templating
- [ ] Custom elements SSR
- [ ] Custom elements hydrate
- [x] No more string routes references
- [x] Page support
- [ ] Partial page refresh
- [ ] Live View

### Build component based webs

I like the idea of [next.js](https://nextjs.org), where the development feels seamsless while working
in the front or backend. Going with custom elements I believe is the best option, it's supported
natively by the browsers so no need to write any extra dom manipulation framework for the frontend,
it might make the apps lighter and faster than other popular frameworks (react, vue, svelte, etc).

### Solid web app foundations

Web development for me feels so fragile, where you have your backend in X language and your frontend
in another, this most of the time brings friction and also is error prone, changing a property in
the backend could break the frontend, so you need to careful test all the app and write tests to
cover everything, this is somewhere where Rust shines, you can easily write tests but you don't need
to write extensive tests because the rust compiler do a lot of checks for you.

If you use something like [next.js](https://nextjs.org) you have use the same language for both, but the issue
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
and with web components we can now doit natively (so no need to build a framework for that).

### Flexible

One thing I loved a while back from node was Express, it was so simple to work with and to extend it,
I like the middleware architecture, this makes easier to build a flexible system. You can just ship
the basic server and the developer can integrate his preferred libraries.

## Similar/Interesting projects

- [Dioxus](http://dioxuslabs.com)
- [Perseus](https://github.com/arctic-hen7/perseus)
