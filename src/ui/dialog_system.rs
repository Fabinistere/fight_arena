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
//!   - A node can send Specific Event
//!
//! Tree structure based on https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75

// use bevy::prelude::*;

use std::{cell::RefCell, fmt, rc::Rc, str::FromStr};

use bevy::prelude::{info, warn, Component};

use crate::constants::character::{KARMA_MAX, KARMA_MIN};

// mod tests;

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
    /// it don't compare content
    fn _eq(&self, comp: DialogType) -> bool {
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

// TODO: MOVE IT UP
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GameEvent {
    BeatTheGame,
    FirstKill,
    AreaCleared,
    HasCharisma,
    HasFriend,
}

impl fmt::Display for GameEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameEvent::BeatTheGame => write!(f, "BeatTheGame"),
            GameEvent::FirstKill => write!(f, "FirstKill"),
            GameEvent::AreaCleared => write!(f, "AreaCleared"),
            GameEvent::HasCharisma => write!(f, "HasCharisma"),
            GameEvent::HasFriend => write!(f, "HasFriend"),
        }
    }
}

impl FromStr for GameEvent {
    type Err = (); // ParseIntError;

    fn from_str(input: &str) -> Result<GameEvent, Self::Err> {
        match input {
            "BeatTheGame" => Ok(GameEvent::BeatTheGame),
            "FirstKill" => Ok(GameEvent::FirstKill),
            "AreaCleared" => Ok(GameEvent::AreaCleared),
            "HasCharisma" => Ok(GameEvent::HasCharisma),
            "HasFriend" => Ok(GameEvent::HasFriend),
            _ => Err(()),
        }
    }
}

/// Happens in
///   - ui::dialog_player
///     - dialog_dive
///     Exit a node
/// Read in
///   - ui::dialog_player
///     - throw_trigger_event
///     Match the Enum and handle it
///     REFACTOR: or/and TriggerEvent Handle by sending these real Event
pub struct TriggerEvent(pub Vec<ThrowableEvent>);
// pub struct FightEvent;

/// DOC
/// 
/// List all triggerable event,
/// that can be send when quitting a dialog node
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ThrowableEvent {
    FightEvent,
    HasFriend,
}

impl fmt::Display for ThrowableEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ThrowableEvent::FightEvent => write!(f, "FightEvent"),
            ThrowableEvent::HasFriend => write!(f, "HasFriend"),
        }
    }
}

impl FromStr for ThrowableEvent {
    type Err = (); // ParseIntError;

    fn from_str(input: &str) -> Result<ThrowableEvent, Self::Err> {
        match input {
            "FightEvent" => Ok(ThrowableEvent::FightEvent),
            "HasFriend" => Ok(ThrowableEvent::HasFriend),
            _ => Err(()),
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
}

impl DialogCondition {
    pub fn new() -> DialogCondition {
        DialogCondition {
            karma_threshold: None,
            event: None,
        }
    }

    pub fn is_verified(&self, karma: i32) -> bool {
        // TODO: feature - also check if its event has been already triggered in the game

        match self.karma_threshold {
            Some(karma_threshold) => {
                if karma >= karma_threshold.0 && karma <= karma_threshold.1 {
                    return true;
                }
            }
            // no karma condition
            None => return true,
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
    pub trigger_event: Vec<ThrowableEvent>,
}

impl DialogNode {
    pub fn new() -> DialogNode {
        return DialogNode {
            dialog_type: vec![],
            character: None,
            children: vec![],
            parent: None,
            trigger_event: vec![],
        };
    }

    pub fn is_end_node(&self) -> bool {
        return self.children.is_empty();
    }

    /// # Return
    ///
    /// true if the type of the first element (of dialog_type) is choice
    pub fn is_choice(&self) -> bool {
        if !self.dialog_type.is_empty() {
            return self.dialog_type[0].is_choice();
        }
        return false;
    }

    /// # Return
    ///
    /// true if the type of the first element (of dialog_type) is choice
    pub fn is_text(&self) -> bool {
        if !self.dialog_type.is_empty() {
            return self.dialog_type[0].is_text();
        }
        return false;
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
    /// - first choice for the author 2 only enable when his/her karma is low | karma: -50,0;
    /// - second choice ... only enable when the event OlfIsGone has already occurs | event: OlfIsGone;
    /// - thrid choice always enable | None
    /// - fourth choice with a high karma and when the events OlfIsGone and PatTheDog already occurs | karma: 10,50; event: OlfIsGone, PatTheDog;
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
    pub fn print_file(&self) -> String {
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
                    }

                    None => {
                        res.push_str("None");
                    }
                }
            }
            res.push('\n');
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

        // smooth the end of phase
        res = res.replace(" \n", "\n");

        res = res.replace("#\n\n#", "##");
        res = res.replace("\n\n\n", "\n\n");

        // remove the last "\n\n"
        res = res.replace("END#", "#");
        // keep only one last '\n' (to respect init rules)
        res = res.replace("\n\nEND", "\n");

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
/// The MainCharacter's catchphrase followed by two possible outcomes
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
#[deprecated(
    since = "0.3.0",
    note = "init_tree_flat doesn't implement enought features and is less intuitive than init_tree_file. Users should instead use init_tree_file"
)]
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
            if *c == '/' {
                except = true;
            } else if !is_special_char_flat(*c) || (is_special_char_flat(*c) && except) {
                save.push(*c);
                except = false;

                // println!("add {}", *c);
            }
        }
    }

    return root;
}

