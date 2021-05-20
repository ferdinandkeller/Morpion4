// on utilise des u256 afin de pouvoir stoquer une grille de 12*12=144 (un u128 classique est trop petit)
// un 0 signifie qu'il n'y a rien, et un 1 signifie qu'il y a un joueur
use ethnum::*;

// on définit les masques une fois pour toutes
const fn clean_u256(hi: u128, lo: u128) -> U256 {
  let hi_: u128 = hi>>56;
  let lo_: u128 = lo + (hi<<72);
  U256::from_words(hi_, lo_)
}

const MASKS: [u256; 13] = [
  // masques vérification gagnant
  clean_u256( // gauche
    0b111111111110111111111110111111111110111111111110111111111110111111111110,
    0b111111111110111111111110111111111110111111111110111111111110111111111110),
  clean_u256(
    0b111111111100111111111100111111111100111111111100111111111100111111111100, 0b111111111100111111111100111111111100111111111100111111111100111111111100),
  clean_u256(
    0b111111111000111111111000111111111000111111111000111111111000111111111000, 0b111111111000111111111000111111111000111111111000111111111000111111111000),

  clean_u256( // haut gauche
    0b111111111110111111111110111111111110111111111110111111111110111111111110,
    0b111111111110111111111110111111111110111111111110111111111110000000000000),
  clean_u256(
    0b111111111100111111111100111111111100111111111100111111111100111111111100,
    0b111111111100111111111100111111111100111111111100000000000000000000000000),
  clean_u256(
    0b111111111000111111111000111111111000111111111000111111111000111111111000,
    0b111111111000111111111000111111111000000000000000000000000000000000000000),

  clean_u256( // haut droit
    0b011111111111011111111111011111111111011111111111011111111111011111111111,
    0b011111111111011111111111011111111111011111111111011111111111000000000000),
  clean_u256(
    0b001111111111001111111111001111111111001111111111001111111111001111111111,
    0b001111111111001111111111001111111111001111111111000000000000000000000000),
  clean_u256(
    0b000111111111000111111111000111111111000111111111000111111111000111111111,
    0b000111111111000111111111000111111111000000000000000000000000000000000000),

  // marques coups intéressants
  clean_u256( // droite
    0b011111111111011111111111011111111111011111111111011111111111011111111111,
    0b011111111111011111111111011111111111011111111111011111111111011111111111),
    
  clean_u256( // bas
    0b000000000000111111111111111111111111111111111111111111111111111111111111,
    0b111111111111111111111111111111111111111111111111111111111111111111111111),
    
  clean_u256( // bas droit
    0b000000000000011111111111011111111111011111111111011111111111011111111111,
    0b011111111111011111111111011111111111011111111111011111111111011111111111),

  clean_u256( // bas gauche
    0b000000000000111111111110111111111110111111111110111111111110111111111110,
    0b111111111110111111111110111111111110111111111110111111111110111111111110),
];

const MOVES_ORDER: [u32; 144] = [
  77, 65, 66, 78, 90, 89, 88, 76, 64, 52, 53, 54, 55, 67, 79, 91, 103, 102, 101, 100, 99, 87, 75, 63, 51, 39, 40, 41, 42, 43, 44, 56, 68, 80, 92, 104, 116, 115, 114, 113, 112, 111, 110, 98, 86, 74, 62, 50, 38, 26, 27, 28, 29, 30, 31, 32, 33, 45, 57, 69, 81, 93, 105, 117, 129, 128, 127, 126, 125, 124, 123, 122, 121, 109, 97, 85, 73, 61, 49, 37, 25, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 34, 46, 58, 70, 82, 94, 106, 118, 130, 142, 141, 140, 139, 138, 137, 136, 135, 134, 133, 132, 120, 108, 96, 84, 72, 60, 48, 36, 24, 12, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 23, 35, 47, 59, 71, 83, 95, 107, 119, 131, 143
];

const MOVES_MASKS: [U256; 144] = moves_masks();
const fn moves_masks() -> [U256; 144] {
  const fn pow(n: u32) -> u256 {
    if n < 128 {
      U256::from_words(0, 1 << n)
    } else {
      U256::from_words(1 << (n-128), 0)
    }
  }

  let mut v: [U256; 144] = [U256::ZERO; 144];
  let mut i: usize = 0;
  while i < 144 {
    v[i] = pow(MOVES_ORDER[i]);
    i += 1;
  }
  v
}

const NO_MORE_MOVES: U256 = U256::from_words(
  0b1111111111111111,
  0b11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111);

