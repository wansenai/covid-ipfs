use actix_web::{post, get, web, Responder};

use crate::domain::dto::nucleic_acids::NucleicAcidsDto;
use crate::common::result::IResult;
use crate::service::nucleic_service;

#[post("/nucleic/save")]
pub async fn save(arg: web::Json<NucleicAcidsDto>) -> impl Responder {
    log::info!("NucleicAcidsDto: {:?}", arg.0);

    let data = arg.0;

    let hash = nucleic_service::save_nucleic_ipfs(data).await;

    let success_response: IResult<String> = IResult::success_data(hash);
    web::Json(success_response)

}

#[get("/nucleic/query/{hash}")]
pub async fn query(hash: web::Path<String>) -> impl Responder {
    log::info!("hash: {:?}", hash);

    let data = nucleic_service::get_nucleic(hash.to_string()).await;

    let success_response: IResult<String> = IResult::success_data(data);
    web::Json(success_response)

}