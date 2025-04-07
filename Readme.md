# Lol Eighty

League of legends(lol) profile website.

## Features

- Profile history
- Profile stats

## Necessary secrets

- .env.secret file
- RIOT_API_KEY

## Dependencies

- Web server: Actix web (<https://actix.rs/docs>)
- HTML templating engine: Tera (<https://docs.rs/tera/latest/tera/>)
- HTML interactivity: Htmx (<https://htmx.org/>)
- OpenSSL

## Development

- Hotreloading: cargo-watch (<https://crates.io/crates/watchexec-cli>)

To run server while watching: `watchexec -w src -r cargo run`

## Todo

- Rank display
- Rank graph?
- Rank history?
- Match history
- DATABASE!
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
