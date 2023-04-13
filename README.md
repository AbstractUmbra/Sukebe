<div align="center">
    <h1><a href="https://jisho.org/word/%E5%8A%A9%E5%B9%B3">Sukebe 『すけべ』</a></h1>
</div>

# Archived Project
## Reasons
NHentai has added CloudFlare protection to their entire domain with some pretty restrictive and strong rules.
Sadly I don't have the time nor patience to devise ways around this. It seems like browser side checks as well as network side checks (ip allowing and the like).

As such I will archive this repo. I have abandoned all faith, and entered there.

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
```shell
./sukebei -d 177013  # will download this doujin

./sukebei -s cream   # will search titles for "cream" and download them all

./sukebei -a 177013  # will download all "alike" doujins to the specified one.
```
