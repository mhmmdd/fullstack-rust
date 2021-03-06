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

# Post Oluşturma
1. Migration dosyaları oluşturulur
```
diesel migration generate create_posts
```
2. Migration için Up.sql ve Down.sql dosyalarına sql kodları yazılır.
    1. blog-actix\migrations\2021-03-07-124450_create_posts\up.sql
    2. blog-actix\migrations\2021-03-07-124450_create_posts\down.sql
3. Migration çalıştırılır.
```
diesel migration run
```
4. schema.rs dosyasının otomatik düzenlendiği görülür

The concept of an association in Diesel is always from child to parent, i.e. there is no “has many” like in other ORMs.

# Yorum (Comment) Oluşturma
1. Migration dosyaları oluşturulur
```
diesel migration generate create_comments
```

# Örnek yorum kayıtları oluşturma
```
curl -s -X POST -H "Content-Type: application/json" -d "{\"title\":\"Frank says hello\",\"body\":\"Hello friends\"}" http://localhost:8998/users/1/posts
```
{
"id": 1,
"user_id": 1,
"title": "Frank says hello",
"body": "Hello friends",
"published": false
}

```
curl -s -X POST -H "Content-Type: application/json" -d "{\"title\":\"Bob is here too\",\"body\":\"Hello friends, also\"}" http://localhost:8998/users/2/posts
```
{
"id": 2,
"user_id": 2,
"title": "Bob is here too",
"body": "Hello friends, also",
"published": false
}

### Publish a post
```
curl -s -X POST -H "Content-Type: application/json"  http://localhost:8998/posts/1/publish
```
{
"id": 1,
"user_id": 1,
"title": "Frank says hello",
"body": "Hello friends",
"published": true
}

### Comment on a post
```
curl -s -X POST -H "Content-Type: application/json" -d "{\"user_id\":2,\"body\":\"Hi Frank, this is your friend Bob\"}" http://localhost:8998/posts/1/comments
```
{
"id": 1,
"user_id": 2,
"post_id": 1,
"body": "Hi Frank, this is your friend Bob"
}

### List all posts
```
curl -s -H "Content-Type: application/json" http://localhost:8998/posts
```
```
[
  [
    [
      {
        "id": 1,
        "user_id": 1,
        "title": "Frank says hello",
        "body": "Hello friends",
        "published": true
      },
      {
        "id": 1,
        "username": "Frank"
      }
    ],
    [
      [
        {
          "id": 1,
          "user_id": 2,
          "post_id": 1,
          "body": "Hi Frank, this is your friend Bob"
        },
        {
          "id": 2,
          "username": "Bob"
        }
      ]
    ]
  ]
]
```

### See posts
```
curl -s -H "Content-Type: application/json" http://localhost:8998/users/1/posts
```
```
[
  [
    {
      "id": 2,
      "user_id": 1,
      "title": "Frank says hello",
      "body": "Hello friends",
      "published": false
    },
    [
      
    ]
  ],
  [
    {
      "id": 1,
      "user_id": 1,
      "title": "Frank says hello",
      "body": "Hello friends",
      "published": true
    },
    [
      [
        {
          "id": 1,
          "user_id": 2,
          "post_id": 1,
          "body": "Hi Frank, this is your friend Bob"
        },
        {
          "id": 2,
          "username": "Bob"
        }
      ]
    ]
  ]
]
```

### Publish other post
```
curl -s -X POST -H "Content-Type: application/json" http://localhost:8998/posts/2/publish
```
```
{
  "id": 2,
  "user_id": 2,
  "title": "Bob is here too",
  "body": "Hello friends, also",
  "published": true
}
```

### List all posts again
```
curl -s -H "Content-Type: application/json" http://localhost:8998/posts
```
```
[
  [
    [
      {
        "id": 2,
        "user_id": 1,
        "title": "Frank says hello",
        "body": "Hello friends",
        "published": true
      },
      {
        "id": 1,
        "username": "Frank"
      }
    ],
    [
      
    ]
  ],
  [
    [
      {
        "id": 1,
        "user_id": 1,
        "title": "Frank says hello",
        "body": "Hello friends",
        "published": true
      },
      {
        "id": 1,
        "username": "Frank"
      }
    ],
    [
      [
        {
          "id": 1,
          "user_id": 2,
          "post_id": 1,
          "body": "Hi Frank, this is your friend Bob"
        },
        {
          "id": 2,
          "username": "Bob"
        }
      ]
    ]
  ]
]
```

### See users comments
```
curl -s -H "Content-Type: application/json" http://localhost:8998/users/2/comments
```
```
[
  [
    {
      "id": 1,
      "user_id": 2,
      "post_id": 1,
      "body": "Hi Frank, this is your friend Bob"
    },
    {
      "id": 1,
      "title": "Frank says hello",
      "published": true
    }
  ]
]
```

### See post comments
```
curl -s -H "Content-Type: application/json" http://localhost:8998/posts/1/comments
```
```
[
  [
    {
      "id": 1,
      "user_id": 2,
      "post_id": 1,
      "body": "Hi Frank, this is your friend Bob"
    },
    {
      "id": 2,
      "username": "Bob"
    }
  ]
]
```