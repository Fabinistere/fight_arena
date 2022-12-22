//! All dialog method handler related with the player directly (input, etc)

use bevy::prelude::*;
use bevy_tweening::Animator;

use crate::{
    combat::Karma,
    constants::ui::dialogs::*,
    npc::{aggression::CombatExitEvent, NPC},
    player::Player,
};

use super::{
    dialog_box::{CloseDialogBoxEvent, DialogPanel, PlayerScroll, Scroll, UpperScroll},
    dialog_system::{init_tree_file, Dialog, DialogType},
};

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn skip_forward_dialog(
    mut upper_scroll_query: Query<(&mut UpperScroll, Entity), With<Scroll>>,
    mut player_scroll_query: Query<(&mut PlayerScroll, Entity), With<Scroll>>,
    query: Query<(Entity, &Animator<Style>, &DialogPanel)>,
    mut npc_query: Query<(Entity, &mut Dialog), With<NPC>>,
    player_query: Query<(Entity, &Karma), With<Player>>,

    keyboard_input: Res<Input<KeyCode>>,

    mut close_dialog_box_event: EventWriter<CloseDialogBoxEvent>,
    mut ev_combat_exit: EventWriter<CombatExitEvent>,
) {
    // let any = keyboard_input.any_just_pressed([0, 162]);
    // TODO: don't pop the last text if there is a choice
    // only replace when there is a new text
    // !keyboard_input.get_just_pressed().is_empty()
    if keyboard_input.just_pressed(KeyCode::P) {
        info!("P pressed");
        let (_ui_wall, animator, panel) = query.single();
        let interlocutor = panel.main_interlocutor;

        match npc_query.get_mut(interlocutor) {
            Ok((_npc_entity, mut dialog)) => {
                // check what is the next dialog node
                match &dialog.current_node {
                    Some(text) => {
                        let dialog_tree = init_tree_file(text.to_string());

                        let current = &dialog_tree.borrow();

                        let dialogs = &current.dialog_type;

                        // throw Err(outOfBound) when dialog_type is empty (not intended)
                        if dialogs.len() < 1 {
                            panic!("Err: dialog_type is empty");
                        }

                        // check the first elem of the DialogType's Vector
                        match &dialogs[0] {
                            DialogType::Text(_) => {
                                let (mut upper_scroll, _upper_scroll_entity) =
                                    upper_scroll_query.single_mut();

                                if upper_scroll.texts.len() > 1 {
                                    // pop first
                                    match upper_scroll.texts.split_first() {
                                        Some((_first, rem)) => {
                                            upper_scroll.texts = rem.to_vec();
                                        }
                                        // empty vec
                                        None => warn!("early destruction; Iteration Error"),
                                    }
                                }
                                // we are in the last text of the node
                                // let's check its child
                                else if !&dialog_tree.borrow().children.is_empty() {
                                    // a text node can have a only one child max
                                    // let child = &dialog_tree.borrow().children[0].borrow();
                                    let child_type = &current.children[0].borrow().dialog_type;

                                    match &child_type[0] {
                                        DialogType::Text(_) => {
                                            // replace current by the new text
                                            let mut texts = Vec::<String>::new();
                                            for dialog in child_type.iter() {
                                                match dialog {
                                                    DialogType::Text(text) => {
                                                        texts.push(text.to_owned())
                                                    }
                                                    _ => panic!(
                                                         "Err: DialogTree Incorrect; A texts' vector contains something else"
                                                    ),
                                                }
                                            }
                                            upper_scroll.texts = texts;

                                            // update the dialog tree contain within the main interlocutor
                                            let next_tree: String = dialog_tree.borrow()
                                                .children[0]
                                                .borrow()
                                                .print_file();
                                            dialog.current_node = Some(next_tree);
                                        }
                                        DialogType::Choice {
                                            text: _,
                                            condition: _,
                                        } => {
                                            // update the player_scroll
                                            let (mut player_scroll, _player_scroll_entity) =
                                                player_scroll_query.single_mut();
                                            // replace current by the new set of choices
                                            let mut choices = Vec::<String>::new();
                                            for dialog in child_type.iter() {
                                                match dialog {
                                                    DialogType::Choice { text, condition } => {
                                                        match condition {
                                                            Some(cond) => {
                                                                let (_player, karma) =
                                                                    player_query.single();
                                                                if cond.is_verified(karma.0) {
                                                                    choices.push(text.to_owned())
                                                                }
                                                            }
                                                            // no condition
                                                            None => choices.push(text.to_owned()),
                                                        }
                                                    }
                                                    _ => {
                                                        panic!("Err: DialogTree Incorrect; A choices' vector contains something else")
                                                    }
                                                }
                                            }
                                            player_scroll.choices = choices;

                                            // update the dialog tree contain within the main interlocutor
                                            let next_tree: String = dialog_tree.borrow()
                                                .children[0]
                                                .borrow()
                                                .print_file();
                                            dialog.current_node = Some(next_tree);
                                        }
                                    }
                                } else {
                                    // if we were in the last node of this path
                                    // exit the dialog
                                    // TODO: feature - find a way to execute trigger_event somewhere

                                    if animator.tweenable().unwrap().progress() >= 1.0 {
                                        close_dialog_box_event.send(CloseDialogBoxEvent);

                                        ev_combat_exit.send(CombatExitEvent);
                                    }
                                }
                            }
                            DialogType::Choice {
                                text: _,
                                condition: _,
                            } => {
                                // see choose_choice method
                                // or check here if any button were pressed
                            }
                        }
                    }

                    None => {
                        warn!("Interlocutor's dialog is empty")
                    }
                }
            }

            Err(e) => warn!(
                "The entity in the CombatEvent is not a npc with a dialog: {:?}",
                e
            ),
        }
    }
}
