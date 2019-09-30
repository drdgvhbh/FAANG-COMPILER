import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions
} from "vscode-languageclient";
let client: LanguageClient;
import { workspace } from "vscode";

export async function activate() {
  let config = workspace.getConfiguration("faang");
  let serverExe = config.get("language-server.path", "faang_language-server");

  let serverOptions: ServerOptions = {
    run: { command: serverExe },
    debug: { command: serverExe }
  };

  // Options to control the language client
  let clientOptions: LanguageClientOptions = {
    // Register the server for plain text documents
    documentSelector: [
      {
        pattern: "**/*.faang"
      }
    ]
  };

  // Create the language client and start the client.
  client = new LanguageClient(
    "faang_language_server",
    "Faang Language Server",
    serverOptions,
    clientOptions
  );

  // Start the client. This will also launch the server
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
