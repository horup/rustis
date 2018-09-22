pub type Pattern = [[char;4];4];

#[derive(Clone)]
pub struct Piece
{
    pub pattern:Pattern,
    pub x:i32,
    pub y:i32
}

impl Piece
{
    pub fn new(i:i32, x:i32, y:i32) -> Piece
    {
        let v = i % 7;
        if v == 0
        {
            return Piece::i(x, y);
        }
        else if v == 1
        {
            return Piece::j(x, y);
        }
        else if v == 2
        {
            return Piece::l(x, y);
        }
        else if v == 3
        {
            return Piece::t(x, y);
        }
        else if v == 4
        {
            return Piece::s(x, y);
        }
        else if v == 5
        {
            return Piece::z(x, y);
        }
        else if v == 6
        {
            return Piece::o(x, y);
        }
        else
        {
            return Piece::o(x, y);
        }
    }

    pub fn i(x:i32, y:i32) -> Piece
    {
        let e = ' ';
        let c = 'i';
        Piece {pattern:
        [
            [e, e, e, e],
            [c, c, c, c],
            [e, e, e, e],
            [e, e, e, e]
        ], x:x, y:y}
    }

    pub fn o(x:i32, y:i32) -> Piece
    {
        let e = ' ';
        let c = 'o';
        Piece {pattern:
        [
            [e, e, e, e],
            [e, c, c, e],
            [e, c, c, e],
            [e, e, e, e]
        ], x:x, y:y}
    }

    pub fn s(x:i32, y:i32) -> Piece
    {
        let e = ' ';
        let c = 's';
        Piece {pattern:
        [
            [e, e, e, e],
            [e, c, c, e],
            [c, c, e, e],
            [e, e, e, e]
        ], x:x, y:y}
    }

    pub fn z(x:i32, y:i32) -> Piece
    {
        let e = ' ';
        let c = 'z';
        Piece {pattern:
        [
            [e, e, e, e],
            [e, c, c, e],
            [e, e, c, c],
            [e, e, e, e]
        ], x:x, y:y}
    }

    pub fn l(x:i32, y:i32) -> Piece
    {
        let e = ' ';
        let c = 'l';
        Piece {pattern:
        [
            [e, c, e, e],
            [e, c, e, e],
            [e, c, c, e],
            [e, e, e, e]
        ], x:x, y:y}
    }

    pub fn j(x:i32, y:i32) -> Piece
    {
        let e = ' ';
        let c = 'j';
        Piece {pattern:
        [
            [e, e, c, e],
            [e, e, c, e],
            [e, c, c, e],
            [e, e, e, e]
        ], x:x, y:y}
    }

    pub fn rotate_left(&mut self)
    {
        let org = self.pattern;
        for i in 0..org.len()
        {
            for j in 0..org[i].len()
            {
                self.pattern[j][i] = org[i][3 - j];
            }
        }
    }

    pub fn t(x:i32, y:i32) -> Piece
    {
        let e = ' ';
        let c = 't';
        Piece {pattern:
        [
            [e, e, e, e],
            [e, c, e, e],
            [c, c, c, e],
            [e, e, e, e]
        ], x:x, y:y}
    }
}