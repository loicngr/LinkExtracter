# LinkExtracter
Download file from link

### File url
- > https://communaute.chorus-pro.gouv.fr/annuaire-cpro/

### Parameters
1.  file name = `myfile.xls`
2.  folder name \
    Exemple : \
    A. Current folder = `./` \
    B. User root folder = `/home/user`

### Launch
#### Cargo
- > cargo run annuaire-cpro.xls ./myfolder
#### Standalone
- > executable.exe annuaire-cpro.xls ./myfolder

### Tests
- > cargo test