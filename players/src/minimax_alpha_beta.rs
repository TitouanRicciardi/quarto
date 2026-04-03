use std::arch::naked_asm;

use quarto_core::{Game, Piece};

use crate::Player;

enum Action {
    Choix,
    Placement(Piece),
}

enum ActionPrecedente {
    Placement(usize, usize),
    Piece(Piece),
    Rien,
}

const J0GAGNE: i8 = 2;
const J0PERD: i8 = -2;
const EGALITE: i8 = 0;
const J0INCONNU: i8 = 1;
const J1INCONNU: i8 = -1;
const HAUTEUR: u8 = 5;

struct Noeud {
    enfants: Vec<Noeud>,                 // Vecteur d'enfants
    joueur: u8,                          //0 ou 1
    jeu: Game,                           //etat du jeu
    action: Action,                      //placement ou choix de pion
    action_precedente: ActionPrecedente, //action parent
    score: i8,                           // -2 = vide
    hauteur: u8,                         // hauteur du noeud dans l'arbre
}

pub struct MinimaxAlphaBetaPlayer {}

impl Player for MinimaxAlphaBetaPlayer {
    fn give_piece(&mut self, game: &quarto_core::Game) -> quarto_core::Piece {
        //donner piece
        let mut noeud = Noeud {
            //Créer le noeud de la racine
            enfants: Vec::new(),
            joueur: 0, //Joueur 0
            jeu: game.clone(),
            action: Action::Choix,
            action_precedente: ActionPrecedente::Rien, //aucune action avant la racine
            score: J0INCONNU,                          //initialisation
            hauteur: 0,
        };

        create_arbre(&mut noeud); //création de l'arbre

        minimax(&mut noeud); //remplir les scores

        //Récupérer l'action précédente de l'enfant qui a le même score (min ou max)
        for enfant in &mut noeud.enfants {
            if enfant.score == noeud.score {
                match enfant.action_precedente {
                    ActionPrecedente::Placement(x, y) => {
                        panic!("L'action précédante d'un placement ne peut pas être un placement");
                    }
                    ActionPrecedente::Rien => {
                        panic!("L'action précédante d'un placement ne peut pas être nulle");
                    }
                    ActionPrecedente::Piece(piece) => {
                        return piece;
                    }
                }
            }
        }
        panic!("La boucle s'est terminée");
    }

    fn play_piece(
        &mut self,
        game: &quarto_core::Game,
        given_piece: quarto_core::Piece,
    ) -> (usize, usize) {
        let mut noeud = Noeud {
            //Création du noeud de la racine
            enfants: Vec::new(),
            joueur: 0, //Joueur 0
            jeu: game.clone(),
            action: Action::Placement(given_piece), //placement de la piece en entrée
            action_precedente: ActionPrecedente::Rien, //aucune avant la racine
            score: J0INCONNU,                       //initialisation
            hauteur: 0,
        };

        create_arbre(&mut noeud); //création de l'arbre

        minimax(&mut noeud); //remplir les scores

        //Récupérer l'action précédente de l'enfant qui a le même score (min ou max)
        for enfant in &mut noeud.enfants {
            if enfant.score == noeud.score {
                match enfant.action_precedente {
                    ActionPrecedente::Placement(x, y) => {
                        return (x, y);
                    }
                    ActionPrecedente::Rien => {
                        panic!("L'action précédante d'un choix ne peut pas être nulle");
                    }
                    ActionPrecedente::Piece(_piece) => {
                        panic!("L'action précédante d'un choix ne peut pas être un choix");
                    }
                }
            }
        }
        panic!("La boucle s'est terminée");
    }
}

fn create_arbre(noeud: &mut Noeud) {
    match noeud.action {
        Action::Choix => {
            choix_pion(noeud);
        }
        Action::Placement(piece) => {
            placer_pion(noeud, piece);
        }
    }
}

fn inconnu_joueur(joueur: u8) -> i8 {
    if joueur == 0 {
        return J0INCONNU;
    } else {
        return J1INCONNU;
    }
}

