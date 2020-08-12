// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use async_chat::client::runner;
use async_chat::server::{accept_loop, task, Result};
use structopt::StructOpt;

#[derive(StructOpt)]
struct App {
    #[structopt(short = "s", long = "server")]
    is_server: bool,
    // TODO: make port and host configurable
    // #[structopt(short = "p", long = "port")]
    // port: u16,
}

enum Mode {
    Client,
    Server,
}

fn main() -> Result<()> {
    let app = App::from_args();
    let mut mode = Mode::Client;
    if app.is_server {
        mode = Mode::Server;
    }
    match mode {
        Mode::Client => {
            let client_fut = runner("localhost:8080");
            task::block_on(client_fut)?;
        }
        Mode::Server => {
            let server_fut = accept_loop("localhost:8080");
            task::block_on(server_fut)?;
        }
    }
    Ok(())
}
