// MSG Command
pub mod msg{
    use serenity::{builder::{CreateCommand, CreateCommandOption}, all::{Context,CommandOptionType,UserId}};
    pub fn register()->CreateCommand{
        CreateCommand::new("msg")
            .description("Envoyer un message à un utilisateur")
            .add_option(CreateCommandOption::new(CommandOptionType::User,"user","Cible"))
            .add_option(CreateCommandOption::new(CommandOptionType::String,"message","Message à expédier"))
    }
    pub async fn send(ctx:&Context,sender:&UserId,msg:Option<&str>,target:Option<UserId>)->Result<(),String>{
        if msg.is_some() && target.is_some(){
            match target.unwrap().create_dm_channel(ctx).await{
                Ok(chan)=>{
                    if let Ok(s) = ctx.http.get_user(*sender).await{
                        let _ = chan.say(ctx, format!("{} vous dit : {}",s.name,msg.unwrap())).await;
                        return Ok(())
                    }
                    else{
                        return Err(String::from("Une erreur est survenue. User introuvable."))
                    }
                    
                },
                Err(err)=>{return Err(err.to_string())}
            }
        }
        else{
            Err(String::from("Un des champs est vide."))
        }
    }
}
