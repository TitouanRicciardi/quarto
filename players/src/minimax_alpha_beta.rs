use std::arch::naked_asm;

use quarto_core::{Game, Piece};

use crate::Player;

enum Action {
    Choix,
    Placement(Piece),
}

struct Noeud {
    enfants: Vec<Noeud>, // Vecteur d'enfants
    joueur: u8,          //0 ou 1
    jeu: Game,           //etat du jeu
    action: Action,
    score: i8, // -2 = vide
}

struct MinimaxAlphaBetaPlayer {}

impl Player for MinimaxAlphaBetaPlayer {
    fn give_piece(&mut self, game: &quarto_core::Game) -> quarto_core::Piece {
        todo!()
    }

    fn play_piece(
        &mut self,
        game: &quarto_core::Game,
        given_piece: quarto_core::Piece,
    ) -> (usize, usize) {
        todo!()
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
                score: -2,                        //à faire !!
            };

            create_arbre(&mut noeud_enfant); //rappel de la fonction pour l'enfant
            noeud.enfants.push(noeud_enfant); //ajout de l'enfant dans le vecteur enfants
        }
    }

    /*
    Score gagnant:
    -gagant = +1 si le joueur 1 gagne
    -gagant = -1 si le joueur 1 perd
    -gagant = 0 si match nul
    -gagant = -2 si pas fini
     */
    fn placer_pion(noeud: &mut Noeud, piece: Piece) {
        for x in 0..4 {
            for y in 0..4 {
                if noeud.jeu.board.get_piece(x, y).is_none() {
                    //si placement (x,y) vide

                    let mut nouveau_jeu = noeud.jeu.clone();
                    nouveau_jeu.board.set_piece(x, y, Some(piece)); //placer la pièce

                    let gagnant; //score gagnant

                    if nouveau_jeu.board.is_win(x, y) {
                        //quand on est sur une feuille

                        if noeud.joueur == 1 {
                            gagnant = 1;
                        } else {
                            gagnant = -1;
                        }
                    } else if nouveau_jeu.stack.is_empty() {
                        //match nul
                        gagnant = 0;
                    } else {
                        //non fini
                        gagnant = -2;
                    }

                    let mut noeud_enfant = Noeud {
                        enfants: Vec::new(),  // Vecteur d'enfants
                        joueur: noeud.joueur, //0 ou 1
                        jeu: nouveau_jeu,     //etat du jeu
                        action: Action::Choix,
                        score: gagnant,
                    }; // défini plus haut }

                    if noeud_enfant.score == -2 {
                        //si jeu pas fini, continuer l'arbre
                        create_arbre(&mut noeud_enfant);
                    }
                    noeud.enfants.push(noeud_enfant); // dans tous les cas, rajout de l'enfant
                }
            }
        }
    }
}

/*récupérer score des enfants, sachant que les feuilles ont déjà leurs scores inscrits. Récupération des scores en fonction
de quel joueur joue: si J1 joue alors on prends max du score des enfants, si J0 joue alors on prends min du score des enfants
RECURSION !!!!!*/
fn minimax(mut noeud: &mut Noeud) {
    
}
