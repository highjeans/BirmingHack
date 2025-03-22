# Auth /auth

## POST /register

Request:
```
{
    "email": String,
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
    "email": String,
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
    "isbn": String,
    "blurb": String
}
```

## GET /:id

Response 200:
```
{
    "isbn": String,
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
    "isbn": String,
    "blurb": String,
}
```
