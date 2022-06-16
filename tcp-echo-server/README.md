# TCP Echo Server

A tiny server that just echoes what it receives.

## Run the Server
To run the server on Mac OS, first install `netcat`:
```
brew install netcat
```

We will use this command line utility to send messages to the server.

Then start the server:
```
cargo run
```

Then in a separate terminal window, send a message to the server using `netcat`:
```
echo "Hallo Welt\!" | nc localhost 8080
```

The server should echo back what you sent:
```
Hallo Welt!
```

## Links
* [TCP echo server assignment](https://ferrous-systems.github.io/teaching-material/assignments/tcp-echo-server.html)