fn is_special_char_flat(c: char) -> bool {
    let special_char: Vec<char> = vec!['/', '[', ']', '|', '>', ';', ','];

    return special_char.contains(&c);
}

/// # Argument
///
/// * `s` - A string that holds a DialogTree
///
/// # Panics
///
/// The creation will panic
/// if any argument to the process is not valid DialogTree format.
///
/// # Conventions
///
/// ## Dialog Creation
///
/// A Dialog can involve as many characters as you like.
/// A Dialog's Node is composed of:
///
/// - the author
/// - the content
///   - A serie of text;
///   To create a monologue.
///   - A serie of choice.
///
///     If the author of this serie is
///     - a NPC:
///     The choice will be selected by a priority system,
///     Important Event dialog before random dialog.
///     - the Main Character:
///     The choices to be made will be indicated to the player.
///     In The player scroll at the bottom the UI Wall.
/// - the event the Node will launch when it's exited.
///
/// ## Rules
///
/// - A choice is made of
///   - a text
///   - condition; Must end by `;` if != None
///     - karma threshold; (x,y) with x, y ∈ N.
///     The player's karma must be within this certain range
///     - event;
///     All followed events must be triggered to enable this choice.
/// - A text can have only one child
/// - A dialog node cannot have more than one type of dialog_type
///   - for example
///   within a same DialogNode, having a text and a choice in the dialog_type field
///   will break soon or later
///   FIXME: make it break soon as possible
///
/// ***The end marker of the dialog is `\n`.***
/// You **MUST** end this string by a `\n`.
///
/// ## Tips
///
/// - You can type
///   - `k: x,y;` instead of `karma: x,y;`
///   - `e: Event1, Event2;` instead of `event: Event1, Event2;`
/// - You can use `MAX`/`MIN` to pick the highest/lowest karma threshold possible
/// - Prefere not typing anything if it's something like this: `k: MIN,MAX;`
/// - No matter in the order: `karma: 50,-50;` will result by `karma_threshold: Some((-50,50))`
/// - To use '-', '\n', '|', '#' or '>' prefere use the char '/' just before these chars
/// to prevent unwanted phase transition (from content phase to whatever phase)
///
/// # Examples
///
/// The MainCharacter's catchphrase is followed
/// by two possible outcomes/choices for the interlocutor.
///
/// - a generic one
///   - random chill dialog,
///   a simple Text("I have to tell something")
/// - a huge text to cheer the fact that Olf's reign is over
///   - only enable when the event `Olf's takedown` occurs
///   - a first simple Text with a second simple Text as child
///
/// ```rust
/// # // ignore the extra '#' on the example (code block )
/// # main() -> Result<(), std::num::ParseIntError> {
///
/// let tree: DialogTree = init_tree_file(
///     String::from(
/// "# Morgan
///
/// - Hello
///
/// ### Fabien lae Random
///
/// - I have to tell something | None
/// - You beat Olf ! | event: OlfBeaten;
///
/// #### Fabien lae Random
///
/// - Something
///
/// #### Fabien lae Random
///
/// - Now you can chill at the hospis
///
/// -> OpenTheHospis\n"
///     )
/// );
/// #     Ok(())
/// # }
/// ```
///
/// To create a long monologue,
/// avoid using `\n`, prefere using `-`
/// to seperated paragraph
///
/// ```rust
/// # main() -> Result<(), std::num::ParseIntError> {
///
/// let tree: DialogTree = init_tree_file(
///     String::from(
/// "# Olf
///
/// - Hello
/// - Do you mind giving me your belongings ?
/// - Or maybe...
/// - You want to fight me ?
///
/// ### Morgan
///
/// - Here my money | e: WonTheLottery;
/// - You will feel my guitar | None
/// - Call Homie | k: 10,MAX;
///
/// #### Olf
///
/// - Thank you very much
///
/// #### Olf
///
/// - Nice
/// -> FightEvent
///
/// #### Olf
///
/// - Not Nice
/// -> FightEvent\n"
///     )
/// );
///
/// #     Ok(())
/// # }
/// ```
pub fn init_tree_file(s: String) -> Rc<RefCell<DialogNode>> {
    let root = Rc::new(RefCell::new(DialogNode::new()));

    let mut current = root.clone();

    let mut author = String::new();
    let mut save = String::new();
    // k, e, karma or event
    let mut condition = DialogCondition::new();
    // CANT due to reading text one letter/number by one: avoid using karma as a string into .parse::<i32>().unwrap()
    // let mut negative_karma = false;
    let mut karma = String::new();
    let mut event = String::new();

    // init with text
    let mut dialog_type: DialogType = DialogType::new_text();

    // Check if a given char should be insert as text
    // allow to have text with special char (like - / # )
    let mut except = false;

    let mut header_numbers = 0;
    let mut last_header_numbers = 0;

    let mut author_phase = false;
    let mut content_phase = false;
    let mut condition_phase = false;

    let mut karma_phase = false;
    let mut event_phase = false;
    // allow to write `event:` or `ejhlksdfh:` instend of `e:`
    let mut post_colon = false;

    let mut trigger_phase = false;

    // let mut new_line = false;

    let chars = s.chars().collect::<Vec<char>>();
    // println!("{}", chars.len());
    // println!("{}", s);

    for (_, c) in chars
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx < chars.len())
    {
        // println!("c: {}", *c);

        if (content_phase && author_phase)
            || (content_phase && condition_phase)
            || (author_phase && condition_phase)
        {
            // warn!(
            //     "author phase: {}, content phase: {}, condition phase: {}",
            //     author_phase, content_phase, condition_phase
            // );
            panic!("Illegal Combinaison of phase; author phase: {}, content phase: {}, condition phase: {}",
            author_phase, content_phase, condition_phase);
        }

        // transitions

        if *c == '#' && !except {
            header_numbers += 1;
            author_phase = true;
        }
        // !condition_phase to permit negative number in the karma threshold
        else if *c == '-' && !condition_phase && !except {
            content_phase = true;
        } else if *c == '>' && content_phase && !except {
            content_phase = false;
            trigger_phase = true;
        } else if *c == '|' && content_phase && !except {
            // remove the space on the first position
            while save.starts_with(" ") {
                save.remove(0);
            }

            dialog_type = DialogType::new_choice();

            condition_phase = true;
            content_phase = !content_phase;
        } else if *c == ',' && karma_phase && condition_phase {
            // can be solved with karma_min, karma_max
            // println!("karma 1rst elem: {}", karma);
            let k: i32;
            // karma = karma.to_uppercase();
            if karma == "MAX".to_string() || karma == "max".to_string() {
                k = KARMA_MAX;
            } else if karma == "MIN".to_string() || karma == "min".to_string() {
                k = KARMA_MIN;
            } else {
                k = karma.parse::<i32>().unwrap();
            }
            condition.karma_threshold = Some((k, KARMA_MAX));

            // reuse this variable
            karma.clear();
        }
        //
        // End of Phase
        //
        else if *c == '\n' && author_phase {
            while author.starts_with(" ") {
                author.remove(0);
            }
            // println!("author: {}", author);

            // root: header_numbers != 1
            if last_header_numbers != 0 {
                // println!("previous:\n{}", current.borrow().print_file());
                // println!("last_header_numbers {}", last_header_numbers);
                // println!("header_numbers {}", header_numbers);

                // set current to the parent of the incomming node
                // if last_header_numbers - header_numbers +1 == 0;
                //     cause 0..0 = 0 iteration
                // then current = incomming node's parent
                // so skip this step
                let limit = last_header_numbers - header_numbers + 1;
                for _step in 0..limit {
                    // println!("step {}", _step);
                    // should not panic anyway
                    let parent = current.clone().borrow().parent.clone();
                    // println!(
                    //     "parent {}:\n{}",
                    //     _step,
                    //     parent.clone().unwrap().borrow().print_file()
                    // );
                    current = parent.clone().unwrap();

                    // match &current.borrow_mut().parent {
                    //     Some(parent) => current.borrow_mut() = parent.to_owned(),
                    //     None => panic!("no parent"),
                    // }
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
            }

            // TODO: give the real entity_id or remove id
            current.borrow_mut().character = Some((0, author.to_owned()));

            author.clear();

            last_header_numbers = header_numbers;
            header_numbers = 0;

            except = false;
            author_phase = false;
        } else if *c == ';' && karma_phase {
            // println!("karma: {}", karma);
            let k: i32;
            // karma = karma.to_uppercase();
            if karma == "MAX".to_string() || karma == "max".to_string() {
                k = KARMA_MAX;
            } else if karma == "MIN".to_string() || karma == "min".to_string() {
                k = KARMA_MIN;
            } else {
                k = karma.parse::<i32>().unwrap();
            }
            match condition.karma_threshold {
                // _ cause the second i32 is KARMA_MAX
                Some((x, _)) => {
                    if x <= k {
                        condition.karma_threshold = Some((x, k));
                    } else {
                        condition.karma_threshold = Some((k, x));
                    }
                }
                None => {
                    warn!("init creation condition has been skipped");
                }
            }
            karma.clear();

            karma_phase = false;
        } else if (*c == ';' || *c == ',') && event_phase {
            // println!("event : {}", event);
            let e = event.parse::<GameEvent>().unwrap();
            match condition.event {
                Some(vec) => {
                    let mut events = vec.clone();
                    events.push(e);
                    condition.event = Some(events);
                }
                None => condition.event = Some(vec![e]),
            }

            // println!("event cleared");
            event.clear();

            if *c == ';' {
                event_phase = false;
            }
        } else if *c == '\n' && condition_phase && dialog_type.is_choice()
        // && !karma_phase
        // && !event_phase
        {
            // remove the space on the first position
            while save.starts_with(" ") {
                save.remove(0);
            }

            // remove the space on the last position
            while save.ends_with(" ") {
                save.remove(save.len() - 1);
            }

            // println!("choice inserted: {}", save);

            let choice: DialogType;

            match (condition.karma_threshold, condition.clone().event) {
                (Some(_), None) | (None, Some(_)) | (Some(_), Some(_)) => {
                    choice = DialogType::Choice {
                        text: save.clone(),
                        condition: Some(condition.clone()),
                    };
                }
                // (None, None)
                _ => {
                    choice = DialogType::Choice {
                        text: save.clone(),
                        condition: None,
                    }
                }
            }

            current.borrow_mut().dialog_type.push(choice);

            condition_phase = false;

            save.clear();
            condition = DialogCondition::new();
        } else if *c == '\n'
            && content_phase
            && !save.is_empty()
            && dialog_type.is_text()
            && !except
        {
            // remove the space on the first position
            while save.starts_with(" ") {
                save.remove(0);
            }

            // remove the space on the last position
            while save.ends_with(" ") {
                save.remove(save.len() - 1);
            }

            // println!("text inserted: {}", save);

            let dialog = DialogType::Text(save.clone());
            current.borrow_mut().dialog_type.push(dialog);

            save.clear();

            content_phase = false;
        } else if (*c == ',' || *c == '\n') && trigger_phase {
            // println!("event : {}", event);
            let e = event.parse::<ThrowableEvent>().unwrap();

            // add the triggered event to the vector of the current DialogNode
            current.borrow_mut().trigger_event.push(e);

            // println!("event cleared");
            event.clear();

            if *c == '\n' {
                trigger_phase = false;
            }
        }
        // read text or condition
        else if c.is_ascii() || c.is_alphanumeric() {
            // the except char is /
            if *c == '/' {
                info!("except");
                except = true;
            }
            // `;` or `\n` put an end to the selection of condtion
            else if condition_phase {
                if *c == 'k' {
                    karma_phase = true;
                    post_colon = true;
                } else if *c == 'e' && !event_phase {
                    event_phase = true;
                    post_colon = true;
                } else if *c == ':' && (event_phase || karma_phase) {
                    post_colon = false;
                }
                // c.is_numeric() ||
                // authorize MAX or MIN input
                else if karma_phase && !post_colon && *c != ' '
                // || *c == '-'
                {
                    // negative symbol -
                    karma.push(*c);

                    println!("k: {}", karma);
                } else if event_phase && *c != ' ' && !post_colon {
                    event.push(*c);

                    // println!("e: {}", event);
                }
            } else if trigger_phase && *c != ' ' {
                event.push(*c);
            }
            // ignore the new line: "\n"
            // the \n is a marker to end some phase
            else if *c == '\n' && !except {
                // new_line = true;
                // println!("skip");

                // full reset

                dialog_type = DialogType::new_text();

                author_phase = false;
                content_phase = false;
                // useless protection cause by if condition_phase just above
                // condition_phase = false;
            } else if author_phase && *c != '#' {
                author.push(*c);
            }
            // if !is_special_char_file(*c) || (is_special_char_file(*c) && except)
            else {
                save.push(*c);
                except = false;

                // println!("add {} ", *c);
            }
        }
    }

    return root;
}

