use std::{env, println};
use std::fs;
mod commands;

fn get_ignores() -> Vec<String> {
    let content: Vec<char> = fs::read(".gitignore")
        .unwrap()
        .into_iter()
        .map(|byte| byte as char)
        .collect();

    let mut ret_vec: Vec<String> = vec![String::from(".git")];
    let mut res = String::new();

    for i in content {
        if i == '\n' {
            ret_vec.push(res.clone());
            res = String::new();
            continue;
        }
        res += &format!("{}", i);
    }

    ret_vec
}

fn list_files(dir: &str, files: &mut Vec<String>) -> std::io::Result<()> {
    let iter = fs::read_dir(dir)?;

    let iterator: Vec<fs::DirEntry> = iter.map(|r| r.unwrap()).collect::<Vec<fs::DirEntry>>();

    for i in 0..(iterator.len() + 1) {
        let _dir: &fs::DirEntry = match iterator.get(i) {
            None => {
                break;
            }
            Some(i) => i,
        };
        if _dir.file_type().unwrap().is_dir() {
            match get_ignores()
                .iter()
                .find(|x| x == &&format!("{}/", _dir.file_name().into_string().unwrap()))
            {
                None => {
                    if _dir.file_name().into_string().unwrap() != ".git" {
                        list_files(
                            format!("./{}", _dir.file_name().into_string().unwrap()).as_str(),
                            files,
                        )?;
                    }
                }
                Some(_) => {}
            }
        } else {
            files.push(format!(
                "{}/{}",
                dir,
                _dir.file_name().into_string().unwrap()
            ));
        }
    }
    Ok(())
}

// TODO
// This is a test
fn main() {
    let argv: Vec<String> = env::args().collect::<Vec<String>>();

    if argv.len() == 1 {
      commands::show_help();
      return;
    }

    match argv[1].as_str() {
        "@" => {
          let mut filelist = vec![];
          let folder = if argv.len() == 3 { argv[2].as_str() } else { "." };

          list_files(folder, &mut filelist).unwrap();

                for file in filelist {
                    let name = file.as_str();
                    let f = fs::read(name)
                        .unwrap()
                        .iter()
                        .map(|x| *x as char)
                        .collect::<Vec<char>>();
                    let mut res = String::new();
                    let mut lines: Vec<String> = vec![];
                    
                    for i in f {
                        if i == '\n' {
                            lines.push(res);
                            
                            res = String::new();
                            continue;
                        }
                        if (i == ' ' || i == '\t') && res == "" {
                            continue;
                        }

                        res += &format!("{}", i);
                    }
                    if lines.len() != 0 {
                        let mut l = 1;
                        lines.iter().for_each(|line| {
                            if line.as_str() == "// TODO" {
                                println!("{} :{}", name, l);
                            }
                            l += 1;
                        });
                    }
                }
        }
        _ => {}
    }
}
