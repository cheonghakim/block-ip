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

#[post("/api/ips/v1/signup")]
pub async fn signup(mut req: web::Json<SignupForm>) -> impl Responder {
    let id = req.id.clone();
    let password = req.password.clone();
    let email = req.email.clone();
    let try_signup = signup_service::SignupService::signup(&id, &password, &email);

    match try_signup {
        Some(try_signup) => HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .json(response::Response::new(
                true,
                String::from("성공"),
                try_signup,
            )),
        _ => HttpResponse::Ok().json(response::Response::new(
            false,
            String::from("회원가입에 실패 하였습니다."),
            try_signup,
        )),
    }
}
