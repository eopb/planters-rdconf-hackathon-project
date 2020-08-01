use seed::{prelude::*, *};
use std::rc::Rc;
use std::cell::RefCell;
use seed_hooks::*;
use crate::Msg;

#[derive(Clone, PartialEq)]
pub enum LoopStatus {
    Running,
    Stopped,
}

type RcMutClosure = Rc<RefCell<Option<Closure<(dyn FnMut(f64) + 'static)>>>>;

impl Default for LoopStatus {
    fn default() -> Self {
        Self::Stopped
    }
}


// Controlling struct for a deterministic main loop
// fires Msg::TimeStepAdvanced every timestep.
#[derive(Clone)]
pub struct RafLoop {
    pub last_frame_ts : Option<f64>,
    pub timestep: f64, // 60fps
    pub raf_closure: RcMutClosure,
    pub command: Option<RafLoopCommand>,
    pub status: LoopStatus,
}

impl Default for RafLoop{
    fn default() -> RafLoop{
        RafLoop{
            last_frame_ts : None,
            timestep: 1000.0/60.0, // 60fps
            raf_closure: Rc::new(RefCell::new(None)),
            command: None,
            status: LoopStatus::Stopped,
            
        }
    }
}

impl RafLoop{
    pub fn start(&mut self) -> () {
    request_animation_frame(
        self.raf_closure.borrow().as_ref().unwrap(),
    );
    self.status = LoopStatus::Running;
    }

    pub fn stop(&mut self) -> () {
        if self.status == LoopStatus::Running {
            self.command = Some(RafLoopCommand::Stop);
        }
    }
}


#[derive(Clone,PartialEq)]
pub enum RafLoopCommand {
    Stop
}


#[atom]
pub fn raf_loop_atom() -> Atom<RafLoop>{
    let raf_loop = RafLoop::default();

    let mut delta = 0.0;
    let closure = raf_loop.raf_closure.clone();
    
    *closure.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp| {
    let  raf_loop = raf_loop_atom().get();
      
        
    if let Some(cmd) = raf_loop.command {
        if cmd == RafLoopCommand::Stop {
            raf_loop_atom().update(|e|
            {
                e.last_frame_ts = None;
                e.status = LoopStatus::Stopped;
                e.command = None;
            });
            log!("stopping raf loop");
            crate::my_app().get().unwrap().update(Msg::TimeStepLoopStopped);
            return;
       }
    }

        // if !raf_loop.commands.is_empty(){
        //     for cmd in raf_loop.commands.drain(..) {
        //         if cmd == RafLoopCommand::Stop {
        //             raf_loop_atom().update(|e|
        //             {
        //                 e.last_frame_ts = None;
        //                 e.status = LoopStatus::Stopped;
        //                 e.commands = vec![]
        //             });
        //             log!("stopping raf loop");
        //             crate::my_app().get().unwrap().update(Msg::TimeStepLoopStopped);
        //             return;
        //         }
        //     }     
        // }

        // If possible_last_frame_timestep is none, then this is the first run after a pause.
        // shedule a raf restart and return.
        if raf_loop.last_frame_ts.is_none (){
            raf_loop_atom().update(|e| e.last_frame_ts = Some(timestamp));
            request_animation_frame( raf_loop.raf_closure.borrow().as_ref().unwrap());
            log!("restarting due to no last frame data.");
            crate::my_app().get().unwrap().update(Msg::TimeStepLoopStarted);
            return;
            // do setup stuff on first loop of raf loop
        }

        let last_frame_timestamp = raf_loop.last_frame_ts.unwrap();
        delta += timestamp - last_frame_timestamp;
        raf_loop_atom().update(|e| e.last_frame_ts = Some(timestamp));

        while delta >= raf_loop.timestep {
            //
            // make any adjustments based on timestep or detla
            delta -= raf_loop.timestep;
            crate::my_app().get().unwrap().update(Msg::TimeStepAdvanced);
        }
        let closure = raf_loop.raf_closure.clone();
        request_animation_frame(
            closure.borrow().as_ref().unwrap(),
        );

    })   as  Box<dyn FnMut(f64)> ));

    raf_loop
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
