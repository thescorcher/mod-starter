"""Create a new game-mod project from this starter.

Asks for anything not given on the command line, copies the template into a new folder, fills in
the placeholders, sets up the loader config and runs `git init`. Nothing is committed.

    python new_project.py
    python new_project.py --name "Keyblade Ring" --host "Elden Ring" --guest "Kingdom Hearts II Final Mix" \
        --route 7 --loader me3 --out ../keyblade-ring

Standard library only.
"""

import argparse
import datetime
import re
import shutil
import subprocess
import sys
from pathlib import Path

TEMPLATE = Path(__file__).resolve().parent

# Routes from ai-modding-guides guide 11.
ROUTES = {
    "1": ("1. Live passthrough",
          "Both real games run at once and exchange state (positions, collision, hits, events). "
          "The bridge needs a loader in each game; `host-bridge` is the host side.",
          "UNDECIDED: pick one game to own movement (SkyCraft: the guest owns it, the host supplies collision)"),
    "2": ("2. Frame compositing",
          "Both real games run; the guest's rendered image is pasted into the host with a camera pose.",
          "Host owns movement; the guest follows the host's pose"),
    "3": ("3. Native geometry transfer",
          "Both real games run; the host draws the guest's meshes and collision itself.",
          "UNDECIDED"),
    "4": ("4. Shared neutral simulation",
          "A separate server owns the simulation; each game is only a viewer.",
          "The neutral simulation server"),
    "5": ("5. Engine recreation",
          "One rebuilt engine reads the player's own game data; nothing crosses between programs. "
          "`guest-core` grows into the engine and `host-bridge` is not needed.",
          "Not applicable (one program)"),
    "6": ("6. Asset or map conversion",
          "Only the host runs; guest assets are converted offline from the player's own copy. "
          "`guest-data` becomes the converter and `guest-core` may not be needed.",
          "Host (unchanged)"),
    "7": ("7. Rebuilt guest engine or mechanic inside a real host",
          "The real host runs with a rebuilt version of the guest's mechanics plugged in. "
          "No guest-game code runs.",
          "Host owns position and collision; `guest-core` owns the velocity and state it requests"),
}

LOADERS = ("me3", "asi", "other")

TEXT_SUFFIXES = {".md", ".toml", ".rs", ".me3", ".txt", ""}
SKIP = {".git", "target", "local", "references", "starter", "new_project.py", "STARTER.md"}


def ask(prompt, default=None):
    suffix = f" [{default}]" if default else ""
    while True:
        value = input(f"{prompt}{suffix}: ").strip()
        if value:
            return value
        if default is not None:
            return default


def slugify(name):
    return re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-") or "mod-project"


def gather(args):
    v = {}
    v["PROJECT_NAME"] = args.name or ask("Project name")
    v["HOST_GAME"] = args.host or ask("Host game (the game that runs and that you play in)")
    v["HOST_PATH"] = args.host_path or ask("Host install folder", "UNKNOWN: fill in")
    v["HOST_SAVES"] = args.host_saves or ask("Host save folder", "UNKNOWN: fill in")
    v["GUEST_GAME"] = args.guest or ask("Guest game (the game whose gameplay you bring in)")
    v["GUEST_PATH"] = args.guest_path or ask("Guest install folder", "NOT INSTALLED YET")

    if not args.route:
        print("\nRoutes (ai-modding-guides guide 11):")
        for key, (title, _, _) in ROUTES.items():
            print(f"  {key}  {title[3:]}")
    route = args.route or ask("Route number", "7")
    if route not in ROUTES:
        sys.exit(f"unknown route {route!r}; pick 1-7")
    v["ROUTE"], v["ROUTE_NOTE"], default_owner = ROUTES[route]

    loader = args.loader or ask(f"Loader ({', '.join(LOADERS)})", "asi")
    if loader not in LOADERS:
        sys.exit(f"unknown loader {loader!r}; pick one of {', '.join(LOADERS)}")
    v["LOADER_KIND"] = loader
    if loader == "me3":
        v["LOADER"] = "me3"
        v["ME3_GAME"] = args.me3_game or ask("me3 game id (eldenring, nightreign, sekiro, darksouls3, armoredcore6)", "eldenring")
        v["LOADER_NOTE"] = ("The host is launched through me3, which starts it without Easy Anti-Cheat. "
                            "Never go online with the mod loaded.")
    elif loader == "asi":
        v["LOADER"] = "Ultimate ASI Loader"
        v["LOADER_NOTE"] = "The mod is only ever used offline."
    else:
        v["LOADER"] = args.loader_name or ask("Loader name (e.g. SKSE, BepInEx, UE4SS, Fabric)")
        v["LOADER_NOTE"] = "The mod is only ever used offline."

    v["PLAYER_OWNER"] = args.player_owner or default_owner
    v["SUMMARY"] = args.summary or (
        f"Bring {v['GUEST_GAME']} gameplay into {v['HOST_GAME']}, reading guest data from the "
        f"player's own install.")
    v["PROJECT_SLUG"] = slugify(v["PROJECT_NAME"])
    v["DATE"] = datetime.date.today().isoformat()
    return v


