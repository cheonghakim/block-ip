// controllers/user_controller.rs

use actix_web::{get, http::StatusCode, post, web, Error, HttpRequest, HttpResponse, Responder};

use crate::{
    model::user_model::{LoginForm, SignupForm, User},
    service::{signup_service, user_service},
}; // Import the UserService

use crate::utils::response;
use actix_multipart::{
    form::{self, json::Json},
    Multipart,
};
use actix_session::Session;
use futures_util::StreamExt;

#[post("/api/ips/v1/login")]
pub async fn login_user(mut req: web::Json<LoginForm>, session: Session) -> impl Responder {
    let id = req.id.clone();
    let password = req.password.clone();
    let check = user_service::UserService::check_user(&id, &password);

    match check {
        Some(check) => HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .json(response::Response::new(true, String::from("성공"), check)),
        _ => HttpResponse::build(StatusCode::UNAUTHORIZED).json(response::Response::new(
            false,
            String::from("로그인에 실패 하였습니다."),
            check,
        )),
    }
}
