use actix_web::{post, get, web, Responder};

use crate::domain::dto::nucleic_acids::NucleicAcidsDto;
use crate::common::result::IResult;

#[post("/nucleic/save")]
pub async fn save(arg: web::Json<NucleicAcidsDto>) -> impl Responder {
    log::info!("NucleicAcidsDto: {:?}", arg.0);

    let success_response: IResult<String> = IResult::success();
    web::Json(success_response)

}