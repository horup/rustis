use winconsole::input::*;
use input::*;

pub struct InputHandler
{
    ctx:InputContext,
    input:Input,
    left:i32,
    right:i32
}

impl InputHandler
{
    pub fn new() -> InputHandler
    {
        InputHandler {left:0, right:0, ctx: start().unwrap(), input:Input {left:false, right:false, down:false, rotate:false}}
    }

    pub fn current(&self) -> Input
    {
        self.input
    }
    
    pub fn update(&mut self)
    {
        let f = 3;
        self.input.left = false;
        self.input.right = false;
        self.input.rotate = false;

        if is_key_down(KeyCode::Left)
        {
            if self.left == 0
            {
                self.input.left = true;
            }

            self.left += 1;
        }
        else
        {
            self.left = 0;
        }

        if self.left > f
        {
            self.input.left = true;
        }

        if is_key_down(KeyCode::Right)
        {
            if self.right == 0
            {
                self.input.right = true;
            }

            self.right += 1;
        }
        else
        {
            self.right = 0;
        }

        if self.right > f
        {
            self.input.right = true;
        }

        for e in self.ctx.poll()
        {
            match e 
            {
                InputEvent::KeyDown(k) => {
                    if k.key_code == KeyCode::Down
                    {
                        self.input.down = true;
                    }
                    else if k.key_code == KeyCode::Space
                    {
                        self.input.rotate = true;
                    }
                },
                InputEvent::KeyUp(k) => {
                    if k.key_code == KeyCode::Down
                    {
                        self.input.down = false;
                    }
                    else if k.key_code == KeyCode::Space
                    {
                        self.input.rotate = false;
                    }
                },
                _ => ()
            }
        }
    }
}