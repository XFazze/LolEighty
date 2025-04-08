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

## Development

- Hotreloading: cargo-watch (<https://crates.io/crates/watchexec-cli>)

To run server while watching: `watchexec -w src -r cargo run`

## Todo

- DATABASE!
- Docker!
- Tests!
- Css & Styling

- Past 30 performance
- Champion performance
- Teamate performance
- Pings

- Detailed user stat page
- Tags
- Account & match notes
- AI score?
- Wallpaper
- Mastery

- Live game & important cds
- Fun data page & competition?
- Leaderboard
- Stats table
