# Settings: the Permissions page (src/settings/panels/permissions.rs).

## The Permissions page (src/settings/panels/permissions.rs)

# Name of the Permissions page, in the page list and as its heading.
settings-permissions-title = Permissions
# Words people may search for to find the Permissions page, separated by spaces.
settings-permissions-keywords = permissions allow privacy flatpak flathub sandbox internet network files sound microphone camera bluetooth notifications plugins deb x11 xwayland older programs steam wine java screen terminal computer thunderbolt usb4 dock devices outside notice downloads downloaded
# Help line at the foot of the Permissions page.
settings-permissions-help = What each app may use. Apps from Flathub, programs installed in their own folder and plugins are each installed in their own folder, restricted by default. A filled icon is allowed; click it to change it. A change to a Flathub app takes effect the next time it starts. Thunderbolt and USB4 devices allowed always connect without asking; forget one to be asked again.
# Red words when a permission could not be changed and nothing more is known.
settings-permissions-not-changed = It could not be changed.
# Words beside a plugin when its change was saved but the plugin could not be started again.
settings-permissions-plugin-not-restarted = Saved, but the plugin could not be started again. The change takes effect the next time it starts.
# Said beside an app, plugin or switch once a permission change is saved.
settings-permissions-saved = Saved
# Help text of the AppImages card (the information button).
settings-permissions-loose-images = AppImages are each run in a box of their own, with a private home. A filled icon is allowed; click it to change it. A change takes effect the next time the AppImage starts.
# Help text of the Started from Folders card (the information button).
settings-permissions-loose-programs = Programs started from Folders are each run in a box of their own, with a private home. A filled icon is allowed; click it to change it. A change takes effect the next time the program starts.
# A permission's name and what is true with it off, one after another in a card's help text: { $title } is the permission, { $words } what it means.
settings-permissions-title-and-words = { $title }: { $words }
# Under the Allow older programs switch.
settings-permissions-older-why = Each is kept apart from the rest. Off, they do not open.
# Heading of the card with the switch for older programs.
settings-permissions-older = Older programs
# Label and tip of the switch that lets older programs open.
settings-permissions-allow-older = Allow older programs
# Help text on the Thunderbolt and USB4 card when the ports only carry screens.
settings-permissions-lets-in-screens = These ports only carry screens, so no device can reach this computer's memory and there is nothing to ask.
# Help text on the Thunderbolt and USB4 card when the ports only carry USB and screens.
settings-permissions-lets-in-usb = These ports only carry USB and screens, so no device can reach this computer's memory and there is nothing to ask.
# Help text on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-lets-in-all = These ports let every device connect by themselves, so there is nothing to ask. This is set in the computer's firmware.
# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry screens.
settings-permissions-short-screens = These ports only carry screens.
# Under Nothing to ask on the Thunderbolt and USB4 card, when the ports only carry USB and screens.
settings-permissions-short-usb = These ports only carry USB and screens.
# Under Nothing to ask on the Thunderbolt and USB4 card, when the firmware lets every device in.
settings-permissions-short-firmware = The firmware lets every device connect.
# Under a package's name: its version number is { $v }.
settings-permissions-version = Version { $v }
# Under a plugin's name when it asks for nothing: its version number is { $v }.
settings-permissions-version-asks-nothing = Version { $v }. It asks for nothing.
# Under a plugin's name: the words so far are { $sub }, then the ports it listens on in { $detail }.
settings-permissions-sub-listen = { $sub }. { $detail }
# Red words when a Thunderbolt or USB4 device could not be forgotten.
settings-permissions-not-forgotten = It could not be forgotten.
# Heading of the Thunderbolt and USB4 card.
settings-permissions-thunder = Thunderbolt and USB4
# Help text of the Thunderbolt and USB4 card: { $about } is a further sentence, or nothing.
settings-permissions-thunder-about = Thunderbolt and USB4 devices allowed always connect without asking; forget one to be asked again. { $about }
# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-ports = The ports
# Under The ports while the Thunderbolt and USB4 card is read.
settings-permissions-reading-ports = Reading what they let in…
# Heading of a line on the Thunderbolt and USB4 card.
settings-permissions-devices-allowed = Devices allowed
# Under a line of the Thunderbolt and USB4 card while it is read.
settings-permissions-reading = Reading…
# Heading of a line on the Thunderbolt and USB4 card while it is read.
settings-permissions-memory-apart = Memory kept apart
# Heading of the line on the Thunderbolt and USB4 card when the computer has no such ports.
settings-permissions-no-ports = No Thunderbolt or USB4 ports
# Under the heading when the computer has no Thunderbolt or USB4 ports.
settings-permissions-nothing-to-ask = So there is nothing to ask.
# Under Devices allowed when the computer has no Thunderbolt or USB4 ports.
settings-permissions-none-plug = None: nothing plugs in this way.
# Heading of the line on the Thunderbolt and USB4 card when the ports let every device in.
settings-permissions-nothing-ask = Nothing to ask
# Under Devices allowed when the ports let every device in themselves.
settings-permissions-every-one = Every one, by the ports themselves.
# Help text of the Thunderbolt and USB4 card when the computer keeps devices away from memory.
settings-permissions-iommu = This computer also keeps devices away from memory they were not given, so the risk is smaller. Forgetting a device leaves it connected until it is unplugged.
# Help text of the Thunderbolt and USB4 card when the computer does not keep devices away from memory.
settings-permissions-no-iommu = Forgetting a device leaves it connected until it is unplugged; next time it is asked about again.
# Heading of the line on the Thunderbolt and USB4 card that lists the devices.
settings-permissions-asked-before = Asked before a new device connects
# Under the heading above.
settings-permissions-forget-one = Forget one to be asked again.
# Red words when the list of allowed Thunderbolt devices could not be read: the reason is { $why }.
settings-permissions-devices-not-read = The allowed devices could not be read: { $why }
# Shown when no Thunderbolt or USB4 device has been allowed always.
settings-permissions-no-device = No device may connect without asking yet.
# Tip of the button that forgets an allowed Thunderbolt or USB4 device.
settings-permissions-forget-device = Forget this device
# Name given to a Thunderbolt or USB4 device that has none.
settings-permissions-a-device = A Thunderbolt device
# Under an allowed device's name when its maker is not known.
settings-permissions-may-always = May always connect
# Under an allowed device's name: the maker is { $vendor }.
settings-permissions-made-by = Made by { $vendor }. May always connect.
# Headline while the list of apps is being read.
settings-permissions-reading-apps = Reading what each app may use…
# Headline while Flatpak is still answering: how many apps are listed.
settings-permissions-reading-each =
    { $n ->
        [one] Reading what each of { $n } may use…
       *[other] Reading what each of { $n } may use…
    }
