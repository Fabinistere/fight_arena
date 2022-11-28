#[cfg(test)]
mod tests {
    use dialog_system::*;

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

        assert_eq!(dialog.borrow().print_flat(), "[Hello]->[[I have to tell something]; [You beat Olf !, Now you can chill at the hospis]]".to_string());
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
            "[Hello]->[[Hey, No Hello, Want to share a flat ?]->[[:)]; [:O]; [Sure]]]".to_string()
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
        // TODO find out what the warning referred to
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

- Sure"
                .to_string()
        );
    }

     #[test]
     fn test_init_tree_from_file_simple_text_1() {
          let root = init_tree_flat(String::from("# Olf\n\n- Hello"));

          assert_eq!(
               root.borrow().character,
               Some((0, String::from("Olf")))
          );

          assert_eq!(
               root.borrow().dialog_type,
               vec![DialogType::Text("Hello".to_string())]
          );

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