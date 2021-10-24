#[macro_export]
macro_rules! register_services {
    ($app: ident, $($module: ident)::*) => {
        $app.service($($module::)* generated::index)
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

#[macro_export]
macro_rules! delegate {
    ($handlers: ident . $op: ident ($raw: ident)) => {
        actix_web::Result::Ok(
            match handcraft_models::inline::$op::Request::from_raw($raw).await {
                Ok(request) => {
                    let response = $handlers.$op(request).await;
                    handcraft_models::inline::$op::Responder::to_raw(response)
                }
                Err(e) => $handlers.on_bad_request(e),
            },
        )
    };
}

use handcraft_models::inline::RequestError;

pub trait BadRequestHandler {
    fn on_bad_request(&self, error: RequestError) -> actix_web::HttpResponse;
}
