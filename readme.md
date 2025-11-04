# impulse

## server-driven fullstack template for performance and safety

### dependencies
- axum
- datastar
- sqlx on postgres

### Get Started

- development
```shell
npx @tailwindcss/cli -i ./src/input.css -o ./static/output.css --watch
cargo watch -x run
```

- build
```shell
pnpx @tailwindcss/cli -i ./src/input.css -o ./static/output.css --minify
cargo build
```