#[cfg(test)]
mod tests {

    #[allow(deprecated)]
    mod flat {
        use crate::ui::dialog_system::*;

        #[test]
        fn test_print_flat_child() {
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
                dialog.borrow().print_flat(),
                "[Hello]->[[Hey, No Hello, Want to share a flat ?]]".to_string()
            );
        }

        #[test]
        fn test_print_flat_children() {
            let dialog = Rc::new(RefCell::new(DialogNode::new()));
            dialog.borrow_mut().dialog_type = vec![DialogType::Choice {
                text: String::from("Hello"),
                condition: None,
            }];
            // The Player
            dialog.borrow_mut().character = Some((0b0000001u32, "Morgan".to_string()));

            let random_dialog = Rc::new(RefCell::new(DialogNode::new()));
            random_dialog.borrow_mut().dialog_type =
                vec![DialogType::Text(String::from("I have to tell something"))];
            // The npc
            random_dialog.borrow_mut().character = Some((0b0000010u32, "Fabien".to_string()));
            dialog.borrow_mut().add_child(random_dialog);

            let olf_no_longer_a_dj = Rc::new(RefCell::new(DialogNode::new()));
            olf_no_longer_a_dj.borrow_mut().dialog_type = vec![
                DialogType::Text(String::from("You beat Olf !")),
                DialogType::Text(String::from("Now you can chill at the hospis")),
            ];
            // The npc
            olf_no_longer_a_dj.borrow_mut().character = Some((0b0000010u32, "Fabien".to_string()));
            dialog.borrow_mut().add_child(olf_no_longer_a_dj);

            assert_eq!(
        dialog.borrow().print_flat(),
        "[Hello]->[[I have to tell something]; [You beat Olf !, Now you can chill at the hospis]]"
            .to_string()
    );
        }

