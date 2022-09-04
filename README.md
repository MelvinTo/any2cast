# What is it
* This is a tool to convert directories of mp3 to podcast, so that podcast client can subscribe and listen
* When running, it will serve a http service where you can get podcast links for each directory.
* Paste the link in podcast client and enjoy!

* Screenshot

``` bash
$ find .
.
./podcast1
./podcast1/1.mp3
./podcast3
./podcast3/3.mp3
./podcast2
./podcast2/2.mp3
```

# Build

``` bash
cargo build
```

# Run

``` bash
# cd to a directory with a list of directories containing mp3 files.

# run any2cast, make sure the host running this service is reachable by visiting <domain>
any2cast --server <domain> --port <port>

# Example
any2cast --server mydns.com --port 9999

# Visit http://mydns.com:9999
```

