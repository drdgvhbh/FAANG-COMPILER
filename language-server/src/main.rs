#![feature(rustc_private)]

mod lsp;

#[macro_use]
extern crate lazy_static;

use self::lsp::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct Handler {
	files: HashMap<String, String>,
}

impl LSPHandler for Handler {
	fn initialize(&self, params: InitializeParams) -> InitializeResult {
		InitializeResult {
			capabilities: ServerCapabilities {
				text_document_sync: Some(TextDocumentSyncCapability::Kind(
					TextDocumentSyncKind::Full,
				)),
				hover_provider: None,
				completion_provider: Some(CompletionOptions {
					resolve_provider: Some(true),
					trigger_characters: None,
				}),
				signature_help_provider: None,
				definition_provider: None,
				type_definition_provider: None,
				implementation_provider: None,
				references_provider: None,
				document_highlight_provider: None,
				document_symbol_provider: None,
				workspace_symbol_provider: None,
				code_action_provider: None,
				code_lens_provider: None,
				document_formatting_provider: None,
				document_range_formatting_provider: None,
				document_on_type_formatting_provider: None,
				rename_provider: None,
				document_link_provider: None,
				color_provider: None,
				folding_range_provider: None,
				execute_command_provider: None,
				workspace: None,
			},
		}
	}

	fn text_document_completion(&self, params: CompletionParams) -> CompletionResponse {
		CompletionResponse::Array(vec![CompletionItem {
			label: "println".into(),
			kind: Some(CompletionItemKind::Method),
			detail: None,
			documentation: Some(Documentation::MarkupContent(MarkupContent {
				kind: MarkupKind::Markdown,
				value: "Prints to the standard output, with a newline.\n\n# Example\n\n```println(\"Hello World\")```".into(),
			})),
			deprecated: Some(false),
			preselect: None,
			sort_text: None,
			filter_text: None,
			insert_text: Some("println($1)".into()),
			insert_text_format: Some(InsertTextFormat::Snippet),
			text_edit: None,
			additional_text_edits: None,
			command: None,
			data: None,
		}])
	}

	fn initialized(&self, _params: InitializedParams) {}
	fn text_document_did_open(&self, _params: DidOpenTextDocumentParams) {}
	fn text_document_did_change(&self, _params: DidChangeTextDocumentParams) {}
	fn text_document_did_save(&self, _params: DidSaveTextDocumentParams) {}
}

fn main() {
	let handler = Handler {
		files: HashMap::default(),
	};
	stderrlog::new()
		.module(module_path!())
		.verbosity(5)
		.timestamp(stderrlog::Timestamp::Second)
		.init()
		.unwrap();
	start_lsp_server(Arc::new(Mutex::new(handler)));
}
