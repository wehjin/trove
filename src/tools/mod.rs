use std::fmt::Debug;
use std::sync::mpsc::Sender;
use std::thread;

#[allow(unused)]
pub use crossterm::style::Color;

pub mod captor;
pub mod console;
pub mod fill;
pub mod frame;
pub mod inset;
pub mod sample;
pub mod screen;
pub mod solar_dark;
pub mod views;

pub enum Cmd<Msg> {
	None,
	Unit(Box<dyn Fn() -> Msg + Send + Sync + 'static>),
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
	pub fn map<U: Send + Sync + 'static>(self, map: impl Fn(Msg) -> U + Send + Sync + 'static) -> Cmd<U> {
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
}