fn choix_pion(noeud: &mut Noeud) {
    for piece in noeud.jeu.stack.get_all_pieces() {
        //pour chaque pièce du sac

        let mut nouveau_jeu = noeud.jeu.clone(); //copie du jeu
        nouveau_jeu.stack.pick(piece); //retirer pièce du sac

        let mut noeud_enfant = Noeud {
            enfants: Vec::new(),
            joueur: (noeud.joueur + 1) % 2, //changement de joueur
            jeu: nouveau_jeu,
            action: Action::Placement(piece), //changement d'action
            action_precedente: ActionPrecedente::Piece(piece),
            score: inconnu_joueur((noeud.joueur + 1) % 2),
            hauteur: noeud.hauteur + 1,
        };

        if noeud_enfant.hauteur < HAUTEUR {
            create_arbre(&mut noeud_enfant);
        }
        noeud.enfants.push(noeud_enfant); //ajout de l'enfant dans le vecteur enfants
    }
}

/*
Score gagnant:
-gagant = +1 si le joueur 0 gagne
-gagant = -1 si le joueur 0 perd
-gagant = 0 si match nul
-gagant = -2 si pas fini
    */
fn placer_pion(noeud: &mut Noeud, piece: Piece) {
    for x in 0..4 {
        for y in 0..4 {
            if noeud.jeu.board.get_piece(x, y).is_none() {
                /*
                if let ActionPrecedente::Rien = noeud.action_precedente {
                    println!("Essai des coordonnées {}, {}", x, y)
                }
                */
                //si placement (x,y) vide

                let mut nouveau_jeu = noeud.jeu.clone();
                nouveau_jeu.board.set_piece(x, y, Some(piece)); //placer la pièce

                let gagnant; //score gagnant

                if nouveau_jeu.board.is_win(x, y) {
                    //quand on est sur une feuille

                    if noeud.joueur == 0 {
                        gagnant = J0GAGNE;
                    } else {
                        gagnant = J0PERD;
                    }
                } else if nouveau_jeu.stack.is_empty() {
                    //match nul
                    gagnant = EGALITE;
                } else {
                    //non fini
                    gagnant = inconnu_joueur(noeud.joueur);
                }

                let mut noeud_enfant = Noeud {
                    enfants: Vec::new(),  // Vecteur d'enfants
                    joueur: noeud.joueur, //0 ou 1
                    jeu: nouveau_jeu,     //etat du jeu
                    action: Action::Choix,
                    action_precedente: ActionPrecedente::Placement(x, y),
                    score: gagnant,
                    hauteur: noeud.hauteur + 1,
                }; // défini plus haut }

                if noeud_enfant.score == J0INCONNU || noeud_enfant.score == J1INCONNU {
                    //si jeu pas fini, continuer l'arbre
                    if noeud_enfant.hauteur < HAUTEUR {
                        create_arbre(&mut noeud_enfant);
                    }
                }
                noeud.enfants.push(noeud_enfant); // dans tous les cas, rajout de l'enfant
            }
        }
    }
}

/*récupérer score des enfants, sachant que les feuilles ont déjà leurs scores inscrits. Récupération des scores en fonction
de quel joueur joue: si J1 joue alors on prends max du score des enfants, si J0 joue alors on prends min du score des enfants
RECURSION !!!!!*/
fn minimax(noeud: &mut Noeud) {
    if noeud.enfants.is_empty() {
        // Si on est sur une feuille => condition d'arrêt
        return;
    } else {
        for enfant in &mut noeud.enfants {
            // On calcule le minmax de chaque enfant
            minimax(enfant);
        }
        if noeud.joueur == 0 {
            // On prend le score maximum des enfants
            let mut max = -2; //pire cas
            for enfant in &mut noeud.enfants {
                //boucle simple pour trouver max
                if enfant.score > max {
                    max = enfant.score;
                }
            }
            noeud.score = max; //lui donner max
        } else {
            // On prend le score minimum des enfants
            let mut min = 2; //meilleur cas
            for enfant in &mut noeud.enfants {
                //boucle simple pour trouver min
                if enfant.score < min {
                    min = enfant.score;
                }
            }
            noeud.score = min; //lui donner min
        }
    }
}
