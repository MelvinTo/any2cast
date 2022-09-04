use std::path::Path;
use std::fs;

use log::debug;
use anyhow::Result;
use serde::Serialize;

use handlebars::Handlebars;

use std::cell::RefCell;

use crate::dir::Directory;

#[derive(Debug, Default, Serialize, Clone)]
pub struct Site<'a> {
    path: String,
    name: String,

    #[serde(skip_serializing)]
    hb: Option<Handlebars<'a>>,

    pub dirs: Vec<Directory>,

    output_cache: RefCell<Option<String>>,
}

impl<'a> Site<'a> {
    pub fn new(path: String) -> Result<Site<'a>> {

        let file_path = Path::new(&path);
        let file_name = file_path.file_name().unwrap();

        debug!("Setting up site on {} ...", path);

        Ok(Site{
            path: path.to_string(),
            name: file_name.to_str().unwrap().to_string(),
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
        let cache = self.output_cache.borrow();
        let is_none = cache.is_none();

        if is_none {
            drop(cache); // readonly to mut
            let json = serde_json::to_value(&self)?;
            let body = self.hb.as_ref().unwrap().render("podcasts", &json)?;
            let mut mut_cache = self.output_cache.borrow_mut();
            mut_cache.replace(body.to_string());
        }

        Ok(self.output_cache.borrow().clone().unwrap())
    }

    pub fn detect_directories(&mut self) -> Result<()> {
        let paths = fs::read_dir(&self.path)?;

        for path in paths {
            if let Ok(path) = path {
                if let Ok(file_type) = path.file_type() {
                    if file_type.is_dir() {
                        let path = path.path().to_str().unwrap().to_string();
                        let mut dir = Directory::new(&path)?;
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
