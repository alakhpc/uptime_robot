use serde::Deserialize;
use serde_repr::Deserialize_repr;
use strum::Display;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Stat {
    Ok,
    Fail,
}

#[derive(Deserialize_repr, Display, Debug, Clone)]
#[repr(u8)]
pub enum MonitorType {
    Http = 1,
    Keyword = 2,
    Ping = 3,
    Port = 4,
    Heartbeat = 5,
}

#[derive(Deserialize_repr, Display, Debug, Clone)]
#[repr(u8)]
pub enum Status {
    Paused = 0,
    NotCheckedYet = 1,
    Up = 2,
    SeemsDown = 8,
    Down = 9,
}
