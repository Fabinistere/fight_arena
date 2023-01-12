//! All base method involved in creating the UI ingame
//!
//! EventHandler:
//!
//! - Enter in Combat
//! - Exit in Combat
//! - Open HUD manually (pressing 'o')
//! - Scolls Gestion
//!   - Update Dialog Tree
//!   - Update each Scroll
//!   - Update Dialog Box / Text

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_tweening::{lens::UiPositionLens, *};
use std::time::Duration;

use crate::{
    combat::{CombatEvent, CombatExitEvent, Karma},
    constants::ui::dialogs::*,
    npc::NPC,
    player::Player,
    ui::{
        dialog_scroll::{
            PlayerChoice, PlayerScroll, Scroll, ScrollTimer, UpdateScrollEvent, UpperScroll,
        },
        dialog_system::{init_tree_file, Dialog, DialogType},
    },
};

/// Represents The UI Wall.
/// Every UI Wall is associated with an entity (can be the player or the interlocutor like an object).
///
/// If the dialog_tree changes, it will update the scrolls.
///
/// To modify the dialog, just modify the DialogPanel.
///
/// # Note
///
/// TODO: feature - sync author with interlocutor (to know which one is talking)
#[derive(Component, Inspectable)]
pub struct DialogPanel {
    // keep track of the origanal interlocutor
    // their dialog will be change/update in update_dialog_tree
    pub main_interlocutor: Entity,
    // XXX: will allow us to detect change especially in the opening
    pub dialog_tree: String,
}

/// Happens when
///   - ui::dialog_panel::create_dialog_panel_on_key_press
///     - press 'o' to open the UI
///   - ui::dialog_panel::create_dialog_panel_on_combat_event
///     - when CombatEvent is triggered ( and ui not open )
///     open the ui with the interlocutor gived by the CombatEvent
/// Read in
///   - ui::dialog_panel::create_dialog_panel
///     - for a given String, creates basic ui entities ( ui + fx )
pub struct CreateDialogPanelEvent {
    interlocutor: Entity,
    dialog_tree: String,
}

/// Happens when
///   - ui::dialog_panel::update_dialog_panel
///     - the dialog tree contained within the DialogPanel is empty
/// Read in
///   - ui::dialog_panel::end_node_dialog
///     - fills the given interlocutor with a blank "..." dialog
///     and exit the Combat ( send CombatExitEvent )
pub struct EndNodeDialogEvent;

/// Happens when
///   - ui::dialog_panel::create_dialog_panel_on_key_press
///     - ui already open
///   - ui::dialog_panel::create_dialog_panel_on_combat_event
///     - ui already open
///   - combat::mod::exit_combat
///     - Close the Ui when CombatExitevent triggered
/// Read in
///   - ui::dialog_panel::close_dialog_panel
///     - close ui
pub struct CloseDialogPanelEvent;

pub struct DialogPanelResources {
    text_font: Handle<Font>,
    appartements: Handle<Image>,
    stained_glass_panels: Handle<Image>,
    background: Handle<Image>,
    _stained_glass_closed: Handle<Image>,
    stained_glass_opened: Handle<Image>,
    _stained_glass_bars: Handle<Image>,
    chandelier: Handle<Image>,
    pub scroll_animation: Vec<Handle<Image>>,
}

pub fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // let scroll_texture = asset_server.load("textures/hud/scroll_animation.png");
    // let scroll_atlas = TextureAtlas::from_grid(scroll_texture, SCROLL_SIZE.into(), 1, 45);

    let mut scroll_animation_frames = vec![];
    for i in 0..SCROLL_ANIMATION_FRAMES_NUMBER {
        scroll_animation_frames
            .push(asset_server.load(&format!("textures/hud/scroll_animation/frame_{}.png", i)));
    }

    commands.insert_resource(DialogPanelResources {
        text_font: asset_server.load("fonts/dpcomic.ttf"),
        appartements: asset_server.load("textures/hud/papier_paint.png"),
        background: asset_server.load("textures/hud/dialog_background.png"),
        scroll_animation: scroll_animation_frames,
        chandelier: asset_server.load("textures/hud/chandelier.png"),
        _stained_glass_closed: asset_server.load("textures/hud/stained_glass_closed.png"),
        stained_glass_opened: asset_server.load("textures/hud/stained_glass_opened.png"),
        _stained_glass_bars: asset_server.load("textures/hud/stained_glass_bars.png"),
        stained_glass_panels: asset_server.load("textures/hud/stained_glass_panels.png"),
    });
}

