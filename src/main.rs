use mongodb::bson::doc;
use mongodb::error::Result;

mod db_client;

#[tokio::main]
async fn main() -> Result<()> {    
    // const COLLECTION: &str = "cats";
    // const DB: &str = "animals";

    // let result = db_client::insert_one("cats", doc! {"name": "Olivia", "nation": "Peru"}, "animals").await?;

    // let docs = db_client::find_many("cats", doc! {}, "animals").await?;
    // println!("Found docs: {:?}", docs);

    // let result = db_client::list_databases().await?;
    // for db in result {
    //     println!("{}", db);
    // }

    // let result = db_client::fetch_collections(DB).await?;
    // for coll in result {
    //     println!("{}", coll);
    // }

    // let result = db_client::fetch_collection_fields(DB, COLLECTION).await?;
    // for field in result {
    //     println!("{}", field);
    // }

    // let result = db_client::update_one(COLLECTION, doc! {"name": "Black"}, doc! {"$set": {"nation": "USA"}}, DB).await?;
    // println!("{:?}", result);

    // let result = db_client::delete_many(COLLECTION, vec! [doc! {"name": "Olivia"}], DB).await?;
    // println!("{:?}", result);


    db_client::init_client().await?; 
    let docs = db_client::find_many("cats", doc! {}, "animals").await?;
    println!("Found docs: {:?}", docs);
    

    Ok(())
}

