# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Bevy 0.11 Migration - [v0.4.2](https://github.com/Fabinistere/figh_arena/releases/tag/v0.4.2) - 2023-08-19

[![v0.4.2](https://img.shields.io/badge/v0.4.2-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/figh_arena/releases/tag/v0.4.2)](https://github.com/Fabinistere/figh_arena/releases/tag/v0.4.2)
[![**Full Commits History**](https://img.shields.io/badge/GitHubLog-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/bevy_turn-based_combat/commits/v0.4.2)](https://github.com/Fabinistere/bevy_turn-based_combat/commits/v0.4.2)

- [Migration Guide Bevy 0.10 -> 0.11](https://bevyengine.org/learn/migration-guides/0.10-0.11/)
- *not needed* [Changelog Bevy Rapier 0.21 -> 0.22](https://github.com/dimforge/bevy_rapier/blob/master/CHANGELOG.md#0220-10-july-2023)

### Changed

- ECS
  - `in_set(OnUpdate(*))` -> `run_if(in_state(*))`
  - Add the `#[derive(Event)]` macro for events.
  - Allow tuples and single plugins in `add_plugins`, deprecate `add_plugin`
  - [Schedule-First: the new and improved `add_systems`](https://bevyengine.org/learn/migration-guides/0.10-0.11/#schedule-first-the-new-and-improved-add-systems)
- UI
  - Flatten UI Style properties that use Size + remove Size
    - The `size`, `min_size`, `max_size`, and `gap` properties have been replaced by the `width`, `height`, `min_width`, `min_height`, `max_width`, `max_height`, `row_gap`, and `column_gap` properties. Use the new properties instead.
  - [Remove `Val::Undefinded`](https://bevyengine.org/learn/migration-guides/0.10-0.11/#remove-val-undefined)
    - `Val::Undefined` has been removed. Bevy UI’s behaviour with default values should remain the same.
    The default values of `UiRect`’s fields have been changed to `Val::Px(0.)`.
    `Style`’s position field has been removed. Its `left`, `right`, `top` and `bottom` fields have been added to `Style` directly.
    For the `size`, `margin`, `border`, and `padding` fields of `Style`, `Val::Undefined` should be replaced with `Val::Px(0.)`.
    For the `min_size`, `max_size`, `left`, `right`, `top` and `bottom` fields of `Style`, `Val::Undefined` should be replaced with `Val::Auto`
  - `Interaction::Clicked` replaced by `Interaction::Pressed`
  <!-- - TODO: The Y axe's inverted once again ! -->
- Dependencies
  - bevy_rapier_2d `0.22`
  - bevy_tweening `0.8`
  - bevy-inspector-egui `main tracking`

### Note

- I did this migration twice :0
Always check your branch tree !!

## Bevy 0.10 Migration - [v0.4.1](https://github.com/Fabinistere/figh_arena/releases/tag/v0.4.1) - 2023-07-10

- [Migration Guide Bevy 0.9 -> 0.10](https://bevyengine.org/learn/migration-guides/0.9-0.10/)
- [Changelog Bevy Rapier 0.20 -> 0.21](https://github.com/dimforge/bevy_rapier/blob/master/CHANGELOG.md#0210--07-march-2023)

### Added

- [![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Fabinistere/fight_arena#license)
- the player can now move with wasd and the arrows
- Startup underun :) (with the last commit: "Minor Fixes and GameFeeling: Merge branch 'feature-web' into develop")

### Changed

- Bevy [0.10.1](https://bevyengine.org/learn/migration-guides/0.9-0.10/)
  - Bevy 0.9 migration artefact
    - change UI Coordinates to topLeft (multipliate all x coordinates by -1)
  - bevy_rapier2d [0.21](https://github.com/dimforge/bevy_rapier/blob/master/CHANGELOG.md#0210--07-march-2023)
    - feature `debug-render` change to `debug-render-2d`
  - bevy_inspector
    - `Inspector` -> `Reflect`
    And `register_inspector` to `register_type`
  - Visibility change
  - ECS
    - `add_systems()` accepts tuples
    - No More Stage -> `.in_base_set(CoreSet::T)`
    - No more systemSet -> `in_set()`
    - `Label` -> `SystemSet`
    - `FreeSystemSet` no longer support raw `str`
  - bevy_ui
    - `UIImage` its now compose of the field `texture`
  - Windows as entities
- the npcs spawn after the map, with z=2: They are below.
- DarkGrey for inactive buttons is not good -> now transparent

### Removed

- remove no longer working (in add_systems tuple) methods
  - `ui::dialog_player::throw_trigger_event()`

## Dialog Update - [v0.4.0](https://github.com/Fabinistere/figh_arena/releases/tag/v0.4.0) - 2023-01-17

[![v0.4.0](https://img.shields.io/badge/v0.4.0-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/figh_arena/releases/tag/v0.4.0)](https://github.com/Fabinistere/figh_arena/releases/tag/v0.4.0)

### Preview

[Dialog Rush](https://user-images.githubusercontent.com/73140258/212979807-92f376d4-a974-4827-88af-2687e725bc3b.mp4)

### Added

- Interpeter of Dialog File into a full Dialog Tree
  - in dialog_system;
- Display the current state of the dialog with a certain entity
- The player can interact
  - Can **action** (P) continue to read simple text and monologue
  - Display all player choices in the player scroll
  - Can select the wanted choice and really discuss (read the interactive dialog)

### Changed

- Bevy 0.9
- Pillar Spawn method
  - a simple loop

### Fixed

- Spam `o` key no longer creates multiple UI

## Chase Update - [0.3.0](https://github.com/Fabinistere/figh_arena/releases/tag/v0.3.0) - 2022-10-27

[![v0.3.0](https://img.shields.io/badge/v0.3.0-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/figh_arena/releases/tag/v0.3.0)](https://github.com/Fabinistere/figh_arena/releases/tag/v0.3.0)

### Preview

[Preview of Chase](https://user-images.githubusercontent.com/73140258/198221963-00eaaa8c-6ab9-4142-8519-d4124fc5dd82.mp4)

### Added

- The NPC can now Chase an enemy.
  - Sensor detection
    - Detection Sensor
      - If the player or Any NPC which isn't in the same team
      is detected, start the hunt.
    - Pursuit Sensor
      - if leaving this sensor, the npc stops the chase.
      And waits the evasion time (5s) before retargetting anybody.
- Hitbox with TesselatedCollider
  - Map
- Dialog UI imported from [FTO official repertory](https://github.com/Elzapat/fabien-et-la-trahison-de-olf).
- Documentation on all Event.

### Changed

- New Character Spritesheet
- Map assets
- Systems now communicates with one to another by Event
  - No more querying abuse
- System Ordering by labels

## Follow Update - [0.2.0](https://github.com/Fabinistere/figh_arena/releases/tag/v0.2.0) - 2022-09-20

[![v0.2.0](https://img.shields.io/badge/v0.2.0-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/figh_arena/releases/tag/v0.2.0)](https://github.com/Fabinistere/figh_arena/releases/tag/v0.2.0)

### Preview

[Preview of the Follow Update](https://user-images.githubusercontent.com/73140258/191371097-67efe5e6-5cec-4b2e-99e2-70eff91ff2dd.mp4)

### Added

- Follow Behavior
  - Run to the target untli they are too close
- Hitbox with TesselatedCollider
  - Column
  - Character

### Changed

- Bevy 0.8
- New map (v3.9.6)

## Stroll Update - [0.1.0](https://github.com/Fabinistere/figh_arena/releases/tag/v0.1.0) - 2022-08-20

[![v0.1.0](https://img.shields.io/badge/v0.1.0-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/figh_arena/releases/tag/v0.1.0)](https://github.com/Fabinistere/figh_arena/releases/tag/v0.1.0)

### Added

- Multiple NPC travel the room with two behavior
  - RunToDestination (walks towards a zone and switch to Rest)
  - Rest (Waits 10s -independently- and starts to walking towards a new destination)
- The timer is now attached to one specific npc
