# Testing

```bash
# trycmd debug
cargo b && TRYCMD=dump cargo test --lib cli_tests
# trycmd refresh
cargo b && TRYCMD=OVERWRITE cargo test --lib cli_tests
```

```bash
# dev tools
cargo install cargo-watch
cargo install httpmock --features="standalone"
```

```bash
# run these two commands in separate terminals
cargo watch -c -w example/local/mock -- httpmock -p 5000 --mock-files-dir ./example/local/mock
cargo watch -c -w example -i example/local/mock -x 'run example/local/config.toml'
```

# Release

- change version in `Cargo.toml`
- running `scripts/release.sh` will create a git tag based on the version number in `Cargo.toml` and push the tag
- release workflow will kick off, generate cross platform binaries, and put everything into a github release

### TODO for Release

- `cargo publish` on successful releases
- implement CHANGELOG

## Binary Analysis

using `cargo audit` `cargo auditable`

### `cargo install cargo-bloat`

see biggest functions in release

```bash
# the following must be set in Cargo.toml for it to work
# [profile.release]
# strip = false
cargo bloat --release -n 10
```

see biggest dependencies in release build

```bash
cargo bloat --release --crates
```

# Docs

- https://rust-cli.github.io/book/tutorial/impl-draft.html
- using https://jsonplaceholder.typicode.com/guide/ for mocking tests and responses
- exit codes: https://rust-cli.github.io/book/in-depth/exit-code.html
- helpful cli projects
  - https://github.com/casey/just
  - https://github.com/BurntSushi/ripgrep

# Design

- Global Variables
  - can define global variables to be used in any step or multi-step test
- Test
  - `name`: add context or meaning to your test with a name or description
  - `http`: create the request in one of two ways
    - simple method + url: `<METHOD> <URL>` e.g. GET https://google.com
    - use an .http file and point to it: `path/to/my-http-request.http`
  - `assertions`: define your expectations of the HTTP response by querying the response data and using bit operations (==, >, <, !=, etc.)
    - e.g. check the response status `{{ status }} == 200`
    - e.g. check the response content length `{{ headers | content-length }} > 0`
    - e.g. check the response content length `{{ body | post }} == "I am a post"`
  - `outputs`: define the outputs of the test for use in any following test

```toml
# example of defining a test output(s)
[tests.outputs]
contentLength = "{{headers | content-length}}"
post = "{{body | post}}"
```

- Multi Step Test
  - can define outputs to be used in the next step
  - outputs cannot be used in the step they are defined

### Body Content Types

Response bodies can exist in multiple format kinds, such as xml, json, plain text, etc.

This tool aims to support most cases by making the data in those body formats available to be queried in assertions and passed as an output to be used in later tests.

| format | data type | data                               | query     | result        |
| ------ | --------- | ---------------------------------- | --------- | ------------- |
| json   | array     | `{ "posts": [ "I am a post" ] }`   | `posts.0` | "I am a post" |
| json   | object    | `{ "id": 0, post: "I am a post" }` | `id`      | 0             |

# Done

- tests
  - assertion strings with operators can be evaluated using `evalexpr`
  - test results have a nice output
  - if at least one test fails, the parent test name shows a failure
- variables
  - variables are matched and replaced
  - added support for single word e.g. {{ title }}
  - added support for json-like syntax e.g. {{ body | username }}
  - added support for json array syntax e.g. {{ body | 0.title }}
  - implemented output variables for multi-step tests
- .http files can be used for more complex requests w/ variable replacement
- supported response body content types:
  - json
  - text

# TODO

- bugs
  - headers may sometimes have ':' after header name
  - header values can be space delimited
- add `Download` section to README.md
- errors
  - ✅ if error occurs within test, fail the test, keep running all the other tests
  - display error underneath failed test
  - show failed test if it can't find specified `.http` file instead of panic
- deployment
  - cargo publish/cargo install
  - linux
  - choco
  - homebrew
- use the cli in a github action
  - https://docs.github.com/en/actions/creating-actions/creating-a-docker-container-action
  - https://docs.github.com/en/actions/creating-actions/publishing-actions-in-github-marketplace#publishing-an-action
- remove dependency on serde_json
- ✅ add STATUS URL to test output after name
- ✅ use new .http syntax that supports file path or raw
- ✅ integrate config_v2
- ✅ abstract service to replace reqwest for offline testing
- ✅ use a rust version of jq (https://github.com/MiSawa/xq or https://lib.rs/crates/jaq-parse)
- ✅ replace reqwest with hand rolled simple http client with (hopefully) zero dependencies
- ✅ finish plan for how to release
- ✅ exit codes
  - ✅ if at least one test fails, should respond with failed exit code

# Ideas / Improvements

- allow parameters to be passed into `.http` files as if they are a function
  - currently, `.http` files can reference variables from the global environment store
  - however, this may not be flexible enough for some cases where a `.http` file needs to be reused across multiple tests
  - in this case, the variables in the `.http` may change across each use, but the global environment store is static
  - thus, there should be a way to define parameters at the `.http` file level so that different variables can be passed in during testing
- support non-json response body extraction: xml querying and regex
- make global store aware of environment variables, and allow them to be used with {{env:...}} syntax
- run tests inside container with httpmock
  - job container: https://docs.github.com/en/actions/using-jobs/running-jobs-in-a-container
  - service container: https://docs.github.com/en/actions/using-containerized-services/about-service-containers
- add `http <PATH>` subcommand
  - `<PATH>` can be .http or .toml
  - if .http then it runs the requests and prints out the response status, headers, and body
  - if .toml then it runs every HTTP request in the toml and prints out each response status, headers, and body

# Missing Configuration

- named env settings (prod, dev, staging, etc.)
- default headers
- timeout on requests

# Helpful Commands

## Watch Specific Test

```bash
# watch specific tests
cargo watch -c -w src -x 'test -- --nocapture --test test::v2'
```

## Find Dependencies

```bash
# find crate dependency on specific target
# e.g. looking for openssl on x86_64-unknown-linux-musl target
cargo tree --target=x86_64-unknown-linux-musl | grep open
cargo tree --target=x86_64-unknown-linux-musl -i openssl-sys | grep open
```
