<div align="center">
    <h1><a href="https://jisho.org/word/%E5%8A%A9%E5%B9%B3">Sukebei 『助平』</a></h1>
</div>

I wanted to learn Rust so I've made this contraption.

This downloads specified six digits for you. Simple.

You can install it yourself, or build it from source:

### Install
```shell
cargo install --git https://github.com/AbstractUmbra/sukebe --branch main sukebe
```
(Requires latest stable [Rust](https://rust-lang.org) compiler).

or...
### Build from source

```shell
git clone https://github.com/AbstractUmbra/Sukebe.git  # HTTPS
git clone git@github.com:AbstractUmbra/Sukebe.git  # SSH

cd Sukebe

cargo build
```

#### How to use it...
```shell
./sukebei -d 177013  # will download this doujin

./sukebei -s cream   # will search titles for "cream" and download them all

./sukebei -a 177013  # will download all "alike" doujins to the specified one.
```
