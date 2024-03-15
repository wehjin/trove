use std::fmt::Debug;
use std::thread;

use crossbeam::channel::Sender;
#[allow(unused)]
pub use crossterm::style::Color;

pub mod beats;
pub mod captor;
pub mod console;
pub mod fill;
pub mod frame;
pub mod inset;
pub mod screen;
pub mod solar_dark;
pub mod user;
pub mod views;

#[must_use]
pub enum Cmd<Msg> {
	None,
	Unit(Box<dyn FnOnce() -> Msg + Send + Sync + 'static>),
}

impl<Msg: Send + Sync + 'static + Debug> Cmd<Msg> {
	pub fn process(self, process: Sender<Msg>) {
		let cmd = self;
		thread::spawn(move || match cmd {
			Cmd::None => {}
			Cmd::Unit(gen_msg) => {
				let msg = gen_msg();
				process.send(msg).expect("can send msg from Cmd::process");
			}
		});
	}
	pub fn wrap<U: Send + Sync + 'static>(self, map: impl Fn(Msg) -> U + Send + Sync + 'static) -> Cmd<U> {
		match self {
			Cmd::None => Cmd::None,
			Cmd::Unit(gen_t) => {
				let into_u = Box::new(map);
				Cmd::Unit(Box::new(move || into_u(gen_t())))
			}
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UserEvent {
	Quit,
	Select,
	FocusLeft,
	FocusRight,
	FocusUp,
	FocusDown,
	DeleteBack,
	Char(char),
}

