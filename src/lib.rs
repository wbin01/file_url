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
            // Abstrair 'match' somente até Some e None, pq
            // para ext "", tem retorno que é Some(...("")) e None.
            Some(ext) => {
                // ext -> Some(OsStr::new("ext"))
                // &'ext' -> ext.to_str().unwrap()
                let ext = ext.to_str().unwrap();

                // Ja foram tratados os arquivos que retornam "" para ext:
                // ("file", ".file", "file.", ".file.")
                if ext != "" {
                    // Tratar extensão dupla, como "tar.gz".
                    let in_filename = &filename[..(filename.len() - ext.len() - 1)];

                    if Path::new(in_filename).extension() == Some(OsStr::new("tar")) {
                        extension.push_str(".tar.");
                        extension.push_str(ext);

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
    use std::collections::HashMap;
    use super::*;
    
    #[test]
    fn general_test() {
        let hash_files_list = vec![
            HashMap::from([ // file.txt
                ("url", "/home/user/file.txt"),
                ("path", "/home/user"),
                ("filename", "file.txt"),
                ("filename_without_extension", "file"),
                ("extension", ".txt"),
            ]),
            HashMap::from([ // file
                ("url", "/home/user/file"),
                ("path", "/home/user"),
                ("filename", "file"),
                ("filename_without_extension", "file"),
                ("extension", ""),
            ]),
            // Anomalias
            HashMap::from([ // .file
                ("url", "/home/user/.file"),
                ("path", "/home/user"),
                ("filename", ".file"),
                ("filename_without_extension", ".file"),
                ("extension", ""),
            ]),
            HashMap::from([ // file.
                ("url", "/home/user/file."),
                ("path", "/home/user"),
                ("filename", "file."),
                ("filename_without_extension", "file."),
                ("extension", ""),
            ]),
            HashMap::from([ // .file.
                ("url", "/home/user/.file."),
                ("path", "/home/user"),
                ("filename", ".file."),
                ("filename_without_extension", ".file."),
                ("extension", ""),
            ]),
            // Duplas
            HashMap::from([ // file.tar.gz
                ("url", "/home/user/file.tar.gz"),
                ("path", "/home/user"),
                ("filename", "file.tar.gz"),
                ("filename_without_extension", "file"),
                ("extension", ".tar.gz"),
            ]),
            HashMap::from([ // file.2.tar.gz
                ("url", "/home/user/file.2.tar.gz"),
                ("path", "/home/user"),
                ("filename", "file.2.tar.gz"),
                ("filename_without_extension", "file.2"),
                ("extension", ".tar.gz"),
            ]),
            // Url Windows
            HashMap::from([ // file:///c:/
                ("url", "file:///c:/test/example/file.txt"),
                ("path", "file:///c:/test/example"),
                ("filename", "file.txt"),
                ("filename_without_extension", "file"),
                ("extension", ".txt"),
            ]),
            HashMap::from([ // file:c:/
                ("url", "file:c:/test/example/file.txt"),
                ("path", "file:c:/test/example"),
                ("filename", "file.txt"),
                ("filename_without_extension", "file"),
                ("extension", ".txt"),
            ]),
            HashMap::from([ // c:/
                ("url", "c:/test/example/file.txt"),
                ("path", "c:/test/example"),
                ("filename", "file.txt"),
                ("filename_without_extension", "file"),
                ("extension", ".txt"),
            ]),

        ];

        for hash_file in hash_files_list {
            // Hash keys
            let hash_url = hash_file.get("url").unwrap();
            let hash_path = hash_file.get("path").unwrap();
            let hash_filename = hash_file.get("filename").unwrap();
            let hash_extension = hash_file.get("extension").unwrap();
            let hash_filename_without_extension = hash_file.get(
                "filename_without_extension").unwrap();

            // Create File
            let file_uri = FileUrl::new(&hash_url);

            // Test Url
            assert_eq!(file_uri.url(), &hash_url.to_string());
            // Test Path
            assert_eq!(file_uri.path(), &hash_path.to_string());
            // Test Filename
            assert_eq!(file_uri.filename(), &hash_filename.to_string());
            // Test Extension
            assert_eq!(file_uri.extension(), &hash_extension.to_string());
            // Test Filename without extension
            assert_eq!(
                file_uri.filename_without_extension(),
                &hash_filename_without_extension.to_string()
            );
        }
    }

    #[test]
    fn url_test() {
        let url_list = vec![
            "/home/user/video.mp4",
            "/home/user/file.txt",
            "file:///c:/test/example/file.txt",
            "file:c:/test/example/file.txt",
            "c:/test/example/file.txt",
        ];

        for url_item in url_list {
            let file_uri = FileUrl::new(&url_item);

            assert_eq!(file_uri.url(), &url_item.to_string());
        }
    }

    #[test]
    fn path_test() {
        //
    }
}
