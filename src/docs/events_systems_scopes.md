# Events, Sytems, and Scopes

## When should I use events, and when should I use systems?

You should prefer to use events over sytems whenever possible.

Events in bevy are push-based: they will execute a system whenever they are
called. This is extremely useful as it avoids running systems every frame,
which is unnecessary in most cases.

There are certain cases where it is impossible to avoid using systems. These are
cases where you need pull-based event handling, such as handling user input
per-frame, resolving physics every fixed update, or polling a web service on a
timer. These should definitely be systems. Note that these are all examples of
when you should use `FixedUpdate`. There are very few cases where you'll need
to use the `Update` schedule.

You can think of the `FixedUpdate` schedule as a particular kind of pull-based
Event handler, where the Event is simply a timer.

## How do I scope events and systems?

There are many cases where you only want a system to run given a certain world
state. Traditionally in Bevy we use `SystemSets` and the `run_if` function on a
schedule to handle conditional runs.

```rust
app.configure_sets(Update, MySystemSet.run_if(in_state(MyStates::Ready)));
```

## Event filtering

Generally, you'll want to keep observers on the top-level for handling of things
like user interaction and general state management. However, in some cases
you'll want to listen to events only on certain entities, for example to keep
track of status effects.

In this case, you'll want to use an entity scoped observers. Entity scoped
observers can be used to _filter_ events from the global level to specific
entities.

As an example, you could create a `MyEventListener` component, which, on
add, creates an observer which listens for `MyEvent`. Then, when you create the
event, you should have a system which queries for the `MyEventListener`
component and, on trigger, forwards the event to the entities.

TODO: This would be better as a relationship.
<https://www.christopherbiscardi.com/bevy-observer-filters>
