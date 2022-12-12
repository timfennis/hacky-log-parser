# Hacky Log Parser

This is a hacky log parser that parses JSON logs without parsing JSON.

It is highly encouraged to create a release build before running this program.

To create a file with all the paths found in the log files in the directory:

```bash
cargo build --release
target/release/log-parser directory-with-logs/ > output.txt
```

