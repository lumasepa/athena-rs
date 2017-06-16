extern crate time;
extern crate tokio_core;
extern crate futures;
extern crate tk_bufstream;
extern crate netbuf;
extern crate tk_http;
extern crate tk_listen;
extern crate env_logger;

use std::env;
use std::time::Duration;

use self::tokio_core::reactor::Core;
use self::tokio_core::net::{TcpListener};
use self::futures::{Stream, Future};
use self::futures::future::{FutureResult, ok};

use self::tk_http::{Status};
use self::tk_http::server::buffered::{Request, BufferedDispatcher};
use self::tk_http::server::{Encoder, EncoderDone, Config, Proto, Error};
use self::tk_listen::ListenExt;
use config::config_manager::ConfigurationManager;

const INDEX: &'static str = include_str!("dashboard.html");
const JS: &'static str = include_str!("dashboard.js");

fn service<S>(req: Request, mut encoder: Encoder<S>) -> FutureResult<EncoderDone<S>, Error> {
    if let Some(ws) = req.websocket_handshake() {
        encoder.status(Status::SwitchingProtocol);
        encoder.format_header("Date", time::now_utc().rfc822()).unwrap();
        encoder.add_header("Server",
                     concat!("tk_http/", env!("CARGO_PKG_VERSION"))
        ).unwrap();
        encoder.add_header("Connection", "upgrade").unwrap();
        encoder.add_header("Upgrade", "websocket").unwrap();
        encoder.format_header("Sec-Websocket-Accept", &ws.accept).unwrap();
        encoder.done_headers().unwrap();
        ok(encoder.done())
    } else {
        let (data, ctype) = match req.path() {
            "/dashboard.js" => (JS, "text/javascript; charset=utf-8"),
            _ => (INDEX, "text/html; charset=utf-8"),
        };
        encoder.status(Status::Ok);
        encoder.add_length(data.as_bytes().len() as u64).unwrap();
        encoder.format_header("Date", time::now_utc().rfc822()).unwrap();
        encoder.add_header("Content-Type", ctype).unwrap();
        encoder.add_header("Server",
                     concat!("tk_http/", env!("CARGO_PKG_VERSION"))
        ).unwrap();
        if encoder.done_headers().unwrap() {
            encoder.write_body(data.as_bytes());
        }
        ok(encoder.done())
    }
}


pub fn start_dashboard (config_manager: ConfigurationManager, event_loop: &mut Core) {
    let handle = event_loop.handle();

    let addr = "127.0.0.1:8911".parse().unwrap();
    let listener = TcpListener::bind(&addr, &event_loop.handle()).unwrap();
    let cfg = Config::new().done();

    let done = listener.incoming()
        .sleep_on_error(Duration::from_millis(100), &event_loop.handle())
        .map(move |(socket, addr)| {
            Proto::new(
                socket, &cfg, BufferedDispatcher::new_with_websockets(
                    addr, &handle, service, |out, inp| {
                                   inp.forward(out)
                                       .map(|_| ())
                                       .map_err(|e| error!("Websock err: {}", e))
                               }),
                &handle)
                .map_err(|e| { println!("Connection error: {}", e); })
                .then(|_| Ok(())) // don't fail, please
        })
        .listen(1000);
    event_loop.run(done);
}
