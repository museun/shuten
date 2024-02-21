#![cfg_attr(debug_assertions, allow(dead_code, unused_variables,))]
use shuten_core::{
    event::{Event, Key},
    geom::{offset, pos2, Offset, Pos2},
    style::Rgb,
    Canvas, Config, Terminal,
};

fn main() -> std::io::Result<()> {
    let mut terminal = Terminal::new(Config::default().fixed_timer(60.0))?;
    let mut app = App::new();

    while let Ok(event) = terminal.wait_for_next_event() {
        if event.is_quit() {
            break;
        }

        match event {
            Event::Keyboard(ev, _) => match ev {
                Key::Char(' ') => app.hard_drop(),
                Key::Char('w') => app.hold_piece(),
                Key::Char('a') => app.rotate_left(),
                Key::Char('A') => app.move_left(),
                Key::Char('s') => app.move_down(),
                Key::Char('d') => app.rotate_right(),
                Key::Char('D') => app.move_right(),
                Key::Char('p') => app.pause(),
                _ => {}
            },
            Event::Blend(factor) => app.blend(factor),
            _ => {}
        }
    }

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    const fn flip(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }

    const DOWN: Self = Self::Down;
    const LEFT: Self = Self::Left;
    const RIGHT: Self = Self::Right;

    const KICKS: [Offset; 5] = [
        offset(0, 0),
        offset(-1, 0),
        offset(-1, 1),
        offset(0, -2),
        offset(-1, -2),
    ];
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Rot {
    Right,
    Left,
}

impl Rot {
    const fn flip(self) -> Self {
        match self {
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Clone)]
struct Piece {
    rot: usize,
    block: Block,
}

impl Piece {
    const fn new(block: Block) -> Self {
        Self { rot: 0, block }
    }

    fn get_block_geom(&self) -> [Pos2; 4] {
        let mut points = [Pos2::ZERO; 4];
        let rot = self.block.geom[self.rot];
        let center = self.block.center;
        for (i, &p) in rot.iter().enumerate() {
            debug_assert!(center.x as i32 + p.x >= 0);
            points[i] = add_offset(center, p)
        }
        points[3] = center;
        points
    }

    fn move_piece_offset(&mut self, offset: Offset) {
        self.block.center = add_offset(self.block.center, offset);
    }

    fn move_piece(&mut self, dir: &Direction) {
        let offset = match dir {
            Direction::Up => offset(-1, 0),
            Direction::Down => offset(1, 0),
            Direction::Right => offset(0, -1),
            Direction::Left => offset(0, 1),
        };
        self.move_piece_offset(offset)
    }

    fn rotate(&mut self, rot: &Rot) {
        self.rot = match rot {
            Rot::Right => self.rot.checked_sub(1).unwrap_or(3),
            Rot::Left => (self.rot + 1) % 4,
        };
    }
}

fn add_offset(pos: Pos2, offset: Offset) -> Pos2 {
    pos2(
        u16::try_from(pos.x as i32 + offset.x).unwrap(),
        u16::try_from(pos.y as i32 + offset.y).unwrap(),
    )
}

#[derive(Clone)]
struct Block {
    center: Pos2,
    geom: [[Offset; 3]; 4],
    color: Rgb,
}

impl Block {
    const I: Self = Self {
        center: pos2(1, 5),
        geom: [
            [offset(0, -2), offset(0, -1), offset(0, 1)],
            [offset(-2, 0), offset(-1, 0), offset(1, 0)],
            [offset(0, -2), offset(0, -1), offset(0, 1)],
            [offset(-2, 0), offset(-1, 0), offset(1, 0)],
        ],
        color: Rgb::from_u32(0x00FFFF),
    };
    const J: Self = Self {
        center: pos2(1, 5),
        geom: [
            [offset(0, -1), offset(0, 1), offset(-1, 1)],
            [offset(-1, 1), offset(-1, 0), offset(1, 0)],
            [offset(1, -1), offset(0, -1), offset(0, 1)],
            [offset(-1, 0), offset(1, 0), offset(1, 0)],
        ],
        color: Rgb::from_u32(0x0000FF),
    };
    const L: Self = Self {
        center: pos2(1, 4),
        geom: [
            [offset(-1, -1), offset(0, -1), offset(0, 1)],
            [offset(-1, 0), offset(1, 0), offset(1, -1)],
            [offset(0, -1), offset(0, 1), offset(1, 1)],
            [offset(-1, 1), offset(-1, 0), offset(1, 0)],
        ],
        color: Rgb::from_u32(0xFF8000),
    };
    const O: Self = Self {
        center: pos2(1, 4),
        geom: [
            [offset(-1, 0), offset(0, 1), offset(-1, 1)],
            [offset(-1, 0), offset(0, 1), offset(-1, 1)],
            [offset(-1, 0), offset(0, 1), offset(-1, 1)],
            [offset(-1, 0), offset(0, 1), offset(-1, 1)],
        ],
        color: Rgb::from_u32(0xFFFF00),
    };
    const S: Self = Self {
        center: pos2(0, 4),
        geom: [
            [offset(1, -1), offset(1, 0), offset(0, 1)],
            [offset(-1, 0), offset(0, 1), offset(1, 1)],
            [offset(1, -1), offset(1, 0), offset(0, 1)],
            [offset(-1, 0), offset(0, 1), offset(1, 1)],
        ],
        color: Rgb::from_u32(0x00FF00),
    };
    const T: Self = Self {
        center: pos2(1, 4),
        geom: [
            [offset(-1, 0), offset(0, -1), offset(0, 1)],
            [offset(-1, 0), offset(0, -1), offset(1, 0)],
            [offset(0, -1), offset(1, 0), offset(0, 1)],
            [offset(-1, 0), offset(1, 0), offset(0, 0)],
        ],
        color: Rgb::from_u32(0xFF00FF),
    };
    const Z: Self = Self {
        center: pos2(0, 4),
        geom: [
            [offset(0, -1), offset(1, 0), offset(1, 1)],
            [offset(-1, 1), offset(0, 1), offset(1, 0)],
            [offset(0, -1), offset(1, 0), offset(1, 1)],
            [offset(-1, 1), offset(0, 1), offset(1, 0)],
        ],
        color: Rgb::from_u32(0xFF0000),
    };
}

enum GameEvent {
    Tick,
    Move(Direction),
    Rotate(Rot),
    HardDrop,
    Hold,
    Pause,
    Quit,
}

struct Game {
    ghost: Option<Piece>,
    held: Option<Piece>,

    level: u16,
    score: u16,
    cleared: u16,

    moving: Piece,
    next_pieces: Vec<Piece>,

    area: [[Rgb; 10]; 22],
}

impl Game {
    fn new() -> Self {
        let mut next_pieces = generate_pieces().to_vec();
        Self {
            ghost: None,
            held: None,
            level: 1,
            score: 0,
            cleared: 0,
            moving: next_pieces.pop().unwrap(),
            next_pieces,
            area: [[Rgb::from_u16(0); 10]; 22],
        }
    }

    fn get_next_piece(&mut self, pop: bool) -> Piece {
        if self.next_pieces.is_empty() {
            self.next_pieces.extend_from_slice(&generate_pieces());
        }
        if pop {
            self.next_pieces.pop().unwrap()
        } else {
            self.next_pieces.last().unwrap().clone()
        }
    }

    fn fill(&mut self, piece: &Piece) {
        for point in piece.get_block_geom() {
            if self.area[point.x as usize][point.y as usize] == piece.block.color {
                continue;
            }
            self.area[point.x as usize][point.y as usize] = piece.block.color;
        }
    }

    fn clear(&mut self, piece: &Piece) {
        for point in piece.get_block_geom() {
            if self.area[point.x as usize][point.y as usize] != piece.block.color {
                continue;
            }
            self.area[point.x as usize][point.y as usize] = Rgb::from_u32(0)
        }
    }
}

struct App {
    game: Game,
}

impl App {
    fn new() -> Self {
        Self { game: Game::new() }
    }

    fn hard_drop(&mut self) {}

    fn hold_piece(&mut self) {}

    fn rotate_left(&mut self) {}

    fn move_left(&mut self) {}

    fn move_down(&mut self) {}

    fn rotate_right(&mut self) {}

    fn move_right(&mut self) {}

    fn pause(&mut self) {}

    fn draw(&mut self, mut canvas: Canvas) {
        canvas.erase();
        let piece = self.game.get_next_piece(false);
        for point in piece.get_block_geom() {
            let p = point - piece.block.center;
            self.game.area[(p.x + 3) as usize][(p.y + 4) as usize] = piece.block.color;
        }
    }

    fn blend(&mut self, factor: f32) {}
}

fn generate_pieces() -> [Piece; 7] {
    let mut pieces = [
        Piece::new(Block::J),
        Piece::new(Block::I),
        Piece::new(Block::L),
        Piece::new(Block::O),
        Piece::new(Block::S),
        Piece::new(Block::T),
        Piece::new(Block::Z),
    ];
    fastrand::shuffle(&mut pieces);
    pieces
}
