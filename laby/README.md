# Polytech/INFO3/ALM -- Labyrithe interactif C / ASM

_Exerice 4 du TP final d'ALM soft_

## Description

Ce programme est un labyrithe interactif. Lors du lancement, un menu nous permet de choisir entre plusieurs configurations (tailles et formes différentes). Après la création du labyrithe, on peut naviguer dedans avec les flèches du clavier et tenter d'atteindre la sortie marquée par un arrière-plan bleu.

## Lancement

Terminal 1:

`make qemu`

Terminal 2:

```
make gdb
...
cont
```

# Fonctionalités

  * Plusieurs configurations de labyrithe possibles: rectangle, logo polytech
  * A défaut d'avoir accès à une source d'entropie simplement, le générateur de nombres pseudo-aléatoires (un [générateur congruentiel linéaire](https://en.wikipedia.org/wiki/Linear_congruential_generator)) est initialisé avec le nombre d'appui sur des flèches sur l'écran de sélection. Ainsi, on peut obtenir des labyrinthes différents en déplaçant plusieurs fois la sélection avant de lancer le jeu
  * Les labyrinthes peuvent soit être rectangles, soit avoir une forme personalisée extraite d'un fichier XBM (une image monochromatique)
  * Une piste est affichée derrière le curseur, montrant le chemin actuel. Les positions sont stockées dans une pile car il était initialement prévu de permettre un retour arrière rapide au moyen d'une touche dédiée
  * Couleurs et déplacements de curseur assurés par les codes ANSI supportés par la majorité des terminaux

## Personalisation des modèles de labyrinthe

Le fichier `maze_configs.c` peut être modifié pour rajouter de nouveaux modèles de labyrinthe qui s'afficheront dans le menu principal. Il faut bien penser à rajouter le nom du labyrinthe au tableau `MAZES[]` en laissant un `NULL` pour marquer la fin.

Si le labyrinthe a sa valeur `.custom_mask = NULL`, il sera rectangle. Sinon il aura la forme du masque. Les masques sont décrit au format [XBM](https://en.wikipedia.org/wiki/X_BitMap), un format d'images monochromatiques textuel utilisant la syntaxe C pour représenter les dimensions de l'image et la valeur de ses pixels. Ces fichiers peuvent êtres générés grâce au logiciel d'édition d'images GIMP.

Attention: les labyrinthes ont une taille maximale donnée par `max_maze_width` et `max_maze_height` dans le fichier `maze_gen.c`.

## Répartition du code et dépendences

Côté C:
  * génération du labyrinthe
  * configuration des modèles de labyrinthe dans `maze_configs.{c,h}`
  * affichage des cellules dans la console
  * génération de nombres pseudo-aléatoires

Côté Assembleur:
  * point d'entrée principal
  * menu de sélection du modèle
  * boucle principale: lecture des flèches, empilement et dépilement des positions, détection de la fin du jeu

Dépendences (fichiers +- externes):
  * `print_string.s`: l'exercice 1, permet d'afficher des chaines de caractères d'un seul coup
  * `io.s`: fichier fourni lors d'un TP précédent, pour afficher et lire des caractères sur le port UART virtuel de QEMU
  * `vendored/uidiv.s`: morceau de la bibliothèque [agbabi](https://github.com/felixjones/agbabi/blob/master/source/uidiv.s). Fournit une implémentation de la fonction `__aeabi_uidivmod` émise par gcc pour effectuer un modulo. En effet, ARM ne supporte pas cette opération nativement et il faut l'émuler, et la plupart des compilateurs remplacent leurs divisions par un appel à une fonction nommée `__aeabi_uidivmod` qu'ils n'implémentent pas eux même.

---

Edgar Onghena

Mattéo Decorsaire (exercice 1)
