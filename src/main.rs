use codec::Encode;
use futures::StreamExt;
use serenity::async_trait;
use serenity::http::Http;
use serenity::model::channel::{Channel, Message};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use subxt::ext::sp_runtime::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "./gm_metadata.scale")]
pub mod gm {}

use gm::runtime_types::gm_chain_runtime::Coooooins;
use gm::runtime_types::pallet_identity::types::Data;
use gm::runtime_types::pallet_identity::types::Judgement;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn get_discord_user_data(
    api: &OnlineClient<PolkadotConfig>,
    http: &Http,
    account: &AccountId32,
) -> Option<(String, String, u64)> {
    let key = gm::storage().identity().identity_of(account);

    if let Some(a) = api.storage().fetch(&key, None).await.unwrap() {
        if a.judgements.0.into_iter().any(|(_, j)| {
            j.encode() == Judgement::<u128>::KnownGood.encode()
                || j.encode() == Judgement::<u128>::Reasonable.encode()
        }) {
            let discord = a.info.additional.0;

            let d = &discord
                .iter()
                .find(|(l, _)| l.encode() == Data::Raw7(*b"discord").encode())
                .unwrap()
                .1;

            let discord_id = match d {
                Data::Raw1(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw2(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw3(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw4(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw5(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw6(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw7(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw8(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw9(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw10(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw11(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw12(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw13(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw14(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw15(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw16(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw17(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw18(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw19(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw20(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw21(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw22(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw23(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw24(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw25(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw26(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw27(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw28(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw29(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw30(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw31(i) => String::from_utf8(i.to_vec()).unwrap(),
                Data::Raw32(i) => String::from_utf8(i.to_vec()).unwrap(),

                _ => String::new(),
            };

            let (discord_name, discord_discriminator) = discord_id.split_once('#').unwrap();
            let discord_name = discord_name.trim_start_matches('@');

            let users = http
                .get_guild_members(978703103782174750, None, None)
                .await
                .unwrap();

            if let Some(user) = users.iter().find(|member| {
                member.user.name == discord_name
                    && member.user.discriminator == discord_discriminator.parse::<u16>().unwrap()
            }) {
                let user_avatar = user
                    .avatar_url()
                    .unwrap_or_else(|| user.user.avatar_url().unwrap());
                let user_name = user.display_name();
                let user_id = user.user.id.0;

                Some((user_name.to_string(), user_avatar, user_id))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = dotenv::var("DISCORD_TOKEN").unwrap();
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let api = OnlineClient::<PolkadotConfig>::
   // from_url("wss://kusama.gmordie.com:443")
    new()
    .await
    .unwrap();

    // Subscribe to several balance related events. If we ask for more than one event,
    // we'll be given a correpsonding tuple of `Option`'s, with exactly one
    // variant populated each time.
    let mut balance_events = api
        .events()
        .subscribe()
        .await
        .unwrap()
        .filter_events::<(gm::tokens::events::Transfer,)>();

    let http = client.cache_and_http.http.clone();

    // Our subscription will see all of the balance events we're filtering on:
    while let Some(ev) = balance_events.next().await {
        let event_details = ev.unwrap();

        let event = event_details.event;

        if let gm::tokens::events::Transfer {
            currency_id: coin @ (Coooooins::GM | Coooooins::GN),
            from,
            to,
            ..
        } = event
        {
            if let Some((_, _, receiver_id)) = get_discord_user_data(&api, &http, &to).await {
                let discord_data_sender = get_discord_user_data(&api, &http, &from).await;

                if let Channel::Guild(channel) =
                    &http.get_channel(1009449052548644864).await.unwrap()
                {
                    channel
                    .send_message(&http, |m| {
                        m.content(format!("<@!{}>", receiver_id))
                            .add_embed(|embed| {
                            embed
                                .author(|a|
                                        if let Some((name, avatar, _)) = discord_data_sender {
                                        a.name(name).icon_url(avatar)
                                        } else {a.name(from)})
                                .description(format!("<@!{}>", receiver_id))
                                .color(serenity::utils::Color::new(match coin {
                                    Coooooins::GM => 14778951,
                                    Coooooins::GN => 8598667,
                                    _ => 0,
                                }))
                                .title(format!(
                                    "sent a {} to",
                                    match coin {
                                        Coooooins::GM => "<:GM:978736456887595058>",
                                        Coooooins::GN => "<:GN:978736456686239767>",
                                        _ => "",
                                    }
                                )).thumbnail(match coin {
                                    Coooooins::GM => "https://media.discordapp.net/attachments/620330473876357123/1009520005941043210/unknown.png",
                                    Coooooins::GN => "https://media.discordapp.net/attachments/620330473876357123/1009519970968948786/unknown.png",
                                    _ => ""
                                })
                        }).allowed_mentions(|am| am.parse(serenity::builder::ParseValue::Users))
                    })
                    .await
                    .unwrap();
                }
            }
        }
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
