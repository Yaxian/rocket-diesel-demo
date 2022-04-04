# rocket-diesel-demo

## Environment

`postgres` and `diesel-cli` preinstalled


`postgres` add new account, whose user/password is `postgres/pgpassword`


```
rustup override set nightly

```

```
cargo run
```

### visit


```
curl -X POST http://localhost:8000/posts/create --header 'Content-Type: application/json'  --data '{"title": "my first post", "body": "Hello World!"}'
```

```
curl http://localhost:8000/posts
```

```
curl -X DELETE http://localhost:8000/posts/delete/1
```