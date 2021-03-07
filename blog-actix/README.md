`$ cargo new blog-actix`

## Installing the Diesel CL
`$ cargo install diesel_cli --no-default-features --features "sqlite-bundled"`

### For diesel installing error [see](https://github.com/diesel-rs/diesel/issues/487)
`= note: LINK : fatal error LNK1181: cannot open input file 'sqlite3.lib'`

1. diesel setup
1. To see all migrations and whether they have  been applied we use the list subcommand.\
`$ diesel migration list`
1. To run all pending migrations we use the run subcommand
`$ diesel migration run`

## Users
1. The first step is to add a migration that will create the database table users to hold our users:\
    `$ diesel migration generate create_users`
    ```   
    CREATE TABLE users (
       id       INTEGER PRIMARY KEY NOT NULL,
       username VARCHAR             NOT NULL
    )  
    ```
1. The corresponding down.sql file should perform whatever transformations are necessary to undue what happens in up.sql. In this case as the up migration is creating a table, we can drop the table in our down migration:
   `DROP TABLE users`
1. We create yet another migration, this time to add an index to our users table.
    `$ diesel migration generate index_username`
1. Then we add the code to create the index to up.sql:
    `CREATE UNIQUE INDEX username_unique_idx ON users (username)`
1. As before, we want our down migration to reverse what we did in up, so we drop the index in down.sql:
    `DROP INDEX username_unique_idx`
   
1. Run migrations
  `diesel migration run`


#[derive(Debug)]
{:?} yazmak için kullanılır


### Yeni kullanıcı oluştur
```
curl -s -X POST -H "Content-Type: application/json" -d "{\"username\":\"Frank\"}" http://localhost:8998/users
```
{"id":1,"username":"Frank"}

### Yeni Bir kullanıcı daha oluştur
```
curl -s -X POST -H "Content-Type: application/json" -d "{\"username\":\"Bob\"}" http://localhost:8998/users
```
{"id":2,"username":"Bob"}

### Kullanıcı adı ile kayıt bulmak
```
curl -s -H "Content-Type: application/json" http://localhost:8998/users/find/Frank
```

### Primary key ile kayıt bulmak
```
curl -s -H "Content-Type: application/json" http://localhost:8998/users/1
```

### Olmayan bir kayıt sorgulamak
```
curl -s -H "Content-Type: application/json" http://localhost:8998/users/find/Steve
```