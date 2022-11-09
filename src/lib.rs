use axum::{response::Html, routing::get, Extension, Router};

const OAUTH_RECEIVER_HTML: &str = include_str!("oauth-receiver.html");
const INDEX_STYLE: &str = include_str!("index.css");
const SWAGGER_UI_TEMPLATE: &str = r#"
<!-- HTML for static distribution bundle build -->
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <title>Swagger UI</title>
    <link rel="stylesheet" type="text/css" href="//cdnjs.cloudflare.com/ajax/libs/swagger-ui/4.14.1/swagger-ui.min.css" />
    <style charset="UTF-8">{:index_style}</style>

    <script src="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/4.14.1/swagger-ui-bundle.min.js" integrity="sha512-mcnqgXGqnz02yD/AAjn/4bxoXzbDiJ2o7CJ9VD/l2J9OIH/4EHjWRyeGyxXWbcCDNBonOo38wcKle473zo48DA==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/4.14.1/swagger-ui-standalone-preset.min.js" integrity="sha512-KcIoqg6XREcrU/cNEm5Ovh2DbxQ4R1IkvIQOf4mtbbQmI3Oyb35KIaDfv5a0Blx73ogz0cu4cmHCWv36M2A3nw==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
    </head>

  <body>
    <div id="swagger-ui"></div>
    <script>
        fetch("{:spec_url}").then(response => {
            if (!response.ok) {
                throw new Error(`Failed to retrieve spec! Status: ${response.status}`);
            }
            return response.json();
        })
        .then(spec => {
            let oauth2RedirectUrl;

            let query = window.location.href.indexOf("?");
            if (query > 0) {
                oauth2RedirectUrl = window.location.href.substring(0, query);
            } else {
                oauth2RedirectUrl = window.location.href;
            }

            if (!oauth2RedirectUrl.endsWith("/")) {
                oauth2RedirectUrl += "/";
            }
            oauth2RedirectUrl += "oauth-receiver.html";

            SwaggerUIBundle({
                dom_id: '#swagger-ui',
                spec: spec,
                filter: false,
                oauth2RedirectUrl: oauth2RedirectUrl,
            })
        });
    </script>
  </body>
</html>
"#;

pub struct SwaggerUi;

impl SwaggerUi {
    pub fn setup(spec_url: &'static str) -> Router {
        Router::new()
            .route("/", get(index))
            .route("/oauth-receiver.html", get(oauth_receiver))
            .layer(Extension(spec_url))
    }
}

async fn index(Extension(spec_url): Extension<&'static str>) -> Html<String> {
    let html = SWAGGER_UI_TEMPLATE
        .replace("{:index_style}", INDEX_STYLE)
        .replace("{:spec_url}", spec_url);
    Html(html)
}

async fn oauth_receiver() -> Html<&'static str> {
    Html(OAUTH_RECEIVER_HTML)
}
