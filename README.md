# Force Graph

A very simple, arbitrarily dimensional, force-directed graph implementation in Rust.

This implementation is an exercise
in the use
of the [ndarray crate](https://docs.rs/ndarray/0.12.1/ndarray/index.html).

The well known force-directed graph algorithm used here
is known to work well for up to a hundred or so nodes.
Other more complex algorithms scale to larger graphs.

You could get a constant-time speedup
by avoiding the luxurious number of `ndarray` allocations
used in this code,
favoring instead re-use of arrays
and in-place operations.

## Dependencies

A Rust development environment is required,
e.g., as installed by [rustup](https://rustup.rs/).

The [cargo-make](https://docs.rs/crate/cargo-make/0.8.0) tool
is used to run the example web application.

## Todo: Librarification

The [glayout.rs](src/glayout.rs) sources contain the platform-independent
graph layout code, but this repo isn't set up to build a proper library right now.

## Example Web UI

An example showing the layout working
can be run via `trunk serve`
after installing
[trunk](https://trunkrs.dev/).

(This repo is also an exercise in Web Assembly and JavaScript interation.)

The most simple workflow is to hit the "New Graph" button
and then hit the iterate button to see the algorithm changing the layout.

You can pan the viewpoint and zoom using a scrollwheel or two fingers on a trackpad,
as usual for [graphosaurus](https://github.com/frewsxcv/graphosaurus).
