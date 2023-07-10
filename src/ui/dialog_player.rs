//! All dialog method handler related with the player directly (input, etc)

use bevy::prelude::*;
use bevy_tweening::Animator;

use crate::{
    constants::ui::dialogs::*,
    ui::{
        dialog_panel::DialogPanel,
        dialog_scroll::{PlayerChoice, PlayerScroll, Scroll, UpdateScrollEvent, UpperScroll},
        dialog_system::init_tree_file,
    },
};

/// Happens when
///   - ui::dialog_player::button_system
///     - Choice selected
///   - ui::dialog_player::skip_forward_dialog
///     - P pressed
/// Read in
///   - ui::dialog_player::dialog_dive
///     - analyze the current node;
///     If not empty,
///       - drop until there is 1 or less text in the UpeerScroll
///       OR
///       - go down to the correct child index
pub struct DialogDiveEvent {
    pub child_index: usize,
    pub skip: bool,
}

/// Happens when
///   - ui::dialog_player::dialog_dive
///     - there is 2 or more text in the UpeerScroll
/// Read in
///   - ui::dialog_player::drop_first_text_upper_scroll
///     - drop first text from the UpperScroll
pub struct DropFirstTextUpperScroll;

/// Action for each Interaction of the button
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &PlayerChoice, &Children),
        (Changed<Interaction>, With<Button>),
    >,

    mut dialog_dive_event: EventWriter<DialogDiveEvent>,
    // mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, index, _children) in &mut interaction_query {
        // let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
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

/// When P's pressed, dive into the dialog ( to the first very child )
///
/// # Process
///
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
    // TODO: feature - if the text is not entirely prompted, skip the animation (update_dialog_box)
    // instead of skiping the text

    // REFACTOR: ? If Ui is open ? instead of testing this query ?
    // or just with ui_wall.finished: bool
    if let Ok((_ui_wall, animator)) = query.get_single() {
        // prevent skip while opening the panel
        if keyboard_input.just_pressed(KeyCode::P) && animator.tweenable().progress() < 1.0 {
            // be patient for god sake
            warn!("attempt of skip while the panel was opening");
            // TODO: feature - skip the animation ?! (i think it's already fast, so no)
        }
        // let any = keyboard_input.any_just_pressed([0, 162]);
        // !keyboard_input.get_just_pressed().is_empty()
        else if keyboard_input.just_pressed(KeyCode::P) {
            info!("DEBUG: P pressed");

            dialog_dive_event.send(DialogDiveEvent {
                child_index: 0,
                skip: true,
            });
        }
    }
    // FIXME: prevent more than one Ui Wall open at the same time
}

/// Analyze the current node;
///
/// If not empty,
/// - drop until there is 1 or less text in the UpeerScroll
/// - go down to the correct child index
///
/// # Note
///
/// Every modification of the DialogPanel's content
/// will modify the dialog contained the concerned interlocutor
///
/// DOC: Noisy comments
/// FIXME: Quit dialog issue
pub fn dialog_dive(
    mut dialog_dive_event: EventReader<DialogDiveEvent>,

    mut panel_query: Query<&mut DialogPanel, With<Animator<Style>>>,
    upper_scroll_query: Query<&mut UpperScroll, With<Scroll>>,

    mut drop_first_text_upper_scroll_event: EventWriter<DropFirstTextUpperScroll>,
    // mut trigger_event: EventWriter<TriggerEvent>,
) {
    for event in dialog_dive_event.iter() {
        info!("DEBUG: DialogDive Event");
        let mut panel = panel_query.single_mut();
        // let interlocutor = panel.main_interlocutor;

        let dialog_panel = panel.dialog_tree.clone();

        if dialog_panel.is_empty() {
            warn!("Empty DialogTree; The Interlocutor is still in dialog but has nothing to say.");

            // force the chnage detection
            panel.dialog_tree.clear();
            warn!("DEBUG:force clear dialog panel");
        } else {
            let dialog_tree = init_tree_file(dialog_panel.to_owned());
            let upper_scroll = upper_scroll_query.single();

            // option 1: if it is the very last text of the dialog
            // or
            // option 2: if the monologue is not finished (except the last text `> 1`)
            // then drop it

            // option 2: (precision)
            // if there is at least 2 elem in the upper scroll
            // XXX: after selecting a choice, this test will **normally** always be ignored
            // cause can't be in a choice phase while having text left in the UpperScroll
            // if not, the player could choose smth for 'nothing'

            if upper_scroll.texts.len() > 1 {
                drop_first_text_upper_scroll_event.send(DropFirstTextUpperScroll);
            } else if !(dialog_tree.borrow().is_choice() && event.skip) {
                // shouldn't exist : end choice (which hasn't child)
                // so, we don't test it here

                // REFACTOR: Check if the Trigger event field is not empty before sending anything
                // trigger_event.send(TriggerEvent(dialog_tree.borrow().trigger_event.clone()));

                if dialog_tree.borrow().is_end_node() {
                    // will be handle by the update_dialog_panel system
                    // as Exit the Combat

                    panel.dialog_tree.clear();
                    info!("clear dialog panel");
                } else {
                    // go down on the first child
                    // DOC: Specifics Rules link
                    // ignore the other child if there is one
                    // **the rule implied not**
                    // cause a text must have one child or none

                    let child = dialog_tree.borrow().children[event.child_index]
                        .borrow()
                        .print_file();

                    panel.dialog_tree = child;
                }
            }
        }
    }
}

