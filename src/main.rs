#![feature(async_closure)]

extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod config;
mod obs;
mod watchdog;
mod test;
mod error;

use std::{thread, time::Duration};

use anyhow::{anyhow, Result};
use obws::Client;
use tokio::fs;
use clap::{Parser, Args};
use crate::config::Config;

/// Program used for controlling OBS VLC video sources
#[derive(Parser, Debug)]
#[clap(author, version, about)]
enum Omse {
    /// Trigger the change of the VLC video source manually
    Trigger(Trigger),
    /// Schedule the the triggering process as specified in config
    Schedule(Schedule)
}

#[derive(Args, Debug)]
struct Trigger {
    /// Name of the VLC source
    #[clap(short, long)]
    name: String,
    /// Won't restart the OBS recording or switch the scene, just changes the URLs
    #[clap(short, long)]
    change_only: bool,
    /// Optionally specify the config file path
    #[clap(default_value_t = String::from("config.toml"))]
    config: String,
    /// Optionally specify the URL of the source video, instead of getting one from the watchdog URL
    #[clap(short, long)]
    source_url: Option<String>
}

#[derive(Args, Debug)]
struct Schedule {
    /// Optionally specify the config file path
    #[clap(default_value_t = String::from("config.toml"))]
    config: String
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    match Omse::parse() {
        Omse::Trigger(trigger) => {
            let contents = fs::read_to_string(trigger.config).await?;
            let config: Config = toml::from_str(&contents)?;

            for instance in config.instances {
                for source in instance.vlc_sources {
                    // The first found VLC source by name is used, the others are ignored
                    if trigger.name == source.name {
                        let client = Client::connect(instance.host, instance.port).await?;

                        // If the user has specified a custom source URL
                        if let Some(source_url) = trigger.source_url {
                            obs::change_vlc_source_url(&client, &source.name, &source_url).await?;
                        } else {
                            match watchdog::check_url(&source.watchdog_url).await? {
                                Some(url) => {
                                    // If the user didn't specify an option to change only the source URL
                                    if !trigger.change_only {
                                        client.recording().stop_recording().await?;
                                        thread::sleep(Duration::from_secs(1));
                                        client.recording().start_recording().await?;
                                   
                                        client.scenes().set_current_scene(&source.scene_name).await?;
                                    }
    
                                    obs::change_vlc_source_url(&client, &source.name, &url).await?;
                                },
                                None => {
                                    return Err(anyhow!("No VLC source video URL available yet, please try again"))
                                },
                            };
                        }

                        return Ok(());
                    }
                }
            }
        },
        Omse::Schedule(schedule) => {
            println!("{schedule:#?}");

            info!("Started omse-rs scheduling");

        },
    }

    Ok(())

    /* 

    let client = Client::connect("localhost", 4444).await?;

    let scenes = client
        .scenes()
        .get_scene_list()
        .await?;

    let matching_scenes = scenes
        .scenes
        .iter()
        .filter(|scene| scene.name == "OMSA TA")
        .collect::<Vec<&Scene>>();

    // Get and print out version information of OBS and obs-websocket.
    let scene = matching_scenes.first().unwrap();
    println!("{:#?}", scene);

    let settings = SourceSettings {
        source_name: "vlc",
        source_type: Some("vlc_source"),
        source_settings: &VlcSource {
            playlist: vec![SlideshowFile {
                value: PathBuf::from_str("https://youtube.com/watch?v=00000")?,
                ..Default::default()
            }],
            ..Default::default()
        }
    };

    // client.sources().set_source_settings::<VlcSource, VlcSource>(settings).await?;
    
    let contents = fs::read_to_string("config.toml").await?;
    let toml: Config = toml::from_str(&contents).unwrap();

    println!("{:#?}",toml);

    */
}
