use std::fs;

const README_PATH: &str = "./README.md";

fn main() -> Result<(), std::io::Error> {
    change_content()
}

fn change_content() -> Result<(), std::io::Error> {
    let mut contents = fs::read_to_string(README_PATH)
        .expect("Something went wrong reading the file, maybe you should do it the lame way");
    match contents.len() % 2 {
        0 => fs::write(README_PATH, format!("{} ", contents)),
        _ => {
            contents.remove(contents.len() - 1);
            fs::write(README_PATH, contents)
        }
    }
}
