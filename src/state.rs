use matrix::*;
use piece::*;
use input::*;

pub struct State
{
    pub timer:i32,
    pub iterations:i32,
    pub matrix:Matrix,
    pub piece:Option<Piece>,
    pub pieces:i32,
    pub score:i32,
    pub top_score:i32
}

impl State
{
    pub fn new() -> State
    {
        State 
        {
            timer:0, 
            iterations:0, 
            pieces:0,
            matrix:Matrix::new(), 
            piece:None,
            score:0,
            top_score:0
        }
    }

    pub fn test(&mut self)
    {
        let h = self.matrix.height();
        for i in 0..self.matrix.width()-1
        {
            self.matrix.set(i, h-1, 't');
        }
    }

    fn spawn_piece(&mut self)
    {
        self.piece = Some(Piece::new(self.pieces, self.matrix.width() as i32 / 2, -3));
        self.pieces += 1;
    }

    fn pop(&mut self)
    {
        let matrix = &mut self.matrix;
        let h = matrix.height();
        let mut popped = 0;
        loop 
        {
            let mut pop = false;
            for y in (0..h).rev()
            {
                if matrix.filled(y)
                {
                    matrix.pop_line(y);
                    pop = true;
                    popped += 1;
                    break;
                }
            }

            if pop == false
            {
                break;
            }
        }

        self.score += popped * popped;
    }

    fn reset(&mut self)
    {
        self.piece = None;
        self.matrix = Matrix::new();
        if self.score > self.top_score
        {
            self.top_score = self.score;
        }

        self.score = 0;
    }

    fn update_piece(&mut self)
    {
        let mut destroy_piece = false;
        let mut end_game = false;
        if let Some(ref mut piece) = self.piece
        {
            let org = piece.clone();
            piece.y += 1;
            if !self.matrix.valid(piece)
            {
                *piece = org;
                if piece.y < -1
                {
                    end_game = true;
                }
                else
                {
                    self.matrix.imprint(piece);
                    destroy_piece = true;
                }
            }
        }

        if destroy_piece
        {
            self.piece = None;
            self.pop();
        }

        if end_game
        {
            self.reset();
        }
    }

    fn update_input(&mut self, input:Input)
    {
        if let Some(ref mut piece) = self.piece
        {
            let org = piece.clone();
            if input.left
            {
                piece.x -= 1;
            }
            if input.right
            {
                piece.x += 1;
            }
            if input.down
            {
                piece.y += 1;
            }
            if input.rotate
            {
                piece.rotate_left();
            }

            if !self.matrix.valid(piece)
            {
                *piece = org;
            }
        }
    }

    pub fn update(&mut self, input:Input)
    {
        self.update_input(input);

        self.timer -= 1;
        if self.timer <= 0
        {
            match self.piece
            {
                Some(_) => self.update_piece(),
                None => self.spawn_piece()
            }

            self.timer = 10;
        }

        self.iterations += 1;
    }
}