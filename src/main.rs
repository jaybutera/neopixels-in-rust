mod state;

use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType, RawColor};
use actix::prelude::*;
use state::{Strip, Solid};
use futures::future::Future;

const N: usize = 60;

fn to_solid(leds: &mut [RawColor], color: RawColor) {
    for i in 0..N {
        leds[i] = color;
    }
}

fn main() {
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(0,
            ChannelBuilder::new()
                .pin(18)
                .count(N as i32)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(255)
                .build()
        )
        .build()
        .unwrap();


    let sys = actix::System::new("leds");
    let strip = Strip(controller, N);
    let addr = strip.start();

    let fut = addr.send( Solid([0,0,255,0]) );

    Arbiter::spawn(
        fut.map(|res| {
            match res {
                Ok(b) => println!("{}",b),
                Err(e) => println!("Error: {}", e),
            }
        })
        .map_err(|e| {
            println!("Actor mighta died... ({})", e);
        }));

    //let mut leds = controller.leds_mut(0);
    //to_solid(leds, [0,255,0,0]);
    sys.run();

    //controller.render();
}
