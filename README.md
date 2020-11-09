# Finthnif 

## Launch database with docker

Go to `backend/docker` and run `docker-compose up -d` to setup and launch a postgresql db and pgadmin.

You can see more details [here](https://linuxhint.com/postgresql_docker/) to setup with pgadmin.

Connect to the database and create drivers table, run `psql -h 127.0.0.1 -p 5432 -d postgres -U admin -f backend/sql/drivers.sql`,

You can look at the directory `backend/sql/` for create the mandatory schema for the app.

## Launch the server

Go to `backend`, run `cargo run` and connect to `http://localhost:8000/`

## Launch the electron app

Go to `app-electron`, run `npm start`

You need to have install:
- `node`, with wsl2 follow steps [here](https://docs.microsoft.com/fr-fr/windows/nodejs/setup-on-wsl2)
- `electron`, with wsl2 you have to install [VcXsrv Windows X Server](https://sourceforge.net/projects/vcxsrv/), for install help [here](https://techcommunity.microsoft.com/t5/windows-dev-appconsult/running-wsl-gui-apps-on-windows-10/ba-p/1493242) and config bashrc [here](https://wiki.ubuntu.com/WSL#Running_Graphical_Applications)
- `wasm-pack` follow steps [here](https://github.com/rustwasm/wasm-pack)

You need to create:
- `app-electron/dist` and create a file `index.html` which contains a basic skeleton:

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Webpack App</title>
  <meta name="viewport" content="width=device-width, initial-scale=1"></head>
  <body>
  <script src="bundle.js"></script></body>
</html>
```