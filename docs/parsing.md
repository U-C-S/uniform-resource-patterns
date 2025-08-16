## URL Format

```
URI         = scheme ":" ["//" authority] path ["?" query] ["#" fragment]
authority   = [userinfo "@"] host [":" port]
```

```
[http,https]://*.google.com/meow/*?
```

### Valid Characters in URL

```
A-Z, a-z, 0-9,
-, ., _, ~, :, /, ?, #, [, ], @, !, $, &, ', (, ), *, +, ,, ;, %, =
```

# Pattern Matching

Standard pattern matching applies in `scheme`, `authority` (host and port), `path`, `query` and `fragment`. `Path` additionally supports `**` recursive primitive.

## Seperators

- The first `:` literal encountered in the regex, while seperate the pattern into scheme (to the left) and the rest
- If the first `:` literal is followed by `//` literal,

### Features specific to this Crate/Non-standard features

- {$:http,https}\*.google.com

[servo url crate](https://docs.rs/url/latest/url/)`

:
