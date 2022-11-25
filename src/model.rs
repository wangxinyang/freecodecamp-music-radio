use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeRadioMessage {
    pub station: Station,
    pub is_online: bool,
    pub cache: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Station {
    pub id: i64,
    pub name: String,
    pub shortcode: String,
    pub description: String,
    pub frontend: String,
    pub backend: String,
    pub listen_url: String,
    pub url: String,
    pub public_player_url: String,
    pub playlist_pls_url: String,
    pub playlist_m3u_url: String,
    pub is_public: bool,
    pub mounts: Vec<Mount>,
    pub remotes: Vec<Remote>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mount {
    pub path: String,
    pub is_default: bool,
    pub id: i64,
    pub name: String,
    pub url: String,
    pub bitrate: i64,
    pub format: String,
    pub listeners: Listeners,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Remote {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub bitrate: i64,
    pub format: String,
    pub listeners: Listeners,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Listeners {
    pub total: i64,
    pub unique: i64,
    pub current: i64,
}
