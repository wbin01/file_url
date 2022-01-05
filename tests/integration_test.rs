use std::collections::HashMap;

use file_url::FileUrl;

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
    let path_list = vec![
        ["c:/test/example/file.txt", "c:/test/example"],
        ["/home/user/video.mp4", "/home/user"],
        ["file:///c:/test/example/file.txt", "file:///c:/test/example"],
        ["file:c:/test/example/file.txt", "file:c:/test/example"],
        ["c:/test/example/file.txt", "c:/test/example"],
    ];

    for path_item in path_list {
        let file_uri = FileUrl::new(&path_item[0]);
        assert_eq!(file_uri.path(), &path_item[1]);
    }
}

#[test]
fn filename_test() {
    let filename_list = vec![
        ["c:/test/example/file.txt", "file.txt"],
        ["/home/user/.file.txt", ".file.txt"],
        ["/home/user/.file.txt.", ".file.txt."],
        ["/home/user/file.txt.", "file.txt."],
        ["/home/user/file.tar.gz", "file.tar.gz"],
        ["/home/user/.file.tar.gz", ".file.tar.gz"],
        ["/home/user/file.tar.gz.", "file.tar.gz."],
        ["/home/user/.file.tar.gz.", ".file.tar.gz."],
    ];

    for filename_item in filename_list {
        let file_uri = FileUrl::new(&filename_item[0]);
        assert_eq!(file_uri.filename(), &filename_item[1]);
    }
}

#[test]
fn filename_without_extension_test() {
    let filename_list = vec![
        ["c:/test/example/file.txt", "file"],
        ["/home/user/.file.txt", ".file"],
        ["/home/user/.file.txt.", ".file.txt."],
        ["/home/user/file.txt.", "file.txt."],
        ["/home/user/file.tar.gz", "file"],
        ["/home/user/.file.tar.gz", ".file"],
        ["/home/user/file.tar.gz.", "file.tar.gz."],
        ["/home/user/.file.tar.gz.", ".file.tar.gz."],
    ];

    for filename_item in filename_list {
        let file_uri = FileUrl::new(&filename_item[0]);
        assert_eq!(file_uri.filename_without_extension(), &filename_item[1]);
    }
}

#[test]
fn extension_test() {
    let extension_list = vec![
        ["c:/test/example/file.txt", ".txt"],
        ["/home/user/.file.txt", ".txt"],
        ["/home/user/.file.txt.", ""],
        ["/home/user/file.txt.", ""],
        ["/home/user/file.tar.gz", ".tar.gz"],
        ["/home/user/.file.tar.gz", ".tar.gz"],
        ["/home/user/file.tar.gz.", ""],
        ["/home/user/.file.tar.gz.", ""],
    ];

    for extension_item in extension_list {
        let file_uri = FileUrl::new(&extension_item[0]);
        assert_eq!(file_uri.extension(), &extension_item[1]);
    }
}