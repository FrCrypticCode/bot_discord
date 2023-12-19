use crate::serenity::all::*;
use crate::commands;
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
pub struct App<'a>{
    pub servers:Vec<GuildId>,
    pub forbid:Vec<&'a str>,
    pub logs:String
}
#[async_trait]
impl EventHandler for App<'static>{
    // Restreindre la lecture uniquement sur les channels serveurs
    async fn message(&self,ctx:Context,msg:Message){
        if !msg.author.bot && msg.channel_id != ChannelId::from(*****){    // Vérification de l'auteur (Bot + Localisation msg)
            let log = String::from(msg.author.to_string())+" : "+msg.content.as_str();
            if let Err(err) = ChannelId::new(*****).send_message(&ctx.http, CreateMessage::new().content(log)).await{
                self.write_err(err.to_string()).await;
            }
            for nword in self.forbid.clone(){   // Contrôle du contenu
                if msg.content.contains(nword){
                    match msg.channel_id.delete_message(&ctx.http, msg.id).await{
                        Ok(_x)=>{
                            if let Ok(chan) = msg.author.create_dm_channel(&ctx.http).await{
                                println!("Chan Ok");
                                let message = String::from("Avertissement : Emploi d'un langage non autorisé.");
                                if let Err(err) = chan.say(&ctx, message).await{
                                    self.write_err(err.to_string()).await;
                                }
                            }
                        },
                        Err(err)=>{
                            self.write_err(err.to_string()).await;
                        }
                    }
                }
            }
        }
        
    }

    async fn interaction_create(&self, ctx:Context, int:Interaction){
        let req = int.as_command();
        if req.is_some(){
            match req.clone().unwrap().data.name.as_str(){
                "msg"=>{
                    let opts = &req.unwrap().data.options;
                    let (target,msg) =(opts[0].value.clone(),opts[1].value.clone());
                    println!("{target:?} : {msg:?}");
                    if let Err(err) = commands::msg::send(&ctx,&req.unwrap().user.id.clone(), msg.as_str(), target.as_user_id()).await{
                        self.write_err(err).await;
                    }
                },
                _=>{}
            }
        }
    }

    // Gestion de l'arrivée d'un nouveau membre sur le serveur
    async fn guild_member_addition(&self, ctx:Context, new:Member){
        // Mettre le message sur le main chan du serveur en question
        match new.guild_id.channels(&ctx.http).await{
            Ok(chans)=>{    // Recherche du channel d'accueil => Par défaut le plus squatté
                let mut amount = 0;
                let mut chan_look =ChannelId::new(0);
                for ch in chans{
                    let (id,name) = ch;
                    if name.member_count.is_some(){
                        if name.member_count.unwrap()>amount{
                            chan_look = id;
                            amount = name.member_count.unwrap();
                        }
                    }
                }
                if let Ok(chan_write) = ctx.http.get_channel(chan_look).await{  // Ecriture du message d'accueil
                    if let Err(err) = chan_write.id().say(&ctx, format!("{} rejoint l'équipage ! Bienvenue à lui !",new.user.name)).await{
                        self.write_err(err.to_string()).await;
                    }
                }
            },
            Err(err)=>{
                self.write_err(err.to_string()).await;
            }
        }

        // Notification sur le chan Logs du Serveur Principal
        match ctx.http.get_channel(ChannelId::from(*****)).await{
            Ok(x)=>{
                let msg = String::from(new.user.name);
                if let Err(err) = x.id().say(ctx.http, format!("{} a rejoint l'équipage du Serveur {}",msg,new.guild_id.name(&ctx.cache).unwrap())).await{
                    self.write_err(err.to_string()).await;
                }
            },
            Err(err)=>{
                self.write_err(err.to_string()).await;
            }
        }
    }

    // Gestion du départ d'un membre du serveur -> Notification sur le canal Logs du Serveur Principal
    async fn guild_member_removal(&self, ctx:Context, gid:GuildId, user:User, _member_data:Option<Member>){
        match ctx.http.get_channel(ChannelId::from(*****)).await{
            Ok(x)=>{
                if user.global_name.is_some(){
                    let _ = x.id().say(&ctx, format!("{} connu sous le nom de {} a quitté le serveur {}",user.global_name.unwrap(),user.name,gid.name(&ctx.cache).unwrap())).await;
                }
                else{
                    let _ = x.id().say(&ctx, format!("{} a quitté le serveur {}",user.name,gid.name(&ctx.cache).unwrap())).await;
                }
                
            },
            Err(err)=>{
                self.write_err(err.to_string()).await;
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot actif en tant que {}", ready.user.name);
        let api = Arc::new(Mutex::new(ctx));
        let guilds = Arc::new(Mutex::new(ready.guilds));
        let cmds = GuildId::new(*****);
        
        let t1= tokio::spawn(verify_guild(api.clone(),guilds.clone()));
        let t2= tokio::spawn(get_members(api.clone(),guilds.clone()));

        if let Err(err1) = t1.await{
            self.write_err(err1.to_string()).await;
        }
        if let Err(err2) = t2.await{
            self.write_err(err2.to_string()).await;
        }

        // Concevoir l'initialisation des commandes -> Voir Commands.rs, réfléchir à la structure fichiers
        let ctx = api.lock().await;
        let _ = cmds.set_commands(ctx.http(), vec![
            commands::msg::register()
        ]).await;
    }
}
impl App<'static>{
    async fn write_err(&self,err:String){
        if let Ok(mut f) = OpenOptions::new().append(true).open(&self.logs).await{
            let _ = f.write_all(err.as_bytes());
        }
    }
}
// Fonctions de référencement en BDD lors du Ready
//---------------------------------------------------
async fn verify_guild(_ctx:Arc<Mutex<Context>>,guilds:Arc<Mutex<Vec<UnavailableGuild>>>){
    let mut ids:Vec<GuildId> = vec![];
    {   // Extraction des ID
        let guilds = guilds.lock().await;
        for g in guilds.clone(){
            if g.unavailable{
                ids.push(g.id);
            }
        }
    }   // Libération de la donnée
    // Appel vers bdd pour check-up
}

async fn get_members(ctx:Arc<Mutex<Context>>,guilds:Arc<Mutex<Vec<UnavailableGuild>>>){
    let mut ids:Vec<GuildId> = vec![];
    {   // Extraction des id
        let guilds = guilds.lock().await;
        for g in guilds.clone(){
            ids.push(g.id);
        }
    }   // Libération de la donnée

    // Déclaration des Variables d'extraction
    let mut errs:Vec<String> = vec![];
    let mut members:Vec<(Member,GuildId)> = vec![]; 
    
    {   // Extraction des listes de membres
        let ctx = ctx.lock().await;
        for id in ids{
            match ctx.http.get_guild_members(id,Some(255),Some(0)).await{
                Ok(x)=>{
                    for m in x{
                        members.push((m,id))
                    }
                },
                Err(err)=>{errs.push(err.to_string())}
            }
        }
    }   // Libération de la donnée
    // Appel vers bdd pour check-up
}