use std::{
  collections::HashMap,
  error::Error,
  io::Error as IoError,
  net::SocketAddr,
  result::Result as StdResult,
  sync::{Arc, Mutex},
  thread,
  time::Duration
};

// use clap::{App, Clap};

use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use futures_channel::mpsc::{unbounded, UnboundedSender};
use rppal::gpio::Gpio;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Controller {
  pub id: String,
  pub axes: Vec<String>,
  pub buttons: Vec<bool>
}


// #[derive(Serialize, Deserialize)]
// struct AppConfiguration {
// }

use local_ipaddress;
use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;
type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

// Set up GPIO pin numbers
const A_FWD: u8 = 21;
const A_BWD: u8 = 12;

const B_FWD: u8 = 16;
const B_BWD: u8 = 20;

const C_FWD: u8 = 19;
const C_BWD: u8 = 26;

const D_FWD: u8 = 6;
const D_BWD: u8 = 13;


async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
  
  println!("Incoming TCP connection from: {}", addr);
 
  let ws_stream = tokio_tungstenite::accept_async(raw_stream)
    .await
    .expect("Oops! An error occurred during the websocket handshake");
  
    println!("WebSocket connection established with: {}", addr);
  
  // Insert the write part of this peer to the peer map.
  let (tx, rx) = unbounded();
  peer_map.lock().unwrap().insert(addr, tx);
  let (outgoing, incoming) = ws_stream.split();

  let broadcast_incoming = incoming.try_for_each(|msg| {
    
    let message_text = msg.to_text().unwrap();
    // println!("Got this from {}: {}", addr, &message_text);
  
    let gamepad: Vec<Controller> = serde_json::from_str(message_text).unwrap();
    
    if gamepad[0].axes[1] == "-1.00" {
      drive_forwards().map_err(|error| println!("An error occured while trying to go forwards: {:?}", error)).ok();
    }

    if gamepad[0].axes[1] == "1.00" {
      drive_backwards().map_err(|error| println!("An error occured while trying to go backwards: {:?}", error)).ok();
    }

    if gamepad[0].axes[0] == "-1.00" {
      turn_left().map_err(|error| println!("An error occured while trying to turn left: {:?}", error)).ok();
    }

    if gamepad[0].axes[0] == "1.00" {
      turn_right().map_err(|error| println!("An error occured while trying to turn right: {:?}", error)).ok();
    }

    if gamepad[0].axes[0] == "0.00" && gamepad[0].axes[1] == "0.00" {
      pin_resetter().map_err(|error| println!("An error occured: {:?}", error)).ok();
    }
    
    let peers = peer_map.lock().unwrap();
    
    // Broadcast the message to everyone except ourselves
    let broadcast_recipients = peers
      .iter()
      .filter(|(peer_addr, _)| peer_addr != &&addr)
      .map(|(_, ws_sink)| ws_sink);
    
      for recp in broadcast_recipients {
        recp.unbounded_send(msg.clone()).unwrap();
      }
    
    future::ok(())

  });

  let receive_from_others = rx.map(Ok).forward(outgoing);
  pin_mut!(broadcast_incoming, receive_from_others);
  future::select(broadcast_incoming, receive_from_others).await;
  println!("{} disconnected", &addr);
  peer_map.lock().unwrap().remove(&addr);

}


#[tokio::main]
async fn main() -> StdResult<(), IoError> {

  // let config: AppConfiguration = confy::load("codercar")?;
  // dbg!(config);

  pin_resetter().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let mut addr_pi: String = local_ipaddress::get().unwrap().to_string();

  addr_pi.push_str(":8080");

  let state = PeerMap::new(Mutex::new(HashMap::new()));

  // Create the event loop and TCP listener we'll accept connections on.
  let try_socket = TcpListener::bind(&addr_pi).await;
  let mut listener = try_socket.expect("Failed to bind");
  println!("Listening to incoming connections on: {}", addr_pi);

  // Let's spawn the handling of each connection in a separate task.
  while let Ok((stream, addr_pi)) = listener.accept().await {
    tokio::spawn(handle_connection(state.clone(), stream, addr_pi));
  }

  Ok(())
}

fn gpio_timeout() -> StdResult<(), Box<dyn Error>> {
  thread::sleep(Duration::from_millis(100));
  Ok(())
}

fn pin_resetter() -> StdResult<(), Box<dyn Error>> {

  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _a_fwd = Gpio::new()?.get(A_FWD)?.into_output().set_low();
  let _a_bwd = Gpio::new()?.get(A_BWD)?.into_output().set_low();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _b_fwd = Gpio::new()?.get(B_FWD)?.into_output().set_low();
  let _b_bwd = Gpio::new()?.get(B_BWD)?.into_output().set_low();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _c_fwd = Gpio::new()?.get(C_FWD)?.into_output().set_low();
  let _c_bwd = Gpio::new()?.get(C_BWD)?.into_output().set_low();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _d_fwd = Gpio::new()?.get(D_FWD)?.into_output().set_low();
  let _d_bwd = Gpio::new()?.get(D_BWD)?.into_output().set_low();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  Ok(())
}


fn drive_forwards() -> StdResult<(), Box<dyn Error>> {

  pin_resetter().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _a_fwd = Gpio::new()?.get(A_FWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _b_fwd = Gpio::new()?.get(B_FWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _c_fwd = Gpio::new()?.get(C_FWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _d_fwd = Gpio::new()?.get(D_FWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  Ok(())
}


fn drive_backwards() -> StdResult<(), Box<dyn Error>> {

  pin_resetter().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _a_bwd = Gpio::new()?.get(A_BWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _b_bwd = Gpio::new()?.get(B_BWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _c_bwd = Gpio::new()?.get(C_BWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _d_bwd = Gpio::new()?.get(D_BWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  Ok(())
}


fn turn_left() -> StdResult<(), Box<dyn Error>> {

  pin_resetter().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _a_bwd = Gpio::new()?.get(A_BWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _b_fwd = Gpio::new()?.get(B_FWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _c_bwd = Gpio::new()?.get(C_BWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _d_fwd = Gpio::new()?.get(D_FWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  Ok(())
}


fn turn_right() -> StdResult<(), Box<dyn Error>> {

  pin_resetter().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _a_fwd = Gpio::new()?.get(A_FWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _b_bwd = Gpio::new()?.get(B_BWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _c_fwd = Gpio::new()?.get(C_FWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  let _d_bwd = Gpio::new()?.get(D_BWD)?.into_output().set_high();
  gpio_timeout().map_err(|error| println!("An error occured: {:?}", error)).ok();

  Ok(())
}
