# monarch 👑

![Demo](https://raw.githubusercontent.com/vladislav-yemelyanov/monarch/main/mon.gif)

**monarch** is a fast CLI tool for discovering and opening projects on your system.
It is designed for developers who work with many repositories and want a quick way to search, preview, and open projects in their favorite editor.

> ⚠️ **Note**: At the moment, `monarch` is tested and supported only on **macOS**.
> Other operating systems may work, but are not officially supported yet.


---

## Features

- 🔍 **Project discovery**
  Scans directories and finds all projects that contain a `.git` folder.

- 📂 **Quick project selection**
  Select any project from the list and instantly open it in your editor (e.g. **Helix**, **Neovim**, etc.).

- 🔎 **Interactive search**
  - Fuzzy search through project paths
  - Highlighted matches while typing
  - File preview for better context

- 🕘 **Recently used projects**
  Recently opened projects are always shown at the bottom of the list — making it faster to switch between frequently used repositories.

---

## Why monarch?

If you often jump between multiple Git repositories, `monarch` helps you:
- avoid manually navigating directories,
- keep frequently used projects close at hand,
- open projects in your editor with minimal friction.

---

## Platforms

- [x] x86_64-apple-darwin
- [x] aarch64-apple-darwin
- [x] x86_64-unknown-linux-gnu
- [ ] x86_64-pc-windows-gnu

---

## Manual Installation

`monarch` is currently distributed as a standalone binary.
Homebrew support is planned for the future.

1. Download the latest release [IN PROGRESS]
2.
```bash
sudo mv monarch /usr/local/bin/monarch
```
### Shell integration

#### Fish
You can create a small helper function to launch monarch more conveniently:

```fish
function mon
    set dir (monarch --projects-dir=/Users/{NAME}/Documents/projects)
    cd $dir; or return
    hx .
end

```

Restart your shell or reload the config:

```fish
source ~/.config/fish/config.fish
```

#### Zsh / Bash
For zsh or bash, add an alias to your shell config (`~/.zshrc` or `~/.bashrc`):

```bash
mon() {
  local dir
  dir="$(monarch --projects-dir="/Users/{NAME}/Documents/projects")" || return
  cd "$dir" || return
  hx .
}
```

After restart your shell, run:

```bash
mon
```

### Future installation plans

- 🍺 Homebrew formula (once the project is ready for wider distribution)
- 📦 Prebuilt binaries for Linux and Windows
- 🤖 Automated install script (curl | sh)

If you like the tool and want Homebrew support — leave a ⭐️ on GitHub!
