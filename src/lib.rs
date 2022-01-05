//! # Crate file_url
//!
//! `file_url` is a structure to conveniently separate
//! the url parts of a file, such as path, name and extension. 

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// use file_url::FileUrl;
///
/// let my_file_url = "/home/user/package.tar.gz";
/// let my_file = FileUrl::new(&my_file_url);
///
/// assert_eq!(my_file.path(), "/home/user");
/// assert_eq!(my_file.filename(), "package.tar.gz");
/// assert_eq!(my_file.filename_without_extension(), "package");
/// assert_eq!(my_file.extension(), ".tar.gz");
/// ```
use std::path::Path;
use std::ffi::OsStr;

#[derive(Debug)]
pub struct FileUrl {
    url: String,
    path: String,
    filename: String,
    filename_without_extension: String,
    extension: String,
}

impl FileUrl {
    pub fn new(url: &str) -> FileUrl {
        // Path::new(url) -> &std::path::Path
        //   .parent()    -> Option(Some(&std::path::Path))
        //   .unwrap()    -> &std::path::Path
        //   .to_str()    -> Option(&"/path")
        //   .unwrap()    -> &"/path"
        //   .to_string() -> String::new("/path")
        let std_path = Path::new(url);

        // Url
        let url = url.to_string();

        // Path
        let path = std_path.parent().unwrap().to_str().unwrap().to_string();

        // Filename
        let filename = std_path.file_name().unwrap().to_str().unwrap().to_string();

        // Extension
        let mut extension = String::new();

        match std_path.extension() {
            Some(ext) => {  // ext -> Some(OsStr::new("ext"))
                let ext = ext.to_str().unwrap();  // &'ext' -> ext.to_str().unwrap()

                if ext != "" {  // Tratar extensão dupla, como "tar.gz".
                    let (filename_len, ext_len) = (filename.len(), ext.len());

                    if filename_len - ext_len > 5 {  // 5 = ".tar.".len()
                        let in_filename = &filename[..(filename_len - ext_len - 1)];

                        if Path::new(in_filename).extension() == Some(OsStr::new("tar")) {
                            extension.push_str(".tar.");
                            extension.push_str(ext);
                        } else {
                            extension.push_str(ext);
                            extension.insert(0, '.');
                        }
                    } else {
                        extension.push_str(ext);
                        extension.insert(0, '.');
                    }
                }
            },
            None => ()
        }

        // Filename without extension
        let filename_without_extension = filename[..].replace(&extension, "");

        // FileUrl
        FileUrl {
            url,
            path,
            filename,
            filename_without_extension,
            extension,
        }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn filename(&self) -> &String {
        &self.filename
    }

    pub fn filename_without_extension(&self) -> &String {
        &self.filename_without_extension
    }

    pub fn extension(&self) -> &String {
        &self.extension
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_url_test() {
        let my_file = FileUrl::new("/home/user/package.tar.gz");
        
        assert_eq!(my_file.path(), "/home/user");
        assert_eq!(my_file.filename(), "package.tar.gz");
        assert_eq!(my_file.filename_without_extension(), "package");
        assert_eq!(my_file.extension(), ".tar.gz");
    }

    #[test]
    fn small_file_url_test() {
        let my_file = FileUrl::new("/home/user/f.tar.bz");
        
        assert_eq!(my_file.path(), "/home/user");
        assert_eq!(my_file.filename(), "f.tar.bz");
        assert_eq!(my_file.filename_without_extension(), "f");
        assert_eq!(my_file.extension(), ".tar.bz");
    }

    #[test]
    fn big_file_url_test() {
        let my_file = FileUrl::new("/home/user/file-2022_01_01 (2) #My [1].tar.bz");
        
        assert_eq!(my_file.path(), "/home/user");
        assert_eq!(my_file.filename(), "file-2022_01_01 (2) #My [1].tar.bz");
        assert_eq!(my_file.filename_without_extension(), "file-2022_01_01 (2) #My [1]");
        assert_eq!(my_file.extension(), ".tar.bz");
    }
}
