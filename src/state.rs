use std::io;
use actix::prelude::*;
use rs_ws281x::{Controller, RawColor};

// Turn led strip a solid color
pub struct Solid(pub RawColor);
impl Message for Solid {
    type Result = Result<bool, io::Error>;
}

pub struct Strip(pub Controller, pub usize);

impl Actor for Strip {
    type Context = Context<Self>;
}

impl Handler<Solid> for Strip {
    type Result = Result<bool, io::Error>;

    fn handle(&mut self, msg: Solid, ctx: &mut Context<Self>) -> Self::Result {
        let mut leds = self.0.leds_mut(0);
        let num_leds = self.1;
        
        for i in 0..num_leds {
            leds[i] = msg.0;
        }

        self.0.render();
        Ok(true)
    }
}
