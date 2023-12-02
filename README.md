# Swagger UI

Swagger UI code comes from: [https://github.com/swagger-api/swagger-ui](https://github.com/swagger-api/swagger-ui).

Usage:

```rust
let doc_url = "swagger/openapi.json";
let app = Router::new()
    .route("/swagger", get(|| async { swagger_ui(doc_url) }))
    .route(doc_url, get(|| async { include_str!("openapi.json") }))
    // your other routes
    .route("/", get(|| async { "Hello, World!" }));
```
