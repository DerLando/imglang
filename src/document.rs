#[derive(Default)]
pub(crate) struct Document {
    content: String,
    path: Option<std::path::PathBuf>,
}

impl Document {
    pub fn open<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(&path)?;
        Ok(Self {
            content,
            path: Some(path.as_ref().to_path_buf()),
        })
    }

    pub fn save(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.path {
            std::fs::write(path, &self.content)
        } else {
            todo!("Set the path here and then write")
        }
    }

    // TODO: Better api for save()/can_save() package, as it's
    // an error vector
    pub fn can_save(&self) -> bool {
        self.path.is_some()
    }

    pub fn set_path<P: AsRef<std::path::Path>>(&mut self, path: P) {
        self.path = Some(path.as_ref().to_path_buf());
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn content_mut(&mut self) -> &mut String {
        &mut self.content
    }
}