/// # Note
///
/// TODO: feature - exit the personal thought or any tab when being touch by aggro
///
/// FIXME: PB Spamming the ui key 'o'; ?throws an error
pub fn create_dialog_panel_on_key_press(
    mut create_dialog_panel_event: EventWriter<CreateDialogPanelEvent>,
    mut close_dialog_panel_event: EventWriter<CloseDialogPanelEvent>,

    mut ev_combat_exit: EventWriter<CombatExitEvent>,

    query: Query<(Entity, &Animator<Style>, &Style), With<DialogPanel>>,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<(Entity, &Dialog), With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::O) {
        if let Ok((_entity, animator, _style)) = query.get_single() {
            if animator.tweenable().unwrap().progress() >= 1.0 {
                close_dialog_panel_event.send(CloseDialogPanelEvent);

                ev_combat_exit.send(CombatExitEvent);
            }
        } else {
            info!("here second");

            let (player, dialog) = player_query.single();
            // warn!("The player doesn't have a Dialog")

            let dialog_tree: String;
            match &dialog.current_node {
                Some(text) => dialog_tree = text.to_owned(),
                None => dialog_tree = String::new(),
            }

            create_dialog_panel_event.send(CreateDialogPanelEvent {
                // keep track of player's personal thoughts
                interlocutor: player,
                dialog_tree,
            });
        }
    }
}

/// # Event Handler
///
/// **Handle** the CombatEvent
///     open a new ui or // TODO: go to Discussion ui
///
/// # Behavior
///
/// Interpret the dialog carried by the entity.
///
/// In Dialog Sequence,
/// we might -want to- have the last text
/// when the player is ask to choose a answer.
///
/// For simplificity,
/// the feature: `recreate the dialog tree to include the last text in the root`
/// is deactivated.
///
/// So, when the dialog is stopped during a choice,
/// the root of the dialog tree is not modified and contains only the previous choice.
///
/// Unlucky situation :
/// having to answer something without the context.
pub fn create_dialog_panel_on_combat_event(
    mut ev_combat: EventReader<CombatEvent>,

    mut create_dialog_panel_event: EventWriter<CreateDialogPanelEvent>,
    query: Query<(Entity, &Animator<Style>, &Style), With<DialogPanel>>,

    // npc with dialog
    // cause player can only talk with theirself
    // by create_dialog_panel_on_key_press
    // not by CombatEvent
    npc_query: Query<(Entity, &Dialog), With<NPC>>,
) {
    for ev in ev_combat.iter() {
        // if already open go to combat tab
        if let Ok((_entity, _animator, _style)) = query.get_single() {
            // close any open ui
            // if animator.tweenable().unwrap().progress() >= 1.0 {
            //     close_dialog_panel_event.send(CloseDialogPanelEvent);
            // }
        } else {
            // open a new ui with the first dialog within the NPC targeted

            info!("Open UI Combat");

            let npc = ev.npc_entity;
            match npc_query.get(npc) {
                Ok((_npc_entity, dialog)) => {
                    let dialog_tree: String;
                    match &dialog.current_node {
                        Some(text) => dialog_tree = text.to_owned(),
                        None => dialog_tree = String::new(),
                    }
                    create_dialog_panel_event.send(CreateDialogPanelEvent {
                        interlocutor: npc,
                        dialog_tree,
                    });
                }

                Err(e) => {
                    // FIXME: Handle this error
                    // exit the combat and log the name of this weird entity
                    warn!(
                        "The entity {:?} in the CombatEvent is not a npc with a dialog: {:?}",
                        npc, e
                    );
                }
            }
        }
    }
}

