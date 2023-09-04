//! Complete Example of a three Dialog.
//!
//! - Press any key to continue the dialog.
//! - Choose your answer with the down right buttons.
//! - You can press the reset button to ... to reset.
//! - Click on one of the three frog portrait above.
//!
//! Also, on't worry about the timer. It's the lore.
//! Press r to reset it but it won't be on the `ShortcutLess`.

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    render::texture::ImagePlugin,
    time::Stopwatch,
    window::WindowResolution,
    winit::WinitSettings,
};
use rand::seq::SliceRandom;
use std::{collections::BTreeMap, fmt, str::FromStr};

use yml_dialog::{Content, DialogNode};

// dark purple #25131a = 39/255, 19/255, 26/255
const CLEAR: bevy::render::color::Color = bevy::render::color::Color::rgb(0.153, 0.07, 0.102);
const FIXED_TIME_STEP: f32 = 1.0 / 60.0;

const HEIGHT: f32 = 720.0;
const RESOLUTION: f32 = 16.0 / 9.0;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

/// Say, for whatever reason, we want to speedrun this example (bigup to Juju <3)
#[derive(Resource, Debug, Reflect, Deref, DerefMut, Clone, Default)]
struct SpeedrunTimer(Stopwatch);

/// Points to the Speedrun Timer Visualizer
#[derive(Component)]
struct SpeedrunTimerText;

/// Points to the current entity, if they exist, who we're talking with.
/// Query this entity to get the current Dialog.
#[derive(Debug, Reflect, Deref, DerefMut, Clone, Default, Resource)]
struct CurrentInterlocutor {
    interlocutor: Option<Entity>,
}

/// Points to the current entity, if they exist, who we're talking with.
/// Query this entity to get the current Dialog.
#[derive(Debug, Deref, DerefMut, Clone, Default, Resource)]
struct ActiveWorldEvents {
    active_world_events: Vec<WorldEvent>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum WorldEvent {
    FrogLove,
    FrogHate,
    FrogTalk,
    SpeedrunEnd,
}

impl fmt::Display for WorldEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WorldEvent::FrogLove => write!(f, "FrogLove"),
            WorldEvent::FrogHate => write!(f, "FrogHate"),
            WorldEvent::FrogTalk => write!(f, "FrogTalk"),
            WorldEvent::SpeedrunEnd => write!(f, "SpeedrunEnd"),
        }
    }
}

impl FromStr for WorldEvent {
    type Err = ();

    fn from_str(input: &str) -> Result<WorldEvent, Self::Err> {
        match input {
            "FrogTalk" => Ok(WorldEvent::FrogTalk),
            "FrogLove" => Ok(WorldEvent::FrogLove),
            "FrogHate" => Ok(WorldEvent::FrogHate),
            "SpeedrunEnd" => Ok(WorldEvent::SpeedrunEnd),
            _ => Err(()),
        }
    }
}

/// - `key`: interlocutor
/// - `value`: (current state, BinaryTreeMap of the dialog)
#[derive(Debug, Deref, DerefMut, Default, Resource)]
struct DialogMap(BTreeMap<Entity, (usize, BTreeMap<usize, DialogNode>)>);

/// Contains all the line of the current monolog
///
/// Help us keep the `DialogMap` unchanged
#[derive(Debug, Reflect, Clone, Default, Resource)]
struct Monolog {
    source: String,
    texts: Vec<String>,
}

/// Points to a interactable portrait.
///
/// REFACTOR: remove all the `Dialog` Component
#[derive(Component)]
struct Portrait;

/// Points to the NPC portrait on the dialog Panel.
#[derive(Component)]
struct InterlocutorPortait;

/// Contains the state number of the choice: `exit_state` and its position in the ui.
#[derive(Debug, Reflect, PartialEq, Eq, PartialOrd, Ord, Clone, Default, Component)]
struct ButtonChoice {
    exit_state: usize,
    ui_posiiton: usize,
}

impl ButtonChoice {
    fn new(ui_posiiton: usize) -> Self {
        ButtonChoice {
            exit_state: usize::default(),
            ui_posiiton,
        }
    }
}

