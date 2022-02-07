## file_url

A `struct` in Rust to slice url files

https://github.com/w-a-gomes/file_url

Example:
```rust
use file_url::FileUrl;

fn main() {
    // Internal extension ".tar"
    let my_file = FileUrl::new("/home/user/package.tar.gz");

    assert_eq!(my_file.url(), "/home/user/package.tar.gz");
    assert_eq!(my_file.path(), "/home/user");
    assert_eq!(my_file.filename(), "package.tar.gz");
    assert_eq!(my_file.name(), "package");
    assert_eq!(my_file.extension(), ".tar.gz");

    // Hidden file with extension
    let my_file = FileUrl::new("/home/user/.file.txt");

    assert_eq!(my_file.url(), "/home/user/.file.txt");
    assert_eq!(my_file.path(), "/home/user");
    assert_eq!(my_file.filename(), ".file.txt");
    assert_eq!(my_file.name(), ".file");
    assert_eq!(my_file.extension(), ".txt");

    // Hidden file without extension
    let my_file = FileUrl::new("/home/user/.bashrc");

    assert_eq!(my_file.url(), "/home/user/.bashrc");
    assert_eq!(my_file.path(), "/home/user");
    assert_eq!(my_file.filename(), ".bashrc");
    assert_eq!(my_file.name(), ".bashrc");
    assert_eq!(my_file.extension(), "");

    // Small filename
    let my_file = FileUrl::new("/home/user/f.tar.bz");

    assert_eq!(my_file.url(), "/home/user/f.tar.bz");
    assert_eq!(my_file.path(), "/home/user");
    assert_eq!(my_file.filename(), "f.tar.bz");
    assert_eq!(my_file.name(), "f");
    assert_eq!(my_file.extension(), ".tar.bz");

    // Big filename
    let my_file = FileUrl::new("/home/user/file-2022_01_01 (2) #My [1].tar.bz");

    assert_eq!(my_file.url(), "/home/user/file-2022_01_01 (2) #My [1].tar.bz");
    assert_eq!(my_file.path(), "/home/user");
    assert_eq!(my_file.filename(), "file-2022_01_01 (2) #My [1].tar.bz");
    assert_eq!(my_file.name(), "file-2022_01_01 (2) #My [1]");
    assert_eq!(my_file.extension(), ".tar.bz");
}

```
