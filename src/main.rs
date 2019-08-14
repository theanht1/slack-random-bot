use actix_web::{App, web, get, post, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;

pub mod errors;
pub mod random;
pub mod commands;

#[get("/")]
fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[derive(Deserialize, Debug)]
struct AuthRequest {
    code: String,
}

#[get("/slack/auth_redirect")]
fn slack_auth_redirect(query: web::Query<AuthRequest>) ->  impl Responder {
    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let url = format!("https://slack.com/api/oauth.access?code={}&client_id={}&client_secret={}",
                      query.code, client_id, client_secret);

    let mut res = reqwest::get(&url[..]);
    let response = match res {
        Ok(r) => {
            println!("Auth successfully");
            "Random app has been installed in your workspace. Thank you!"
        },
        Err(e) => {
            println!("Error: {:?}", e);
            "Something wrong. Please try again!"
        },
    };
    HttpResponse::Ok().body(response)
}

#[derive(Deserialize, Debug)]
struct SlackFormData {
    token: String,
    team_id: String,
    team_domain: String,
    channel_id: String,
    channel_name: String,
    user_id: String,
    user_name: String,
    command: String,
    text: String,
    response_url: String,
    trigger_id: String,
}

#[derive(Serialize)]
struct SlackMessageReponse {
    response_type: String,
    text: String,
}

#[post("/api/rand")]
fn rand(data: web::Form<SlackFormData>) -> Result<HttpResponse, errors::UserError> {
    let options: Vec<String> = data.text.split_whitespace().map(String::from).collect();

    let mut cmd: &str;
    let mut opts: Vec<String> = [].to_vec(); //vec!([]);
    if options.len() == 0 {
        cmd = commands::Command::Help.as_str();
    } else {
        cmd = options[0].as_str();
        opts = options[1..].to_vec();
    }
    let cmd = match cmd {
        "help" => commands::Command::Help,
        "num" => commands::Command::Number,
        "choice" => commands::Command::Choice,
        _ => commands::Command::Help,
    };
    let response = cmd.handle(&opts).map_err(|_e| errors::UserError::InputError)?;
    Ok(HttpResponse::Ok().json(SlackMessageReponse {
        response_type: String::from("in_channel"),
        text: format!("{}", response),
    }))
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .service(index)
            .service(slack_auth_redirect)
            .service(rand)
    });
    let port = env::var("PORT")
       .unwrap_or_else(|_| "3737".to_string())
       .parse()
       .expect("PORT must be a number");

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(("0.0.0.0", port)).expect("Can not bind to port")
    };

    server.run().unwrap();
}
