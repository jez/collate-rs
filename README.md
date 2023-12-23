# collate

> Command line filter for collating the top and bottom halves of a file

```bash
❯ cat foo.txt
1
2
3
a
b
c

❯ collate < foo.txt
1
a
2
b
3
c

❯ collate < foo.txt | uncollate
1
2
3
a
b
c
```

## Install

To install from GitHub:

```
cargo install -f --git https://github.com/jez/collate-rs
```

To install from a clone:

```bash
git clone git@github.com:jez/collate-rs.git
make install
```

## Testing

```bash
# All the tests:
make test

# One specific test:
make test/06-six.txt.collate.exp
```

## License

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://jez.io/MIT-LICENSE.txt)
