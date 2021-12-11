use crate::error::ApiError;
use actix_web::post;
use handcraft_models::inline::post_multipart_form_data::Response::Created;
use handcraft_models::inline::post_multipart_form_data::{Request, Response};
use handcraft_models::schemas::{
    MultiPartFormDataResponse, ReceivedBinary, ReceivedObject, ReceivedString,
};

#[post("/multipart_form_data")]
pub async fn post_multipart_form_data(request: Request) -> Result<Response, ApiError> {
    println!("[start] post_multipart_form_data: {:#?}", request);

    let body = request.body?;
    println!("body: {:#?}", body);

    Ok(Created {
        content: MultiPartFormDataResponse {
            string_field: ReceivedString {
                name: body.string_field.name()?.to_string(),
                value: body.string_field.to_string(),
            },
            binary_field: ReceivedBinary {
                name: body.binary_field.name()?.to_string(),
                length: body.binary_field.len() as i64,
                file_name: body.binary_field.file_name().map(|x| x.to_string()),
            },
            optional_string_field: None,
            optional_object_field: body.optional_object_field.map(|field| ReceivedObject {
                // TODO: remove unwrap()
                name: field.name().unwrap().to_string(),
                value: field.to_object(),
            }),
        },
    })
}
