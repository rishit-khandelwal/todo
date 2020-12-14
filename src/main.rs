use std::env;

fn getIgnore() -> Vec<String> {
    use std::fs;

    let content: Vec<char> = fs::read(".gitignore").unwrap().into_iter().map(|byte| byte as char).collect();
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

fn listFiles(dir: &str) -> std::io::Result<()> {
    use std::fs;
    
    let iter = fs::read_dir(dir)?;

    let iterator: Vec<fs::DirEntry> = iter.map(|r| r.unwrap()).collect::<Vec<fs::DirEntry>>();

    for i in 0..(iterator.len()+1) {
        let _dir: &fs::DirEntry = match iterator.get(i) {
            None => {
                break;
            },
            Some(i) => i
        };

        if _dir.file_type().unwrap().is_dir() {
            if _dir.file_name().to_str().unwrap() == ".git" {
                continue;
            }
            match getIgnore().iter().find(|x| x.as_str() == _dir.file_name().to_str().unwrap()) {
                None => {continue;},
                Some(_) => {}
            }
            println!("{}", _dir.file_name().into_string().unwrap());
            listFiles(_dir.file_name().into_string().unwrap().as_str())?;
        }
        else {
            println!("  {}", _dir.file_name().into_string().unwrap())
        }
    }
    Ok(())
}

fn main() {
    println!("TODO!\n");
    let argv: Vec<String> = env::args().collect::<Vec<String>>();

    match argv[1].as_str() {
        "@for" => {
            if argv.len() == 3 {
                let user  = argv[2].as_str();
                listFiles(".");
            }
        }
        _ => {}
    }
}