# Headline when no app or plugin is kept in a box of its own.
settings-permissions-no-apps = No apps or plugins kept in a box of their own are installed.
# Headline when exactly one app is kept in a box of its own.
settings-permissions-one-app = One app is kept in a box of its own and uses only what is allowed here.
# Headline: how many apps and plugins are kept in a box of their own.
settings-permissions-many-apps =
    { $n ->
        [one] { $n } app or plugin is kept in a box of its own, using only what is allowed here.
       *[other] { $n } apps and plugins are each kept in a box of their own, using only what is allowed here.
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
# Under a kind of app on the Permissions page when none is installed.
settings-permissions-none-installed = None installed.
# Under a kind of app on the Permissions page when none has been run.
settings-permissions-none-run = None run yet.
settings-permissions-not-added = Run from Folders, not added to Apps
settings-permissions-kind-tip = Kind of app
# Heading of the card that says nothing is installed of any kind.
# Help text of the Flathub apps card (the information button).
settings-permissions-flathub-about = Apps from Flathub are each kept in a box of their own and may use only what is allowed here. A filled icon is allowed; click it to change it. A change takes effect the next time the app starts.
# Help text of the In their own folder card (the information button).
settings-permissions-debs-about = Programs that came with Trindle, such as the Terminal, and programs you installed from .deb files. Each is kept in its own folder and restricted by default. A filled icon is allowed; click it to change it.
# Help text of the Plugins card (the information button).
settings-permissions-plugins-about = Plugins are restricted by default and may only be allowed what they ask for. A plugin running in the background starts again at once with the change.
# Typing codes card: every short code Trindle swaps for words as you type.
settings-permissions-codes = Typing codes
settings-permissions-codes-about = Short codes Trindle swaps for words as you type, each with the plugin that owns it. Switch a plugin's codes off with its round icon.
settings-permissions-codes-none = No codes are active
settings-permissions-codes-none-sub = A plugin allowed to expand codes lists them here.
# Line on the typing codes card when the plugins folder could not be read: the list may be missing codes.
settings-permissions-codes-unread-line = The plugins folder could not be read
# Red words under that line.
settings-permissions-codes-unread = The plugins folder could not be read, so some codes may be missing.
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
settings-outside-about = Trindle looks every few seconds and tells you. It cannot stop these programs, and it can miss one that is quick. A program in a box is held by its box.
# Label of the switch for the notice about the network.
settings-outside-network = Say when a program reaches the network
# Label of the switch for the notice about the person's files.
settings-outside-files = Say when a program has your files open
# Under those two switches.
settings-outside-once = Once for each program.
# Tip of the round button that makes a program be said again; { $name } is the program.
settings-outside-forget = Say it again for { $name }
# Under a program marked fine for the network.
settings-outside-fine-network = Reaching the network is fine.
# Under a program marked fine for the person's files.
settings-outside-fine-files = Having your files open is fine.
# The line in the card when no program is marked fine.
settings-outside-no-fine = No program is marked fine.
# Heading of the card for downloaded files.
settings-downloads-heading = Downloaded files
# Help text of that card (the information button).
settings-downloads-about = A file from the internet is one in Downloads, or one the browser marked. The program that opens it gets that file only, read only, with no internet and none of your folders. This does not cover a browser's own Open button, which starts the program itself.
# Label of the switch that opens downloaded files in a box.
settings-downloads-box = Open downloaded files in a box
# Under that switch.
settings-downloads-box-sub = No internet, and only that file, read only.
# Tip of the switch that opens one type of downloaded file as normal; { $kind } is the type, such as pdf.
settings-downloads-normal = Open { $kind } files as normal
# Name of a file type in the list; { $kind } is the ending, such as pdf.
settings-downloads-type = { $kind } files
# Under a file type's switch.
settings-downloads-normal-sub = On: opens as normal, with the internet.
# The line in the card when no downloaded file has been opened in a box yet.
settings-downloads-none = No type yet. One is listed when a downloaded file opens in a box.
