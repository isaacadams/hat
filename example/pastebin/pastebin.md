```console
$ hat example/pastebin/pastebin.toml
? success

✅ 200 OK POST http://localhost:7777/documents HTTP/1.1

  ✅ 200 == 200


✅ 200 OK GET http://localhost:7777/raw/[..] HTTP/1.1

  ✅ 200 == 200
  ✅ "I was created by hat." == "I was created by hat."

```
