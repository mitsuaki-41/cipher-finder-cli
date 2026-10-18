# cipher-finder-cli

A command-line tool written in Rust for gematria cipher discovery, which searches for an alphabet cipher matching a set of user-provided words and target values.

## Features

- Randomly permutes the integers from 1 to 26 to map values to letters (a-z). It then checks if the Gematria value of the supplied word equals the specified target value.
- Accept multiple target pairs in a single invocation.
- Print the discovered cipher that match the criteria as letter-value pairs.

## Requirements

- Rust 1.99.0+

Install Rust on your system:https: //rust-lang.org/tools/install/

## Build

Clone the repository and build the project:

```bash
git clone https://github.com/mitsuaki-41/cipher-finder-cli
cd cipher-finder-cli
cargo build --release
```

The executable will be available at: `target/release/cipher-finder-cli`

Now you can also install:

```bash
cargo install --path .
```

## Usage

Pass target word-value pairs using `--targets-input` (or `-t`). Each pair must use the format `word=value`. Separate multiple pairs with spaces.

```bash
cipher-finder-cli --targets-input "cat=24 dog=26"
```

Short option:

```bash
cipher-finder-cli -t "cat=24 dog=26"
```

You can also provide the option more than once:

```bash
cipher-finder-cli \
  -t "cat=24" \
  -t "dog=26"
```

## Output

During the search, the program displays the current attempt count and percentage of the configured attempt limit.

If a matching cipher is found, the output includes the elapsed time and the mapping in the following format:

```text
a(12), b(4), c(21), ...
```

If the search limit is reached without a match, the program reports that no solution was found.

## Limitations

- The program tests at most 5,000,000 candidate permutations per run. If no candidate satisfies all constraints within that limit, it reports that no solution was found.
- The program does not guarantee that it will find a solution, even when a solution exists.
- Only English letters a through z contribute to word values.
- Invalid numeric values are currently parsed as 0; check your input carefully.

## Uninstall

```bash
cargo uninstall cipher-finder-cli
rm -r cipher-finder-cli
```

## License

MIT
