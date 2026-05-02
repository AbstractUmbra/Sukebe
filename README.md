<div align="center">
    <h1><a href="https://jisho.org/word/%E5%8A%A9%E5%B9%B3">Sukebe 『すけべ』</a></h1>
</div>

# Sukebe

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
##### Direct with digits
```shell
./sukebei -d 177013  # will download this doujin
./sukebei --digits 177013  # will download this doujin

./sukebei --digits 177013 320992 # will download both
```
##### Download from tags
This is currently a very simple implementation that only features "inclusion" and sorts by popular.
```sh
./sukebei --tags nakadashi # downloads up to 25 results from this tag search
./sukebei --tags 'big breasts' nakadashi paizuri # downloads up to 25 results that have all 3 tags
```
##### Download from 'alike'
WIP.
