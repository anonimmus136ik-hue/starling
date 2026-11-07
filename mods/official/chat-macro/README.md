# Chat Macro

A client-side Horizon mod with two independent command macros.

## Bind keys

Open **Options → Mods → Chat Macro**. The mod registers two keys:

- **Run macro 1** — Right Control by default
- **Run macro 2** — Right Shift by default

Select either key in the game's key-binding controls to assign another supported keyboard or mouse key. These are normal mod keybinds managed by Horizon, not hard-coded Caps Lock handling.

The settings page also has an ON/OFF button for each macro.

## Set actions

Use the in-game chat:

```text
/chatmacro set 1 <command>
/chatmacro set 2 <command>
```

Examples:

```text
/chatmacro set 1 home
/chatmacro set 2 msg Friend Hello!
```

The leading slash is optional. The command runs as the local player and cannot grant permissions the player does not already have.

Other commands:

- `/chatmacro clear 1` or `/chatmacro clear 2` — clear an action
- `/chatmacro on 1` or `/chatmacro on 2` — enable it
- `/chatmacro off 1` or `/chatmacro off 2` — disable it
- `/chatmacro help` — show help

Commands and enabled states are saved in the mod's storage.

## Compatibility notes

The key-binding API is experimental in this Horizon SDK. This source has not yet been built into an Android `.hmod` or tested on-device. The app's existing **Options → Mods** screen is the mod menu; this mod adds its own settings page there.
