# Settings: the Permissions page (src/settings/panels/permissions.rs).

## The Permissions page (src/settings/panels/permissions.rs)

# Name of the Permissions page, in the page list and as its heading.
settings-permissions-title = Permissions
# Words people may search for to find the Permissions page, separated by spaces.
settings-permissions-keywords = permissions allow privacy flatpak flathub sandbox internet network files sound microphone camera bluetooth notifications plugins deb x11 xwayland older programs steam wine java screen terminal computer thunderbolt usb4 dock devices outside notice downloads downloaded
# Help line at the foot of the Permissions page.
settings-permissions-help = This page shows what each app may use. Apps from Flathub, programs installed in their own folder and plugins each run in their own box, with limits set by default. A change to a Flathub app takes effect the next time it starts. Thunderbolt and USB4 devices you've allowed always connect without asking. Forget a device if you want to be asked again.
# Red words when a permission could not be changed and nothing more is known.
settings-permissions-not-changed = Couldn't change it.
# Words beside a plugin when its change was saved but the plugin could not be started again.
settings-permissions-plugin-not-restarted = Saved, but Trindle couldn't restart the plugin. The change takes effect the next time it starts.
# Said beside an app, plugin or switch once a permission change is saved.
settings-permissions-saved = Saved
# Help text of the AppImages card (the information button).
settings-permissions-loose-images = Each AppImage runs in its own box, with a private home folder. A change takes effect the next time the AppImage starts.
# Help text of the Started from Folders card (the information button).
settings-permissions-loose-programs = Each program you start from Folders runs in its own box, with a private home folder. A change takes effect the next time the program starts.
# A permission's name and what is true with it off, one after another in a card's help text: { $title } is the permission, { $words } what it means.
settings-permissions-title-and-words = { $title }: { $words }
# Under the Allow older programs switch.
settings-permissions-older-why = Each runs separately. If this is off, they won't open.
# Heading of the card with the switch for older programs.
settings-permissions-older = Older programs
# Label and tip of the switch that lets older programs open.
settings-permissions-allow-older = Allow older programs
# Help text on the Thunderbolt and USB4 card when the ports only carry screens.
settings-permissions-lets-in-screens = These ports are only for screens, so no device can reach this computer's memory. There is nothing to ask.
# Help text on the Thunderbolt and USB4 card when the ports only carry USB and screens.
settings-permissions-lets-in-usb = These ports are only for USB and screens, so no device can reach this computer's memory. There is nothing to ask.
# Help text on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-lets-in-all = These ports let every device connect, so there is nothing to ask. This is set in the computer's firmware.
# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry screens.
settings-permissions-short-screens = Only screens use these ports.
# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry USB and screens.
settings-permissions-short-usb = Only USB and screens use these ports.
# Under Nothing to ask on the Thunderbolt and USB4 card, when the firmware lets every device in.
settings-permissions-short-firmware = The firmware lets every device connect.
# Under a package's name: its version number is { $v }.
settings-permissions-version = Version { $v }
# Under a plugin's name when it asks for nothing: its version number is { $v }.
settings-permissions-version-asks-nothing = Version { $v }. It asks for nothing.
# Under a plugin's name: its version number is { $v }, and { $notes } lists what it may do that matters most, such as "it can act as you and it can type into your windows".
settings-permissions-version-notes = Version { $v }: { $notes }.
# Under a plugin's name: the words so far are { $sub }, then the ports it listens on in { $detail }.
settings-permissions-sub-listen = { $sub }. { $detail }
# Red words when a Thunderbolt or USB4 device could not be forgotten.
settings-permissions-not-forgotten = Couldn't forget it.
# Heading of the Thunderbolt and USB4 card.
settings-permissions-thunder = Thunderbolt and USB4
# Help text of the Thunderbolt and USB4 card: { $about } is a further sentence, or nothing.
settings-permissions-thunder-about = Thunderbolt and USB4 devices you've allowed always connect without asking. Forget a device if you want to be asked again. { $about }
# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-ports = Ports
# Under The ports while the Thunderbolt and USB4 card is read.
settings-permissions-reading-ports = Checking the ports…
# Heading of a line on the Thunderbolt and USB4 card.
settings-permissions-devices-allowed = Devices allowed
# Under a line of the Thunderbolt and USB4 card while it is read.
settings-permissions-reading = Loading…
# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-memory-apart = Memory protection
# Heading of the line on the Thunderbolt and USB4 card when the computer has no such ports.
settings-permissions-no-ports = No Thunderbolt or USB4 ports
# Under the heading when the computer has no Thunderbolt or USB4 ports.
settings-permissions-nothing-to-ask = So there is nothing to ask.
# Under Devices allowed when the computer has no Thunderbolt or USB4 ports.
settings-permissions-none-plug = No devices connect this way.
# Heading of the line on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-nothing-ask = Nothing to ask
# Under Devices allowed when the ports let every device in themselves.
settings-permissions-every-one = All of them. The ports decide.
# Help text of the Thunderbolt and USB4 card when the computer keeps devices away from memory.
settings-permissions-iommu = This computer also stops devices reaching memory that isn't theirs, so the risk is smaller. If you forget a device, it stays connected until you unplug it.
# Help text of the Thunderbolt and USB4 card when the computer does not keep devices away from memory.
settings-permissions-no-iommu = If you forget a device, it stays connected until you unplug it. The next time you plug it in, you'll be asked again.
# Heading of the line on the Thunderbolt and USB4 card that lists the devices.
settings-permissions-asked-before = You're asked about new devices
# Red words when the list of allowed Thunderbolt devices could not be read: the reason is { $why }.
settings-permissions-devices-not-read = Trindle couldn't read the list of allowed devices: { $why }
# Shown when no Thunderbolt or USB4 device has been allowed always.
settings-permissions-no-device = No device is allowed to connect without asking yet.
# Tip of the button that forgets an allowed Thunderbolt or USB4 device.
settings-permissions-forget-device = Forget this device
# Name given to a Thunderbolt or USB4 device that has none.
settings-permissions-a-device = A Thunderbolt device
# Under an allowed device's name when its maker is not known.
settings-permissions-may-always = May always connect
# Under an allowed device's name: the maker is { $vendor }.
settings-permissions-made-by = Made by { $vendor }. May always connect.
# Headline while the list of apps is being read.
settings-permissions-reading-apps = Checking what apps may use…
# Headline while Flatpak is still answering: how many apps are listed.
settings-permissions-reading-each =
    { $n ->
        [one] Checking what { $n } app may use…
       *[other] Checking what { $n } apps may use…
    }
