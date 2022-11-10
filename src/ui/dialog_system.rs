//! Dialog System
//!
//! Complex
//!
//! - A struct DialogNode can be insert to an entity
//!   - This Node may contains
//!     - some Text
//!     - some Choice
//!   - A specific Dialog can have some conditon
//!     - Karma based
//!     - Event based
//!     - Choice based

// use bevy::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use bevy::prelude::{info, warn};

/// TODO This may be not usefull
/// because the current node will be contained in the npc
///
/// The i32 stored will refer to the actual state of the conversation
/// might rework it to protect overload adn misunderstood
/// could be a pointer to the current state (position) of the dialog
/// To enable pause/remuse whenever the player wants to
pub struct DialogState(pub i32);

#[derive(PartialEq, Clone, Debug)]
enum DialogType {
    Text(String),
    Choice {
        text: String,
        condition: Option<DialogCondition>,
    },
}

impl DialogType {
    fn new_text() -> DialogType {
        DialogType::Text(String::from(""))
    }

    fn new_choice() -> DialogType {
        DialogType::Choice {
            text: String::from(""),
            condition: None,
        }
    }

    /// Only compare the type,
    /// it don't compare fields
    fn eq(&self, comp: DialogType) -> bool {
        match (self.clone(), comp) {
            (DialogType::Text(_), DialogType::Text(_)) => return true,
            (
                DialogType::Choice {
                    text: _text1,
                    condition: _cond1,
                },
                DialogType::Choice {
                    text: _text2,
                    condition: _cond2,
                },
            ) => return true,
            _ => return false,
        }
    }

    fn is_choice(&self) -> bool {
        match self.clone() {
            DialogType::Choice {
                text: _text,
                condition: _cond,
            } => return true,

            DialogType::Text(_) => return false,
        }
    }

    fn is_text(&self) -> bool {
        match self.clone() {
            DialogType::Text(_) => return true,
            _ => return false,
        }
    }
}

// TODO MOVE IT UP
#[derive(PartialEq, Clone, Debug)]
enum GameEvent {
    BeatTheGame,
    FirstKill,
    AreaCleared,
}

#[derive(PartialEq, Clone, Debug)]
struct DialogCondition {
    /// `(0,0) = infinite / no threshold`
    ///
    /// A dialog choice with a condition karma_threshold at (0,0)
    /// will always be prompted
    karma_threshold: Option<(i32, i32)>,
    event: Option<GameEvent>,
    /// The position of the parent choice made to access this node
    ///
    /// Start at 1 for the first choice
    ///
    /// Does not require to imply :
    ///
    /// - that a child Node will be treated after the last sentence of a `Vec<Text>`
    /// as if the `index_parent` = index of the last sentence (length)
    index_parent: Option<i32>,
}

/// Points to the first DialogNode
/// Used to be linked with an entity
#[derive(PartialEq, Clone, Debug)]
struct DialogTree {
    root: Rc<RefCell<DialogNode>>,
}

#[derive(PartialEq, Clone, Debug)]
struct DialogNode {
    /// Choice can have multiple children (to give a real impact to the choice)
    ///
    /// Every character can have multitude choice
    ///
    /// - A Roaster of answer enable for the player
    ///   - each with certain conditions to be enabled
    /// - A Roaster of catchphrase for npc
    ///   - with a priority system to alloy important event
    ///   to take over the ambiance
    ///   - it will permit more life and warm content,
    ///   some not cold npc to grow with the place
    pub dialog_type: Vec<DialogType>,
    /// Actor / Actress
    ///
    /// can be an npc OR a player
    pub character: Option<u32>,
    pub children: Vec<Rc<RefCell<DialogNode>>>,
    // maybe too much (prefer a stack in the TreeIterator)
    pub parent: Option<Rc<RefCell<DialogNode>>>,
}

impl DialogNode {
    pub fn new() -> DialogNode {
        return DialogNode {
            dialog_type: vec![],
            character: None,
            children: vec![],
            parent: None,
        };
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<DialogNode>>) {
        self.children.push(new_node);
    }

