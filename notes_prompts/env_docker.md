# Set-up MySQL Server & Connect it with SQLx

Let set-up MySQL, you can use your own MySQL or using Docker instead.

For Docker let create new environment file .env and docker compose file `docker-compose.yml`

```conf
# For docker compose mysql
MYSQL_DATABASE=rust_axum_sqlx
MYSQL_USER=user
MYSQL_PASSWORD=password123
MYSQL_ROOT_PASSWORD=password123

# Rust support placeholder
# For sqlx cli and rust sqlx
DATABASE_URL=mysql://${MYSQL_USER}:${MYSQL_PASSWORD}@127.0.0.1:3306/${MYSQL_DATABASE}
```

```yml
version: '3'
services:
  mysql:
    image: mysql:latest
    container_name: mysql
    env_file:
      - ./.env
    ports:
      - '3306:3306'
    volumes:
      - mysqlDb:/var/lib/mysql
volumes:
  mysqlDb:
```

Then run it to start MySQL on root project or in folder where `docker-compose.yml` file located.

```shell
# Run Docker Compose & Detach
docker compose up -d

# (Bonus! for stopping MySQL Docker)
docker compose stop

```