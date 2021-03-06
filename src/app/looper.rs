//
// Copyright 2019, Niels Sascha Reedijk <niels.reedijk@gmail.com>
// All rights reserved. Distributed under the terms of the MIT License.
//

use std::collections::VecDeque;
use std::marker::Send;
use std::thread;
use std::time::Duration;

use ::app::{Context, Message, Messenger, B_QUIT_REQUESTED, QUIT};
use ::kernel::ports::Port;
use ::kernel::INFINITE_TIMEOUT;
use ::support::{ErrorKind, Flattenable, HaikuError, Result};

pub trait Handler<A> where A: Send + 'static {
	fn message_received(&mut self, context: &Context<A>, message: &Message);
}

pub struct Looper<A> where A: Send + 'static {
	pub(crate) name: String,
	pub(crate) port: Port,
	pub(crate) message_queue: VecDeque<Message>,
//	pub(crate) handlers: Vec<Box<dyn Handler<A> + Send>>,
	pub(crate) context: Context<A>,
	pub(crate) state: Box<dyn Handler<A> + Send>,
	pub(crate) terminating: bool
}

impl<A> Looper<A> where A: Send + 'static {	
	pub fn name(&self) -> &str {
		&self.name
	}
	
	pub fn get_messenger(&self) -> Messenger {
		Messenger::from_port(&self.port).unwrap()
	}
	
	pub fn run(mut self) -> Result<()> {
		let child = thread::spawn(move || {
			println!("[{}] Running looper", self.name());
			self.looper_task();
		});
		Ok(())
	}

	pub(crate) fn looper_task(&mut self) {
		loop {
			println!("[{}] outer loop", self.name());

			// Try to read the first message from the port
			// This will block until there is a message
			match self.read_message_from_port(INFINITE_TIMEOUT) {
				Ok(message) => self.message_queue.push_back(message),
				Err(e) => {
					println!("[{}] Error getting message: {:?}", self.name(), e); 
					continue;
				}
			}

			// Fetch next messages
			let message_count = self.port.get_count().unwrap();
			for _ in 0..message_count {
				// use timeout of 0 because we know there is a next message
				match self.read_message_from_port(Duration::new(0,0)) {
					Ok(message) => self.message_queue.push_back(message),
					Err(e) => {
						println!("Error getting message: {:?}", e); 
						break;
					}
				}
			}

			// Handle messages, until we have new messages waiting in the
			// queue, this is the inner loop
			let mut dispatch_next_message = true;
			while dispatch_next_message && ! self.terminating {
				let message = self.message_queue.pop_front();
				
				if message.is_none() {
					dispatch_next_message = false;
				} else {
					let message = message.unwrap();
					println!("[{}] Handling message {:?}", self.name(), message);
					
					match message.what() {
						B_QUIT_REQUESTED => {},
						QUIT => { self.terminating = true; },
						_ => {
							// Todo: support handler tokens and targeting
					
		//					for handler in self.handlers.iter_mut() {
		//						handler.message_received(&self.context, &message);
		//					}
							self.state.message_received(&self.context, &message);
						}
					}
				}

				if self.terminating {
					break;
				}
				
				match self.port.get_count() {
					Ok(count) => {
						if count > 0 {
							dispatch_next_message = false;
						}
					},
					Err(e) => println!("Error getting the port count: {:?}", e)
				}
			}
			if self.terminating {
				println!("[{}] terminating looper", self.name());
				break;
			}
			println!("[{}] at the end of the outer loop", self.name());
		}
	}

	fn read_message_from_port(&self, timeout: Duration) -> Result<Message> {
		// TODO: handle B_INTERRUPTED?
		let (type_code, buffer) = self.port.try_read(timeout)?;
		if type_code as u32 == Message::type_code() {
			let message = Message::unflatten(&buffer)?;
			Ok(message)
		} else {
			Err(HaikuError::new(ErrorKind::InvalidData, "the data on the looper's port does not contain a Message"))
		}
	}
}
