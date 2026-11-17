# The About this computer page of Settings: the computer's name, what it is made of, what runs on it and how long it takes to start.

## About (src/settings/panels/about.rs)

# Name of the About page in the Settings side bar and its title.
settings-about-title = About this computer
# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = hostname computer name network name about system version debian trindle memory ram processor cpu disk graphics screen model kernel rolling release activity running programs busy task manager system monitor disks drives usb space updates upgrade check automatic automatically security fixes
# Text behind the info button of the About page.
settings-about-help = This page shows this computer's name as other computers on the network see it, what it is made of, and how long it takes to start. Activity shows what the computer is doing right now and which programs use the memory and processor. Drives shows your drives and their space. Each opens in its own window. Use the arrows in More detail to open them.
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
settings-about-name-bad = Use letters, numbers and hyphens only. Don't start or end with a hyphen.
# Words under the computer name box while the password window is open.
settings-about-name-waiting = Waiting for your password…
# Red words under the computer name box when renaming failed.
settings-about-name-failed = Couldn't rename the computer.
# Words under the computer name box after renaming worked.
settings-about-name-done = Programs that are already open will use the new name after you next log in.
# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Computer name
# Tip of the save button beside the computer name box.
settings-about-rename = Rename this computer
# Bold line of the computer name setting.
settings-about-name = Computer name
# Heading of the card with the computer's name.
settings-about-name-group = Name
# Text behind the info button of the Name card.
settings-about-name-about = This is the computer's name on the network and at the prompt in a Terminal. Use letters, numbers and hyphens, but don't start or end with a hyphen. You need to enter your password to change it.
# Heading of the card about the computer's hardware.
settings-about-hardware = The computer
# Text behind the info button of the hardware card.
settings-about-hardware-about = This shows the computer's maker and model, its processor and its graphics. It also shows how many threads the processor can run at once, and each screen in use with its size and refresh rate.
# Heading of the card with the switch for looking for updates.
settings-about-updates = Updates
# Text behind the info button of the updates card.
settings-about-updates-about = Trindle checks for new versions of apps, plugins, firmware and itself soon after you log in, then every few hours. If you switch this off, Trindle only checks when you press Check for updates in Apps, or the updates button on the ring. Debian's security fixes install every day and have their own switch. If you switch that off, they no longer install by themselves, and known security holes stay unfixed until you update. Changing either switch needs your administrator's password, so the whole computer uses your choice.
# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Check for updates
# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Waiting for your password…
# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Security updates
# Warning line under the Security updates switch when it is off.
settings-about-security-off = Security fixes no longer install by themselves.
# Heading of the card about the software.
settings-about-software = The software
# Text behind the info button of the software card.
settings-about-software-about = Trindle is a desktop that runs on top of Debian, which looks after the programs underneath. The kernel is the part that works with the hardware. Trindle's own compositor draws the desktop, using Wayland.
# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Trindle's own compositor
# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } app
       *[other] { $n } apps
    }
# Heading of the card with the ways to Activity and Drives.
settings-about-more = More detail
# Text behind the info button of the More detail card.
settings-about-more-about = Activity shows what the computer is doing right now and which programs use the memory and processor. Drives shows every drive, how full and healthy it is, and any USB sticks.
# Bold line of the setting that opens Activity.
settings-about-activity = See what is running
# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Activity
# Bold line of the setting that opens Drives.
settings-about-drives = Drives and space
# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Drives
# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Trindle couldn't save the start-up times to Downloads.
# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = Date and time,Seconds
# File name of the saved start-up times, in Downloads.
settings-about-times-file = Trindle start-up times.csv
# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = In Downloads, as { $file }
# Label of the word button that saves every start-up time.
settings-about-times-save = Save
# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Save every start-up time to Downloads
# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Save each start time
# Added after the words behind the i of its card.
settings-about-times-keep-sub = Saved to Downloads as a spreadsheet.
# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Each column is one start. The newest is on the right.
# Heading of the card with the last ten start-up times.
settings-about-times = Last ten start-ups
# Text behind the info button of the start-up times card.
settings-about-times-about = Trindle times each start-up from the power button to the login screen. Time spent typing the disk password isn't counted. The newest is at the top.
# Heading of the memory tile.
settings-about-memory = Memory
# Heading of the disk tile.
settings-about-disk = Disk

## Problem reports card (src/settings/panels/report_card.rs)

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = Problem reports
# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = Reports go to Buildmorphic, the community that helps build Trindle. You can join with just an email address. You earn credit and levels for problems you're first to report, even if you can't fix them. A report contains only a fixed list of details, and nothing is sent until you press Send. A username or GitHub name is optional, and your username is never your email address.
# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = Offer to send a report when something goes wrong
# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = Buildmorphic username
# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = GitHub name
# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = About the community
# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
        [one] Off. { $n } waiting report deleted.
       *[other] Off. { $n } waiting reports deleted.
    }
# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
        [one] Off. { $n } waiting report could not be deleted.
       *[other] Off. { $n } waiting reports could not be deleted.
    }
# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = Off, but the Reporting add-on's waiting reports weren't deleted.
