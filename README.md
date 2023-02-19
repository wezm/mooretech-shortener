MooreTech Shortener
===================

This is a URL shortener that I build for [my YouTube channel][mooretech]. It's
implemented in Rust, which is compiled to WebAssembly for deployment to [Deno Deploy].

I published two videos on writing the code and deploying it:

* [Part 1](https://www.youtube.com/watch?v=d-tsfUVg4II)
* [Part 2](https://www.youtube.com/watch?v=bQnab_6K1ok)

The shortener is running at [tv.wezm.net](https://tv.wezm.net/).

Usage
-----

### Dependencies

* [Rust] with `wasm32-unknown-unknown` target installed
* [Deno]
* [wasm-bindgen]
* `make` (GNU make or BSD make will work)

### Tasks

After cloning the repo the following tasks are available:

* `make` — build the project
* `make run` — run the web server, building if necessary
* `make test` — run the integration tests

Deployment
----------

There are a few ways to deploy a project using this template to Deno Deploy:

* Manually with `deployctl`
* [Automatic git integration](https://deno.com/deploy/docs/projects#git-integration)
* GitHub Actions

The easiest, whilst also providing preview deployments is probably the
Automatic git integration. To do this there needs to be no build step so the
`build` directory must be committed to the repository.

[Rust]: https://www.rust-lang.org/
[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen
[Deno]: https://deno.land/
[Deno Deploy]: https://deno.com/deploy
[mooretech]: https://www.youtube.com/channel/UCLi0H57HGGpAdCkVOb_ykVg
