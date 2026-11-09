# The ring's own words (src/ring): the circle opened from the top of the screen, its pages, the notification circles, the tray and the small notices.
# Keys are ring-<page>-<thing>. Words shared by several pages are ring-<word>.

## Words shared by the ring's pages

# Tip and name of the round button that closes or puts away a ring panel, a notice or a battery card.
ring-close = Close
# Tip and name of a button that opens what is shown (a notification's drive, a notification's program).
ring-open = Open
# Said in the middle of the ring when Close is chosen on a plugin's icon and none of its windows are open.
ring-plugin-not-open = Nothing open to close
# Name of the line or button that opens Settings from the ring (plugin menus, the ring's own tools).
ring-settings = Settings

## Command line (src/ring/mod.rs)

# First line of the ring's --help, printed in a terminal.
ring-cli-usage = Usage:
# Second line of the ring's --help, printed in a terminal: the command and where options go; the command's name is filled in.
ring-cli-command = { $program } [OPTION…]
# Line of the ring's --help, printed in a terminal, above the list of options.
ring-cli-options = Options:
# Help for the ring's --say option, printed in a terminal.
ring-cli-say = Do what these words ask, as if spoken to the ring (to try voice commands without a microphone)
# Help for the ring's --background option, printed in a terminal.
ring-cli-background = Start without showing, so the first opening is instant
# Help for the ring's --show-desktop option, printed in a terminal.
ring-cli-show-desktop = Put every window away to show the desktop, or bring them back (Super and D asks for this)
# Help for the ring's --listen option, printed in a terminal.
ring-cli-listen = Open the ring already listening, as a keyboard shortcut can
# Help for the ring's --all option, printed in a terminal.
ring-cli-all = Open the ring with every app shown round it
# Help for the ring's --power option, printed in a terminal.
ring-cli-power = Open the ring at log out, restart and shut down
# Help for the ring's --favourites option, printed in a terminal.
ring-cli-favourites = Open the favourites page in Settings
# Help for the ring's --restyle option, printed in a terminal.
ring-cli-restyle = Take up a new look (light or dark, or a new accent) from Settings at once
# Help for the ring's --corners option, printed in a terminal.
ring-cli-corners = Make the touch spots in the top corners again
# Help for the ring's --notes option, printed in a terminal.
ring-cli-notes = Move the keyboard into the notification circles on the screen (Super and N asks for this)
# Help for the option that opens the shortcut list, or closes it when it is showing.
ring-cli-shortcuts = Show every keyboard shortcut in a list, or put the list away (Super and K, unless changed in Settings)
# Error printed in a terminal when the ring is given an option it does not know.
ring-cli-unknown = unknown option { $option }
# Error printed in a terminal when --say is given no words.
ring-cli-missing-say = missing argument for --say

## The ring itself (src/ring/view/)

