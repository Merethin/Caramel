use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Deserialize)]
pub struct WaMemberRoot {
    #[serde(rename = "MEMBERS")]
    pub members: String,
}

#[derive(Deserialize)]
pub struct RmbRoot {
    #[serde(rename = "MESSAGES")]
    pub messages: Messages,
}

#[derive(Deserialize, Debug)]
pub struct Messages {
    #[serde(rename = "POST")]
    pub posts: Vec<Post>,
}

#[repr(u8)]
#[derive(Deserialize_repr, Debug)]
pub enum PostStatus {
    Regular = 0,
    Suppressed = 1,
    SelfDeleted = 2,
    ModDeleted = 9,
}

#[derive(Deserialize, Debug)]
pub struct Post {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: u64,
    #[serde(rename = "NATION")]
    pub nation: String,
    #[serde(rename = "STATUS")]
    pub status: PostStatus,
    #[serde(rename = "LIKES")]
    pub likes: u64,
    #[serde(rename = "LIKERS")]
    pub likers: Option<String>,
    #[serde(rename = "EMBASSY")]
    pub embassy: Option<String>,
    #[serde(rename = "SUPPRESSOR")]
    pub suppressor: Option<String>,
    #[serde(rename = "MESSAGE")]
    pub message: Option<String>,
}