// on définit la structure de la grille
// on s'assure aussi que la struct puisse être hachée,
// c'est nécessaire pour utiliser .contains_key()
#[derive(Hash)]
pub struct Board {
  pub players: u256,
  pub mask: u256,
}

// on s'assure qu'elle puisse être copiée sur le stack directement
impl Copy for Board {}
impl Clone for Board {
  fn clone(&self) -> Board {
    Board {
      players: self.players,
      mask: self.mask,
    }
  }
}

// on s'asure que l'objet puisse être comparé
impl std::cmp::PartialEq for Board {
  fn eq(&self, other: &Board) -> bool {
    self.players == other.players && self.mask == other.mask
  }
}
impl std::cmp::Eq for Board {
}

// on s'assure qu'elle puisse être affichée correctement
impl std::fmt::Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut board_as_string = String::from("┏━━━━━ MORPION 12x12 ━━━━━┓\n");
    let mut players = self.players;
    let mut mask = self.mask;
    for _ in 0..12 {
      board_as_string += "┃ ";
      for _ in 0..12 {
        if players&1 == 1 {
          if mask&1 == 1 {
            board_as_string += "● ";
          } else {
            board_as_string += "○ ";
          }
        } else {
          board_as_string += "· ";
        }
        players >>= 1;
        mask >>= 1;
      }
      board_as_string += "┃\n";
    }
    write!(f, "{}", board_as_string + "┗━━━━━━━━━━━━━━━━━━━━━━━━━┛")
  }
}

// on implémente les méthodes nécessaires
impl Board {
  // création d'une nouvelle grille
  pub fn new() -> Self {
    Self {
      players: U256::ZERO,
      mask: U256::ZERO
    }
  }

  // on check si un coup est valide
  pub fn is_play_valid(self, position: u32) -> bool {
    position < 144 && ((self.players & U256::ONE<<position) == 0)
  }

  // on effectue un coup
  pub fn play(mut self, position: u32, player: bool) -> Board {
    if self.is_play_valid(position) {
      let delta: u256 = U256::ONE << position;
      self.players = self.players | delta;
      if player {
        self.mask = self.mask | delta;
      }
    } else {
      panic!("cette position n'est pas valide !!!");
    }
    self
  }

  // on check si il ya un gagnant
  pub fn is_winner(self, player: bool) -> bool {
    // on calcule la grille joueur
    let mut p: u256 = self.players;
    if player { p &= self.mask; } else { p &= !self.mask; };

    // check vertical
    if  (p)
      & (p >> 12)
      & (p >> 24)
      & (p >> 36) != U256::ZERO { return true; }

    // check horizontal
    if  (p)
      & ((p&MASKS[0]) >> 1)
      & ((p&MASKS[1]) >> 2)
      & ((p&MASKS[2]) >> 3) != U256::ZERO { return true; }

    // check diago 1  <^
    if  (p)
      & ((p&MASKS[3]) >> 13)
      & ((p&MASKS[4]) >> 26)
      & ((p&MASKS[5]) >> 39) != U256::ZERO { return true; }
      
    // check diago 2 ^>
    if  (p)
      & ((p&MASKS[6]) >> 11)
      & ((p&MASKS[7]) >> 22)
      & ((p&MASKS[8]) >> 33) != U256::ZERO { return true; }

    false
  }

