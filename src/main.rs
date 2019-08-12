use actix_web::{App, web, get, post, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};

mod errors;
mod random;

#[get("/")]
fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[derive(Deserialize)]
#[derive(Debug)]
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
fn random_number(data: web::Form<SlackFormData>) -> Result<HttpResponse, errors::UserError> {
    let numbers: Vec<&str> = data.text.split(' ').collect();
    let (low, high): (i64, i64) =
        if numbers.len() < 2 {
            (0, 10)
        } else {
            (numbers[0].parse().map_err(|_e| errors::UserError::InputError)?,
             numbers[1].parse().map_err(|_e| errors::UserError::InputError)?)
        };

    let rand_number = random::gen_random_range(low, high + 1)
        .map_err(|_e| errors::UserError::InputError)?;
    Ok(HttpResponse::Ok().json(SlackMessageReponse {
        response_type: String::from("in_channel"),
        text: format!("*{}*", rand_number),
    }))
}

#[post("/api/rand_choice")]
fn random_choice(data: web::Form<SlackFormData>) -> Result<HttpResponse, errors::UserError> {
    let options: Vec<String> = data.text.split_whitespace().map(String::from).collect();
    let rand_choice = random::select_random(&options)
        .map_err(|_e| errors::UserError::InputError)?;

    Ok(HttpResponse::Ok().json(SlackMessageReponse {
        response_type: String::from("in_channel"),
        text: format!("*{}*", rand_choice),
    }))
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .service(index)
            .service(random_number)
            .service(random_choice)
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:3737").unwrap()
    };

    server.run().unwrap();
}
