# Coffee Notebook
A lightweight, full-stack app allowing me to create, update and delete espresso recipes.
I use this app locally to store bean brands and their dialed in recipes.

## Frontend
The frontend uses [Svelte](https://svelte.dev/) and [AstroJS](https://astro.build/) to make building static sites a breeze.

## Backend
The backend serves the static files using [Axum](https://github.com/tokio-rs/axum) and stores the recipes in a simple JSON file (a database would be overkill).

## Usage
1. Clone the repository.
2. Enter the `frontend/` folder, run `npm i` to install modules and `npm run build` to build the static files.
3. Enter the `coffee-notebook/` directory and run `docker build -t coffee_notebook .` to build the docker image.
4. Run `docker images`, and confirm you see a `coffee_notebook` image.
5. Run `docker run -p 3000:PORT --name coffee_notebook_instance coffee_notebook`, changing `PORT` with your desired out-facing port.
6. Access the webpage to confirm it's running!
