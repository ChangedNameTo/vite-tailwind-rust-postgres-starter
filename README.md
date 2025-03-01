# vite-tailwind-actix-rust-postgres-starter

I was looking for a guide on implementing Oauth2 in Rust and found this: `https://codevoweb.com/how-to-implement-google-oauth2-in-rust/`, but wasn't able to find anything that described how to implement it using a backing database. So I decided to do it myself as an exercise!

You will need to update the values in `env.example` then rename to `.env`

Also make sure that the redirect URI is set in the google cloud console like so: ![image](https://github.com/user-attachments/assets/476c9dd7-c0d7-4c52-9101-aafcf938ed6e)

This is a Vite/React + Actix Web/Rust + PostgresQL stack. Build with `docker-compose up --build` and a valid `.env`. If all of the config values are set properly and the image builds, you should be able to use an existing Google account to log in like so: ![image](https://github.com/user-attachments/assets/0e62d5b5-dd38-4897-b44c-c2bca129bb88)
