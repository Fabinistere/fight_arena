//! All dialog method handler related with the player directly (input, etc)

use bevy::prelude::*;
use bevy_tweening::Animator;

use crate::constants::ui::dialogs::*;

use super::{
    dialog_box::{DialogPanel, PlayerChoice, Scroll, UpperScroll, DialogBox},
    dialog_system::{init_tree_file, Dialog},
};

pub struct DialogDiveEvent {
    pub child_index: usize,
    pub skip: bool,
}

pub struct DropFirstTextUpperScroll;
// No need of : (# upper_scroll <= 1)
// {
//     pub upper_scroll: UpperScroll,
// }

// TODO: Button shoudl have a activate bool ?

/// DOC
/// 
/// FIXME: Crash when clicking a ghost button
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &PlayerChoice, &Children),
        (Changed<Interaction>, With<Button>),
    >,

    mut dialog_dive_event: EventWriter<DialogDiveEvent>,
    // mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, index, _children) in &mut interaction_query {
        // let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                // if let Ok((_ui_wall, animator, panel)) = query.get_single() {

                dialog_dive_event.send(DialogDiveEvent {
                    child_index: index.0,
                    skip: false,
                });

                // text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

// FIXME: URGE The DialogTree does not seem to be respected
// It seems it just a question of referesh

/// check the upper scroll content
/// 
/// Only skip text
///
/// - if len > 1
///   - pop the first elem from the vector texts
/// - else (if) only one text remain or none
///   - replace the current dialog node of the panel by its child (of the current)
pub fn skip_forward_dialog(
    query: Query<(Entity, &Animator<Style>), With<DialogPanel>>,
    keyboard_input: Res<Input<KeyCode>>,

    mut dialog_dive_event: EventWriter<DialogDiveEvent>,
) {
    // REFACTOR: ? If Ui is open ? instead of testing this query ?
    // or just with ui_wall.finished: bool
    if let Ok((_ui_wall, animator)) = query.get_single() {
        // prevent skip while opening the panel
        if keyboard_input.just_pressed(KeyCode::P) && animator.tweenable().unwrap().progress() < 1.0
        {
            // be patient for god sake
            warn!("attempt of skip while the panel was opening");
            // TODO: feature - skip the animation ?! (i think it's already fast, so no)
        }
        // let any = keyboard_input.any_just_pressed([0, 162]);
        // !keyboard_input.get_just_pressed().is_empty()
        else if keyboard_input.just_pressed(KeyCode::P) {
            info!("DEBUG: P pressed");

            dialog_dive_event.send(DialogDiveEvent { child_index: 0, skip: true});
        }
    }
    // FIXME: prevent more than one Ui Wall open at the same time
}

/// DOC
/// 
/// Go Down
/// 
/// Every modification of the DialogPanel's content will modify the dialog contained the concerned interlocutor
pub fn dialog_dive(
    mut dialog_dive_event: EventReader<DialogDiveEvent>,

    query: Query<&DialogPanel, With<Animator<Style>>>,
    mut interlocutor_query: Query<(Entity, &mut Dialog)>,

    upper_scroll_query: Query<&mut UpperScroll, With<Scroll>>,
    mut drop_first_text_upper_scroll_event: EventWriter<DropFirstTextUpperScroll>,
) {
    // DOC: Noisy comments

    for event in dialog_dive_event.iter() {
        let panel = query.single();
        let interlocutor = panel.main_interlocutor;
    
        match interlocutor_query.get_mut(interlocutor) {
            Err(e) => {
                warn!("no interloctor with NPC and Dialog: {:?}", e);
            }
            Ok((_interlocutor, mut dialog)) => {
                // check what is the current dialog node
                match &dialog.current_node {
                    None => warn!(
                        "Empty DialogTree; The Interlocutor is still in dialog but has nothing to say."
                    ),
                    Some(texts) => {
                        let upper_scroll = upper_scroll_query.single();
    
                        // if there is at least 2 elem in the upper scroll
                        // XXX: after selecting a choice, this test will **normally** always be ignored
                        // cause can't be in a choice phase while having text left in the UpperScroll
                        // if not, the player could choose smth for 'nothing'
                        if upper_scroll.texts.len() > 1 {
                            drop_first_text_upper_scroll_event.send(DropFirstTextUpperScroll);
                        } else {
                            // REFACTOR: seek help for this if (condition)
                            // if !(event.skip && dialog_tree.borrow().is_choice()) {}
                            let dialog_tree = init_tree_file(texts.to_owned());

                            // must cancel the skip possibility while still in choice phase
                            if dialog_tree.borrow().is_end_node() && !(dialog_tree.borrow().is_choice() && event.skip) {
                                // will be handle by the update_dialog_panel system
                                // as Exit the Combat
                                dialog.current_node = None;
                            } else if !(dialog_tree.borrow().is_choice() && event.skip){
                                // go down on the first child
                                // DOC: Specifics Rules link
                                // ignore the other child if there is one
                                // **the rule implied not**
                                // cause a text must have one child or none
    
                                let child = dialog_tree.borrow().children[event.child_index].borrow().print_file();
    
                                dialog.current_node = Some(child);
                            }
                        }
                    }
                }
            }
        }
    }   
}

/// DOC
pub fn drop_first_text_upper_scroll(
    mut drop_event: EventReader<DropFirstTextUpperScroll>,
    mut upper_scroll_query: Query<(&mut UpperScroll, Entity), With<Scroll>>,
) {
    for _event in drop_event.iter() {
        // TOTEST: calling this event mean the scroll do exist but maybe not ?
        let (mut upper_scroll, _upper_scroll_entity) = upper_scroll_query.single_mut();

        if let Some((_first, rem)) = upper_scroll.texts.split_first() {
            // pop first only
            upper_scroll.texts = rem.to_vec();
        } else {
            // shouldn't be the case
            warn!("The UpperScroll does not contain text")
        }
    }
}

/// DOC
///
/// Disable empty button (invisible == disable)
pub fn hide_empty_button(
    mut button_query: Query<
        (Entity, &mut Visibility, &DialogBox),
        //FIXME: infinite chnage occurs to the DialogBox (choice)
        (Or<(Added<DialogBox>, Changed<DialogBox>)>, With<Button>, With<PlayerChoice>),
    >,
){
    // for (_button, mut visibility, dialog_box) in button_query.iter_mut() {
    //     info!("visibility switch");
    //     visibility.is_visible = !dialog_box.text.is_empty();
    // }
}
