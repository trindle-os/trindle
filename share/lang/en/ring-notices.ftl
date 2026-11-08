# The ring's notices and notification circles: battery card, snooze choices, crash notices, the microphone switch and the magnifier. Part of the ring's own words (see ring.ftl).

## Battery card (src/ring/battery_alert.rs)

# Title of the low battery warning card, and the line under Charging when the charger goes in.
ring-battery-at = Battery at { $pct }%
# Line under the low battery warning's title.
ring-battery-plug-in = Plug the charger in soon.
# Title of the card that shows for a few seconds when the charger goes in.
ring-battery-charging = Charging
# Title of the card that shows for a few seconds when the charger comes out.
ring-battery-unplugged = Charger unplugged
# Line under the title when the charger comes out.
ring-battery-running = Running on the battery, at { $pct }%

## Notification circles (src/ring/circles.rs)

# Choice on the snooze page of a notification circle (how long to put it off).
ring-snooze-10-minutes = 10 minutes
# Choice on the snooze page of a notification circle.
ring-snooze-1-hour = 1 hour
# Choice on the snooze page of a notification circle.
ring-snooze-6-hours = 6 hours
# Choice on the snooze page of a notification circle.
ring-snooze-next-morning = Next morning
# Tip and screen reader name of the snooze choice for ten minutes.
ring-snooze-for-10-minutes = Snooze for 10 minutes
# Tip and screen reader name of the snooze choice for an hour.
ring-snooze-for-1-hour = Snooze for 1 hour
# Tip and screen reader name of the snooze choice for six hours.
ring-snooze-for-6-hours = Snooze for 6 hours
# Tip and screen reader name of the snooze choice for the next morning.
ring-snooze-for-next-morning = Snooze for next morning
# Tip of the button on the snooze page that goes back to the notification's message.
ring-act-back = Back to the message
# Tip of the pin button when the notification is already pinned.
ring-act-unpin = Unpin
# Tip of the pin button on a notification circle: keeps it until closed.
ring-act-pin = Pin
# Tip of the button on a notification circle that opens the snooze choices.
ring-act-snooze = Snooze
# Tip of the button on the stack of notification circles that clears them all.
ring-act-clear-all = Clear all
# Tip of the copy button for a moment after the message was copied.
ring-act-copied = Copied
# Tip of the button on a notification circle that copies its message.
ring-act-copy = Copy the message

## Crash notices (src/ring/crashes.rs)

# Line under the title of the notice that says one of Trindle's programs crashed.
ring-crash-click = Click to hand it to your assistant.
# Name of Trindle's own main program in the crash notice.
ring-crash-trindle = Trindle
# Name of the desktop part in the crash notice.
ring-crash-desktop = Trindle's desktop
# Name of the login screen in the crash notice.
ring-crash-greeter = Trindle's login screen
# Name of the lock screen in the crash notice.
ring-crash-lock = Trindle's lock screen
# Name of Settings in the crash notice.
ring-crash-settings = Trindle Settings
# Name of Apps in the crash notice.
ring-crash-apps = Trindle's Apps
# Name of any other Trindle program in the crash notice; the name is the program's own, with spaces for dashes.
ring-crash-other = Trindle's { $name }
# Title of the notice that says a program crashed.
ring-crash-stopped = { $name } stopped unexpectedly

## Drives (src/ring/drives.rs)

# Notice when a drive's button was pressed and the drive has gone.
ring-drive-gone = That drive is not there any more
# Title of the notice when a drive could not be opened; the reason follows under it.
ring-drive-not-opened = { $name } could not be opened

## Closing the lid (src/ring/lid.rs)

# Choice for what closing the laptop lid does: sleep.
ring-lid-sleep = Sleep
# Choice for what closing the laptop lid does: hibernate.
ring-lid-hibernate = Hibernate
# Choice for what closing the laptop lid does: lock the screen.
ring-lid-lock = Lock the screen
# Choice for what closing the laptop lid does: nothing.
ring-lid-nothing = Nothing
# Choice for what closing the laptop lid does: shut down.
ring-lid-shut-down = Shut down
# Reason the system lists beside Trindle's hold on the lid switch (shown by tools that list what stops sleep).
ring-lid-reason = Closing the lid does what Settings, Power says

## Magnifier (src/ring/magnify.rs)

# Notice at the foot of the screen for a few seconds when the magnifier starts.
ring-magnify-hint = Hold anywhere to magnify
# Screen reader name of the magnifier's glass.
ring-magnify-name = Magnifier
# Tip of the lit button that puts the magnifier away.
ring-magnify-stop = Stop magnifying

## Microphone switch (src/ring/mic.rs)

# Tip of the switch in the ring that turns the microphone off for every program.
ring-mic-off-tip = Turn the microphone off for every program
# Tip of the switch in the ring that turns the microphone back on.
ring-mic-on-tip = Turn the microphone back on
# Line in the ring when something other than the person pressed the microphone switch on.
ring-mic-own-press = Only your own press can turn the microphone back on.
# Line in the ring when the microphone switch could not be changed; the reason follows.
ring-mic-not-changed = The microphone switch could not be changed: { $why }

## Clicking a notification (src/ring/note_goes.rs)

# Notice when a notification was clicked and no program could be found for it.
ring-goes-no-program = No program found
# Notice when a notification was clicked and it was sent by a program Trindle cannot open.
ring-goes-other-program = Sent by another program
# Notice when the program for a clicked notification could not be started.
ring-goes-not-started = Could not start it

## Notification stack (src/ring/notes/)

# Notice after a click on a crash notification: its details were copied for an assistant.
ring-notes-copied = Copied for your assistant
# Screen reader name of a notification circle.
ring-notes-name = Notification
# Said when a pinned notification has been held long enough and now follows the pointer.
ring-notes-moving = Moving. Let go to drop, Escape to put back.
# Said when a pinned notification has been dropped or moved by keys.
ring-notes-moved = Moved
# Said when Escape put a held notification back.
ring-notes-put-back = Put back
# Shown in a notification that is not pinned when a key to move it is pressed.
ring-notes-pin-to-move = Pin it to move it

## Notification circles (src/ring/notify.rs)

# Choice in Settings, Ring, for how long a notification circle stays.
ring-notify-seconds =
    { $n ->
        [one] { $n } second
       *[other] { $n } seconds
    }
# Choice in Settings, Ring: the notification circle stays until it is closed.
ring-notify-until-closed = Until you close it
# Time under a notification from yesterday.
ring-notify-yesterday = Yesterday { $clock }
# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-right = Bottom right
# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-left = Bottom left
# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-left = Top left
# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-right = Top right

## Plugins in the ring (src/ring/plugins.rs)

# Title of the notice when a plugin that asks for permissions has been given none.
ring-plugin-needs = { $name } needs your permission
# Line under the notice that a plugin needs permission: none of the permissions it asks for are given.
ring-plugin-cannot-work = It cannot work yet: 0 of { $asks } allowed.

# Said to a screen reader when a notification was copied; $said is what the circle said.
ring-copied-said = Copied. { $said }

## Programs outside a box (src/ring/outside.rs)

# Title of the notice for ordinary programs that have just reached the network; the program names are under it, and Allow means this is fine.
ring-outside-network = Reached the network
# Title of the notice for ordinary programs that have your files open; the program names are under it, and Allow means this is fine.
ring-outside-files = Has your files open
# Under the title of those two notices: up to three program names, then how many more.
ring-outside-body =
    { $more ->
        [0] { $names }
       *[other] { $names } and { $more } more
    }
