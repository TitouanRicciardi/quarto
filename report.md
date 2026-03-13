# 1 - Présentation du jeu

*Joseph DALY, Hector STEINMETZ--KUPP,
Nilsea HANNA et Titouan RICCIARDI*

Nous avons choisi comme jeu le « Quarto ». Il est composé d’un
plateau 4x4 et de 16 pions caractérisés par avec une forme
(rond ou carré), une couleur (blanc ou noir), une profondeur
(creux ou plein) et une taille (petit ou grand). Le but est de
faire une ligne, une colonne ou une diagonale de 4 pions avec au
moins un point en commun : la taille, la profondeur, la couleur ou
la forme.

Déroulement d’une partie :

- Le Joueur 1 donne un pion au Joueur 2
- Le Joueur 2 place le pion sur le plateau
- Le Joueur 2 donne un pion aux Joueur 1

Ainsi de suite jusqu’à que l’un des joueurs gagne, ou qu’il y ait
égalité (aucune lignée de 4 avec au moins un point commun a été fait
après que tous les pions aient été posés).

C’est un jeu combinatoire puisqu’on peut voir les pièces de
l’adversaire et il n’y a pas de hasard puisque chaque joueur décide
quelle pièce il va donner à l’autre.

# 2 - Partie code

*Joseph DALY et Hector STEINMETZ--KUPP*

Pour l'implémentation logicielle, nous avons choisi d'utiliser le
langage Rust pour sa sécurité mémoire, sa performance et son
infrastructure de tests intégrée.

Nous avons utilisé un grand nombre d'optimisation, notamment de
réduction d'empreinte mémoire de toute structure utilisée. La
vitesse de notre simulation sera primordiale pour le bon
fonctionnement des algorithmes minimax plus tard.

Une pièce peut être représentée par 4 flags booléens, un pour chaque
caractéristique. Nous avons donc utilisé un entier de 8 bits pour
contenir ces flags, où les 4 premiers bits sont associés à une
caractéristique de la pièce.
Une pièce "vide" est définie comme une pièce "impossible" en temps
normal (chaque bit défini à 1).

Le plateau est une matrice carrée de dimension 4 de pièces. Aucune
optimisation particulière n'est nécessaire.

Les pièces restantes (le "sac") est un ensemble de pièces, optimisé
pour tenir entièrement en 16 bits. Sachant qu'il n'existe que 16
types de pièces valides, et qu'il n'y en a qu'une par type, nous
pouvons indexer cet ensemble par la "valeur" de chaque pièce
(sa représentation 8 bits). La valeur renvoyée par le sac est donc
une valeur booléenne ("si la pièce est disponible dans le sac").

Ainsi, un état de jeu complet peut être représenté comme une
structure contenant un plateau et un sac.

Pour représenter les joueurs, nous avons créé un trait "Joueur", qui
définit une fonction pour chaque décision possible du joueur :
choisir une pièce à donner à son adversaire, et placer une pièce
donnée. Le joueur "humain" implémente ce trait en demandant le coup
à jouer sur l'entrée standard.

On implémente une fonction pour jouer un tour de jeu, puis une
fonction pour jouer le jeu entièrement avec des joueurs donnés. On
peut donc jouer au jeu contre n'importe quel adversaire générique
qui implémente le trait "Joueur" pour simuler une partie.

# 3 - Partie algorithmique

*Nilsea HANNA et Titouan RICCIARDI*

Ce que nous avons fait :

- Compréhension de l’algo minimax : on l’a déjà appliqué au morpion
pour le comprendre et avec un petit exemple sur le quarto
- Compréhension du code qui a été fait par nos camarades
- Tentative de trouver une stratégie dans le jeu (peut-être : forcer
l’autre joueur à faire un coup perdant en faisant deux lignée avec chacune un point commun)

Ce que nous allons faire :

- Perfectionner la compréhension de l’algorithme minimax (avec
alpha-bêta)
- Se refamiliariser avec le Rust
- Implémenter l’algorithme minimax en Rust
