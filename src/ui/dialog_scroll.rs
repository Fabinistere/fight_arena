//! Scrolls

use bevy::prelude::*;

use crate::{
    constants::ui::dialogs::SCROLL_ANIMATION_FRAMES_NUMBER, ui::dialog_box::ResetDialogBoxEvent,
};

use super::dialog_panel::DialogPanelResources;

/// Any scroll should have this component.
///
/// Used to animate scroll.
///
/// # Note
///
/// Can't merge the PlayerScroll and UpperSrcoll int othe Scroll Component,
/// due to quering manners, and update scrolls function being to different
/// from one scroll to another.
///
/// Cause a serie of text is just a monologue and we don't care
/// about the previous text displayed.
/// All choice need to be prompted (not especially on the same page).
#[derive(Component)]
pub struct Scroll {
    pub current_frame: usize,
    pub reverse: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct ScrollTimer(pub Timer);

/// Saves every text, in order, contained in the current dialog node.
#[derive(Component, Reflect)]
pub struct UpperScroll {
    pub texts: Vec<String>,
}

/// Saves all choice we could have to display
#[derive(Component, Reflect)]
pub struct PlayerScroll {
    pub choices: Vec<String>,
}

/// Represents all button which may contain choice for the player to made
#[derive(Component)]
pub struct PlayerChoice(pub usize);

/// Happens when
///   - ui::dialog_panel::create_dialog_panel
///     - UI Wall creation
///   - ui::dialog_panel::update_dialog_panel
///     - After any change in the dialog tree
///     ( except when the d. tree is empty )
///   - ui::dialog_player::drop_first_text_upper_scroll
///     - ask to update the upper scroll after droping one text
/// Read in
///   - ui::dialog_panel::update_upper_scroll
///     - create a dialogBox with the text contained in the UpperScroll,
///     or update Text in existing dialogBox.
///   - ui::dialog_panel::update_player_scroll
///     - update choices displayed in the player scroll.
pub struct UpdateScrollEvent;

/// # Note
///
/// Waiting for the use of spritesheet in bevy ui.
/// To stop using frame by frame update.
pub fn animate_scroll(
    time: Res<Time>,
    // texture_atlases: Res<Assets<TextureAtlas>>,
    dialog_panel_resources: Res<DialogPanelResources>,
    mut commands: Commands,
    mut scroll_query: Query<
        (&mut UiImage, &mut Scroll, &mut ScrollTimer, Entity),
        (With<UpperScroll>, Without<PlayerScroll>),
    >,
) {
    for (mut image, mut scroll, mut timer, entity) in scroll_query.iter_mut() {
        timer.tick(time.delta());

        if timer.finished() {
            if scroll.reverse {
                scroll.current_frame -= 1;

                if scroll.current_frame == 0 {
                    commands.entity(entity).remove::<ScrollTimer>();
                }
            } else {
                scroll.current_frame += 1;

                if scroll.current_frame >= SCROLL_ANIMATION_FRAMES_NUMBER - 1 {
                    commands.entity(entity).remove::<ScrollTimer>();
                }
            }

            image.texture = dialog_panel_resources.scroll_animation[scroll.current_frame].clone();
        }
    }
}

/// Displays the content of the first element contained in the Upper Scroll
///
/// # Note
///
/// TODO: feature - execute animation when any update occurs; (Handle by event allow it)
/// For example, the closure opening to clear and display.
pub fn update_upper_scroll(
    mut scroll_event: EventReader<UpdateScrollEvent>,

    // mut scroll_query: Query<(Or<&PlayerScroll, &mut UpperScroll>, Entity), With<Scroll>>,
    upper_scroll_query: Query<(&UpperScroll, Entity), With<Scroll>>,

    mut reset_event: EventWriter<ResetDialogBoxEvent>,
) {
    for _ev in scroll_event.iter() {
        info!("- Upper - Scroll Event !");

        match upper_scroll_query.get_single() {
            Err(e) => warn!("{}", e),
            Ok((upper_scroll, upper_scroll_entity)) => {
                // let text = upper_scroll.texts.pop();
                // just collect the first without removing it
                match upper_scroll.texts.first() {
                    None => {
                        info!("empty upper scroll");
                        // TODO: feature - send event to close (reverse open) upper scroll ?
                    }
                    Some(dialog_box_text) => {
                        info!("upper scroll gain a text");

                        reset_event.send(ResetDialogBoxEvent {
                            dialog_box: upper_scroll_entity,
                            text: dialog_box_text.to_owned(),
                        });
                    }
                }
            }
        }
    }
}

/// Player scroll can contain multiple choice
/// that will be displayed at the same time.
///
/// For each choice, resets the DialogBox associated with its index
pub fn update_player_scroll(
    mut scroll_event: EventReader<UpdateScrollEvent>,

    player_scroll_query: Query<(&PlayerScroll, &Children, Entity), With<Scroll>>,

    mut reset_event: EventWriter<ResetDialogBoxEvent>,
) {
    for _ev in scroll_event.iter() {
        info!("- Player - Scroll Event !");

        match player_scroll_query.get_single() {
            Err(e) => warn!("{}", e),
            Ok((player_scroll, scroll_children, _player_scroll_entity)) => {
                let mut place = 0;

                // REFACTOR: every 3 choices create a page and start again from the 1st child
                for choice in &player_scroll.choices {
                    // FIXME: CRASH HERE OutOfBound if place > 3 (view the refactor above)
                    if place > 3 {
                        continue;
                        // place = place + 1;
                    }

                    // The button's visibility is based on the size
                    // of the vector: player_scroll.choices
                    reset_event.send(ResetDialogBoxEvent {
                        dialog_box: scroll_children[place],
                        text: choice.to_owned(),
                    });

                    place = place + 1;
                }
                info!("DEBUG: player scroll gain {} choice-s", place);
            }
        }
        // if no choice
        // info!("empty player scroll");
        // send event to close (reverse open) upper scroll ?
    }
}
