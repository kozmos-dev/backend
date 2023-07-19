#![feature(lazy_cell)]

pub mod database;

pub mod config;
#[macro_use]
pub mod utils;

use actix_files::{Files, NamedFile};
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde_json::{json, value::Value};
use std::sync::LazyLock;
use handlebars::*;
use std::boxed::Box;
use serde::{Deserialize, Serialize};

pub static TEMPLATES: LazyLock<Handlebars> = LazyLock::new(|| {
    let mut reg = Handlebars::new();
    reg.register_templates_directory(".hbs", "src/templates").unwrap();
    reg.register_helper("ifEquals", Box::new(if_equals));
    reg
});

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let config = config::CONFIG.read().unwrap();

    HttpServer::new(|| {
        App::new()
            .service(Files::new("/templates", "src/templates"))
            .service(script)
            .service(style)
            .service(home)
            .service(news)
            .service(explore)
            .service(posts)
            .service(guilds)
            .service(messages)
            .service(user)
            .service(user_with_page)
            .service(settings)
            .service(home)
    })
    .bind((config.web_server.address.as_ref(), config.web_server.port))?
    .run()
    .await
}

// AUTHENTICATION

#[derive(Deserialize, Serialize, Clone)]
struct AuthToken {
    sub: utils::CowStr,
}

enum AuthStatus {
    Authenticated,
    Unauthenticated,
    InvalidToken,
}

impl AuthStatus {
    pub async fn from_req(req: HttpRequest) -> AuthStatus {
        if let Some(token) = req.cookie("token") {
            let config = config::CONFIG.read().unwrap();
            
        }

        AuthStatus::Unauthenticated
    }

    pub async fn handle(self, req: HttpRequest) -> bool {
        true
    }
}

// JAVASCRIPT & CSS

#[get("/script.js")]
async fn script(req: HttpRequest) -> impl Responder {
    NamedFile::open_async("src/static/script.js").await
}

#[get("/style.css")]
async fn style(req: HttpRequest) -> impl Responder {
    NamedFile::open_async("src/static/compiled.css").await
}

// TEMPLATING HELP

fn if_equals<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    r: &'reg Handlebars<'reg>,
    ctx: &'rc Context,
    rc: &mut RenderContext<'reg, 'rc>,
    out: &mut dyn Output,
) -> HelperResult {
    let data = ctx.data();

    let mut key = &String::new();
    let p1 = h.param(0).map(|v| v.value()).ok_or(RenderError::new("param not found"))?;
    if let Value::String(k) = p1 {
        key = k;
    } else {
        return Err(RenderError::new("bad param type"));
    }

    let mut expected = &String::new();
    let p2 = h.param(1).map(|v| v.value()).ok_or(RenderError::new("param not found"))?;
    if let Value::String(e) = p2 {
        expected = e;
    } else {
        return Err(RenderError::new("bad param type"));
    }

    if let Value::Object(map) = data {
        if let Some(Value::String(value)) = map.get(key) {
            if value == expected {
                return h.template()
                    .map(|t| t.render(r, ctx, rc, out))
                    .unwrap_or(Ok(()));
            }
        }
    }

    h.inverse()
        .map(|t| t.render(r, ctx, rc, out))
        .unwrap_or(Ok(()))
}

// HOME

#[get("/")]
async fn home(req: HttpRequest) -> impl Responder {
    let auth = true;
    let body = TEMPLATES.render("app", &json!({"authenticated": auth, "container": "", "active": "home"})).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

// NEWS

#[get("/news")]
async fn news(req: HttpRequest) -> impl Responder {
    let auth = true;
    let body = TEMPLATES.render("app", &json!({"authenticated": auth, "container": "", "active": "news"})).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

// EXPLORE

#[get("/explore")]
async fn explore(req: HttpRequest) -> impl Responder {
    let auth = true;
    let body = TEMPLATES.render("app", &json!({"authenticated": auth, "container": "", "active": "explore"})).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

// POSTS

#[get("/posts")]
async fn posts(req: HttpRequest) -> impl Responder {
    let auth = true;
    let body = TEMPLATES.render("app", &json!({"authenticated": auth, "container": "", "active": "posts"})).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

// GUILDS

#[get("/guilds")]
async fn guilds(req: HttpRequest) -> impl Responder {
    let auth = true;
    let body = TEMPLATES.render("app", &json!({"authenticated": auth, "container": "", "active": "guilds"})).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

// MESSAGES

#[get("/messages")]
async fn messages(req: HttpRequest) -> impl Responder {
    let auth = true;
    let body = TEMPLATES.render("app", &json!({"authenticated": auth, "container": "", "active": "messages"})).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

// USERS

#[get("/@{name}")]
async fn user(path: web::Path<String>) -> Result<String> {
    let name = path.into_inner();
    Ok(format!("user: {}", name))
}

// pages:
// - following
// - followers
// - replies
// - media
// - likes
// - mutual friends
// - mutual guilds
#[get("/@{name}/{page}")]
async fn user_with_page(path: web::Path<(String, String)>) -> Result<String> {
    let (name, page) = path.into_inner();
    Ok(format!("user: {}, page: {}", name, page))
}

// SETTINGS

#[get("/settings")]
async fn settings(req: HttpRequest) -> impl Responder {
    let auth = true;
    let body = TEMPLATES.render("app", &json!({"authenticated": auth, "container": "", "active": "settings"})).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

// SIGN IN

#[get("/signin")]
async fn signin(req: HttpRequest) -> impl Responder {
    let auth = true;
    let body = TEMPLATES.render("app", &json!({"authenticated": auth, "container": "", "active": "signin"})).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

// SIGN UP

#[get("/signup")]
async fn signup(req: HttpRequest) -> impl Responder {
    let auth = true;
    let body = TEMPLATES.render("app", &json!({"authenticated": auth, "container": "", "active": "signin"})).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}
