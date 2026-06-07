# rmux — État d'avancement (recap)

> Recap **humain**, dérivé de `git log` + README + `Cargo.toml` + `spec/`.
> L'**état machine** des agents reste les `.json` dans `~/.omega/state/` — ce fichier les rend lisibles, il ne les remplace pas. Régénérable.
> Dernière synchro : 2026-06-07

## En un coup d'œil
- **Phase** : préversion publique. README annonce **v0.3.1** (publié le 25 mai 2026), les 90 commandes compatibles tmux implémentées, bugs attendus.
- **Repo** : `github.com/agentik-os/rmux` @ `4455da0` sur `main` — arbre **propre** (0 fichier modifié), HEAD == `origin/main`.
- **Tags présents** : `v0.1.1` → `v0.3.1`, puis `v0.4.0`–`v0.4.3`, `v0.5.0`. HEAD = `v0.3.1-5-g4455da0` (5 commits de corrections au-dessus de v0.3.1 ; la chronologie tags vs HEAD est **à préciser**).
- **Upstream** : remote `upstream` = `github.com/Helvesec/rmux` (projet d'origine) ; `origin` = fork maison agentik-os.
- **État machine oracle** : **aucun** fichier `oracle-*rmux*.json` dans `~/.omega/state/` — pas de mission oracle enregistrée pour ce projet à ce jour.

## Workspace (crates)
| Crate | Rôle | Publication |
|---|---|---|
| `rmux-types` | Types valeur neutres | public |
| `rmux-proto` | DTOs IPC, framing, erreurs wire-safe | public |
| `rmux-os` | Helpers frontière OS | public |
| `rmux-ipc` | Endpoints / transports IPC locaux | public |
| `rmux-sdk` | SDK Rust adossé au daemon | public |
| `ratatui-rmux` | Widget d'intégration Ratatui | public |
| `rmux-pty` | Allocation PTY, resize, contrôle process enfant | support |
| `rmux-core` | Sessions, panes, layouts, formats, hooks, buffers | support |
| `rmux-server` | Daemon Tokio + dispatch des requêtes | support |
| `rmux-client` | Client IPC local + plomberie d'attach | support |
| `rmux` | CLI + entrypoint daemon caché | binaire public |
| `rmux-render-core` | Cœur de rendu de snapshots partagé | interne |

## Fait récemment (git)
- `4455da0` — fix(paste) : `PasteFilter` à état — une paste plus grande qu'un `read()` reste UN bloc bracketed unique
- `726d9e7` — fix(paste-detect) : ne plus traiter les bursts initiés par ESC comme des pastes (restaure le scroll souris)
- `91ef871` — fix(identity) : noms de session toujours ciblables + idempotents
- `98e856d` — test(paste) : flush du corps d'une bracketed paste surdimensionnée non terminée
- `0e4abb2` — fix(render+paste) : couleurs terminal-default + auto-wrap bracketed-paste
- `6301d12` / `02fcd26` — merge / prepare release v0.3.1
- antérieurs : env non-utf8 toléré, passthrough graphique SIXEL + tests, prompts attachés finis sur task runtime

## Prochaines étapes
- Aligner la doc publique (README, `Cargo.toml` repository) qui pointe encore vers `helvesec/rmux` alors que `origin` est `agentik-os/rmux` — **à préciser** si voulu.
- Clarifier la position de HEAD vis-à-vis des tags `v0.4.x`/`v0.5.0` (HEAD reste sur la lignée v0.3.1).
- Continuer la stabilisation préversion (bugs attendus, issues à remonter).
- Roadmap détaillée : **à préciser** (aucun fichier plan/roadmap dans le repo).

## Docs clés
- README : `README.md` (+ `README.fr.md`, `README.zh-CN.md`, `README.ja.md`)
- Doc complète : `rmux.io/docs` ; guides locaux : `docs/README.md`, `docs/human-friendly-config.md`
- Spec / contrats : `spec/feature-inventory-v1.yaml`, `spec/runtime.yaml`
- Manifeste workspace : `Cargo.toml` ; manpage : `rmux.1`
- Règles projet : `RULES.md`
