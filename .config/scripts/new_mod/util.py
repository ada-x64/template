from os import PathLike, fspath, path
import re
from dataclasses import dataclass, asdict
from os.path import basename, join, realpath
from re import Match, Pattern
from pprint import pformat
import logging
import argparse

type Expr = Pattern[str]


@dataclass
class Args(argparse.Namespace):
    module_path: PathLike[str] | None
    remove: bool
    debug: bool


@dataclass
class Vars:
    use_expr: Expr
    mod_expr: Expr
    prelude_expr: Expr
    plugin_expr: Expr
    screens_expr: Expr
    screens_pat_expr: Expr
    screens_match_expr: Expr
    screens_exists_expr: Expr
    screens_data_mod: str
    super_mod: str
    mod_name: str
    mod_str: str
    prelude_str: str
    plugin_str: str
    screens_str: str
    screens_pat: str
    file_str: str
    is_screen: bool


def get_vars(args: Args):
    global vars, logger

    logging.basicConfig(
        level=logging.DEBUG if args.debug else logging.INFO,
        format="[%(levelname)s] %(message)s",
    )
    logger = logging.getLogger(__name__)

    if args.module_path is None:
        logger.error("Please provide a module path.\n")
        exit(1)

    mod_name = basename(args.module_path)
    mod_camel_case = mod_name.replace("_", " ").title().replace(" ", "")
    vars = Vars(
        use_expr=re.compile(r"(\s*use\s+[\w:]+;\n?)+"),
        mod_expr=re.compile(r"(\s*mod\s+\w+;\n?)+"),
        prelude_expr=re.compile(r"\s*pub mod prelude \{"),
        plugin_expr=re.compile(r"\s*pub fn plugin\(_?\w+: &mut App\) \{[^}]*"),
        screens_expr=re.compile(r"enum Screens \{[^}]*"),
        screens_exists_expr=re.compile(rf"\s+{mod_camel_case},?\s+"),
        screens_pat_expr=re.compile(rf"Screens::{mod_camel_case}\s+=>\s+\"\w+\",?"),
        screens_match_expr=re.compile(
            r"pub\s+const\s+fn\s+as_screen_type\(self\)\s+->\s+ScreenId\s+\{\n.+match self \{"
        ),
        screens_data_mod=realpath("src/screen/data.rs"),
        super_mod=realpath(join(args.module_path, "../mod.rs")),
        mod_name=mod_name,
        mod_str=f"mod {mod_name};",
        prelude_str=f"pub use super::{mod_name}::prelude::*;",
        plugin_str=f"app.add_plugins({mod_name}::plugin);",
        screens_str=f"{mod_camel_case},",
        screens_pat=f'Screens::{mod_camel_case} => "{mod_name}",',
        file_str="",
        is_screen="/screen" in fspath(args.module_path),
    )

    if args.debug:
        logger.debug(pformat(asdict(vars)))


def add_module():
    global vars
    ms = vars.mod_str.center(len(vars.mod_str) + 2)
    prs = vars.prelude_str.center(len(vars.prelude_str) + 2)
    pls = vars.plugin_str.center(len(vars.plugin_str) + 2)

    if not try_insert_after([vars.mod_expr, vars.use_expr], ms):
        insert(0, ms)

    insert_after(
        [vars.prelude_expr], prs, "No prelude found, not exposing new module's prelude"
    )
    insert_after(
        [vars.plugin_expr],
        pls,
        "Could not find plugin insertion point, please insert the plugin manually.",
    )


def remove_module():
    global vars
    replace(vars.mod_str, "")
    replace(vars.prelude_str, "")
    replace(vars.plugin_str, "")


def insert(pos: int, inserted_str: str):
    global vars
    vars.file_str = vars.file_str[:pos] + inserted_str + vars.file_str[pos:]


def read(file: str):
    global vars
    with open(file) as buf:
        vars.file_str = buf.read()


def write(file: str):
    global vars
    with open(file, "w") as buf:
        logger.info(f"Writing to {file}")
        _ = buf.write(vars.file_str)


def replace(match: str, replace: str):
    vars.file_str = vars.file_str.replace(match, replace)


def re_replace(pattern: Expr, replace: str, msg_on_err: str):
    if match := find([pattern]):
        vars.file_str = vars.file_str.replace(match.group(0), replace)
    else:
        logger.warning(msg_on_err)


def find(exprs: list[Expr]) -> Match[str] | None:
    for expr in exprs:
        if match := expr.search(vars.file_str, re.MULTILINE):
            return match
    return None


def try_insert_after(exprs: list[Expr], replace: str) -> bool:
    global vars
    if match := find(exprs):
        insert(match.end(), replace)
        return True
    return False


def insert_after(exprs: list[Expr], replace: str, msg_on_err: str):
    global vars
    if match := find(exprs):
        insert(match.end(), replace)
    else:
        logger.warning(msg_on_err)
