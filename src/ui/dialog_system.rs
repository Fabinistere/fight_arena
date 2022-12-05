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
//!
//! Tree structure based on https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75

// use bevy::prelude::*;

use std::rc::Rc;
use std::{cell::RefCell, fmt};

use bevy::prelude::{info, warn, Bundle, Component, Query, Entity};

/// Points to the current DialogNode the npc is in.
///
/// Holds a String which can be converted to a Rc<RefCell<DialogNode>>
/// by print_file()
///
/// # Example
///
/// ```rust
/// Dialog {
///      current_node: Some(
/// "# Fabien
///
/// - Hello
///
/// ## Morgan
///
/// - Hey | None
/// - No Hello | None
/// - Want to share a flat ? | None
///
/// ### Fabien
///
/// - :)
///
/// ### Fabien
///
/// - :O
///
/// ### Fabien
///
/// - Sure"
/// .to_string())
/// }
///
/// ```
#[derive(Component, PartialEq, Clone, Debug)]
pub struct Dialog {
    pub current_node: Option<String>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum DialogType {
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
pub enum GameEvent {
    BeatTheGame,
    FirstKill,
    AreaCleared,
}

impl fmt::Display for GameEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameEvent::BeatTheGame => write!(f, "BeatTheGame"),
            GameEvent::FirstKill => write!(f, "FirstKill"),
            GameEvent::AreaCleared => write!(f, "AreaCleared"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct DialogCondition {
    /// `(0,0) = infinite / no threshold`
    ///
    /// A dialog choice with a condition karma_threshold at (0,0)
    /// will always be prompted
    karma_threshold: Option<(i32, i32)>,
    event: Option<Vec<GameEvent>>,
    /// TODO REMOVE or REWORK
    ///
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

impl DialogCondition {
    pub fn is_verified(&self, karma: i32) -> bool {
        // TODO verify also if event is inclueded in all game's event triggered

        match self.karma_threshold {
            Some(karma_threshold) => {
                if karma >= karma_threshold.0 && karma <= karma_threshold.1 {
                    return true;
                }
            }
            None => {}
        }
        return false;
    }
}

/// Points to the first DialogNode
/// Used to be linked with an entity
#[derive(PartialEq, Clone, Debug)]
pub struct DialogTree {
    pub current: Option<DialogNode>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct DialogNode {
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
    /// can be an npc OR a player.
    ///
    /// The u32 is the id of the entity,
    /// The String is their name
    pub character: Option<(u32, String)>,
    pub children: Vec<Rc<RefCell<DialogNode>>>,
    /// maybe too much (prefer a stack in the TreeIterator)
    pub parent: Option<Rc<RefCell<DialogNode>>>,
    // TODO add event throw
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
    pub fn print_flat(&self) -> String {
        let mut res = String::from("[");
        for dialog in &self.dialog_type {
            if let DialogType::Text(text) = dialog {
                res.push_str(&text);
                res.push_str(", ");
            } else if let DialogType::Choice { text, condition: _ } = dialog {
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
                .map(|tn| tn.borrow().print_flat())
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

    /// # Convention
    ///
    /// ```markdown
    /// # Author 1
    ///
    /// - first element of a text
    /// - second element of a text
    ///
    /// -> Event
    ///
    /// ### Author 2
    ///
    /// - first choice for the author 2 only enable when his/her karma is low | karma: -50,0
    /// - second choice ... only enable when the event OlfIsGone has already occurs | event: OlfIsGone
    /// - thrid choice always enable | None
    /// - fourth choice with a high karma and when the events OlfIsGone and PatTheDog already occurs | karma: 10,50; event: OlfIsGone, PatTheDog
    ///
    /// #### Author 1
    ///
    /// - This text will be prompted only if the first choice is selected by the player
    /// - This DialogNode (and the three other below) is a direct child of the second DialogNode
    ///
    /// #### Author 1
    ///
    /// - This text will be prompted only if the second choice is selected by the player
    ///
    /// #### Author 1
    ///
    /// - This text will be prompted only if the third choice is selected by the player
    ///
    /// #### Author 1
    ///
    /// - This text will be prompted only if the fourth choice is selected by the player
    ///
    /// ```
    fn print_file(&self) -> String {
        return self.print_file_aux(String::from("#"));
    }

    fn print_file_aux(&self, headers: String) -> String {
        let mut res = headers.clone();

        let character: String;
        match &self.character {
            Some((_id, name)) => character = " ".to_string() + name,

            None => character = String::from(" Narator"),
        }
        res.push_str(&character);
        res.push_str("\n\n");

        for dialog in &self.dialog_type {
            if let DialogType::Text(text) = dialog {
                res.push_str("- ");
                res.push_str(&text);
            } else if let DialogType::Choice { text, condition } = dialog {
                res.push_str("- ");
                res.push_str(&text);
                res.push_str(" | ");

                match condition {
                    Some(dialog_condition) => {
                        match dialog_condition.karma_threshold {
                            Some((min, max)) => {
                                res.push_str("karma: min,max; ");
                                res = res.replace("min", &min.to_string());
                                res = res.replace("max", &max.to_string());
                            }
                            None => {}
                        }

                        match &dialog_condition.event {
                            Some(events) => {
                                if !events.is_empty() {
                                    // plurial ?
                                    res.push_str("event: ");
                                    for event in events {
                                        res.push_str(&event.to_string());
                                        res.push_str(",");
                                    }
                                    res.push_str(";");
                                    res = res.replace(",;", ";");
                                }
                            }
                            None => {}
                        }

                        // we might remove the index
                    }

                    None => {
                        res.push_str("None\n");
                    }
                }
            }
        }
        res.push_str("\n\nEND");

        // res = res.replace("\n\n\n", "\n\n");

        // event

        let children = &self
            .children
            .iter()
            .map(|tn| tn.borrow().print_file_aux(headers.clone() + "#"))
            .collect::<Vec<String>>()
            .join("\n\n");

        res.push_str(&children);

        res = res.replace("#\n\n#", "##");
        res = res.replace("\n\n\n", "\n\n");

        // remove the last "\n\n"
        res = res.replace("END#", "#");
        res = res.replace("\n\nEND", "");

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
/// let tree: DialogTree = init_tree_flat(
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
pub fn init_tree_flat(s: String) -> Rc<RefCell<DialogNode>> {
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
/// A NPC's catchphrase followed by two possible outcomes/choices
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
/// let tree: DialogTree = init_tree_file(
///     String::from(
///         "# Morgan
///
/// - Hello
///
/// ## Fabien lae Random
///
/// - I have to tell something | None
/// - You beat Olf ! | event: OlfBeaten
///
/// ### Fabien lae Random
///
/// - Something | None
/// - Now you can chill at the hospis | event: OpenTheHospis"
///     )
/// );
///
/// #     Ok(())
/// # }
/// ```
pub fn init_tree_file(s: String) -> Rc<RefCell<DialogNode>> {
    let root = Rc::new(RefCell::new(DialogNode::new()));

    let mut current = root.clone();

    let mut save = String::new();
    // init with text
    let mut dialog_type: DialogType = DialogType::new_text();

    // Check if a given char should be insert as text
    // allow to have text with special char (like [ ] > , ;)
    let mut except = false;

    let mut headers_numbers = 0;

    // root.borrow_mut().dialog_type = vec![DialogType::Text(s.replace("[", "").replace("]", ""))];

    let chars = s.chars().collect::<Vec<char>>();
    for (_, c) in chars
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx < chars.len())
    {
        if *c == '#' && !except {
            headers_numbers += 1;
        } else if c.is_ascii() || c.is_alphanumeric() {
            // the except char \
            if *c == '\\' {
                except = true;
            }
            // ignore the new line: "\n"
            else if *c == 'n' && except {
                except = false;
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
