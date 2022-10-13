#![cfg_attr(feature = "nightly", feature(portable_simd))]

pub use makepad_widgets;
use makepad_widgets::*;
use makepad_draw_2d::*;
mod mandelbrot;

#[cfg(feature = "nightly")]
mod mandelbrot_simd;

live_register!{
    import makepad_widgets::frame::*;
    registry Widget::*;
    App: {{App}} {
        frame: {
            walk:{width: Fill, height: Fill},
            
            Mandelbrot{
                walk:{width: Fill, height: Fill}
            }
            
            // alright lets put a slider over the thing
            // ok so first i want a panel
            // something that has an icon
            // and animates closed
            // and open.
            // OK go 
        }
    }
}
main_app!(App);
 
#[derive(Live, LiveHook)]
pub struct App {
    frame: Frame,
    window: BareWindow,
}

impl App {
    pub fn live_register(cx: &mut Cx) {
        makepad_widgets::live_register(cx);
        mandelbrot::live_register(cx);
    }
    
    pub fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.window.handle_event(cx, event);
        
        for _ in self.frame.handle_event_vec(cx, event) {
        }
        
        match event {
            Event::Draw(event) => {
                return Cx2d::draw(cx, event, self, | cx, s | s.draw(cx));
            }
            _ => ()
        }
    }
    
    pub fn draw(&mut self, cx: &mut Cx2d) {
        if self.window.begin(cx).not_redrawing() {
            return;
        }
        while self.frame.draw(cx).is_not_done(){
        };
        self.window.end(cx);
    }
}