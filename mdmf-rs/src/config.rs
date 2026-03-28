use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub metadata: Metadata,
    pub author: Author,
    pub agent: Option<Agent>,
    pub structure: Vec<StructureItem>,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub subtitle: Option<String>,
    pub byline: String,
    pub genre: Option<String>,
    pub short_title: String,
    pub last_name: String,
    pub file_name: Option<String>,
    pub story_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub legal_name: String,
    pub pen_name: Option<String>,
    pub street_address: String,
    pub city_state_zip: String,
    pub phone: String,
    pub email: String,
    pub website: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Agent {
    pub name: String,
    pub agency: String,
    pub street_address: String,
    pub city_state_zip: String,
    pub phone: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum StructureItem {
    #[serde(rename = "part")]
    Part {
        title: String,
        content: Vec<StructureItem>,
    },
    #[serde(rename = "chapter")]
    Chapter {
        title: Option<String>,
        number: Option<u32>,
        file: Option<String>,
        files: Option<Vec<String>>,
    },
    #[serde(rename = "text")]
    Text {
        file: Option<String>,
        files: Option<Vec<String>>,
    },
}
