# The page shown before a Flatpak program is installed, with what it will be able to reach.
# Headline over the page; the source of the program is given.
store-fp-headline-from = Install { $name } from { $host }?
# Headline over the page for a program that has been downloaded.
store-fp-headline = Install { $name }?
# Name of the card that lists what the program will be able to reach.
store-fp-card = What it will be able to reach
# Tooltip of the round button that leaves without installing.
store-fp-cancel = Don't install
# The main button.
store-fp-install = Install
# Said while Flatpak is being asked what the program wants.
store-fp-reading = Reading what it asks for…
# Red words when what the program asks for could not be read.
store-fp-unread = Trindle couldn't read what it asks for.
# Red words after Install was pressed once for a program whose list could not be read.
store-fp-unread-again = Press Install again to go ahead.
# Said when the program is already on this computer; the name is given.
store-fp-have = { $name } is already installed.
# Said when the program asks to reach nothing outside its own box.
store-fp-none = Nothing outside its own box.
# Each line below is one thing the program will be able to reach.
store-fp-network = The internet and your local network
store-fp-x11 = Windows of other apps, and what you type in them
store-fp-sound = Sound, and your microphone
store-fp-devices = Your camera and every other plugged-in device
store-fp-input = Game controllers and other input devices
store-fp-kvm = Fast virtual machines
store-fp-shm = Memory shared with other apps
store-fp-usb = USB devices
store-fp-canbus = CAN bus devices
store-fp-cups = Your printers
store-fp-multiarch = Running 32-bit programs
store-fp-escape = Run any program on this computer
# A reference that also adds a second place to get apps from, named by its address.
store-fp-adds-source = Add { $host } to your app sources
# Last line, when the program asks for more than the lines above say.
store-fp-other = Plus other settings
store-fp-bluetooth = Bluetooth devices
store-fp-smartcard = Smart cards
store-fp-ssh = Your SSH keys
store-fp-gpg = Your GPG keys
store-fp-devel = Debugging tools, which let it watch and control programs it starts
store-fp-session-bus = Every program running in your session
store-fp-system-bus = Every service on this computer
# A place the program can open and change; the place is given, as below.
store-fp-files = Open and change { $what }
# The same for a place it can only look at.
store-fp-files-read = Look at { $what }
store-fp-where-host = every file on this computer
store-fp-where-system = this computer's system files
store-fp-where-home = your whole home folder, hidden files included
store-fp-where-path = the folder { $path }
# A program on your session that it can talk to; the name is given.
store-fp-talk = Talk to { $name }
# A service the program runs on your session.
store-fp-own = Run the service { $name }
store-fp-talk-system = Talk to the system service { $name }
store-fp-own-system = Run the system service { $name }
# Said in red when Install failed and taking the program out again failed too.
store-fp-undo-failed = It may be partly installed.
# Said in red when a bundle could not be copied for a safe read.
store-fp-copy-failed = Couldn't copy the file to read it safely.
