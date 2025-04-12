# Lol Eighty

League of legends(lol) profile website.

## Features/Goal

- Profile history
- Profile stats

- General statictics
- Trivia
- Drafting

## Necessary secrets

- .env.secret file
- RIOT_API_KEY
- /static/htmx.min.js (<https://unpkg.com/htmx.org@2.0.4/dist/htmx.min.js>)
- /staic/ddragon (<https://ddragon.leagueoflegends.com/cdn/dragontail-12.6.1.tgz>)

## Dependencies

- Web server: Actix web (<https://actix.rs/docs>)
- HTML templating engine: Tera (<https://docs.rs/tera/latest/tera/>)
- HTML interactivity: Htmx (<https://htmx.org/>)
- Sqlx (<https://docs.rs/sqlx/latest/sqlx/>)
- Tailwindcss  (<https://tailwindcss.com/blog/standalone-cli>)

## Development

- Hotreloading: watchexec (<https://crates.io/crates/watchexec-cli>)
- Docker (<https://docs.docker.com/compose/>)

To run server raw watching: `watchexec -w src -r cargo run`.

### With docker

Postgres container is the only service on by default.
To start watching development server: `docker compose --profile dev up`.
To interact with postgres server you need to use the docker network. This can be done with: `docker compose --profile dev up postgres-sqlx` and then attaching to the container: `docker attach [CONTAINER ID]`.

## Deployment

To run development server: `docker compose --profile prod up`.

## Todo

- setup db pool
- user model
- user rank model
- user mastery model

- match model
- user_match model

- Tests!
- Css & Styling

- Past 30 performance
- Champion performance
- Teamate performance
- Pings
- Mastery

- Detailed user stat page
- Tags
- Account & match notes
- AI score?
- Wallpaper

- Live game & important cds
- Fun data page & competition?
- Leaderboard
- Stats table
- Aram stats rating
