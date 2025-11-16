This is the TFW library crate.

## Testing patterns

There are to main ways to run tests. The first is with a manual-update style.
This is typically what you see in something like the official bevy tests, where
we want to guarantee a specific number of updates has occured before the next
assertion. As a nice side-effect, this guarantees that the test takes as little
time as possible.

```rust
#[test]
fn pattern_1() {
    let mut app = App::new();
    // add plugins etc.
    app.update();
    assert!(something);
    // repeat as needed
}
```

The second style of test requires a nondeterministic number of updates to pass
before the next assertion can be called, i.e. when the test requires I/O (such
as asset loading). These tests require some more setup.

```rust
#[test]
fn pattern_2() {
    let mut app = App::new();
    // plugins etc.
    // then set up systems
    let a = |mut commmands: Commands| {
        // do this to end the test successfully
        commands.write_message(AppExit::Success);
    }
    let b = |mut commmands: Commands| {
        // do this to end the test in failure
        error!("Something went wrong!")
        commands.write_message(AppExit::error());
    }
    app.add_systems(PostUpdate, (a,b).chain());
    assert!(app.run().is_success());
}
```

Note that in pattern 2 you _cannot_ use assertions as bevy systems run in
separate threads. Panicking will not kill the process, but only the thread.
