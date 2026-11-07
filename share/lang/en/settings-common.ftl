# Settings: words shared by the pages (src/settings/ui.rs).

## Shared by the pages (src/settings/ui.rs)

# Said beside a line once its change is saved.
settings-common-saved = Saved
# Said beside a button that worked, when nothing more is known.
settings-common-done = Done
# Said beside a button that failed, when nothing more is known.
settings-common-not-done = Not done
# Tip of the information button at the top of every Settings page.
settings-common-about-page = About this page
# Tip of the information button on a card: { $name } is the card's heading in lower case.
settings-common-about-card = About { $name }
# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name } is already there
# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
        [one] Do the same for the rest ({ $n } more)
       *[other] Do the same for the rest ({ $n } more)
    }
# Label of the switch that answers the same way for all that wait.
settings-common-same-for-rest = Do the same for the rest
# Tip of the button that keeps the one already there.
settings-common-skip = Skip: keep the one already there
# Button that replaces the one already there.
settings-common-replace = Replace
# Spoken name of the Replace button.
settings-common-replace-spoken = Replace the one already there with the one being brought
# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = The one being brought
# Heading of the line that shows the one already there.
settings-common-already-line = The one already there
# Heading of the line with the Skip and Replace buttons.
settings-common-replace-question = Replace the one already there?
# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
        [one] It goes back in { $left } second.
       *[other] It goes back in { $left } seconds.
    }
# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
        [one] It goes back in { $left } second. { $more }
       *[other] It goes back in { $left } seconds. { $more }
    }
# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = Go back now
# Button at the foot of the window that keeps the change on trial.
settings-common-keep = Keep
# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } is not available
# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = Install it with: sudo apt install { $package }
# Red words when a web page could not be opened.
settings-common-no-browser = The web page could not be opened: no web browser could be started.
# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = { $name } could not be opened.
# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = It could not be saved.
# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = Not done: the system's message bus could not be reached. Restart the computer, then try again.
# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = Not done: this login's message bus could not be reached. Log out and in, then try again.
# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = The window for choosing could not be opened. Log out and in, then try again.
# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = The disk is full.
# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = The disk cannot be written to just now.
# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = The part of the system that does this is not running.
# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = It took too long to answer. Try again.
# Cause given in red words after an action failed: the password was not given.
settings-common-why-password = The password was not given.
# Cause given in red words after an action failed: the account is not allowed.
settings-common-why-not-allowed = This account is not allowed to.
# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = Something it needs is missing from this computer.
# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }
# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } and { $last }
# What goes between names in a list, before the last two.
settings-common-list-comma = { ", " }

## The picture of the ring (src/settings/pictures.rs)

# Spoken name of the picture of the ring opening on a small screen.
settings-common-ring-picture = The ring opens with a double click at the edge of the screen.
# Word drawn in the middle of that picture.
settings-common-ring-name = The ring

# Words shared by Settings and other programs, from src/common: the charge limit and the virtual machine note.

## Charge limit (src/common/charge.rs)

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = Could not reach the laptop's firmware: { $error }
# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = The laptop's firmware would not take the charge limit.
# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = This computer does not let its charging be stopped short.
# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = Could not set { $path }: { $error }

## Virtual machine note (src/common/graphics.rs)

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = Trying Trindle in a virtual machine
# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindle is running in { $machine }. It works well there, but a virtual machine is never quite as quick or as smooth as a real computer, so please judge its speed on real hardware.
# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindle is running in { $machine } without 3D graphics, so the processor has to draw everything a graphics chip normally would. Windows, videos and web pages will feel slower than on a real computer, and Trindle leaves out its animations to help. If the computer it runs on can do it, turning on 3D acceleration in the virtual machine's display settings makes a big difference. Installed on a real computer, none of this applies.
