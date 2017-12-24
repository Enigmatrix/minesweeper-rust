use graphics::types::Color;
use graphics::character::CharacterCache;
use graphics::*;
use graphics::math::*;
use piston::input::*;
use opengl_graphics::*;

use gameboard::*;
use gameboardcontroller::*;
use vecmath::*;

pub struct GameboardViewSettings{
    pub background_color: Color,
    pub text_color: Color,

    pub zoom_epsilon: f64
}

impl GameboardViewSettings{
    pub fn default() -> GameboardViewSettings{
        GameboardViewSettings{
            background_color: [0.0, 0.0, 0.0, 1.0],
            text_color: [0.0, 0.0, 1.0, 1.0],
            zoom_epsilon: 0.1
        }
    }
}

pub struct GameboardView<'a>{
    pub zoom: f64,
    pub pan: [f64; 2],
    pub settings: GameboardViewSettings,
    pub api: GraphicsApi<'a>,
    rvs_transform: Matrix2d,
    board_size: [f64; 2],
    tile_size: f64
}

pub struct GraphicsApi<'a>{
    pub graphics: GlGraphics,
    pub glyphs: GlyphCache<'a>
}

impl<'a> GraphicsApi<'a>{
    pub fn opengl(graphics: GlGraphics, glyphs: GlyphCache)-> GraphicsApi{
        GraphicsApi{
            graphics,
            glyphs
        }
    }
}

impl<'a> GameboardView<'a>{

    pub fn new(settings: GameboardViewSettings, api: GraphicsApi) -> GameboardView{
        GameboardView {
            settings,
            zoom: 1.0,
            pan: [0.0, 0.0],
            api,
            rvs_transform: [[0.0; 3]; 2],
            board_size: [0.0; 2],
            tile_size: 0.0
        }
    }

    pub fn to_cursor(&self, pos: [f64; 2]) -> Option<(usize, usize)>{

        let pos = row_mat2x3_transform_pos2(self.rvs_transform, pos);
        let wmin = -self.board_size[0]/2.0;
        let wmax = self.board_size[0]/2.0;
        let hmin = -self.board_size[1]/2.0;
        let hmax = self.board_size[1]/2.0;

        if wmin < pos[0] && pos[0] <= wmax && hmin < pos[1] && pos[1] <= hmax{
            Some((((pos[0]-wmin)/self.tile_size) as usize, ((pos[1]-hmin)/self.tile_size) as usize))
        } else{ None }
    }

    pub fn zoom_by(&mut self, i: f64){
        self.zoom += i*self.settings.zoom_epsilon;
    }

    pub fn draw(&mut self, board: &Gameboard, ra: RenderArgs, cursor_over: Option<(usize, usize)>){
        let ref settings = self.settings;
        let ref mut rvs_transform = self.rvs_transform;
        let ref mut board_size = self.board_size;
        let ref mut tile_size = self.tile_size;
        let zoom = self.zoom;
        let [px, py] = self.pan;

        let vp = ra.viewport();
        let (vw, vh) = (vp.rect[2] as f64, vp.rect[3] as f64);

        let cell = Texture::from_path("assets/Cell.png", &TextureSettings::new()).unwrap();
        let cell_over = Texture::from_path("assets/CellOver.png", &TextureSettings::new()).unwrap();
        let empty = Texture::from_path("assets/EmptyCell.png", &TextureSettings::new()).unwrap();
        let flagged = Texture::from_path("assets/FlaggedCell.png", &TextureSettings::new()).unwrap();
        let mine = Texture::from_path("assets/ExplodedMineCell.png", &TextureSettings::new()).unwrap();

        let img = Image::new();
        let text_img = Image::new_color(settings.text_color);

        let ref mut glyphs = self.api.glyphs;

        self.api.graphics.draw(vp, |ctx, graphics| {
            clear(settings.background_color, graphics);

            let transform = ctx.transform.trans(vw/2.0,vh/2.0)
                .trans(px,py)
                .zoom(zoom);

            *rvs_transform = identity().zoom(1.0/zoom).trans(-px, -py).trans(-vw/2.0, -vh/2.0);


            *tile_size = cell.get_width() as f64;
            let tile_count_x = board.szx as f64;
            let tile_count_y = board.szy as f64;
            *board_size = [*tile_size*tile_count_x, *tile_size*tile_count_y];
            let board_rect = [-board_size[0]/2.0,  -board_size[1]/2.0, board_size[0], board_size[1]];

            let mut top_left_board = [-board_size[0]/2.0, -board_size[1]/2.0];
            for x in 0..board.szx {
                for y in 0..board.szy {
                    let ref tile = board.tiles[x][y];
                    let pos_trans = transform.trans(
                            top_left_board[0] + *tile_size * x as f64,
                            top_left_board[1] + *tile_size * y as f64);
                    let mut num = 0;
                    let texture = match tile.tile_state {
                        TileState::Unclicked { flag } if flag => &flagged,
                        TileState::Unclicked { flag } if !flag => {
                            match cursor_over {
                                Some((xn,yn)) if x == xn && y == yn => &cell_over,
                                _ => &cell
                            }
                        },
                        TileState::Clicked =>
                            match tile.tile_type {
                                TileType::Empty => &empty,
                                TileType::Neighboured(v) => { num = v; &empty },
                                TileType::Mine => &mine,
                            },
                        _ => panic!()
                    };
                    img.draw(texture, &ctx.draw_state, pos_trans, graphics);
                    if num != 0{
                        if let Ok(character) = glyphs.character((*tile_size) as u32/4*3, ('0' as u8 + num as u8) as char) {
                            let trans = pos_trans.trans(*tile_size*2.0/7.0, *tile_size/6.5);
                            text_img.draw(character.texture, &ctx.draw_state, trans, graphics);
                        }
                    }
                }
            }

        });
    }
}
