use anyhow::Result;
use clokwerk::{Scheduler, TimeUnits, Interval::*};

use crate::config::Config;

pub async fn setup_watchdogs(config: Config) -> Result<Scheduler> {
    let mut scheduler = Scheduler::new();

    for instance in config.instances {
        for vlc_source in instance.vlc_sources {
            let schedule = vlc_source.sched.split(", ").collect::<Vec<&str>>();
            
            for time in schedule {
                info!(
                    "Setting up watchdog for: {}:{}, source {}, with schedule: {}",
                    instance.host,    
                    instance.port,     
                    vlc_source.name,  
                    time              
                );

                /* TODO TODO TODO
                scheduler
                    .every(1.day())
                    .at(time)
                    .run(async move || {
                        let url = check_url(vlc_source.watchdog_url).await.unwrap();
                    });
                */
            }

            scheduler.every(1.day());
        }
    }

    Ok(Scheduler::new())
}
 
/// This function checks the availability of a VLC-compatible link (such as YouTube)
/// at the given url parameter.
/// The function shouldn't panic, but we expect that the link may not be available,
/// therefore the return type is Result<Option<String>>
pub async fn check_url(url: &str) -> Result<Option<String>> {
    let res = reqwest::get(url).await?;

    if res.status() == 200 {
        return Ok(Some(res.text().await?.trim().to_string()));
    } else {
        return Ok(None)
    }
}