#[derive(Component)]
struct Reset;

#[derive(Component)]
struct PlayerPanel;

#[derive(Component)]
struct NPCPanel;

// TODO: Visual - DialogPanel Seperator + background

fn main() {
    let mut app = App::new();
    app.insert_resource(FixedTime::new_from_secs(FIXED_TIME_STEP))
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(Msaa::Off)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::game())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(HEIGHT * RESOLUTION, HEIGHT),
                        title: "Complete Dialog".to_string(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(CurrentInterlocutor::default())
        .insert_resource(ActiveWorldEvents::default())
        .insert_resource(DialogMap::default())
        .insert_resource(Monolog::default())
        .insert_resource(SpeedrunTimer::default())
        .add_event::<ChangeStateEvent>()
        .add_event::<TriggerEvents>()
        .add_startup_systems((setup, spawn_camera))
        .add_systems((
            continue_monolog,
            choose_answer,
            reset_system,
            switch_dialog,
            change_dialog_state,
            update_dialog_panel,
            update_monolog,
            trigger_event_handler.after(change_dialog_state),
            change_interlocutor_portrait,
            button_system,
            // button_visibility,
            update_speedrun_timer.run_if(speedrun_still_on),
        ));

    app.run();
}

fn reset_system(
    mut active_world_events: ResMut<ActiveWorldEvents>,
    mut dialogs: ResMut<DialogMap>,
    mut speedrun_timer: ResMut<SpeedrunTimer>,
    keys: Res<Input<KeyCode>>,
    interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<Reset>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    if let Ok((interaction, children)) = interaction_query.get_single() {
        match interaction {
            Interaction::Clicked => {
                for (_key, (current_state, dialog)) in dialogs.iter_mut() {
                    if let Some(lower_state) = dialog.first_entry() {
                        *current_state = *lower_state.key()
                    }
                }
                active_world_events.clear();
                speedrun_timer.reset();
            }
            Interaction::Hovered => {
                let mut text = text_query.get_mut(children[0]).unwrap();
                text.sections[0].value = "Press r".to_string();
            }
            Interaction::None => {
                let mut text = text_query.get_mut(children[0]).unwrap();
                text.sections[0].value = "Reset".to_string();
            }
        }
    }

    if keys.just_pressed(KeyCode::R) {
        for (_key, (current_state, dialog)) in dialogs.iter_mut() {
            if let Some(lower_state) = dialog.first_entry() {
                *current_state = *lower_state.key()
            }
        }
        active_world_events.clear();
        speedrun_timer.reset();
    }
}

fn switch_dialog(
    mut interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Portrait>)>,
    mut current_interlocutor: ResMut<CurrentInterlocutor>,
    mut current_monolog: ResMut<Monolog>,
) {
    for (portrait, interaction) in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            // info!("Switch Interlocutor");
            current_interlocutor.interlocutor = Some(portrait);
            current_monolog.texts.clear();
            current_monolog.source.clear();
        }
    }
}

fn choose_answer(
    choice_query: Query<(&ButtonChoice, &Interaction), Changed<Interaction>>,
    mut change_state_event: EventWriter<ChangeStateEvent>,
) {
    for (button_infos, interaction) in &choice_query {
        if *interaction == Interaction::Clicked {
            change_state_event.send(ChangeStateEvent(button_infos.exit_state))
        }
    }
}

/// Happens when
///   - `continue_monolog()`
///     - any key pressed in a monolog
///
/// Read in
///   - `change_dialog_state()`
///     - analyze the current node;
///     If the state asked is a `Content::Choice`
///     without any choice verified it won't transit to the new state.
///     Else transit and throw all trigger events,
///     while leaving the `current_node`.
struct ChangeStateEvent(usize);

/// Happens when
///   - `dialog_dive()`
///     - when leaving a node
///
/// Read in
///   - `trigger_event_handler()`
///     - If the event is not already active
///     add it to the WorldEvent list.
struct TriggerEvents(Vec<String>);

