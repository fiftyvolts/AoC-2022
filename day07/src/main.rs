use std::{env, fs, path::{PathBuf, Path}, str::FromStr, collections::HashMap};

fn input_txt() -> String {
    let path = env::args().nth(1).unwrap_or(String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part12(&input);
}

fn part12(input: &String) {
    let mut cwd: PathBuf = PathBuf::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut sizes: HashMap<String, u32> = HashMap::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("$ cd") {
            if &line[5..6] == "/" {
                cwd = PathBuf::from_str(&line[5..].to_string()).ok().unwrap();
            } else {
                for p in (&line[5..]).split("/") {
                    if p == ".." {
                        cwd.pop();
                    } else {
                        cwd.push(p);
                    }
                }
            }
            i+=1;
        } else if line.starts_with("$ ls") {
            i+=1;
            while i < lines.len() && !lines[i].starts_with("$") {
                let line = lines[i];
                if line.starts_with("dir") {

                } else {
                    let ent : Vec<&str> = line.split(" ").collect();
                    sizes.insert(cwd.join(ent[1]).to_str().unwrap().to_owned(), u32::from_str_radix(ent[0], 10).unwrap());
                }
                i+=1;
            }
        }
    }

    let mut sums : HashMap<String, u32> = HashMap::new();
    for (f, s) in &sizes {
        let mut path = Path::new(f);
        loop {
            path = path.parent().unwrap();
            sums.entry(path.to_str().unwrap().to_owned()).and_modify(|x| *x+=s).or_insert(*s);
            if path.eq(Path::new("/")) {
                break;
            }
        }
    }

    println!("{:?}", sums.values().filter(|s| **s<=100000).sum::<u32>());

    let total = sizes.values().sum::<u32>();
    let target = total - (70000000 - 30000000);

    let mut sorted : Vec<(&u32, &String)> = sums.iter().map(|(k,v)| (v,k)).collect();
    sorted.sort();
    for (v,k) in sorted {
        if *v >= target {
            println!("{}", v);
            break;
        }
    }
    
}
