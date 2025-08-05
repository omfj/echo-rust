# echo::rust(🦀)

Nettsiden til interessegruppen for Rust-programmeringsspråket på echo &mdash; Linjeforeningen for informatikk ved UiB.

## Avhengigheter

- `cargo` (Rust-pakkehåndterer)
- `npm` (Node.js-pakkehåndterer)

## Utvikling

Vi anbefaler å installere `cargo-watch` for å automatisk bygge prosjektet når filer endres. Dette kan gjøres ved å kjøre `cargo install cargo-watch`.

For å starte utviklingsserveren, kjør `npm run dev`. Denne vil starte en lokal server på `localhost:8000` og automatisk bygge prosjektet når filer endres.

## Lage et nytt innlegg

Vi bruker filen `rss.toml` for å lagre innlegg på nettsiden. Denne filen blir lest når nettsiden blir bygget for å generere de nødvendige filene. For å lage et nytt innlegg, bruk kommandoen:

```bash
cargo run --bin generate
```

Dette vil legge til en post i `rss.toml`.
