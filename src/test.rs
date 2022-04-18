#[cfg(test)] 
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::{watchdog::check_url, config::{VLCSource, ObsInstance}, obs::setup_obs};
    use anyhow::Result;
    use obws::{Client, requests::{SourceSettings, custom::source_settings::{VlcSource, SlideshowFile}, CreateSource}};

    #[tokio::test]
    async fn check_for_url() -> Result<()> {
        let url = "https://www.3omedia.sk/ta.php".to_string();

        if let Err(err) = check_url(&url).await {
            Err(err)
        } else {
            Ok(())
        }
    }

    /// Needs an OBS websocket server running at localhost:4444
    #[tokio::test]
    async fn obs_test() -> Result<()> {
        let client = Client::connect("localhost", 4444).await?;

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

        const SCENE_NAME: &str = "OMSE-RS TEST SCENE"; 
        const VLC_SOURCE_NAME: &str = "VLC TEST SRC";

        client.scenes().create_scene(SCENE_NAME).await?;

        let create_source = CreateSource {
            source_name: VLC_SOURCE_NAME,
            source_kind: "vlc_source",
            scene_name: SCENE_NAME,
            ..Default::default()
        };

        client.sources().create_source(create_source).await?;

        // i dont know why it wants to return something, but we gotta make do
        client.sources().set_source_settings::<VlcSource, VlcSource>(settings.clone()).await?;

        //     
        //   Checking if it's been successfully applied  
        //   

        // TODO

        Ok(())
    }

    #[tokio::test]
    async fn obs_client_mult_connections_test() -> Result<()> {
        setup_obs(
            vec![
                ObsInstance {
                    host: "localhost".to_string(),
                    port: 4444,
                    vlc_sources: vec![]
                },
                ObsInstance {
                    host: "localhost".to_string(),
                    port: 4444,
                    vlc_sources: vec![]
                }
            ]
        ).await?;



        Ok(())
    }

    #[test]
    fn parse_sched() {
        const SCHED_STR: &str = "12:00, 17:00";
        let sched_parsed: Vec<&str> = SCHED_STR
            .split(", ")
            .collect();
        assert_eq!(
            sched_parsed,
            vec!["12:00", "17:00"]
        );
    }
}