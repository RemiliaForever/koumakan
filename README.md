Koumakan
====
Personal website, written in Rust.

## Framwork
* DataBase: sqlite3
* ConnectionPool: r2d2
* ORM: diesel
* Web: rocket
* Template: handlebars

## Building Instructions
Build with cargo and rust nightly version.
``` shell
$ sqlite3 koumakan.db
> .read sql/koumakan.sql
> .exit
$ cargo build
```
