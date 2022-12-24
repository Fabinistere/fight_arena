# Fight Arena

## A test arena where i can dev some NPC for our FTO game

### For now

NPC can

- follow the player within a range
- walk around towards random destination

Aggressive NPC can

- detect player and enemy
- chase them
- trigger CombatEvent
- wait 5s before retargeting someone

Collision works thanks to bevy_retrograde

Press o to see the grateful future


[Preview of Chase](https://user-images.githubusercontent.com/73140258/198221963-00eaaa8c-6ab9-4142-8519-d4124fc5dd82.mp4)


### For a future

- NPC will Avoid collider
  - Pathfind ?
- NPC will have some personnality
  - Implement landmark/place
  - Will have dialogue
- Combat system
  - Talk
  - Fight
    - Placement
    - UI
    - Combat Phases

    ```mermaid
    graph
        Observation-->ManageStuff;
        ManageStuff-->Observation;
        Observation-->Skills;
        Skills-->Observation;
        Skills-->Target;
        Target-->Skills;
        Target-->RollInitiative;
        RollInitiative-->Target;
        RollInitiative-->ExecuteSkills-->RollInitiative;
        ExecuteSkills-->Observation;
    ```

## Assets deported - Ecological Issue

From now on, all my repertory using musics and images changing a lot will have a particular folder in our org's cloud.
To avoid using the git storage for such maters.
Indeed store image in git means if only one pixel changed the git will save the previous and the next image completly.
Which happens to be a pure waste of energy in my case.

SO, to have the assets of the last commit, please download this folder:
[assets](https://drive.google.com/drive/folders/1jcYH7U0qzLvyE25JEkXixoA6EWw6KNN5?usp=share_link)

To find previous assets, they will be given in the given realase
