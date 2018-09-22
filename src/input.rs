#[derive(Copy, Clone)]
pub struct Input
{
    pub left:bool,
    pub right:bool,
    pub down:bool,
    pub rotate:bool
}

impl Input
{
    pub fn new() -> Input
    {
        Input {left:false, right:false, down:false, rotate:false}
    }
}