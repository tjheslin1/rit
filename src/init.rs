use std::fs::create_dir;
use std::env::current_dir;
use std::io;

pub fn init() -> io::Result<String> {
	let binding = current_dir().unwrap();
	let dir = binding.to_str().unwrap();
	create(&dir)?;
	Ok(format!("{}/.rit", dir))
}

fn create(path: &str) -> io::Result<()> {
    println!("Init: {}", path);

    create_dir(format!("{}/{}", path, ".rit"))
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;
    use std::fs::read_dir;

    #[test]
    fn create_creates_dir() {
        let tmp = temp_dir();

        create(&tmp).unwrap();

        let mut ls = read_dir(&tmp).unwrap();

        let found_path = ls.find(|path| {
            path.as_ref().unwrap().path().to_str().unwrap() == format!("{}/.rit", tmp)
        });

        assert_eq!(found_path.is_some(), true);
    }

    #[test]
    fn err_if_dir_exists() {
        let tmp = temp_dir();

        create_dir(format!("{}/{}", tmp, ".rit")).unwrap();

        let result = init(&tmp);

        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::AlreadyExists);
    }

    fn temp_dir() -> String {
        let num = rand::thread_rng().gen_range(0..100000);
        let path = format!("/tmp/{}", num);

        create_dir(&path).expect(format!("Error occurred creating: {}", &path).as_str());
        path
    }
}