  // on vérifie si il y deux jeutons à côtés qui se touchent sans voisins
  pub fn danger_2(self, player: bool) -> bool {
    // on calcule la grille joueur
    let mut p: u256 = self.players;
    if !player { p &= self.mask; } else { p &= !self.mask; };

    // check horizontal
    let h: u256 = 
        (p)
      & ((p&MASKS[0]) >> 1)
      & (((!self.players)&MASKS[1]) >> 2)
      & (((!self.players)&MASKS[9]) << 1);
    //  | (
    //     (p)
    //   & ((p&MASKS[1]) >> 2)
    //   & (((!self.players)&MASKS[0]) >> 1)
    //   & (  (((!self.players)&MASKS[2]) >> 3) | (((!self.players)&MASKS[9]) << 1)  )
    // );

    // check vertical
    let v: u256 = 
        (p)
      & (p>>12)
      & (!self.players >> 24)
      & (((!self.players)&MASKS[10]) << 12);
    //  | (
    //     (p)
    //   & (p>>24)
    //   & (!self.players >> 12)
    //   & (  ((!self.players) >> 36) | (((!self.players)&MASKS[10]) << 12)  )
    // );

    // check diago 1 <^
    let d1 = 
        (p)
      & ((p&MASKS[3]) >> 13)
      & (((!self.players)&MASKS[4]) >> 26)
      & (((!self.players)&MASKS[11]) << 13);
    //  | (
    //     (p)
    //   & ((p&MASKS[4]) >> 26)
    //   & (((!self.players)&MASKS[3]) >> 13)
    //   & (  (((!self.players)&MASKS[5]) >> 39) | (((!self.players)&MASKS[11]) << 13)  )
    // );

    // check diago 2 ^>
    let d2 = 
        (p)
      & ((p&MASKS[6]) >> 11)
      & (((!self.players)&MASKS[7]) >> 22)
      & (((!self.players)&MASKS[12]) << 11);
    //  | (
    //     (p)
    //   & ((p&MASKS[7]) >> 22)
    //   & (((!self.players)&MASKS[6]) >> 11)
    //   & (  (((!self.players)&MASKS[8]) >> 33) | (((!self.players)&MASKS[12]) << 11)  )
    // );

    // combined
    h | v | d1 | d2 != U256::ZERO
  }
  pub fn bonus_blocage_2(self, m: u32, player: bool) -> bool {
    // on calcule la grille joueur
    let mut p: u256 = self.players;
    if !player { p &= self.mask; } else { p &= !self.mask; };

    // fonction qui vérifie si 2 pions ennemis alignés avec vide à côté
    fn check(board: Board, p: u256) -> u256 {
      // check horizontal
      let h: u256 = 
          (p)
        & ((p&MASKS[0]) >> 1)
        & (((!board.players)&MASKS[1]) >> 2)
        & (((!board.players)&MASKS[9]) << 1);
      //  | (
      //     (p)
      //   & ((p&MASKS[1]) >> 2)
      //   & (((!board.players)&MASKS[0]) >> 1)
      //   & (  (((!board.players)&MASKS[2]) >> 3) | (((!board.players)&MASKS[9]) << 1)  )
      // );

      // check vertical
      let v: u256 = 
          (p)
        & (p>>12)
        & (!board.players >> 24)
        & (((!board.players)&MASKS[10]) << 12);
      //  | (
      //     (p)
      //   & (p>>24)
      //   & (!board.players >> 12)
      //   & (  ((!board.players) >> 36) | (((!board.players)&MASKS[10]) << 12)  )
      // );

      // check diago 1 <^
      let d1 = 
          (p)
        & ((p&MASKS[3]) >> 13)
        & (((!board.players)&MASKS[4]) >> 26)
        & (((!board.players)&MASKS[11]) << 13);
      //  | (
      //     (p)
      //   & ((p&MASKS[4]) >> 26)
      //   & (((!board.players)&MASKS[3]) >> 13)
      //   & (  (((!board.players)&MASKS[5]) >> 39) | (((!board.players)&MASKS[11]) << 13)  )
      // );

      // check diago 2 ^>
      let d2 = 
          (p)
        & ((p&MASKS[6]) >> 11)
        & (((!board.players)&MASKS[7]) >> 22)
        & (((!board.players)&MASKS[12]) << 11);
      //  | (
      //     (p)
      //   & ((p&MASKS[7]) >> 22)
      //   & (((!board.players)&MASKS[6]) >> 11)
      //   & (  (((!board.players)&MASKS[8]) >> 33) | (((!board.players)&MASKS[12]) << 11)  )
      // );

      // combined
      h | v | d1 | d2
    }

    check(self, p) != check(self.play(m, player), p)
  }

