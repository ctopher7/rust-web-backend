# rust-web-backend
web backend written in Rust language, for learning purposes

## tech stack:
- PostgreSQL for database (https://hub.docker.com/_/postgres)
- Actix-web for routing (https://github.com/actix/actix-web)
- sqlx for sql object mapper (https://github.com/launchbadge/sqlx)
- jsonwebtoken for user authentication
- bcrypt for password hash

## to run in development mode:
- modify ENV files, .env is needed for sqlx
- create migrate database using pg admin
- execute:
```
cargo run -- -e dev
```

## to deploy:
- modify ENV files, .env is needed for sqlx
- execute:
```
docker build -t rust-app -f .Dockerfile .
docker-compose up -d
```

## pre-built API:
- customer sign up:
```
curl --location --request POST 'localhost:8080/global/user/signup' \
--header 'x-api-key: your_api_key' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "example@example.com" ,
    "password":"AlphaNum3ricW!thSpecialChar" ,
    "phone_number": "+628123456789" ,
    "name": "valid name"
}'
```

- user login (superadmin,admin,customer)
```
curl --location --request POST 'localhost:8080/global/user/login/web' \
--header 'x-api-key: your_api_key' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email":"example@example.com",
    "password":"AlphaNum3ricW!thSpecialChar"
}'
```

- get all user (superadmin,admin)
```
curl --location --request GET 'localhost:8080/admin/user/all' \
--header 'x-api-key: your_api_key' \
--header 'Cookie: Authorization=jwt token acquired after login'
```

-create new user (superadmin)
```
curl --location --request POST 'localhost:8080/superadmin/user/create' \
--header 'x-api-key: your_api_key' \
--header 'Content-Type: application/json' \
--header 'Cookie: Authorization=jwt token acquired after login' \
--data-raw '{
    "user_role": "customer" ,
    "email": "example@example.com" ,
    "password":"AlphaNum3ricW!thSpecialChar" ,
    "phone_number": "+628123456789" ,
    "name": "valid name"
    "date_of_birth": "1970-01-01"
}'
```