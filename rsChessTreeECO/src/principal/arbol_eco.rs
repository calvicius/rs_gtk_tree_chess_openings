use gtk::*;
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;


use arbol_subeco;
use tablero;



#[derive(Clone)]
pub struct ArbolECO {
    
}


#[derive(Clone)]
struct Datos {
    id : String,
    parent : String,
    data : String,
}

impl Datos {
  fn init(id: String, parent: String, data: String) -> Datos {
      Datos {
          id,
          parent,
          data,
      }
  }
}

impl ArbolECO {
  pub fn init(hbox: &gtk::Box) {
    let mut data: Vec<Datos> = Vec::new();
    data = ArbolECO::crea_lista_datos(data);
    ArbolECO::muestra_tabla(&hbox, data);
  }
  
  fn crea_lista_datos(mut lista: Vec<Datos>) -> Vec<Datos>{
    let mut fila = Datos::init("ECO".to_string(), "0".to_string(), " ".to_string());
    lista.push(fila);
    fila = Datos::init("A00".to_string(), "ECO".to_string(), "Aperturas de flanco".to_string());
    lista.push(fila);
    fila = Datos::init("B00".to_string(), "ECO".to_string(), "Aperturas semiabiertas".to_string());
    lista.push(fila);
    fila = Datos::init("C00".to_string(), "ECO".to_string(), "Def. Francesa y abiertas".to_string());
    lista.push(fila);
    fila = Datos::init("D00".to_string(), "ECO".to_string(), "Cerradas y Grunfeld".to_string());
    lista.push(fila);
    fila = Datos::init("E00".to_string(), "ECO".to_string(), "Defensas Indias".to_string());
    lista.push(fila);
    
    lista
  }
  
