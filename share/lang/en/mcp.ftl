# Trindle's assistant access (MCP): the question window that asks the person
# before an assistant changes anything, and the words of its answer buttons.
# What the assistant reads back (its tool descriptions and errors) is written
# for the assistant, not the person, so it is not here.
#
# Rules for new words: one key for each, a comment above saying where it is
# shown, sentence case, UK English, no em dashes, and never written inline in
# the code. Keys start with the part of Trindle that shows them.

## The question window (src/mcp/ask.rs)

# Window title of the question an assistant's request raises.
mcp-title-ask = Assistant request
# Heading over the words of what the assistant wants to do.
mcp-ask-heading = An assistant connected to Trindle wants to:
# Button that refuses the change the assistant asked for.
mcp-no = No
# Button that lets the change the assistant asked for go ahead.
mcp-yes = Yes, go ahead

## What the question says, one line for each kind of change (src/mcp/tools.rs)

# The question for bringing a window to the front; the app's name, in quotes, comes in.
mcp-ask-focus = bring { $name } to the front
# The question for changing a Trindle setting; the setting's name and its old and new values come in.
mcp-ask-setting = change the setting { $key } from { $from } to { $to }
# The question for changing the look; the look (light or dark) comes in.
mcp-ask-theme = use the { $look } theme
# The question for changing the look and the accent colour; both come in.
mcp-ask-theme-accent = use the { $look } theme with { $accent } as the accent colour
# The question for installing an app from Flathub; its Flathub id comes in.
mcp-ask-install = install { $id } from Flathub
# The question for removing an app; its Flathub id comes in.
mcp-ask-remove = remove { $id } and keep its files
# The question for joining a Wi-Fi network; the network's name, in quotes, comes in.
mcp-ask-network = join the Wi-Fi network { $name }
# The question for pairing a Bluetooth device that has no name; its address comes in.
mcp-ask-pair = pair with the Bluetooth device { $address }
# The question for pairing a Bluetooth device; its name, in quotes, and its address come in.
mcp-ask-pair-named = pair with the Bluetooth device { $name } ({ $address })
# The question for closing a window; the window's title or app name, in quotes, comes in.
mcp-ask-close = close { $name }
# The question for opening an app; the app's name, in quotes, comes in.
mcp-ask-open = open { $name }
# The question for letting an app use something; the app's Flathub id and the permission (network, home, devices, sound or bluetooth) come in.
mcp-ask-allow = let { $id } use { $what }
# The question for taking a permission away from an app; the app's Flathub id and the permission come in.
mcp-ask-deny = stop { $id } using { $what }
# The question for putting settings back; the name of the saved point comes in.
mcp-ask-undo = put settings and app permissions back as they were before: { $note }
# The question for putting back a snapshot that also holds an app's own settings file or cache; the app's id and the earlier question, in quotes, come in.
mcp-ask-undo-app = put settings, app permissions and the files of { $id } back as they were before: { $note }. { $id } is closed first if it is open
# The question for closing an app and starting it again; the app's id comes in.
mcp-ask-restart = close { $id } and start it again. Anything not saved in it is lost
# The question for changing a setting in an app's own settings file; the app's id, and the setting, the file and both values in quotes, come in.
mcp-ask-app-setting = change { $key } in { $file } of { $id } from { $from } to { $to }. { $id } is closed first if it is open
# The same question when the setting is not in the file yet and is added.
mcp-ask-app-setting-new = add { $key } as { $to } in { $file } of { $id }. { $id } is closed first if it is open
# The question for clearing an app's cache; the app's id comes in.
mcp-ask-cache = clear the cache of { $id }. { $id } is closed first if it is open, and a copy of the cache is kept so it can be put back