use actix_web::{
    get,
    put,
    post,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};

use strum_macros::Display;

use serde::{Serialize, Deserialize};


#[derive(Debug, Display)]
pub enum RequestErorr {
    BadRequest,
    RequestNotFound,
    RequestUpdateFailure,
    RequestCreationFailure,
}

impl ResponseError for RequestErorr {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            RequestErorr::BadRequest => StatusCode::BAD_REQUEST,
            RequestErorr::RequestNotFound => StatusCode::NOT_FOUND,
            RequestErorr::RequestUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            RequestErorr::RequestCreationFailure => StatusCode::FAILED_DEPENDENCY
        }
    }
}

#[derive(Deserialize)]
pub struct SubmitSupportRequestForm {
   req_name: String,
   req_email: String,
   support_type: String,
   sup_message: String,
}

#[derive(Serialize)]
pub struct RequestIdentifier {
    request_global_id: String,
}



#[post("/create_request")]
pub async fn create_request(request_data: Json<SubmitSupportRequestForm>) -> Result<Json<RequestIdentifier>, RequestErorr>  {
    println!("New Support Request Recieved!\nName: {}\nEmail: {}\nSupport Type: {}\nMessage: {}", request_data.req_name, request_data.req_email, request_data.support_type, request_data.sup_message);

    let sup_req_id = RequestIdentifier {request_global_id: "12312314124".to_string()};

    Ok(Json(sup_req_id))
    //create new Support_Req
        // add method to Support_Req struct to create uuid
    //push db
    //return id?
}