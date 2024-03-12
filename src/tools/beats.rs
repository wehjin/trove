use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::ptr::addr_eq;
use std::sync::Arc;
use std::thread;

use crossbeam::channel::{Receiver, Sender, unbounded};
use crossbeam::select;
use rand::random;

pub fn signal<Value: Send + 'static, Msg: Send + 'static>(into_msg: fn(Value) -> Msg) -> (Sender<Value>, Beat<Msg>) {
	let (send_signal, receive_signal) = unbounded::<Value>();
	let beat = Beat {
		id: random(),
		start: Arc::new(move |send_beat: Sender<Msg>| {
			let receive_signal = receive_signal.clone();
			let (end_beat, beat_ended) = unbounded::<()>();
			thread::spawn(move || {
				loop {
					select! {
						recv(beat_ended) -> _ => break,
						recv(receive_signal) -> signal => {
							if let Ok(signal) = signal {
								let msg = into_msg(signal);
								send_beat.send(msg).expect("Send beat");
							} else {
								break;
							}
						},
					}
				}
			});
			EndBeat(end_beat)
		}),
	};
	(send_signal, beat)
}

#[derive(Clone)]
pub struct EndBeat(Sender<()>);

impl EndBeat {
	pub fn send(&self) {
		self.0.send(()).expect("Send EndBeat");
	}
}

pub struct Beat<Msg> {
	pub id: usize,
	pub start: Arc<dyn Fn(Sender<Msg>) -> EndBeat + Send + Sync>,
}

impl<Msg: Send + 'static> Beat<Msg> {
	pub fn wrap<OuterMsg: Send + 'static>(self, wrap_msg: fn(Msg) -> OuterMsg) -> Beat<OuterMsg> {
		Beat {
			id: self.id,
			start: Arc::new(move |outer_msg_sender: Sender<OuterMsg>| {
				let inner_start = self.start.clone();
				let (inner_msg_sender, inner_msg_receiver) = unbounded::<Msg>();
				let (cancel_sender, cancel_receiver) = unbounded::<()>();
				thread::spawn(move || {
					let inner_end_beat = inner_start.deref()(inner_msg_sender);
					loop {
						select! {
							recv(cancel_receiver) -> _ => break,
							recv(inner_msg_receiver) -> result => {
								if let Ok(inner_msg) = result {
									let outer_msg = wrap_msg(inner_msg);
									outer_msg_sender.send(outer_msg).expect("send outer message");
								} else {
									break;
								}
							},
						}
					}
					inner_end_beat.send();
				});
				EndBeat(cancel_sender)
			}),
		}
	}
}

impl<Msg> Clone for Beat<Msg> {
	fn clone(&self) -> Self {
		Self {
			id: self.id,
			start: self.start.clone(),
		}
	}
}

impl<Msg> PartialEq<Self> for Beat<Msg> {
	fn eq(&self, other: &Self) -> bool {
		let self_start: *const dyn Fn(Sender<Msg>) -> EndBeat = self.start.as_ref();
		let other_start: *const dyn Fn(Sender<Msg>) -> EndBeat = self.start.as_ref();
		self.id == other.id && addr_eq(self_start, other_start)
	}
}

impl<Msg> Eq for Beat<Msg> {}

impl<Msg> Hash for Beat<Msg> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
		let self_start: *const dyn Fn(Sender<Msg>) -> EndBeat = self.start.as_ref();
		self_start.hash(state);
	}
}


pub struct Thumper<Msg> {
	end_beats: HashMap<Beat<Msg>, EndBeat>,
	beat_sender: Sender<Msg>,
	beat_receiver: Receiver<Msg>,
}

impl<Msg> Thumper<Msg> {
	pub fn new() -> Self {
		let (beat_sender, beat_receiver) = unbounded::<Msg>();
		Self {
			end_beats: HashMap::new(),
			beat_sender,
			beat_receiver,
		}
	}
}

impl<Msg> Thumper<Msg> {
	pub fn contains_beat(&self, beat: &Beat<Msg>) -> bool {
		self.end_beats.contains_key(beat)
	}
	pub fn beats_receiver(&self) -> &Receiver<Msg> { &self.beat_receiver }
}

impl<Msg> Thumper<Msg> {
	pub fn update(&mut self, beats: Vec<Beat<Msg>>) {
		let keep_beats = beats.into_iter().collect::<HashSet<Beat<Msg>>>();
		let expired_beats = self.end_beats.keys()
			.cloned()
			.filter(|old| !keep_beats.contains(old))
			.collect::<Vec<_>>();
		for expired_beat in expired_beats {
			if let Some(end_beat) = self.end_beats.remove(&expired_beat) {
				end_beat.send();
			}
		}
		for beat in keep_beats {
			if !self.end_beats.contains_key(&beat) {
				let end_beat = (beat.start)(self.beat_sender.clone());
				self.end_beats.insert(beat, end_beat);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::tools::beats::{signal, Thumper};

	#[derive(Debug, Eq, PartialEq)]
	enum Msg {
		SetCount(usize)
	}

	#[test]
	fn thumper_recognizes_cloned_beat() {
		let (_, signal_beat) = signal(Msg::SetCount);
		let mut thumper = Thumper::new();
		thumper.update(vec![signal_beat.clone()]);
		assert!(thumper.contains_beat(&signal_beat));
	}

	#[test]
	fn thumper_removes_expired_beats() {
		let (_, signal_beat) = signal(Msg::SetCount);
		let mut thumper = Thumper::new();
		thumper.update(vec![signal_beat.clone()]);
		thumper.update(vec![]);
		assert!(!thumper.contains_beat(&signal_beat));
	}

	#[test]
	fn thumper_delivers_signal() {
		let (signal_sender, signal_beat) = signal(Msg::SetCount);
		let mut thumper = Thumper::new();
		thumper.update(vec![signal_beat]);
		signal_sender.send(3).unwrap();
		assert_eq!(Msg::SetCount(3), thumper.beat_receiver.recv().unwrap());
	}

	#[test]
	fn thumper_delivers_wrapped_signal() {
		#[derive(Debug, Eq, PartialEq)]
		struct ForInner(pub Msg);
		let (signal_sender, signal_beat) = signal(Msg::SetCount);
		let outer_beat = signal_beat.wrap(ForInner);
		let mut thumper = Thumper::new();
		thumper.update(vec![outer_beat]);
		signal_sender.send(3).unwrap();
		assert_eq!(ForInner(Msg::SetCount(3)), thumper.beat_receiver.recv().unwrap());
	}
}

