## URL Format

```
URI         = scheme ":" ["//" authority] path ["?" query] ["#" fragment]
authority   = [userinfo "@"] host [":" port]
```

```
[http,https]://*.google.com/meow/*?
```

# Pattern Matching
Standard pattern matching applies in `scheme`, `authority` (host and port), `path`, `query` and `fragment`. `Path` additionally supports `**` recursive primitive.

## Seperators
- The first `:` literal encountered in the regex, while seperate the pattern into scheme (to the left) and the rest
- If the first `:` literal is followed by `//` literal,