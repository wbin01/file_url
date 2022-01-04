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

        // Extension
        let mut extension = String::new();

        match std_path.extension() {
            // Abstrair 'match' somente até Some e None, pq
            // para "", tem retorno que é Some(...("")) e None.
            Some(ext) => {
                // ext -> Some(OsStr::new("ext"))
                // &'ext' -> ext.to_str().unwrap()
                let ext = ext.to_str().unwrap();

                if ext != "" {
                    //

                    // Format extension
                    extension.push_str(ext);
                    extension.insert(0, '.');
                }
            },
            None => ()
        }

        // Filename
        let filename = std_path.file_name().unwrap().to_str().unwrap().to_string();

        // Filename without extension
        let filename_without_extension = String::new();

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
    use std::collections::HashMap;
    use super::*;
    
    #[test]
    fn general_test() {
        let hash_files_list = vec![
            HashMap::from([
                ("url", "/home/user/file.txt"),
                ("path", "/home/user"),
                ("filename", "file.txt"),
                ("filename_without_extension", "file"),
                ("extension", ".txt"),
            ]),
            HashMap::from([
                ("url", "/home/alien/video.mp4"),
                ("path", "/home/alien"),
                ("filename", "video.mp4"),
                ("filename_without_extension", "video"),
                ("extension", ".mp4"),
            ]),
            HashMap::from([
                ("url", "/home/unix/.hiddendoc"),
                ("path", "/home/unix"),
                ("filename", ".hiddendoc"),
                ("filename_without_extension", ".hiddendoc"),
                ("extension", ""),
            ]),
            HashMap::from([
                ("url", "/home/unix/doc"),
                ("path", "/home/unix"),
                ("filename", "doc"),
                ("filename_without_extension", "doc"),
                ("extension", ""),
            ]),
        ];

        for hash_file in hash_files_list {
            // Hash keys
            let hash_url = hash_file.get("url").unwrap();
            let hash_path = hash_file.get("path").unwrap();
            let hash_filename = hash_file.get("filename").unwrap();
            let hash_extension = hash_file.get("extension").unwrap();
            // let hash_filename_without_extension = hash_file.get(
            //     "filename_without_extension").unwrap();

            // File creation
            let file_uri = FileUrl::new(&hash_url);

            // Url
            assert_eq!(file_uri.url(), &hash_url.to_string());

            // Path
            assert_eq!(file_uri.path(), &hash_path.to_string());

            // Filename
            assert_eq!(file_uri.filename(), &hash_filename.to_string());

            // Extension
            assert_eq!(file_uri.extension(), &hash_extension.to_string());
        }
    }

    #[test]
    fn url_test() {
        let url_list = vec![
            "/home/user/video.mp4",
            "/home/user/file.txt",
        ];

        for url_item in url_list {
            let file_uri = FileUrl::new(&url_item);

            assert_eq!(file_uri.url(), &url_item.to_string());
        }
    }

    #[test]
    fn url_path_test() {
        // 
        assert_eq!(Some(OsStr::new("rs")), Path::new("foo.rs").extension());
        assert_eq!(Some(OsStr::new("gz")), Path::new("foo.tar.gz").extension());
        assert_eq!(Some(OsStr::new("tar")), Path::new("foo.tar").extension());
        assert_eq!(None, Path::new("foo").extension());
        assert_eq!(None, Path::new(".gz").extension());
        assert_eq!(Some(OsStr::new("")), Path::new("gz.").extension());
        assert_eq!(Some(OsStr::new("")), Path::new(".gz.").extension());

        // println!("{:?}", Path::new("foo").extension().unwrap());
        // assert_eq!(None, Path::new(".gz").extension().unwrap());
    }
}
