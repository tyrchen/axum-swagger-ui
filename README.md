# Swagger UI

Swagger UI code comes from: [https://github.com/swagger-api/swagger-ui](https://github.com/swagger-api/swagger-ui).

Usage:

```rust
let doc_url = "/openapi.json";
let app = Router::new()
    .nest("/swagger-ui", SwaggerUi::setup(doc_url))
    .route(doc_url, get(openapi_spec_handler))
    // your other routes
    .route("/", get(|| async { "Hello, World!" }));
```
