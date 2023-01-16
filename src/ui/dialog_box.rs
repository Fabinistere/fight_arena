//! Every Structs and methods only about Dialog Box

use bevy::prelude::*;

use crate::{
    constants::ui::dialogs::DIALOG_BOX_UPDATE_DELTA_S,
    ui::dialog_scroll::{PlayerChoice, UpperScroll},
};

/// Represents the entity containing the displayed text as first children.
///
/// Used to animate the Text, letter by letter.
#[derive(Debug, Component)]
pub struct DialogBox {
    pub text: String,
    progress: usize,
    finished: bool,
    update_timer: Timer,
}

impl DialogBox {
    pub fn new(text: String, update_time: f32) -> Self {
        DialogBox {
            text,
            progress: 0,
            finished: false,
            update_timer: Timer::from_seconds(update_time, TimerMode::Once),
        }
    }

    // Same as new but keep the signature
    // fn reset(&self, text: String, update_time: f32) {
    //     *self.text = text;
    //     *self.progress = 0;
    //     *self.finished = false;
    //     *self.update_timer = Timer::from_seconds(update_time, TimerMode::Once);
    // }
}

/// Happens when
///   - ui::dialog_panel::update_upper_scroll
///     - updates UpperScroll Text with the UpperScroll infos
///   - ui::dialog_panel::update_player_scroll
///     - updates PlayerScroll Text with the UpperScroll infos
///     happens for every choice there is in the PlayerScroll
/// Read in
///   - ui::dialog_panel::reset_dialog_box
///     - creates a DialogBox to transfer info to the child Text
///     if there is none
///     or resets the text and dialogBox
pub struct ResetDialogBoxEvent {
    pub dialog_box: Entity,
    /// could be
    ///
    /// - a Choice
    /// - a Text
    pub text: String,
}

/// Animates, letter by letter, each Text.
/// ( being the DialogBox's 1rt child )
pub fn update_dialog_box(
    time: Res<Time>,
    mut dialog_box_query: Query<(&mut DialogBox, &Children)>,
    mut text_query: Query<&mut Text>,
) {
    for (mut dialog_box, children) in dialog_box_query.iter_mut() {
        dialog_box.update_timer.tick(time.delta());

        if dialog_box.update_timer.finished() && !dialog_box.finished {
            // let mut text = text_query.get_mut(children[0]).unwrap();
            match text_query.get_mut(children[0]) {
                Ok(mut text) => {
                    // prompt the simple text
                    // FIXME: bug - if the given text contains a accent this will crash
                    match dialog_box.text.chars().nth(dialog_box.progress) {
                        // will ignore any louche symbol
                        // FIXME: infinite call when there is a accent
                        None => warn!("Accent Typical Crash"),
                        Some(next_letter) => {
                            text.sections[0].value.push(next_letter);

                            dialog_box.progress += 1;
                            if dialog_box.progress >= dialog_box.text.len() {
                                dialog_box.finished = true;
                            }
                        }
                    }
                }
                // FIXME: If there is no TEXT then insert one in it
                // pb: on which scroll...
                Err(e) => warn!("No Text in the Dialog Wall: {:?}", e),
            }
        }
    }
}

/// Reset DialogBox on Event
pub fn reset_dialog_box(
    mut commands: Commands,

    mut reset_event: EventReader<ResetDialogBoxEvent>,

    mut dialog_box_query: Query<
        (&mut DialogBox, &Children, Entity),
        Or<(With<PlayerChoice>, With<UpperScroll>)>,
    >,
    mut text_query: Query<&mut Text>,
) {
    for event in reset_event.iter() {
        match dialog_box_query.get_mut(event.dialog_box) {
            Err(_e) => {
                info!("DEBUG: no DialogBox in the UpperScroll");
                commands.entity(event.dialog_box).insert(DialogBox::new(
                    event.text.clone(),
                    DIALOG_BOX_UPDATE_DELTA_S,
                ));
            }
            Ok((mut dialog_box, children, _)) => {
                // FIXME: bug - Reset the text even if there is no change
                // Clear the DialogBox Child: the Text
                match text_query.get_mut(children[0]) {
                    Err(e) => warn!("No Text Section: {:?}", e),
                    Ok(mut text) => {
                        if dialog_box.text != event.text.clone() {
                            text.sections[0].value.clear();
                            // replace current DialogBox with a brand new one
                            *dialog_box =
                                DialogBox::new(event.text.clone(), DIALOG_BOX_UPDATE_DELTA_S);
                        }
                    }
                }
            }
        }
    }
}
