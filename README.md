# tomo_photofix

A [Skyline](https://github.com/skyline-dev/skyline) plugin for **Tomodachi Life: Living the Dream** that fixes in game photos and write them as proper PNGs in emulators. It should also produce higher quality photos on real consoles.

### Output

Photos are written to `tomo_photofix/photo_<timestamp>.png` on the (virtual) SD card. The directory is created automatically on launch. Where that lands depends on where you run it:

| Platform          | Screenshot location                                           |
| ----------------- | ------------------------------------------------------------- |
| Real console      | `sd:/tomo_photofix/`                                          |
| Ryujinx - Windows | `%AppData%\Ryujinx\sdcard\tomo_photofix\`                     |
| Ryujinx - Linux   | `~/.config/Ryujinx/sdcard/tomo_photofix/`                     |
| Ryujinx - macOS   | `~/Library/Application Support/Ryujinx/sdcard/tomo_photofix/` |
| Eden - Windows    | `%AppData%\eden\sdmc\tomo_photofix\`                          |
| Eden - Linux      | `~/.local/share/eden/sdmc/tomo_photofix/`                     |

## Installation

1. Download the latest `tomo_photofix-<version>.zip` from the [Releases](https://github.com/alexislours/tomo_photofix/releases) page.
2. Extract it. You'll get a single `tomo_photofix/` folder (containing `exefs/` and `romfs/`).
3. Copy that folder into your title's mod directory, then restart the game.

The game's title ID is **`010051F0207B2000`**. Each emulator wants the mod inside its own named subfolder.

### Ryujinx

Right-click the game → **Open Mods Directory**, then copy the `tomo_photofix` folder.

### Eden

Right-click the game → **Open Mod Data Location**, then copy the `tomo_photofix` folder.

## License

Licensed under the [GNU General Public License v3.0](LICENSE).
