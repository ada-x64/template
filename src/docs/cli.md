## Implementation in Mise

_Note: The task descriptions in this document were last updated Nov 5, 2025. Use `mise [task] --help` to get the up-to-date descriptions._

The command-line interface is currently being handled by
[mise](https://mise.jdx.dev). This allows us to set up our dev environment in a
reproducible way. It also gives us access to command-line scripts. You can view all available tasks by running `mise task`.

## Build deps

The environment automatically uses [sccache](https://github.com/mozilla/sccache), [mold](https://github.com/rui314/mold), and [cranelift](https://cranelift.dev) to enable fast
builds. Feel free to remove these dependencies if they cause issues for you.

### Removing fast build

You will need to delete `.cargo/config.toml` (or modify it), and you will need to delete reference to sccache in `.config/mise.toml`.

## Building and Running

If you want to use `cargo` instead of these scripts, feel free. These are just
convenient shorthands, so we don't have to manually specify features and
configurations when developing over SSH.

The following tasks are available:

- `build` - Build the specified workspace binary. (Default: app)
- `play` - Run the specified binary. automatically syncs when SSH is enabled.

## Testing and Linting

- `test` - Runs [nextest](https://nexte.st) for the specified package. Arguments are passed directly to nextest.
- `ci` - Runs [act](https://github.com/nektos/act) for the repository, emulating CI tests. Passed arguments are sent straight to act.
- `check` - Runs Clippy and bevy_lint
- `fix` - Runs Clippy and bevy_lint with the `--fix` flag enabled.
- `check:deps` - Runs [cargo-deny](https://github.com/EmbarkStudios/cargo-deny)

## Templating scripts

Common actions, like the creation of [Screens](crate::framework::screen) and
[Services](crate::docs::architecture#services), are handled through the CLI.

### new

The `new` command creates a new screen or service.

This command is aliased to `new`, `n`, `generate`, and `g`. (`add` is built-in
to mise.)

```text
Usage: new [--debug] <template> <name>

Arguments:
  <template>  The template to generate.
    [possible values: screen, service]
  <name>
    The chosen template will be output to the src directory corresponding to the
    template, postfixed with given template name. For example, calling
    'mise new service foo/bar' will generate 'src/service/foo/bar/...'.

Flags:
  --debug  Enable debug output.
```

### delete

The `delete` command deletes a screen or service.

This command is aliased to `delete` and `d`. (`rm` is built-in to mise.)

```text
Usage: delete [--debug] <template> <name>

Arguments:
  <template>  The template to remove.
    [possible values: screen, service]
  <name>
    The chosen template will be removed from the src directory corresponding to the
    template, postfixed with given template name. For example, calling
    'mise new service foo/bar' will generate 'src/service/foo/bar/...'.

Flags:
  --debug  Enable debug output.
```

## Developing Over SSH

This project supports development over SSH. If you have a beefy GPU, you
can run something like
[Sunshine](https://github.com/LizardByte/Sunshine)/[Moonlight](https://moonlight-stream.org)
to stream your development environment. However, for most of us, the SSH setup
is convenient.

For example, I develop on a laptop client which connects to my desktop server
over SSH. When I establish a connection, the mise configuration automatically
detects it and sets `MISE_ENV=ssh`, activating the `mise.ssh.toml` file, which
overwrites a few settings, including the play task. Now, when I run `mise play`,
it will build the application, rsync it to my client laptop. In order for my laptop to recieve the file, it must connect via HTTP using the `listen` task described below.

When connecting over SSH, the `dylib` feature is disabled. This slows down builds, but allows for far faster synchronization.

Available tasks:

- `listen` - Starts the server to listen for rsync requests. This should be run _on the client machine_, i.e., on the client laptop described above. This will automatically open the sent binary file, alongside any commands sent with it.
- `play` - Overwrites the default play task; builds the application like normal, but then sends it to the client machine.
