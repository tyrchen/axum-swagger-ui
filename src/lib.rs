const INDEX_STYLE: &str = include_str!("index.css");
const SWAGGER_UI_TEMPLATE: &str = r#"
<!-- HTML for static distribution bundle build -->
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <title>Swagger UI</title>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/5.10.3/swagger-ui.min.css" integrity="sha512-GjxduL6utVm4zShr/F1ulzFRBq08BMVm6vBKBsdIRajay+5JQCCo8nTB+RuUT6WrSeRh3TOJ7JbQNRXznpYlhg==" crossorigin="anonymous" referrerpolicy="no-referrer" />
    <style charset="UTF-8">{:index_style}</style>

    </head>

  <body>
    <div id="swagger-ui"></div>

    <script src="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/5.10.3/swagger-ui-bundle.min.js" integrity="sha512-sQ0p2uQ26Rl59qkMJt+ltkoBuJG2qFfkiA79QXhdxfr2JAIKZ8X+H8SWhMLIaNsFJPNNyJnN4RllWMrVczPIgw==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/5.10.3/swagger-ui-standalone-preset.min.js" integrity="sha512-qwGi7EG31HcylzamsmacHLZJrfUGRuuHEaCMcOojuNpMu+paR554VjaCZ9LdUVTrmF8xC03YVqTzuKx0SDdruA==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
    <script>
      window.onload = function() {
        window.ui = SwaggerUIBundle({
          url: "{:spec_url}",
          dom_id: '#swagger-ui',
          deepLinking: true,
          presets: [
            SwaggerUIBundle.presets.apis,
            SwaggerUIStandalonePreset
          ],
          plugins: [
            SwaggerUIBundle.plugins.DownloadUrl
          ],
          layout: "StandaloneLayout"
        });
      };
    </script>
  </body>
</html>
"#;

/// Returns the HTML for the Swagger UI page.
pub fn swagger_ui(spec_url: &'static str) -> String {
    SWAGGER_UI_TEMPLATE
        .replace("{:index_style}", INDEX_STYLE)
        .replace("{:spec_url}", spec_url)
}
