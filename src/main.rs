use serenity::{self, Client, all::GatewayIntents};
use tokio::fs::File;
mod bot;
use bot::App;
mod commands;


#[tokio::main]
async fn main() {
    let token = String::from("*****");
    let app = App{
        servers:vec![], // Problème de mutabilité
        forbid:vec!["pute ","fdp","salop","nique","ntm","conn","con ","con\n","conna","batard","pd","sale chien","enculé"],
        logs:init_logs().await
    };
    let bot = Client::builder(token,GatewayIntents::all())
        .event_handler(app)
        .await;
    match bot{
        Ok(mut x)=>{
            if let Err(err) = x.start().await{
                println!("Erreur : {}",err)
            }
        },
        Err(err)=>{println!("Erreur : {}",err)}
    }
}

async fn init_logs()->String{
    if tokio::fs::try_exists("Logs.ctx").await.unwrap(){
        File::open("Logs.ctx").await.unwrap();
    }
    else{
        File::create("Logs.ctx").await.unwrap();
    }
    return String::from("Logs.ctx")
    
}