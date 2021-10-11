# `kubectl-beers`

Insanely useful tool to get beer using `kubectl`.

> Note: this uses the codepoint `f0fc` for the beer icon. This is taken from [Nerdfonts][nerdf].
> Make sure to install such a font if the output is broken.

[nerdf]: https://www.nerdfonts.com/#home

## Installation

Just download the binary from the release page. And put it somewhere in your `PATH` (and rename it
to `kubectl-beers`).

Check that it works using:

```bash
kubectl beers -h
```

### From Source

```bash
git clone https://github.com/f4z3r/kubectl-beers.git
cd kubectl-beers
cargo build --release
cp target/release/kubectl-beers $HOME/.local/bin/
```

## Usage

```
kubectl-beers 0.1.3
Jakob B. <beckmann_jakob@hotmail.fr>
Gets you beer!

USAGE:
    kubectl-beers [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -A, --all-namespaces    It ... well it does nothing really...
    -a, --ascii             Uses ASCII art instead of Unicode.
    -b, --bottled           Gives you bottled beer.
    -h, --help              Prints help information
    -V, --version           Prints version information

OPTIONS:
    -n, --namespace <NAMESPACE>    It namespaces you beer???

SUBCOMMANDS:
    get     Gets you as much beer as you want.
    help    Prints this message or the help of the given subcommand(s)
```
