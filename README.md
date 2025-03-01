# vite-tailwind-actix-rust-postgres-starter

I was looking for a guide on implementing Oauth2 in Rust and found this: `https://codevoweb.com/how-to-implement-google-oauth2-in-rust/`, but wasn't able to find anything that described how to implement it using a backing database. So I decided to do it myself as an exercise!

You will need to update the values in `env.example` then rename to `.env`

This is a Vite/React + Actix Web/Rust + PostgresQL stack

## Run the MW:
`cargo watch -q -c -w src/ -x run`

This past week, I heard a startup idea from a close connection and had an upcoming interview with a company that I knew leveraged a rust-coded REST API, and saw here an opportunity for some learning. In an effort to brush up on my rust and set a foundation for the app, I forked a guide/repository (https://codevoweb.com/how-to-implement-google-oauth2-in-rust), and got to work ripping out the Arc-based db it leveraged in favor of Postgres, such that you could use it as a base for a relational web-app.


This starter stack implements a Vite front-end, Actix/Rust middleware, and a PostgreSQL backend. It includes a docker-compose.yml to easily build the stack on your own hardware, and a README to guide one through the steps necessary to edit the .env.example 