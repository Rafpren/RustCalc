# 🧮 RustCalc (GUI)

Une calculatrice performante et élégante développée en **Rust** avec l'interface graphique **egui**. Ce projet gère les priorités opératoires et les parenthèses.

## ✨ Fonctionnalités
- **Calcul complet** : Supporte les additions, soustractions, multiplications et divisions.
- **Priorités mathématiques** : Gère correctement l'ordre des opérations et les parenthèses.
- **Interface Graphique (GUI)** : Une fenêtre interactive avec thème sombre et boutons colorés.
- **Portable** : Un seul fichier par OS `.exe`, `.rpm` et `.deb` sans installation nécessaire pour Windows mais bien pour Debian et Fedora

## 🛠️ Installation & Compilation

Vous pouvez télécharger la dernière version dans la section [Releases](https://github.com/Rafpren/RustCalc/releases).

### 🪟 Windows
* Téléchargez `rustcalc.exe`.
* Lancez simplement l'exécutable.

### 🐧 Linux (Fedora, RHEL, openSUSE)
* Téléchargez le fichier `.rpm`.
* Installez-le avec :
  ```bash
  sudo dnf install ./rustcalc-*.x86_64.rpm

### 🐧 Linux (Debian, Ubuntu, Mint)
* Téléchargez le fichier .deb.
* Installez-le avec :
  ```bash
  sudo apt install ./rustcalc_*_amd64.deb

Si vous souhaitez compiler le projet vous-même :

1. Assurez-vous d'avoir [Rust](https://www.rust-lang.org/) installé.
2. Clonez le dépôt : 
```bash
git clone https://github.com/Rafpren/RustCalc.git
```

3. Installez les dépendances (Linux uniquement) : `libxcb-devel libxkbcommon-devel openssl-devel`
4. Compilez : `cargo build --release`
