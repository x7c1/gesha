#[macro_export]
macro_rules! register_services {
    ($app: ident, $($module: ident)::*) => {
        $app.service($($module::)* generated::index)
            .service($($module::)* generated::add_pet)
            .service($($module::)* generated::create_pets)
            .service($($module::)* generated::find_pets)
            .service($($module::)* generated::list_pets)
            .service($($module::)* generated::show_pet_by_id)
    };
    ($app: ident --generated = $($module: ident)::*) => {
        register_services!($app, $($module)::*)
    };
    ($app: ident) => {
        register_services!($app,)
    };
}

#[macro_export]
macro_rules! http_server {
    ($handlers: expr) => {
        actix_web::HttpServer::new(|| {
            let app = actix_web::App::new().data($handlers);
            register_services!(app)
        })
    };
}

use handcraft_models::errors::RequestError;

pub trait BadRequestHandler {
    fn on_bad_request(&self, error: RequestError) -> actix_web::HttpResponse;
}
