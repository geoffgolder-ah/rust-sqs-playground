extern crate rusoto_core;
extern crate rusoto_sqs;
extern crate rusoto_credential;
extern crate clokwerk;

use clokwerk::{ScheduleHandle, Scheduler, TimeUnits};
use rusoto_core::Region;
use rusoto_sqs::SqsClient;
use crate::rusoto_sqs::Sqs;
use rusoto_credential::ChainProvider;
use std::{thread,time};
use std::time::Duration;

use rusoto_core::request::HttpClient;


fn main() {
  let provider = ChainProvider::new();
  let dispatcher = match HttpClient::new(){
      Ok(client) => client,
      Err(e) => panic!(e)
  };

  let client = SqsClient::new_with(dispatcher, provider, Region::EuWest1);
  let mut receive_message_input: rusoto_sqs::ReceiveMessageRequest = Default::default();
  receive_message_input.queue_url = "https://sqs.eu-west-1.amazonaws.com/421605451075/testqueue".to_string();

	const DEFAULT_INTERVAL: u32 = 1;

	let interval = DEFAULT_INTERVAL.seconds();

	let mut scheduler = Scheduler::new();
	scheduler.every(interval).run(move || {
		match client.receive_message(receive_message_input.clone()).sync() {
			Ok(output) => {
				  match output.messages {
					Some(message) => {
						match &message[0].body {
							Some(body) => println!("The body is {}", body),
							None => println!("there was no body on the message we received")
						}
					},
					None => println!("the queue is empty, good job"),
				  }
			},
			Err(error) => {
				println!("Error: {:?}", error);
			},
		}
	});

	let handle = scheduler.watch_thread(Duration::from_millis(100));

    let ten_seconds = time::Duration::from_millis(100000);
    thread::sleep(ten_seconds);

    handle.stop();
}
