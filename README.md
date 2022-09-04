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

# run any2cast in any directory with lots of sub directories that contain mp3 files 
$ any2cast
```

### Website
* by default it listens on `0.0.0.0:8080`

<img width="1124" alt="Screen Shot 2022-09-04 at 3 22 28 PM" src="https://user-images.githubusercontent.com/108800/188302286-acdd7a45-cd5d-4c83-aca9-ddf8670202f1.png">

### Podcast App
<img width="986" alt="image" src="https://user-images.githubusercontent.com/108800/188303036-a777fd6a-b047-405a-ac89-e2e40a450e01.png">


# Build

``` bash
cargo build
```

