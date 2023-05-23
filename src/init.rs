use std::fs::create_dir;
use std::io;

pub fn init(path: &str) -> io::Result<()> {
    println!("Init: {}", path);

    create_dir(format!("{}/{}", path, ".rit"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use std::fs::read_dir;

    #[test]
    fn init_creates_dir() {
        let tmp = temp_dir();

        init(&tmp).unwrap();

        let mut ls = read_dir(&tmp).unwrap();

        let found_path = ls.find(|path| {
            path.as_ref().unwrap().path().to_str().unwrap() == format!("{}/.rit", tmp)
        });

        assert_eq!(found_path.is_some(), true);
    }

    fn temp_dir() -> String {
        let num = rand::thread_rng().gen_range(0..100000);
        let path = format!("/tmp/{}", num);

        create_dir(&path).expect(format!("Error occurred creating: {}", &path).as_str());
        path
    }
}
