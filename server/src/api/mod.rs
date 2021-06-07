use actix_web::{get, HttpResponse, Result};

extern crate data;
use data::DataResponse;

#[get("/data")]
pub async fn get_data()-> Result<HttpResponse> {
    let response = DataResponse {
        status: "Ok".to_string()
    };

    Ok(
        HttpResponse::Ok().json(
            response
        )
    )
}