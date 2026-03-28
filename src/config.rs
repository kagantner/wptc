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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_novel_config() {
        let yaml = r#"
metadata:
  title: "A Great Novel"
  byline: "John Doe"
  short_title: "Great Novel"
  last_name: "Doe"
  story_type: "novel"
author:
  legal_name: "Jonathan Doe"
  street_address: "123 Story Lane"
  city_state_zip: "Booktown, NY 12345"
  phone: "555-0100"
  email: "john@example.com"
structure:
  - type: part
    title: "Part One"
    content:
      - type: chapter
        title: "The Beginning"
        number: 1
        file: "ch1.md"
"#;
        let config: Result<Config, _> = serde_yaml::from_str(yaml);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.metadata.title, "A Great Novel");
        assert_eq!(config.metadata.story_type.unwrap(), "novel");
        assert_eq!(config.structure.len(), 1);
    }

    #[test]
    fn test_deserialize_short_story_config() {
        let yaml = r#"
metadata:
  title: "A Short Story"
  byline: "Jane Doe"
  short_title: "Short Story"
  last_name: "Doe"
author:
  legal_name: "Jane Doe"
  street_address: "456 Tale Blvd"
  city_state_zip: "Storyville, CA 90210"
  phone: "555-0200"
  email: "jane@example.com"
structure:
  - type: text
    file: "story.md"
"#;
        let config: Result<Config, _> = serde_yaml::from_str(yaml);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.metadata.story_type, None);
        assert_eq!(config.structure.len(), 1);
    }
}
