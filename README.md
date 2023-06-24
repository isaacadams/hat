# Usage

- `hat <PATH>`
- `hat config.toml`

Define your tests in a `*.toml` file and then call `hat <name-of-your-file>.toml`

```toml
# <name-of-your-file>.toml
[environment]
base = "http://localhost:5000"

[[tests]]
name = "status only, no headers or body"
http = "GET {{base}}/200"
assertions = """
{{r.status}} == 200
"""

[[tests]]
name = "status + json body, no headers"
http = "GET {{base}}/message"
assertions = """
{{r.status}} == 200
"{{r.body.message}}" == "hello world!"
"""

[[tests]]
name = "post message w/ header"
http = "POST {{base}}/message"
assertions = """
{{r.status}} == 200
{{r.body.id}} == 2
"{{r.headers.content-type}}" == "application/json"
"{{r.body.message}}" == "hello, a second time"
"{{r.body.next_route}}" == "posts"
"""
[tests.outputs]
messageId = "{{r.body.id}}"
nextRoute = "{{r.body.next_route}}"

[[tests]]
name = "show example of querying arrays in response"
http = "GET {{base}}/{{nextRoute}}"
assertions = """
{{r.status}} == 200
{{r.headers.content-length}} > 0
"{{r.body.[0]}}" == "how to build a CLI program in rust"
"{{r.body.[1]}}" == "why you might need a vacation after build a CLI program in rust"
"{{r.body.[2]}}" == "reasons for drinking beer while writing a rust CLI program"
"""

[[tests]]
name = "text body works"
http = "GET {{base}}/posts/1"
assertions = """
{{r.status}} == 200
"{{r.body}}" == "how to build a CLI program in rust"
"""

[[tests]]
name = "show example of querying arrays in response"
http = "example/create-post.http"
assertions = """
{{r.status}} == 201
"""
```

the above configuration points to `create-post.http`, this is a file type unique to this CLI tool. Below is an example of how you can use an `.http` file. The idea that some requests are complex and the request bodies can become very large, distracting from the flow of the config file. Having the ability to define requests in their own file also opens up the possibility to reuse a request.

```
POST {{base}}/posts
Content-Type application/json

[
    "I made a new post today"
]
```
