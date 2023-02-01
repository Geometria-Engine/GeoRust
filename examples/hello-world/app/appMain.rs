use geometria::behavior::behaviorCore::ScriptBehaviorCore;
use geometria::behavior::behaviorCore::BCORE_ALL_SCRIPTS;
use geometria::behavior::behaviorCore::BCORE_ALL_UPDATE_SCRIPTS;
use core::any::Any;

pub struct AppMain {}

impl AppMain 
{
    pub fn init() 
    {
        geometria::new_script!(ballCon, TheBallContainer, {balls: 69});
    }
}

geometria::script_behavior!(TheBallContainer,

    {
        balls: u32
    },

    fn on_start(&mut self)
    {
        //self.balls += 1;
        println!("I Start with {} balls!", self.balls);
    }

    fn on_update(&mut self)
    {
        self.balls += 1;
        println!("Now i have {} balls!", self.balls);
    }

);