    /// # Convention
    ///
    /// - parent->child
    /// - adelphe_1, adelphe_2
    /// - [member_1, member_2] == A group
    ///
    /// # Examples
    ///
    /// `[parent]->[child]`
    ///      *parent* has only one outcome, *child*
    ///
    /// `[obj_1]->[[obj_2], [obj_3]]`
    ///      *obj_1* has *obj_2* and *obj_3* as children
    ///      After *obj_1*, the two outcome possible are *obj_2* or *obj_3*
    ///
    /// ["CP"]->["Hello"->["NiceTalk"], "No Hello"->["BadTalk"], "Give ChickenSandwich"->["WinTalk"]]
    ///
    pub fn print(&self) -> String {
        let mut res = String::from("[");
        for dialog in &self.dialog_type {
            if let DialogType::Text(text) = dialog {
                res.push_str(&text);
                res.push_str(", ");
            } else if let DialogType::Choice { text, condition } = dialog {
                res.push_str(&text);
                res.push_str(", ");
            }
        }
        res.push(']');
        // Each cell is followed by a comma, except the last.
        res = res.replace(", ]", "]");

        let children = String::from("->[")
            + &self
                .children
                .iter()
                .map(|tn| tn.borrow().print())
                .collect::<Vec<String>>()
                .join("; ")
            + "]";

        res.push_str(&children);

        // Each children is followed by a semi-colon, except the last.
        res = res.replace("; ]", "]");

        // remove when being a leaf (having no child)
        res = res.replace("->[]", "");

        return res;
    }
}

/// # Argument
///
/// * `s` - A string that holds a DialogTree
///
/// # Panics
///
/// The creation will panic
/// if any argument to the process is not valid DialogTree format
///
/// # Examples
///
/// A NPC's catchphrase followed by two possible outcomes
///
/// - a generic one
///   - random chill dialog,
///   a simple Text("I have to tell something")
/// - a huge text to cheer the fact that Olf's reign is over
///   - only enable when the event `Olf's takedown` occurs
///   - a first simple Text with a second simple Text as child
///
/// ```rust
/// # main() -> Result<(), std::num::ParseIntError> {
///
/// let tree = init_tree(
///     String::from(
///         "[Hello]->[[I have to tell something], [You beat Olf !]->[[Now you can chill at the hospis]]]"
///     )
/// );
///
/// #     Ok(())
/// # }
/// ```
///
/// For a future:
///
/// ```
/// [Say]->[[Text,Text_continue],[Other_branch],[Olf is dead !]]
///
/// [1]->[[2],[3],[4]]
/// 1 = Say
/// 2 = Text,Text_continue | karma = (0,0)
/// 3 = Other_branch
/// 4 = Olf is dead !
/// ```
fn init_tree(s: String) -> Rc<RefCell<DialogNode>> {
    let root = Rc::new(RefCell::new(DialogNode::new()));

    let mut current = root.clone();

    let mut save = String::new();
    // init with text
    let mut dialog_type: DialogType = DialogType::new_text();

    // Check if a given char should be insert as text
    // allow to have text with special char (like [ ] > , ;)
    let mut except = false;

    // root.borrow_mut().dialog_type = vec![DialogType::Text(s.replace("[", "").replace("]", ""))];

    let chars = s.chars().collect::<Vec<char>>();
    for (_, c) in chars
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx < chars.len())
    {
        if *c == ']' && save.is_empty() {
            // go Up

            match &current.clone().borrow().parent {
                Some(parent) => current = Rc::clone(&parent),
                None => info!("orphan"),
            }
        } else if (*c == ',' || (*c == ']' && dialog_type.is_choice() && !except))
            && !save.is_empty()
        {
            dialog_type = DialogType::new_choice();

            // remove first blank before the dialog
            if save.chars().nth(0) == Some(' ') {
                save.remove(0);
            }

            let choice = DialogType::Choice {
                text: save.clone(),
                condition: None,
            };
            current.borrow_mut().dialog_type.push(choice);

            // println!("choice with {}", save.clone());

            save.clear();
        } else if *c == ']' && dialog_type.is_text() && !except && !save.is_empty() {
            // remove first blank before the dialog
            if save.chars().nth(0) == Some(' ') {
                save.remove(0);
            }

            let text = DialogType::Text(save.clone());
            current.borrow_mut().dialog_type.push(text);

            // println!("text with {}", save.clone());

            save.clear();
        } else if *c == '>' && !except {
            // remove the dash only if it's before >
            // this allow the text to have dash in it
            if save.chars().nth(save.len() - 1) == Some('-') {
                save.remove(save.len() - 1);
            }

            let child = Rc::new(RefCell::new(DialogNode::new()));
            current.borrow_mut().children.push(Rc::clone(&child));
            {
                let mut mut_child = child.borrow_mut();
                mut_child.parent = Some(Rc::clone(&current));
            }

            // setting up the *second* link parent
            child.borrow_mut().parent = Some(current);

            // go down into a new child
            current = child;

            // Reset the default type
            dialog_type = DialogType::new_text();
        } else if *c == ';' && !except {
            let adelphe = Rc::new(RefCell::new(DialogNode::new()));

            match &current.borrow().parent {
                Some(parent) => {
                    // setting the same parent as the previous adelphe
                    adelphe.borrow_mut().parent = Some(Rc::clone(&parent));

                    parent.borrow_mut().children.push(Rc::clone(&adelphe));
                    // setting the same parent as the previous adelphe
                    // {
                    //     let mut mut_adelphe = adelphe.borrow_mut();
                    //     mut_adelphe.parent = Some(Rc::clone(&parent));
                    // }
                }

                // A group of adelphe are'nt suppose to be orphean
                None => {
                    warn!("Adelphes/group of children being orphean");
                    // the field parent of our new adelphe is already None
                }
            }

            // go 'right' into a new adelphe
            current = adelphe;

            // Reset the default type
            dialog_type = DialogType::new_text();
        }
        // instead of `&& (*c != '[' || (*c == '['  && except))` in the next `else if c.is_ascii()`
        // else if *c == '[' && !except {
        // }
        else if c.is_ascii() || c.is_alphanumeric() {
            // the except char \
            if *c == '\\' {
                except = true;
            } else if !is_special_char(*c) || (is_special_char(*c) && except) {
                save.push(*c);
                except = false;

                // println!("add {}", *c);
            }
        }
    }

    return root;
}

