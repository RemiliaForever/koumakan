Koumakan
====
Personal website backend, written in Rust.

frontend: [koumakan-frontend](https://github.com/RemiliaForever/koumakan-frontend)

## Framwork
* DataBase: sqlite3
* ConnectionPool: r2d2
* ORM: diesel
* Web: rocket
* Email: lettre

## Building Instructions
Build with cargo and rust nightly version.
``` shell
$ sqlite3 koumakan.db
> .read sql/koumakan.sql
> .exit
$ cargo build
```
