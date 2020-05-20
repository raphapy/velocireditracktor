# velocireditracktor
A fast velociraptor, writen in Rust, for url tracking and redirect.

# Requeriments
* Docker
* Rust and Cargo 1.43.*

# Installation and config

1. Run postgres instance with docker compose. (If you change connection parameters in docker-compose.yml file, you need to reflect the same changes in .env file)
```bash
docker-compose up -d
```
Configurations for db connection
```bash
PG.HOST=127.0.0.1
PG.PORT=5432
PG.DBNAME=velocireditracktor_db
```
2. Execute db_ddl.sql script in the running postgres
3. Run the application with:
```bash
cargo run
```
4. Optionally, you can change host and port binding parameters in .env file.
```bash
SERVER.HOST=127.0.0.1
SERVER.PORT=8080
```
5. Database connecion pool max size can be changed in .env file
```bash
PG.POOL.MAX_SIZE=30
```
6. Data is stored in redirects table.
```bash
select * from redirects;
```

# WIP
1. Store a more complete information about the request in jsob format.
