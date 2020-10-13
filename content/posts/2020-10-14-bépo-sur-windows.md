---
title: "Bépo sur Windows"
description: "Comment installer la disposition clavier spéciale français sur Windows ?"
---

J'utilise depuis plusieurs années Bépo, une disposition spéciale français, sur mes ordinateurs à la place du traditionnel Azerty. L'avantage de cette disposition c'est que les touches sont placées de manière à faciliter l'écriture du français (lettres les plus communes sur la ligne de repos, alternance entre les lettres main gauche/main droite…). La position des touches en Azerty est plutôt pour éviter les contraintes mécaniques des machines à écrire. Cette disposition permet également d'utiliser des caractères typographiques facilement comme les espaces insécables fines, les guillements français, les majuscule accentuées, etc.

Je n'ai jamais eu aucun soucis avec ma disposition sur Linux, mais après l'installation sur Windows je me suis rendu compte de deux problèmes :
- Sur Linux je tapais deux fois sur la touche accent grave \` afin d'en écrire un, sur Windows cette technique ne fonctionne pas car elle  me fait en caractère étrange « ̏` ».La seule technique que j'ai trouvée était de taper un accent grave puis une espace mais c'est beaucoup moins pratique.
- Deuxième problème, le créateur du pilote Bépo Windows officiel a trouvé que c'était une bonne idée d'écrire des apostrophes typographiques ’. C'est très bien pour de la rédaction, mais impossible lors de l'écriture de code où l'apostrophe est partout.

J'ai donc décidé de modifier la disposition clavier classique via le logiciel [Microsoft Keyboard Layout Creator](https://www.microsoft.com/en-us/download/details.aspx?id=102134). Ce logiciel fonctionne très bien, j'ai pu importer la disposition officielle et faire les deux changements :
- Améliorer la gestion de l'accent grave en retirant l'option « Touche morte » qui permet en tapant une fois sur l'accent grave puis sur une lettre classique de mettre cet accent sur la lettre. Je n'utilise jamais cette fonctionnalité car la disposition Bépo fournit déjà des touches pour toutes les lettres avec des accents graves. Ce changement a même amélioré mon écriture vis-à-vis de Linux car je n'ai maintenant qu'à écrire un seul accent grave pour le voir s'afficher (une fois que je serai habitué dans quelques jours car pour l'instant il m'arrive encore régulièrement d'en taper deux à la suite)
- Remplacer l'apostrophe typographique par une apostrophe normale

Mais lors de l'export, le logiciel refuse de créer la disposition car en Bépo le `_` est lié à la touche `Shift+Espace` et le logiciel est configuré pour lancé une erreur si un caractère autre qu'une espace est défini sur la touche espace.

Je me demandais comment la version officielle de la disposition avait pu être compilée avec cette « erreur » et je suis tombé sur ce forum : [Utiliser Microsoft Keyboard Layout Creator avec le fichier bepo.klc](https://forum.bepo.fr/viewtopic.php?id=1306) expliquant comment modifier l'exécutable de Microsoft Keyboard Layout Creator pour supprimer l'erreur :
```
ERROR: 'VK_SPACE' in Shift State 'Ctl+Alt' must be made up of white space character(s), but is defined as '_' (U+005f) instead.
```

Je recopie ici les explications au cas où le forum disparaitrait (ce qui arrive régulièrement !).
- Faire une copie par sécurité de l'exécutable
- Ouvrir l'exécutable avec un éditeur hexadécimal (j'ai téléchargé [HxD](https://mh-nexus.de/en/hxd/))
- Rechercher `2D 05 04 2D 02 17 2A 17 0A` (en faisant bien attention d'être en recherche mode hexadécimale)
- Remplacer `0A` par `2A`

Tout fonctionne après ces modifications.