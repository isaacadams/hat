```console
$ hat example/pastebin/pastebin.toml
? success

✅ 200 OK GET https://pastebin.run/api/v1/languages HTTP/1.1

  ✅ 200 == 200
  ✅ "application/json" == "application/json"
  ✅ "Plain text" == "Plain text"


✅ 200 OK POST https://pastebin.run/api/v1/pastes HTTP/1.1

  ✅ 200 == 200
  ✅ 12 > 0


✅ 200 OK GET https://pastebin.run/[..].txt HTTP/1.1

  ✅ 200 == 200
  ✅ 21 > 0
  ✅ "I was created by hat." == "I was created by hat."

```