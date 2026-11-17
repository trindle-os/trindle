# Settings: words shared by the pages (src/settings/ui.rs).

## Shared by the pages (src/settings/ui.rs)

# Said beside a line once its change is saved.
settings-common-saved = Saved
# Said beside a button that worked, when nothing more is known.
settings-common-done = Done
# Words that only say "that worked", in lower case and separated by bars. A line saying one of them under a control that has already moved is left out, as the glow says it. Translate each word, and keep the bars.
settings-common-quiet-words = saved|done|in use now|changed|set|taken|added|freed|removed|stored|installed|updated|joined|copied|default now|played|sent|connected|connecting|on|off|forgotten|stops at 80%|charges to full|starts at login|kept, but does not start|up to date
# Said beside a button that failed, when nothing more is known.
settings-common-not-done = Not done
# Tip of the information button at the top of every Settings page.
settings-common-about-page = About this page
# Tip of the information button on a card: { $name } is the card's heading as it is written.
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
settings-common-skip = Skip and keep the existing one
# Button that replaces the one already there.
settings-common-replace = Replace
# Spoken name of the Replace button.
settings-common-replace-spoken = Replace the existing one with the new one
# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = The new one
# Heading of the line that shows the one already there.
settings-common-already-line = The one already there
# Heading of the line with the Skip and Replace buttons.
settings-common-replace-question = Replace the one already there?
# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
        [one] Undoing it in { $left } second.
       *[other] Undoing it in { $left } seconds.
    }
# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
        [one] Undoing it in { $left } second. { $more }
       *[other] Undoing it in { $left } seconds. { $more }
    }
# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = Undo now
# Button at the foot of the window that keeps the change on trial.
settings-common-keep = Keep
# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } isn't available
# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = To install it, run: sudo apt install { $package }
# Red words when a web page could not be opened.
settings-common-no-browser = Trindle couldn't open the web page, because it couldn't start a web browser.
# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = Couldn't open { $name }.
# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = Couldn't save it.
# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = Trindle couldn't do that because part of the system isn't responding. Restart the computer, then try again.
# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = Trindle couldn't do that because part of your login isn't responding. Log out and back in, then try again.
# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = Trindle couldn't open the window for choosing a file or folder. Log out and back in, then try again.
# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = The disk is full.
# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = Trindle can't write to the disk now.
# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = The part of the system that does this isn't running.
# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = It took too long to answer. Try again.
# Cause given in red words after an action failed: the password was not given.
settings-common-why-password = No password was given.
# Cause given in red words after an action failed: the account is not allowed.
settings-common-why-not-allowed = This account can't do that.
# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = Something it needs is missing from this computer.
# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }
# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } and { $last }
# A list of choices ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-or = { $rest } or { $last }
# What goes between names in a list, before the last two.
settings-common-list-comma = { ", " }

## The picture of the ring (src/settings/pictures.rs)

# Spoken name of the picture of the ring opening on a small screen.
settings-common-ring-picture = You open the ring by double-clicking at the edge of the screen.
# Word drawn in the middle of that picture.
settings-common-ring-name = The ring

# Words shared by Settings and other programs, from src/common: the charge limit and the virtual machine note.

## Charge limit (src/common/charge.rs)

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = Trindle couldn't reach the laptop's charging controls: { $error }
# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = The laptop didn't accept the charge limit.
# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = This computer doesn't let you stop charging early.
# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = Couldn't set { $path }: { $error }

## Virtual machine note (src/common/graphics.rs)

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = Trying Trindle in a virtual machine
# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = Trindle is running in { $machine }. It works well there, but a virtual machine is never quite as quick or smooth as a real computer. Judge its speed on real hardware.
# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = Trindle is running in { $machine } without 3D graphics. The processor has to draw everything a graphics chip would normally draw. Windows, videos and web pages will feel slower than on a real computer, so Trindle leaves out its animations. If the computer it runs on can do it, switch on 3D acceleration in the virtual machine's display settings. That makes a big difference. None of this applies if you install Trindle on a real computer.
