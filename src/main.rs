extern crate rand;
extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::*;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::*;
use opengl_graphics::*;

mod gameboard;

use gameboard::*;
use std::io::*;

fn main() {
    println!("Hello, world!");
    /*let opengl = OpenGL::V3_2;
    let window_settings = WindowSettings::new("Minesweeper-Rust", [500; 2])
        .opengl(opengl);
    let mut window: GlutinWindow = window_settings.build()
        .expect("Window could not be created");
    
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window){
        
    }*/
    let mut game = Gameboard::new(9, 9, 10).unwrap();
    loop {
        let mut input = String::new();
        print_gameboard(&game);
        println!("{} Mines Left", game.mines_total - game.mines_found);
        println!("Enter click in x,y format:");
        stdin().read_line(&mut input).unwrap();
        let xy: Vec<usize> = input.split(',').map(|v| v.trim().parse().unwrap()).collect();
        let (x, y) = (xy[1], xy[2]);
        match xy[0] {
            0 => match game.click(x, y) {
                ClickResult::Lost => {
                    print_gameboard(&game);
                    println!("You have lost!");
                    break;
                }
                ClickResult::Won => {
                    println!("You have won!");
                    break;
                }
                ClickResult::Continued => ()
            },
            1 => game.flag_toggle(x, y),
            _ => println!("Invalid Option!")
        }
    }
}

fn print_gameboard(g: &Gameboard) {
    for row in g.tiles.iter() {
        for tile in row.iter() {
            let c = match tile.tile_state {
                TileState::Unclicked { flag } if flag => '!',
                TileState::Unclicked { flag } if !flag => 'O',
                TileState::Clicked =>
                    match tile.tile_type {
                        TileType::Empty => '.',
                        TileType::Neighboured(x) => ('0' as u8 + x as u8) as char,
                        TileType::Mine => 'X',
                    },
                _ => panic!()
            };
            print!("{} ", c);
        }
        println!();
    }
}