# scolengo-token-tauri
[scolengo-token](https://github.com/maelgangloff/scolengo-token/) en plus léger et plus rapide.

Pour utiliser le wrapper [scolengo-api](https://github.com/maelgangloff/scolengo-api), il est nécessaire de posséder des jetons d'authentification OpenID Connect. Cette application permet de les obtenir auprès du CAS.

Des exécutables sont disponibles dans la dernière [Release](https://github.com/oriionn/scolengo-token-tauri/releases).

🚨 ATTENTION: Ne communiquez jamais vos jetons à un tiers. Ils vous sont strictement personnels. Si vous pensez que vos jetons ont été dérobés, révoquez-les immédiatement.

<img src="docs/preview.png">

## Remarque importante
- Il est clairement mentionné que ce logiciel n'est pas officielle.
- Ce logiciel n'est pas une contrefaçon car il n'existe pas de module similaire édité officiellement.
- Les utilisateurs ne peuvent accéder qu'à leurs propres données. Ils sont soumis au même processus d'authentification que celui implémenté dans l'application.
- Les données des utilisateurs ne sont pas davantage exposées puisqu'un utilisateur ne peut accéder qu'à ses propres données. Personne n'a le contrôle sur cette limitation qui est inhérente au fonctionnement de l'API des serveurs de Skolengo.
- L'utilisateur final est le seul responsable de son code et des éventuelles conséquences.
- Tout utilisateur de ce logiciel a *a priori* lu l'entièreté du fichier de licence GPLv3 disponible publiquement [LICENSE](https://github.com/oriionn/scolengo-token-tauri/blob/main/LICENSE) ainsi que de ce présent fichier de présentation.
- Tout utilisateur de ce logiciel a *a priori* lu l'entièreté du code de ce projet avant toute utilisation.
- Eu égard l'ensemble de ces remarques, les contributeurs et *a fortiori* l'auteur du projet ne peuvent être tenus responsables de tout dommage potentiel.

## Lancer le projet depuis les sources
Préréquis: Node.js, Rust

1. Cloner le dépôt
```shell
git clone https://github.com/oriionn/scolengo-token-tauri
```

2. Installer les dépendances
```shell
npm install
```

3. Lancer l'application
```shell
npm run tauri dev
```

4. Créer un éxecutable (Optionel)
```shell
npm run tauri build
```

---
Crédit Front End: [scolengo-token](https://github.com/maelgangloff/scolengo-token/tree/master)
Crédit Tauri Action: [tauri-action (Fork)](https://github.com/Avocadocs/tauri-action)