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
    if args.len() >= 2 {
		match args[1].trim().parse::<String>() {
			Ok(i) => { file_name = i; }
			Err(error) => panic!(error)
        }
		match args[2].trim().parse::<String>() {
			Ok(i) => { dir_name = i; }
			Err(error) => panic!(error)
		}
    } else {
        panic!("Please, enter two parameters (filename and dirname).");
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