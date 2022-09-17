//! Example code for using MongoDB with Actix.

// mod model;
// #[cfg(test)]
// mod test;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use futures::TryStreamExt;
// use model::User;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client, Collection, Database,
};

const ORG_NAME: &str = "organizations";
// const COLL_NAME: &str = "users";

#[get("/generate_invoice")]
async fn generate_invoice(db: web::Data<Database>) -> Result<HttpResponse, String> {
    let orgs = db
        .collection::<Document>(ORG_NAME)
        .find(doc! {}, None)
        .await
        .map_err(|x| x.to_string())?
        .try_collect::<Vec<Document>>()
        .await
        .map_err(|x| x.to_string())?;
    println!("1");
    // match collection.find_one(doc! {}, None).await {
    //     Ok(Some(org)) => HttpResponse::Ok().json(org),
    //     Ok(None) => HttpResponse::NotFound().body(format!("No user found with username")),
    //     Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    // }
    Ok(HttpResponse::Ok().json(orgs))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_cluster = std::env::var("ORG_DB_CLUSTER").expect("ORG_DB_CLUSTER not set");

    let mut client_options = match ClientOptions::parse(&db_cluster).await {
        Ok(options) => options,
        Err(_) => panic!("Database connection failure"),
    };
    client_options.app_name = Some("auditplus".to_string());

    let client = Client::with_options(client_options).unwrap();
    let db = client.default_database().expect("Default database not set");
    println!("jjjj");
    let _server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            // .service(add_user)
            .service(generate_invoice)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    println!("Server started successfully on port {}", 8080);
    Ok(())
}
