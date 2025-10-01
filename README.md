# hermanha-gui

I really didnt have much time to do this, so its absolutly discusting to watch the code. A lot of just bruteforced functionality added here and there, and probably a lot of duplicate code. I hope that is ok for this time. It works:D

# How it works
the server side needs to check what IP address it's on.

one players runs:
```
cargo run -- server own_ip:port
```
example: 
```
cargo run -- server 192.168.0.1:8080
```

the other player runs:
```
cargo run -- client server_ip:port
```
use the server ip and port, same example:

```
cargo run -- client 192.168.0.1:8080
```

the client will start as white, the server side will start as black.

# If playing with another application together with mine
DON'T do castling, it won't work!!!!!! and it will break:D

DON'T promote to anything other than a queen, it won't work!!!!! and it will break:D

API doesnt support castling or promotions to other than queen.
