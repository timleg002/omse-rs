use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub instances: Vec<ObsInstance>
}

#[derive(Deserialize, Debug)]
pub struct ObsInstance {
    pub host: String,
    pub port: u16,
    pub vlc_sources: Vec<VLCSource>
}

#[derive(Deserialize, Debug)]
pub struct VLCSource {
    pub name: String,
    pub scene_name: String,
    pub watchdog_url: String,
    pub sched: String
}

/*
Table(
    {
        "instances": Array(
            [
                Table(
                    {
                        "host": String(
                            "localhost",
                        ),
                        "port": String(
                            "4444",
                        ),
                        "vlc_sources": Table(
                            {
                                "OMSA TA": Table(
                                    {
                                        "watchdog_url": String(
                                            "http://example.com/kostol?kostol=ta",
                                        ),
                                    },
                                ),
                                "OMSA TS": Table(
                                    {
                                        "watchdog_url": String(
                                            "http://example.com/kostol?kostol=ts",
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                ),
                Table(
                    {
                        "host": String(
                            "localhost",
                        ),
                        "port": String(
                            "4455",
                        ),
                    },
                ),
            ],
        ),
    },
)
 */