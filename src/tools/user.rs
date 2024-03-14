use std::error::Error;
use std::thread;

use crossbeam::channel::Sender;
use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::ProcessMsg;
use crate::tools::console::Console;
use crate::tools::UserEvent;

pub fn connect(send_process: &Sender<ProcessMsg>) {
	let process = send_process.clone();
	thread::spawn(move || match loop_keyboard_events(process.clone()) {
		Ok(_) => {
			let quit = ProcessMsg::User(UserEvent::Quit);
			process.send(quit).expect("send process msg");
		}
		Err(err) => {
			let error = ProcessMsg::Error(err);
			process.send(error).expect("send process msg");
		}
	});
}

fn loop_keyboard_events(process: Sender<ProcessMsg>) -> Result<(), Box<dyn Error + Send + Sync>> {
	loop {
		match Console::read()? {
			Event::Key(key_event) => match key_event.code {
				KeyCode::Char(' ') => {
					process.send(ProcessMsg::User(UserEvent::Select))?;
				}
				KeyCode::Backspace => {
					process.send(ProcessMsg::User(UserEvent::DeleteBack))?;
				}
				KeyCode::Left => {
					process.send(ProcessMsg::User(UserEvent::FocusLeft))?;
				}
				KeyCode::Right => {
					process.send(ProcessMsg::User(UserEvent::FocusRight))?;
				}
				KeyCode::Up => {
					process.send(ProcessMsg::User(UserEvent::FocusUp))?;
				}
				KeyCode::Down => {
					process.send(ProcessMsg::User(UserEvent::FocusDown))?;
				}
				KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
					break;
				}
				KeyCode::Char(c) if !c.is_control()
					&& (key_event.modifiers == KeyModifiers::NONE || key_event.modifiers == KeyModifiers::SHIFT) => {
					process.send(ProcessMsg::User(UserEvent::Char(c)))?;
				}
				_ => {}
			},
			_ => {}
		}
	}
	Ok(())
}

