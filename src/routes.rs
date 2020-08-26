use actix_web::{ get, post, HttpResponse, web::{ Data, Json, ServiceConfig } };

use crate::models::{ Team, Member, ApiResult };
use sqlx::PgPool;

#[get("/")]
pub async fn index() -> String {
    "Hello World".to_owned()
}

#[get("/members")]
pub async fn list(connection: Data<PgPool>) -> HttpResponse {
    let result = Team::list(&connection).await;

    match result {
        Ok(members) => HttpResponse::Ok().json(members),
        _ => HttpResponse::Ok().json(
            ApiResult {
                success: false
            }
        )
    }
}

#[get("/member")]
pub async fn get_one_member(connection: Data<PgPool>) -> HttpResponse {
    let result = Team::specific(&connection).await;

    match result {
        Ok(members) => HttpResponse::Ok().json(members),
        _ => HttpResponse::Ok().json(
            ApiResult {
                success: false
            }
        )
    }
}

#[post("/insert")]
pub async fn insert(member: Json<Member>, connection: Data<PgPool>) -> HttpResponse {
    let result = Team::new_member(&member.into_inner() ,&connection).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(
            ApiResult {
                success: true
            }
        ),
        _ => HttpResponse::Ok().json(
            ApiResult {
                success: false
            }
        )
    }
}

pub fn routes(config: &mut ServiceConfig) {
    config
        .service(index)
        .service(get_one_member)
        .service(list)
        .service(insert);
}