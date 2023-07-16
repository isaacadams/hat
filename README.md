# Usage

A `.toml` file configured with HTTP requests and assertions can be loaded by the `hat` CLI which will then execute the HTTP requests and run the assertions again the HTTP responses.

```toml
# see other examples of a hat .toml config file in the example folder
# e.g. example/local/config.toml
# e.g. example/pastebin/pastebin.toml
[environment]
base = "https://your-api-domain.com/api/v1"

[[tests]]
http = "GET {{base}}/users"
assertions = """
{{ status }} == 200
{{ headers | content-type }} == "application/json"
{{ body | users.0.username }} == "isaacadams"
{{ body | users.#(username=="isaacadams").username }} == "isaacadams"
"""
[tests.outputs]
userId = "{{ body | users.#(username==\"isaacadams\").id }}"

[[tests]]
http = "GET {{base}}/users/{{userId}}"
assertions = """
{{ status }} == 200
{{ headers | content-type }} == "application/json"
{{ body | username }} == "isaacadams"
"""
```

```console
$ hat --help
hat runs HTTP tests based on a toml configuration file.

The configured tests can check response status, heeaders, and body
using binary operations such as ==, >, <, !=, etc.

If one or more tests fail, hat will return a failed exit code.

Use --help for more USAGE details.

Project homepage: https://github.com/isaacadams/hat


Usage: hat[EXE] [OPTIONS] <PATH>

Arguments:
  <PATH>  path to .toml configuration file

Options:
  -v, --verbose <VERBOSE>  verbose level: DEBUG, INFO, ERROR [default: DEBUG]
  -h, --help               Print help
  -V, --version            Print version

```

# .http files

the `example/local/config.toml` uses a `create-post.http` file.

this is a file type unique to this CLI tool. Below is an example of how you can use an `.http` file. The idea that some requests are complex and the request bodies can become very large, distracting from the flow of the config file. Having the ability to define requests in their own file also opens up the possibility to reuse a request.

```http
POST {{base}}/posts
Content-Type application/json

[
    "I made a new post today"
]
```