pub fn close_dialog_panel(
    mut commands: Commands,
    mut close_dialog_panel_events: EventReader<CloseDialogPanelEvent>,
    mut query: Query<(Entity, &mut Animator<Style>, &Style), With<DialogPanel>>,
) {
    for CloseDialogPanelEvent in close_dialog_panel_events.iter() {
        info!("close dialog event");
        if let Ok((entity, mut _animator, style)) = query.get_single_mut() {
            let dialog_panel_tween = Tween::new(
                EaseFunction::QuadraticIn,
                TweeningType::Once,
                Duration::from_millis(DIALOG_PANEL_ANIMATION_TIME_MS),
                UiPositionLens {
                    start: style.position,
                    end: UiRect {
                        left: Val::Auto,
                        top: Val::Px(0.0),
                        right: Val::Px(DIALOG_PANEL_ANIMATION_OFFSET),
                        bottom: Val::Px(0.0),
                    },
                },
            )
            .with_completed_event(0);

            commands
                .entity(entity)
                .remove::<Animator<Style>>()
                .insert(Animator::new(dialog_panel_tween));
        }
    }
}

pub fn despawn_dialog_panel(
    mut commands: Commands,
    mut completed_event: EventReader<TweenCompleted>,
) {
    for TweenCompleted { entity, user_data } in completed_event.iter() {
        if *user_data == 0 {
            commands.entity(*entity).despawn_recursive();
        }
    }
}

