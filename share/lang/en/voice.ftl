# Trindle's voice control: the words the ring says back after a spoken
# request, the voice set-up's progress lines, and the reasons it did not start.
# The words the person says are matched in English and are not here.
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## What the ring says back after a spoken request (src/voice/commands.rs)

# Said under the ring when a radio is switched on by speaking; the radio's name comes first.
voice-radio-on = { $name } on
# Said under the ring when a radio is switched off by speaking; the radio's name comes first.
voice-radio-off = { $name } off
# Said under the ring after "brighter" turns the screen up.
voice-brighter = Brighter
# Said under the ring after "dimmer" turns the screen down.
voice-dimmer = Dimmer
# Said under the ring when the request is to let the screen rest again.
voice-keep-awake-off = Screen can sleep again
# Said under the ring when the screen is kept awake.
voice-keep-awake-on = Staying awake
# Said under the ring when the request is to look for updates.
voice-updates = Looking for updates
# Said under the ring when nothing was heard at all.
voice-nothing-heard = Trindle couldn't hear you. Tap the middle of the ring and speak.
# Said under the ring when the volume goes up.
voice-louder = Louder
# Said under the ring when the volume goes down.
voice-quieter = Quieter
# Said under the ring when the sound is switched back on.
voice-sound-on = Sound on
# Said under the ring when the sound is muted.
voice-muted = Muted
# Said under the ring when the wallpaper changes to the next one.
voice-next-wallpaper = Next wallpaper
# Said under the ring after a screenshot is taken; Pictures is the folder's name.
voice-screenshot-saved = Screenshot saved in Pictures
# Said under the ring when the request is to show every app.
voice-all-apps = All apps
# Said under the ring before shutting down, which waits for a click to confirm.
voice-confirm-power-off = Shut down? Click to confirm
# Said under the ring before restarting, which waits for a click to confirm.
voice-confirm-reboot = Restart? Click to confirm
# Said under the ring before logging out, which waits for a click to confirm.
voice-confirm-logout = Log out? Click to confirm
# Said under the ring when the request is to find an app in Apps; the words the person said for it come in.
voice-looking-for-app = Looking for { $what } in Apps
# Said under the ring when the request opens Apps (the store).
voice-apps = Apps
# Said under the ring when the request opens the disks page of Settings; Drives is the page's name.
voice-drives = Drives
# Said under the ring when the request opens Settings.
voice-settings = Settings
# Said under the ring when the request closes the window in front.
voice-closing-window = Closing the window
# Said under the ring when an open app is closed; the app's name comes in.
voice-closing-app = Closing { $name }
# Said under the ring when no app has the name the person said; the name they said comes in.
voice-no-app = No app called { $name }
# Said under the ring when a window is brought to the front; the app's name comes in.
voice-showing-app = Showing { $name }
# Said under the ring when an app is opened; the app's name comes in.
voice-opening-app = Opening { $name }
# Said under the ring when the request was heard but not understood; what was heard comes in.
voice-heard-unknown = Trindle couldn't recognise the command “{ $heard }”

## Why voice did not start (src/voice/mod.rs)

# Said in the ring when voice has not been added yet.
voice-optional = Voice is an optional extra. Add it in Settings, on the Ring page.
# Said in the ring when voice has been added but is switched off.
voice-switched-off = Voice is switched off. Switch it on in Settings, on the Ring page.
# Said in the ring when the microphone cannot be opened; the reason comes after it.
voice-mic-failed = Trindle couldn't access the microphone: { $why }
# The reason after "Trindle couldn't access the microphone:" when the session has no private folder to keep a recording in.
voice-no-private-folder = no private folder is available.
# Said in the ring when the microphone's recorder would not start.
voice-mic-not-started = Trindle couldn't access the microphone because the recording program didn't start
# Said in the ring when listening stopped and nothing was recorded.
voice-nothing-recorded = Nothing was recorded.
# Said in the ring when the recording had no speech in it.
voice-nothing-heard-after = Trindle couldn't hear you. Tap the middle of the ring and speak.
# Said in the ring when the speech engine failed and gave no reason.
voice-whisper-failed = Whisper failed

## Adding and removing voice (src/voice/setup.rs)

# Progress line while the build tools are added for the build; the names of the packages come in.
voice-installing-tools = Installing build tools temporarily: { $names }
# Progress line while whisper.cpp is built; its version comes in.
voice-building = Building whisper.cpp { $version } for voice control. This takes a minute or two.
# Progress line while the speech model is downloaded (148 MB).
voice-fetching-model = Downloading speech model (148 MB)
# Progress line while the build tools are taken away again.
voice-removing-tools = Removing the build tools now
# Said once voice is added and ready to use.
voice-ready = Voice control is ready. Open the ring, tap the middle, and speak.
# Said once voice is removed.
voice-removed = Voice control removed.
# Shown in the terminal window that stays open, asking for Enter to close it.
voice-press-enter = Press Enter to close.
# Shown when the command is run with neither install nor remove.
voice-usage = usage: trindle-voice-setup install | remove

# Said when a step of the voice set-up fails. { $program } is the program that was run, with its first word.
voice-step-failed = { $program } didn't work.
# Said when the whisper.cpp download isn't the one Trindle checks for, so nothing is built from it.
voice-whisper-fingerprint = The whisper.cpp download isn't the one Trindle expected, so it wasn't built.
# Said when the speech model download isn't the one Trindle checks for, so it isn't installed.
voice-model-fingerprint = The speech model download isn't the one Trindle expected, so it wasn't installed.
