use std::path::Path;
use std::fs;
use std::fs::metadata;

use log::info;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use handlebars::Handlebars;

use crate::dir::Directory;

#[derive(Debug, Default, Serialize, Clone)]
pub struct Site<'a> {
    path: String,
    name: String,
    link: String,

    #[serde(skip_serializing)]
    hb: Option<Handlebars<'a>>,

    pub dirs: Vec<Directory>,
}

impl<'a> Site<'a> {
    pub fn new(path: String, link: &str) -> Result<Site<'a>> {

        let file_path = Path::new(&path);
        let file_name = file_path.file_name().unwrap();

        info!("Setting up site on {} ...", path);

        Ok(Site{
            path: path.to_string(),
            name: file_name.to_str().unwrap().to_string(),
            link: link.to_string(),
            ..Default::default()
        })
    }

    pub fn prepare_static_files(&mut self) -> Result<()> {
        let mut hbars = Handlebars::new();
        hbars.register_template_string("podcasts", include_str!("../static/podcasts.html"))?;
        self.hb = Some(hbars);
        Ok(())
    }

    pub fn to_html(&self) -> Result<String> {
        let json = serde_json::to_value(&self)?;
        let body = self.hb.as_ref().unwrap().render("podcasts", &json)?;
        Ok(body.to_string())
    }

    pub fn detect_directories(&mut self) -> Result<()> {
        let paths = fs::read_dir(&self.path)?;

        for path in paths {
            if let Ok(path) = path {
                if let Ok(file_type) = path.file_type() {
                    if file_type.is_dir() {
                        let path = path.path().to_str().unwrap().to_string();
                        let mut dir = Directory::new(&path)?;
                        dir.link_prefix = self.link.clone();
                        dir.analyze()?;
                        self.dirs.push(dir);
                    }
                }
            }
        }

        // sort by dir name
        self.dirs.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());

        Ok(())
    }

    pub fn get_directory(&self, name: &str) -> Option<Directory> {
        for dir in self.dirs.iter() {
            if dir.name == name {
                return Some(dir.clone());
            }
        }

        None
    }


}

#[cfg(test)]
mod tests {

    use super::Site;

    #[test]
    fn test_new() {
        let path = "/tmp/a/b/c";
        let site = Site::new(path.to_string()).expect("failed to parse path");
        assert_eq!(site.path, path);
        assert_eq!(site.name, "c");
    }

}
