from os.path import join, basename, realpath
import re
import argparse

parser = argparse.ArgumentParser(prog="update_parent_module")
parser.add_argument('module_path')
parser.add_argument('--debug', type=bool)
parser.add_argument('--remove', action='store_true')
args = parser.parse_args()

if args.module_path is None:
    print("Please provide a module path.\n")
    exit(1)

use_expr = re.compile(r'(\s*use\s+[\w:]+;\n?)+')
mod_expr = re.compile(r'(\s*mod\s+\w+;\n?)+')
prelude_expr = re.compile(r'\s*pub mod prelude {')
plugin_expr = re.compile(r'app.add_plugins\(\(')
mod_name = basename(args.module_path)

mod_str = f"mod {mod_name};"
prelude_str = f"pub use super::{mod_name}::prelude::*;"
plugin_str = f"{mod_name}::plugin,"
super_mod =realpath(join(args.module_path, "../mod.rs"))

if args.debug:
    print("DEBUG")
    print(f"{args.module_path=}\n{mod_name=}\n{super_mod=}")

def insert(str, pos, inserted_str):
    return str[:pos] + inserted_str + str[pos:]

def add_module(str):
    ms = mod_str.center(len(mod_str) + 2)
    prs = prelude_str.center(len(prelude_str) + 2)
    pls = plugin_str.center(len(plugin_str) + 2)

    if match := mod_expr.search(str, re.MULTILINE) or use_expr.search(str, re.MULTILINE):
        str = insert(str, match.end(), ms)
    else:
        str = insert(str, 0, ms)

    if match := prelude_expr.search(str, re.MULTILINE):
        str = insert(str, match.end(), prs)
    else:
        print("WARN: No prelude found, not exposing new module's prelude")

    if match := plugin_expr.search(str, re.MULTILINE):
        str = insert(str, match.end(), pls)
    else:
        print("WARN: Could not find plugin insertion point, please insert the plugin manually.")

    return str

def remove_module(str: str):
    str = str.replace(mod_str, '')
    str = str.replace(prelude_str, '')
    str = str.replace(plugin_str, '')
    return str

str = ''
with open(super_mod) as buf:
    str = buf.read()

if args.remove:
    str = remove_module(str)
else:
    str = add_module(str)

print(f"[INFO] Writing to {super_mod}")
with open(super_mod, 'w') as buf:
    buf.write(str)
