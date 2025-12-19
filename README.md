# monarch 👑

![Demo](https://raw.githubusercontent.com/vladislav-yemelyanov/monarch/main/mon.gif)

**monarch** is a fast interactive CLI for instantly finding and opening Git repositories.

> ⚠️ **Note**: `monarch` is fully supported on **macOS**.
> Linux builds are available but considered experimental.

---

## Features

🔍 **Automatic Git repository discovery**
- Recursively scans directories and finds repositories containing a `.git` folder.

⚡ **Instant project switching**
- Select a repository and open it instantly.

🔎 **Interactive fuzzy search**
- Fuzzy matching on full paths
- Live highlighted results
- Project preview for context

🕘 **Recently used projects**
- Frequently used projects are pinned at the bottom for quick access.

---

## Why monarch?

- Faster than manual `cd` navigation
- No need to remember project paths
- Editor-agnostic
- Follows the Unix philosophy: does one thing well and composes nicely with shells

---

## How it works

💡 `monarch` prints the selected project path to **stdout**,
making it easy to compose with shells and editors.

## Keyboard Shortcuts

Navigate and manage projects quickly using the following keyboard shortcuts:

Ctrl + P → Move up through the list

Ctrl + N → Move down through the list

Enter → Select the highlighted project

Esc → Exit the menu

---

## Platform support

| Platform | Status |
|--------|--------|
| macOS (Intel / Apple Silicon) | ✅ Fully supported |
| Linux (x86_64) | ⚠️ Experimental |
| Windows | ❌ Not supported yet |

Linux builds exist, but have not been extensively tested yet.

---

## Manual Installation

`monarch` is currently distributed as a standalone binary.
Homebrew support is planned for the future.

1. [Download the latest release](https://github.com/vladislav-yemelyanov/monarch/releases)
2. Then:
```bash
chmod +x monarch
sudo mv monarch /usr/local/bin/monarch
```
### Shell integration

#### Fish
You can create a small helper function to launch monarch more conveniently:

```fish
function mon
    set dir (monarch --projects-dir=/Users/{NAME}/Documents/projects) || return

    if test -z "$dir"
      return # dir is empty - exit
    end

    cd $dir; or return

    hx . # hx, nvim, etc.
end

```

Replace the path with the directory where you keep your repositories.

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

  if [ -z "$dir" ]; then
     return # dir is empty -> exit
  fi

  cd "$dir" || return

  hx . # hx, nvim, etc.
}
```
Replace the path with the directory where you keep your repositories.

After restart your shell, run:

```bash
mon
```

## Basic usage

```bash
monarch --projects-dir=/Users/{NAME}/Documents/projects
```

Replace the path with the directory where you keep your repositories.

## CLI options

- `--projects-dir <path>` — directory to scan for Git repositories

### Future installation plans

- 🍺 Homebrew formula (once the project is ready for wider distribution)
- 📦 Prebuilt binaries for Linux and Windows
- 🤖 Automated install script (curl | sh)

---

If you find `monarch` useful:
- ⭐️ Star the repo
- 🐞 Report bugs or ideas
- 💡 Suggest features

---

Built with 🦀 Rust.
