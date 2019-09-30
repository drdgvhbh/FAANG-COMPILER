use bytes::{BufMut, Bytes, BytesMut};
use regex::bytes::Regex;
use std::io;
use tokio_io::_tokio_codec::{Decoder, Encoder};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LSPCodec;

impl LSPCodec {
	pub fn new() -> LSPCodec {
		LSPCodec
	}
}

impl Decoder for LSPCodec {
	type Item = BytesMut;
	type Error = io::Error;

	fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<BytesMut>, io::Error> {
		lazy_static! {
			static ref HEADER_CONTENT_SEP_RE: Regex = Regex::new("(\r\n\r\n).+").unwrap();
			static ref CONTENT_LENGTH_SEP_RE: Regex =
				Regex::new("Content-Length: ([0-9]+)").unwrap();
		}

		let buf_clone = &buf.clone();

		match (
			HEADER_CONTENT_SEP_RE.captures(buf_clone),
			CONTENT_LENGTH_SEP_RE.captures(buf_clone),
		) {
			(Some(sep_match), Some(content_length_cap)) => {
				let cl_match = content_length_cap.get(1).unwrap();
				let cl = std::str::from_utf8(&buf[cl_match.start()..cl_match.end()])
					.unwrap()
					.parse::<usize>()
					.unwrap();
				buf.advance(sep_match.get(1).unwrap().end());

				Ok(Some(buf.split_to(cl)))
			}
			_ => Ok(None),
		}
	}
}

impl Encoder for LSPCodec {
	type Item = Bytes;
	type Error = io::Error;

	fn encode(&mut self, data: Bytes, buf: &mut BytesMut) -> Result<(), io::Error> {
		if data.len() > 0 {
			let cl_header = format!("Content-Length: {}\r\n\r\n", data.len()).into_bytes();
			buf.reserve(cl_header.len() + data.len());
			buf.put(cl_header);
			buf.put(data);
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;

	#[test]
	fn decodes_correctly() {
		let mut codec = LSPCodec::new();
		let mut stream = BytesMut::default();

		fn putting_content_length_without_data_should_do_nothing(
			stream: &mut BytesMut,
			codec: &mut LSPCodec,
		) {
			stream.put(&b"Content-Length: 69"[..]);
			assert_eq!(codec.decode(stream).expect("should decode"), None);
			assert_eq!(stream, &BytesMut::from(&b"Content-Length: 69"[..]));
		}
		putting_content_length_without_data_should_do_nothing(&mut stream, &mut codec);

		fn put_data_longer_than_content_length_should_only_return_cl_bytes(
			stream: &mut BytesMut,
			codec: &mut LSPCodec,
		) {
			stream.extend(
				&format!(
				"\r\n\r\n{}Content-Length: {}\r\n\r\n",
				"{\"jsonrpc\": \"2.0\", \"method\": \"subtract\", \"params\": [42, 23], \"id\": 1}",
				61
			)
				.into_bytes()[..],
			);
			assert_eq!(
			codec.decode(stream).expect("it should decode"),
			Some(BytesMut::from(&b"{\"jsonrpc\": \"2.0\", \"method\": \"subtract\", \"params\": [42, 23], \"id\": 1}"[..]))
		);
			assert_eq!(stream, &BytesMut::from(&b"Content-Length: 61\r\n\r\n"[..]));
		}
		put_data_longer_than_content_length_should_only_return_cl_bytes(&mut stream, &mut codec);
	}

	#[test]
	fn encodes_prefixes_data_with_content_length() {
		let mut codec = LSPCodec::new();
		let mut output_stream = BytesMut::default();

		let data = Bytes::from(&b"{\"jsonrpc\":\"2.0\",\"error\":{\"code\":-32601,\"message\":\"Method not found\"},\"id\":1}"[..]);
		codec
			.encode(data, &mut output_stream)
			.expect("it should encode");

		assert_eq!(output_stream, Bytes::from(&b"Content-Length: 77\r\n\r\n{\"jsonrpc\":\"2.0\",\"error\":{\"code\":-32601,\"message\":\"Method not found\"},\"id\":1}"[..]));
	}
}
