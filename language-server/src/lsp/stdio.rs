use tokio;
use tokio_stdin_stdout;

pub use jsonrpc_core;

use super::codec::LSPCodec;
use bytes::Bytes;
use jsonrpc_core::IoHandler;
use log;
use std::sync::Arc;
use tokio::prelude::{Future, Stream};
use tokio_codec::{FramedRead, FramedWrite};

pub struct ServerBuilder {
	handler: Arc<IoHandler>,
}

impl ServerBuilder {
	pub fn new<T>(handler: T) -> Self
	where
		T: Into<IoHandler>,
	{
		ServerBuilder {
			handler: Arc::new(handler.into()),
		}
	}

	/// Will block until EOF is read or until an error occurs.
	/// The server reads from STDIN line-by-line, one request is taken
	/// per line and each response is written to STDOUT on a new line.
	pub fn build(&self) {
		let mut core = tokio_core::reactor::Core::new().unwrap();

		let stdin = tokio_stdin_stdout::stdin(0);
		let stdout = tokio_stdin_stdout::stdout(0).make_sendable();

		let framed_stdin = FramedRead::new(stdin, LSPCodec::new());
		let framed_stdout = FramedWrite::new(stdout, LSPCodec::new());
		let handler = self.handler.clone();
		let future = framed_stdin
			.and_then(move |line| {
				log::trace!("Request: {}", std::str::from_utf8(&line).unwrap());
				return process(&handler, std::str::from_utf8(&line).unwrap().into())
					.map_err(|_| unreachable!());
			})
			.forward(framed_stdout)
			.map(|_| ())
			.map_err(|e| panic!("{:?}", e));

		core.run(future).unwrap();
	}
}

/// Process a request asynchronously
fn process(io: &Arc<IoHandler>, input: String) -> impl Future<Item = Bytes, Error = ()> + Send {
	io.handle_request(&input).map(move |result| match result {
		Some(res) => {
			let bytes = Bytes::from(res);
			log::debug!("Response: {:#?}", bytes.clone());
			bytes
		}
		None => {
			log::info!("JSON RPC request produced no response: {:?}", input);
			"".into()
		}
	})
}
