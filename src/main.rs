extern crate clap;

use clap::{Arg, App};

fn main() {
  let matches = App::new("keasy")
    .version("0.1.0")
    .author("Anas Kadam <anaskadam@gmail.com>")
    .about("easy kubectl port-forward and exec in Rust")
    .arg(Arg::with_name("Deployment")
              .required(true)
              .takes_value(true)
              .index(1)
              .help("Deployment to work with"))
    .get_matches();
    
    let deployment = matches.value_of("Deployment").unwrap();
    println!("{}", deployment);
}
