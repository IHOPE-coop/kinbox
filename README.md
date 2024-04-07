This is the repository for the Kinbox server, an app to help humanity save itself via
[reciprocity](https://view.genial.ly/65e3bc100e521600145a316a/interactive-content-kinbox-reciprocal-stamp).

This project is written in Rust, uses [Maud](https://maud.lambda.xyz) for html templating, and [htmx](https://htmx.org) for reactivity.

To run it, first [install Rust](https://www.rust-lang.org/tools/install), open a command line in the project directory, and run `cargo run`.
The server will be viewable at `localhost:3000`.

We will host an instance for the PDX area.
We encourage others to host instances for their local communities.
Instances will start out isolated, but will be able to connect following the fediverse update.

Roadmap:
- [ ] "Virtual worksheet" page (no state persistence)
- [ ] User authentication and state persistence (add [surrealdb](https://surrealdb.com))
- [ ] Add reciprocal stamps
- [ ] Add support for fediverse (implement [atproto](https://atproto.com))
