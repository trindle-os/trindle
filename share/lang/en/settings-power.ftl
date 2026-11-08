# Settings page "Power": battery, power mode, screen dimming and locking, PIN, log out and shut down; also the PIN time names from src/common/pin.rs.

## Page (src/settings/panels/power.rs)

# Name of the Settings page about power, in the list and as its heading.
settings-power-title = Power
# Words a person might type into the Settings search to find the power page; separate them with spaces.
settings-power-keywords = battery suspend sleep shut down restart log out screen off idle dim lock password pin keypad caffeine keep awake charge charging limit 80 battery health
# Introduction under the heading of the power page, and the words behind the page i.
settings-power-help = Battery, power mode, and when the screen dims, locks and turns off, on a laptop separately for its battery and for the charger. The cup in the ring keeps the screen awake. Lock, restart and shut down are here too.

## Times in the lists

# Choice in a list of how long to wait.
settings-power-time-never = Never
# Choice in a list of how long to wait.
settings-power-time-1m = 1 minute
# Choice in a list of how long to wait.
settings-power-time-2m = 2 minutes
# Choice in a list of how long to wait.
settings-power-time-5m = 5 minutes
# Choice in a list of how long to wait.
settings-power-time-10m = 10 minutes
# Choice in a list of how long to wait.
settings-power-time-15m = 15 minutes
# Choice in a list of how long to wait.
settings-power-time-30m = 30 minutes
# Choice in a list of how long to wait.
settings-power-time-45m = 45 minutes
# Choice in a list of how long to wait.
settings-power-time-1h = 1 hour
# Choice in a list of how long to wait.
settings-power-time-90m = 1½ hours
# Choice in a list of how long to wait.
settings-power-time-2h = 2 hours
# Choice in the lock after list: lock at no other time than when the computer sleeps.
settings-power-time-only-sleep = Only when it sleeps

## Session buttons and lid

# Heading of the line, and name of its round button, that suspends the computer.
settings-power-suspend = Suspend
# Heading of the line, and name of its round button, that restarts the computer.
settings-power-restart = Restart
# Heading of the line, and name of its round button, that shuts the computer down.
settings-power-shut-down = Shut down
# Choice in the list of what closing the lid does: the computer sleeps.
settings-power-lid-sleep = Sleep
# Choice in the list of what closing the lid does: the computer hibernates.
settings-power-lid-hibernate = Hibernate
# Choice in the list of what closing the lid does: the screen locks.
settings-power-lid-lock = Lock the screen
# Choice in the list of what closing the lid does: nothing.
settings-power-lid-nothing = Nothing
# Choice in the list of what closing the lid does: the computer shuts down.
settings-power-lid-shut-down = Shut down

## Idle timers

# Heading of an idle timer line, and the name of its lists.
settings-power-timer-dim = Dim the screen after
# Small words under an idle timer line.
settings-power-timer-dim-about = Any movement brings the brightness back.
# Heading of an idle timer line, and the name of its lists.
settings-power-timer-clock = Show the moving clock after
# Small words under an idle timer line.
settings-power-timer-clock-about = The time alone, moving slowly about.
# Heading of an idle timer line, and the name of its lists.
settings-power-timer-off = Turn the screen off after
# Small words under an idle timer line.
settings-power-timer-off-about = Any key or movement turns it on again.
# Heading of an idle timer line, and the name of its lists.
settings-power-timer-suspend = Suspend after
# Small words under an idle timer line.
settings-power-timer-suspend-about = Left alone this long, it goes to sleep.

## Warnings and states

# Red words under the keep the screen awake switch when turning it on could not be saved.
settings-power-no-awake = The screen could not be kept awake: the choice could not be saved.
# Red words under the keep the screen awake switch when turning it off could not be saved.
settings-power-no-sleep = Keeping the screen awake could not be turned off: the choice could not be saved.
# Name of the power mode that saves the battery.
settings-power-profile-saver = Battery saver
# Name of the balanced power mode.
settings-power-profile-balanced = Balanced
# Name of the power mode that favours speed.
settings-power-profile-performance = Performance
# What the battery is doing: charging.
settings-power-state-charging = Charging
# What the battery is doing: running on its own charge.
settings-power-state-battery = On battery
# What the battery is doing: empty.
settings-power-state-empty = Empty
# What the battery is doing: full.
settings-power-state-full = Full
# What the battery is doing: the charger is in but it is not charging.
settings-power-state-plugged = Plugged in, not charging
# Time left on the battery; { $h } hours and { $m } minutes. The h and min are short for hours and minutes.
settings-power-hours-minutes = { $h } h { $m } min
# Time left on the battery when under an hour; { $m } minutes. The min is short for minutes.
settings-power-minutes = { $m } min

## Top sentence

