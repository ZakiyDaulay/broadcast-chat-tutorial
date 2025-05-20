## 3 Client terminals and 1 server terminal to show chat broadcast

![Image](https://github.com/user-attachments/assets/6b10b1bd-5943-4e81-9e3b-67e5cfcc32ed)

To run the broadcast chat application, we first run the server terminal, then run the 3 client
terminals. Each client will connect to the server and wait for the user input. Once the client terminals are 
connected, We can type a message and that message will appear on the other terminals, and it will show which client it came from

This asynchronous chat application is built with `Tokio` and `tokio-websockets`. 

## Modifying the websocket port
![Image](https://github.com/user-attachments/assets/5b2cdb7c-7837-41ef-b282-6005f3587d9f)

We use the WebSocket protocol (ws://) to connect clients to the server. 
The server listens to port 8080, and the client connects to the same address using ClientBuilder::from_uri(...). 
By modifying the server and client to the same port, we ensure there will be a proper connection. 

## Add some information to client
![Image](https://github.com/user-attachments/assets/15491b45-d033-48e0-88d9-345ad47deb08)

I change one line in both `server.rs` and `client.rs`

In `server.rs` I modified the line `println!("New connection from {addr:?}");` to become `println!("New connection in Zakiy's Computer from {addr}");`

In `client.rs` I modified the line `println!("> {text}");` to become `println!("New connection in Zakiy's computer from{addr:?}");`

