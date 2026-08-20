# rmux, brief de l'agent

Multiplexeur de terminal local en Rust : sessions détachables, CLI compatible tmux,
daemon, SDK Rust, widget Ratatui. Version du workspace : `0.3.1` (`Cargo.toml`).

`RULES.md` est la fiche canon du projet et reste la référence longue. Ce fichier
est le résumé opérationnel, vérifié contre le dépôt.

## Coordonnées git (vérifiées)

- `origin` : `https://github.com/agentik-os/rmux.git` (fork maison)
- `upstream` : `https://github.com/Helvesec/rmux.git` (projet d'origine)
- Branche : `main`
- `Cargo.toml` et le README pointent encore vers `helvesec/rmux` : c'est voulu, ne
  "corrige" pas sans demander.

## Stack réelle

- Rust, edition 2021, workspace Cargo (`resolver = "2"`), 11 crates `crates/*` plus
  le paquet racine `rmux` et `xtask`.
- Binaire `rmux` : `src/main.rs`, CLI en `clap` 4.5.39 (dérive).
- Daemon : `rmux-server` sur Tokio. IPC local dans `rmux-ipc` : `UnixListener` sur
  Unix, Named Pipe Windows (`crates/rmux-ipc/src/listener.rs`).
- PTY : `rmux-pty` (Unix PTY, ConPTY côté Windows).
- Profil release : `codegen-units = 1`, `lto = "fat"`, `strip = "symbols"`.
- Double licence MIT OR Apache-2.0.

## Structure

- `src/` : le binaire et le parsing CLI (`cli.rs`, `cli_args*`).
- `crates/` : `rmux-types`, `rmux-proto`, `rmux-core`, `rmux-os`, `rmux-ipc`,
  `rmux-pty`, `rmux-client`, `rmux-server`, `rmux-sdk`, `rmux-render-core`,
  `ratatui-rmux`.
- `spec/` : `feature-inventory-v1.yaml` et `runtime.yaml`, le contrat de
  fonctionnalités et de runtime. Ne pas éditer à la légère.
- `tests/` : tests d'intégration au niveau du binaire (surface CLI, compat tmux,
  daemon interne, stress).
- `scripts/` : les gardes locales. `xtask/` : les tâches cargo maison.
- `docs/`, `README.md` (plus `README.fr.md`, `README.ja.md`, `README.zh-CN.md`),
  `rmux.1` (page de man).

## Commandes réelles

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked --no-fail-fast
```

Ces trois-là sont celles du README (lignes 304 à 306) et de `.github/workflows/ci.yml`.
La CI lance en plus `cargo doc --workspace --locked --no-deps`, `cargo deny check`,
et les scripts `scripts/no-network-in-runtime.sh`, `scripts/check-platform-neutrality.sh`,
`scripts/smoke-sdk-v1.sh`, `scripts/perf-bench.sh`.

## Conventions visibles dans le code

- `#![forbid(unsafe_code)]` dans les crates haut niveau (`rmux-core`, `rmux-types`,
  `rmux-proto`, `rmux-sdk`, `rmux-server`, `rmux-render-core`, `ratatui-rmux`) : le
  code proche de l'OS reste confiné aux crates bas niveau.
- Pas de réseau au runtime, garde-fou `scripts/no-network-in-runtime.sh`.
- `Cargo.lock` versionné : builds `--locked`, ne pas régénérer sans raison.
- Côté serveur Agentik : git toujours via `sudo -u vibe`, pas de commit ni de push
  non demandé.

## Notes

- `PROGRESS.md` à la racine ne contient pas de progression : c'est un message
  d'erreur de quota collé par accident. Ne t'en sers pas comme source.
