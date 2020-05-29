# Database pulse test

This is a contrived check that creates a pretend order history table 
and just goes into an infinite loop of writing and reading.

The execution will stop if you CTRL+C out of it, or touch a file called
disable-test.txt in the current working dir.

In the current working dir, this reads a simple conf file that specifies the database url to connect to.

## Example ini config
[config]
db_url = "mysql://root:@localhost:3306/pulse_test"


## Build notes

### macos
Installed stuff via rustup after installing xcode commandline tools;

### centos7

Fired up a docker image and did a 
yum group install "Development Tools"
yum install openssl-devel

Then installed rust via rustup.