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

// TODO MOVE IT
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

// [Say]->[[Text,Text_continue],[Other_branch],[Olf is dead !]]
//
// [1]->[[2],[3],[4]]
// 1 = Say
// 2 = Text,Text_continue | karma = (0,0)
// 3 = Other_branch
// 4 = Olf is dead !

#[derive(PartialEq)]
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
                res.push(',');
            } else if let DialogType::Choice { text, condition } = dialog {
                res.push_str(&text);
                res.push(',');
            }
        }
        res.push(']');
        // Each cell is followed by a comma, except the last.
        res = res.replace(",]", "]");

        let children = String::from("->[")
            + &self
                .children
                .iter()
                .map(|tn| tn.borrow().print())
                .collect::<Vec<String>>()
                .join(",")
            + "]";
        res.push_str(&children);

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
///   - random chill dialog
/// - a huge text to cheer the fact that Olf's reign is over
///   - only enable when the event `Olf's takedown` occurs
///
/// ```rust
/// # main() -> Result<(), std::num::ParseIntError> {
///
/// let tree = init_tree(
///     String::from(
///         "[Hello]->[[I have to tell something],[You beat Olf !,Now you can chill at the hospis]]"
///     )
/// );
///
/// #     Ok(())
/// # }
/// ```
fn init_tree(s: String) -> Rc<RefCell<DialogNode>> {
    let root = Rc::new(RefCell::new(DialogNode::new()));
    let mut current = Rc::clone(&root);

    let chars = s.chars().collect::<Vec<char>>();
    for (_, c) in chars
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx > 0 && *idx + 1 < chars.len())
    {
        // if *c == '[' || c.is_alphabetic() {
        //     let child = Rc::new(RefCell::new(TreeNode::new()));
        //     current.borrow_mut().children.push(Rc::clone(&child));
        //     {
        //         let mut mut_child = child.borrow_mut();
        //         mut_child.parent = Some(Rc::clone(&current));
        //         if c.is_alphabetic() {
        //             mut_child.value.texts[0] = c.to_string();
        //         }
        //     }
        //     current = child;
        // } else if *c == ',' || *c == ']' {
        //     let current_clone = Rc::clone(&current);
        //     current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
        // } else {
        //     panic!("Unknown character: {}", c);
        // }
    }
    return root;
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
            "[Hello]->[[Hey,No Hello,Want to share a flat ?]]".to_string()
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

        assert_eq!(dialog.borrow().print(), "[Hello]->[[I have to tell something],[You beat Olf !,Now you can chill at the hospis]]".to_string());
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
            "[Hello]->[[Hey,No Hello,Want to share a flat ?]->[[:)],[:O],[Sure]]]".to_string()
        );
    }

    #[test]
    fn test_init_tree_1() {
        let tree = init_tree(String::from("[Hello]"));
        assert_eq!(
            tree.borrow().dialog_type,
            vec![DialogType::Text("Hello".to_string())]
        );
    }

    // #[test]
    // fn test_init_tree_2() {
    //     let tree = init_tree(String::from("[1,2]"));
    //     assert_eq!(tree.borrow().children[1].borrow().value.unwrap(), 2);
    // }

    // #[test]
    // fn test_init_tree_3() {
    //     let tree = init_tree(String::from("[0,1,[3,4,5,[7,8]],2]"));
    //     assert_eq!(tree.borrow().print(), "[0,1,[3,4,5,[7,8]],2]");
    // }

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
