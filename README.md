# aias-gm

Group Manager of AIAS project.

This entity is responsible for generate credential and trace user from signature.

**Note: This project use orignal anonymous authentication primitives, so do not use on production environment.**

## Installation

### 1. Clone Repository

Clone this repository.

```sh
git clone https://github.com/pj-aias/aias-gm
cd aias-gm
git checkout develop
```

### 2. Set enviroment variables

Write .env file to set enviroment variables.
It have to fill all variable to work.

```sh
cp .env.sample .env
vim .env # write
```

### 3. Crate database

Create sqlite3 DB with the touch command.

```sh
touch aias.db
```

### 4. Run servers

Run servers with docker-compose command.

```sh
docker-compose up 
# cargo run
```
