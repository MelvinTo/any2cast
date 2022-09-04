# What is it
* This is a tool to convert directories of mp3 to podcasts, so that podcast client can subscribe and listen
* When running, it will serve a http service where you can get podcast links for each directory.
* Paste the link in podcast client and enjoy!

# Screenshot

``` bash
$ find .
.
./podcast1
./podcast1/1.mp3
./podcast3
./podcast3/3.mp3
./podcast2
./podcast2/2.mp3

$ any2cast
```

### Website
<img width="1124" alt="Screen Shot 2022-09-04 at 3 22 28 PM" src="https://user-images.githubusercontent.com/108800/188302286-acdd7a45-cd5d-4c83-aca9-ddf8670202f1.png">

### Podcast App
<img width="986" alt="image" src="https://user-images.githubusercontent.com/108800/188303036-a777fd6a-b047-405a-ac89-e2e40a450e01.png">


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

