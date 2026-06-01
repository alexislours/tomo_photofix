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

## Which version to download

Each release ships two builds. Install **one** of them, not both:

| Download                               | What it does                                                                                                                                                                      |
| -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tomo_photofix-<version>.zip`          | Saves the PNG **and** forwards to `nn::album`, so the photo also lands in the system album.                                                                                       |
| `tomo_photofix-emulator-<version>.zip` | Saves the PNG only, skipping the `nn::album` call. Removes the duplicate save into the system album, and avoids the crash on emulators that don't implement `nn::album` properly. |

Both builds run on real consoles and emulators, pick based on whether you want the photo in the system album. The standard build keeps that copy; the `-emulator` build drops it (just the PNG on the SD card) and is the one to use if your emulator crashes the moment a photo is taken.

## Installation

1. Download the appropriate `.zip` for your setup (see above) from the [Releases](https://github.com/alexislours/tomo_photofix/releases) page.
2. Extract it. You'll get a single mod folder (`tomo_photofix/` or `tomo_photofix-emulator/`, containing `exefs/` and `romfs/`).
3. Copy that folder into your title's mod directory, then restart the game.

The game's title ID is **`010051F0207B2000`**. Each emulator wants the mod inside its own named subfolder.

### Ryujinx

Right-click the game → **Open Mods Directory**, then copy the extracted mod folder.

### Eden

Right-click the game → **Open Mod Data Location**, then copy the extracted mod folder.

## License

Licensed under the [GNU General Public License v3.0](LICENSE).
