## When should I use events, and when should I use systems?

You should use events whenever possible.

Events in bevy are push-based: they will execute a system whenever they are
called. This is extremely useful as it avoids running systems when unnecessary.
There are certain cases where it is impossible to avoid using systems. These are
cases where you need pull-based event handling, such as handling user input
per-frame, resolving physics every fixed update, or polling a web service on a
timer. These should definitely be systems.

## How do I scope certain events and systems?

Observers can be scoped to entities. Instead of calling `app.add_observer`, prefer to add observers to a screen entity.

## Screen scoping

Screens