/// Disables empty button,
/// (hidden == disable)
///
/// Prevents checking a index in the choices list.
pub fn hide_empty_button(
    mut button_query: Query<(Entity, &mut Visibility, &PlayerChoice), With<Button>>,

    player_scroll_query: Query<
        (Entity, &PlayerScroll, &Children),
        // if the choices field is modified
        (Changed<PlayerScroll>, With<Scroll>),
    >,
) {
    for (_, player_scroll, children) in player_scroll_query.iter() {
        for button in children.iter() {
            match button_query.get_mut(*button) {
                // FIXME: handle this error
                Err(e) => warn!("Err: A Player Scroll's child is not a button: {:?}", e),
                Ok((_, mut visibility, player_choice)) => {
                    // REFACTOR: just deref it
                    let choice_index = player_choice.0;
                    let choices = player_scroll.choices.clone();

                    *visibility = if choice_index < choices.len() {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    };

                    info!(
                        "button °{:?} visibility switch: {:?}",
                        choice_index,
                        *visibility == Visibility::Inherited
                    );
                }
            }
        }
    }
}

// /// DOC
// /// TODO: feat - Add triggerEvent (and add to the app)
// ///
// /// Options
// ///
// /// - Match the enum into handle it direclty
// /// - Match the enum into throw the correct event
// pub fn throw_trigger_event(mut trigger_event: EventReader<TriggerEvent>) {
//     for TriggerEvent(triggers) in trigger_event.iter() {
//         for event_to_exec in triggers.iter() {
//             match event_to_exec {
//                 ThrowableEvent::FightEvent => {
//                     info!("Fight Event")
//                 }
//                 ThrowableEvent::HasFriend => {
//                     info!("Has Friend Event")
//                 }
//             }
//         }
//     }
// }

/// Drops first text from the UpperScroll
pub fn drop_first_text_upper_scroll(
    mut drop_event: EventReader<DropFirstTextUpperScroll>,

    mut upper_scroll_query: Query<(&mut UpperScroll, Entity), With<Scroll>>,

    // update the DialogBox according to the Scroll
    mut scroll_event: EventWriter<UpdateScrollEvent>,
) {
    for _event in drop_event.iter() {
        // TOTEST: calling this event mean the scroll do exist but maybe not ?
        let (mut upper_scroll, _upper_scroll_entity) = upper_scroll_query.single_mut();

        if let Some((_first, rem)) = upper_scroll.texts.split_first() {
            // pop first only
            upper_scroll.texts = rem.to_vec();

            // ask to update the content of scroll (which will update the DialogBox)
            scroll_event.send(UpdateScrollEvent);
        } else {
            // shouldn't be the case
            warn!("The UpperScroll does not contain text")
        }
    }
}
