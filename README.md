# Force Graph

A very simple, arbitrarily dimensional, force-directed graph implementation in Rust.

This implementation is an exercise
in the use
of the [ndarray crate](https://docs.rs/ndarray/0.12.1/ndarray/index.html).

The well known force-directed graph algorithm used here
is known to work well for up to a hundred or so nodes.
Other more complex algorithms scale to larger graphs.

## Example Web UI

An example showing the layout working
can be run via `trunk serve`
after installing
[trunk](https://trunkrs.dev/).