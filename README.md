# Database pulse test

This is a contrived check that creates a pretend order history table 
and just goes into an infinite loop of writing and reading.

The execution will stop if you CTRL+C out of it, or touch a file called
disable-test.txt in the current working dir.

I wrote this in rust because I want to learn rust. I also wrote it as a client
to use during and RDS parameter group update to prove that the changes being made
will not impact the availability of the database.

See --help or better yet, the code, for details.
## Build notes

### macos
Installed stuff via rustup after installing xcode commandline tools.

### centos7

Fired up a docker image of centos:7 to build binaries for rhel 7 and did:
yum group install "Development Tools"
yum install openssl-devel

Then installed rust via rustup.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh