#[macro_export]
macro_rules! sample_server {
    ($api: expr) => {
        actix_web::HttpServer::new(|| {
            actix_web::App::new()
                .data($api)
                .service(index)
        })
    };
}
