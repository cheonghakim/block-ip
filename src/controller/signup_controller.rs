use actix_web::{get, http::StatusCode, post, web, Error, HttpRequest, HttpResponse, Responder};

use crate::{
    model::user_model::{LoginForm, SignupForm, User},
    service::{signup_service, user_service},
}; // Import the UserService

use crate::utils::response;
use crate::utils::utils::hash_password;
use actix_multipart::{
    form::{self, json::Json},
    Multipart,
};
use actix_session::Session;
use futures_util::StreamExt;

#[post("/api/ips/v1/signup")]
pub async fn signup(req: web::Json<SignupForm>) -> Result<impl Responder, actix_web::Error> {
    let id = req.id.clone();
    let password = req.password.clone();
    let email = req.email.clone();
    let hashed_password = hash_password(&password).expect("Failed to hash");

    match signup_service::SignupService::signup(&id, &hashed_password, &email) {
        Ok(user) => Ok(HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .json(response::Response::new(true, String::from("성공"), user))),
        Err(err) => {
            eprintln!("회원가입 에러: {:?}", err);
            Ok(
                HttpResponse::Ok().json(response::Response::<Option<User>>::new(
                    false,
                    String::from("회원가입에 실패 하였습니다."),
                    None,
                )),
            )
        }
    }
}
