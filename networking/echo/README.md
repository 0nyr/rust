# Echo server

[std::net | rust](https://doc.rust-lang.org/std/net/index.html)

##### examples

Code obtained there: [echo server in rust](https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo)

### commands

> NOTE: Run server before running client !

##### client

`rust main.rs`: compile client inside `client/`

`./main`: run client inside `client/`

##### server

`cargo run`: compile and run inside `server/`

### result

Client:

```shell
(base)  ❮ onyr ★  kenzae❯ ❮ client❯❯ rustc main.rs 
(base)  ❮ onyr ★  kenzae❯ ❮ client❯❯ ./main 
Successfully connected to server in port 3333

Enter a message to send to the server:
helloworld
Sent following message: helloworld [nb of chars: 10]
Server replied with: helloworld
Reply is ok!

Enter a message to send to the server:
lolilol
Sent following message: lolilol [nb of chars: 7]
Server replied with: lolilol
Reply is ok!

Enter a message to send to the server:
exit
Exiting...Terminated.
```

Server:

```shell
(base)  ❮ onyr ★  kenzae❯ ❮ server❯❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/echo-server`
Server listening on port 3333
New connection: 127.0.0.1:57892
New connection: 127.0.0.1:58234
New connection: 127.0.0.1:58240
New connection: 127.0.0.1:58294
New connection: 127.0.0.1:58300
New connection: 127.0.0.1:58312
New connection: 127.0.0.1:58348
New connection: 127.0.0.1:58354
```
