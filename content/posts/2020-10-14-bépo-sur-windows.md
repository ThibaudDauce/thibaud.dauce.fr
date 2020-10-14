---
title: "Bépo sur Windows"
description: "Comment installer la disposition clavier spéciale français sur Windows ?"
---

J'utilise depuis plusieurs années [Bépo](https://bepo.fr/wiki/Accueil) sur mes ordinateurs à la place du traditionnel Azerty. Bépo est une disposition spécialement conçue pour le français. Son principale avantage est que les touches sont placées de manière à faciliter l'écriture du français (lettres les plus communes sur la ligne de repos, alternance entre les lettres main gauche/main droite, lettres rares sur les doigts les plus faibles…). La position des touches d'Azerty a été choisie afin d'éviter les contraintes mécaniques des machines à écrire, et n'est donc pas du tout adapté à l'écriture sur ordinateur. Bépo permet également d'utiliser facilement des caractères typographiques comme les espaces insécables fines, les guillemets français, les majuscules accentuées, etc.

Je n'ai jamais eu aucun souci avec ma disposition sur Linux, mais après l'installation sur Windows je me suis rendu compte de deux problèmes :
- Sur Linux je tapais deux fois sur la touche accent grave « \` » afin d'en écrire un (très utile pour le Markdown). Sur Windows cette technique affiche un caractère étrange « ̏` ». La seule technique que j'ai trouvée était de taper un accent grave puis une espace : beaucoup moins pratique et difficile de changer cette habitude.
- Deuxième problème, [le pilote Bépo Windows officiel](https://bepo.fr/wiki/Windows) remplace la touche « apostrophe » par des apostrophes typographiques « ’ ». C'est techniquement le bon caractère pour de la rédaction en français mais inutilisable pour un développeur qui a besoin de l'apostrophe classique dans son code.

J'ai donc décidé de modifier la disposition clavier classique via le logiciel [Microsoft Keyboard Layout Creator](https://www.microsoft.com/en-us/download/details.aspx?id=102134). Ce logiciel fonctionne très bien, j'ai pu importer la disposition officielle et faire les deux changements :
- Améliorer la gestion de l'accent grave en retirant l'option « Touche morte ». L'option « Touche morte », d'après ce que j'en ai compris, permet, en tapant une fois sur l'accent grave puis sur une lettre classique, de mettre cet accent sur la lettre. Je n'utilise jamais cette fonctionnalité car la disposition Bépo fournit déjà des touches pour toutes les lettres avec des accents graves. En retirant cette option, je n'ai maintenant qu'à appuyer une fois sur la touche pour voir s'afficher un accent grave seul. Ce changement va me demander un petit temps d'adaptation mais il sera à terme bien plus pratique que sur Linux.
- Remplacer l'apostrophe typographique par une apostrophe normale

Mais lors de l'export, le logiciel refuse de créer la disposition clavier.
```
ERROR: 'VK_SPACE' in Shift State 'Ctl+Alt' must be made up of white space character(s), but is defined as '_' (U+005f) instead.
```

En Bépo, le « _ » est lié à la touche `Shift+Espace` et le logiciel est configuré pour lancer une erreur si un caractère autre qu'une espace est défini sur la touche espace.

Je me suis donc demandé comment la version officielle de la disposition avait pu être compilée avec cette « erreur » et je suis tombé sur ce forum : « [Utiliser Microsoft Keyboard Layout Creator avec le fichier bepo.klc](https://forum.bepo.fr/viewtopic.php?id=1306) » expliquant comment modifier l'exécutable de Microsoft Keyboard Layout Creator pour supprimer l'erreur.

Je recopie ici les explications au cas où le sujet du forum disparaîtrait :
- Faire une copie de l'exécutable par sécurité
- Ouvrir l'exécutable avec un éditeur hexadécimal (j'ai téléchargé [HxD](https://mh-nexus.de/en/hxd/))
- Rechercher `2D 05 04 2D 02 17 2A 17 0A` (en faisant bien attention d'être en mode recherche hexadécimale)
- Remplacer `0A` par `2A`

Je trouve ça incroyable qu'une modification aussi simple dans l'exécutable permette de corriger ce problème sans poser de plantage autre part. Je remercie les personnes qui ont décompilé le programme et trouvé cette solution à mon problème !

Il me reste encore un problème avec la touche accent circonflexe « ^ » qui, comme pour l'accent grave, m'affiche « ôô » lors d'une répétition, alors que sous Linux cela me permettait de créer le smiley « ^^ ». Mais vu que Windows possède un clavier de smiley accessible via `Windows+.` peut-être que je n'aurais pas trop à utiliser cette touche 😊. Je ne peux pas faire la même modification que pour l'accent circonflexe (retirer l'option « Touche morte ») car je l'utilise pour écrire « ô », « î » ou « û » (Bépo n'a pas de touche dédiée à ces lettres).

Si cela vous intéresse, j'écrirai prochainement un article sur mon changement d'Azerty à Bépo, les techniques que j'ai utilisé, le temps que cela m'a pris mais ce que je peux déjà vous dire c'est que je ne regrette absolument pas le changement ! 