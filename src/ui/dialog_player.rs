//! All dialog method handler related with the player directly (input, etc)

use bevy::prelude::*;
use bevy_tweening::Animator;

use crate::constants::ui::dialogs::*;

use super::{
    dialog_box::{DialogPanel, Scroll, UpperScroll},
    dialog_system::{init_tree_file, Dialog},
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
    // mut player_scroll_query: Query<(&mut PlayerScroll, Entity), With<Scroll>>,
    query: Query<(Entity, &Animator<Style>, &DialogPanel)>,
    
    mut interlocutor_query: Query<(Entity, &mut Dialog)>, //, With<NPC>

    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok((_ui_wall, animator, panel)) = query.get_single() {

        // let any = keyboard_input.any_just_pressed([0, 162]);
        // !keyboard_input.get_just_pressed().is_empty()
        if keyboard_input.just_pressed(KeyCode::P) {
            info!("// DEBUG: P pressed");

            let interlocutor = panel.main_interlocutor;
 
            // prevent skip while opening the panel (be patient for god sake)
            if animator.tweenable().unwrap().progress() < 1.0 {
                // be patient for god sake
                warn!("attempt of skip while the panel was opening");
                // TODO: skip the animation ?! (i think it's already fast, so no)
            }

            // check the upper scroll content
            // if len > 1
            //   pop the first elem from the vector texts
            // else (if) only one text remain or none
            //   replace the current of the interlocutor by its child (of the current)

            match interlocutor_query.get_mut(interlocutor) {
                Err(e) => {
                    warn!("no interloctor with NPC and Dialog: {:?}", e);
                }
                Ok((_interlocutor, mut dialog)) => {
                    // check what is the current dialog node
                    match &dialog.current_node {
                        None => warn!("Empty DialogTree; The Interlocutor is still in dialog but has nothing to say."),
                        Some(texts) => {
                            let (mut upper_scroll, _upper_scroll_entity) =
                                upper_scroll_query.single_mut();


                            // REFACTOR: don't pop the upper_scroll.texts but call a event to change theDialogBox 
                            // still in monologue ?
                            if upper_scroll.texts.len() > 1 {
                                // if there is at least 2 elem in the upper scroll
                                if let Some((_first, rem)) = upper_scroll.texts.split_first() {
                                    // pop first only
                                    upper_scroll.texts = rem.to_vec();
                                }
                                else {
                                    info!("upper scroll is empty");
                                }
                            }
                            else {
                                let dialog_tree = init_tree_file(texts.to_owned());

                                // XXX: Care about skip choice here
                                // a test to check if the panel is on Choice Phase ?
                                // must cancel the skip possibility while still in choice phase

                                if dialog_tree.borrow().is_end_node() {
                                    // will be handle by the update_dialog_panel system
                                    // as Exit the Combat
                                    dialog.current_node = None; 
                                } else if !dialog_tree.borrow().is_choice() {
                                    // go down on the first child
                                    // ignore the other child if there is one
                                    // **the rule implied not**
                                    // cause a text must have one child or none

                                    let child = dialog_tree.borrow()
                                        .children[0].borrow()
                                        .print_file();

                                    dialog.current_node = Some(child);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