# Sentence at the top of the page on a computer with no battery.
settings-power-top-no-battery = Plugged in: this computer has no battery to look after.
# Sentence at the top of the page on a computer with no battery; { $mode } is the power mode.
settings-power-top-no-battery-mode = Plugged in, in { $mode } mode: no battery to look after.
# Sentence at the top of the page while charging; { $percent } is how full.
settings-power-top-charging = Charging, { $percent }%.
# Sentence at the top of the page while charging; { $percent } is how full and { $left } how long until full.
settings-power-top-charging-left = Charging, { $percent }%, full in { $left }.
# Sentence at the top of the page when the battery is full.
settings-power-top-full = The battery is full.
# Sentence at the top of the page when plugged in but not charging; { $percent } is how full.
settings-power-top-plugged = Plugged in at { $percent }%, not charging.
# Sentence at the top of the page on battery; { $percent } is how full.
settings-power-top-battery = Battery at { $percent }%.
# Sentence at the top of the page on battery; { $percent } is how full and { $left } how long is left.
settings-power-top-battery-left = Battery at { $percent }%, about { $left } left.
# Sentence at the top of the page while the battery is being read.
settings-power-top-looking = Looking at the battery…

## Power mode and battery card

# Red words under the power mode line when the mode could not be changed.
settings-power-mode-failed = The power mode could not be changed.
# Heading of the power mode line, and the name of its list.
settings-power-mode = Power mode
# Small words under the power mode line.
settings-power-mode-about = Saver runs cooler; performance faster.
# Said in green under a line after its choice was kept.
settings-power-saved = Saved
# Name of the list of battery levels to warn at.
settings-power-warn-name = Warn when the battery is down to
# Heading of the line for the battery warning.
settings-power-warn = Warn when it is down to
# Small words under the battery warning line.
settings-power-warn-about = Once, in the middle of the screen.
# Choice in the list of battery levels: never warn.
settings-power-never = Never
# Heading of the line, and name of its switch, for stopping charging at 80%.
settings-power-limit = Stop charging at 80%
# Small words under the charge limit line.
settings-power-limit-about = Kinder to a battery mostly plugged in.
# Title of the battery card on a laptop.
settings-power-card-battery = Battery
# Title of the battery card when there is no battery picture and no power modes.
settings-power-card-looking-after = Looking after the battery
# Words behind the i of the battery card.
settings-power-battery-about = Battery saver runs cooler and makes the battery last longer; performance runs faster; balanced sits between the two. The warning appears in the middle of the screen once, while the battery runs down. Stopping at 80% is kinder to the battery of a laptop that is mostly plugged in: it charges to 80% and no further, and lasts more years. Turn it off before a day away from a socket.
# Words under the charge limit line while the password window is open.
settings-power-limit-waiting = Waiting for the password...
# Green words under the charge limit line after charging was limited to 80%.
settings-power-limit-on = Stops at 80%
# Green words under the charge limit line after the limit was removed.
settings-power-limit-off = Charges to full

## Screen cards

# Heading of the keep the screen awake line, and name of its switch.
settings-power-awake = Keep the screen awake
# Small words under the keep the screen awake line.
settings-power-awake-about = The cup in the ring does the same.
# Title of the card about dimming and the moving clock.
settings-power-screen-title = Screen
# Words behind the i of the Screen card.
settings-power-screen-about = While Keep the screen awake is on, none of these happen; the cup in the ring turns it on and off too. Dimming: any movement brings the brightness back; only screens with their own brightness, such as a laptop's, dim. The moving clock: the screen goes dark with just the time on it, moving slowly about, and any key or movement brings everything back. On a laptop each time is set twice, for its battery and for the charger.
# Title of the card about turning the screen off and sleeping.
settings-power-sleep-title = Screen off and sleep
# Words behind the i of the Screen off and sleep card.
settings-power-sleep-about = Any key or movement turns the screen on again. Suspending puts the computer to sleep until you wake it. On a laptop each time is set twice, for its battery and for the charger. With a second screen plugged in, closing the lid does nothing, so you can carry on with the bigger screen.
# Heading of the line for what closing the lid does.
settings-power-lid = When the lid is closed
# Small words under that line.
settings-power-lid-about = Nothing happens with a second screen in.

## Battery or charger captions

# Caption above a list that applies while the laptop runs on its battery.
settings-power-when-battery = On battery
# Caption above a list that applies while the laptop is plugged in.
settings-power-when-charger = Plugged in
# Name of a list that applies on battery; { $title } is the name of its line.
settings-power-named-battery = { $title }: on battery
# Name of a list that applies while plugged in; { $title } is the name of its line.
settings-power-named-charger = { $title }: plugged in

## Locking

