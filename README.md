Koumakan
====
Personal website, written in Rust.

## Framwork
* DataBase: sqlite3
* ConnectPool: r2d2
* ORM: diesel
* Web: rocket

## Building Instructions
Build with cargo and rust nightly version.
``` shell
$ sqlite3 koumakan.db
> .read koumakan.sql
> .exit
$ cargo build
```
