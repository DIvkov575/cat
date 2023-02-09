use std::env;
use std::fs;
fn cp_cont(append: bool, content: &str, file_path: &str){
    println!("{}", &content);
    // if (append==true){
    //
    // }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!(std::env:fs::read_to_string(args[1]));
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

    }
}
