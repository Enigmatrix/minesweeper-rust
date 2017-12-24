use piston::input::*;

use gameboard::*;
use gameboardview::*;
use graphics::*;

pub struct GameboardController<'a> {
    pub board: Gameboard,
    pub view: GameboardView<'a>,
    pub cursor_over: Option<(usize, usize)>,
    cursor_pos: [f64; 2],
    dragging: bool,
    clicked: bool,
    prev_drag_pos: [f64; 2]
}

impl<'a> GameboardController<'a> {
    pub fn new(board: Gameboard, view: GameboardView) -> GameboardController {
        GameboardController {
            board,
            view,
            cursor_over: None,
            cursor_pos: [0.0; 2],
            dragging: false,
            clicked: false,
            prev_drag_pos: [0.0; 2]
        }
    }

    pub fn process(&mut self, e: Event) {
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            self.clicked = true;
            self.prev_drag_pos = self.cursor_pos;
        }
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
            self.cursor_over = self.view.to_cursor(self.cursor_pos);
            if self.clicked{
                self.dragging = true;
                self.view.pan[0] += self.cursor_pos[0]-self.prev_drag_pos[0];
                self.view.pan[1] += self.cursor_pos[1]-self.prev_drag_pos[1];
                self.prev_drag_pos = self.cursor_pos;
            }
        }
        if let Some(Button::Mouse(btn)) = e.release_args() {
            match btn{
                MouseButton::Left => {
                    self.clicked = false;
                    if !self.dragging{
                        if let Some((x,y)) = self.cursor_over{
                            match self.board.click(x,y){
                                ClickResult::Lost => println!("LOSE"),
                                ClickResult::Won => println!("Won"),
                                ClickResult::Continued => ()
                            }
                        }
                    }
                    self.dragging = false;
                },
                MouseButton::Right => {
                    if let Some((x,y)) = self.cursor_over{
                        self.board.flag_toggle(x,y);
                    }
                }
                _ => ()
            }
        }
        if let Some(sa) = e.mouse_scroll_args(){
            self.view.zoom_by(sa[1]);
        }
        if let Some(ra) = e.render_args() {
            self.view.draw(&self.board, ra, self.cursor_over);
        }

    }
}

