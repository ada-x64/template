# Screens

The [Screen] trait

## Screen scoping

First, let's go over some terminology to make sure we have a clear conceptual
understanding.

| term          | definition                                                                                                                 |
| ------------- | -------------------------------------------------------------------------------------------------------------------------- |
| Screen        | A type of world-state which can help determine behavior.                                                                   |
| Screen-scoped | This entity will be despawned when the screen changes.                                                                     |
| Persistent    | This entity will _not_ be despawned when the screen changes.                                                               |
| Propogate     | A propogating component will descend the parent/child hierarchy and clone its inner component onto its recursive children. |
| Top-level     | An entity without any parents.                                                                                             |

By default, all entities are screen-scoped. This means that, whenever the screen
changes, all entities are removed from the world. However, there are some
exceptions to this rule. First, there are a few built-in components that mark
an entity as persistent. They are listed below:

- [Window]
- [Observer] [^1]

[^1]: These components are persistent _only if_ they are top-level.

Second, there is the explicitly-marked [Persistent] component. This component
is designed to make screen-persistent interfaces, such as UI and global
settings.
