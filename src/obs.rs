use std::{borrow::Borrow, path::PathBuf, str::FromStr};

use obws::{Client, requests::{SourceSettings, custom::source_settings::{VlcSource, SlideshowFile}}};

use anyhow::{anyhow, Result};

use crate::{config::ObsInstance};

/// Connects to all the clients specified in the instances & returns them.
pub async fn setup_obs(instances: Vec<ObsInstance>) -> Result<Vec<Client>> {
    let mut clients = vec![];

    for instance in instances {
        let client = Client
            ::connect(instance.host, instance.port)
            .await?;

        clients.push(client);    
    }
        
    Ok(clients)
}

/// Overwrites the VLC source playlist with the given URL
pub async fn change_vlc_source_url(client: &Client, source_name: &str, path: &str) -> Result<()> {
    let settings = SourceSettings {
        source_name,
        source_type: Some("vlc_source"), // Always "vlc_source".
        source_settings: &VlcSource {
            playlist: vec![SlideshowFile {
                value: PathBuf::from_str(path)?,
                ..Default::default()
            }],
            ..Default::default()
        }
    };

    // TODO: fix this weird deserializing crap
    client.sources().set_source_settings::<VlcSource, VlcSource>(settings).await?;

    Ok(())
}