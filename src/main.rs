use std::fs;
use std::collections::HashMap;
use CommandLib::{Befehl, system, ask};
use colored::Colorize;
use dirs;
use std::env;

fn verify() -> bool {
    let fff = format!(":: Verifying Configuration");
    println!("{}", fff.yellow().bold());

    // Get the home directory
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Error: Could not determine home directory.");
            return false;
        }
    };

    // Construct the full path to the second file
    let bfile_path = home_dir.join("etc/rbps/downloads.pt");
    let afile_path = home_dir.join("etc/rbps/mirror.pt");

    if let Ok(ametadata) = fs::metadata(afile_path) {
        if let Ok(bmetadata) = fs::metadata(&bfile_path){
            if ametadata.is_file() && bmetadata.is_file() {
                return true;
            }
        }
    }
    println!("");
    return false;
}

fn download(mirror: &str, file: &str) -> String{
    let cmd = format!("wget -q {}/{} > /dev/null", &mirror[..mirror.len() - 1], file);
    // :: Downloading Package
    println!("{}", ":: Downloading Package".yellow().bold());
    system(Befehl::new(&cmd));
    return cmd;
}

fn extract(file: &str, outdir: &str) {
    system(Befehl::new(format!("mkdir {}", outdir).as_str()));
    let cmd = format!("tar -xzvf {} --strip-components=1 -C {} > /dev/null", file, outdir);
    println!("{}", ":: Extracting Package".yellow().bold());
    system(Befehl::new(&cmd));
}

fn readfile(file: &str) -> String {
    fs::read_to_string(file).expect("File not found!")
}

fn readconf(file: &str) -> Vec<Vec<String>> {
    let file_contents = readfile(file);
    let binding = file_contents;
    let lines: Vec<&str> = binding.split('\n').collect();
    let installfiles: Vec<&str> = lines[1].split(' ').collect();
    let installdirs: Vec<&str> = lines[0].split(' ').collect();
    let installfilesstr = installfiles.iter().map(|sa| sa.to_string()).collect();
    let installdirsstr = installdirs.iter().map(|s| s.to_string()).collect();
    let returnarray: Vec<Vec<String>> = vec![installdirsstr, installfilesstr];
    return returnarray;
}

fn install(file: &str, logging: bool, autoconf: bool){
    if file != ""{
        let outdirname: &str = file.split(".").collect::<Vec<_>>()[0];
        extract(file, outdirname);
        let mut conf0: Vec<String> = vec![];
        let mut conf1: Vec<String> = vec![];
        // :: Finding Binaries
        println!("{}", ":: Finding Binaries".yellow().bold());
        let config = readconf(format!("{}/Path", outdirname).as_str());
        let mut conf: HashMap<&str, &str> = HashMap::new();
        // :: Binding HashMap Keys
        println!("{}", ":: Binding HashMap Keys".yellow().bold());
        for item0 in &config[0] {
            conf0.push(item0.to_string());
        }
        // :: Binding HashMap Values
        println!("{}", ":: Binding HashMap Values".yellow().bold());
        for item1 in &config[1] {
            conf1.push(item1.to_string());
        }
        if conf0.len() == conf1.len(){
            let mut i = 0;
            // :: Binding HashMap
            println!("{}", ":: Binding HashMap".yellow().bold());
            for _item44 in &conf0{
                conf.insert(conf0[i].as_str(), conf1[i].as_str());
                i += 1;
            }
            if !autoconf{
                let qa = ask("Do you wish to continue? [Y/n]: ");
                let q = qa;
                if q == true{
                    if logging{
                        // :: Logging Install
                        println!("{}", ":: Logging Install".yellow().bold());
                        system(Befehl::new(format!("sudo echo \"{}\" >> ~/etc/rbps/downloads.pt", file).as_str()));
                    }
                    // :: Installing Binaries
                    println!("{}", ":: Installing Binaries".yellow().bold());
                    let mut _insbin: String = String::new();
                    for (filek, dir) in &conf{
                        system(Befehl::new(format!("sudo mkdir {:#?}", outdirname).as_str()));
                        // :: Making Directories
                        println!("{}", ":: Making Directories".yellow().bold());
                        system(Befehl::new(format!("sudo mv {:#?}/{:#?} {:#?}", outdirname, dir, filek).as_str()));
                        _insbin = format!("{} {} into {}", ":: Installing:", dir, filek);
                        println!("{}", _insbin.yellow().bold());
                    }
                } else {
                    println!("{}", "Aborted..".yellow().bold());
                }
            } else {
                let qa = true;
                let q = qa;
                if q == true{
                    if logging{
                        // :: Logging Install
                        println!("{}", ":: Logging Install".yellow().bold());
                        system(Befehl::new(format!("sudo echo \"{}\" >> ~/etc/rbps/downloads.pt", file).as_str()));
                    }
                    // :: Installing Binaries
                    println!("{}", ":: Installing Binaries".yellow().bold());
                    let mut _insbin: String = String::new();
                    for (filek, dir) in &conf{
                        system(Befehl::new(format!("sudo mkdir {:#?}", outdirname).as_str()));
                        // :: Making Directories
                        println!("{}", ":: Making Directories".yellow().bold());
                        system(Befehl::new(format!("sudo mv {:#?}/{:#?} {:#?}", outdirname, dir, filek).as_str()));
                        _insbin = format!("{} {} into {}", ":: Installing:", dir, filek);
                        println!("{}", _insbin.yellow().bold());
                    }
                } else {
                    println!("{}", "Aborted..".yellow().bold());
                }
            }
        } else {
            // :: Error: Not Enough Binaries For Paths
            println!("{}", ":: Error: Number Of Keys And Values Do Not Match".red().bold());
        }
        // :: Cleaning Up
        println!("{}", ":: Cleaning Up".yellow().bold());
        system(Befehl::new(format!("sudo rm -rf ./{} ./{}", file, outdirname).as_str()));
    } else{
        println!("{}", ":: Error: No File".red().bold());
    }
}

