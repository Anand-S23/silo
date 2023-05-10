# Silo
Small slideshow website, made with axum and react

## Quick Start

Silo uses docker for easy development, docker and docker-compose are pre-requistes. The following has to be done to get up and running:

1. Create a `.env` file using example.env as an example
2. Run `docker-compose build`
3. Run `docker-compose up`

Note: the docker-compose is for development, one for production will be made later

## Setting Up Database

Now that Silo is up and running there is going to be a couple more steps, that need to be taken in order get the database setup.

1. Run `docker ps` after the containers are up, in order to list all continers
2. Run `docker exec -it [ID] sh`, where `[ID]` is the id of the silo_server conainter
3. Run `cargo install sqlx-cli` to install the sqlx-cli tools
4. Run `sqlx migrate run` in order to create the tables in the database

Following these steps will get up up and running with Silo!
