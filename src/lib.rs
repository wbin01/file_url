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
    // use super::*;

    #[test]
    fn test() {
        //
    }
}
