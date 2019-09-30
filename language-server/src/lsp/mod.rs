mod codec;
mod stdio;

use self::stdio::{
	jsonrpc_core::{self, serde_json, IoHandler, Params},
	ServerBuilder,
};
use log::{debug, trace};
pub use lsp_types::*;
use std::sync::{Arc, Mutex, MutexGuard};

pub trait LSPHandler {
	fn initialize(&self, params: InitializeParams) -> InitializeResult;
	fn initialized(&self, params: InitializedParams);
	fn text_document_completion(&self, params: CompletionParams) -> CompletionResponse;
	fn text_document_did_open(&self, params: DidOpenTextDocumentParams);
	fn text_document_did_change(&self, params: DidChangeTextDocumentParams);
	fn text_document_did_save(&self, params: DidSaveTextDocumentParams);
}

macro_rules! add_method {
	($handler:expr, $io:expr, $name:expr, $handler_call:expr) => {{
		let handler_c = $handler.clone();
		$io.add_method($name, move |params: Params| {
			let invalid_params_err = jsonrpc_core::Error::invalid_params_with_details(
				"Invalid Params",
				" https://microsoft.github.io/language-server-protocol/ ",
			);
			trace!("[JSON-RPC Method] {} invoked", $name);
			let handler = handler_c
				.lock()
				.map_err(|_| jsonrpc_core::Error::internal_error())?;
			let init_params = serde_json::from_value(params.into()).map_err(|p| {
				log::debug!("failed to parse params {:#?}", p);
				invalid_params_err.clone()
			})?;
			let result = $handler_call(handler, init_params);

			serde_json::to_value(result).map_err(|_| jsonrpc_core::Error::internal_error())
			})
		}};
}

macro_rules! add_notif {
	($handler:expr, $io:expr, $name:expr, $handler_call:expr) => {{
		let handler_c = $handler.clone();
		$io.add_notification($name, move |json_params: Params| {
			trace!("[JSON-RPC Method] {} invoked", $name);
			match handler_c.lock() {
				Ok(handler) => match serde_json::from_value(json_params.into()) {
					Err(err) => debug!(
						"{}",
						format!("failed to parse {} parameters, {}", $name, err)
					),
					Ok(initd_params) => $handler_call(handler, initd_params),
				},
				Err(err) => {}
				}
		});
		}};
}

pub fn start_lsp_server<H: LSPHandler>(handler: Arc<Mutex<H>>)
where
	H: std::marker::Send,
	H: 'static,
{
	let mut io = IoHandler::default();

	add_method!(
		handler,
		io,
		"initialize",
		|handler: MutexGuard<H>, params| handler.initialize(params)
	);
	add_method!(
		handler,
		io,
		"textDocument/completion",
		|handler: MutexGuard<H>, params| handler.text_document_completion(params)
	);
	add_notif!(
		handler,
		io,
		"initialized",
		|handler: MutexGuard<H>, params| handler.initialized(params)
	);
	add_notif!(
		handler,
		io,
		"textDocument/didOpen",
		|handler: MutexGuard<H>, params| handler.text_document_did_open(params)
	);
	add_notif!(
		handler,
		io,
		"textDocument/didChange",
		|handler: MutexGuard<H>, params| handler.text_document_did_change(params)
	);
	add_notif!(
		handler,
		io,
		"textDocument/didSave",
		|handler: MutexGuard<H>, params| handler.text_document_did_save(params)
	);
	ServerBuilder::new(io).build();
}
