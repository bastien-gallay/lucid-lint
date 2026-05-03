<!-- en-source-sha: 2dd19908bb5ad2d1081f4acb00a06e9acf125a66 -->
# Installation

`lucid-lint` propose quatre voies d'installation. Choisissez celle qui correspond à votre environnement.

## Installeur en une ligne (Linux, macOS, WSL)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/bastien-gallay/lucid-lint/releases/latest/download/lucid-lint-installer.sh | sh
```

Le script est généré par [`cargo-dist`](https://github.com/axodotdev/cargo-dist) à chaque version publiée. Il détecte votre plate-forme. Il télécharge le binaire pré-compilé correspondant depuis la version GitHub. Il le place sur `$PATH` (par défaut : `$CARGO_HOME/bin` si défini, sinon `~/.cargo/bin`).

### Auditer avant d'exécuter

`curl … | sh` est rapide mais opaque. Pour lire le script avant de l'exécuter :

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/bastien-gallay/lucid-lint/releases/latest/download/lucid-lint-installer.sh -o install.sh
less install.sh
sh install.sh
```

Le script est court — moins de 200 lignes de shell POSIX. Une lecture rapide reste réaliste. Il fixe la version pour laquelle il a été généré. Il vérifie la taille attendue de l'archive téléchargée. Il sort en erreur si une valeur diffère.

### Fixer une version précise

`latest` pointe vers la version la plus récente. Pour fixer une version connue et stable :

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/bastien-gallay/lucid-lint/releases/download/v0.2.2/lucid-lint-installer.sh | sh
```

## Installeur en une ligne (Windows PowerShell)

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/bastien-gallay/lucid-lint/releases/latest/download/lucid-lint-installer.ps1 | iex"
```

Même mécanique `cargo-dist`, version PowerShell. Le binaire atterrit dans `%CARGO_HOME%\bin` si `CARGO_HOME` est défini, sinon dans `%USERPROFILE%\.cargo\bin`.

Pour auditer avant d'exécuter, sauvegardez le script et inspectez-le :

```powershell
irm https://github.com/bastien-gallay/lucid-lint/releases/latest/download/lucid-lint-installer.ps1 -OutFile install.ps1
notepad install.ps1
.\install.ps1
```

## Via Cargo

```bash
cargo install lucid-lint
```

Cette voie compile depuis les sources publiées sur [crates.io](https://crates.io/crates/lucid-lint). Elle place le binaire dans votre dossier `bin` de Cargo (par défaut `~/.cargo/bin/`). Plus lent que l'installeur pré-compilé. Utile quand les cibles pré-compilées ne couvrent pas votre plate-forme.

## Depuis les sources

```bash
git clone https://github.com/bastien-gallay/lucid-lint
cd lucid-lint
cargo install --path .
```

## Binaires pré-compilés

Chaque version publie des binaires pré-compilés pour :

- Linux (`x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`)
- macOS (`aarch64-apple-darwin`, `x86_64-apple-darwin`)
- Windows (`x86_64-pc-windows-msvc`)

Les installeurs shell et PowerShell ci-dessus choisissent l'archive correcte automatiquement. Pour installer à la main, téléchargez depuis la [page des versions GitHub](https://github.com/bastien-gallay/lucid-lint/releases) et placez le binaire extrait sur `$PATH`.

## Vérifier l'installation

```bash
lucid-lint --version
```

## Pré-requis système

- Rust 1.75 ou plus récent (utile uniquement pour la compilation depuis les sources ou via `cargo install`).
- Aucune dépendance d'exécution.