  // on vérifie si il y trois jeutons à côtés qui se touchent avec un voisin vide
  pub fn danger_3(self, player: bool) -> bool {
    // on calcule la grille joueur
    let mut p: u256 = self.players;
    if !player { p &= self.mask; } else { p &= !self.mask; };

    // check horizontal
    let h: u256 = (p)
      & ((p&MASKS[0]) >> 1)
      & ((p&MASKS[1]) >> 2)
      & ((((!self.players)&MASKS[2]) >> 3) | (((!self.players)&MASKS[9]) << 1));

    // check vertical
    let v: u256 = (p)
      & (p>>12)
      & (p>>24)
      & ((!self.players >> 36) | (((!self.players)&MASKS[10]) << 12));

    // check diago 1 <^
    let d1 = (p)
      & ((p&MASKS[3]) >> 13)
      & ((p&MASKS[4]) >> 26)
      & ((((!self.players)&MASKS[5]) >> 39) | (((!self.players)&MASKS[11]) << 13));

    // check diago 2 ^>
    let d2 = (p)
      & ((p&MASKS[6]) >> 11)
      & ((p&MASKS[7]) >> 22)
      & ((((!self.players)&MASKS[8]) >> 33) | (((!self.players)&MASKS[12]) << 11));

    // combined
    h | v | d1 | d2 != U256::ZERO
  }
  pub fn bonus_blocage_3(self, m: u32, player: bool) -> bool {
    // on calcule la grille joueur
    let mut p: u256 = self.players;
    if !player { p &= self.mask; } else { p &= !self.mask; };

    // fonction qui vérifie si 2 pions ennemis alignés avec vide à côté
    fn check(board: Board, p: u256) -> u256 {
      // check horizontal
      let h: u256 = (p)
        & ((p&MASKS[0]) >> 1)
        & ((p&MASKS[1]) >> 2)
        & ((((!board.players)&MASKS[2]) >> 3) ^ (((!board.players)&MASKS[9]) << 1));

      // check vertical
      let v: u256 = (p)
        & (p>>12)
        & (p>>24)
        & ((!board.players >> 36) ^ (((!board.players)&MASKS[10]) << 12));

      // check diago 1 <^
      let d1 = (p)
        & ((p&MASKS[3]) >> 13)
        & ((p&MASKS[4]) >> 26)
        & ((((!board.players)&MASKS[5]) >> 39) ^ (((!board.players)&MASKS[11]) << 13));

      // check diago 2 ^>
      let d2 = (p)
        & ((p&MASKS[6]) >> 11)
        & ((p&MASKS[7]) >> 22)
        & ((((!board.players)&MASKS[8]) >> 33) ^ (((!board.players)&MASKS[12]) << 11));

      // combined
      h | v | d1 | d2
    }

    check(self, p) != check(self.play(m, player), p)
  }

  // on calcule la valeur gagnante de la position
  pub fn terminal_test(self) -> Option<i128> {
    if self.players == NO_MORE_MOVES {
      return None;
    } else {
      if self.is_winner(false) {
        return Some(-(1<<126));
      } else if self.is_winner(true) {
        return Some(1<<126);
      }
      return Some(0);
    }
  }

  // on calcule une estimation de la position
  pub fn state_value(self) -> i128 {
    if self.is_winner(false) {
      return -(1<<126);
    } else if self.is_winner(true) {
      return 1<<126;
    } else if self.danger_3(true) {
      return -(1<<125);
    } else if self.danger_3(false) {
      return 1<<125;
    } else if self.danger_2(true) {
      return -(1<<124);
    } else if self.danger_2(false) {
      return 1<<124;
    }
    0
  }

  // on calcule à quel point un coup est proche des autres
  pub fn move_proximity(self, m: u32, player: bool) -> u32 {
    // on calcule la grille joueur
    let mut p1: u256 = self.players;
    let mut p2: u256 = self.players;
    if player { p1 &= self.mask; p2 &= !self.mask; } else { p1 &= !self.mask; p2 &= self.mask; };

    let mut tot: u32 = 0;
    let mut one = U256::ONE;
    let _row = m/12;
    let _col = m%12;
    
    for row in 0..12 {
      for col in 0..12 {
        if p1&one != U256::ZERO {
          tot += 4*(col-_col)*(col-_col) + 4*(row-_row)*(row-_row);
        }
        one <<= 1;
      }
    }

    let neighbours = ((p2&p2>>1 != U256::ZERO) as u32) + ((p2&p2<<1 != U256::ZERO) as u32) + ((p2&p2>>12 != U256::ZERO) as u32) + ((p2&p2<<12 != U256::ZERO) as u32);

    if self.play(m, !player).danger_3(player) && neighbours>1 {
      tot = 4-neighbours;
    }

    tot
  }

  // on obtient la liste des coups intéressants
  pub fn get_move_candidates(self) -> Vec<u32> {
    let mut players = self.players;
    let mut moves: Vec<u32> = Vec::new();
  
    if players == U256::ZERO {
      players = U256::ONE << 77;
    } else {
      players = players 
        | ((players&MASKS[0]) >>  1) | ((players&MASKS[9])  <<  1)
        | (players            >> 12) | ((players&MASKS[10]) << 12)
        | ((players&MASKS[6]) >> 11) | ((players&MASKS[12]) << 11)
        | ((players&MASKS[3]) >> 13) | ((players&MASKS[11]) << 13);
      players &= !self.players;
    }
  
    for i in 0..144 {
      if (players & MOVES_MASKS[i]) != 0 {
        moves.push(MOVES_ORDER[i]);
      }
    }

    moves
  }
}