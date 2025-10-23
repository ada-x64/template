import argparse
from util import get_vars, add_module, remove_module
import util

# parse args
parser = argparse.ArgumentParser(prog="update_parent_module")
parser.add_argument("module_path")
parser.add_argument("--debug", type=bool)
parser.add_argument("--remove", action="store_true")
args = parser.parse_args()

get_vars(args)
vars = util.vars

# modify super module
util.read(vars.super_mod)
if args.remove:
    remove_module()
else:
    add_module()
util.write(vars.super_mod)

# update screen enum
if vars.is_screen:
    util.read(vars.screens_data_mod)
    if args.remove:
        util.re_replace(
            vars.screens_exists_expr,
            "",
            "Could not find screen name in Screens enum. Please remove the screen manually, if it exists.",
        )
    else:
        util.insert_after(
            [vars.screens_expr],
            vars.screens_str,
            "Could not find Screens enum. Please register the screen manually.",
        )
    util.write(vars.screens_data_mod)
