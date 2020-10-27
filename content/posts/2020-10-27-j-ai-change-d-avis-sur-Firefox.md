---
title: "J'ai changé d'avis sur Firefox"
description: "Pendant longtemps j'ai promu Firefox et débattu pour mettre fin au monopole de Chrome. J'avais tort."
---

Pendant longtemps j'ai débattu pour mettre fin au monopole de Chrome et encourager les personnes autour de moi à utiliser un navigateur autre tel que Firefox : « Le web doit rester ouvert, si tout le monde utilise Chrome, les développeurs de Google auront la possibilité de contrôler les spécifications ». Cette affirmation reste correcte aujourd'hui mais je ne pense pas que ça soit une mauvaise chose.

Beaucoup de personnes pensent que le web est le seul écosystème à permettre un développement multi-plateforme ce qui est complètement faux. Le contre-exemple le plus connu est Java, mais tous les langages basés sur une machine virtuelle qui elle-même est développée sur plusieurs plateformes le permette. Lorsque j'ai commencé à considérer le web comme une machine virtuelle, mon avis a changé.

L'avantage d'une machine virtuelle est d'avoir des spécifications précises pour permettre justement un développement unique, aujourd'hui le web est sous-optimal car les spécifications sont mal respectées par les implémentations et ces dernières doivent être mises à jour individuellement. C'est comme si les développeurs Java devaient attendre que 4 JVM différentes implémentent la nouvelle spécification pour l'utiliser, ou encore qu'une méthode d'affichage GUI donne des résultats un peu différents en fonction de la JVM du client (oui je pense à toi le CSS…).

En observant le web sous cet angle, il serait largement préférable d'avoir un seul moteur de rendu HTML/CSS et un seul interpréteur JavaScript utilisé par tous. Il serait préférable que ce moteur soit un moteur totalement open-source contrôlé par une fondation plutôt que par une entreprise mais ce n'est pas le cas aujourd'hui. Je pense que le web a déjà assez de problèmes (je pourrais en parler plus tard sur le blog…) pour rajouter en plus de ça une contrainte inutile supplémentaire.

Cette réflexion est toute récente, j'ai peut-être manqué certains avantages de cette approche, n'hésitez pas à passer sur [mon Discord](https://discordapp.com/invite/tPtVM9V) si vous voulez discuter. Aussi, je ne connais pas d'autre machine virtuelle basée sur le même modèle que le web (avec plusieurs implémentations principales), si vous en connaissez, ça m'intéresse !