# Headline when no app or plugin is kept in a box of its own.
settings-permissions-no-apps = You haven't installed any apps or plugins that run in their own box.
# Headline when exactly one app is kept in a box of its own.
settings-permissions-one-app = One app runs in its own box and can only use what you allow here.
# Headline: how many apps and plugins are kept in a box of their own.
settings-permissions-many-apps =
    { $n ->
        [one] { $n } app or plugin runs in its own box and can only use what you allow here.
       *[other] { $n } apps and plugins each run in their own box and can only use what you allow here.
    }
# Heading of the card for apps from Flathub.
settings-permissions-flathub-apps = Flathub apps
# Heading of the card for programs installed in their own folder.
settings-permissions-own-folder = Programs
# Heading of the card for plugins.
settings-permissions-plugins = Plugins
# Heading of the card for AppImages.
settings-permissions-appimages = AppImages
# Heading of the card for programs started from Folders.
settings-permissions-from-folders = Started from Folders
settings-permissions-other = Other settings
# Under a kind of app on the Permissions page when none is installed.
settings-permissions-none-installed = None installed.
# Under a kind of app on the Permissions page when none has been run.
settings-permissions-none-run = None yet.
settings-permissions-not-added = Opened from Folders, not in Apps
settings-permissions-kind-tip = Kind of app
# Heading of the card that says nothing is installed of any kind.
# Help text of the Flathub apps card (the information button).
settings-permissions-flathub-about = Each app from Flathub runs in its own box and may use only what is allowed here. A change takes effect the next time the app starts.
# Help text of the In their own folder card (the information button).
settings-permissions-debs-about = Programs that came with Trindle, such as the Terminal, and programs you installed from .deb files. Each one runs in its own folder, with limits set by default.
# Help text of the Plugins card (the information button).
settings-permissions-plugins-about = Plugins start with limits, and you can only allow what a plugin asks for. A plugin that runs in the background restarts straight away when you change something.
# Typing codes card: every short code Trindle swaps for words as you type.
settings-permissions-codes = Typing codes
settings-permissions-codes-about = Short codes that Trindle swaps for words as you type. Each one shows the plugin it comes from.
settings-permissions-codes-none = No codes are active
# Line on the typing codes card when the plugins folder could not be read: the list may be missing codes.
settings-permissions-codes-unread-line = Couldn't read the plugins folder
# Red words under that line.
settings-permissions-codes-unread = Trindle couldn't read the plugins folder, so some codes may be missing.
# Button on a plugin's row: stops that plugin's codes.
settings-permissions-codes-off = Switch off { $name }'s codes
settings-permissions-codes-count =
    { $n ->
        [one] One code, { $id }
       *[other] { $n } codes, { $id }
    }
# A code's second line: what it types and who owns it.
settings-permissions-code-types = Types "{ $words }" for { $name }
settings-permissions-codes-more =
    { $n ->
        [one] One more code is active
       *[other] { $n } more codes are active
    }

## Programs outside a box, and downloaded files (src/settings/panels/outside_card.rs)

# Heading of the card for programs that are not in a box.
settings-outside-heading = Programs outside a box
# Help text of that card (the information button): what Trindle can and cannot do for them.
settings-outside-about = Trindle checks every few seconds and tells you what it finds. It can't stop these programs, and it may miss one that only runs for a moment. Programs in a box can only use what their box allows.
# Label of the switch for the notice about the network.
settings-outside-network = Tell me when programs use the network
# Label of the switch for the notice about the person's files.
settings-outside-files = Tell me when programs open my files
# Under those two switches.
settings-outside-once = Once for each program.
# Tip of the round button that makes a program be said again; { $name } is the program.
settings-outside-forget = Stop ignoring { $name }
# Under a program marked fine for the network.
settings-outside-fine-network = Using the network is fine.
# Under a program marked fine for the person's files.
settings-outside-fine-files = Having your files open is fine.
# The line in the card when no program is marked fine.
settings-outside-no-fine = No program is marked fine.
# Heading of the card for downloaded files.
settings-downloads-heading = Downloaded files
# Help text of that card (the information button).
settings-downloads-about = Files in Downloads, and files your browser has marked as downloaded, open in a box. The program that opens one gets only that file. It can only read it, and it can't use the internet or your folders. This doesn't cover a browser's own Open button, which starts the program itself.
# Label of the switch that opens downloaded files in a box.
settings-downloads-box = Open downloaded files in a box
# Tip of the switch that opens one type of downloaded file as normal; { $kind } is the type, such as pdf.
settings-downloads-normal = Open { $kind } files as normal
# Name of a file type in the list; { $kind } is the ending, such as pdf.
settings-downloads-type = { $kind } files
# Under a file type's switch.
settings-downloads-normal-sub = They open as normal, with the internet.
# The line in the card when no downloaded file has been opened in a box yet.
settings-downloads-none = No file types yet. A type shows here once a downloaded file has opened in a box.
