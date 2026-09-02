# TimePulse

Utilitaire Windows local de suivi du temps, construit avec Tauri 2, React, TypeScript, Vite et SQLite.

## Développement

```bash
pnpm install
pnpm tauri dev
```

Les données sont stockées dans `timepulse.sqlite` dans le répertoire de données local de l’application. Aucun compte ni serveur n’est requis.

## Build Windows

```bash
pnpm tauri build
```

Les installateurs sont générés dans `src-tauri/target/release/bundle/` aux formats NSIS (`.exe`) et MSI (`.msi`).

Le bouton de fermeture masque la fenêtre dans le System Tray. Le menu du tray permet de rouvrir ou quitter TimePulse.