def fill(text, values):
    for key, value in values.items():
        text = text.replace("{{" + key + "}}", value)
    return text


def copy_template(dest, values):
    for src in TEMPLATE.rglob("*"):
        rel = src.relative_to(TEMPLATE)
        if rel.parts[0] in SKIP or src.is_dir():
            continue
        out = dest / rel
        out.parent.mkdir(parents=True, exist_ok=True)
        if src.suffix in TEXT_SUFFIXES or src.name.startswith("."):
            out.write_text(fill(src.read_text(encoding="utf-8"), values), encoding="utf-8", newline="\n")
        else:
            shutil.copy2(src, out)


def write_loader(dest, values):
    loaders = TEMPLATE / "starter" / "loaders"
    kind = values["LOADER_KIND"]
    loader_dir = dest / "loader"
    loader_dir.mkdir(exist_ok=True)
    doc = loaders / f"{kind}.md"
    (loader_dir / "README.md").write_text(fill(doc.read_text(encoding="utf-8"), values),
                                          encoding="utf-8", newline="\n")
    if kind == "me3":
        profile = fill((loaders / "me3.me3").read_text(encoding="utf-8"), values)
        (loader_dir / f"{values['PROJECT_SLUG']}.me3").write_text(profile, encoding="utf-8", newline="\n")


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--name")
    p.add_argument("--host")
    p.add_argument("--host-path")
    p.add_argument("--host-saves")
    p.add_argument("--guest")
    p.add_argument("--guest-path")
    p.add_argument("--route", choices=sorted(ROUTES))
    p.add_argument("--loader", choices=LOADERS)
    p.add_argument("--loader-name", help="name of the loader when --loader other")
    p.add_argument("--me3-game")
    p.add_argument("--player-owner")
    p.add_argument("--summary")
    p.add_argument("--out", help="new project folder (default: next to this starter, named after the project)")
    p.add_argument("--no-git", action="store_true")
    args = p.parse_args()

    values = gather(args)
    dest = Path(args.out) if args.out else TEMPLATE.parent / values["PROJECT_SLUG"]
    dest = dest.resolve()
    if dest.exists() and any(dest.iterdir()):
        sys.exit(f"{dest} already exists and is not empty")
    dest.mkdir(parents=True, exist_ok=True)

    copy_template(dest, values)
    write_loader(dest, values)
    for sub in ("guest", "re", "logs", "save-backups"):
        (dest / "local" / sub).mkdir(parents=True, exist_ok=True)

    guides = TEMPLATE / "references" / "ai-modding-guides"
    if guides.is_dir():
        shutil.copytree(guides, dest / "references" / "ai-modding-guides")
    else:
        print("note: references/ai-modding-guides not found in the starter; download it into the new "
              "project's references/ folder.")

    leftovers = sorted({m for f in dest.rglob("*") if f.is_file() and "references" not in f.parts
                        and f.suffix in TEXT_SUFFIXES
                        for m in re.findall(r"\{\{[A-Z0-9_]+\}\}", f.read_text(encoding="utf-8", errors="ignore"))})
    if leftovers:
        print("warning: unfilled placeholders:", ", ".join(leftovers))

    if not args.no_git:
        subprocess.run(["git", "init", "-q", "-b", "main"], cwd=dest, check=False)

    print(f"\nCreated {dest}")
    print("Next: open it with your agent, run `cargo test -p guest-core`, then follow docs/DESIGN.md.")


if __name__ == "__main__":
    main()