fn update(mirror: &str){
    let home_dir = dirs::home_dir();
    let bfile_path = home_dir.expect("Something went wrong!").join("etc/rbps/downloads.pt");

    let cock = readfile(bfile_path.as_os_str().to_str().expect("Somethjüerjgperg"));
    let binding = cock;
    let lines: Vec<&str> = binding.split('\n').collect();
    let a: Vec<_> = lines.iter().map(|sa| sa.to_string()).collect();
    let v = ask("Do you wish to continue? [Y/n]: ");
    if v{
        for b in a{
            if b.as_str() != ""{
                let _ = download(mirror, b.as_str()).as_str();
                install(b.as_str(), false, true);
            }
        }
    } else{
        println!("{}", "Aborted..".yellow().bold());
    }
}

fn remove(file: &str, autoconf: bool) {
    if file != ""{
        let outdirname: &str = file.split(".").collect::<Vec<_>>()[0];
        extract(file, outdirname);
        let mut conf0: Vec<String> = vec![];
        let mut conf1: Vec<String> = vec![];
        // :: Finding Binaries
        println!("{}", ":: Finding Binaries".yellow().bold());
        let config = readconf(format!("{}/Path", outdirname).as_str());
        let mut conf: HashMap<&str, &str> = HashMap::new();
        // :: Binding HashMap Keys
        println!("{}", ":: Binding HashMap Keys".yellow().bold());
        for item0 in &config[0] {
            conf0.push(item0.to_string());
        }
        // :: Binding HashMap Values
        println!("{}", ":: Binding HashMap Values".yellow().bold());
        for item1 in &config[1] {
            conf1.push(item1.to_string());
        }
        if conf0.len() == conf1.len(){
            let mut i = 0;
            // :: Binding HashMap
            println!("{}", ":: Binding HashMap".yellow().bold());
            for _item44 in &conf0{
                conf.insert(conf0[i].as_str(), conf1[i].as_str());
                i += 1;
            }
            if !autoconf{
                let qa = ask("Do you wish to continue? [Y/n]: ");
                let q = qa;
                if q == true{
                    // :: Unlogging Removal
                    println!("{}", ":: Unlogging Removal".yellow().bold());
                    system(Befehl::new(format!("sudo sed -i 's/{}//g' ~/etc/rbps/downloads.pt", file).as_str()));
                    // :: Removing Binaries
                    println!("{}", ":: Removing Binaries".yellow().bold());
                    let mut _insbin: String = String::new();
                    for (filek, dir) in &conf{
                        system(Befehl::new(format!("sudo mkdir {:#?}", outdirname).as_str()));
                        system(Befehl::new(format!("sudo rm -rf {:#?}/{:#?}", filek, dir).as_str()));
                        _insbin = format!("{} {} from {}", ":: Removing:", dir, filek);
                        println!("{}", _insbin.yellow().bold());
                    }
                } else {
                    println!("{}", "Aborted..".yellow().bold());
                }
            } else {
                let qa = true;
                let q = qa;
                if q == true{
                    // :: Removing Binaries
                    println!("{}", ":: Removing Binaries".yellow().bold());
                    let mut _insbin: String = String::new();
                    for (filek, dir) in &conf{
                        system(Befehl::new(format!("sudo mkdir {:#?}", outdirname).as_str()));
                        system(Befehl::new(format!("sudo mv {:#?}/{:#?} {:#?}", outdirname, dir, filek).as_str()));
                        _insbin = format!("{} {} from {}", ":: Removing:", dir, filek);
                        println!("{}", _insbin.yellow().bold());
                    }
                } else {
                    println!("{}", "Aborted..".yellow().bold());
                }
            }
        } else {
            // :: Error: Not Enough Binaries For Paths
            println!("{}", ":: Error: Number Of Keys And Values Do Not Match".red().bold());
        }
        // :: Cleaning Up
        println!("{}", ":: Cleaning Up".yellow().bold());
        system(Befehl::new(format!("sudo rm -rf ./{} ./{}", file, outdirname).as_str()));
    } else {
        println!("{}", ":: Error: No File".red().bold());
    }
}

