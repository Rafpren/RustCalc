# CONTEXTE PROJET : RUSTCALC 30-03-2026

## 1. Stack Technique
* **Langage :** Rust (Édition 2024).
* **GUI :** `egui` / `eframe` (Version 0.34.1 stricte).
* **Environnements de dev :** * Principal : Windows 11 (RustRover).
    * Secondaire/Cible native : Fedora Workstation 43 (Wayland/Gnome).

## 2. Infrastructure & Sécurité (GitHub)
* **Branche `main` :** Verrouillée par un Ruleset. Modifications directes interdites. Passage obligatoire par une branche annexe + Pull Request.
* **Intégration Continue (CI) :** Workflow `ci.yml` actif sur `push` et `pull_request` vers `main`.
    * Matrice : `ubuntu-latest` et `windows-latest`.
    * Étapes : `cargo fmt --check` (Linux), `cargo clippy -D warnings`, et `cargo test` (avec `+crt-static` sur Windows).
* **Déploiement Continu (CD) :** Workflow `release.yml` actif sur les tags `v*`.
    * Génération et publication automatiques sur GitHub Releases : `rustcalc.exe` (Windows), `rustcalc-*.rpm` (Fedora), `rustcalc_*.deb` (Debian/Ubuntu).

## 3. État Actuel
* **Dernière version en production :** v1.2.14.
* **Workflow :** Règles de branche respectées sans contourner le Ruleset
* **Statut du dépôt local :** Nettoyé de toute dette Git.

## 4. Objectif de la session actuelle
* ...