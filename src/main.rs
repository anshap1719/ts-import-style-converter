use std::fs;
use std::fs::DirEntry;

fn fix_dir(dir: std::io::Result<DirEntry>, depth: u32) {
    let dir = dir.unwrap();
    let metadata = dir.metadata().unwrap();

    if metadata.is_dir() {
        for item in fs::read_dir(dir.path()).unwrap() {
            fix_dir(item, depth + 1)
        }
    }

    if metadata.is_file() {
        let filename = dir.file_name();
        let filename = filename.to_str().unwrap();

        if !filename.contains(".ts") && !filename.contains(".tsx") {
            return;
        }

        let mut contents = fs::read_to_string(dir.path()).unwrap();

        let mut relative_path = String::new();
        for _ in 0..depth {
            relative_path = format!("../{relative_path}");
        }

        contents = contents.replace(
            "'src/",
            format!("'{relative_path}").as_str()
        );

        fs::write(dir.path(), contents).unwrap();
    }
}

fn main() {
    let dir = fs::read_dir("./src").unwrap();

    for item in dir {
        fix_dir(item, 0)
    }
}
