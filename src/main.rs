use std::error::Error;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use crossterm::event::{Event, KeyCode};

use tools::console::Console;
use tools::screen::Screen;

use crate::tools::sample::{SampleApp, SampleAppMsg};
use crate::tools::UserEvent;

pub mod tools;

#[derive(Debug)]
pub enum ProcessMsg {
	User(UserEvent),
	Internal(SampleAppMsg),
	Error(Box<dyn Error + Send + Sync>),
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut app = SampleApp::new();
	let mut console = Console::start()?;
	let (send_process, recv_process) = channel::<ProcessMsg>();
	thread::spawn({
		let process = send_process.clone();
		move || {
			let msg = match read_keyboard(process.clone()) {
				Ok(_) => ProcessMsg::User(UserEvent::Quit),
				Err(err) => ProcessMsg::Error(err),
			};
			process.send(msg).expect("send process msg");
		}
	});
	loop {
		let mut screen = Screen::new(console.width_height());
		let (fills, captors) = app.get_fills_captors(screen.to_frame());
		screen.add_fills(fills);
		screen.print_to(&mut console);
		match recv_process.recv()? {
			ProcessMsg::User(user_event) => {
				match user_event {
					UserEvent::Select => {
						// TODO Impl better policy. Rudimentary policy works only for this specific case.
						for captor in &captors {
							if let Some(msg) = captor.get_msg(user_event) {
								send_process.send(ProcessMsg::Internal(msg)).expect("can send process msg");
								break;
							}
						}
					}
					UserEvent::Quit => break,
				}
			}
			ProcessMsg::Error(err) => {
				return Err(err);
			}
			ProcessMsg::Internal(app_msg) => {
				let cmd = app.update(app_msg);
				cmd.map(ProcessMsg::Internal).process(send_process.clone());
			}
		}
	}
	Ok(())
}

fn read_keyboard(process: Sender<ProcessMsg>) -> Result<(), Box<dyn Error + Send + Sync>> {
	loop {
		match Console::read()? {
			Event::Key(key_event) => match key_event.code {
				KeyCode::Char(' ') => {
					process.send(ProcessMsg::User(UserEvent::Select))?;
				}
				KeyCode::Char('q') => {
					break;
				}
				_ => {}
			},
			_ => {}
		}
	}
	Ok(())
}
