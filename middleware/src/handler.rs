use crate::{
    authenticate_token::AuthenticationGuard,
    google_oauth::{get_google_user, request_token},
    model::{AppState, NewUser, QueryCode, TokenClaims, User},
    response::{FilteredUser, UserData, UserResponse},
    schema::users::dsl::*,
};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie}, get, http::header::LOCATION, web, HttpResponse, Responder
};
use chrono::{prelude::*, Duration};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use jsonwebtoken::{encode, EncodingKey, Header};

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "App is Alive!";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

#[get("/sessions/oauth/google")]
async fn google_oauth_handler(
    query: web::Query<QueryCode>,
    data: web::Data<AppState>,
) -> impl Responder {
    let code = &query.code;
    let other_state = &query.state;

    if code.is_empty() {
        return HttpResponse::Unauthorized().json(
            serde_json::json!({"status": "fail", "message": "Authorization code not provided!"}),
        );
    }

    let token_response = request_token(code.as_str(), &data).await;
    if token_response.is_err() {
        let message = token_response.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let token_response = token_response.unwrap();
    let google_user = get_google_user(&token_response.access_token, &token_response.id_token).await;
    if google_user.is_err() {
        let message = google_user.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let google_user = google_user.unwrap();

    let mut conn = data.db_pool.get().expect("Failed to get DB connection");

    let user_email = google_user.email.to_lowercase();

    let other_user: Option<User> = users.filter(email.eq(user_email.clone())).first(&mut conn).ok();

    let datetime = Some(Utc::now().into()).unwrap();

    if other_user.is_some() {
        let mut other_user = other_user.unwrap();

        other_user.email = user_email.to_owned();
        other_user.photo = Some(google_user.picture);
        other_user.updated_at = datetime;
    } else {
        let user_data = NewUser {
            name: google_user.name,
            verified: google_user.verified_email,
            email:user_email,
            provider: "Google".to_string(),
            provider_id:Some(google_user.id.clone()),
            photo: Some(google_user.picture),
            created_at: datetime,
            updated_at: datetime,
        };

        // Replace with a pg call
        diesel::insert_into(users).values(user_data).execute(&mut conn).expect("Failed to insert new user!");
    }

    let jwt_secret = data.env.jwt_secret.to_owned();
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(data.env.jwt_max_age)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: google_user.id,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token)
        .path("/")
        .max_age(ActixWebDuration::new(60 * data.env.jwt_max_age, 0))
        .http_only(true)
        .finish();

    let frontend_origin = data.env.client_origin.to_owned();
    let mut response = HttpResponse::Found();
    response.append_header((LOCATION, format!("{}{}", frontend_origin, other_state)));
    response.cookie(cookie);
    response.finish()
}

#[get("/auth/logout")]
async fn logout_handler(_: AuthenticationGuard) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({"status": "success"}))
}

#[get("/users/me")]
async fn get_me_handler(
    auth_guard: AuthenticationGuard,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut conn = data.db_pool.get().expect("DB Connection failed!");
    
    let user: Option<User> = users.filter(provider_id.eq(auth_guard.user_id.to_owned())).first(&mut conn).ok();

    let json_response = UserResponse {
        status: "success".to_string(),
        data: UserData {
            user: user_to_response(&user.unwrap()),
        },
    };

    HttpResponse::Ok().json(json_response)
}

// To Do - Make this as a WIP route that lets you test alternative methods of data retrieval/log
// async fn get_users() -> impl Responder {
// }

pub fn user_to_response(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id.to_owned().to_string(),
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        verified: user.verified.to_owned(),
        photo: user.photo.clone().unwrap().to_owned(),
        provider: user.provider.to_owned(),
        created_at: user.created_at.to_owned(),
        updated_at: user.updated_at.to_owned(),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(google_oauth_handler)
        .service(logout_handler)
        .service(get_me_handler);

    conf.service(scope);
}