fn is_special_char(c: char) -> bool {
    // not '-' cause handle differently
    let special_char: Vec<char> = vec!['\\', ';', ',', '>', '[', ']'];

    return special_char.contains(&c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_child_print() {
        let dialog = Rc::new(RefCell::new(DialogNode::new()));
        dialog.borrow_mut().dialog_type = vec![DialogType::Text(String::from("Hello"))];

        let answers = Rc::new(RefCell::new(DialogNode::new()));
        answers.borrow_mut().dialog_type = vec![
            DialogType::Choice {
                text: String::from("Hey"),
                condition: None,
            },
            DialogType::Choice {
                text: String::from("No Hello"),
                condition: None,
            },
            DialogType::Choice {
                text: String::from("Want to share a flat ?"),
                condition: None,
            },
        ];

        dialog.borrow_mut().add_child(answers);

        assert_eq!(
            dialog.borrow().print(),
            "[Hello]->[[Hey, No Hello, Want to share a flat ?]]".to_string()
        );
    }

    #[test]
    fn test_children_print() {
        let dialog = Rc::new(RefCell::new(DialogNode::new()));
        dialog.borrow_mut().dialog_type = vec![DialogType::Choice {
            text: String::from("Hello"),
            condition: None,
        }];
        // The Player
        dialog.borrow_mut().character = Some(0b0000001u32);

        let random_dialog = Rc::new(RefCell::new(DialogNode::new()));
        random_dialog.borrow_mut().dialog_type =
            vec![DialogType::Text(String::from("I have to tell something"))];
        // The npc
        random_dialog.borrow_mut().character = Some(0b0000010u32);
        dialog.borrow_mut().add_child(random_dialog);

        let olf_no_longer_a_dj = Rc::new(RefCell::new(DialogNode::new()));
        olf_no_longer_a_dj.borrow_mut().dialog_type = vec![
            DialogType::Text(String::from("You beat Olf !")),
            DialogType::Text(String::from("Now you can chill at the hospis")),
        ];
        // The npc
        olf_no_longer_a_dj.borrow_mut().character = Some(0b0000010u32);
        dialog.borrow_mut().add_child(olf_no_longer_a_dj);

        assert_eq!(dialog.borrow().print(), "[Hello]->[[I have to tell something]; [You beat Olf !, Now you can chill at the hospis]]".to_string());
    }

    #[test]
    fn test_complex_print() {
        let dialog = Rc::new(RefCell::new(DialogNode::new()));
        dialog.borrow_mut().dialog_type = vec![DialogType::Text(String::from("Hello"))];

        let answers = Rc::new(RefCell::new(DialogNode::new()));
        answers.borrow_mut().dialog_type = vec![
            DialogType::Choice {
                text: String::from("Hey"),
                condition: None,
            },
            DialogType::Choice {
                text: String::from("No Hello"),
                condition: None,
            },
            DialogType::Choice {
                text: String::from("Want to share a flat ?"),
                condition: None,
            },
        ];

        let dialog_2 = Rc::new(RefCell::new(DialogNode::new()));
        dialog_2.borrow_mut().dialog_type = vec![DialogType::Text(String::from(":)"))];

        let dialog_3 = Rc::new(RefCell::new(DialogNode::new()));
        dialog_3.borrow_mut().dialog_type = vec![DialogType::Text(String::from(":O"))];

        let dialog_4 = Rc::new(RefCell::new(DialogNode::new()));
        dialog_4.borrow_mut().dialog_type = vec![DialogType::Text(String::from("Sure"))];

        answers.borrow_mut().add_child(dialog_2);
        answers.borrow_mut().add_child(dialog_3);
        answers.borrow_mut().add_child(dialog_4);

        dialog.borrow_mut().add_child(answers);

        assert_eq!(
            dialog.borrow().print(),
            "[Hello]->[[Hey, No Hello, Want to share a flat ?]->[[:)]; [:O]; [Sure]]]".to_string()
        );
    }

    #[test]
    fn test_init_tree_simple_text_1() {
        let tree = init_tree(String::from("[Hello]"));
        assert_eq!(
            tree.borrow().dialog_type,
            vec![DialogType::Text("Hello".to_string())]
        );
    }

    #[test]
    fn test_init_tree_simple_text_2() {
        let tree = init_tree(String::from("[I want to talk]"));
        assert_eq!(
            tree.borrow().dialog_type,
            vec![DialogType::Text("I want to talk".to_string())]
        );
    }

    #[test]
    fn test_init_tree_simple_choice_1() {
        // carefull with
        let tree = init_tree(String::from("[Let's talk,I don't wanna talk]"));
        assert_eq!(
            tree.borrow().dialog_type,
            vec![
                DialogType::Choice {
                    text: "Let's talk".to_string(),
                    condition: None
                },
                DialogType::Choice {
                    text: "I don't wanna talk".to_string(),
                    condition: None
                }
            ]
        );
    }

    #[test]
    fn test_init_tree_simple_choice_spaced() {
        // carefull with
        let tree = init_tree(String::from("[Let's talk, I don't wanna talk]"));
        assert_eq!(
            tree.borrow().dialog_type,
            vec![
                DialogType::Choice {
                    text: "Let's talk".to_string(),
                    condition: None
                },
                DialogType::Choice {
                    text: "I don't wanna talk".to_string(),
                    condition: None
                }
            ]
        );
    }

    #[test]
    fn test_init_tree_famiglia_figlio_unico() {
        // TODO find out what the warning referred to
        // carefull with ???
        let tree = init_tree(String::from(
            "[Catchphrase]->[[I love you, Give me your wallet]]",
        ));

        assert_eq!(
            tree.borrow().dialog_type,
            vec![DialogType::Text("Catchphrase".to_string())]
        );

        assert_eq!(
            tree.borrow().children[0].borrow().dialog_type,
            vec![
                DialogType::Choice {
                    text: "I love you".to_string(),
                    condition: None
                },
                DialogType::Choice {
                    text: "Give me your wallet".to_string(),
                    condition: None
                }
            ]
        );
    }

    #[test]
    fn test_init_tree_famiglia() {
        let tree = init_tree(String::from(
            "[Catchphrase]->[[I love you, Give me your wallet]->[[Me Too]; [Here all my chicken sandwich]]]",
        ));

        assert_eq!(
            tree.borrow().children[0].borrow().dialog_type,
            vec![
                DialogType::Choice {
                    text: String::from("I love you"),
                    condition: None
                },
                DialogType::Choice {
                    text: String::from("Give me your wallet"),
                    condition: None
                }
            ]
        );

        assert_eq!(
            tree.borrow().children[0].borrow().children[0]
                .borrow()
                .dialog_type,
            vec![DialogType::Text("Me Too".to_string())]
        );

        assert_eq!(
            tree.borrow().children[0].borrow().children[1]
                .borrow()
                .dialog_type,
            vec![DialogType::Text("Here all my chicken sandwich".to_string())]
        );
    }

    #[test]
    fn test_init_tree_complex_famiglia() {
        // helped me see that accent wasn't include in the ascii char

        let tree = init_tree(String::from(
            "[Il faut absolument sauver les Fabien du Chien Géant]->[[Il me faut donc obtenir le trône...]->[[...,et de l'argent]->[[Et de l'argent]->[[C'est essentiel]]];[C'est essentiel]]]",
        ));

        // root
        assert_eq!(
            tree.borrow().dialog_type,
            vec![DialogType::Text(String::from(
                "Il faut absolument sauver les Fabien du Chien Géant"
            ))]
        );

        // first and only child of root
        assert_eq!(
            tree.borrow().children[0].borrow().dialog_type,
            vec![DialogType::Text(String::from(
                "Il me faut donc obtenir le trône..."
            ))]
        );

        // choose of the player
        assert_eq!(
            tree.borrow().children[0].borrow().children[0]
                .borrow()
                .dialog_type,
            vec![
                DialogType::Choice {
                    text: "...".to_string(),
                    condition: None
                },
                DialogType::Choice {
                    text: "et de l'argent".to_string(),
                    condition: None
                }
            ]
        );

        // two possible outcome from the player's answer

        // first (when the player don't say anything == "...")
        assert_eq!(
            tree.borrow().children[0].borrow().children[0]
                .borrow()
                .children[0]
                .borrow()
                .dialog_type,
            vec![DialogType::Text("Et de l'argent".to_string())]
        );
        // just after the npc said "Et de l'argent"
        assert_eq!(
            tree.borrow().children[0].borrow().children[0]
                .borrow()
                .children[0]
                .borrow()
                .children[0]
                .borrow()
                .dialog_type,
            vec![DialogType::Text("C'est essentiel".to_string())]
        );

        // println!("{}", tree.borrow().print());

        // second
        assert_eq!(
            tree.borrow().children[0].borrow().children[1]
                .borrow()
                .dialog_type,
            vec![DialogType::Text("C'est essentiel".to_string())]
        );

        // test print
        assert_eq!(
            tree.borrow().print(),
            "[Il faut absolument sauver les Fabien du Chien Géant]->[[Il me faut donc obtenir le trône...]->[[..., et de l'argent]->[[Et de l'argent]->[[C'est essentiel]]]; [C'est essentiel]]]".to_string()
        );
    }

    // #[test]
    // fn test_add_child() {
    //     let tree = init_tree(String::from("[0,1,[3,4,5,[7,8]],2]"));
    //     let new_node = Rc::new(RefCell::new(TreeNode::new()));
    //     new_node.borrow_mut().value = Some(9);
    //     let child = &tree.borrow().children[2];
    //     child.borrow_mut().add_child(new_node);
    //     assert_eq!(tree.borrow().print(), "[0,1,[3,4,5,[7,8],9],2]");
    // }
}
