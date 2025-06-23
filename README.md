# Générateur de mots de passe en Rust

Ce projet est un générateur de mots de passe sécurisé écrit en Rust. Il permet à l'utilisateur de choisir les types de caractères à inclure (minuscules, majuscules, chiffres, caractères spéciaux) et la longueur du mot de passe. L'entropie du mot de passe généré est également affichée.

## Fonctionnalités

- Sélection interactive des types de caractères via le terminal
- Choix de la longueur du mot de passe
- Génération aléatoire et sécurisée du mot de passe
- Affichage de l'entropie du mot de passe

## Dépendances

- [inquire](https://crates.io/crates/inquire) : pour les prompts interactifs
- [rand](https://crates.io/crates/rand) : pour la génération aléatoire

## Installation

Clone le dépôt puis compile le projet avec Cargo :

```sh
git clone <url-du-repo>
cd rust_password_gen
cargo build --release
```

## Utilisation
Lance le programme avec la commande suivante :
```sh
cargo run --release
```