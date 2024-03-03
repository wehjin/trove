use std::collections::HashMap;
use std::error::Error;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use crossterm::event::{Event, KeyCode, KeyModifiers};

use tools::console::Console;
use tools::screen::Screen;

use crate::tools::captor::{Captor, CaptorId};
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
	let mut active_captor_id: Option<CaptorId> = None;
	loop {
		let mut screen = Screen::new(console.width_height());
		app.set_edge_frame(screen.to_frame());
		let (screen_fills, ready_captors);
		loop {
			let (fills, captors) = app.get_fills_captors(active_captor_id);
			let captors = captors.into_iter().map(|it| (it.id, it)).collect::<HashMap<_, _>>();
			match active_captor_id {
				None => {
					if captors.is_empty() {
						// No active captor and no captors available to become active captor.
						// Current fills and captors are ready to go!
						(screen_fills, ready_captors) = (fills, captors);
						break;
					} else {
						// No active captor, but at least one available to become active captor.
						// Pick one and let the views re-make their fills and captors knowing the new active captor.
						// TODO Do a better job picking the active captor.
						active_captor_id = captors.keys().next().cloned();
					}
				}
				Some(captor_id) => {
					if captors.contains_key(&captor_id) {
						// Active captor and it remains useful.
						// Current fills and captors are ready to go!
						(screen_fills, ready_captors) = (fills, captors);
						break;
					} else {
						// Active captor but it is no longer a captor.
						// Forget it and let the view re-make their fills and captors without an active captor.
						active_captor_id = None;
					}
				}
			}
		}
		screen.add_fills(screen_fills);
		screen.print_to(&mut console);
		match recv_process.recv()? {
			ProcessMsg::User(user_event) => {
				match user_event {
					UserEvent::Quit => break,
					UserEvent::Select => {
						if let Some(msg) = get_user_event_msg(user_event, &active_captor_id, ready_captors) {
							send_process.send(ProcessMsg::Internal(msg)).expect("can send process msg");
						}
					}
					UserEvent::FocusUp => {
						if let Some(old_captor) = active_captor_id.map(|it| ready_captors.get(&it)).flatten() {
							let mut candidates = ready_captors.values()
								.filter(|&captor| captor.frame.bottom <= old_captor.frame.top)
								.collect::<Vec<_>>();
							// TODO Do a better job selecting higher captor.
							if !candidates.is_empty() {
								candidates.sort_by_key(|captor| -captor.frame.bottom);
								let &next_captor = candidates.first().expect("higher captor");
								active_captor_id = Some(next_captor.id);
								send_process.send(ProcessMsg::Internal(next_captor.pre_focus_msg)).expect("can send process msg");
							}
						}
					}
					UserEvent::FocusDown => {
						if let Some(old_captor) = active_captor_id.map(|it| ready_captors.get(&it)).flatten() {
							let mut candidates = ready_captors.values()
								.filter(|&captor| captor.frame.top >= old_captor.frame.bottom)
								.collect::<Vec<_>>();
							// TODO Do a better job selecting lower captor.
							if !candidates.is_empty() {
								candidates.sort_by_key(|captor| captor.frame.top);
								let &next_captor = candidates.first().expect("lower captor");
								active_captor_id = Some(next_captor.id);
								send_process.send(ProcessMsg::Internal(next_captor.pre_focus_msg)).expect("can send process msg");
							}
						}
					}
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

fn get_user_event_msg(user_event: UserEvent, active_captor_id: &Option<CaptorId>, captors: HashMap<CaptorId, Captor<SampleAppMsg>>) -> Option<SampleAppMsg> {
	let update_msg = active_captor_id
		.map(|id| captors.get(&id)).flatten()
		.map(|captor| captor.get_msg(user_event)).flatten();
	update_msg
}

fn read_keyboard(process: Sender<ProcessMsg>) -> Result<(), Box<dyn Error + Send + Sync>> {
	loop {
		match Console::read()? {
			Event::Key(key_event) => match key_event.code {
				KeyCode::Char(' ') => {
					process.send(ProcessMsg::User(UserEvent::Select))?;
				}
				KeyCode::Up | KeyCode::Char('k') => {
					process.send(ProcessMsg::User(UserEvent::FocusUp))?;
				}
				KeyCode::Down | KeyCode::Char('j') => {
					process.send(ProcessMsg::User(UserEvent::FocusDown))?;
				}
				KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
					break;
				}
				_ => {}
			},
			_ => {}
		}
	}
	Ok(())
}
