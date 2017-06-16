//extern crate amq_protocol;
//extern crate futures;
//extern crate tokio_core;
//extern crate lapin_futures as lapin;
//
//use std::net::Ipv4Addr;
//use self::amq_protocol::types::FieldTable;
//use self::futures::Stream;
//use self::futures::future::Future;
//use self::tokio_core::reactor::Core;
//use self::tokio_core::net::TcpStream;
//use self::lapin::client::ConnectionOptions;
//use self::lapin::channel::{BasicPublishOptions,QueueDeclareOptions, BasicProperties};

#[derive(Debug)]
pub struct AMQPBroker {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub vhost: String,
    pub name: String,
    //handle: Handle,
}
//trait AMQPBrokerTrait {
//    fn get_broker_future(&self);
//}
//impl AMQPBrokerTrait for AMQPBroker {
//    fn get_broker_future(&self) {
//        let addr = format!("{}:{}", self.host, self.port).parse().unwrap();
//
//        TcpStream::connect(&addr, &handle).and_then(|stream| {
//
//            // connect() returns a future of an AMQP Client
//            // that resolves once the handshake is done
//            lapin::client::Client::connect(stream, &ConnectionOptions{
//                username: self.user,
//                password: self.password,
//                vhost: self.vhost,
//                heartbeat: 10,
//            })
//        }).and_then(|client| {
//            client.create_channel()
//        }).and_then(|channel| {
//            let id = channel.id;
//            info!("created channel with id: {}", id);
//
//            // we using a "move" closure to reuse the channel
//            // once the queue is declared. We could also clone
//            // the channel
//            channel.queue_declare("hello", &QueueDeclareOptions::default(), FieldTable::new()).and_then(move |_| {
//                info!("channel {} declared queue {}", id, "hello");
//
//                channel.basic_publish("hello", b"hello from tokio", &BasicPublishOptions::default(),
//                                      BasicProperties::default())
//            })
//        })
//    }
//}
//
//describe! test_amqp_client {
//    before_each {
//
//    }
//    it "Should publish in a queue" {
//
//    }
//}