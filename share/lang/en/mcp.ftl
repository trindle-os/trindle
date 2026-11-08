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
mcp-title-ask = An assistant asks
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
mcp-ask-theme = switch to the { $look } look
# The question for changing the look and the accent colour; both come in.
mcp-ask-theme-accent = switch to the { $look } look with the accent colour { $accent }
# The question for installing an app from Flathub; its Flathub id comes in.
mcp-ask-install = install { $id } from Flathub
# The question for removing an app; its Flathub id comes in.
mcp-ask-remove = remove { $id } (its own files are kept)
# The question for joining a Wi-Fi network; the network's name, in quotes, comes in.
mcp-ask-network = join the Wi-Fi network { $name }
# The question for pairing a Bluetooth device; its address comes in.
mcp-ask-pair = pair with the Bluetooth device { $address }
# The question for putting settings back; the name of the saved point comes in.
mcp-ask-undo = put settings and app permissions back as they were before: { $note }
