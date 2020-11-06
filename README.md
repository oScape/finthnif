# Finthnif 

Go to `backend/docker` and run `docker-compose up -d` to setup and launch a postgresql db and pgadmin.

You can see more details [here](https://linuxhint.com/postgresql_docker/) to setup with pgadmin.

Connect to the database and create drivers table, run `psql -h 127.0.0.1 -p 5432 -d postgres -U admin -f backend/sql/drivers.sql`,

You can look at the directory `backend/sql/` for create the mandatory schema for the app.