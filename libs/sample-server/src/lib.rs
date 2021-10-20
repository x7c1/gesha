#[macro_export]
macro_rules! register_services {
    ($app: ident, generated in $($module: ident)::*) => {
        $app.service($($module::)*generated::index)
            .service($($module::)*generated::show_pet_by_id)
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
