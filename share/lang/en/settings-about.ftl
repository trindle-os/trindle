# The About this computer page of Settings: the computer's name, what it is made of, what runs on it and how long it takes to start.

## About (src/settings/panels/about.rs)

# Name of the About page in the Settings side bar and its title.
settings-about-title = About this computer
# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = hostname computer name network name about system version debian trindle memory ram processor cpu disk graphics screen model kernel rolling release activity running programs busy task manager system monitor disks drives usb space updates upgrade check automatic automatically security fixes
# Text behind the info button of the About page.
settings-about-help = This computer's name, as other computers on the network see it, what it is made of, and how long it takes to start. What it is doing right now, and which programs are using the memory and the processor, is in Activity, and the drives and the space on them in Drives, each a window of its own: the arrows in More detail open them.
# What is shown when a fact about the computer cannot be found.
settings-about-unknown = Unknown
# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
        [one] { $model }, { $cores } thread
       *[other] { $model }, { $cores } threads
    }
# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Intel graphics
# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = AMD graphics
# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = NVIDIA graphics
# The graphics line inside a virtual machine.
settings-about-gpu-virtual = Virtual machine graphics
# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = Graphics
# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height } at { $hz } Hz
# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = { $amount } in use
# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = { $amount } free
# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = of { $amount }
# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }, switched on for { $on }.
# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }, switched on for { $on }. The disk is nearly full.
# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n } seconds
# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n } minutes
# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
        [one] { $n } minute
       *[other] { $n } minutes
    }
# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
        [one] { $n } hour
       *[other] { $n } hours
    }
# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
        [one] { $n } day
       *[other] { $n } days
    }
# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = Letters, numbers and hyphens only, not starting or ending with a hyphen.
# Words under the computer name box while the password window is open.
settings-about-name-waiting = Waiting for the password...
# Red words under the computer name box when renaming failed.
settings-about-name-failed = The computer could not be renamed.
# Words under the computer name box after renaming worked.
settings-about-name-done = Programs already open see the new name after the next login.
# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Computer name
# Tip of the save button beside the computer name box.
settings-about-rename = Rename this computer
# Bold line of the computer name setting.
settings-about-name = Computer name
# Quiet line under the computer name setting.
settings-about-name-sub = Letters, numbers and hyphens.
# Heading of the card with the computer's name.
settings-about-name-group = Name
# Text behind the info button of the Name card.
settings-about-name-about = How this computer appears on the network, and at the prompt in a Terminal. Letters, numbers and hyphens, not starting or ending with a hyphen. Changing it asks for your password.
# Heading of the card about the computer's hardware.
settings-about-hardware = The computer
# Text behind the info button of the hardware card.
settings-about-hardware-about = The maker and model, the processor and how many threads it runs at once, the graphics, and each screen in use with its size and refresh rate.
# Quiet line under the maker and model.
settings-about-hw-computer = The computer
# Quiet line under the processor.
settings-about-hw-processor = The processor
# Quiet line under the graphics.
settings-about-hw-graphics = Draws the pictures
# Quiet line under the screens.
settings-about-hw-screen = The screen
# Heading of the card with the switch for looking for updates.
settings-about-updates = Updates
# Text behind the info button of the updates card.
settings-about-updates-about = Trindle looks for new versions of apps, plugins, firmware and itself soon after you log in and every few hours, and Debian's security fixes go in every day. Switched off, Trindle looks only when you press Check for updates in Apps, or the updates button on the ring. Debian's security fixes have their own switch; switched off, they no longer install by themselves and known faults stay open until you update. Changing either switch asks for your administrator's password, so the computer's own services see the choice too.
# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Check for updates
# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Waiting for the password...
# Quiet line under that switch.
settings-about-updates-by-itself-sub = Off, nothing looks unless you ask.
# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Security updates
# Quiet line under the Security updates switch when it is on.
settings-about-security-sub = Security fixes install by themselves every day.
# Warning line under the Security updates switch when it is off.
settings-about-security-off = Security fixes no longer install by themselves.
# Heading of the card about the software.
settings-about-software = The software
# Text behind the info button of the software card.
settings-about-software-about = Trindle is a desktop on top of Debian, which looks after the programs underneath. The kernel is the part that talks to the hardware, and Trindle's own compositor draws the desktop, on Wayland.
# Quiet line under the Debian version.
settings-about-sw-debian = Runs underneath
# Quiet line under the kernel version.
settings-about-sw-kernel = The kernel
# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindle's own compositor
# Quiet line under the compositor line.
settings-about-sw-compositor-sub = Draws the desktop, on Wayland
# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } app
       *[other] { $n } apps
    }
# Quiet line under the number of apps.
settings-about-sw-apps-sub = Installed, ready in the ring
# Heading of the card with the ways to Activity and Drives.
settings-about-more = More detail
# Text behind the info button of the More detail card.
settings-about-more-about = What it is doing right now, and which programs are using the memory and the processor, is in Activity. Every drive, how full and how healthy it is, and USB sticks, are in Drives.
# Bold line of the setting that opens Activity.
settings-about-activity = See what is running
# Quiet line under the setting that opens Activity.
settings-about-activity-sub = Programs using memory and the processor.
# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Activity
# Bold line of the setting that opens Drives.
settings-about-drives = Drives and space
# Quiet line under the setting that opens Drives.
settings-about-drives-sub = Every drive, how full and healthy it is.
# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Drives
# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = The start-up times could not be saved to Downloads.
# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = In Downloads, as { $file }
# Label of the word button that saves every start-up time.
settings-about-times-save = Save
# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Save every start-up time to Downloads
# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Every start, to keep
# Quiet line under that setting.
settings-about-times-keep-sub = Saved to Downloads as a spreadsheet.
# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Each column is one start, the newest on the right.
# Heading of the card with the last ten start-up times.
settings-about-times = The last ten starts
# Text behind the info button of the start-up times card.
settings-about-times-about = Each start is timed from the power button to the login screen, leaving out any time spent typing the disk password. The newest is at the top.
# Heading of the memory tile.
settings-about-memory = Memory
# Heading of the disk tile.
settings-about-disk = Disk
