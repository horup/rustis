use matrix::*;
use state::*;
use winconsole::console;
use winconsole::console::*;

pub struct Render
{
    surface:Vec<Vec<char>>,
    width:usize,
    height:usize
}

impl Render
{
    pub fn new(matrix:&Matrix) -> Render
    {
        let w = matrix.width() * 2 + 2;
        let h = matrix.height() + 2;

        console::set_window_size(w as u16, h as u16).unwrap();

        let _ = console::clear();
        let _ = console::set_cursor_visible(false);
        Render
        {
            surface:vec![vec![' '; w];h],
            width: w,
            height: h
        }
    }

    fn get_color(&self, c:char) -> ConsoleColor
    {
        match c
        {
            ' ' => ConsoleColor::Black,
            'a' => ConsoleColor::Red,
            't' => ConsoleColor::Blue,
            'i' => ConsoleColor::Yellow,
            'l' => ConsoleColor::Green,
            'j' => ConsoleColor::DarkGreen,
            's' => ConsoleColor::Magenta,
            'z' => ConsoleColor::Teal,
            'o' => ConsoleColor::DarkYellow,
            _   => ConsoleColor::Gray
        }
    }

    fn set_color(&self, c:char, x:u16, y:u16)
    {
        let colors = vec![(ConsoleColor::White, self.get_color(c))];
        console::write_output_colors(&colors, x, y).unwrap();
    }

    fn draw_borders(&mut self)
    {
        let c = '#';
        for i in 0..self.width
        {
            let h = self.height;
            let _ = self.set(c, i as i32, 0);
            let _ = self.set(c, i as i32, h as i32 - 1);
        }
        for i in 0..self.height
        {
            let w = self.width;
            let _ = self.set(c, 0, i as i32 );
            let _ = self.set(c, w as i32  - 1, i as i32);
        }
    }

    fn draw_piece(&mut self, state:&State)
    {
        if let Some(ref piece) = state.piece
        {
            for y in 0..piece.pattern.len()
            {
                for x in 0..piece.pattern[y].len()
                {
                    let v = piece.pattern[y][x];
                    let xx = x as i32 + piece.x;
                    let yy = y as i32 + piece.y;
                    if v!= ' ' && xx >= 0 && xx < state.matrix.width() as i32 && yy >= 0 && yy < state.matrix.height() as i32
                    {
                        self.set2(v, xx as i32 * 2 + 1, yy as i32 + 1);
                    }
                }
            }
        }
    }

    fn draw_matrix(&mut self, state:&State)
    {
        let matrix = &state.matrix;
        for y in 0..matrix.height()
        {
            for x in 0..matrix.width()
            {
                let v = matrix.get(x, y);
                self.set2(v, x as i32 * 2 + 1, y as i32 + 1);
            }
        }
    }

    fn draw_score(&mut self, state:&State)
    {
        let current = state.score.to_string();
        let top = state.top_score.to_string();
        let _ = console::write_output(&current, 1, 0).unwrap();
        let _ = console::write_output(&top, 1, self.height as u16 - 1).unwrap();
    }

    fn set(&mut self, v:char, x:i32, y:i32)
    {
        self.surface[y as usize][x as usize] = v;
    }

    fn set2(&mut self, v:char, x:i32, y:i32)
    {
        self.surface[y as usize][x as usize] = v;
        self.surface[y as usize][x as usize + 1] = v;
    }

    fn blit(&mut self)
    {
        for y in 0..self.height
        {
            for x in 0..self.width
            {
                self.set_color(self.surface[y][x], x as u16, y as u16);
            }
        }
    }

    pub fn draw(&mut self, state:&State)
    {
        self.draw_borders();
        self.draw_matrix(state);
        self.draw_piece(state);
        self.draw_score(state);
        self.blit();
    }
}