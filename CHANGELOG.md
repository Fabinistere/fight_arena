# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Bevy 0.10 Migration - [0.4.1] - 2023-07-10

[![v0.4.1](https://img.shields.io/badge/v0.4.1-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/figh_arena/releases/tag/v0.4.1)](https://github.com/Fabinistere/figh_arena/releases/tag/v0.4.1)

### Added

- [![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/fabinistere/bevy_trun-based_combat#license)
- the player can now move with wasd and the arrows
- Startup underun :) (with the last commit: "Minor Fixes and GameFeeling: Merge branch 'feature-web' into develop")

### Changed

- Bevy 0.10.1
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

## Dialog Update - [0.4.0] - 2023-01-17

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

## Chase Update - [0.3.0] - 2022-10-27

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

## Follow Update - [0.2.0] - 2022-09-20

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

## Stroll Update - [0.1.0] - 2022-08-20

[![v0.1.0](https://img.shields.io/badge/v0.1.0-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/figh_arena/releases/tag/v0.1.0)](https://github.com/Fabinistere/figh_arena/releases/tag/v0.1.0)

### Added

- Multiple NPC travel the room with two behavior
  - RunToDestination (walks towards a zone and switch to Rest)
  - Rest (Waits 10s -independently- and starts to walking towards a new destination)
- The timer is now attached to one specific npc
