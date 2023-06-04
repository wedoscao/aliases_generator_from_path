use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let path = env::var_os("PATH").unwrap();
    fs::write("aliases.txt", "").unwrap();

    let dir_names = path.to_str().unwrap().split(':').collect::<Vec<&str>>();

    for dir in dir_names.iter() {
        let files = match fs::read_dir(dir) {
            Ok(files) => files,
            Err(err) => {
                println!("error dir: {:?}", dir);
                panic!("{}", err)
            }
        };

        let exe_file_names = files
            .filter(|file| {
                String::from(file.as_ref().unwrap().file_name().to_str().unwrap()).contains(".exe")
            })
            .map(|file| file.unwrap().file_name())
            .collect::<Vec<_>>();

        for file_name in exe_file_names.iter() {
            let mut writer = OpenOptions::new()
                .write(true)
                .append(true)
                .open("aliases.txt")
                .unwrap();
            writer
                .write_all(
                    format!(
                        "alias {}={}\n",
                        &file_name.to_str().unwrap().split('.').collect::<Vec<_>>()[0],
                        file_name.to_str().unwrap()
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
    }
}
