/*
extern crate gtk;
extern crate cairo;
extern crate gdk;
extern crate gdk_pixbuf;
#[macro_use]
extern crate lazy_static;
extern crate mut_static;    // https://github.com/tyleo/mut_static
*/

use gtk::*;
use gdk::prelude::*;
//use gdk::ContextExt;
use mut_static::MutStatic;
use std::mem;
use std::ops::DerefMut;
use std::collections::HashMap;


use super::ajedrez;
use super::parser;




const FILAS: i32 = 8;
const COLUMNAS: i32 = 8;
const Y_EJE: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const X_EJE: [i32; 8] = [1,2,3,4,5,6,7,8];


struct VariablesTablero {
  color1: (f64, f64, f64),
  color2: (f64, f64, f64),
  dim_square: f64,
  dir_piezas: String,
  fen_inicial: String,
  fen_actual: String,
  tablero_interno: HashMap<String, String>,
  flipped: bool,
}


lazy_static! {
    static ref VAR_TABLERO: MutStatic<VariablesTablero> = MutStatic::new();
}


pub struct Tablero {
  // window: gtk::Window,
}

impl Tablero {
  //
  pub fn init(hbox: &gtk::Box, fen_inicial: &str, jugadas: String)   {
    
    // Variables del Tablero
    let volteado = false;
    let mut board = ajedrez::Tablero::init();
    let _fen_valida = ajedrez::set_fen(fen_inicial, &mut board);
    let fen_actual = ajedrez::get_fen(&mut board);
    let grafico = ajedrez::tablero_grafico(&mut board);
    let tablero_interno = procesa_notacion(grafico, volteado);
    
    
    // inicializamos las variables globales del tablero_grafico
    lazy_static::initialize(&VAR_TABLERO);
    
    // a cada llamada a este modulo nos encontramos que VAR_TABLERO
    // esta ya inicilizada y arroja un error al hacer .set()
    // entonces la desreferenciamos y colocamos en el mismo sitio_piezas
    // de memoria la reinicializacion
    
    let mut obj_tab: mut_static::ForceSomeRwLockWriteGuard<VariablesTablero>;
    let correcto = VAR_TABLERO.set( VariablesTablero {
      color2: (221.0 / 255.0, 184.0 / 255.0, 140.0 / 255.0),
      color1: (166.0 / 255.0, 109.0 / 255.0, 79.0 / 255.0),
      dim_square: 45.0,
      dir_piezas: "./piezas/Merida96/".to_string(),
      fen_inicial: fen_inicial.to_string(),
      fen_actual: fen_actual.clone(),
      tablero_interno: tablero_interno.clone(),
      flipped : volteado,
    });   //.unwrap();
    
    match correcto {
      Ok(_correcto) => {obj_tab = VAR_TABLERO.write().unwrap();}, //correcto,
      Err(_err) => {
            obj_tab = VAR_TABLERO.write().unwrap();
            
            mem::replace(obj_tab.deref_mut(), VariablesTablero {
              color2: (221.0 / 255.0, 184.0 / 255.0, 140.0 / 255.0),
              color1: (166.0 / 255.0, 109.0 / 255.0, 79.0 / 255.0),
              dim_square: 45.0,
              dir_piezas: "./piezas/Merida96/".to_string(),
              fen_inicial: fen_inicial.to_string(),
              fen_actual: fen_actual.clone(),
              tablero_interno: tablero_interno.clone(),
              flipped : volteado,
            });
      },
    };
    
    
    let lista_piezas = crea_lista_piezas(obj_tab.dir_piezas.clone());
    
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.set_size_request(160,160);
    
    let frm = gtk::AspectFrame::new(None, 0.0, 0.0, 5.0, true);
    let area_tablero = gtk::DrawingArea::new();
    area_tablero.set_hexpand(true);
    frm.add(&area_tablero);
    vbox.pack_start(&frm, true, true, 0);
    
    // =======================================
    // parseamos las jugadas
    let partida = parser::procesa_jugadas(jugadas, obj_tab.fen_inicial.clone());
    // =======================================
    
    // ahora el texto de la partida y la ventana con scroll
    let scrolledwindow = gtk::ScrolledWindow::new(
          gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scrolledwindow.set_policy(
          gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    let visor_partida = gtk::TextView::new();
    visor_partida.set_wrap_mode(gtk::WrapMode::Word);
    let textbuffer = visor_partida.get_buffer().unwrap();
    Tablero::crea_texto_partida (
          &visor_partida, &textbuffer, partida, &area_tablero);
    
    // un closure para cambiar el cursor del visor de partida
    visor_partida.connect_motion_notify_event (move |widget, _eventmotion| {
      let display = gdk::Display::get_default()
              .expect("error en display");
      let gcursor = gdk::Cursor::new_for_display(&display, 
              gdk::CursorType::Hand1);
      let gwindow = gtk::TextViewExt::get_window(widget, 
              gtk::TextWindowType::Text)
              .expect("error en gwindow");
      gdk::WindowExt::set_cursor(&gwindow, Some(&gcursor));
      Inhibit(false)
    });
    
    scrolledwindow.add(&visor_partida);
    vbox.pack_start(&scrolledwindow, true, true, 0);
    hbox.pack_start(&vbox, true, true, 10);
    
    
    // el dibujo del tablero
    area_tablero.connect_draw ( move |widget, ctx| {
      let mut obj_tab = VAR_TABLERO.write().unwrap();
      let mut color = obj_tab.color2;
      obj_tab.dim_square = (widget.get_allocated_width() / 8) as f64;
      // el padding
      let leftover_space = widget.get_allocated_width() as f64 - 
            obj_tab.dim_square * 8.0;
      let padding = leftover_space / 2.0;
      cairo::Context::translate(ctx, padding as f64, padding as f64);
      
      // el tablero
      for r in 0..FILAS {
        if color == obj_tab.color2 {
          color = obj_tab.color1;
        } else { color = obj_tab.color2; }
        for c in 0..COLUMNAS {
          let x1 = c as f64 * obj_tab.dim_square;
          let y1 = (7-r) as f64 * obj_tab.dim_square;
          ctx.set_source_rgb(color.0, color.1, color.2);
          ctx.rectangle(x1, y1, obj_tab.dim_square, obj_tab.dim_square);
          ctx.fill();
          if color == obj_tab.color2 {
            color = obj_tab.color1;
          } else { color = obj_tab.color2; }
        }
      }
      
      // las piezas
      for (xycoord, valor) in &obj_tab.tablero_interno {
        let (x, y) = num_notacion(xycoord);
        
        let x0 = (y as f64 * obj_tab.dim_square) + 
              (obj_tab.dim_square/2.0) - 
              (45.0 * obj_tab.dim_square) / 100.0 ;
        let y0 = ((7-x) as f64 * obj_tab.dim_square) + 
              (obj_tab.dim_square/2.0) - 
              (45.0 * obj_tab.dim_square) / 100.0 ;
        
        let pieza = lista_piezas.get(valor)
              .expect("error al obtener la pieza");
        let pixbuf = pieza.scale_simple (
              (obj_tab.dim_square * 0.90) as i32,
              (obj_tab.dim_square * 0.90) as i32,
              gdk_pixbuf::InterpType::Bilinear
              ).expect("error al escalar pixbuf");
        let _sr1 = ctx.set_source_pixbuf(&pixbuf, x0, y0);
        
        ctx.paint();
      }
      
      Inhibit(false)
    });
  }
  
  
  fn crea_link <'a>(texto: &'a str, 
          argumento: &'a str,
          d_area: &gtk::DrawingArea) -> gtk::EventBox {
      let label = gtk::Label::new(None);
      
      let texto1: String = texto.to_string();
      /*
      // this transforms into unicode figurines
      if texto1.contains("K") {
        texto1 = texto1.replace("K", "\u{2654}" );
      }
      if texto1.contains("Q") {
        texto1 = texto1.replace("Q", "\u{2655}" );
      }
      if texto1.contains("R") {
        texto1 = texto1.replace("R", "\u{2656}" );
      }
      if texto1.contains("B") {
        texto1 = texto1.replace("B", "\u{2657}" );
      }
      if texto1.contains("N") {
        texto1 = texto1.replace("N", "\u{2658}" );
      }
      */
      let texto_etiq = format!(
            "{}{}{}", "<span color=\"blue\">", texto1, "</span>");
      label.set_markup(&texto_etiq);
      
      let eventbox = gtk::EventBox::new();
      eventbox.add(&label);
      let arg_clon = argumento.to_string().clone();
      
      let weak_area = d_area.downgrade();
    
      eventbox.connect_button_press_event(move |_widget, _btn_event| {
        let d_area = match weak_area.upgrade() {
                    Some(d_area) => d_area,
                    None => return Inhibit(true),
                };
        Tablero::muestra_arg_link(&arg_clon, &d_area);
        Inhibit(true)
      });
      eventbox
  }
  
  fn muestra_arg_link (arg: &str, d_area: &gtk::DrawingArea) {
    let mut obj_tab = VAR_TABLERO.write().unwrap();
    
    let mut board = ajedrez::Tablero::init();
    let fen_valida = ajedrez::set_fen(arg, &mut board);
    if fen_valida {
      let grafico = ajedrez::tablero_grafico(&mut board);
      let tablero_interno = procesa_notacion(grafico, obj_tab.flipped);
      
      obj_tab.fen_actual = arg.to_string();
      obj_tab.tablero_interno = tablero_interno;
    }
    d_area.queue_draw();
  }
  
  pub fn crea_texto_partida (visor: &gtk::TextView,
            buffer: &gtk::TextBuffer,
            partida: Vec<Vec<parser::MoveT>>,
            d_area: &gtk::DrawingArea) {
    let mut num_jugada = 0;
    visor.set_editable(true);
    for movim in partida[0].clone() {
      if movim.turno == "w".to_string() {
        num_jugada += 1;
        let num_text = format!("{}. ", num_jugada);
        // en lugar de texto insertamos una etiqueta
        let lbl_num = gtk::Label::new(Some(&num_text));
        let mut iter = buffer.get_end_iter();
        let ancla= buffer.create_child_anchor(&mut iter)
              .expect("error en ancla");
        visor.add_child_at_anchor(&lbl_num, &ancla);
      }
      //buffer.insert_at_cursor(&movim.san);
      let enlace = Tablero::crea_link(&movim.san, &movim.fen, &d_area);
      let mut iter = buffer.get_end_iter();
      let ancla= buffer.create_child_anchor(&mut iter)
            .expect("error en ancla");
      visor.add_child_at_anchor(&enlace, &ancla);
      
      buffer.insert_at_cursor(" ");
    }
    visor.set_editable(false);
  }
}


/*
 funciones para crear un tablero interno del tablero grafico
 ===========================================================
*/

fn procesa_notacion(arr_piezas: Vec<String>,
        flipped: bool) -> HashMap<String, String> {
  
  let mut tablero: HashMap<String, String> = HashMap::new();
  let mut grafico: Vec<Vec<String>> = Vec::new();
  let mut temporal: Vec<String> = Vec::new();
  let mut sitio_piezas = arr_piezas.clone();
  
  if flipped {
    //sitio_piezas = arr_piezas.iter().rev().collect();
    sitio_piezas.reverse();
  }
  
  // ahora hacemos un array bidimensional
  for i in 0..8 {
    temporal.push(sitio_piezas[i].clone());
  }
  grafico.push(temporal);
  temporal = Vec::new();
  for i in 8..16 {
    temporal.push(sitio_piezas[i].clone());
  }
  grafico.push(temporal);
  temporal = Vec::new();
  for i in 16..24 {
    temporal.push(sitio_piezas[i].clone());
  }
  grafico.push(temporal);
  temporal = Vec::new();
  for i in 24..32 {
    temporal.push(sitio_piezas[i].clone());
  }
  grafico.push(temporal);
  temporal = Vec::new();
  for i in 32..40 {
    temporal.push(sitio_piezas[i].clone());
  }
  grafico.push(temporal);
  temporal = Vec::new();
  for i in 40..48 {
    temporal.push(sitio_piezas[i].clone());
  }
  grafico.push(temporal);
  temporal = Vec::new();
  for i in 48..56 {
    temporal.push(sitio_piezas[i].clone());
  }
  grafico.push(temporal);
  temporal = Vec::new();
  for i in 56..64 {
    temporal.push(sitio_piezas[i].clone());
  }
  grafico.push(temporal);
  
  for fila in 0..grafico.len() {
    for col in 0..grafico[fila].len() {
      let alfabeto = &grafico[fila][col];
      if *alfabeto == "-".to_string() {
        continue;
      }
      let xycoord = alfa_notacion((7-fila, col));
      if xycoord != "None" {
        tablero.insert(xycoord, alfabeto.to_string());
      }
    }
  }
  tablero
}

// Necesitamos una manera de convertir las coordenadas x e y de una pieza 
// a su notación equivalente alfabética, por ejemplo, A1, D5, E3, etc.
fn alfa_notacion (xycoord: (usize, usize)) -> String {
  if !esta_en_tablero(xycoord) {
    return "None".to_string();
  }
  return format!("{}{}", Y_EJE[xycoord.1], X_EJE[xycoord.0])
}

// la definición de un método para comprobar si una determinada
// coordenada está en el tablero
fn esta_en_tablero(coord: (usize, usize)) -> bool {
    //if coord.1 < 0 || coord.1 > 7 || coord.0 < 0 || coord.0 > 7 {
    if coord.1 > 7 || coord.0 > 7 {
        return false;
    }
    else { return true; }
    //false
}

// Necesitamos convertir una notacion a1, a8, etc a coordenadas x,y
// definimos un método que toma una coordenada x, y como una tupla y 
// devuelve su notación numérica equivalente, de la siguiente manera:
fn num_notacion(xycoord: &str) -> (usize, usize) {
  let car = xycoord.chars().nth(0).unwrap();
  let num_car = xycoord.chars().nth(1).unwrap();
  let col = Y_EJE.iter().position(|&x| x == car)
        .expect("error al obtener el num de col."); // Option<usize>
  let fila = (num_car.to_string()).parse::<usize>().unwrap() - 1;
  
  (fila, col)
}

/*
Fin de las funciones del tablero interno
*/ 

/*
Crea el mapa de piezas
======================
*/

fn crea_lista_piezas (directorio: String) -> HashMap<String,  gdk_pixbuf::Pixbuf>{
  let mut piezas: HashMap<String,  gdk_pixbuf::Pixbuf> = HashMap::new();
  
  // las piezas negras
  // alfil negro
  let mut pieza = "b".to_string();
  let mut nom_fichero = format!("{}b{}.png", directorio, pieza.to_lowercase());
  let mut pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // rey negro
  pieza = "k".to_string();
  nom_fichero = format!("{}b{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // caballo negro
  pieza = "n".to_string();
  nom_fichero = format!("{}b{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // peon negro
  pieza = "p".to_string();
  nom_fichero = format!("{}b{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // dama negra
  pieza = "q".to_string();
  nom_fichero = format!("{}b{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // torre negra
  pieza = "r".to_string();
  nom_fichero = format!("{}b{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  
  // las piezas blancas
  // alfil blanco
  pieza = "B".to_string();
  nom_fichero = format!("{}w{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // rey blanco
  pieza = "K".to_string();
  nom_fichero = format!("{}w{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // caballo blanco
  pieza = "N".to_string();
  nom_fichero = format!("{}w{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // peon blanco
  pieza = "P".to_string();
  nom_fichero = format!("{}w{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // dama blanca
  pieza = "Q".to_string();
  nom_fichero = format!("{}w{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  // torre blanca
  pieza = "R".to_string();
  nom_fichero = format!("{}w{}.png", directorio, pieza.to_lowercase());
  pixbuf = pieza_pixbuf(nom_fichero);
  piezas.insert(pieza, pixbuf);
  
  
  piezas
}

fn pieza_pixbuf (nom_fichero: String) -> gdk_pixbuf::Pixbuf {
  let pixbuf = gdk_pixbuf::Pixbuf::new_from_file (
      nom_fichero
    ).expect("error al obtener pixbuf");
    
  pixbuf
}

