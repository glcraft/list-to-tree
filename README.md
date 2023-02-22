# list-to-tree

Convert list of text separated by line into a tree. 

Each repetition in slice of text are detected and branched into list of branches or leaf to make the list optimized for storage or research.

For example, let say wi have this list :
```
all
alias
abra
abro
```
The output `list-to-tree --format rust --pretty` will be 
```
Branch(
    "a",
    [
        Branch(
            "br",
            [
                Leaf(
                    "a",
                ),
                Leaf(
                    "o",
                ),
            ],
            "l",
            [
                Leaf(
                    "ias",
                ),
                Leaf(
                    "l",
                ),
            ],
        ),
    ],
)
```

The program was first designed to make big optimized regex of keywords for TextMate grammar (used in the [nushell vscode extension](https://github.com/nushell/vscode-nushell-lang))

## Usage

```
list-to-tree <--format <Format>> [--input <path>] [--output <path>] [--pretty] 
```

### Parameters

- `format` : Format of the output. Possible value :
  - `regex` : Output the tree into a regex
  - `rust` : Output the tree into rust ast
- `input` : Path to the file containing the list of words. Default to stdin
- `output` : Path to the file where the output will be written. Default to stdout
- `pretty` : Pretty print the output (for `rust` format). Default to false

## Build

Since it's using rust cargo, all you have to do is `cargo b --release` to build the program.