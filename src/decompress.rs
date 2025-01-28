use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
fn main(){
    std::process::exit(real_main());
}

fn real_main()->i32{
    //declare vector args nd collect data 
    let args: Vec<_> = std::env::args().collect();
    if args.len()<2{
        println!("write file name {}",args[0]);
       // return 1;
    }
    let file_name=std::path::Path::new(&args[1]);
    let file=std::fs::File::open(file_name).unwrap();//to open the file
    let mut archieve  = zip::ZipArchive::new(file).unwrap();

    for i in 0..archieve.len(){
        let mut file = archieve.by_index(i).unwrap();
        // to check which one is closed 
        let outpath =match file.enclosed_name(){
            Some(path)=>path.to_owned(),
            //to borrow or clone the data 
            None=>continue,
        };

        {
            let comment =file.comment();
            //to check is there is any comment 
            if comment.is_empty(){
                println!("empty {} comment:{}",i,comment);
            } 
        }
        if (file.name()).ends_with("/"){
            println!("sub files exisit {} extracted to \"{}\"",i,outpath.display());
            fs::create_dir_all(&outpath).unwrap();//outpath.display() location of the file 
        }  else{
            println!("file {} is exrtaced to \"{}\"({}bytes )",i,outpath.display(),file.size());
            if let Some(p) =outpath.parent(){
                if !p.exists(){
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file ,&mut outfile).unwrap();
        }
        #[cfg(unix)]{
            
            if let Some(mode)= file.unix_mode(){
                fs::set_permissions(&outpath,std::fs::Permissions::from_mode(mode)).unwrap();
            } 
        }  
    
    }
    return 0;
}