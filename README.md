# SimSync v1.0.0

A blazingly fast one way synchronization tool written in Rust🦀

I have built this to synchronize my Music folder on my phone with the one on my computer. I did not want ot do it manually cuz there are hundreds of em

I have thought about how to transfer files themselves, and ended up on good old http servers. i did not want to bother with the server itself so i choose python http server

to get the server up and running you simply have to run this in the remote directory itself:
```
python -m http.server
```

#

To get started with this you have to compile the project

1. You have to get the rust environment ready \
You can get rust from https://rustup.rs/

2. In order to compile project run:
```
cargo build --release
```
3. At last, to run the binary:
```
cargo run --release [Host] [Port] [Local Directory]
```

Or, after compiling you can get the binary itself out from target/release/simsync. You can then move it /bin folder or somwhere else you want in your system

You can run with "help" flag to get more info


Most of the code logic is explained in the comments


Feature plans are to add some more helpfull output, and implement multithreading
