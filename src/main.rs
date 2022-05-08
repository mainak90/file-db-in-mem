mod db_access;

extern crate actix_web;

use actix_web::{web, web::Path, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_derive::Deserialize;
//use std::sync::Mutex;

fn invalid_resource(req: HttpRequest) -> impl Responder {
    println!("Invalid URI: \"{}\"", req.uri());
    HttpResponse::NotFound()
}

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8091";
    println!("Listening at addrress {}", addr);

    HttpServer::new(move || {
        App::new()
            //.register_data()
            .service(web::resource("/persons/ids").route(web::get().to(get_all_persons_ids)))
            .service(web::resource("/person/name_by_id/{id}").route(web::get().to(get_person_name_by_id)))
            .service(web::resource("/persons").route(web::get().to(get_persons)))
            .service(web::resource("/person/{name}").route(web::post().to(insert_person)))
            .default_service(web::route().to(invalid_resource))
    })
        .bind(&addr)?
        .run()
}