        #[test]
        fn test_print_flat_complex() {
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
                dialog.borrow().print_flat(),
                "[Hello]->[[Hey, No Hello, Want to share a flat ?]->[[:)]; [:O]; [Sure]]]"
                    .to_string()
            );
        }

        #[test]
        fn test_init_tree_flat_simple_text_1() {
            let root = init_tree_flat(String::from("[Hello]"));
            assert_eq!(
                root.borrow().dialog_type,
                vec![DialogType::Text("Hello".to_string())]
            );
        }

        #[test]
        fn test_init_tree_flat_simple_text_2() {
            let root = init_tree_flat(String::from("[I want to talk]"));
            assert_eq!(
                root.borrow().dialog_type,
                vec![DialogType::Text("I want to talk".to_string())]
            );
        }

        #[test]
        fn test_init_tree_flat_simple_choice_1() {
            // carefull with
            let root = init_tree_flat(String::from("[Let's talk,I don't wanna talk]"));
            assert_eq!(
                root.borrow().dialog_type,
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
        fn test_init_tree_flat_simple_choice_spaced() {
            // carefull with
            let root = init_tree_flat(String::from("[Let's talk, I don't wanna talk]"));
            assert_eq!(
                root.borrow().dialog_type,
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
        fn test_init_tree_flat_famiglia_figlio_unico() {
            // TODO: find out what the warning referred to
            // carefull with ???
            let root = init_tree_flat(String::from(
                "[Catchphrase]->[[I love you, Give me your wallet]]",
            ));

            assert_eq!(
                root.borrow().dialog_type,
                vec![DialogType::Text("Catchphrase".to_string())]
            );

            assert_eq!(
                root.borrow().children[0].borrow().dialog_type,
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
        fn test_init_tree_flat_famiglia() {
            let root = init_tree_flat(String::from(
            "[Catchphrase]->[[I love you, Give me your wallet]->[[Me Too]; [Here all my chicken sandwich]]]",
        ));

            assert_eq!(
                root.borrow().children[0].borrow().dialog_type,
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
                root.borrow().children[0].borrow().children[0]
                    .borrow()
                    .dialog_type,
                vec![DialogType::Text("Me Too".to_string())]
            );

            assert_eq!(
                root.borrow().children[0].borrow().children[1]
                    .borrow()
                    .dialog_type,
                vec![DialogType::Text("Here all my chicken sandwich".to_string())]
            );
        }

        #[test]
        fn test_init_tree_flat_complex_famiglia() {
            // helped me see that accent wasn't include in the ascii char

            let root = init_tree_flat(String::from(
            "[Il faut absolument sauver les Fabien du Chien Géant]->[[Il me faut donc obtenir le trône...]->[[...,et de l'argent]->[[Et de l'argent]->[[C'est essentiel]]];[C'est essentiel]]]",
        ));

            // root
            assert_eq!(
                root.borrow().dialog_type,
                vec![DialogType::Text(String::from(
                    "Il faut absolument sauver les Fabien du Chien Géant"
                ))]
            );

            // first and only child of root
            assert_eq!(
                root.borrow().children[0].borrow().dialog_type,
                vec![DialogType::Text(String::from(
                    "Il me faut donc obtenir le trône..."
                ))]
            );

            // choose of the player
            assert_eq!(
                root.borrow().children[0].borrow().children[0]
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
                root.borrow().children[0].borrow().children[0]
                    .borrow()
                    .children[0]
                    .borrow()
                    .dialog_type,
                vec![DialogType::Text("Et de l'argent".to_string())]
            );
            // just after the npc said "Et de l'argent"
            assert_eq!(
                root.borrow().children[0].borrow().children[0]
                    .borrow()
                    .children[0]
                    .borrow()
                    .children[0]
                    .borrow()
                    .dialog_type,
                vec![DialogType::Text("C'est essentiel".to_string())]
            );

            // println!("{}", root.borrow().print_flat());

            // second
            assert_eq!(
                root.borrow().children[0].borrow().children[1]
                    .borrow()
                    .dialog_type,
                vec![DialogType::Text("C'est essentiel".to_string())]
            );

            // test print
            assert_eq!(
            root.borrow().print_flat(),
            "[Il faut absolument sauver les Fabien du Chien Géant]->[[Il me faut donc obtenir le trône...]->[[..., et de l'argent]->[[Et de l'argent]->[[C'est essentiel]]]; [C'est essentiel]]]".to_string()
        );
        }
    }

    mod from_file {
        use crate::ui::dialog_system::*;

        #[test]
        fn test_print_from_file() {
            let fabien = Some((0, String::from("Fabien")));
            let morgan = Some((1, String::from("Morgan")));

            let dialog = Rc::new(RefCell::new(DialogNode::new()));
            dialog.borrow_mut().dialog_type = vec![DialogType::Text(String::from("Hello"))];

            dialog.borrow_mut().character = fabien.clone();

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

            answers.borrow_mut().character = morgan;

            let dialog_2 = Rc::new(RefCell::new(DialogNode::new()));
            dialog_2.borrow_mut().dialog_type = vec![DialogType::Text(String::from(":)"))];
            dialog_2.borrow_mut().character = fabien.clone();

            let dialog_3 = Rc::new(RefCell::new(DialogNode::new()));
            dialog_3.borrow_mut().dialog_type = vec![DialogType::Text(String::from(":O"))];
            dialog_3.borrow_mut().character = fabien.clone();

            let dialog_4 = Rc::new(RefCell::new(DialogNode::new()));
            dialog_4.borrow_mut().dialog_type = vec![DialogType::Text(String::from("Sure"))];
            dialog_4.borrow_mut().character = fabien.clone();

            answers.borrow_mut().add_child(dialog_2);
            answers.borrow_mut().add_child(dialog_3);
            answers.borrow_mut().add_child(dialog_4);

            dialog.borrow_mut().add_child(answers);

            // println!("{}", dialog.borrow().print_flat());

            assert_eq!(
                dialog.borrow().print_file(),
                "# Fabien

- Hello

## Morgan

- Hey | None
- No Hello | None
- Want to share a flat ? | None

### Fabien

- :)

### Fabien

- :O

### Fabien

- Sure\n"
                    .to_string()
            );
        }

        #[test]
        fn test_print_from_file_monologue() {

            let root = init_tree_file(String::from("# Olf\n\n- Hello\n- Did you just\n- Call me ?\n- Or was it my imagination\n"));

            assert_eq!(
                root.borrow().print_file(),
                "# Olf\n\n- Hello\n- Did you just\n- Call me ?\n- Or was it my imagination\n".to_string()
            );
        }

        #[test]
        fn test_init_tree_from_file_simple_text_1() {
            let root = init_tree_file(String::from("# Olf\n\n- Hello\n"));

            assert_eq!(root.borrow().character, Some((0, String::from("Olf"))));

            assert_eq!(
                root.borrow().dialog_type,
                vec![DialogType::Text("Hello".to_string())]
            );
        }

        #[test]
        fn test_init_tree_from_file_space_overdose_1() {
            let root = init_tree_file(String::from("#            Olf\n\n-      Hello\n"));

            assert_eq!(root.borrow().character, Some((0, String::from("Olf"))));

            assert_eq!(
                root.borrow().dialog_type,
                vec![DialogType::Text("Hello".to_string())]
            );
        }

        #[test]
        fn test_init_tree_from_file_space_overdose_2() {
            let root = init_tree_file(String::from(
                "# Morgan\n\n- Hello         |   None\n- No Hello    | None\n",
            ));

            assert_eq!(root.borrow().character, Some((0, String::from("Morgan"))));

            assert_eq!(
                root.borrow().dialog_type,
                vec![
                    DialogType::Choice {
                        text: "Hello".to_string(),
                        condition: None
                    },
                    DialogType::Choice {
                        text: "No Hello".to_string(),
                        condition: None
                    }
                ]
            );
        }

        #[test]
        fn test_init_tree_from_file_space_deficiency_1() {
            let root = init_tree_file(String::from("#Olf\n\n-Hello\n"));

            assert_eq!(root.borrow().character, Some((0, String::from("Olf"))));

            assert_eq!(
                root.borrow().dialog_type,
                vec![DialogType::Text("Hello".to_string())]
            );
        }

        #[test]
        fn test_init_tree_from_file_space_deficiency_2() {
            let root = init_tree_file(String::from("# Morgan\n\n- Hello|None\n- No Hello|None\n"));

            assert_eq!(root.borrow().character, Some((0, String::from("Morgan"))));

            assert_eq!(
                root.borrow().dialog_type,
                vec![
                    DialogType::Choice {
                        text: "Hello".to_string(),
                        condition: None
                    },
                    DialogType::Choice {
                        text: "No Hello".to_string(),
                        condition: None
                    }
                ]
            );
        }

        #[test]
        fn test_init_tree_from_file_monologue_1() {
            let root = init_tree_file(String::from(
                "# Morgan\n\n- Hello\n- I was wondering\n-Alone...\n",
            ));

            assert_eq!(root.borrow().character, Some((0, String::from("Morgan"))));

            assert_eq!(
                root.borrow().dialog_type,
                vec![
                    DialogType::Text("Hello".to_string()),
                    DialogType::Text("I was wondering".to_string()),
                    DialogType::Text("Alone...".to_string())
                ]
            );
        }

        #[test]
        fn test_init_tree_from_file_simple_choice_1() {
            let root = init_tree_file(String::from(
                "# Morgan\n\n- Hello | None\n- No Hello | None\n",
            ));

            assert_eq!(root.borrow().character, Some((0, String::from("Morgan"))));

            assert_eq!(
                root.borrow().dialog_type,
                vec![
                    DialogType::Choice {
                        text: "Hello".to_string(),
                        condition: None
                    },
                    DialogType::Choice {
                        text: "No Hello".to_string(),
                        condition: None
                    }
                ]
            );
        }

        #[test]
        fn test_init_tree_from_file_complex_choice_1() {
            let root = init_tree_file(String::from(
                "# Morgan\n\n- Hello | None\n- No Hello | k: -10,0;\n",
            ));

            assert_eq!(root.borrow().character, Some((0, String::from("Morgan"))));

            assert_eq!(
                root.borrow().dialog_type,
                vec![
                    DialogType::Choice {
                        text: "Hello".to_string(),
                        condition: None
                    },
                    DialogType::Choice {
                        text: "No Hello".to_string(),
                        condition: Some(DialogCondition {
                            karma_threshold: Some((-10, 0)),
                            event: None
                        })
                    }
                ]
            );
        }

        #[test]
        fn test_init_tree_from_file_complex_choice_2() {
            let root = init_tree_file(String::from(
                "# Morgan\n\n- Hello | None\n- Mary me Hugo. | e: HasCharisma;\n",
            ));

            assert_eq!(root.borrow().character, Some((0, String::from("Morgan"))));

            assert_eq!(
                root.borrow().dialog_type,
                vec![
                    DialogType::Choice {
                        text: "Hello".to_string(),
                        condition: None
                    },
                    DialogType::Choice {
                        text: "Mary me Hugo.".to_string(),
                        condition: Some(DialogCondition {
                            karma_threshold: None,
                            event: Some(vec![GameEvent::HasCharisma])
                        })
                    }
                ]
            );
        }

        // allow to explicit/implicit karma/k; event/e
        // extend the tolerance

        #[test]
        fn test_init_tree_from_file_complex_choice_3() {
            let root = init_tree_file(String::from(
                "# Morgan\n\n- Hello | k: -50,100;\n- No Hello | karma : -100,0;\n",
            ));

            assert_eq!(root.borrow().character, Some((0, String::from("Morgan"))));

            assert_eq!(
                root.borrow().dialog_type,
                vec![
                    DialogType::Choice {
                        text: "Hello".to_string(),
                        condition: Some(DialogCondition {
                            karma_threshold: Some((-50, 100)),
                            event: None
                        })
                    },
                    DialogType::Choice {
                        text: "No Hello".to_string(),
                        condition: Some(DialogCondition {
                            karma_threshold: Some((-100, 0)),
                            event: None
                        })
                    }
                ]
            );
        }

        #[test]
        fn test_init_tree_from_file_complex_choice_4() {
            let root = init_tree_file(String::from(
                "# Morgan\n\n- Hello my Friend | e: HasFriend;\n- You droped this (*crown*) | event: HasCharisma;\n",
            ));

            assert_eq!(root.borrow().character, Some((0, String::from("Morgan"))));

            assert_eq!(
                root.borrow().dialog_type,
                vec![
                    DialogType::Choice {
                        text: "Hello my Friend".to_string(),
                        condition: Some(DialogCondition {
                            karma_threshold: None,
                            event: Some(vec![GameEvent::HasFriend])
                        })
                    },
                    DialogType::Choice {
                        text: "You droped this (*crown*)".to_string(),
                        condition: Some(DialogCondition {
                            karma_threshold: None,
                            event: Some(vec![GameEvent::HasCharisma])
                        })
                    }
                ]
            );
        }

        // allow to type MAX or MIN to select

        #[test]
        fn test_init_tree_from_file_complex_choice_karma_max_min() {
            let root = init_tree_file(String::from(
                "# Morgan\n\n- Hello | k: -10,MAX;\n- No Hello | k: MIN,0;\n",
            ));

            assert_eq!(root.borrow().character, Some((0, String::from("Morgan"))));

            assert_eq!(
                root.borrow().dialog_type,
                vec![
                    DialogType::Choice {
                        text: "Hello".to_string(),
                        condition: Some(DialogCondition {
                            karma_threshold: Some((-10, KARMA_MAX)),
                            event: None
                        })
                    },
                    DialogType::Choice {
                        text: "No Hello".to_string(),
                        condition: Some(DialogCondition {
                            karma_threshold: Some((KARMA_MIN, 0)),
                            event: None
                        })
                    }
                ]
            );
        }

        #[test]
        fn test_init_tree_from_file_simple_kinship_1() {
            let root = init_tree_file(String::from(
                "# Morgan\n\n- Hello\n## Hugo\n- Hey! How are you ?\n",
            ));

            assert_eq!(root.borrow().character, Some((0, String::from("Morgan"))));
            assert_eq!(
                root.borrow().children[0].borrow().character,
                Some((0, String::from("Hugo")))
            );

            assert_eq!(
                root.borrow().dialog_type,
                vec![DialogType::Text("Hello".to_string())]
            );
            assert_eq!(
                root.borrow().children[0].borrow().dialog_type,
                vec![DialogType::Text("Hey! How are you ?".to_string())]
            );
        }

        #[test]
        fn test_init_tree_from_file_monologue_2() {
            let root = init_tree_file(String::from("# Morgan\n\n- Hello\n- I was wondering\n\n## Morgan\n\n- With Friends ! | event: HasFriend;\n- Alone... | None\n"));

            assert_eq!(root.borrow().character, Some((0, String::from("Morgan"))));

            assert_eq!(
                root.borrow().dialog_type,
                vec![
                    DialogType::Text("Hello".to_string()),
                    DialogType::Text("I was wondering".to_string())
                ]
            );

            assert_eq!(
                root.borrow().children[0].borrow().character,
                Some((0, String::from("Morgan")))
            );

            assert_eq!(
                root.borrow().children[0].borrow().dialog_type,
                vec![
                    DialogType::Choice {
                        text: "With Friends !".to_string(),
                        condition: Some(DialogCondition {
                            karma_threshold: None,
                            event: Some(vec![GameEvent::HasFriend])
                        })
                    },
                    DialogType::Choice {
                        text: "Alone...".to_string(),
                        condition: None
                    }
                ]
            );
        }

        #[test]
        fn test_init_tree_from_file_complex_kinship_1() {
            let root = init_tree_file(String::from(
                "# Morgan\n\n- Hello | None\n- Do you want to work with me ? | None\n\n## Hugo\n\n- Hey! How are you ?\n\n## Hugo\n\n- I'm sure you'll do just fine without me.\n",
            ));

            assert_eq!(root.borrow().character, Some((0, String::from("Morgan"))));
            assert_eq!(
                root.borrow().dialog_type,
                vec![
                    DialogType::Choice {
                        text: "Hello".to_string(),
                        condition: None
                    },
                    DialogType::Choice {
                        text: "Do you want to work with me ?".to_string(),
                        condition: None
                    }
                ]
            );

            println!("{}", root.borrow().print_file());

            // By choosing the n-eme choice, you will get the result of the n-eme child.

            assert_eq!(
                root.borrow().children[0].borrow().character,
                Some((0, String::from("Hugo")))
            );
            assert_eq!(
                root.borrow().children[0].borrow().dialog_type,
                vec![DialogType::Text("Hey! How are you ?".to_string())]
            );

            assert_eq!(
                root.borrow().children[1].borrow().character,
                Some((0, String::from("Hugo")))
            );
            assert_eq!(
                root.borrow().children[1].borrow().dialog_type,
                vec![DialogType::Text(
                    "I'm sure you'll do just fine without me.".to_string()
                )]
            );
        }

        // TODO: add test for multiple throwable event `-> HasFriend, FightEvent\n`

        #[test]
        fn test_init_tree_from_file_throwable_event_1() {
            let root = init_tree_file(String::from(
                "# Morgan\n\n- Let's Talk | None\n- Let's Fight | None\n\n## Hugo\n\n- :)\n\n-> HasFriend\n\n## Hugo\n\n- :(\n\n-> FightEvent\n",
            ));

            assert_eq!(root.borrow().character, Some((0, String::from("Morgan"))));
            assert_eq!(
                root.borrow().dialog_type,
                vec![
                    DialogType::Choice {
                        text: "Let's Talk".to_string(),
                        condition: None
                    },
                    DialogType::Choice {
                        text: "Let's Fight".to_string(),
                        condition: None
                    }
                ]
            );

            println!("{}", root.borrow().print_file());

            // By choosing the n-eme choice, you will get the result of the n-eme child.

            // first child
            assert_eq!(
                root.borrow().children[0].borrow().character,
                Some((0, String::from("Hugo")))
            );
            assert_eq!(
                root.borrow().children[0].borrow().dialog_type,
                vec![DialogType::Text(":)".to_string())]
            );
            assert_eq!(
                root.borrow().children[0].borrow().trigger_event,
                vec![ThrowableEvent::HasFriend]
            );

            // second child
            assert_eq!(
                root.borrow().children[1].borrow().character,
                Some((0, String::from("Hugo")))
            );
            assert_eq!(
                root.borrow().children[1].borrow().dialog_type,
                vec![DialogType::Text(":(".to_string())]
            );
            assert_eq!(
                root.borrow().children[1].borrow().trigger_event,
                vec![ThrowableEvent::FightEvent]
            );
        }

        #[test]
        fn test_init_tree_from_file_except_1() {
            let root = init_tree_file(String::from(
                "# Morgan\n\n- Bonjour Florian. /\nComment vas/-tu :/# ? /\nJ'ai faim. /<3 /</|3\n",
            ));

            assert_eq!(root.borrow().character, Some((0, String::from("Morgan"))));

            assert_eq!(
                root.borrow().dialog_type,
                vec![DialogType::Text(
                    "Bonjour Florian. \nComment vas-tu :# ? \nJ'ai faim. <3 <|3".to_string()
                )]
            );
        }
    }

    // #[test]
    // fn test_add_child() {
    //     let root = init_tree_flat(String::from("[0,1,[3,4,5,[7,8]],2]"));
    //     let new_node = Rc::new(RefCell::new(TreeNode::new()));
    //     new_node.borrow_mut().value = Some(9);
    //     let child = &root.borrow().children[2];
    //     child.borrow_mut().add_child(new_node);
    //     assert_eq!(root.borrow().print_flat(), "[0,1,[3,4,5,[7,8],9],2]");
    // }
}
