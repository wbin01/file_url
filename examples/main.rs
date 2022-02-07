use file_url::FileUrl;

fn main() {
    // Double extension file
    println!("Double extension file:");
    let my_file = FileUrl::new("/home/user/package.tar.gz");

    assert_eq!(my_file.url(), "/home/user/package.tar.gz");
    assert_eq!(my_file.path(), "/home/user");
    assert_eq!(my_file.filename(), "package.tar.gz");
    assert_eq!(my_file.name(), "package");
    assert_eq!(my_file.extension(), ".tar.gz");
    
    println!(r#"    Url: "{}""#, my_file.url());
    println!(r#"    Path: "{}""#, my_file.path());
    println!(r#"    Filename: "{}""#, my_file.filename());
    println!(r#"    Name: "{}""#, my_file.name());
    println!(r#"    Extension: "{}""#, my_file.extension());

    // Hidden file with extension
    println!("\nHidden file with extension:");
    let my_file = FileUrl::new("/home/user/.file.txt");

    assert_eq!(my_file.url(), "/home/user/.file.txt");
    assert_eq!(my_file.path(), "/home/user");
    assert_eq!(my_file.filename(), ".file.txt");
    assert_eq!(my_file.name(), ".file");
    assert_eq!(my_file.extension(), ".txt");
    
    println!(r#"    Url: "{}""#, my_file.url());
    println!(r#"    Path: "{}""#, my_file.path());
    println!(r#"    Filename: "{}""#, my_file.filename());
    println!(r#"    Name: "{}""#, my_file.name());
    println!(r#"    Extension: "{}""#, my_file.extension());

    // Hidden file without extension
    println!("\nHidden file without extension:");
    let my_file = FileUrl::new("/home/user/.bashrc");

    assert_eq!(my_file.url(), "/home/user/.bashrc");
    assert_eq!(my_file.path(), "/home/user");
    assert_eq!(my_file.filename(), ".bashrc");
    assert_eq!(my_file.name(), ".bashrc");
    assert_eq!(my_file.extension(), "");

    println!(r#"    Url: "{}""#, my_file.url());
    println!(r#"    Path: "{}""#, my_file.path());
    println!(r#"    Filename: "{}""#, my_file.filename());
    println!(r#"    Name: "{}""#, my_file.name());
    println!(r#"    Extension: "{}""#, my_file.extension());

    // Small filename
    println!("\nSmall filename:");
    let my_file = FileUrl::new("/home/user/f.tar.bz");

    assert_eq!(my_file.url(), "/home/user/f.tar.bz");
    assert_eq!(my_file.path(), "/home/user");
    assert_eq!(my_file.filename(), "f.tar.bz");
    assert_eq!(my_file.name(), "f");
    assert_eq!(my_file.extension(), ".tar.bz");

    println!(r#"    Url: "{}""#, my_file.url()); 
    println!(r#"    Path: "{}""#, my_file.path());
    println!(r#"    Filename: "{}""#, my_file.filename());
    println!(r#"    Name: "{}""#, my_file.name());
    println!(r#"    Extension: "{}""#, my_file.extension());

    // Big filename
    println!("\nBig filename");
    let my_file = FileUrl::new("/home/user/file-2022_01_01 (2) #My [1].tar.bz");

    assert_eq!(my_file.url(), "/home/user/file-2022_01_01 (2) #My [1].tar.bz");
    assert_eq!(my_file.path(), "/home/user");
    assert_eq!(my_file.filename(), "file-2022_01_01 (2) #My [1].tar.bz");
    assert_eq!(my_file.name(), "file-2022_01_01 (2) #My [1]");
    assert_eq!(my_file.extension(), ".tar.bz");

    println!(r#"    Url: "{}""#, my_file.url()); 
    println!(r#"    Path: "{}""#, my_file.path());
    println!(r#"    Filename: "{}""#, my_file.filename());
    println!(r#"    Name: "{}""#, my_file.name());
    println!(r#"    Extension: "{}""#, my_file.extension());
}