pub fn create_dialog_panel(
    mut create_dialog_panel_events: EventReader<CreateDialogPanelEvent>,
    mut create_scroll_content: EventWriter<UpdateScrollEvent>,

    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    _texture_atlases: Res<Assets<TextureAtlas>>,
    dialog_panel_resources: Res<DialogPanelResources>,
    asset_server: Res<AssetServer>,
) {
    for CreateDialogPanelEvent {
        interlocutor,
        dialog_tree,
    } in create_dialog_panel_events.iter()
    {
        info!("open dialog event");

        let dialog_panel_tween = Tween::new(
            EaseFunction::QuadraticOut,
            TweeningType::Once,
            Duration::from_millis(DIALOG_PANEL_ANIMATION_TIME_MS),
            UiPositionLens {
                start: UiRect {
                    left: Val::Auto,
                    top: Val::Px(0.0),
                    right: Val::Px(DIALOG_PANEL_ANIMATION_OFFSET),
                    bottom: Val::Px(0.0),
                },
                end: UiRect {
                    left: Val::Auto,
                    top: Val::Px(0.0),
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
            },
        );

        let panels_tween = Tween::new(
            EaseMethod::Linear,
            TweeningType::Once,
            Duration::from_millis(1000),
            UiPositionLens {
                start: UiRect {
                    top: Val::Px(0.0),
                    ..UiRect::default()
                },
                end: UiRect {
                    top: Val::Px(-160.0),
                    ..UiRect::default()
                },
            },
        );

        commands
            .spawn_bundle(ImageBundle {
                image: dialog_panel_resources.appartements.clone().into(),
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Relative,
                    position: UiRect {
                        top: Val::Px(0.0),
                        left: Val::Auto,
                        right: Val::Px(DIALOG_PANEL_ANIMATION_OFFSET),
                        bottom: Val::Px(0.0),
                    },
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                    },
                    size: Size::new(Val::Auto, Val::Percent(100.0)),
                    aspect_ratio: Some(284.0 / 400.0),
                    ..Style::default()
                },
                ..ImageBundle::default()
            })
            .insert(Name::new("UI Wall"))
            .with_children(|parent| {
                let child_sprite_style = Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Style::default()
                };

                // panels under the wall to prevent them from sticking out of the window after being lifted.
                parent
                    .spawn_bundle(ImageBundle {
                        image: dialog_panel_resources.stained_glass_panels.clone().into(),
                        style: child_sprite_style.clone(),
                        ..ImageBundle::default()
                    })
                    .insert(Animator::new(panels_tween));

                parent.spawn_bundle(ImageBundle {
                    image: dialog_panel_resources.background.clone().into(),
                    style: child_sprite_style.clone(),
                    ..ImageBundle::default()
                });

                parent.spawn_bundle(ImageBundle {
                    image: dialog_panel_resources.stained_glass_opened.clone().into(),
                    style: child_sprite_style.clone(),
                    ..ImageBundle::default()
                });

                parent.spawn_bundle(ImageBundle {
                    image: dialog_panel_resources.chandelier.clone().into(),
                    style: child_sprite_style.clone(),
                    ..ImageBundle::default()
                });

                // Upper Scroll

                parent
                    .spawn_bundle(ImageBundle {
                        image: dialog_panel_resources.scroll_animation[0].clone().into(),
                        style: Style {
                            position_type: PositionType::Absolute,
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            justify_content: JustifyContent::FlexEnd,
                            ..Style::default()
                        },
                        ..ImageBundle::default()
                    })
                    .insert(Scroll {
                        current_frame: 0,
                        reverse: false,
                    })
                    .insert(UpperScroll {
                        // will be changed in update_dialog_panel
                        texts: vec![],
                    })
                    .insert(Name::new("Upper Scroll"))
                    .insert(ScrollTimer(Timer::from_seconds(
                        SCROLL_ANIMATION_DELTA_S,
                        false,
                    )))
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    font: dialog_panel_resources.text_font.clone(),
                                    font_size: 30.0,
                                    color: Color::BLACK,
                                },
                            )
                            .with_alignment(TextAlignment {
                                vertical: VerticalAlign::Top,
                                horizontal: HorizontalAlign::Left,
                            }),
                            style: Style {
                                flex_wrap: FlexWrap::Wrap,
                                margin: UiRect {
                                    top: Val::Percent(74.0),
                                    left: Val::Percent(24.0),
                                    ..UiRect::default()
                                },
                                max_size: Size::new(Val::Px(300.0), Val::Percent(100.0)),
                                ..Style::default()
                            },
                            ..TextBundle::default()
                        });
                    })
                    // .insert(DialogBox::new(dialog[0].clone(), DIALOG_BOX_UPDATE_DELTA_S))
                    ;

                // parent
                //     .spawn_bundle(ImageBundle {
                //         image: texture_atlases
                //             .get(dialog_panel_resources.scroll_animation.clone())
                //             .unwrap()
                //             .texture
                //             .clone_weak()
                //             .into(),
                //         style: child_sprite_style.clone(),
                //         ..ImageBundle::default()
                //     });

                // Player Scroll

                let player_scroll_img =
                    asset_server.load("textures/hud/HUD_1px_parchemin_MC_ouvert.png");

                parent
                    .spawn_bundle(ImageBundle {
                        image: player_scroll_img.clone().into(),
                        style: Style {
                            position_type: PositionType::Absolute,
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            justify_content: JustifyContent::FlexEnd,
                            ..Style::default()
                        },
                        ..ImageBundle::default()
                    })
                    .insert(Scroll {
                        current_frame: 0,
                        reverse: false,
                    })
                    .insert(PlayerScroll {
                        // will be changed in update_dialog_panel
                        choices: vec![],
                    })
                    .insert(Name::new("Player Scroll"))
                    .insert(ScrollTimer(Timer::from_seconds(
                        SCROLL_ANIMATION_DELTA_S,
                        false,
                    )))
                    .with_children(|parent| {
                        // TODO: feature - 3 PlayerChoice is enough, to have much reuse theses three in another page

                        // First potential choice
                        parent
                            .spawn_bundle(ButtonBundle {
                                style: Style {
                                    // TODO: custom size ? (text dependent)
                                    size: Size::new(Val::Px(300.0), Val::Px(30.0)),
                                    margin: UiRect::all(Val::Auto),
                                    // margin: UiRect {
                                    //     top: Val::Percent(105.0),
                                    //     left: Val::Percent(24.0),
                                    //     ..UiRect::default()
                                    // },
                                    // justify_content: JustifyContent::SpaceAround,
                                    position: UiRect {
                                        top: Val::Px(-30.0),
                                        left: Val::Px(10.0),
                                        ..UiRect::default()
                                    },
                                    ..default()
                                },
                                color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .insert(PlayerChoice(0))
                            // .insert(DialogBox::new("".to_owned(), DIALOG_BOX_UPDATE_DELTA_S))
                            .with_children(|parent| {
                                parent.spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        "",
                                        TextStyle {
                                            font: dialog_panel_resources.text_font.clone(),
                                            // TODO: Find the correct value for the choice font size
                                            font_size: 20.0,
                                            color: Color::BLACK,
                                        },
                                    )
                                    .with_alignment(
                                        TextAlignment {
                                            vertical: VerticalAlign::Top,
                                            horizontal: HorizontalAlign::Left,
                                        },
                                    ),
                                    style: Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        margin: UiRect {
                                            top: Val::Percent(100.0),
                                            left: Val::Percent(0.0),
                                            ..UiRect::default()
                                        },
                                        max_size: Size::new(Val::Px(300.0), Val::Percent(100.0)),
                                        ..Style::default()
                                    },
                                    ..TextBundle::default()
                                });
                            });

                        // Second potential choice
                        parent
                            .spawn_bundle(ButtonBundle {
                                style: Style {
                                    // TODO: custom size ? (text dependent)
                                    size: Size::new(Val::Px(300.0), Val::Px(30.0)),
                                    margin: UiRect::all(Val::Auto),
                                    // margin: UiRect {
                                    //     top: Val::Percent(125.0),
                                    //     left: Val::Percent(24.0),
                                    //     ..UiRect::default()
                                    // },
                                    // justify_content: JustifyContent::SpaceAround,
                                    position: UiRect {
                                        top: Val::Px(250.0),
                                        left: Val::Px(10.0),
                                        ..UiRect::default()
                                    },
                                    ..default()
                                },
                                color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .insert(PlayerChoice(1))
                            // .insert(DialogBox::new("".to_owned(), DIALOG_BOX_UPDATE_DELTA_S))
                            .with_children(|parent| {
                                parent.spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        "",
                                        TextStyle {
                                            font: dialog_panel_resources.text_font.clone(),
                                            // TODO: Find the correct value for the choice font size
                                            font_size: 20.0,
                                            color: Color::BLACK,
                                        },
                                    )
                                    .with_alignment(
                                        TextAlignment {
                                            vertical: VerticalAlign::Top,
                                            horizontal: HorizontalAlign::Left,
                                        },
                                    ),
                                    style: Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        margin: UiRect {
                                            top: Val::Percent(100.0),
                                            left: Val::Percent(0.0),
                                            ..UiRect::default()
                                        },
                                        max_size: Size::new(Val::Px(300.0), Val::Percent(100.0)),
                                        ..Style::default()
                                    },
                                    ..TextBundle::default()
                                });
                            });

                        // Third potential choice
                        parent
                            .spawn_bundle(ButtonBundle {
                                style: Style {
                                    // TODO: custom size ? (text dependent)
                                    size: Size::new(Val::Px(300.0), Val::Px(30.0)),
                                    margin: UiRect::all(Val::Auto),
                                    // margin: UiRect {
                                    //     top: Val::Percent(145.0),
                                    //     left: Val::Percent(24.0),
                                    //     ..UiRect::default()
                                    // },
                                    // justify_content: JustifyContent::SpaceAround,
                                    position: UiRect {
                                        top: Val::Px(530.0),
                                        left: Val::Px(10.0),
                                        ..UiRect::default()
                                    },
                                    ..default()
                                },
                                color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .insert(PlayerChoice(2))
                            // .insert(DialogBox::new("".to_owned(), DIALOG_BOX_UPDATE_DELTA_S))
                            .with_children(|parent| {
                                parent.spawn_bundle(TextBundle {
                                    text: Text::from_section(
                                        "",
                                        TextStyle {
                                            font: dialog_panel_resources.text_font.clone(),
                                            // TODO: Find the correct value for the choice font size
                                            font_size: 20.0,
                                            color: Color::BLACK,
                                        },
                                    )
                                    .with_alignment(
                                        TextAlignment {
                                            vertical: VerticalAlign::Top,
                                            horizontal: HorizontalAlign::Left,
                                        },
                                    ),
                                    style: Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        margin: UiRect {
                                            top: Val::Percent(100.0),
                                            left: Val::Percent(0.0),
                                            ..UiRect::default()
                                        },
                                        max_size: Size::new(Val::Px(300.0), Val::Percent(100.0)),
                                        ..Style::default()
                                    },
                                    ..TextBundle::default()
                                });
                            });
                    });

                // Button

                // parent
                //     .spawn_bundle(ButtonBundle {
                //         style: Style {
                //             size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                //             // center button
                //             margin: UiRect::all(Val::Auto),
                //             // horizontally center child text
                //             justify_content: JustifyContent::Center,
                //             // vertically center child text
                //             align_items: AlignItems::Center,
                //             ..default()
                //         },
                //         color: NORMAL_BUTTON.into(),
                //         ..default()
                //     })
                //     .with_children(|parent| {
                //         parent.spawn_bundle(TextBundle::from_section(
                //             "Button",
                //             TextStyle {
                //                 font: asset_server.load("fonts/dpcomic.ttf"),
                //                 font_size: 40.0,
                //                 color: Color::rgb(0.9, 0.9, 0.9),
                //             },
                //         ));
                //     });
            })
            .insert(DialogPanel {
                main_interlocutor: *interlocutor,
                dialog_tree: dialog_tree.to_owned(),
            })
            .insert(Animator::new(dialog_panel_tween));

        // check with system ordering if this event will be catch
        create_scroll_content.send(UpdateScrollEvent);
    }
}

