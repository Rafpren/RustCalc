# CONTEXTE PROJET : RUSTCALC

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
* **Dernière version en production :** v1.2.12.
* **Dernière modification majeure :** Refonte de la logique de saisie (`nouveau_calcul`) et nettoyage de l'interface graphique (API eframe v0.34.1).
* **Statut du dépôt local :** Synchronisé et propre.

## 4. Objectif de la session actuelle
* Ajouter la gestion des fonctions trigonométriques, Changer le thème de couleurs, psser à la version 1.2.13 une fois fait]