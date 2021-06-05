use actix_web::{get, HttpResponse, Result};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct DataResponse {
    status: String,
}

#[get("/data")]
pub async fn data()-> Result<HttpResponse> {
    let response = DataResponse {
        status: "Ok".to_string()
    };

    Ok(
        HttpResponse::Ok().json(
            response
        )
    )
}