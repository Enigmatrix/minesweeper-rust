use std::collections::*;
use std::iter::*;
use rand::*;

#[derive(Clone, Copy)]
pub enum TileType {
    Neighboured(u32),
    Empty,
    Mine,
}

#[derive(Eq, PartialEq)]
pub enum TileState{
    Unclicked {flag: bool},
    Clicked
}

pub enum ClickResult{
    Won,
    Lost,
    Continued
}

pub struct Tile {
    pub tile_type: TileType,
    pub tile_state: TileState,
}

pub struct Gameboard {
    pub tiles: Vec<Vec<Tile>>,
    pub szx: usize,
    pub szy: usize,
    pub mines_total: usize,
    pub mines_found: usize
}


use TileType::*;
use TileState::*;
use std::result::Result::*;

//todo implement clicking on clicked neighboured
impl Gameboard {

    fn neighbours(x: usize, y: usize, szx: usize, szy: usize) -> Vec<(usize, usize)>{
        [(1, 1), (1, 0), (0, 1), (-1, -1), (-1, 0), (0, -1), (1, -1), (-1, 1)]
            .iter()
            .map(|&(xn, yn)| (x as i32 + xn, y as i32 + yn))
            .filter(|&(xn, yn)| 0 <= xn && xn < szx as i32 && 0 <= yn && yn < szy as i32)
            .map(|(xn, yn)| (xn as usize, yn as usize))
            .collect()
    }

    pub fn new(szx: usize, szy: usize, mines_total: usize) -> Result<Gameboard, String> {
        if mines_total > szx * szy {
            Err("Too many mines".to_string())
        } else {
            let mut mine_locs = HashSet::new();
            let mut rng = thread_rng();

            while mine_locs.len() < mines_total {
                mine_locs.insert((rng.gen_range(0,szx), rng.gen_range(0, szy)));
            }

            let tiles: Vec<Vec<_>> = (0..szx).map(|x|
                (0..szy).map(|y| {
                    if mine_locs.contains(&(x, y)) {
                        Tile { tile_type: Mine, tile_state: Unclicked{flag:false} }
                    } else {
                        let v = Gameboard::neighbours(x, y, szx, szy).iter()
                            .filter(|loc| mine_locs.contains(loc))
                            .count();
                        Tile {
                            tile_type: if v == 0 { Empty }
                                else { Neighboured(v as u32) },
                            tile_state: Unclicked { flag:false }
                        }
                    }
                }).collect()).collect();
            Ok(Gameboard { tiles, szx, szy, mines_total, mines_found: 0 })
        }
    }
    // returns whether game ended
    pub fn click(&mut self, x:usize, y:usize) -> ClickResult{
        {
            let ref mut tile = self.tiles[x][y];
            match tile.tile_state {
                Unclicked {flag} if flag => return ClickResult::Continued,
                Unclicked {flag} if !flag => (),
                Clicked => return ClickResult::Continued,
                _ => ()
            }
            tile.tile_state = Clicked;
        }
        let tile_type = self.tiles[x][y].tile_type.clone();
        if let Mine = tile_type{
            return ClickResult::Lost;
        }
        if let Empty = tile_type{
            for (xn, yn) in Gameboard::neighbours(x,y,self.szx, self.szy){
                self.click(xn, yn);
            }
        }
        if self.mines_total == self.tiles.iter().map(
            |v| v.iter().filter(|y| y.tile_state != Clicked).count()).sum() {
            ClickResult::Won
        }
        else { ClickResult::Continued }
    }

    pub fn flag_toggle(&mut self, x:usize, y:usize){
        let ref mut tile_state = self.tiles[x][y].tile_state;
        match *tile_state{
            Unclicked {flag} if flag => { self.mines_found-=1; *tile_state = Unclicked {flag: false} }
            Unclicked {flag} if !flag => { self.mines_found+=1; *tile_state = Unclicked {flag: true} }
            _ => return ()
        }
    }
}