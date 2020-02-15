use const_eco;

#[derive(Clone)]
pub struct Cabecera {
  pub sitio: String,
  pub blancas: String,
  pub negras: String,
}

#[derive(Clone)]
pub struct Partida {
  pub cabecera: Cabecera,
  pub jugadas: String
}



pub fn obtiene_partidas (eco: String) -> Vec<Partida> {
  let mut cabecera: Cabecera = Cabecera {
    sitio: "".to_string(),
    blancas: "".to_string(),
    negras: "".to_string(),
  };
  let mut arr_partidas: Vec<Partida> = Vec::new();
  let mut hay_cabecera = false;
  let mut cuerpo_jugadas = "".to_string();
  let pgn_txt: &str;
  match eco.as_str() {
    "A00" => pgn_txt = const_eco::A_PGN,
    "B00" => pgn_txt = const_eco::B_PGN,
    "C00" => pgn_txt = const_eco::C_PGN,
    "D00" => pgn_txt = const_eco::D_PGN,
    "E00" => pgn_txt = const_eco::E_PGN,
    _ => pgn_txt = "",
  };
  
  let lineas_texto_pgn: Vec<&str> = pgn_txt.split("\n\n").collect();
  
  for i in 0..lineas_texto_pgn.len() {
    let mut cab_clon = cabecera.clone();
    if lineas_texto_pgn[i].starts_with("[") {
      // por si acaso, las lineas finales en blanco
      if lineas_texto_pgn[i].len()== 0 {
        break;
      }
      if !hay_cabecera {
        // la cabecera es [Event ...]\n[Site...] etc
        cabecera = arregla_cabecera(lineas_texto_pgn[i]);
        cab_clon = cabecera.clone();
        hay_cabecera = true;
      }
    }
    else {
      if hay_cabecera {
        cuerpo_jugadas = lineas_texto_pgn[i].to_string();
        let p = Partida {
            cabecera : cab_clon,
            jugadas : cuerpo_jugadas,
        };
        arr_partidas.push(p);
        hay_cabecera = false;
      }
    }
  }
  
  arr_partidas
}

fn arregla_cabecera (cabeza: &str) -> Cabecera {
  let mut elems: Vec<&str> = cabeza.split("\n").collect();
  
  if elems.len() < 3 {
    elems.push("[Black \" \"]");
  }
  
  for i in 0..elems.len() {
    let ele: Vec<&str> = elems[i].split("\"").collect();
    //println!("{}", ele[1]);
    elems[i] = ele[1].clone();
  }
  
  Cabecera {
    sitio: elems[0].to_string(),
    blancas: elems[1].to_string(),
    negras: elems[2].to_string(),
  }
}