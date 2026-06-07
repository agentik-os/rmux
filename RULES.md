# rmux — Règles projet (canon · à garder & vérifier)

> Fiche de référence **load-bearing** : un agent ou un humain qui reprend le projet lit ça en premier.
> **Ne pas supprimer.** Mettre à jour quand une règle change.

## Identité
- **Projet** : rmux — multiplexeur de terminal universel en Rust pour « l'ère agentique » : sessions détachables, scriptables, inspectables, avec CLI compatible tmux, SDK adossé à un daemon, et intégration native [Ratatui](https://ratatui.rs).
- **Owner** : à préciser (commits signés `Hacker` / `Helvesec` ; pas de nom explicite dans le repo).
- **Repo** : `github.com/agentik-os/rmux` (remote `origin`) — fork maison.
- **Upstream** : `github.com/Helvesec/rmux` (remote `upstream`) — projet d'origine. Le README et `Cargo.toml` pointent encore vers `helvesec/rmux`.
- **Branche** : `main`.
- **Trois surfaces publiques** : CLI `rmux`, crate `rmux-sdk`, widget `ratatui-rmux` — partagent un protocole local unique vers le daemon.

## Stack (Rust)
- **Langage** : Rust (edition 2021), workspace Cargo multi-crates (`resolver = "2"`).
- **Runtime** : daemon Tokio (`rmux-server`), IPC local (Unix socket sur Linux/macOS, Named Pipe sur Windows).
- **CLI** : `clap` (dérive) — binaire `rmux` (`src/main.rs`), 90 commandes compatibles tmux annoncées.
- **Profil release** : `codegen-units = 1`, `lto = "fat"`, `strip = "symbols"`.
- **Plateformes** : Linux (Unix PTY), macOS (Unix PTY), Windows (ConPTY).

## Intouchable (source de vérité)
- `spec/` (`feature-inventory-v1.yaml`, `runtime.yaml`) = contrat de fonctionnalités / runtime. Ne pas éditer à la légère.
- `Cargo.lock` versionné = builds verrouillés (`--locked`). Ne pas régénérer sans raison.
- `LICENSE-MIT` + `LICENSE-APACHE` = double licence MIT OR Apache-2.0.
- Crates `rmux-*` du workspace (voir `Cargo.toml` `[workspace].members`) : frontières OS/PTY/IPC isolées dans les crates bas-niveau.

## Règles spécifiques (build cargo)
- **Vérification source** (depuis le README, dépendances verrouillées) :
  ```sh
  cargo fmt --all -- --check
  cargo clippy --workspace --all-targets --locked -- -D warnings
  cargo test --workspace --locked --no-fail-fast
  ```
- **Checks locaux additionnels** : `scripts/cfg-check.sh`, `scripts/unsafe-check.sh`, `scripts/no-network-in-runtime.sh`, `scripts/check-platform-neutrality.sh`, `scripts/ratatui-rmux-budget.sh`, `scripts/verify-package.sh`.
- **Politique unsafe** : `#![forbid(unsafe_code)]` dans les crates haut-niveau ; le code « boundary » OS/terminal est confiné aux crates bas-niveau.
- **Pas de réseau au runtime** (garde-fou `no-network-in-runtime.sh`).
- **Release / packaging** : `scripts/release-local.sh`, `scripts/package-unix.sh`.
- Côté serveur Agentik : git **toujours via `sudo -u vibe`** ; pas de commit/push non demandé.

## Objectif
- Multiplexeur tmux-compatible mais **pas un clone byte-for-byte** : faire tourner des agents longue-durée détachés (ex. via SSH) tout en gardant terminaux inspectables, scriptables et orchestrables, utilisable par agents, workflows CLI headless et humains.
- Cible produit : préversion publique stabilisée (cf. README v0.3.1) — feuille de route détaillée **à préciser** (aucun roadmap dédié trouvé dans le repo).
