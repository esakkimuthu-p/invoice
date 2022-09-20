use actix_web::{get, web, App, HttpResponse, HttpServer};
use chrono::NaiveDate;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, DateTime, Document},
    options::{ClientOptions, FindOptions},
    Client, Collection, Database,
};

// pub mod date;
pub mod error;
pub mod model;

// use date::Date;
use error::{Error, Result};
use model::{
    Invoice, Organization, OrganizationPricing, OrganizationPricingTier, OrganizationUsage,
};

#[get("/generate_invoice")]
pub async fn generate_invoice(db: web::Data<Database>) -> Result<HttpResponse> {
    let organizations = Organization::collection(&db)
        .find(doc! {}, None)
        .await?
        .try_collect::<Vec<Organization>>()
        .await?;
    for organization in organizations {
        let find_opts = FindOptions::builder()
            .sort(doc! {"date": -1})
            .limit(1)
            .build();
        let mut invoices = Invoice::collection(&db)
            .find(doc! {"name": organization.clone().name}, find_opts)
            .await?
            .try_collect::<Vec<Invoice>>()
            .await?;
        let mut from_date = organization.clone().book_begin;
        let to_date = DateTime::now();
        for invoice in invoices.iter_mut() {
            if invoice.draft && invoice.total_value > 0.0 {
                invoice.draft = false;
            }
            from_date = invoice.date;
            Invoice::collection(&db)
                .replace_one(doc! {"_id": invoice.id}, invoice, None)
                .await?;
        }
        make_invoice(&db, organization, from_date, to_date).await?;
    }
    Ok(HttpResponse::Ok().json(organizations))
}

async fn make_invoice(
    db: &Database,
    organization: Organization,
    from_date: DateTime,
    to_date: DateTime,
) -> Result<()> {
    let tax_ratio = 18;
    if from_date > to_date {
        let organization_usage = calculate_usage(from_date, to_date, organization.pricing).await;
    }
    Ok(())
}

async fn calculate_usage(
    from_date: DateTime,
    to_date: DateTime,
    pricing: OrganizationPricingTier,
) -> Vec<OrganizationUsage> {
    let start_date = from_date;
    let end_date = to_date;
    while (end_date > start_date)
        || (start_date.to_chrono().format("%m").to_string()
            == end_date.to_chrono().format("%m").to_string())
    {
        let total_days = from_date.to_chrono();
    }
    vec![OrganizationUsage {
        billing_period: "hello".to_string(),
        plan: OrganizationPricingTier::Free.to_string(),
        base_charge: 999.0,
        additional_usage_charges: 555.0,
    }]
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
    let _server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(generate_invoice)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    println!("Server started successfully on port {}", 8080);
    Ok(())
}
