pub mod index;

#[macro_export]
macro_rules! register_services {
    ($app: ident) => {
        $app.service(generated::index)
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
