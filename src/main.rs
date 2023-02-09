use std::path::Path;
use std::env;
use std::fs;
fn cp_cont(append: bool, content: &str, file_path: &str){
    println!("{}", &content);
    // if (append==true){
    //
    // }
}
fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    // output to terminal
    if args[0].eq("1"){
        println!(std::env:fs::read_to_string(args[2]))
        // overwrite or create file if it doesnt exist
    } else if args[0].eq("2"){
    //    let content = std::env:fs::read_to_string(args[2]);
        let file_path = args[3]
        match fs::File::open(&file_path) {
            Ok(T) -> let f,
            Err(E) -> {fs::File:Create(&file_path); println!("file created");},
        }


    }
    // println!("{}", &args[2])
    // println!("{}", content);

    // let content = String::from("");

    // let mut iter = args.iter();
    // iter.next()
    // loop {
    //     let elem = iter.next();
    //     println!("{}", elem.unwrap());
        // match iter.next() {
        //     Some(inner) => elem,
        //     None        => _,
        // }
        // if elem.eq(">") {
        //     // cp_cont(false, content, iter.next());
        // } else if elem.eq(">>") {
        //     // cp_cont(true, content, iter.next());
        // } else {

        // content += fs::read_to_string(elem).unwrap();
        /* } */
        Ok(()j)
    }
}
