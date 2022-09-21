use chrono::{TimeZone, Utc};
use mongodb::bson::doc;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::env;
use std::error::Error;
use std::time::Instant;

use tokio;
//use async_std;

// #[async_std::main]
#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>> {
    // set environment variable:
    env::set_var("MONGODB_URI", "mongodb+srv://aunjum:u9t8vOivNEzL6I4y@cluster0.nmuah3o.mongodb.net/?retryWrites=true&w=majority");

    // Load the mongodb connection string from the environment variable:
    let client_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    // A client is needed to work connect to MongoDB
    // An extra line of code to work around a DNS issue on windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    // print the database in our mongodb cluster:
    println!("Databases:");
    let start = Instant::now();
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    let duration = start.elapsed();
    println!("Time elapsed in get database list is: {:?}", duration);

    //define new document model:
    let new_doc = doc! {
        "title": "Parasite",
        "year": 2020,
        "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks.
        But their easy life gets complicated when their deception is threatened with exposure.",
        "released": Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),
    };
    println!("Defined new doc: {:?}", new_doc);

    //insert new document:
    let movies = client.database("sample_mflix").collection("movies");
    let insert_result = movies.insert_one(new_doc.clone(), None).await?;
    println!("Inserted document: {:?}", insert_result);

    // Look for the document we just inserted:
    let movie = movies
        .find_one(Some(doc! { "title": "Parasite" }), None)
        .await?
        .expect("Missing 'Parasite' document");
    println!("Found document: {:?}", movie);

    //update the document:
    let update_result = movies
        .update_one(
            doc! { "_id": &movie.get("_id") },
            doc! { "$set": { "year": 2018 } },
            None,
        )
        .await?;
    println!("Updated document: {:?}", update_result.modified_count);

    let movie = movies
        .find_one(Some(doc! { "_id": &movie.get("_id") }), None)
        .await?
        .expect("Missing 'Parasite' document");
    println!("Updated Movie: {:?}", &movie);

    //Delete all documents for movies called "Parasite":
    let delete_result = movies
        .delete_many(doc! { "title": "Parasite" }, None)
        .await?;
    println!("Deleted documents: {:?}", delete_result.deleted_count);


    Ok(())
}
