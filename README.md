# tsql-extractor
Extract T-SQL from parameterized strings

first few lines of code, for my own usage.

To call :

```
cargo run inputfile.sql
```

The "cleaned" string will be printed on STDOUT. Redirect to store it in a file.

Hopefully, I will also format the output using the sqlformat Rust library...
