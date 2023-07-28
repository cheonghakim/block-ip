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
pub async fn login_user(
    req: web::Json<LoginForm>,
    session: Session,
) -> Result<impl Responder, Error> {
    let id = req.id.clone();
    let password = req.password.clone();
    let check = user_service::UserService::check_user(&id, &password);

    if check {
        // 로그인에 성공하면 사용자 ID를 세션에 저장
        if let Err(err) = session.insert("user_id", id) {
            eprintln!("Error while inserting user_id into session: {:?}", err);
            // 세션에 데이터를 저장하는데 실패하면 에러 응답 반환
            return Ok(HttpResponse::InternalServerError().finish());
        }

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(response::Response::<Option<User>>::new(
                true,
                String::from("성공"),
                None,
            )))
    } else {
        Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).json(
            response::Response::<Option<User>>::new(
                false,
                String::from("로그인에 실패 하였습니다."),
                None,
            ),
        ))
    }
}
