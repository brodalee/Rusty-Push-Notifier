use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::handler::swagger::definition::ApiDoc;

pub fn configure(config: &mut web::ServiceConfig) {
    let mut api_doc = ApiDoc::openapi();
    api_doc.info.title = String::from("Rusty Push Notifier");
    api_doc.info.description = Option::from(String::from("An api to deliver web and mobile notifications"));

    config
        .service(
            SwaggerUi::new("/swagger/{_:.*}")
                .url("/swagger/doc.json", api_doc)
        );
}