This is the repository for the Kinbox server, an app to help humanity save itself via
[reciprocity](https://view.genial.ly/65e3bc100e521600145a316a/interactive-content-kinbox-reciprocal-stamp).

This project is written in Rust, uses [Maud](https://maud.lambda.xyz) for html templating, and [htmx](https://htmx.org) for reactivity.

To run it, first [install Rust](https://www.rust-lang.org/tools/install), open a command line in the project directory, and run `cargo run`.
The server will be viewable at `localhost:3000`.

Roadmap:
- [ ] Technical proof of concept of Rust, Maud, and htmx
- [ ] Flesh out user interface with dummy data
- [ ] Build [surrealdb](https://surrealdb.com) database
- [ ] Implement [atproto](https://atproto.com) protocol for fediverse support