fn trigger_event_handler(
    mut trigger_event: EventReader<TriggerEvents>,
    mut active_world_events: ResMut<ActiveWorldEvents>,
) {
    for TriggerEvents(incomming_events) in trigger_event.iter() {
        for event_to_trigger in incomming_events {
            match WorldEvent::from_str(event_to_trigger) {
                Err(_) => {}
                Ok(event) => {
                    if !active_world_events.contains(&event) {
                        active_world_events.push(event)
                    }
                }
            }
        }
    }
}

fn continue_monolog(
    mut key_evr: EventReader<KeyboardInput>,
    mut current_monolog: ResMut<Monolog>,
    current_interlocutor: Res<CurrentInterlocutor>,
    dialogs: Res<DialogMap>,

    mut change_state_event: EventWriter<ChangeStateEvent>,
) {
    for ev in key_evr.iter() {
        if ev.key_code == Some(KeyCode::R) {
            return;
        }
        if ev.state == ButtonState::Pressed {
            if current_monolog.texts.len() > 1 {
                if let Some((_first, rem)) = current_monolog.texts.split_first() {
                    current_monolog.texts = rem.to_vec();
                }
            } else {
                match current_interlocutor.interlocutor {
                    None => {}
                    Some(interlocutor) => {
                        if let Some(&(current_state, ref dialog)) = dialogs.get(&interlocutor) {
                            if let Some(current_node) = dialog.get(&current_state) {
                                match current_node.content() {
                                    Content::Choices(_) => {}
                                    Content::Monolog {
                                        text: _,
                                        exit_state,
                                    } => change_state_event.send(ChangeStateEvent(*exit_state)),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Analyze the current node;
///
/// If the state asked is a `Content::Choice` without any choice verified
/// don't transit to the new state.
/// Else transit and throw all trigger events.
fn change_dialog_state(
    mut change_state_event: EventReader<ChangeStateEvent>,
    current_interlocutor: Res<CurrentInterlocutor>,
    mut dialogs: ResMut<DialogMap>,
    active_world_events: Res<ActiveWorldEvents>,

    mut trigger_event: EventWriter<TriggerEvents>,
) {
    for ChangeStateEvent(new_state) in change_state_event.iter() {
        match current_interlocutor.interlocutor {
            None => {}
            Some(interlocutor) => {
                if let Some((current_state, ref dialog)) = dialogs.get_mut(&interlocutor) {
                    if let Some(current_node) = dialog.get(new_state) {
                        let new_state_is_available = match current_node.content() {
                            Content::Choices(choices) => {
                                let mut at_least_one_is_verified = false;
                                for choice in choices {
                                    if choice.is_verified(
                                        None,
                                        active_world_events
                                            .iter()
                                            .map(|x| x.to_string())
                                            .collect::<Vec<String>>(),
                                    ) {
                                        // transit if at least on verified
                                        at_least_one_is_verified = true;
                                        break;
                                    }
                                }
                                at_least_one_is_verified
                            }
                            Content::Monolog { .. } => true,
                        };

                        if new_state_is_available {
                            *current_state = *new_state;
                            trigger_event
                                .send(TriggerEvents(current_node.trigger_event().to_vec()));
                        }
                    }
                }
            }
        }
    }
}

/// If the resource `Monolog` is changed,
/// update the NPC/Player text.
fn update_monolog(
    current_monolog: Res<Monolog>,

    mut npc_panel_query: Query<&mut Text, (With<NPCPanel>, Without<PlayerPanel>)>,
    mut player_panel_query: Query<&mut Text, (With<PlayerPanel>, Without<NPCPanel>)>,
) {
    if current_monolog.is_changed() {
        match current_monolog.texts.first() {
            None => {
                let mut player_text = player_panel_query.single_mut();
                let mut npc_text = npc_panel_query.single_mut();
                player_text.sections[0].value.clear();
                npc_text.sections[0].value.clear();
            }
            Some(first) => {
                if current_monolog.source == *"Player" {
                    let mut player_text = player_panel_query.single_mut();
                    player_text.sections[0].value = first.to_string();
                } else {
                    let mut npc_text = npc_panel_query.single_mut();
                    npc_text.sections[0].value = first.to_string();
                }
            }
        }
    }
}

/// # Purpose
///
/// When the dialog file implied in the talk is changed,
/// updates the panels' content.
///
/// # Process
///
/// check the current node from the interlocutor
/// - this is a monolog
///   - change the resource monolog
/// - this is a set of choices
///   - Player Choice
///     - display only the verified choice to the button choice
///   - NPC Choice
///     - Randomly choose without display anything and ask to change state instantly
fn update_dialog_panel(
    current_interlocutor: Res<CurrentInterlocutor>,
    active_world_events: Res<ActiveWorldEvents>,
    dialogs: Res<DialogMap>,

    mut current_monolog: ResMut<Monolog>,
    mut npc_panel_query: Query<&mut Text, (With<NPCPanel>, Without<PlayerPanel>)>,
    mut player_panel_query: Query<&mut Text, (With<PlayerPanel>, Without<NPCPanel>)>,
    mut player_choices_query: Query<(&mut ButtonChoice, &mut Visibility, &Children)>,
    mut text_query: Query<&mut Text, (Without<PlayerPanel>, Without<NPCPanel>)>,

    mut change_state_event: EventWriter<ChangeStateEvent>,
) {
    if !current_interlocutor.is_none()
        && (current_interlocutor.is_changed() || dialogs.is_changed())
    {
        // info!("UpdateDialogPanel");
        match current_interlocutor.interlocutor {
            // assert_eq!(!current_interlocutor.is_none(), current_interlocutor.interlocutor == None)
            None => warn!("Logic Crack: current_interlocutor is None while being not None"),
            Some(interlocutor) => {
                if let Some(&(current_state, ref dialog)) = dialogs.get(&interlocutor) {
                    // info!("current_state: {}", current_state);
                    let mut player_text = player_panel_query.single_mut();
                    let mut npc_text = npc_panel_query.single_mut();

                    match dialog.get(&current_state) {
                        None => {
                            npc_text.sections[0].value.clear();
                            player_text.sections[0].value.clear();
                            for (_, mut visibility, _) in &mut player_choices_query {
                                *visibility = Visibility::Hidden;
                            }
                        }
                        Some(current_node) => {
                            match current_node.content() {
                                Content::Monolog {
                                    text,
                                    exit_state: _,
                                } => {
                                    if current_node.source() == &"Player".to_string() {
                                        current_monolog.texts = text.clone();
                                        current_monolog.source = current_node.source().to_string();
                                    } else {
                                        current_monolog.texts = text.clone();
                                        current_monolog.source = current_node.source().to_string();

                                        // Clear the previous choice if there is any
                                        for (_, mut visibility, _) in &mut player_choices_query {
                                            *visibility = Visibility::Hidden;
                                        }
                                        // Cler the player Part
                                        player_text.sections[0].value.clear();
                                    }
                                }
                                Content::Choices(choices) => {
                                    if current_node.source() == &"Player".to_string() {
                                        // replace current by the new set of choices
                                        let mut verified_choices = Vec::<(usize, String)>::new();

                                        for choice in choices.iter() {
                                            if choice.is_verified(
                                                None,
                                                active_world_events
                                                    .iter()
                                                    .map(|x| x.to_string())
                                                    .collect::<Vec<String>>(),
                                            ) {
                                                // info!(
                                                //     "{} -> {}",
                                                //     choice.text().to_owned(),
                                                //     *choice.exit_state()
                                                // );
                                                verified_choices.push((
                                                    *choice.exit_state(),
                                                    choice.text().to_owned(),
                                                ));
                                            }
                                        }

                                        player_text.sections[0].value.clear();

                                        for (mut button_infos, mut visibility, children) in
                                            &mut player_choices_query
                                        {
                                            // Here you could compare the index with `dialogs.len()` to incorpore all choice but
                                            // lock the unsatisfied choice's condition
                                            if button_infos.ui_posiiton < verified_choices.len() {
                                                let mut text =
                                                    text_query.get_mut(children[0]).unwrap();
                                                text.sections[0].value = verified_choices
                                                    [button_infos.ui_posiiton]
                                                    .1
                                                    .clone();
                                                button_infos.exit_state =
                                                    verified_choices[button_infos.ui_posiiton].0;
                                                *visibility = Visibility::Inherited;
                                            } else {
                                                *visibility = Visibility::Hidden;
                                            }
                                        }

                                        // Remove all text which aren't said by the current interlocutor
                                        if current_interlocutor.is_changed() {
                                            npc_text.sections[0].value.clear();
                                        }
                                    } else {
                                        // NPC Choices
                                        let mut possible_choices_index: Vec<usize> = Vec::new();
                                        for choice in choices.iter() {
                                            match choice.condition() {
                                                None => possible_choices_index
                                                    .push(*choice.exit_state()),
                                                Some(condition) => {
                                                    if condition.is_verified(
                                                        None,
                                                        active_world_events
                                                            .iter()
                                                            .map(|x| x.to_string())
                                                            .collect::<Vec<String>>(),
                                                    ) {
                                                        possible_choices_index
                                                            .push(*choice.exit_state());
                                                    }
                                                }
                                            }
                                        }
                                        if let Some(child_index) =
                                            possible_choices_index.choose(&mut rand::thread_rng())
                                        {
                                            change_state_event.send(ChangeStateEvent(*child_index))
                                        } else {
                                            // TODO: if `possible_choices_index.is_empty()`
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Put the right portrait in the left panel
fn change_interlocutor_portrait(
    current_interlocutor: Res<CurrentInterlocutor>,
    mut portrait_panel_query: Query<&mut UiImage, With<InterlocutorPortait>>,
    portraits_query: Query<&UiImage, (With<Portrait>, Without<InterlocutorPortait>)>,
    asset_server: Res<AssetServer>,
) {
    if current_interlocutor.is_changed() {
        let mut portrait = portrait_panel_query.single_mut();
        portrait.texture = match current_interlocutor.interlocutor {
            None => asset_server.load("textures/character/background.png"),
            Some(interlocutor) => {
                let new_portrait = portraits_query.get(interlocutor).unwrap();
                new_portrait.texture.clone()
            }
        };
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => *color = PRESSED_BUTTON.into(),
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.1;

    commands.spawn(camera);
}

const OLD_FROG_DIALOG: &str = "1:
  source: Old Frog
  content:
    text:
      - KeroKero
      - I want you to talk with the last Frog. All the way.
    exit_state: 2
2:
  source: Player
  content:
    - text: Done ?
      condition:
        events:
            - FrogTalk
      exit_state: 3
3:
  source: Old Frog
  content:
    text:
      - You have my respect.
      - Press Reset or alt+f4.
    exit_state: 4
  trigger_event:
      - SpeedrunEnd\n";

const FROG_DIALOG: &str = "1:
  source: Frog
  content:
    - text: KeroKero
      condition: null
      exit_state: 2
    - text: Crôaa
      condition: null
      exit_state: 3
    - text: Bêêh
      condition: null
      exit_state: 4
2:
  source: Frog
  content:
    text:
      - KeroKero
    exit_state: 5
3:
  source: Frog
  content:
    text:
      - Crôaa
    exit_state: 5
4:
  source: Frog
  content:
    text:
      - Bêêh
    exit_state: 5
5:
  source: Player
  content:
    text:
      - I wanted to say you something
    exit_state: 6
6:
  source: Player
  content:
  - text: You = Cool
    condition: null
    exit_state: 7
  - text: You = Not Cool
    condition: null
    exit_state: 8
7:
  source: Frog
  content:
    text:
      - Big love on you <3
    exit_state: 9
  trigger_event:
    - FrogLove

8:
  source: Frog
  content:
    text:
      - I'm sad now.
    exit_state: 9
  trigger_event:
    - FrogHate\n";

const WARRIOR_DIALOG: &str = "1:
  source: Warrior Frog
  content:
    text:
      - Hey
      - I mean... KeroKero
      - Can you bring my love to my homegirl the Frog in the Middle ?
    exit_state: 2
2:
  source: Player
  content:
    - text: Oh Jeez I messed up
      condition:
        events:
            - FrogHate
      exit_state: 3
    - text: The Frog is in love
      condition:
        events:
            - FrogLove
      exit_state: 4
3:
  source: Warrior Frog
  content:
    text:
      - :0
    exit_state: 5
  trigger_event:
    - FrogTalk
4:
  source: Warrior Frog
  content:
    text:
      - <3
    exit_state: 5
  trigger_event:
    - FrogTalk\n";

fn update_speedrun_timer(
    time: Res<Time>,
    mut speedrun_timer: ResMut<SpeedrunTimer>,
    mut timer_visualizer: Query<&mut Text, With<SpeedrunTimerText>>,
) {
    let mut text = timer_visualizer.single_mut();
    text.sections[0].value = speedrun_timer.elapsed_secs().to_string();
    speedrun_timer.tick(time.delta());
}

fn speedrun_still_on(active_world_events: Res<ActiveWorldEvents>) -> bool {
    !active_world_events.contains(&WorldEvent::SpeedrunEnd)
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut dialogs: ResMut<DialogMap>) {
    /* -------------------------------------------------------------------------- */
    /*                                  Portraits                                 */
    /* -------------------------------------------------------------------------- */

    let old_frog_portrait = commands
        .spawn((
            ImageBundle {
                image: UiImage {
                    texture: asset_server.load("textures/character/Icons_12.png"),
                    flip_x: true,
                    ..default()
                },
                style: Style {
                    size: Size::width(Val::Percent(10.)),
                    ..default()
                },
                ..default()
            },
            Name::new("Old Frog Portrait"),
            Portrait,
            Interaction::default(),
        ))
        .id();
    // DOC: unwrap use
    let old_frog_deserialized_map: BTreeMap<usize, DialogNode> =
        serde_yaml::from_str(OLD_FROG_DIALOG).unwrap();
    dialogs.insert(
        old_frog_portrait,
        (
            *old_frog_deserialized_map.first_key_value().unwrap().0,
            old_frog_deserialized_map,
        ),
    );

    let frog_portrait = commands
        .spawn((
            ImageBundle {
                image: UiImage {
                    texture: asset_server.load("textures/character/Icons_23.png"),
                    flip_x: true,
                    ..default()
                },
                style: Style {
                    size: Size::width(Val::Percent(10.)),
                    ..default()
                },
                ..default()
            },
            Name::new("Frog Portrait"),
            Portrait,
            Interaction::default(),
        ))
        .id();
    // DOC: unwrap use
    let frog_deserialized_map: BTreeMap<usize, DialogNode> =
        serde_yaml::from_str(FROG_DIALOG).unwrap();
    dialogs.insert(
        frog_portrait,
        (
            *frog_deserialized_map.first_key_value().unwrap().0,
            frog_deserialized_map,
        ),
    );

    let warrior_frog_portrait = commands
        .spawn((
            ImageBundle {
                image: UiImage {
                    texture: asset_server.load("textures/character/Icons_27.png"),
                    flip_x: true,
                    ..default()
                },
                style: Style {
                    size: Size::width(Val::Percent(10.)),
                    ..default()
                },
                ..default()
            },
            Name::new("Warrior Frog Portrait"),
            Interaction::default(),
            Portrait,
        ))
        .id();
    // DOC: unwrap use
    let warrior_frog_deserialized_map: BTreeMap<usize, DialogNode> =
        serde_yaml::from_str(WARRIOR_DIALOG).unwrap();
    dialogs.insert(
        warrior_frog_portrait,
        (
            *warrior_frog_deserialized_map.first_key_value().unwrap().0,
            warrior_frog_deserialized_map,
        ),
    );

    /* -------------------------------------------------------------------------- */
    /*                                    Scene                                   */
    /* -------------------------------------------------------------------------- */

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::all(Val::Percent(100.)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Name::new("Scene"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            size: Size::height(Val::Percent(50.)),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Higher Part"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(150.), Val::Px(65.)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            Name::new(format!("Reset Button")),
                            Reset,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Reset",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(150.), Val::Px(65.)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new(format!("Speedrun Timer Node")),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ),
                                SpeedrunTimerText,
                                Name::new(format!("Speedrun Timer Visualizer")),
                            ));
                        });

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    size: Size::height(Val::Percent(70.)),
                                    flex_direction: FlexDirection::Row,
                                    // horizontally center child portrait
                                    justify_content: JustifyContent::Center,
                                    // vertically center child portrait
                                    align_items: AlignItems::Center,
                                    gap: Size::width(Val::Percent(5.)),
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Interlocutor Choices"),
                        ))
                        .push_children(&[old_frog_portrait, frog_portrait, warrior_frog_portrait]);
                });

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            size: Size::height(Val::Percent(50.)),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Dialog Section"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    size: Size::width(Val::Percent(15.)),
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Interlocutor Portrait NPC"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                ImageBundle {
                                    image: UiImage {
                                        texture: asset_server
                                            .load("textures/character/background.png"),
                                        flip_x: true,
                                        ..default()
                                    },
                                    style: Style {
                                        // size: Size::all(Val::Px(50.)),
                                        size: Size::width(Val::Percent(100.)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new("Portrait"),
                                InterlocutorPortait,
                            ));
                        });

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    size: Size::width(Val::Percent(70.)),
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Dialog Panel"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        "",
                                        TextStyle {
                                            // TODO: Bevy 0.11 default font
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 30.,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_alignment(TextAlignment::Left),
                                    style: Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        // TODO: Text Style
                                        size: Size::width(Val::Percent(50.)),
                                        align_content: AlignContent::SpaceAround,
                                        align_self: AlignSelf::FlexStart,
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new("Dialog NPC"),
                                NPCPanel,
                            ));

                            parent
                                .spawn((
                                    TextBundle {
                                        text: Text::from_section(
                                            "",
                                            TextStyle {
                                                // TODO: Bevy 0.11 default font
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 30.,
                                                color: Color::WHITE,
                                            },
                                        )
                                        .with_alignment(TextAlignment::Left),
                                        style: Style {
                                            size: Size::width(Val::Percent(50.)),
                                            flex_direction: FlexDirection::Column,
                                            align_content: AlignContent::SpaceAround,
                                            // horizontally center child choices
                                            justify_content: JustifyContent::Center,
                                            // vertically center child choices
                                            align_items: AlignItems::Center,
                                            flex_wrap: FlexWrap::Wrap,
                                            // TODO: Text Style
                                            margin: UiRect {
                                                left: Val::Percent(24.),
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Name::new("Dialog Player"),
                                    PlayerPanel,
                                ))
                                .with_children(|parent| {
                                    for i in 0..3 {
                                        parent
                                            .spawn((
                                                ButtonBundle {
                                                    style: Style {
                                                        size: Size::new(
                                                            Val::Px(150.),
                                                            Val::Px(65.),
                                                        ),
                                                        // horizontally center child text
                                                        justify_content: JustifyContent::Center,
                                                        // vertically center child text
                                                        align_items: AlignItems::Center,
                                                        ..default()
                                                    },
                                                    background_color: NORMAL_BUTTON.into(),
                                                    visibility: Visibility::Hidden,
                                                    ..default()
                                                },
                                                Name::new(format!("Choice n°{i}")),
                                                ButtonChoice::new(i),
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn(TextBundle::from_section(
                                                    "",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: 30.,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ));
                                            });
                                    }
                                });
                        });

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    size: Size::width(Val::Percent(15.)),
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("Interlocutor Portrait Player"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                ImageBundle {
                                    image: asset_server
                                        .load("textures/character/Icons_05.png")
                                        .into(),
                                    style: Style {
                                        // size: Size::all(Val::Px(50.)),
                                        size: Size::width(Val::Percent(100.)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new("Portrait"),
                            ));
                        });
                });
        });
}
