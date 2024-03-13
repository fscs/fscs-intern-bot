use poise::serenity_prelude::{
    self as serenity, ChannelType, CreateMessage, CreateThread, EditMessage, EditThread,
    GetMessages,
};
use poise::Modal;
type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;

mod rest;
mod structs;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![antrag(), edit()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

#[derive(Debug, Modal, Default)]
#[name = "Top Erstellen"]
struct CreateTopModal {
    #[name = "Top Name"]
    #[placeholder = ""]
    name: String,
    #[name = "Antragstext"]
    #[paragraph]
    #[placeholder = "Der Fachschaftsrat Informatik möge beschließen, dass:"]
    antragstext: Option<String>,
    #[name = "Begründung"]
    #[paragraph]
    begründung: Option<String>,
}

#[derive(Debug, Modal, Default)]
#[name = "Top Editieren"]
struct EditTopModal {
    #[name = "Top Name"]
    #[placeholder = ""]
    name: String,
    #[name = "Antragstext"]
    #[placeholder = "Der Fachschaftsrat Informatik möge beschließen, dass:"]
    #[paragraph]
    antragstext: Option<String>,
    #[name = "Begründung"]
    begründung: Option<String>,
}

#[poise::command(slash_command)]
pub async fn antrag(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let top = CreateTopModal::execute_with_defaults(
        ctx,
        CreateTopModal {
            antragstext: Some("Der Fachschaftsrat Informatik möge beschließen, dass:".to_string()),
            ..Default::default()
        },
    )
    .await?
    .unwrap();

    let name = top.name;
    let antragstext = &top
        .antragstext
        .unwrap_or_else(|| "Keine Beschreibung".to_string());

    let begruendung = String::from("Begründung: \r")
        + &top
            .begründung
            .unwrap_or_else(|| "Keine Begründung".to_string());

    let channel_id = ctx.interaction.channel_id;

    let builder = CreateMessage::new()
        .content(name.clone() + " - " + &ctx.author().name.clone())
        .tts(false);
    let message = channel_id.send_message(&ctx.http(), builder).await;

    let builder = CreateThread::new(&name);
    let thread = channel_id
        .create_thread_from_message(&ctx.http(), message.unwrap().id, builder)
        .await
        .unwrap();

    let builder = CreateMessage::new()
        .content(antragstext.to_string())
        .tts(true);
    thread.clone().id.send_message(&ctx.http(), builder).await?;

    let builder = CreateMessage::new().content(&begruendung).tts(true);
    thread.id.send_message(&ctx.http(), builder).await?;

    let antrag = structs::Antrag {
        titel: name,
        antragstext: antragstext.to_string(),
        begründung: begruendung,
        antragssteller: ctx.author().name.clone(),
    };

    let resp = rest::create_antrag(antrag).await;
    if resp == "Noch keine Sitzung geplant" {
        return Err("Noch keine Sitzung geplant".into());
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn edit(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let mut channel = ctx.guild_channel().await.unwrap();

    if channel.kind != ChannelType::PublicThread {
        return Err("This command can only be used in a thread".into());
    }

    //get the messageid of the oldest two messages in the channel
    let gm = GetMessages::new();
    let mut messages = channel.id.messages(&ctx.http(), gm).await?;

    //invert messages
    let mut messages: Vec<_> = messages.drain(..).rev().collect();

    //create modal with the name of the thread
    let modal = EditTopModal::execute_with_defaults(
        ctx,
        EditTopModal {
            name: channel.clone().name,
            antragstext: Some(messages[1].content.to_string()),
            begründung: Some(messages[2].content.replace("Begründung: \r", "")),
        },
    )
    .await?
    .unwrap();

    let name = modal.name;
    let antragstext = &modal
        .antragstext
        .unwrap_or_else(|| "Keine Beschreibung".to_string());

    let begruendung = String::from("Begründung: \r")
        + &modal
            .begründung
            .unwrap_or_else(|| "Keine Begründung".to_string());

    //edit thread title
    let editthread = EditThread::new().name(&name);
    channel.edit_thread(&ctx.http(), editthread).await?;

    //edit the messages
    let builder = EditMessage::new().content(antragstext.to_string());
    messages[1].edit(&ctx.http(), builder).await?;

    let builder = EditMessage::new().content(&begruendung);
    messages[2].edit(&ctx.http(), builder).await?;

    //get the message that startet the thread
    let message = channel.id.message(&ctx.http(), messages[0].id).await?;
    let messagetype = message.kind;

    //if the message is a thread starter message, edit the content

    if messagetype == serenity::model::channel::MessageType::ThreadStarterMessage {
        let threadid = channel.id;
        let parentchannel = channel.parent_id.unwrap();
        let mut parentmessage = parentchannel.message(&ctx.http(), threadid.get()).await?;
        let builder = EditMessage::new().content(name.clone() + " - " + &ctx.author().name.clone());
        parentmessage.edit(&ctx.http(), builder).await?;
    }

    //TODO: maybe antragssteller should not be overritten
    let antrag = structs::Antrag {
        titel: name,
        antragstext: antragstext.to_string(),
        begründung: begruendung,
        antragssteller: ctx.author().name.clone(),
    };

    rest::edit_antrag(antrag);

    Ok(())
}
