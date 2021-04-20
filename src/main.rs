use std::{env, io::ErrorKind, net::UdpSocket, thread, time::Duration};

fn main() {
  let args = env::args().into_iter().collect::<Vec<String>>();

  let address = &args[1];

  let message = &args[2];

  let port = 5546;

  let socket = UdpSocket::bind(("0.0.0.0", port)).expect("Failed to bind socket");

  socket
    .set_nonblocking(true)
    .expect("Failed to enter non-blocking mode");

  // Poll for data every 5 milliseconds for 5 seconds.
  let mut buffer = [0u8; 1024];

  loop {
    let sent = socket
      .send_to(message.as_bytes(), (address.as_str(), port))
      .unwrap();
    assert_eq!(sent, message.len());

    let result = socket.recv(&mut buffer);

    match result {
      // If `recv` was successfull, print the number of bytes received.
      // The received data is stored in `buffer`.
      Ok(_) => println!("Message recieved: {}", String::from_utf8_lossy(&buffer)),
      // If we get an error other than "would block", print the error.
      Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
        println!("Something went wrong: {}", err)
      },
      // Do nothing otherwise.
      _ => {},
    }

    // REMOVE FOR EXTRA "PERFORMANCE"
    thread::sleep(Duration::from_millis(5));
  }
}
