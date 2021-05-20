// on importe les modules nécessaires
use std::collections::HashMap;
use crate::board::Board;

// ALGORITHME MINIMAX
pub fn minimax(
  board: Board,
  previous_best_score: Option<i128>,
  cur_rec_depth: u8,
  player: bool,
  known_moves: &mut HashMap<Board, (u32, i128)>
) -> (u32, i128) {
  // si on est sur une feuille (ie au bout de l'arbre)
  if cur_rec_depth == 0 {
    // on calcule la valeur de la feuille
    return (77, board.state_value()); // meilleur coup et valeur

  // si on n'est pas sur une feuille
  } else {
    // on vérifie qu'on ne connait pas déjà cette situation
    if known_moves.contains_key(&board) {
      match known_moves.get(&board) {
        Some(val) => return *val,
        None => panic!("erreur d'accès à un coup enregistré") 
      }
    }
    
    let mut defined = false;
    let mut best_move: u32 = 77;
    let mut best_score: i128 = 0;
    let mut best_move_proximity: u32 = 0;
    // pour chaque coup intelligent (quitter si plus de coups possibles)
    let move_candidates = board.get_move_candidates();
    if move_candidates.len() == 1 {
      return (move_candidates[0], 0);
    }
    // on check si il y a deux pions adverses alignés
    let in_danger_2 = board.danger_2(player) && !board.danger_2(!player);
    let in_danger_3 = board.danger_3(player) && !board.danger_3(!player);
    // pour chaque coup
    for m in move_candidates {
      // on check si situation critique
      if (board.danger_3(!player) && !board.bonus_blocage_3(m, !player)) || (in_danger_3 && !board.bonus_blocage_3(m, player)) || (!in_danger_3 && in_danger_2 && !board.bonus_blocage_2(m, player) && !board.play(m, player).danger_3(!player)) {
        continue;
      }
      // on joue le coup
      let new_board = board.play(m, player);
      // si coup gagnant, on va pas chercher plus loin
      let wining_grid_value = new_board.terminal_test().unwrap();
      if wining_grid_value != 0 {
        known_moves.insert(board, (m, wining_grid_value));
        return (m, wining_grid_value);
      }
      // sinon on va chercher plus loin
      let (_, new_score) = minimax(new_board, if !defined { None } else { Some(best_score) }, cur_rec_depth-1, !player, known_moves);
      if !defined || (player && new_score > best_score) || (!player && new_score < best_score) || (new_score == best_score && board.move_proximity(m, player) < best_move_proximity) {
        defined = true;
        best_score = new_score;
        best_move = m;
        best_move_proximity = board.move_proximity(m, player);
      }
      // élagage alpha beta
      if let Some(pbs) = previous_best_score {
        if (player && best_score > pbs) || (!player && best_score < pbs) {
          return (best_move, best_score/2);
        }
      }
    }
    known_moves.insert(board, (best_move, best_score));
    return (best_move, best_score/2);
  }
}