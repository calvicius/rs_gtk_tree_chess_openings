extern crate gtk;
extern crate cairo;
extern crate gdk;
extern crate gdk_pixbuf;

#[macro_use]
extern crate lazy_static;
extern crate mut_static;    // https://github.com/tyleo/mut_static

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate regex;
extern crate threadpool;
extern crate num_cpus;

use std::process;


mod principal;

use principal::tablero;
use principal::indice;
use principal::arbol_eco;
use principal::arbol_subeco;
use principal::const_eco;



fn main() {
    if gtk::init().is_err() {
        eprintln!("No se ha podido iniciar la aplicacion GTK");
        process::exit(1);
    }
    
    let _app = indice::Indice::init();
    
    gtk::main();
}
