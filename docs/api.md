# Auth /auth

## POST /register

Request:
```
{
    "username": String,
    "password": String,
    "name": String,
}
```

Response 200:
```
{
    "token": String
}```


## POST /login

Request:
```
{
    "username": String,
    "password": String,
}
```

Response 200:
```
{
    "token": String
}
```


# Profile /profile

## GET / and /:id

Response 200:
```
{
    "name": String,
    "socials": {
        "platform": String,
        "username": String
    }[],
}
```

## PUT /

Request:
```
{
    "name": String,
    "socials": {
        "platform": String,
        "username": String
    }[],
}
```

Response 201

# BookListing /listing

## POST

Request:
```
{
    "isbn": Int,
    "blurb": String
}
```

## GET /:id

Response 200:
```
{
    "isbn": Int,
    "title": String,
    "author": String,
    "blurb": String,
}
```

## DELETE /:id

Response 201

## POST /extract

Request:
Image

Response 200:

```
{
    "isbn": Int,
    "blurb": String,
}
```
