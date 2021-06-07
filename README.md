# isometric-rust-app
Rust in the back, rust in the front. It's better than a mullet, because it's party all around

## About

This is just an example of how you could build a fully isometric Rust application where you're using Rust for both the backend and the frontend. The frontend is built with Yew (https://yew.rs/docs/intro/) and the backend is built with actix-web (https://github.com/actix/actix-web). I also included tailwindcss to demonstrate how you could also incorporate some fancy styling to turn this app into something more.

## Running

If you want to run this app in development, you will need to run two different commands: `make watch-server` and `make watch-webpack`. These will run the servers and will watch for changes and rebuild as necessary.

## Structure

The structure is using a Rust workspace broken apart into three separate crates:

1. server - The server side code (endpoints, routes, etc)
2. data - The data side, shared by server and ui
3. ui - The front end components and webpack configuration
