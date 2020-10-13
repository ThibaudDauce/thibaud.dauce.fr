---
title: "Projet Windows"
description: "Un développeur Linux depuis plus de 10 ans peut-il passer à Windows ?"
---

J'ai reçu hier un Dell XPS 17 2020. Après avoir consulté quelques forums, je me suis rendu compte que [les pilotes Linux ne semblent pas tout à fait à jour](https://wiki.archlinux.org/index.php/Dell_XPS_17) pour cette machine et je risque fort d'avoir, au minimum, des problèmes de son. C'est l'occasion rêvée de faire une petite expérimentation : que donne Windows aujourd'hui pour développer ?

Je n'ai personnellement jamais utilisé Windows pour développer, j'ai commencé avec un CD d'Ubuntu (envoyé gratuitement par la poste à l'époque !) il y une dizaine d'années, puis, il y a maintenant plus de 6 ans, je suis passé à ArchLinux pour bénéficier des dernières versions logicielles.

## Objectifs lors de la réception

Mon objectif est donc de tester Windows pendant quelques semaines/mois le temps que le noyau Linux se mette à jour et ensuite d'évaluer l'expérience. Je compte y aller à fond : Windows, mais aussi les applications Mail et Calendar intégrées, Edge en tant que navigateur principal, le nouveau Windows Terminal…

Pour référence, voici les principaux points qui peuvent potentiellement m'embêter par ordre de priorité :

1. Je ne sais pas du tout comment fonctionne les outils de développement du nouveau Edge (est-ce qu'il est compatible avec les extensions Chrome ? Est-ce que toutes les infos sont présentes dans l'explorateur ?)
1. Les bureaux multiples semblent simplistes sous Windows : j'utilise actuellement 4 bureaux virtuels en carré avec des raccourcis pour se déplacer dans l'espace (vers le haut, gauche, droite, bas…) et des catégories de programmes automatiquement ouvertes sur chaque bureau.
1. Des problèmes insolvables : sous Linux j'ai toujours la possibilité d'aller assez loin dans le fonctionnement du système en ligne de commande pour trouver une solution.
1. Un environnement de bureau plus lourd : difficile de faire moins lourd que mon [bspwm](https://wiki.archlinux.org/index.php/Bspwm) actuel.

Mais je pense aussi que Windows va m'apporter quelques avantages :

1. Une meilleure autonomie : en particulier via une meilleure gestion de la puce GPU intégrée d'Intel et de la carte graphique Nvidia dédiée.
1. Une meilleure gestion des périphériques USB (télépone Android, liseuse, imprimante…).
1. Moins de temps passé à configurer des choses, réparer des problèmes, installer des mises à jour et plus de temps à développer.
1. La possibilité d'utiliser Windows Hello et le capteur d'empreintes pour déverrouiller l'ordinateur.
1. La possibilité d'installer des jeux vidéos
1. Des meilleurs débuggeurs si je me lance vraiment dans le développement Rust/C++

## Première mise à jour suite à une journée de développement

Après avoir installé mon ordinateur et développé une journée avec, voici mes premiers retours.

Au niveau des problèmes rencontrés :
- Certains texte de Windows lors des redémarrages pour les mises à jour sont en anglais, incompréhensible… La date et l'heure étaient également en format anglais alors que ma configuration était en français.
- La configuration de mon clavier Bépo n'était pas la même que sous Linux, mais un article arrive très vite pour expliquer tout ça !
- Redis n'existe pas en version Windows (du moins pas depuis la version 3)
- Je n'ai pas trouvé d'alternative à `rsync` pour le moment
- L'application Mail n'arrive pas à se connecter à ProtonMail Bridge et ne propose visiblement pas d'identités multiples pour pouvoir envoyer des mails avec différentes adresses via le même compte SMTP. J'ai installé Thunderbird.
- Je n'ai pas encore trouvé d'alternative à `inotifywait` pour compiler mon code dès qu'il y a une modification (je peux utiliser `cargo watch` dans l'écosystème Rust, cela doit exister dans l'écosystème d'Elm mais je n'ai pas encore cherché).
- Beaucoup de logiciels inutiles pré-installés, dont certains néfastes. J'ai passé un certain temps à rechercher pourquoi mon micro enregistrait avec un écho avant de réaliser que le logiciel Maxx Audio Pro essayait de modifier le son en sortie de mon casque pour lui donner un effet 3D (personne n'a jamais dû tester ce logiciel avant de l'installer par défaut sur tous les ordinateurs Dell vu que ça ne fonctionne pas du tout…).

Mais aussi des bonnes surprises :
- J'adore Windows Hello, c'est très agréable à utiliser et très efficace pour déverrouiller son ordinateur.
- Toutes mes applications à l'exception de Redis/`rsync` tournent sans problème sur Windows. Pour référence j'ai installé : PHP, Composer, Node/NPM, MariaDB, PostgreSQL, FFMPEG, youtube-dl, Git, Github Desktop, VS Code, OpenVPN, Flutter/Dart, Rust/Rustup/Cargo et Elm. Redis fonctionne sans problème via WSL.
- Les extensions PHP sont très faciles à installer, pas besoin de compiler l'extension SSH2 par exemple, il suffit de télécharger le DLL depuis [PECL](https://windows.php.net/downloads/pecl/releases/) et de le déplacer dans mon dossier PHP.
- Les extensions Google Chrome sont compatibles avec Microsoft Edge donc aucun problème à ce niveau là. Je n'ai pas encore eu à faire de l'inspecteur d'élément pour le moment.