# Line in the ring's results when what was typed matches no app or favourite.
ring-find-nothing = No app or favourite is called that
# Tip of the round button on an app's menu that removes the app.
ring-uninstall = Uninstall
# Line that replaces the uninstall button while the app is being removed.
ring-uninstalling = Uninstalling
# Line that replaces the uninstall button once the app is removed.
ring-uninstalled = Uninstalled
# Tip of the round button on the menu of an app put there by hand: it only leaves the ring.
ring-take-off = Take off the ring
# Line that replaces that button while the app is being taken off the ring.
ring-taking-off = Taking off
# Line that replaces that button once the app is off the ring.
ring-taken-off = Taken off
# Name of the ring's round button that opens Apps.
ring-tool-apps = Apps
# Name of the ring's round button that opens Activity.
ring-tool-activity = Activity
# Name of the ring's round button that opens Drives.
ring-tool-drives = Drives
# Name of the ring's round button that takes a screenshot.
ring-tool-screenshot = Screenshot
# Name of the ring's round button that opens the clipboard page.
ring-tool-clipboard = Clipboard
# Name of the ring's round button that opens the notifications page.
ring-tool-notifications = Notifications
# Name of the ring's round button that starts the magnifier.
ring-tool-magnifier = Magnifier
# Name of the ring's round button that picks a colour from the screen.
ring-tool-eye-dropper = Eye dropper
# Name of the ring's round button that opens the report a problem window.
ring-tool-report = Report a problem
# Line in the ring when the item that was pressed is not there any more.
ring-gone = That is no longer there.
# Line in the ring when nothing could open the item that was pressed.
ring-could-not-open = Nothing could open that.
# Line in the ring while it listens to a spoken command.
ring-listening = Listening... speak, and it stops when you do
# Line in the ring while it works out what was said.
ring-working-out = Working it out...
# Answer to the spoken command to close windows when none are open.
ring-nothing-to-close = Nothing to close
# Answer to the spoken question what the time is: the time as 24 hour clock, then the weekday, day and month.
ring-voice-time = It is { $time } on { $date }
# Answer to the spoken command to change brightness when the screen has no brightness control.
ring-brightness-fixed = This screen's brightness cannot be changed from here
# Answer to the spoken command for the on-screen keyboard when it is not installed.
ring-keyboard-missing = The on-screen keyboard is not installed
# Screen reader name of the ring of open windows.
ring-open-windows = Open windows
# Tip of the button on a window's menu that brings a minimised window back.
ring-window-restore = Restore
# Tip of the button on a window's menu that minimises it.
ring-window-minimise = Minimise
# Tip of the button on a window's menu that brings a maximised window back to its size.
ring-window-restore-size = Restore size
# Tip of the button on a window's menu that maximises it.
ring-window-maximise = Maximise
# Tip of the button on a window's menu that stops a program that does not answer.
ring-window-force-close = Force close
# Name and screen reader name of the button in the ring that shows every app.
ring-all-apps = All apps
# Name of the ring's round button that opens the power page.
ring-power = Power
# Screen reader description of the ring's power button.
ring-power-spoken = Log out, restart or shut down
# Screen reader name of the ring's results while something is typed.
ring-finding = Finding { $typed }
# Screen reader name of the time and date in the middle of the ring; pressing it opens Settings, Date and time.
ring-date-time = Date and time
# Tip of the button on an app's menu that opens another window of an app already running.
ring-new-window = New window
# Tip of the button on an app's menu that closes its windows.
ring-close-window =
    { $n ->
        [one] Close window
       *[other] Close all { $n } windows
    }
# Tip of the star on the menu of a folder or file pinned to the ring: takes it off the ring.
ring-star-unpin = Unpin from the ring
# Tip of the star on the menu of a favourite app: removes it from the favourites.
ring-star-remove = Remove from favourites
# Tip of the star on the menu of an app: adds it to the favourites.
ring-star-add = Add to favourites

ring-startup-add = Start at login

ring-startup-remove = Stop starting at login
# Screen reader name of the red button that shows why an uninstall failed.
ring-not-done = Not done
# Note on an app's menu when the middle of the ring has no room for another favourite; the number is how many it holds.
ring-middle-full =
    The middle holds { $n }.
    Remove one to make room.
# Choice on the ring's power page: lock the screen.
ring-power-lock = Lock
# Choice on the ring's power page: start the screensaver.
ring-power-screensaver = Screensaver
# Choice on the ring's power page: put the computer to sleep.
ring-power-suspend = Suspend
# Choice on the ring's power page: hibernate.
ring-power-hibernate = Hibernate
# Choice on the ring's power page: log out.
ring-power-log-out = Log out
# Choice on the ring's power page: restart the computer.
ring-power-restart = Restart
# Choice on the ring's power page: turn the computer off.
ring-power-shut-down = Shut down
# Grey line in the middle of the ring when there are no favourites.
ring-empty-hint = Click All apps, then right-click any app to keep it here
# Line in the ring when a spoken or typed power choice needs a click to confirm.
ring-power-only-click = Only a click confirms this.
# Line in the ring when something other than the person tried to shut the computer down.
ring-power-only-own = Only you can shut this computer down.
# Line in the ring when the power page closed because a program pressed a key.
ring-power-closed = The power page closed: a program pressed a key, not you.

# Notice from the new ring after the parts were restarted from the keyboard.
ring-restarted = Restarted
# Under the notice above.
ring-restarted-why = The ring and the helper parts started again.
