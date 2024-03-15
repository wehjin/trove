use std::collections::HashMap;
use std::error::Error;

use crossbeam::channel::unbounded;

use tools::console::Console;
use tools::screen::Screen;
use tools::user;

use crate::app::sample::{SampleApp, SampleAppMsg};
use crate::tools::beats::Thumper;
use crate::tools::captor::{Captor, CaptorId};
use crate::tools::UserEvent;
use crate::tools::views::{CursorEvent, Shaping, Updating};

pub mod app;
pub mod data;
pub mod tools;

#[derive(Debug)]
pub enum ProcessMsg {
	User(UserEvent),
	Internal(SampleAppMsg),
	Error(Box<dyn Error + Send + Sync>),
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut console = Console::start()?;
	let (send_process, recv_process) = unbounded();
	user::connect(&send_process);
	let mut thumper: Thumper<SampleAppMsg> = Thumper::new();
	thumper.connect(send_process.clone());

	let mut app = SampleApp::new();
	let mut app_captors: HashMap<CaptorId, Captor<SampleAppMsg>> = HashMap::new();
	let mut active_captor_id: Option<CaptorId> = None;
	let mut next_process_message: Option<ProcessMsg> = None;
	loop {
		let mut repeat_process_updates: bool = true;
		while repeat_process_updates {
			repeat_process_updates = false;
			let process_messages = {
				let mut process_messages = next_process_message.take().into_iter().collect::<Vec<_>>();
				process_messages.extend(recv_process.try_iter().collect::<Vec<_>>());
				process_messages
			};
			for msg in process_messages {
				match msg {
					ProcessMsg::Error(err) => return Err(err),
					ProcessMsg::Internal(app_msg) => {
						let app_cmd = app.update(app_msg);
						let process_cmd = app_cmd.wrap(ProcessMsg::Internal);
						process_cmd.process(send_process.clone());
					}
					ProcessMsg::User(user_event) => {
						let active_captor = active_captor_id.map(|ref id| app_captors.get(id)).flatten();
						match user_event {
							UserEvent::Quit => return Ok(()),
							UserEvent::Select => {
								if let Some(captor) = active_captor {
									if captor.kind.takes_select {
										if let Some(sender) = &captor.cursor_events_sender {
											let cursor_event = CursorEvent::Select;
											sender.send(cursor_event).expect("send select event");
											repeat_process_updates = true;
										}
									}
								}
							}
							UserEvent::DeleteBack => {
								if let Some(captor) = active_captor {
									if captor.kind.takes_delete_back {
										if let Some(sender) = &captor.cursor_events_sender {
											let cursor_event = CursorEvent::DeleteBack;
											sender.send(cursor_event).expect("send delete-back event");
											repeat_process_updates = true;
										}
									}
								}
							}
							UserEvent::Char(c) => {
								if let Some(captor) = active_captor {
									if captor.kind.takes_chars {
										if let Some(sender) = &captor.cursor_events_sender {
											let cursor_event = CursorEvent::Char(c);
											sender.send(cursor_event).expect("send char event");
											repeat_process_updates = true;
										}
									}
								}
							}
							UserEvent::FocusLeft => {
								if let Some(old_captor) = active_captor_id.map(|it| app_captors.get(&it)).flatten() {
									let mut candidates = app_captors.values()
										.filter(|&captor| captor.frame.right <= old_captor.frame.left)
										.collect::<Vec<_>>();
									// TODO Do a better job selecting leftward captor.
									if !candidates.is_empty() {
										candidates.sort_by_key(|captor| -captor.frame.right);
										let &next_captor = candidates.first().expect("leftward captor");
										active_captor_id = Some(next_captor.id);
										send_process.send(ProcessMsg::Internal(next_captor.pre_focus_msg.clone())).expect("can send process msg");
										repeat_process_updates = true;
									}
								}
							}
							UserEvent::FocusRight => {
								if let Some(old_captor) = active_captor_id.map(|it| app_captors.get(&it)).flatten() {
									let mut candidates = app_captors.values()
										.filter(|&captor| captor.frame.left >= old_captor.frame.right)
										.collect::<Vec<_>>();
									// TODO Do a better job selecting rightward captor.
									if !candidates.is_empty() {
										candidates.sort_by_key(|captor| captor.frame.left);
										let &next_captor = candidates.first().expect("rightward captor");
										active_captor_id = Some(next_captor.id);
										send_process.send(ProcessMsg::Internal(next_captor.pre_focus_msg.clone())).expect("can send process msg");
										repeat_process_updates = true;
									}
								}
							}
							UserEvent::FocusUp => {
								if let Some(old_captor) = active_captor_id.map(|it| app_captors.get(&it)).flatten() {
									let mut candidates = app_captors.values()
										.filter(|&captor| captor.frame.bottom <= old_captor.frame.top)
										.collect::<Vec<_>>();
									// TODO Do a better job selecting higher captor.
									if !candidates.is_empty() {
										candidates.sort_by_key(|captor| -captor.frame.bottom);
										let &next_captor = candidates.first().expect("higher captor");
										active_captor_id = Some(next_captor.id);
										send_process.send(ProcessMsg::Internal(next_captor.pre_focus_msg.clone())).expect("can send process msg");
										repeat_process_updates = true;
									}
								}
							}
							UserEvent::FocusDown => {
								if let Some(old_captor) = active_captor_id.map(|it| app_captors.get(&it)).flatten() {
									let mut candidates = app_captors.values()
										.filter(|&captor| captor.frame.top >= old_captor.frame.bottom)
										.collect::<Vec<_>>();
									// TODO Do a better job selecting lower captor.
									if !candidates.is_empty() {
										candidates.sort_by_key(|captor| captor.frame.top);
										let &next_captor = candidates.first().expect("lower captor");
										active_captor_id = Some(next_captor.id);
										send_process.send(ProcessMsg::Internal(next_captor.pre_focus_msg.clone())).expect("can send process msg");
										repeat_process_updates = true;
									}
								}
							}
						}
					}
				}
			}
		}
		thumper.update(app.get_beats());

		let mut screen = Screen::new(console.width_height());
		let _ = app.shape(screen.to_frame());
		let (screen_fills, captors) = app.get_fills_captors(active_captor_id);
		app_captors = captors.into_iter().map(|it| (it.id, it)).collect::<HashMap<_, _>>();
		match active_captor_id {
			None => {
				if app_captors.is_empty() {
					// No active captor and no captors available to become active captor. Current fills and captors
					// are ready to go!
				} else {
					// No active captor, but at least one available to become active captor. Pick one and let the
					// views re-make their fills and captors knowing the new active captor.
					// TODO Do a better job picking the active captor.
					let captor_id = app_captors.keys().next().expect("ready-captors has captor-id").clone();
					active_captor_id = Some(captor_id);
					if let Some(focus_msg) = app_captors[&captor_id].get_focus_msg() {
						send_process.send(ProcessMsg::Internal(focus_msg)).expect("send process message with focus");
					}
					// Continue even if there is no focus message because we need to regenerate fills and captors
					// using the new active captor.
					continue;
				}
			}
			Some(captor_id) => {
				if app_captors.contains_key(&captor_id) {
					// Active captor is still good. Current fills and captors are ready to go!
				} else {
					// Active captor no longer exists. Forget it and let the view re-make their fills and captors
					// without an active captor.
					active_captor_id = None;
					continue;
				}
			}
		}
		screen.add_fills(screen_fills);
		screen.print_to(&mut console);

		let next_process_msg = recv_process.recv()?;
		next_process_message = Some(next_process_msg);
	}
}