/// # Purpose
///
/// When the dialog file implied in the talk is changed,
/// updates the scrolls' content.
///
/// # Process
///
/// check the current node from the interlocutor
///
/// - this is a text
///   - change the text from the upper_scroll
///   - clear the player_scroll (choice panel)
/// - this is a choice
///   - Player Choice
///     - update the player_scroll (implied: let the upper_scroll)
///   - NPC Choice
///     TODO: feature - NPC Choice
///     for now, the player has to choose what the npc should say..
pub fn update_dialog_panel(
    panel_query: Query<
        (Entity, &DialogPanel),
        // REFACTOR: Handle the interlocutor change in the UIPanel
        // even detect interlocutor change
        (Changed<DialogPanel>, With<Animator<Style>>),
    >,

    mut upper_scroll_query: Query<(&mut UpperScroll, Entity), With<Scroll>>,
    mut player_scroll_query: Query<(&mut PlayerScroll, Entity), With<Scroll>>,

    player_query: Query<(Entity, &Karma), With<Player>>,

    mut end_node_dialog_event: EventWriter<EndNodeDialogEvent>,
    mut update_scroll_content: EventWriter<UpdateScrollEvent>,
) {
    // REFACTOR: Never Nester Mode requested
    // DOC: Noisy Comments
    // TODO: feature - find a way to execute trigger_event somewhere

    // the panel must be open already and their dialog_tree modified
    // else:
    //   just wait for the DialogTree to change;
    //   Nothing change yet
    if let Ok((_ui_wall, panel)) = panel_query.get_single() {
        info!("DEBUG: smth changed...");

        let dialog_panel = &panel.dialog_tree;
        // DEBUG: print DialogTree
        println!("{:?}", dialog_panel);

        // check what is the current dialog node
        if dialog_panel.is_empty() {
            // info!("DEBUG Empty Dialog Tree");
            end_node_dialog_event.send(EndNodeDialogEvent);
        } else {
            let dialog_tree = init_tree_file(dialog_panel.to_owned());

            let current = &dialog_tree.borrow();

            let dialogs = &current.dialog_type;

            // throw Err(outOfBound) when dialog_type is empty (not intended)
            if dialogs.len() < 1 {
                // FIXME: handle this err
                panic!("Err: dialog_type is empty");
            }

            let (mut player_scroll, _player_scroll_entity) = player_scroll_query.single_mut();

            // check the first elem of the DialogType's Vector
            match &dialogs[0] {
                DialogType::Text(_) => {
                    let mut texts = Vec::<String>::new();
                    for dialog in dialogs.iter() {
                        match dialog {
                            DialogType::Text(text) => {
                                texts.push(text.to_owned());
                                info!("DEBUG: add text: {}", text);
                            }
                            _ => panic!("Err: DialogTree Incorrect; A texts' vector contains something else"),
                        }
                    }
                    // replace the entire upper scroll's content
                    // FIXME: ¿solved? single - if let - first opening or already open
                    let (mut upper_scroll, _upper_scroll_entity) = upper_scroll_query.single_mut();
                    upper_scroll.texts = texts;

                    // Clear the previous choice if there is any
                    player_scroll.choices.clear();
                }
                DialogType::Choice {
                    text: _,
                    condition: _,
                } => {
                    // replace current by the new set of choices
                    let mut choices = Vec::<String>::new();
                    for dialog in dialogs.iter() {
                        match dialog {
                            DialogType::Choice { text, condition } => {
                                match condition {
                                    Some(cond) => {
                                        let (_player, karma) = player_query.single();
                                        if cond.is_verified(karma.0) {
                                            choices.push(text.to_owned());
                                            info!("DEBUG: add choice: {}", text);
                                        }
                                    }
                                    // no condition
                                    None => {
                                        choices.push(text.to_owned());
                                        info!("DEBUG: add choice: {}", text);
                                    }
                                }
                            }
                            _ => panic!("Err: DialogTree Incorrect; A choices' vector contains something else"),
                        }
                    }
                    // update the player_scroll
                    player_scroll.choices = choices;
                }
            }
            // ask to update the content of scroll
            update_scroll_content.send(UpdateScrollEvent);
        }
    }
}

