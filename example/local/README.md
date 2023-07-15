```console
$ hat example/local/config.toml

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
