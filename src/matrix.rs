const WIDTH:usize = 10;
const HEIGHT:usize = 20;

use piece::*;
pub struct Matrix([[char;WIDTH];HEIGHT]);

impl Matrix
{
    pub fn new() -> Matrix
    {
        Matrix ([[' ';WIDTH]; HEIGHT])
    }

    pub fn reset(&mut self)
    {
        self.0 = [[' ';WIDTH]; HEIGHT];
    }

    pub fn filled(&self, y:usize) -> bool
    {
        if y >= 0 as usize && y < self.0.len()
        {
            for v in &self.0[y]
            {
                if *v == ' '
                {
                    return false;
                }
            }
        }
        else
        {
            return false;
        }

        true
    }

    pub fn width(&self) -> usize
    {
        return WIDTH;
    }

    pub fn height(&self) -> usize
    {
        return HEIGHT;
    }

    pub fn valid(&self, piece:&Piece) -> bool
    {
        let x = piece.x;
        let y = piece.y;
        for i in 0..4 as usize
        {
            for j in 0..4 as usize
            {
                let px = j as i32 + x;
                let py = i as i32 + y;
                if piece.pattern[i][j] != ' ' 
                {
                    if px >= WIDTH as i32
                    {
                        return false;
                    }
                    else if px < 0
                    {
                        return false;
                    }
                    else if py >= HEIGHT as i32
                    {
                        return false;
                    }
                    else if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32
                    {
                        if self.0[py as usize][px as usize] != ' '
                        {
                            return false;
                        }
                    }
                }

            }
        }

        true
    }

    pub fn get(&self, x:usize, y:usize) -> char
    {
        return self.0[y][x];
    }

    pub fn set(&mut self, x:usize, y:usize, c:char)
    {
        self.0[y][x] = c;
    }

    pub fn pop_line(&mut self, y:usize)
    {
        self.0[y] = [' '; WIDTH];
        for i in (1..y+1).rev()
        {
            let j = i - 1;
            self.0[i] = self.0[j];
        }
    }

    pub fn imprint(&mut self, piece:&Piece)
    {
        let w = self.width();
        let h =  self.height();
        for i in 0..piece.pattern.len() as usize
        {
            for j in 0..piece.pattern[i].len() as usize
            {
                let x = j as i32 + piece.x;
                let y = i as i32 + piece.y;
                let v = piece.pattern[i][j];
                if v != ' '  && x >= 0 && x < w as i32 && y >= 0 && y < h as i32
                {
                    self.0[y as usize][x as usize] = v;
                }
            }
        }
    }
}