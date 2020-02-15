use gtk::*;

use std::collections::BTreeMap;
use std::process;

use arbol_eco;


pub struct Indice {

}


impl Indice {
  pub fn init () {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Arbol de Aperturas ECO");
    window.set_border_width(10);
    
    window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });
    
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 20);
    window.add(&vbox);
    
    // creamos la estructura del menu y el propio menu
    let accel_group = gtk::AccelGroup::new();
    window.add_accel_group(&accel_group);
    let mut definicion_menu = BTreeMap::new();
    definicion_menu.insert(
        "01-_Fichero".to_string(), vec!["_Salir|S".to_string()]
        );
    Menu::init (&vbox, definicion_menu, &accel_group);
    
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.set_size_request(-1, -1);
    vbox.pack_start(&hbox, false, false, 0);
    
    // el arbol ECO principal
    arbol_eco::ArbolECO::init(&hbox);
    
    window.show_all();
  }
  
  
}


/*
  El menu principal
*/
struct Menu {

}

impl Menu {
  fn init (vbox: &gtk::Box, 
        menu_def: BTreeMap<String, Vec<String>>,
        accel_group: &gtk::AccelGroup) {
      
    let menu_definition = menu_def;
    let menubar = gtk::MenuBar::new();
    vbox.pack_start(&menubar, false, false, 0);
    
    for (toplevel, sublevels) in &menu_definition {
      let top_menu = gtk::MenuItem::new_with_mnemonic(&toplevel);
      top_menu.set_use_underline(true);
      menubar.append(&top_menu);
      let menu = gtk::Menu::new();
      top_menu.set_submenu(Some(&menu));
      
      for submenu in sublevels.iter() {
        let accel_key:String;   // = String::new();
        let sub: String;        // = String::new();
        
        if submenu.contains("|") {
          let splited: Vec<&str> = submenu.split("|").collect();
          sub = splited[0].to_string();
          accel_key = splited[1].to_string();
        }
        else {
          accel_key = "".to_string();
          sub = submenu.to_string();
        }
        
        let menu_item = gtk::MenuItem::new_with_mnemonic(&sub);
        
        if accel_key != "".to_string() {
          // 'Primary' es 'Ctrl' en Windows y Linux, y 'command' en macOS
          // No está disponible directamente a través de gdk::ModifierType, 
          // ya que tiene valores diferentes en diferentes plataformas.
          // Los subrayados en Windows aparecen pulsando Alt
          let txt: &str = "<Primary>";
          let txt_acel = format!("{}{}", txt, accel_key);
          let txt = txt_acel.as_str();
          let (key, modifier) = gtk::accelerator_parse(&txt);
          menu_item.add_accelerator("activate", accel_group, key, modifier, AccelFlags::VISIBLE);
        }
        
        menu.append(&menu_item);
        
        menu_item.connect_activate ( move |widget| {
            Menu::menu_seleccionado(widget);
        });
      }
    }
  }
  
  // procesa la seleccion del menu
  fn menu_seleccionado (menu_item: &gtk::MenuItem) {
    let item = menu_item.get_label().unwrap();
    
    if item.as_str() == "_Salir" {
      gtk::main_quit();
      process::exit(0);
    }
  }
}