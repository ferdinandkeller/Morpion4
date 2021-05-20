// on importe les modules nécessaires
mod board;
mod minimax;

use std::io::{stdin, stdout, Write};
use std::time::{Instant, Duration};
use std::collections::HashMap;
use board::Board;
use minimax::minimax;

// on définit la profondeur de récursion
const MAX_RECURSION_DEPTH: u8 = 7;

// entrée du programme
fn main() {
  // on demande qui doit commencer
  let starter = get_starting_move();
  if starter {
    println!("Le joueur commence !");
  } else {
    println!("L'ordi commence !");
  }

  // on créer le plateau et une liste de coups déjà joués
  let mut board = Board::new();
  let mut total_time_computing = Duration::new(0, 0);

  // on commence la partie
  let mut turn = starter;
  loop {
    match board.terminal_test() {
      Some(val) => if val != 0 { break; }
      None => break
    }

    if turn {
      let player_move = get_player_move();
      board = board.play(player_move, true);
      println!("{}", board);
      display_move(player_move);
    } else {
      let now = Instant::now();
      let (computer_move, _) = minimax(board, None, MAX_RECURSION_DEPTH, false, &mut HashMap::new());
      let elapsed = now.elapsed();
      total_time_computing += elapsed;
      board = board.play(computer_move, false);
      println!("{}", board);
      println!("temps de calcul : {}s", (elapsed.as_millis() as f64)/1000.0);
      display_move(computer_move);
    }
    turn = !turn;
  }

  // on affiche le vainqueur
  let winner = board.terminal_test();
  match winner {
    Some(val) => {
      if val < 0 {
        println!("L'ordinateur gagne !!");
      } else if 0 < val {
        println!("Le joueur gagne !!");
      }
    },
    None => {
      println!("Match nul !!");
    }
  }
  println!("temps total passé à calculer : {}s", (total_time_computing.as_millis() as f64)/1000.0);
}

// on demande qui doit commencer
fn get_starting_move() -> bool {
  let mut buf;
  let joueur: bool;

  loop {
    buf = String::new();
    print!("À qui l'honneur ? (0: ordi, 1: joueur)\n> ");
    match stdout().flush() {
      Ok(_) => {},
      _ => {}
    }
    match stdin().read_line(&mut buf) {
      Ok(_) => {
        match buf.trim().parse::<u32>() {
          Ok(val) => {
            if val == 0 {
              joueur = false;
              break;
            } else if val == 1 {
              joueur = true;
              break
            }
          },
          _ => {}
        }
      },
      _ => {}
    }
  }
  
  joueur
}

// on demande à un joueur de jouer
fn get_player_move() -> u32 {
  let mut buf;

  let mut x: u32;
  loop {
    buf = String::new();
    print!("col > ");
    match stdout().flush() {
      Ok(_) => {},
      _ => {}
    }
    match stdin().read_line(&mut buf) {
      Ok(_) => {
        match buf.trim().parse() {
          Ok(val) => {
            x = val;
            if 0 < x && x <= 12 {
              break;
            }
          },
          _ => {}
        }
      },
      _ => {}
    }
  }
  
  let mut y: u32;
  loop {
    buf = String::new();
    print!("lig > ");
    match stdout().flush() {
      Ok(_) => {},
      _ => {}
    }
    match stdin().read_line(&mut buf) {
      Ok(_) => {
        match buf.trim().parse() {
          Ok(val) => {
            y = val;
            if 0 < y && y <= 12 {
              break;
            }
          },
          _ => {}
        }
      },
      _ => {}
    }
  }

  12*(y-1) + (x-1)
}

// affiche un coup de manière lisible
fn display_move(m: u32) {
  println!("col: {} · lin: {}\n", (m%12)+1, (m/12)+1);
}