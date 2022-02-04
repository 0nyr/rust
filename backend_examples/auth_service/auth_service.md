# Auth service (server) in Rust

### Useful links

[Diesel (Rust) Doc](https://docs.diesel.rs/master/diesel/index.html#modules) 

[Diesel getting started | official](https://diesel.rs/guides/getting-started/) 

### Useful tutos

[tuto 1 - creating the app | Medium](https://medium.com/swlh/creating-a-web-and-api-authentication-service-in-rust-55d0b0a848d)

### TODO

* [X][start app creation](https://medium.com/swlh/creating-a-web-and-api-authentication-service-in-rust-55d0b0a848d)
* [ ][Adding our models](https://medium.com/swlh/creating-a-web-and-api-authentication-service-in-rust-55d0b0a848d)

### commands

`cargo-watch -x run` : recompile and run when a file changes.



### install cli tools

`cargo install cargo-watch` : install cargo-watch cli. Used to automaticaly recompile the project wehn file changes.

`cargo install diesel_cli` : install `diesel` cli. Used for db migration and setup. Here we use PostgreSQL-12. In our case, we used `cargo install diesel_cli --no-default-features --features postgres` because for whatever reason, it didn't detect that I had `mysql` already installed.

> **Error :** `/usr/bin/ld: cannot find -lpq`. I have been struggling on this compilation error of the ``diesel_cli`` for hours. Thanks to @sander#1161, the fix was to install a postgres client, here with: `sudo apt-get install libpq-dev`.

### setup db connection with diesel

> **NB :** Use the user and password of the db, not the UNIX one !

1) Create a`.env` file in the root directory of the project.
2) Go there as a worling directory and use`source .env` to load ENV variables in the terminal.
3) Call the`diesel setup --database-url $DATABASE_URL` command. Normally everything should work fine.

```shell
 ❮ onyr ★  kenzae❯ ❮ auth_service❯❯ source .env
 ❮ onyr ★  kenzae❯ ❮ auth_service❯❯ echo $DATABASE_URL 
postgres://postgres:63ve7Vs294Xd27b@localhost/auth_service
 ❮ onyr ★  kenzae❯ ❮ auth_service❯❯ diesel setup --database-url $DATABASE_URL
```
