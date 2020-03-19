extern crate heroku_rs;

use heroku_rs::endpoints::apps;
use heroku_rs::framework::{
    apiclient::HerokuApiClient,
    auth::Credentials,
    response::{ApiResponse, ApiResult},
    ApiEnvironment, HttpApiClient, HttpApiClientConfig,
};

use serde::Deserialize;
use serde_json::Value;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::config::Config;

#[derive(Debug, Deserialize)]
struct HerokuApp {
    id: String,
    name: String,
    released_at: String,
    web_url: String,
}

// Get app by name or id
#[command]
#[num_args(1)]
pub fn get_app(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let config = bot_config(ctx);
    let api_client = heroku_client(&config.heroku_api_key);

    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let response = api_client.request(&apps::AppDetails { identifier: app_name });
    println!("response {:?}", response);

    msg.reply(
        ctx,
        match response {
            Ok(app) => {
                app_response(app)
            },
            Err(e) => {
                println!("Error: {}", e);
                "An error occured when fetching your Heroku app".into()
            }
        },
    )?;

    Ok(())
}

#[command]
pub fn get_apps(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let config = bot_config(ctx);
    let api_client = heroku_client(&config.heroku_api_key);
    let response = api_client.request(&apps::AppList {});

    print_response(response);

//    let response = heroku_client(&config.heroku_api_key)
//        .get()
//        .apps()
//        .execute::<Vec<HerokuApp>>();

//    msg.reply(
//        ctx,
//        match response {
//            Ok((_, _, Some(apps))) => apps_response(apps),
//            Ok((_, _, None)) => "You have no Heroku apps".into(),
//            Err(err) => {
//                println!("Err {}", err);
//                "An error occured while fetching your Heroku apps".into()
//            }
//        },
 //   )?;

    Ok(())
}

#[command]
#[num_args(1)]
pub fn restart_app(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let config = bot_config(ctx);

    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

/**
    let response = heroku_client(&config.heroku_api_key)
        .delete_empty()
        .apps()
        .app_name(&app_name)
        .app_dynos()
        .execute::<Value>();

    msg.reply(
        ctx,
        match response {
            Ok((_, _, Some(_object))) => format!("All dynos in {} have been restarted.", app_name),
            Ok((_, _, None)) => "There is no Heroku app by that name".into(),
            Err(err) => {
                println!("Err {}", err);
                "An error occured while fetching your Heroku app".into()
            }
        },
    )?;
**/

    Ok(())
}

fn heroku_client(api_key: &str) -> HttpApiClient {
    let credentials: Credentials =
        Credentials::UserAuthToken {
            token: api_key.to_string(),
        };

    HttpApiClient::new(
        credentials,
        HttpApiClientConfig::default(),
        ApiEnvironment::Production,
    ).unwrap()
}

fn app_response(app: heroku_rs::endpoints::apps::App) -> String {
    format!(
        "\nApp ID: {}\nApp Name: {}\nReleased At: {}\nWeb URL: {}\n\n",
        app.id, app.name, app.released_at.unwrap(), app.web_url
    )
}

/**
fn apps_response(processed_app_list: Vec<HerokuApp>) -> String {
    let mut list = String::from("Here are your Heroku apps\n");

    for app in processed_app_list {
        let app_info = app_response(app);
        list.push_str(&app_info);
    }

    list
}
**/

fn bot_config(ctx: &Context) -> std::sync::Arc<Config> {
    ctx.data
        .read()
        .get::<Config>()
        .expect("Expected config")
        .clone()
}

fn print_response<T: ApiResult>(response: ApiResponse<T>) {
    match response {
        Ok(success) => println!("Success: {:#?}", success),
        Err(e) => println!("Error: {}", e),
    }
}