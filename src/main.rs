//MIT License

//Copyright (c) 2017 Colin Rothfels

//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.

#[macro_use]
extern crate serde_json;
extern crate lsp_client;

use lsp_client::start_language_server;
use std::{
    process::{Child, Command, Stdio},
    time::Duration,
};

const TEST_PROJECT_ROOT: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/test_project/");

fn main() {
    let (mut child, lang_server) = start_language_server(prepare_command());
    // this init blob was copied from the atom client example here:
    // https://github.com/jonathandturner/rls_vscode/blob/master/src/extension.ts
    let init = json!({
        "process_id": "Null",
        "root_path": TEST_PROJECT_ROOT,
        "initialization_options": {},
        "capabilities": {
            "documentSelector": ["rust"],
            "synchronize": {
                "configurationSection": "languageServerExample"
            }
        },
    });

    lang_server.send_request("initialize", &init, |_result| {
        //println!("received response {:?}", result);
    });

    std::thread::sleep(Duration::from_millis(500));

    let msg = json!({});
    lang_server.send_notification("initialized", &msg);

    let path = [TEST_PROJECT_ROOT, "src/foo.rs"].join("");
    let uri = ["file://", &path].join("");
    let text = std::fs::read_to_string(path).unwrap();
    let msg = json!({
        "textDocument": {
          "uri": uri,
          "languageId": "rust",
          "version": 488,
          "text": text
    }});
    lang_server.send_notification("textDocument/open", &msg);
    std::thread::sleep(Duration::from_millis(500));

    let msg = json!({
            "textDocument": {
              "uri": uri,
        },

      "position": {
        "line": 2,
        "character": 12,
      },
      "context": {
        "triggerKind": 3
      }

    });

    use std::time::Instant;
    let now = Instant::now();

    lang_server.send_request("textDocument/completion", &msg, move |res| {
        let elapsed = now.elapsed();
        println!("Autocompletion took: {:.2?}", elapsed);

        //dbg!(res);

        std::process::exit(0);
    });

    child.wait().unwrap();
}

fn prepare_command() -> Child {
    Command::new("rust-analyzer")
        .current_dir(TEST_PROJECT_ROOT)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
}
