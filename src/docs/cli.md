## Implementation in Mise

The command-line interface is currently being handled by
[mise](https://mise.jdx.dev). This allows us to set up our dev environment in a
reproducible way. It also gives us access to command-line scripts.

## Build deps

The environment automatically uses [sccache](https://github.com/mozilla/sccache), [mold](https://github.com/rui314/mold), and [cranelift](https://cranelift.dev) to enable fast
builds. Feel free to remove these dependencies if they cause issues for you.

### Removing fast build

You will need to delete `.cargo/config.toml` (or modify it), and you will need to delete reference to sccache in `.config/mise.toml`.

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
