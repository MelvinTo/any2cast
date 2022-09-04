use std::path::Path;
use std::fs;
use std::fmt;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use log::{error, debug};


use rss::ChannelBuilder;
use rss::Item;
use rss::Guid;
use rss::Enclosure;

use rss::extension::itunes::ITunesItemExtension;
//use rss::extension::itunes::ITunesChannelExtensionBuilder;

#[derive(Debug, Serialize, Deserialize)]
enum DirectoryError {
    InvalidFilename,
}

impl fmt::Display for DirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DirectoryError {}

static LINK_PLACEHOLDER : &str = "__PLACEHOLDER__";

#[derive(Debug, Default, Serialize, Clone)]
pub struct Directory {
    path: String,
    pub name: String,
    pub link: Option<String>,

    #[serde(skip_serializing)]
    items: Vec<Item>,
}

impl Directory {
    pub fn new(path: &str) -> Result<Directory> {

        let file_path = Path::new(path);
        let file_name = file_path.file_name().unwrap();
        let name = file_name.to_str().unwrap().to_string();

        Ok(Directory{
            path: path.into(),
            link: Some(format!("/p/{}", &name)),
            name,
            ..Default::default()
        })
    }

    pub fn analyze(&mut self) -> Result<()> {
        let entries = fs::read_dir(&self.path)?;

        let mut valid_files = vec![];

        for entry in entries {
            let entry = entry?;

            if let Err(_e) = entry.file_type() {
                error!("Failed to get file type for file {:?}", &entry);
                continue;
            }

            let file_type = entry.file_type().unwrap();

            if ! file_type.is_file() {
                debug!("Ignore non-file entry: {:?}", &entry);
                continue;
            }

            let path = entry.path();
            let file_name = path.file_name().unwrap();

            if let None = path.extension() {
                debug!("Ignore file with no extension: {:?}", &path);
                continue;
            }

            let ext = path.extension().unwrap().to_str().unwrap().to_string();
            if ext != "mp3" {
                debug!("Ignore non-mp3 file: {:?}", &path);
                continue;
            }

            let name = file_name.to_str().unwrap().to_string();
            valid_files.push(name.clone());
        }

        valid_files.sort();

        for f in valid_files.iter() {
            let item = self.to_rss_item(&f)?;
            self.add_item(&item);
        }

        Ok(())
    }

    fn to_rss_item(&self, file_name: &str) -> Result<Item> {
        let mut item = Item::default();

        let file_stem = Path::new(file_name).file_stem().ok_or(DirectoryError::InvalidFilename)?;

        let title = file_stem.to_str().ok_or(DirectoryError::InvalidFilename)?;
        item.set_title(title.to_string());

        let link = format!("{}/p/{}/{}", LINK_PLACEHOLDER, self.name, file_name);

        item.set_link(link.clone());

        item.set_description("".to_string());

        let mut guid = Guid::default();
        guid.set_value(link.clone());
        item.set_guid(guid);

        let time = chrono::offset::Utc::now();
        item.set_pub_date(time.to_rfc2822());

        let mut enclosure = Enclosure::default();
        enclosure.set_url(link.clone());
        enclosure.set_length("123");
        enclosure.set_mime_type("audio/mpeg".to_string());
        item.set_enclosure(enclosure);

        let mut extension = ITunesItemExtension::default();
        extension.set_duration("1234".to_string());
        item.set_itunes_ext(extension);

        Ok(item)
    }

    fn add_item(&mut self, item: &Item) {
        self.items.push(item.clone());
    }

    #[allow(dead_code)]
    fn num_of_episodes(&self) -> usize {
        self.items.len()
    }

    /// Generate rss xml file content
    pub fn to_rss_xml(&self, scheme: &str, host: &str) -> Result<String> {
//        let ext = ITunesChannelExtensionBuilder::default()
//            .image(IMAGE_URL.to_string())
//            .build()
//            .unwrap();

        let mut channel = ChannelBuilder::default()
            .title(self.name.clone())
            .link(self.get_rss_link(scheme, host))
            .description("".to_string())
          //  .itunes_ext(ext)
            .build()
            .unwrap();


        channel.items.extend(self.items.clone());

        let channel_str = channel.to_string();
        let link = format!("{}/{}", &scheme, &host);
        Ok(channel_str.replace(LINK_PLACEHOLDER, &link))
    }

    fn get_rss_link(&self, scheme: &str, host: &str) -> String {
        format!("{}://{}/p/{}", scheme, host, self.name)
    }

}

#[cfg(test)]
mod tests {

    use super::Directory;

    #[test]
    fn test_new() {
        let path = "/tmp/a/b/c";
        let directory = Directory::new(path).expect("failed to parse path");
        assert_eq!(directory.path, path);
        assert_eq!(directory.name, "c");
    }

    #[test]
    fn test_analyze() {
        let path = "./example/21st_century_movie";
        let mut directory = Directory::new(path).expect("failed to parse path");
        assert_eq!(directory.path, path);
        assert_eq!(directory.name, "21st_century_movie");

        directory.analyze().expect("failed to analyze");
    }

    #[test]
    fn test_to_rss_item() {
        let path = "./example/21st_century_movie";
        let mut directory = Directory::new(path).expect("failed to parse path");
        assert_eq!(directory.path, path);
        assert_eq!(directory.name, "21st_century_movie");

        let item = directory.to_rss_item("file1.mp3").expect("failed to convert to item");
        assert_eq!(item.title.clone().expect("title doesn't exist"), "file1".to_string());

        directory.add_item(&item);

        assert_eq!(directory.num_of_episodes(), 1);
    }

    #[test]
    fn test_to_rss_xml() {
        let path = "./example/21st_century_movie";
        let mut directory = Directory::new(path).expect("failed to parse path");

        assert_eq!(directory.path, path);
        assert_eq!(directory.name, "21st_century_movie");

        let item = directory.to_rss_item("file1.mp3").expect("failed to convert to item");
        assert_eq!(item.title.clone().expect("title doesn't exist"), "file1".to_string());

        directory.add_item(&item);

        assert_eq!(directory.num_of_episodes(), 1);

        let output = directory.to_rss_xml("http", "localhost:8080").expect("failed to generate xml");
        println!("output: {}", output);
    }
}
