# Coffee Notebook
A lightweight, full-stack app allowing me to create, update and delete espresso recipes.
I use this app locally to store bean brands and their dialed in recipes.

This is a sample image of the webpage: <br><br>
![Sample image of site](https://i.imgur.com/UiYJQFn.png)

## Frontend
The frontend uses [Svelte](https://svelte.dev/) and [AstroJS](https://astro.build/) to make building static sites a breeze.

## Backend
The backend serves the static files using [Axum](https://github.com/tokio-rs/axum) and stores the recipes in a simple JSON file (a database would be overkill).

## Usage
1. Clone the repository.
2. Enter the `coffee-notebook/` folder, and run `./prepare_docker.sh` to build the static Astro files.
3. In the same directory, run `docker build -t coffee_notebook .` to build the docker image.
4. Run `docker images`, and confirm you see a `coffee_notebook` image.
5. Run `docker run -d -p PORT:3000 --name coffee_notebook_instance coffee_notebook`, changing `PORT` with your desired out-facing port.
6. Access the webpage to confirm it's running!
<br><br>
##### Note: At the moment, no backups of the `coffee.json` file are produced.
If you would like to access the `coffee.json` file to save your recipes elsewhere, run `(sudo) docker exec -it INSTANCE_NAME /bin/bash` and then `cat coffee.json`. This will output the JSON file contents. If you spin up a new instance and already have the JSON file, you can just `echo JSON_CONTENTS > coffee.json` to load the recipes into the backend.
