# Settings page "Ring": how the ring opens, notifications, the clipboard and voice control.

## Page (src/settings/panels/ring.rs)

# Name of the Settings page about the ring, in the list and as its heading.
settings-ring-title = Ring
# Words a person might type into the Settings search to find the ring page; separate them with spaces.
settings-ring-keywords = favourites favorites launcher edge click hot corner voice speech windows open windows clipboard copy paste history forget notifications
# Introduction under the heading of the ring page, and the words behind the page i.
settings-ring-help = Your favourites sit around the middle of the ring. You choose them in the ring itself. Right-click any app in the ring to add or remove it, and drag a favourite to another place to change the order.
# Sentence at the top of the ring page saying how the ring opens.
settings-ring-headline = The ring opens at the edge of the screen, and your open windows appear in it.
## Opening the ring

# Line saying how many favourites the ring has; { $n } is the number.
settings-ring-favourites =
    { $n ->
        [one] { $n } favourite
       *[other] { $n } favourites
    }
# Title of the card about opening the ring.
settings-ring-opening = Opening the ring
# Words behind the i of the card about opening the ring.
settings-ring-opening-about = Move the pointer to the edge of the screen and double-click, or press the Super key, to open the ring. Your favourites sit around the middle of the ring, and your open windows sit in a ring around them. Click a window to bring it forward, or right-click it to see its buttons. Right-click any app in the ring to add or remove it.
## Notifications

# Heading of the switch line, and the spoken name of the switch, for showing notifications as they arrive.
settings-ring-notify-show = Show them as they arrive
# Heading of the list line, and the name of the list, for how long a notification stays.
settings-ring-notify-long = How long they stay
# Added to the Notifications card's i.
settings-ring-notify-long-about = Urgent ones stay until closed.
# Heading of the list line, and the name of the list, for which corner notifications appear in.
settings-ring-notify-where = Where they appear
# Added to the Notifications card's i.
settings-ring-notify-where-about = They stack up from this corner.
# Title of the card about notifications.
settings-ring-notify-title = Notifications
# Words behind the i of the card about notifications.
settings-ring-notify-about = When a program has something to tell you, a notification appears for a few seconds and then goes away. If you switch this off, nothing appears, but every notification is still collected in the notifications circle in the ring. An urgent one stays until you close it, whatever you choose here, so you can't miss a warning.
## Clipboard

# Heading of the list line, and the name of the list, for when copied lines are forgotten.
settings-ring-clip-forget = Forget what was copied
# Heading of the line saying how many lines the clipboard holds now.
settings-ring-clip-now = Copied now
# Small words under that line; { $n } is how many lines are kept and { $pinned } how many of them are pinned.
settings-ring-clip-count =
    { $n ->
        [one] { $n } line, { $pinned } pinned.
       *[other] { $n } lines, { $pinned } pinned.
    }
# Title of the card about the clipboard.
settings-ring-clip-title = Clipboard
# Words behind the i of the card about the clipboard.
settings-ring-clip-about = The clipboard circle in the ring keeps what you copy, so you can paste something from earlier. Pinned lines are always kept, whatever you choose here.
## Voice

# Small words under the voice line while voice is being added.
settings-ring-voice-adding = Adding voice. You can keep going.
# Small words under the voice line while its build tools are being removed.
settings-ring-voice-removing-tools = Removing the build tools.
# Small words under the voice line while voice is being removed.
settings-ring-voice-removing = Removing voice.
# Heading of the voice line, and the spoken name of its switch.
settings-ring-voice-control = Voice control
# Said, in green, under the voice switch after voice is turned on.
settings-ring-voice-onoff-on = On
# Said, in green, under the voice switch after voice is turned off.
settings-ring-voice-onoff-off = Off
# Heading of the line, and the name of its button, that removes voice.
settings-ring-voice-remove = Remove voice
# Added to the Voice card's i once voice is added.
settings-ring-voice-remove-about = Frees about 150 MB.
# Words on the line offering to remove the build tools that stayed after voice was added.
settings-ring-voice-tools-left = Ready. Build tools are kept.
# Name of the button that removes the build tools voice left behind.
settings-ring-voice-tools-remove = Remove the build tools
# Name of the button that adds voice control to the computer.
settings-ring-voice-add = Add voice control
# Added to the Voice card's i before voice is added.
settings-ring-voice-optional = Optional. Needs about 150 MB.
# Title of the card about voice.
settings-ring-voice-title = Voice
# Words behind the i of the voice card; the quoted words are examples of what to say.
settings-ring-voice-card-about = Say an app's name, “louder” or “Wi-Fi settings” instead of clicking. Tap the middle of the ring, then speak. Trindle stops listening when you stop speaking. Your speech stays on this computer, and nothing is sent anywhere. Adding voice takes about 150 MB and a few minutes, and you can keep working meanwhile. Removing it deletes the speech engine and its model.
# Words beside the round progress dial while voice is being added.
settings-ring-voice-working-add = Adding voice
# Words beside the round progress dial while voice build tools are being removed.
settings-ring-voice-working-tools = Removing build tools
# Words beside the round progress dial while voice is being removed.
settings-ring-voice-working-remove = Removing voice

## Clipboard history choices (src/common/clipboard.rs)

# Choice in the list of when copied lines are forgotten: every time the computer restarts.
settings-ring-forget-reboot = Each time the computer restarts
# Choice in the list of when copied lines are forgotten: after a day.
settings-ring-forget-day = After a day
# Choice in the list of when copied lines are forgotten: after a week.
settings-ring-forget-week = After a week
# Choice in the list of when copied lines are forgotten: after two weeks.
settings-ring-forget-fortnight = After two weeks
# Choice in the list of when copied lines are forgotten: never.
settings-ring-forget-never = Never

# Heading of the switch line, and the spoken name of the switch, for showing a window's title under its picture when pointing at it in the ring.
settings-ring-names = Show window titles on hover
# Added to the Opening card's i.
settings-ring-names-about = When off, screen readers still read the title.
# Red words when voice could not be added.
settings-ring-voice-not-added = Trindle couldn't add voice.
# Red words when voice could not be removed.
settings-ring-voice-not-removed = Trindle couldn't remove voice.
# Red words when the build tools left after adding voice could not be removed.
settings-ring-voice-tools-not-removed = Trindle couldn't remove the build tools.
