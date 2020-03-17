extern crate link_extracter;

#[cfg(test)]
mod integration_tests {
    use link_extracter::functions;

    #[test]
    fn test_get_download_link() {
        let expected_url: String = "https://communaute.chorus-pro.gouv.fr/wp-content/uploads/2020/03/AIFE-Chorus-Pro-Annuaire_20200309.xlsx".to_owned();
        let test_url = functions::get_download_link("https://communaute.chorus-pro.gouv.fr/annuaire-cpro").unwrap();
        assert_eq!(expected_url, test_url);
    }
}