/// # Save principe
///
/// Updates the String within the entity interlocutor.
/// This just updates the Dialog contained in the interlocutor to be retrieve the next time we talk with it.
/// We want to save the dialog progress at each state;
/// Each time the dialog_tree of the panel is changed
/// (?OR can be delay to the end of fight)
///
/// # Note
///
/// XXX: little trick to detect change especially in the creation phase
pub fn update_dialog_tree(
    // XXX: (not an issue.) will detect change if the interlocutor is switch
    dialog_panel_query: Query<&DialogPanel, Changed<DialogPanel>>,
    mut interlocutor_query: Query<(Entity, &mut Dialog)>,
) {
    for panel in dialog_panel_query.iter() {
        let interlocutor = panel.main_interlocutor;
        let new_dialog_tree = panel.dialog_tree.clone();
        match interlocutor_query.get_mut(interlocutor) {
            Ok((_entity, mut dialog)) => dialog.current_node = Some(new_dialog_tree),
            Err(e) => warn!(
                "The entity linked with the Ui Wall doesn't have any Dialog Component: {:?}",
                e
            ),
        }
    }
}

/// Fills the given interlocutor with a blank "..." dialog
/// and exit the Combat ( send CombatExitEvent )
pub fn end_node_dialog(
    mut end_node_dialog_event: EventReader<EndNodeDialogEvent>,

    panel_query: Query<(Entity, &DialogPanel), With<Animator<Style>>>,
    mut interlocutor_query: Query<
        (Entity, &Name, &mut Dialog),
        // Player or NPC
        // TODO: feature - include Object (rm these Withs)
        Or<(With<Player>, With<NPC>)>,
    >,

    mut ev_combat_exit: EventWriter<CombatExitEvent>,
) {
    for _ in end_node_dialog_event.iter() {
        info!("DEBUG: EndNodeEvent...");

        let (_ui_wall, panel) = panel_query.single();

        // TODO: feature - manage a cast of NPC choice for each dialog
        // with a priority system to choose
        // engaging a dialog will then choose a certain dialog from the cast
        // leaving mid course will save the current dialog
        // UNLESS there is a overide
        // in case of big event, cancel previous dialog to stick to the main line

        // reset the dialog to the first node: NPC's Choice cast

        info!("exit dialog");

        let interlocutor = panel.main_interlocutor;
        let (_interlocutor_entity, name, mut dialog) =
            interlocutor_query.get_mut(interlocutor).unwrap();

        // replace the current tree by a simple text: `...`
        let display_name = name.replace("NPC ", "");

        // let blank_dialog_tree = "# name\n\n- ...\n"
        //     .replace("name", &display_name)
        //     .to_owned();

        let blank_dialog_tree = format!("# {}\n\n- ...\n", &display_name);

        // don't change panel.dialog_tree here
        // it will be detect by update_dialog_panel
        // i'm living in the fear
        // i'm in danger
        // my own program wants me dead

        // let's overide update_dialog_tree, here and now.
        dialog.current_node = Some(blank_dialog_tree);

        ev_combat_exit.send(CombatExitEvent);

        // at the next enconter there will be ... as dialog
        // prevent closing the dialog_panel instant after engaging dialog
    }
}
