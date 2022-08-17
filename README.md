# Note Taking App

I wanted to learn more about fullstack development and backend / frontedns in egneral, so I thought I would start with a classic - the note taking app!

Dont judge my shitty code..

Requirements:

- Diesel CLI
- Npm
- Rust

Running:

1) Make a .env file similar to below (or set environment variables)
2) `cd frontend && npm install`
3) `npx vite serve`
4) `cd ..`
5) `disel setup`
6) `diesel migration run`
7) `cargo run`

Example /.env file:

```.env
DATABASE_URL="your_database_file.sqlite"
```

Todo:

- [X] Basic Actix setup
- [X] Database intergration
- [X] Creating notes
- [X] Retrieving notes
- [X] Updating notes
- [X] Deleting notes
- [ ] Frontend intergration
- [ ] Tags / filtering? (PostgreSQL)
