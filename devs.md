# Testing

```bash
cargo t --all
```

```bash
cargo install cargo-watch
# install http mock server for testing
cargo install httpmock --features="standalone"
```

```bash
# run these two commands in separate terminals
cargo watch -c -w example/mock -- httpmock -p 5000 --mock-files-dir ./example/mock
cargo watch -c -w example -i example/mock -i **/*.md -x 'run example/config.toml'
```

# Release

- releases will have git tags associated with them
- version label sync? cargo.toml version updated by the git tag or git tag update the cargo.toml version
- action: Cross Platform Build -> on success, create release w/ tag using version on cargo.toml
- save build artifacts w/ commit hash on push to PR or based on special branch naming
- creating release off of main branch will use packages built on main@HEAD

- ? how to handle version in cargo.toml versus git tag version
- ? where to handle `cargo publish`

# Docs

- https://rust-cli.github.io/book/tutorial/impl-draft.html
- using https://jsonplaceholder.typicode.com/guide/ for mocking tests and responses
- possibly use the following crate for evaluating expressions: (https://docs.rs/evalexpr/latest/evalexpr/)
- exit codes: https://rust-cli.github.io/book/in-depth/exit-code.html
- https://github.com/casey/just

# Design

- Global Variables
  - can define global variables to be used in any step or multi-step test
- Test
  - `name`: add context or meaning to your test with a name or description
  - `http`: create the request in one of two ways
    - simple method + url: `<METHOD> <URL>` e.g. GET https://google.com
    - use an .http file and point to it: `path/to/my-http-request.http`
  - `assertions`: define your expectations by querying the response data and using bit operations (==, >, <, !=, etc.)
    - e.g. check the response status `{{r.status}} == 200`
    - e.g. check the response content length `{{r.headers.content-length}} > 0`
    - e.g. check the response content length `"{{r.body.post}}" == "I am a post"`
  - `outputs`: define the outputs of the test for use in any following test
    - e.g. `{{post}} = {{r.body.post}}`
- Multi Step Test
  - can define outputs to be used in the next step
  - outputs cannot be used in the step they are defined

### Body Content Types

Response bodies can exist in multiple format kinds, such as xml, json, plain text, etc.

This tool aims to support most cases by making the data in those body formats available to be queried in assertions and passed as an output to be used in later tests.

| format | data type | data                               | query        | result        |
| ------ | --------- | ---------------------------------- | ------------ | ------------- |
| json   | array     | `{ "posts": [ "I am a post" ] }`   | `.posts.[0]` | "I am a post" |
| json   | object    | `{ "id": 0, post: "I am a post" }` | `.id`        | 0             |

```rust
trait Queryable {
  fn query(filter: &str) -> String;
}

enum BodyContent {
  Json(serde_json::Value),
  Xml(String),
  Plaintext(String)
}

impl Queryable for BodyContent {
  // pass in arbitrary filter to extract data from body
  // e.g. Json -> filter = ".posts.[0]"
  // e.g. Plaintext -> filter = "/\w+/g"
  pub fn query(filter: &str) -> String;
}
```

# Done

- tests
  - assertion strings with operators can be evaluated using `evalexpr`
  - test results have a nice output
  - if at least one test fails, the parent test name shows a failure
- variables
  - variables are matched and replaced
  - added support for single word e.g. {{title}}
  - added support for json-like syntax e.g. {{response.status}}
  - added support for json array syntax e.g. {{response.body.[0].title}}
  - implemented output variables for multi-step tests
- .http files can be used for more complex requests w/ variable replacement
- supported response body content types:
  - json
  - text

# TODO

- make global store aware of environment variables, and allow them to be used with {{env:...}} syntax
- replace reqwest with hand rolled simple http client with (hopefully) zero dependencies
- finish plan for how to release
- exit codes
  - if at least one test fails, should respond with failed exit code
- errors
  - if error occurs within test, fail the test, keep running all the other tests and display error underneath failed test
  - show failed test if it can't find specified `.http` file instead of panic
- ✅ use new .http syntax that supports file path or raw
- ✅ integrate config_v2
- support non-json response body extraction: xml querying and regex
- deployment
  - cargo publish/cargo install
  - linux
  - choco
  - homebrew
- use the cli in a github action
- ✅ abstract service to replace reqwest for offline testing
- special syntax to build clients with base domain and auth
- use a rust version of jq (https://github.com/MiSawa/xq or https://lib.rs/crates/jaq-parse)
- build lexer to find {{token}} and replace them with Store trait

# Helpful Commands

```bash
# find crate dependency on specific target
# e.g. looking for openssl on x86_64-unknown-linux-musl target
cargo tree --target=x86_64-unknown-linux-musl | grep open
cargo tree --target=x86_64-unknown-linux-musl -i openssl-sys | grep open
```