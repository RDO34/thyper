use std::fs;

use dirs::data_local_dir;

fn main() {
    let mut dir_path = data_local_dir().expect("unable to find local data dir");
    
    if !dir_path.exists() {
        panic!("local data dir does not exist at {}!", dir_path.display());
    }
    dir_path.push("thyper");

    if !dir_path.exists() {
        fs::create_dir(dir_path.clone()).expect("failed to create thyper data dir");
    }

    let files = fs::read_dir("static").expect("failed to read static build files dir");
    for file in files {
        let file = file.unwrap();
        let src = file.path();
        let dst = dir_path.join(file.file_name());
        
        if !dst.exists() {
            fs::copy(&src, &dst).expect("failed to copy build file");
        }
        
        let mut perms = fs::metadata(dst.clone())
            .expect("failed to get metadata for build file")
            .permissions();
    
        perms.set_readonly(false);
        fs::set_permissions(dst, perms).expect("failed to update file write permissions");
    }
}