# Heading of the lock the screen line, and name of its switch.
settings-power-lock = Lock the screen
# Small words under the lock the screen line.
settings-power-lock-about = Your password or a PIN gets you back in.
# Heading of the lock after line, and the name of its list.
settings-power-lock-after-name = Lock after
# Small words under the lock after line.
settings-power-lock-after-about = Left alone this long, whatever else is set.
# Heading of the line, and name of its switch, for locking when the computer sleeps.
settings-power-lock-sleep = Lock when the computer sleeps
# Small words under that line.
settings-power-lock-sleep-about = So it always wakes up locked.
# Heading of the line, and name of its button, that locks the screen now.
settings-power-lock-now = Lock now
# Small words under the lock now line.
settings-power-lock-now-about = Also Super and L together.
# Red words under the lock now line when the lock screen would not start.
settings-power-lock-now-failed = The screen could not be locked: the lock screen would not start.
# Title of the locking card.
settings-power-lock-title = Locking
# Words behind the i of the locking card.
settings-power-lock-card-about = When you are away, the screen locks, and your password, or a four-digit PIN if you choose one, gets you back in. Nothing on the screen shows until then. Logging in always needs your password. Lock after is how long the computer is left alone before it locks, whatever the screen and sleep times are; until then, coming back needs no PIN. Turning locking off also forgets the PIN. Four digits are quicker to type than your password: after the time chosen for the PIN locked, or after five wrong tries, your password is needed instead. The bin stops using a PIN.

## PIN

# Heading of the line for adding a PIN.
settings-power-pin-add = Unlock with a PIN as well
# Small words under that line.
settings-power-pin-add-about = Four digits, quicker than your password.
# Heading of the line, and name of its list, for how long a PIN works after the screen locks.
settings-power-pin-works-name = The PIN works for
# Small words under that line.
settings-power-pin-works-about = Then your password is needed instead.
# Name of the bin button that stops using a PIN.
settings-power-pin-stop = Stop using a PIN
# Heading of the line shown when a PIN is kept.
settings-power-pin-kept = A PIN is kept
# Small words under that line.
settings-power-pin-kept-about = Type four new digits to change it.
# Placeholder of the box for typing a new PIN.
settings-power-pin-new = New PIN
# Name of the button that keeps the PIN typed.
settings-power-pin-keep = Keep this PIN
# Red words under the PIN line when the PIN could not be removed.
settings-power-pin-not-removed = The PIN could not be removed, so it still unlocks the screen.
# Red words under the PIN box when it is not the right length; { $n } is how many digits a PIN has.
settings-power-pin-digits =
    { $n ->
        [one] A PIN is { $n } digit.
       *[other] A PIN is { $n } digits.
    }

## Session

# Heading of the log out line, and name of its round button.
settings-power-log-out = Log out
# Small words under the log out line.
settings-power-log-out-about = Back to the login screen.
# Title of the card with log out, suspend, restart and shut down.
settings-power-session-title = Session
# Words behind the i of the session card.
settings-power-session-about = Logging out closes every window and goes back to the login screen. Suspend puts the computer to sleep until you wake it. Each button asks twice, the same as the power choice in the ring, so a slip does nothing.
# Red words under a session button when the system bus is not there.
settings-power-no-bus = The system bus is not available.
# Red words under a session button when the computer would not do it.
settings-power-would-not = The computer would not do that.

## Pictures

# Spoken description of the battery picture while the battery is being read.
settings-power-art-looking-tip = Looking at the battery.
# Spoken description of the battery picture; { $percent } is how full and { $state } what it is doing, such as Charging.
settings-power-art-tip = Battery at { $percent }%. { $state }
# Word drawn in the empty battery picture while the battery is being read.
settings-power-art-looking = Looking
# Word drawn under the battery figure when its state is not known.
settings-power-art-battery = Battery
# Words drawn in the battery picture: { $left } is the time left and { $state } is either empty or full.
settings-power-art-left =
    { $state ->
        [empty] { $left } to empty
       *[full] { $left } to full
    }
# Spoken description of the battery range bar when there is no warning; { $top } is the percent it charges to.
settings-power-limits-tip = Charges to { $top }%; no warning.
# Spoken description of the battery range bar; { $top } is the percent it charges to and { $warn } where a warning comes.
settings-power-limits-tip-warn = Charges to { $top }%; a warning at { $warn }%.

## PIN times (src/common/pin.rs)

# A length of time in the list of how long a PIN works, in minutes; { $n } is the number.
settings-power-pin-minutes =
    { $n ->
        [one] { $n } minute
       *[other] { $n } minutes
    }
# A length of time in the list of how long a PIN works, in whole hours; { $n } is the number.
settings-power-pin-hours =
    { $n ->
        [one] { $n } hour
       *[other] { $n } hours
    }
# A length of time in the list of how long a PIN works, in hours and minutes; { $hours } and { $minutes } are the two parts already written out, such as 1 hour and 15 minutes.
settings-power-pin-hours-minutes = { $hours } { $minutes }
