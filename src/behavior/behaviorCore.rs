#![feature(concat_idents)]
#[allow(non_snake_case)]

use std::{collections::HashMap, sync::Arc, thread, time::Duration};
use std::rc::Rc;
use lazy_static::lazy_static;
use std::sync::Mutex;
use core::any::Any;

pub static mut BCORE_ALL_SCRIPTS: Vec<Box<dyn ScriptBehaviorCore + Sync>> = Vec::new();
pub static mut BCORE_ALL_UPDATE_SCRIPTS: Vec<Box<dyn ScriptBehaviorCore + Sync>> = Vec::new();

pub trait ScriptBehaviorCore: Sync + Send
{
	fn on_start(&mut self);
	fn on_update(&mut self);

	fn as_any(&self) -> &dyn Any;
}

pub fn update_scripts()
{
	unsafe
	{
		for script in BCORE_ALL_SCRIPTS.iter_mut() {
			script.on_update();
		}
	}
}

#[macro_export]
macro_rules! script_behavior
{
	($a:ident,$c:tt,$($b:tt)*) => {
		pub struct $a $c

		impl ScriptBehaviorCore for $a
		{
			$($b)*

			fn as_any(&self) -> &dyn Any {
        		self
    		}
		}
	}
}

#[macro_export]
macro_rules! new_script
{
	($a:ident,$c:ident,$($b:tt)*) => {
		unsafe {
			let mut obj = Box::new($c $($b)* );
			obj.on_start();
			BCORE_ALL_SCRIPTS.push(obj);

			let $a: &$c = match BCORE_ALL_SCRIPTS.last().unwrap().as_any().downcast_ref::<$c>() {
        		Some($a) => $a,
        		None => panic!("&a isn't a B!"),
    		};
    	}
    }
}