  fn muestra_tabla (hbox: &gtk::Box, data: Vec<Datos>) {
    let model = gtk::TreeStore::new(&[gtk::Type::String,
              gtk::Type::String
              ]);
    let field_header: [&str; 2] = ["ECO", "Apertura"];
    
    // Creamos la view para mostrar la list/tree store
    let view = gtk::TreeView::new_with_model(&model); 
    view.set_headers_visible(false);
    hbox.pack_start(&view, false, false, 0);
    
    // ahora creamos las columnas
    for i in 0..field_header.len() {
      let cell_renderer = gtk::CellRendererText::new();
      let column = gtk::TreeViewColumn::new();
      //column.set_title(field_header[i]);
      column.pack_start(&cell_renderer, true);
      column.add_attribute(&cell_renderer, "text", i as i32);
      view.append_column(&column);
    }
    
    // creamos un hashmap para almacenar las id y sus iter correspondientes
    // por si la lista esta desordenada 
    let mut id_iter = HashMap::new();
    
    for item in &data {
      let id = &item.id;
      let parent = &item.parent;
      let data = &item.data;
      // necesitamos un clone() por problemas de mutabilidad en el match
      let id_iter_clon = id_iter.clone();
      
      if parent == &"0".to_string() {     //no tiene padre
        let iter = model.append(None);
        model.set(&iter, &[0,1], 
              &[&id, &data]);
        id_iter.insert(id, iter);
      }
      else {
        match id_iter_clon.get(parent) {
          Some(valor) => {
            let iter1 = model.append(Some(valor));
            model.set(&iter1, &[0,1], 
                  &[&id, &data]);
            // Si el mapa no tenía esta clave presente, se devuelve None.
            // Si el mapa tenía presente esta clave, el valor se 
            // actualiza y se devuelve el valor anterior. 
            //Sin embargo, la clave no se actualiza; 
            id_iter.insert(id, iter1);  // -> Option<V> ; V = valor
          },
          None => {
            let iter = model.append(None);
            model.set(&iter, &[0,1], 
                  &[&id, &data]);
            id_iter.insert(id, iter);
          },
        }
      }
    }
    
    view.expand_all();
    view.set_enable_tree_lines(true);
    
    // ahora la parte de la seleccion de datos
    let seleccion = view.get_selection();
    
    
    // ******** arbol central ************
    let model_centro = gtk::TreeStore::new(&[gtk::Type::String,
              gtk::Type::String,
              gtk::Type::String,
              gtk::Type::String
              ]);
    let field_header: [&str; 4] = ["ECO", "Apertura", "Variante" ,"Jugadas"];
    
    // Creamos la view_centro para mostrar la list/tree store
    let view_centro = gtk::TreeView::new_with_model(&model_centro); 
    view_centro.set_headers_visible(true);
    
    let sw = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    sw.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    sw.set_size_request(700, 400);
    sw.add(&view_centro);
    hbox.pack_start(&sw, true, true, 0);
    
    
    // ahora creamos las columnas
    for i in 0..field_header.len() {
      let cell_renderer = gtk::CellRendererText::new();
      let column = gtk::TreeViewColumn::new();
      if i == 2 {
        column.set_max_width(180);
      }
      column.set_sort_column_id(i as i32);
      column.set_title(field_header[i]);
      column.pack_start(&cell_renderer, true);
      column.add_attribute(&cell_renderer, "text", i as i32);
      view_centro.append_column(&column);
    }
    
    view_centro.expand_all();
    view_centro.set_enable_tree_lines(true);
    
    let seleccion_centro = view_centro.get_selection();
    
    // ********** fin arbol central **********
    
    let fen_inicial = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let jugadas = "".to_string();
    tablero::Tablero::init(&hbox, fen_inicial, jugadas);
    //hbox.show_all();
    
    
    // ********** el closure del arbol izquierdo *****
    let sel_cen_clon = seleccion_centro.clone();
    let weak_model_centro = model_centro.downgrade();
    let weak_hbox = hbox.downgrade();
    seleccion.connect_changed(move |widget| {
      let model_centro = match weak_model_centro.upgrade() {
                    Some(model_centro) => model_centro,
                    None => return,
                };
      let hbox = match weak_hbox.upgrade() {
                    Some(hbox) => hbox,
                    None => return,
                };
      
      sel_cen_clon.set_mode(gtk::SelectionMode::None);
      if let Some((modelo, iter)) = widget.get_selected() {
        let bruto = modelo.get_value(&iter, 0); // columna 1
        let eco = bruto.get::<String>().unwrap();
        
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
          let validadas = arbol_subeco::obtiene_partidas(eco);
          tx.send(validadas).unwrap();
        });
        let partidas = rx.recv().unwrap();
        
        ArbolECO::arbol_central(&model_centro, &partidas);
        
      }
      sel_cen_clon.set_mode(gtk::SelectionMode::Single);
      
      // el tablero
      let hijos = hbox.get_children();
      if hijos.len() > 2 {
        hijos[2].destroy();
      }
      let fen_inicial = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
      let jugadas = "".to_string();
      tablero::Tablero::init(&hbox, fen_inicial, jugadas);
      hbox.show_all();
    });
    
    // ************ fin del closure del arbol izquierdo *********
    
    // ********** el closure del arbol central ******************
    let weak_hbox = hbox.downgrade();
    seleccion_centro.connect_changed ( move |widget| {
      let hbox = match weak_hbox.upgrade() {
                    Some(hbox) => hbox,
                    None => return,
                };
      
      let hijos = hbox.get_children();
      if hijos.len() > 2 {
        hijos[2].destroy();
      }
      
      if let Some((modelo, iter)) = widget.get_selected() {
        let bruto = modelo.get_value(&iter, 3); // columna 4 las jugadas
        let jugs = bruto.get::<String>().unwrap();
        // el tablero
        let fen_inicial = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        tablero::Tablero::init(&hbox, fen_inicial, jugs);
        hbox.show_all();
      }
      
      
    });
    // ************ fin del closure del arbol central ***********
    
    
  }
  
  fn arbol_central (modelo: &gtk::TreeStore, partidas: &Vec<arbol_subeco::Partida>) {
    modelo.clear();
    
    // creamos un hashmap para almacenar las eco y sus iter correspondientes
    // por si la lista esta desordenada 
    let mut padres = HashMap::new();
    
    for item in partidas {
      let eco = &item.cabecera.sitio;
      let apertura = &item.cabecera.blancas;
      let variante = &item.cabecera.negras;
      let jugadas = &item.jugadas;
      
      let padre = format!("{}{}", eco, apertura);
      
      // necesitamos un clone() por problemas de mutabilecoad en el match
      let padres_clon = padres.clone();
      
      match padres_clon.get(&padre) {
        Some(valor) => {
          let iter1 = modelo.append(Some(valor));
          modelo.set(&iter1, &[0,1,2,3], 
                &[&eco, &apertura,&variante, &jugadas]);
          // Si el mapa no tenía esta clave presente, se devuelve None.
          // Si el mapa tenía presente esta clave, el valor se 
          // actualiza y se devuelve el valor anterior. 
          //Sin embargo, la clave no se actualiza; 
          //padres.insert(padre, iter1);  // -> Option<V> ; V = valor
        },
        None => {
          let iter = modelo.append(None);
          modelo.set(&iter, &[0,1,2,3], 
                &[&eco, &apertura, &variante, &jugadas]);
          padres.insert(padre, iter);
        },
      }
    }
  }
}




