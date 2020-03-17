extern crate dotenv;

mod functions;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let mut file_name: String   = "".to_owned();
    let mut dir_name: String    = "".to_owned();

    // Get args
    let args: Vec<String>   = env::args().collect();
    if args.len() == 3 {
		match args[1].trim().parse::<String>() {
			Ok(i) => { file_name = i; }
			Err(error) => panic!(error)
        }
		match args[2].trim().parse::<String>() {
			Ok(i) => { dir_name = i; }
			Err(error) => panic!(error)
		}
    }
    else if args.len() == 2 && args[1] == "--help" {
        let message = r#"
            Exemple : ./link-extracter.exe annuaire-cpro.xls ./myfolder
            Parameters : 
                1: filename
                2: foldername
        "#;
        println!("{}", message);
        std::process::exit(1);
    }
    else {
        println!("Please, enter two parameters (filename and dirname). Get help with --help flag");
        std::process::exit(1);
    }

    // Just remove / or \ character if available
    if dir_name.ends_with("/") || dir_name.ends_with("\\") {
        dir_name.pop();
    }

    // Get download url
    match functions::get_download_link("https://communaute.chorus-pro.gouv.fr/annuaire-cpro") {
        Ok(url) => {
            // Download file
            functions::download_link(url.as_str(), file_name.as_str(), dir_name.as_str());
        },
        Err(err) => panic!(err)
    };
}