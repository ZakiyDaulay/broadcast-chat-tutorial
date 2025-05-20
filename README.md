## 3 client terminals and 1 server terminal to show chat broadcast

![Image](https://github.com/user-attachments/assets/6b10b1bd-5943-4e81-9e3b-67e5cfcc32ed)

To run the broadcast chat application, we first run the server terminal, then run the 3 client
terminals. Each client will connect to the server and wait for the user input. Once the client terminals are 
connected, We can type a message and that message will appear on the other terminals, and it will show which client it came from

This asynchronous chat application is built with `Tokio` and `tokio-websockets`. 