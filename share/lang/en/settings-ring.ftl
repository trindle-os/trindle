# Settings page "Ring": how the ring opens, notifications, the clipboard and voice control.

## Page (src/settings/panels/ring.rs)

# Name of the Settings page about the ring, in the list and as its heading.
settings-ring-title = Ring
# Words a person might type into the Settings search to find the ring page; separate them with spaces.
settings-ring-keywords = favourites favorites launcher edge click hot corner voice speech windows open windows clipboard copy paste history forget notifications
# Introduction under the heading of the ring page, and the words behind the page i.
settings-ring-help = Your favourites sit round the middle of the ring. They are chosen on the ring itself: right-click any app in the ring to add or remove it, and drag a favourite onto another's place to change the order.
# Sentence at the top of the ring page saying how the ring opens.
settings-ring-headline = The ring opens at the edge of the screen, and your open windows sit in it.
# Name of the picture on the ring page that shows where things open.
settings-ring-picture = Where things open
## Opening the ring

# Heading of the line saying the ring opens by double-clicking at the screen edge.
settings-ring-open-click = Double-click at the edge of the screen
# Small words under that line about the Super key.
settings-ring-open-super = The Super key opens it too.
# Line saying how many favourites the ring has; { $n } is the number.
settings-ring-favourites =
    { $n ->
        [one] { $n } favourite
       *[other] { $n } favourites
    }
# Small words under the favourites line.
settings-ring-favourites-about = Right-click an app in the ring to add one.
# Title of the card about opening the ring.
settings-ring-opening = Opening the ring
# Words behind the i of the card about opening the ring.
settings-ring-opening-about = Move the pointer to the edge of the screen and double-click, or press the Super key, and the ring opens. Your favourites sit round the middle of the ring, and your open windows in a ring round them: click one to bring it forward, or right-click it for its buttons. Right-click any app in the ring to add or remove it.
## Notifications

# Heading of the switch line, and the spoken name of the switch, for showing notifications as they arrive.
settings-ring-notify-show = Show them as they arrive
# Small words under the switch for showing notifications as they arrive.
settings-ring-notify-show-about = Off: they still gather in the ring.
# Heading of the list line, and the name of the list, for how long a notification stays.
settings-ring-notify-long = How long they stay
# Small words under the list for how long notifications stay.
settings-ring-notify-long-about = Urgent ones wait to be closed.
# Heading of the list line, and the name of the list, for which corner notifications appear in.
settings-ring-notify-where = Where they appear
# Small words under the list for where notifications appear.
settings-ring-notify-where-about = The corner they stack away from.
# Title of the card about notifications.
settings-ring-notify-title = Notifications
# Words behind the i of the card about notifications.
settings-ring-notify-about = When a program has something to tell you, it appears for a few seconds and goes again. Turned off, nothing appears, and everything is still gathered in the notifications circle in the ring. An urgent one waits to be closed whatever is chosen here, so a warning cannot take itself away before it has been read.
## Clipboard

# Heading of the list line, and the name of the list, for when copied lines are forgotten.
settings-ring-clip-forget = Forget what was copied
# Small words under the list for when copied lines are forgotten.
settings-ring-clip-forget-about = Pinned lines are kept.
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
settings-ring-clip-about = The clipboard circle in the ring keeps what you copy, so you can paste something from earlier. Pinned lines are always kept, whatever is chosen here.
## Voice

# Small words under the voice line while voice is being added.
settings-ring-voice-adding = Adding voice; carry on meanwhile.
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
# Small words under the voice switch.
settings-ring-voice-about = Understood on this computer only.
# Heading of the line, and the name of its button, that removes voice.
settings-ring-voice-remove = Remove voice
# Small words under the line that removes voice.
settings-ring-voice-remove-about = Frees about 150 MB.
# Words on the line offering to remove the build tools that stayed after voice was added.
settings-ring-voice-tools-left = Voice ready. Build tools kept.
# Name of the button that removes the build tools voice left behind.
settings-ring-voice-tools-remove = Remove the build tools
# Name of the button that adds voice control to the computer.
settings-ring-voice-add = Add voice control (about 150 MB)
# Small words under the voice line before voice is added.
settings-ring-voice-optional = Optional; about 150 MB to add.
# Title of the card about voice.
settings-ring-voice-title = Voice
# Words behind the i of the voice card; the quoted words are examples of what to say.
settings-ring-voice-card-about = Say an app's name, “louder” or “Wi-Fi settings” instead of clicking. Tap the middle of the ring and speak; it stops listening when you do. Speech is understood on this computer and nothing is sent anywhere. Adding it takes about 150 MB and a few minutes, and you can carry on meanwhile; removing it takes away the speech engine and its model.
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
settings-ring-names = Show the title when pointing
# Small words under that switch.
settings-ring-names-about = Off: only the snapshot. Screen readers still get the title.
