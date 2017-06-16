//extern crate futures;
//extern crate tokio_core;
//extern crate tokio_tungstenite;
//extern crate tungstenite;
//extern crate uuid;
//
//use std::cell::RefCell;
//use std::boxed::Box;
//use std::collections::HashMap;
//use std::io::{Error, ErrorKind};
//use std::rc::Rc;
//use std::net::Ipv4Addr;
//use std::marker::Send;
//use self::uuid::Uuid;
//
//use self::futures::*;
//use self::futures::future::*;
//use self::futures::stream::*;
//use self::futures::sink::*;
//
//use self::futures::future;
//use self::futures::Future;
//use self::futures::sync::mpsc::UnboundedSender;
//
//use self::tokio_core::net::{TcpListener, Incoming, TcpStream};
//use self::tokio_core::reactor::{Core, Handle};
//use self::tungstenite::protocol::{Message, WebSocket};
//use self::tungstenite::Error as TungsteniteError;
//use self::tokio_tungstenite::{accept_async, AcceptAsync, WebSocketStream};
//
//type WebSocketSink = SplitSink<WebSocketStream<TcpStream>>;
//
//struct WebSocketServer {
//    listen_ip: Ipv4Addr,
//    port: u32,
//    handle: Handle,
//    on_message_cb: fn(Message, &String) -> (),
//    on_close_cb: fn(String) -> (),
//    connections: Rc<RefCell<HashMap<String, WebSocketSink>>>
//}
//
//impl WebSocketServer {
//    fn new(ip: Ipv4Addr, port: u32, core: &Core, on_message_cb: (fn(Message, &String) -> ()), on_close_cb: fn(String) -> ()) -> WebSocketServer {
//        WebSocketServer {
//            listen_ip: ip,
//            port: port,
//            handle: core.handle(),
//            on_message_cb: on_message_cb,
//            on_close_cb: on_close_cb,
//            connections: Rc::new(RefCell::new(HashMap::new())),
//        }
//    }
//
//    fn start_server(&self) -> BoxFuture<(), ()> {
//        let addr = format!("{}:{}", self.listen_ip, self.port)
//            .parse()
//            .unwrap();
//        // Create a TCP listener we'll accept connections on.
//        let listener: TcpListener = TcpListener::bind(&addr, &self.handle).unwrap();
//        info!("Ws Server Listening on: {}", addr);
//
//        listener.incoming()
//            .for_each(|(socket, addr)| {
//                let socket_handling = self.handle_socket(socket);
//                self.handle.spawn(socket_handling);
//                future::ok(())
//            })
//            .map(|_|{()})
//            .map_err(|_|{()})
//            .boxed()
//    }
//
//    fn handle_socket(&self, socket: TcpStream) -> BoxFuture<(), ()> {
//        accept_async(socket)
//        .map(|ws|{
//            //*self.handle_websocket(ws)
//            ()
//        })
//        .map_err(|e| {
//            error!("Error during the websocket handshake occurred");
//            ()
//        }).boxed()
//
//    }
//
////    fn handle_websocket(&mut self, websocket: WebSocketStream<TcpStream>) -> BoxFuture<(), ()>{
////        let ws_uuid = format!("{}", Uuid::new_v4().hyphenated());
////        info!("New WebSocket connection: {}", ws_uuid);
////
////        let (ws_sink, ws_stream) = websocket.split();
////        self.connections.borrow_mut().insert(ws_uuid, ws_sink);
////
////        let ws_reader = ws_stream.for_each(move |message: Message| {
////            self.on_message(message, &ws_uuid);
////            future::ok(())
////        });
////
////
////        ws_reader.then(move |_| {
////            self.on_close(ws_uuid);
////            future::ok(())
////        }).boxed()
////
////    }
////
////    fn on_message(&mut self, message: Message, ws_uuid: &String) {
////        info!("Received a message from {}: {}", ws_uuid, message);
////        let message_cb = self.on_message_cb;
////        message_cb(message, ws_uuid);
////    }
////
////    fn on_close(&mut self, ws_uuid: String) {
////        info!("Closed websocket {}", ws_uuid);
////        let close_cb = self.on_close_cb;
////        close_cb(ws_uuid);
////    }
//
//}
//
//
//describe! test_configuration_load {
//    before_each {
//
//    }
//    it "Should load the yaml file" {
//
//    }
//}