fn printusage() {
    let usage = "Usage: rbps [OPTIONS] (~DIRECTORY/PACKAGE~)
    OPTIONS:
        -Syi -> [PACKAGE]
            System Install
            # Will install a package and add an entry into '~/etc/rbps/downloads.pt'
        -Syr -> [PACKAGE]
            System Remove
            # Will remove a package WITH removing the entry in '~/etc/rbps/downloads.pt'
        -Syu ->
            System Update
            # Will update all packages that are installed and listed in '~/etc/rbps/downloads.pt'
        -Lr ->
            List Repository
            # Will list all availible packages
        -Lh ->
            List Help
            # Will show this help
";
    println!("{}", usage.red().bold());
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let verified: bool = verify();
    let home_dir = dirs::home_dir();
    let bfile_path = home_dir.clone().expect("fuck").join("etc/rbps/downloads.pt");
    let afile_path = home_dir.clone().expect("fuck").join("etc/rbps/mirror.pt");
    if argv.len() == 3{
        if verified == true{
            let mirror = readfile(afile_path.as_os_str().to_str().expect("Somethjüerjgperg"));
            let mrr = format!("{} {}", "-- Mirror:", &mirror[..mirror.len() - 1]);
            println!("{}", mrr.yellow().bold());
            match argv[1].as_str(){
                "-Syi" => {
                    download(mirror.as_str(), argv[2].as_str());
                    install(argv[2].as_str(), true, false);
                },
                "-Syr" => {
                    download(mirror.as_str(), argv[2].as_str());
                    remove(argv[2].as_str(), false);
                },
                &_ => printusage(),
            }
        } else{
            let ff = format!("{}", "No Configuration Found (~/etc/rbps/mirror.pt, ~/etc/rbps/downloads.pt)");
            println!("{}", ff.red().bold());
        }
    } else if argv.len() == 2{
        let mirror = readfile(afile_path.as_os_str().to_str().expect("Somethjüerjgperg"));
        let mrr = format!("{} {}", "-- Mirror:", &mirror[..mirror.len() - 1]);
        println!("{}", mrr.yellow().bold());
        if verified == true{
            match argv[1].as_str(){
                "-Lr" => {
                    download(mirror.as_str(), "list");
                    let a = readfile("list");
                    println!("{}", a);
                    system(Befehl::new("sudo rm -rf list"));
                },
                "-Syu" => {
                    update(mirror.as_str());
                },
                "-Lh" => {
                    printusage();
                },
                "-Li" => {
                    println!("{}", readfile(bfile_path.as_os_str().to_str().expect("Somethjüerjgperg")));
                },
                &_ => printusage(),
            }
        } else{
            let ff = format!("{}", "No Configuration Found (~/etc/rbps/mirror.pt, ~/etc/rbps/downloads.pt)");
            println!("{}", ff.red().bold());
        }
    } else {
        printusage();
    }
}
