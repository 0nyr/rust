#[macro_use]
extern crate diesel;
extern crate serde_json;
extern crate lettre;
extern crate native_tls;

// import modules to be used everywhere in the app
#[path = "params/vars.rs"] mod vars;
mod models;

// Actix-web is an async framework, so our main has to be asynchronous.
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_cors::Cors;
    use actix_files::Files;
    use actix_session::CookieSession;
    use actix_web::{middleware, web, App, HttpServer, http::header};
    use diesel::{
        prelude::*, 
        r2d2::{self, ConnectionManager}
    };
    
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init(); 
    
    // create a database connection pool
    let manager = ConnectionManager::
        <PgConnection>::new(vars::database_url());
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create a database connection pool.");    
        
    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            // Enable sessions
            .wrap(
                CookieSession::signed(&[0; 32])
                    .domain(vars::domain_url().as_str())
                    .name("auth")
                    .secure(false))
            .wrap(
                Cors::new()
                    .allowed_origin("*")
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .max_age(3600)
                    .finish())
            .service(Files::new("/assets", "./templates/assets"))
    })
    .bind(format!("{}:{}", vars::domain(), vars::port()))?
    .run()
    .await
}