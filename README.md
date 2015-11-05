# StartLearnRust

## install
install rust of current stable version

```
$ curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

## uninstall 
after install procedure, it will saved to /usr/local/lib/rustlib/uninstall.sh... if you want to rm rust, just run it
```
$ sudo /usr/local/lib/rustlib/uninstall.sh
```
## how to use
compile source code to binary

```
$ cd ex-01-hello
$ rustc hello.rs
```

rustc will generate a binary called `hello`, then you can just run it as bellow

```
$ ./hello
Hello World!
```

All the example code is referenced from [Offical Website](http://rustbyexample.com/index.html)
