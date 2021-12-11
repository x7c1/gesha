use actix_web::post;
use handcraft_models::core::{FormDataField, ObjectContent};
use handcraft_models::errors::RequestError;
use handcraft_models::inline::post_multipart_form_data::Response::Created;
use handcraft_models::inline::post_multipart_form_data::{Request, Response};
use handcraft_models::schemas::{
    MultiPartFormDataResponse, ReceivedBinary, ReceivedObject, ReceivedString, SampleObjectField,
};

#[post("/multipart_form_data")]
pub async fn post_multipart_form_data(request: Request) -> crate::Result<Response> {
    println!("[start] post_multipart_form_data: {:#?}", request);

    let body = request.body?;
    println!("body: {:#?}", body);

    let optional_object_field = body
        .optional_object_field
        .map(to_received_object)
        .transpose()?;

    let string_field = ReceivedString {
        name: body.string_field.name()?.to_string(),
        value: body.string_field.to_string(),
    };
    let binary_field = ReceivedBinary {
        name: body.binary_field.name()?.to_string(),
        length: body.binary_field.len() as i64,
        file_name: body.binary_field.file_name().map(|x| x.to_string()),
    };
    Ok(Created {
        content: MultiPartFormDataResponse {
            string_field,
            binary_field,
            optional_string_field: None,
            optional_object_field,
        },
    })
}

fn to_received_object(
    field: FormDataField<ObjectContent<SampleObjectField>>,
) -> Result<ReceivedObject, RequestError> {
    Ok(ReceivedObject {
        name: field.name()?.to_string(),
        value: field.extract(),
    })
}
