# Usage

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
