# Ascii Key

A command line tool for showing how to play piano chords.

Inspired by https://github.com/yzhong52/ascii_chord

## Usage

For a single chord:

```
$ askey get G
This is how you play 'G' chord: 
┌─┬─┬┬─┬┬─┬─┬─┬─┬┬─┬─┬─┬─┬┬─┬┬─┬─┐
│ └┬┘└┬┘└┬┘ │ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │
└──┴●─┴──┴●─┴──┴●─┴──┴──┴──┴──┴──┘
```

For multiple chords:

```
$ askey list A D Bm
A
┌─┬─┬┬─┬┬─┬─┬─┬●┬┬─┬─┬─┬─┬┬─┬┬─┬─┐
│ └┬┘└┬┘└┬┘ │ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │
└──┴──┴●─┴──┴──┴──┴●─┴──┴──┴──┴──┘
D
┌─┬─┬┬─┬─┬─┬●┬┬─┬┬─┬─┬─┬─┬┬─┬─┐
│ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │ └┬┘└┬┘ │
└──┴●─┴──┴──┴──┴●─┴──┴──┴──┴──┘
Bm
┌─┬─┬┬─┬┬─┬─┬─┬─┬┬─┬─┬─┬●┬┬─┬┬─┬─┐
│ └┬┘└┬┘└┬┘ │ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │
└──┴──┴──┴●─┴──┴●─┴──┴──┴──┴──┴──┘
```

For all support chords:
```
$ askey all
```

## Installation

Install local version:

```
git clone git@github.com:t-bltg/ascii_key.git
cd ascii_key
cargo install --path .
```

Install from <https://crates.io/crates/askey>:

```
cargo install askey
```

## Development

### Build & Run

Show a single chord:

```
cargo run -- G
```

Show multiple chords:

```
cargo run -- list Em B D Am
```

Show all chords:

```
cargo run -- all
```

### Unit Tests

```
cargo test
```

### Release

```
cargo publish
```

