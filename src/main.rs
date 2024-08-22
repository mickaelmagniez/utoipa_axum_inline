use std::fmt;

use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use axum::response::Response;
use utoipa::OpenApi;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/openapi.json", get(openapi))
        .route("/test_2params_separated/:resource_type/:id", get(test_2params_separated))
        .route("/test_1param/:resource_type/:id", get(test_1param))
        .route("/test_struc/:resource_type/:id", get(test_struc));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[utoipa::path(
    get, 
    path = "/test_2params_separated/{resource_type}/{id}",
    params(("resource_type" = inline(ResourceType), Path, ),("id" = String, Path, ))
)]
pub async fn test_2params_separated(Path((resource_type, id)): Path<(ResourceType, String)>) -> String {
    format!("Hello, World! {} {}", resource_type, id)
}

#[utoipa::path(
    get, 
    path = "/test_1param/{resource_type}",
    params(("resource_type" = inline(ResourceType), Path,))
)]
pub async fn test_1param(Path(resource_type): Path<ResourceType>) -> String {
    format!("Hello, World! {}", resource_type)
}

#[utoipa::path(
    get,
    path = "/test_struc/{resource_type}/{id}",
    params(PathStruct)
)]
pub async fn test_struc(Path(p): Path<PathStruct>) -> String {
    format!("Hello, World! {} {}", p.resource_type, p.id)
}

#[derive(ToSchema, Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
pub enum ResourceType {
    Type1,
    Type2,
}
impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = serde_json::to_string(&self).map_err(|_| fmt::Error)?;
        write!(f, "{}", &string[1..string.len() - 1])
    }
}

#[derive(utoipa::IntoParams, Deserialize, Serialize, Clone, Debug)]
pub struct PathStruct {
    #[param(inline)]
    pub resource_type: ResourceType,
    pub id: String,
}

pub async fn openapi() -> Response {
    #[derive(OpenApi)]
    #[openapi(paths(test_1param, test_2params_separated, test_struc), components(schemas(ResourceType,)))]
    struct ApiDoc;
    ApiDoc::openapi().to_pretty_json().unwrap().into_response()
}
