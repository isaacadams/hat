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

```console
$ hat example/config.toml

✅ 200 OK GET http://localhost:5000/200 HTTP/1.1
📌 status only, no headers or body

  ✅ 200 == 200


✅ 200 OK GET http://localhost:5000/message HTTP/1.1
📌 status + json body, no headers

  ✅ 200 == 200
  ✅ "hello world!" == "hello world!"


✅ 200 OK POST http://localhost:5000/message HTTP/1.1
📌 post message w/ header

  ✅ 200 == 200
  ✅ 2 == 2
  ✅ "application/json" == "application/json"
  ✅ "hello, a second time" == "hello, a second time"
  ✅ "posts" == "posts"


✅ 200 OK GET http://localhost:5000/posts HTTP/1.1
📌 show example of querying arrays in response

  ✅ 200 == 200
  ✅ 175 > 0
  ✅ "how to build a CLI program in rust" == "how to build a CLI program in rust"
  ✅ "why you might need a vacation after build a CLI program in rust" == "why you might need a vacation after build a CLI program in rust"
  ✅ "reasons for drinking beer while writing a rust CLI program" == "reasons for drinking beer while writing a rust CLI program"


✅ 200 OK GET http://localhost:5000/posts/1 HTTP/1.1
📌 text body works

  ✅ 200 == 200
  ✅ "how to build a CLI program in rust" == "how to build a CLI program in rust"


✅ 201 Created POST http://localhost:5000/posts HTTP/1.1
📌 show example of querying arrays in response

  ✅ 201 == 201

```

```toml
# example/config.toml
[environment]
base = "http://localhost:5000"

[[tests]]
description = "status only, no headers or body"
http = "GET {{base}}/200"
assertions = """
{{status}} == 200
"""

[[tests]]
description = "status + json body, no headers"
http = "GET {{base}}/message"
assertions = """
{{status}} == 200
{{body | message}} == "hello world!"
"""

[[tests]]
description = "post message w/ header"
http = "POST {{base}}/message"
assertions = """
{{status}} == 200
{{body | id}} == 2
{{headers | content-type}} == "application/json"
{{body | message}} == "hello, a second time"
{{body | next_route}} == "posts"
"""
[tests.outputs]
messageId = "{{body | id}}"
nextRoute = "{{body | next_route}}"

[[tests]]
description = "show example of querying arrays in response"
http = "GET {{base}}/{{nextRoute}}"
assertions = """
{{status}} == 200
{{headers | content-length}} > 0
{{body | 0}} == "how to build a CLI program in rust"
{{body | 1}} == "why you might need a vacation after build a CLI program in rust"
{{body | 2}} == "reasons for drinking beer while writing a rust CLI program"
"""

[[tests]]
description = "text body works"
http = "GET {{base}}/posts/1"
assertions = """
{{status}} == 200
{{body}} == "how to build a CLI program in rust"
"""

[[tests]]
description = "show example of querying arrays in response"
http = "example/create-post.http"
assertions = """
{{status}} == 201
"""
```

the above configuration points to `create-post.http`, this is a file type unique to this CLI tool. Below is an example of how you can use an `.http` file. The idea that some requests are complex and the request bodies can become very large, distracting from the flow of the config file. Having the ability to define requests in their own file also opens up the possibility to reuse a request.

```http
POST {{base}}/posts
Content-Type application/json

[
    "I made a new post today"
]
```
