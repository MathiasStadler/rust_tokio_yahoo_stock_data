# setup for project

[no_run](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html)

## create folder ans cargo init

### switch home directory

```bash,no_run
cd
```

- check i'm in my home folder

```bash,no_run,ignore
pwd
```

### mkdir project directory and change inside it

```bash,no_run,ignore
mkdir rust_tokio_yahoo_stock_data && cd $_
```

## add needed rust crates to project, we will use [cargo-edit](https://crates.io/search?q=cargo-edit)

- thirtyfour

```bash,no_run,ignore
cargo add thirtyfour
```

- tokio

```bash,no_run,ignore
cargo add tokio --features full
```

- csv
-- write data to file

```bash,no_run,ignore
cargo add csv
```

## build the project

```bash,no_run,ignore
cargo build
```

## update  project

```bash,no_run,ignore
cargo update 
```

