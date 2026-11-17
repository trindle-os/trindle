# The Help page's topics: titles, search words, paragraphs, tips, steps and picture captions. Part of Settings' own words (see settings-help.ftl).


## Topic: AI programs (src/settings/help_topics.rs)

# Title of the Help topic "AI programs".
settings-help-topics-ai-programs-title = AI programs
# Words people might search for the Help topic "AI programs" by, in lower case, separated by spaces.
settings-help-topics-ai-programs-keywords = ai claude codex cursor gemini aider coding agent fence private files share folder ssh keys sudo assistant mcp flathub restart cache
# Help topic "AI programs": paragraph 1.
settings-help-topics-ai-programs-p1 = This keeps AI programs away from your personal files. For example, it stops one that has been tricked into collecting your bank statements and sending them out. It's limited protection, not a guarantee. A determined program, a flaw in the computer's software, or you approving a request or sharing a folder can still let something through.
# Help topic "AI programs": paragraph 2.
settings-help-topics-ai-programs-p2 = A program running inside the fence sees an empty home folder. It can't read your documents, ssh keys, saved passwords or browser logins. It sees only the folders you share, in the same place as yours. It can still see the rest of the system, so it can read logs and run commands. Everything it starts, including programs it installs, stays inside the fence.
# Help topic "AI programs": step 1 of the numbered steps at block 3.
settings-help-topics-ai-programs-step3-1 = To share a folder, right-click it in Folders and choose Share with AI programs, or use Share a folder on the AI programs page.
# Help topic "AI programs": step 2 of the numbered steps at block 3.
settings-help-topics-ai-programs-step3-2 = You can never share hidden folders, or folders that hold keys, passwords or mail.
# Help topic "AI programs": step 3 of the numbered steps at block 3.
settings-help-topics-ai-programs-step3-3 = To reach another computer by ssh, add it on the page. Trindle uses a separate key, which works only for the computers you list.
# Help topic "AI programs": step 4 of the numbered steps at block 3.
settings-help-topics-ai-programs-step3-4 = If a command needs administrator rights, Trindle opens a window outside the fence and waits for you.
# Help topic "AI programs": paragraph 4.
settings-help-topics-ai-programs-p4 = Only the programs listed on the page start inside the fence. A program you start some other way isn't fenced. If you turn the fence off on the page, AI programs can see everything you can. If you've installed the assistant connection (trindle-mcp), a connected assistant can read some things without asking you. It can read your open windows, installed apps, running programs, Trindle settings, system logs, recent crash notes and devices. It can also read how your memory, disks, battery and network are doing. For an app you installed from Flathub, it can read the names of its settings, but never what they are set to. It asks you before it changes anything. If a Flathub app is misbehaving, it can restart it, change one of its settings or clear its cache, each only after asking you. You can ask it to undo a changed setting or a cleared cache, but a restart can't be undone.

## Topic: Welcome to Trindle (src/settings/help_topics.rs)

# Title of the Help topic "Welcome to Trindle".
settings-help-topics-welcome-title = Welcome to Trindle
# Words people might search for the Help topic "Welcome to Trindle" by, in lower case, separated by spaces.
settings-help-topics-welcome-keywords = start begin new first introduction what is how edge open windows
# Help topic "Welcome to Trindle": paragraph 1.
settings-help-topics-welcome-p1 = Trindle keeps your screen clear. There's no bar or menu along the edges, just your windows and your wallpaper. Everything else is in one circle, called the ring. It appears when you ask for it and goes away when you've finished.
# Help topic "Welcome to Trindle": caption and spoken name of picture 2 (ring-first.png).
settings-help-topics-welcome-pic2 = The ring, with the time and date in the middle
# Help topic "Welcome to Trindle": paragraph 3.
settings-help-topics-welcome-p3 = You can do everything with the mouse or a finger. Here's how to open the ring, where your windows are:
# Help topic "Welcome to Trindle": step 1 of the numbered steps at block 4.
settings-help-topics-welcome-step4-1 = Move the pointer to the edge of the screen and double-click. The ring opens.
# Help topic "Welcome to Trindle": step 2 of the numbered steps at block 4.
settings-help-topics-welcome-step4-2 = The Super key opens it too.
# Help topic "Welcome to Trindle": step 3 of the numbered steps at block 4.
settings-help-topics-welcome-step4-3 = Your open windows are in the ring. Click one to bring it to the front.
# Help topic "Welcome to Trindle": paragraph 5.
settings-help-topics-welcome-p5 = Welcome is a short tour that opens the first time you log in. It uses moving pictures to show you all of this. It asks how much you already know and skips what you can already do. You can open Welcome again whenever you like from All apps.
# Help topic "Welcome to Trindle": paragraph 6.
settings-help-topics-welcome-p6 = For a button that could make you lose something, such as a bin, you need to click twice. The first click fills it with the highlight colour and shows a tick. Click again to go ahead. If you leave it, the button goes back to normal after three seconds.

## Topic: Opening the ring and finding apps (src/settings/help_topics.rs)

# Title of the Help topic "Opening the ring and finding apps".
settings-help-topics-ring-title = Opening the ring and finding apps
# Words people might search for the Help topic "Opening the ring and finding apps" by, in lower case, separated by spaces.
settings-help-topics-ring-keywords = launcher tray icons brightness tools apps applications start menu open program edge click favourites favorites all apps categories office games type search find filter
# Help topic "Opening the ring and finding apps": paragraph 1.
settings-help-topics-ring-p1 = Move the pointer to the very edge of the screen, any edge, and double-click. The ring appears. It works when a window fills the screen, but not over a full-screen game or video, where every click goes to that window. Just moving the pointer to the edge never opens it, so it can't pop up by accident. A single click elsewhere on the desktop opens nothing. The Super key opens the ring too, even over a full-screen window. So does a double-click on the empty desktop.
# Help topic "Opening the ring and finding apps": paragraph 2.
settings-help-topics-ring-p2 = In the middle are the time and date and a row of small icons. These are sound, Bluetooth, Wi-Fi and battery, and the cup that keeps the screen awake. The on-screen keyboard appears here once you switch it on in Settings, Keyboard and mouse. If your computer has a screen backlight, there's a brightness icon too: scroll over it to change the brightness, or click it for a slider. Below those are All apps, a button for updates when there are any, and Power. Power has Lock, Screensaver, Sleep, Hibernate (if your computer can), Log out, Restart and Shut down.
# Help topic "Opening the ring and finding apps": paragraph 3.
settings-help-topics-ring-p3 = Click the sound, Bluetooth, Wi-Fi or battery icon in the middle and a round panel opens over the favourites. There you can change the volume or the microphone, connect Bluetooth headphones, join a Wi-Fi network, or see how long the battery will last and how worn it is. The gear in the panel opens the full Settings page. The cross closes the panel.
# Help topic "Opening the ring and finding apps": paragraph 4.
settings-help-topics-ring-p4 = Your favourite apps are arranged round the middle. Folders and Drives are among them to begin with. There are also four tool circles that look like apps: Screenshot (the camera), Clipboard, Notifications and the Magnifier. Click an app to open it, or to bring its window to the front. Programs that are open appear in the ring of open windows round the favourites, not among the favourites themselves.
# Help topic "Opening the ring and finding apps": paragraph 5.
settings-help-topics-ring-p5 = Icons for other programs that are running, such as a chat app or a backup tool, are in a row at the bottom of the middle. Click one to open its program, or right-click it for its menu. A plugin's icon is there too. Click it to start the plugin, or right-click it to see what it offers, such as Connect and Disconnect for Flow, and its settings.
# Help topic "Opening the ring and finding apps": caption and spoken name of picture 6 (ring-all.png).
settings-help-topics-ring-pic6 = All apps: every app round the outside, grouped by kind
# Help topic "Opening the ring and finding apps": step 1 of the numbered steps at block 7.
settings-help-topics-ring-step7-1 = Click All apps in the middle. Every app that isn't already a favourite appears round the outside, in sections by type: Internet, Office, Graphics, Sound and video, Games and so on. A type with no apps left has no section.
# Help topic "Opening the ring and finding apps": step 2 of the numbered steps at block 7.
settings-help-topics-ring-step7-2 = Click an app in a section to open it.
# Help topic "Opening the ring and finding apps": step 3 of the numbered steps at block 7.
settings-help-topics-ring-step7-3 = Looking for one app? Type a few letters of its name. Only the apps that match stay, with the best match first, and Enter opens the first one. Escape clears what you typed.
# Help topic "Opening the ring and finding apps": step 4 of the numbered steps at block 7.
settings-help-topics-ring-step7-4 = Click All apps again to hide the sections.
# Help topic "Opening the ring and finding apps": tip 8, shown in a box of its own.
settings-help-topics-ring-tip8 = Press Escape, or click outside the circle, to close the ring.

## Topic: Keeping the screen awake (src/settings/help_topics.rs)

# Title of the Help topic "Keeping the screen awake".
settings-help-topics-awake-title = Keeping the screen awake
# Words people might search for the Help topic "Keeping the screen awake" by, in lower case, separated by spaces.
settings-help-topics-awake-keywords = cup caffeine keep awake screen off sleep dim lock video presentation
# Help topic "Keeping the screen awake": paragraph 1.
settings-help-topics-awake-p1 = Watching something, or reading without touching the mouse? Click the cup in the middle of the ring. It lights up. Until you click it again or log out, the screen won't dim, lock, switch off or sleep.
# Help topic "Keeping the screen awake": tip 2, shown in a box of its own.
settings-help-topics-awake-tip2 = The same switch is at the top of Settings, Power.

## Topic: Keeping your favourite apps in the ring (src/settings/help_topics.rs)

# Title of the Help topic "Keeping your favourite apps in the ring".
settings-help-topics-favourites-title = Keeping your favourite apps in the ring
# Words people might search for the Help topic "Keeping your favourite apps in the ring" by, in lower case, separated by spaces.
settings-help-topics-favourites-keywords = favourite favourites favorite favorites keep add remove right-click drag move order
# Help topic "Keeping your favourite apps in the ring": paragraph 1.
settings-help-topics-favourites-p1 = Right-click any app, in the ring or in All apps, and choose Add to favourites. It then always stays round the middle of the ring. To take one away, right-click it and choose Remove from favourites. The middle has room for twelve.
# Help topic "Keeping your favourite apps in the ring": paragraph 2.
settings-help-topics-favourites-p2 = You start with Folders and Drives. You can remove either of them and add it back from All apps.
# Help topic "Keeping your favourite apps in the ring": paragraph 3.
settings-help-topics-favourites-p3 = To change the order of your favourites, drag an app onto another app's place. The other apps move round to make room.
# Help topic "Keeping your favourite apps in the ring": caption and spoken name of picture 4 (ring-menu.png).
settings-help-topics-favourites-pic4 = Right-click an app for more choices
# Help topic "Keeping your favourite apps in the ring": paragraph 5.
settings-help-topics-favourites-p5 = The same menu can open another window of an app or close its windows. For apps you installed from Apps, it can also uninstall the app.

## Topic: Switching between windows (src/settings/help_topics.rs)

# Title of the Help topic "Switching between windows".
settings-help-topics-switcher-title = Switching between windows
# Words people might search for the Help topic "Switching between windows" by, in lower case, separated by spaces.
settings-help-topics-switcher-keywords = windows switch switcher alt tab open ring desktop show minimise clipboard copy paste history keyboard on screen
# Help topic "Switching between windows": paragraph 1.
settings-help-topics-switcher-p1 = Open the ring and you'll see your open windows in a ring round the favourites. Each window has its own part, with its program's icon and name. Point at one and the middle shows a picture of that window. Click it to bring that window to the front, or right-click it for minimise, maximise and close. Pressing Alt and Tab also brings each open window to the front in turn.
# Help topic "Switching between windows": paragraph 2.
settings-help-topics-switcher-p2 = Pressing Super and D hides every window to show the desktop. Press them again to bring the windows back. With nothing open, the screen is clear.
# Help topic "Switching between windows": paragraph 3.
settings-help-topics-switcher-p3 = The clipboard is one of the four tool circles among the favourites in the ring. It opens as a round panel and shows what you've copied lately, newest first. Type in the filter to find an item, click one to copy it again, and pin the ones you want to keep for good. The small cross beside a line removes just that one, which is handy for a password you copied by mistake. The bin at the bottom removes everything except the pinned ones. Click it twice to confirm. Choosing a line only puts it back on the clipboard. You paste it yourself, so nothing lands in a box you didn't choose. In Settings, Ring, Clipboard, you choose when the rest is cleared: each restart, after a day, a week or two weeks, or never.
# Help topic "Switching between windows": paragraph 4.
settings-help-topics-switcher-p4 = Once you switch it on in Settings, Keyboard and mouse, under Keyboard button in the ring, the on-screen keyboard has its own small icon in the middle of the ring. It opens a keyboard along the bottom of the screen. The same icon closes it again.

## Topic: Notifications: what programs tell you (src/settings/help_topics.rs)

# Title of the Help topic "Notifications: what programs tell you".
settings-help-topics-notifications-title = Notifications: what programs tell you
# Words people might search for the Help topic "Notifications: what programs tell you" by, in lower case, separated by spaces.
settings-help-topics-notifications-keywords = notification notifications alert message banner popup toast bell quiet silence do not disturb missed mail battery corner
# Help topic "Notifications: what programs tell you": paragraph 1.
settings-help-topics-notifications-p1 = When a program has something to tell you, such as a new email or a low battery, a small circle appears in a corner of the screen. It stays for a few seconds, then goes away. Up to three show at once, one above another. They stay in the corner, so nothing covers what you're reading.
# Help topic "Notifications: what programs tell you": paragraph 2.
settings-help-topics-notifications-p2 = The Notifications circle in the ring opens a list of every notification, oldest at the top and newest at the bottom, with the time beside each one. If you've been away from the computer there may be a lot, so the list opens at the bottom, where the newest is. Scroll up to see the older ones.
# Help topic "Notifications: what programs tell you": paragraph 3.
settings-help-topics-notifications-p3 = Beside each line are two small buttons. The pin keeps that notification for good: it moves above the rest, and clearing everything leaves it alone. The small bin removes just that one. At the bottom are a bell button, described below, and a bin that removes everything except the pinned ones. Click that bin twice to confirm.
# Help topic "Notifications: what programs tell you": paragraph 4.
settings-help-topics-notifications-p4 = You choose how long each one stays, in Settings, Ring, Notifications: three seconds for a glance, fifteen seconds to read it, or until you close it. An urgent one always stays until you close it, so a warning can't disappear before you've read it.
# Help topic "Notifications: what programs tell you": paragraph 5.
settings-help-topics-notifications-p5 = To switch off notifications, open the list from the Notifications circle and press the bell at the bottom. A line through the bell shows that notifications are off. They stop appearing on screen but still go into the list, so you won't miss anything. Press the bell again to switch them back on.
# Help topic "Notifications: what programs tell you": tip 6, shown in a box of its own.
settings-help-topics-notifications-tip6 = The cross on a notification only hides it. It's still in the list.

## Topic: Getting new apps (src/settings/help_topics.rs)

# Title of the Help topic "Getting new apps".
settings-help-topics-apps-title = Getting new apps
# Words people might search for the Help topic "Getting new apps" by, in lower case, separated by spaces.
settings-help-topics-apps-keywords = install store apps download software flathub browser firefox brave uninstall remove update deb package chrome vs code zoom driver double-click
# Help topic "Getting new apps": paragraph 1.
settings-help-topics-apps-p1 = Trindle doesn't choose your apps for you. The first time you log in, Welcome shows a few pages of popular apps, such as browsers. Pick the ones you want and press Install the chosen apps. You can open Welcome again whenever you like from All apps.
# Help topic "Getting new apps": paragraph 2.
settings-help-topics-apps-p2 = Later, open Apps from the ring. Browse by kind on the left, or search at the top. Click an app to read about it, then press the download arrow to install it. Its star rating is under its name, and pictures of it scroll across under the install button. Its reviews are further down, under Reviews. The globe button opens its page on Flathub.
# Help topic "Getting new apps": caption and spoken name of picture 3 (store-app.png).
settings-help-topics-apps-pic3 = An app's page in Apps
# Help topic "Getting new apps": step 1 of the numbered steps at block 4.
settings-help-topics-apps-step4-1 = To remove an app, find it in Apps and press the bin twice. The first press fills it with the highlight colour and shows a tick. The second press removes the app.
# Help topic "Getting new apps": step 2 of the numbered steps at block 4.
settings-help-topics-apps-step4-2 = Or right-click it in the ring and choose Uninstall.
# Help topic "Getting new apps": step 3 of the numbered steps at block 4.
settings-help-topics-apps-step4-3 = Updates are listed under Updates in Apps. Press the update button.
# Help topic "Getting new apps": tip 5, shown in a box of its own.
settings-help-topics-apps-tip5 = Apps are installed for you only, so no password is needed.
# Help topic "Getting new apps": paragraph 6.
settings-help-topics-apps-p6 = Some apps are not on Flathub. They are published on GitHub instead, as an AppImage or a Flatpak file. In Apps, type a name in the search box. AppImages listed on AppImageHub appear as you type, and pressing Enter searches the whole of GitHub. The eye shows the files in an app's newest release. The download arrow installs it straight away.
# Help topic "Getting new apps": tip 7, shown in a box of its own.
settings-help-topics-apps-tip7 = Flathub checks its own apps, but nobody checks apps from GitHub. Only install apps from people you trust.
# Help topic "Getting new apps": paragraph 8.
settings-help-topics-apps-p8 = The small round key button at the far end of the Search GitHub card opens a box for a GitHub key. You almost certainly don't need one. Without a key, GitHub answers 60 requests an hour from your computer. With one, it answers 5,000, which only matters if you search a great deal. Trindle remembers GitHub's answers for a few hours, so searching for the same thing again doesn't use up any requests. If you reach the limit, the page tells you when GitHub will answer again. The key lets Trindle look at public files and nothing else. It gives no access to your account, and you don't tick any permissions when you make it.
# Help topic "Getting new apps": paragraph 9.
settings-help-topics-apps-p9 = Some makers offer their program as a Debian package instead. This is a file ending in .deb. Google Chrome, VS Code, Zoom and many printer drivers come this way. Once it has downloaded, double-click it in Folders, or open it from your browser's downloads. A small window shows what it is, its version, and whether you already have it. Press the download arrow and enter your password. Anything else it needs is installed with it. If it includes an app, the app appears in the ring.
# Help topic "Getting new apps": tip 10, shown in a box of its own.
settings-help-topics-apps-tip10 = A package can change anything on the computer, so only install ones from makers you trust.

## Topic: Programs from their makers: Claude, Tailscale and more (src/settings/help_topics.rs)

# Title of the Help topic "Programs from their makers: Claude, Tailscale and more".
settings-help-topics-sources-title = Programs from their makers: Claude, Tailscale and more
# Words people might search for the Help topic "Programs from their makers: Claude, Tailscale and more" by, in lower case, separated by spaces.
settings-help-topics-sources-keywords = software sources repository repo apt claude anthropic chatgpt openai codex ai tailscale syncthing jellyfin media server node nodejs github gh docker deb newer
# Help topic "Programs from their makers: Claude, Tailscale and more": paragraph 1.
settings-help-topics-sources-p1 = A few programs aren't on Flathub. They come from their makers' own Debian sources instead, or are available there in much newer versions: Claude, Tailscale, Syncthing, Jellyfin, Node.js, GitHub CLI and Docker. Anything on Flathub, browsers included, comes from Apps as usual. In Apps, choose Software sources and press the plus beside one. Enter your administrator's password, and the source is added and the program installed. From then on it's updated along with Debian's updates.
# Help topic "Programs from their makers: Claude, Tailscale and more": paragraph 2.
settings-help-topics-sources-p2 = A source can install updates for the whole computer, so only add makers you trust. The bin removes the program and its source together.
# Help topic "Programs from their makers: Claude, Tailscale and more": paragraph 3.
settings-help-topics-sources-p3 = For Tailscale, press Log in to Tailscale once it's installed. Tailscale's login page opens in the browser, and Apps tells you when you're connected.
# Help topic "Programs from their makers: Claude, Tailscale and more": tip 4, shown in a box of its own.
settings-help-topics-sources-tip4 = Add another source lets you add any maker's source. Use the details from their install instructions: its address, suite, components and signing key.

## Topic: Undoing an update with snapshots (src/settings/help_topics.rs)

# Title of the Help topic "Undoing an update with snapshots".
settings-help-topics-snapshots-title = Undoing an update with snapshots
# Words people might search for the Help topic "Undoing an update with snapshots" by, in lower case, separated by spaces.
settings-help-topics-snapshots-keywords = snapshots snapshot restore undo rollback roll back update broke boot menu grub btrfs
# Help topic "Undoing an update with snapshots": paragraph 1.
settings-help-topics-snapshots-p1 = When the system is on Btrfs, Trindle takes a snapshot, a saved copy of the system, before and after every software change: installing, updating or removing anything from Debian or a software source. Settings, Snapshots lists them. The camera takes one now, before you try something you may want to undo.
# Help topic "Undoing an update with snapshots": step 1 of the numbered steps at block 2.
settings-help-topics-snapshots-step2-1 = Open Settings and choose Snapshots.
# Help topic "Undoing an update with snapshots": step 2 of the numbered steps at block 2.
settings-help-topics-snapshots-step2-2 = Press the restore arrow beside the snapshot from before things went wrong, then press it again to confirm.
# Help topic "Undoing an update with snapshots": step 3 of the numbered steps at block 2.
settings-help-topics-snapshots-step2-3 = Restart. The system is as it was then. The state it was in just before is set aside, not deleted.
# Help topic "Undoing an update with snapshots": paragraph 3.
settings-help-topics-snapshots-p3 = If the computer won't start, choose Trindle snapshots in the boot menu, then choose a snapshot. It starts as it was then, but nothing you do there is kept. To keep it, restore the snapshot in Settings.
# Help topic "Undoing an update with snapshots": tip 4, shown in a box of its own.
settings-help-topics-snapshots-tip4 = The boot menu is hidden. Tap Esc once as the computer starts to show it. Holding Esc skips past it to a typing screen. If that happens, type normal and press Enter to get the menu back.
# Help topic "Undoing an update with snapshots": tip 5, shown in a box of its own.
settings-help-topics-snapshots-tip5 = If Trindle was installed with its own installer, your home folder has its own partition, a separate section of the disk, and is left out of the snapshots, so restoring never changes your files. If this computer is set up differently, Settings, Snapshots tells you.

## Topic: Moving to another computer (src/settings/help_topics.rs)

# Title of the Help topic "Moving to another computer".
settings-help-topics-moving-title = Moving to another computer
# Words people might search for the Help topic "Moving to another computer" by, in lower case, separated by spaces.
settings-help-topics-moving-keywords = backup back up restore move new computer transfer copy settings apps data flat fmpack
# Help topic "Moving to another computer": paragraph 1.
settings-help-topics-moving-p1 = In Apps, choose Move your apps. Switch on what you want to take with you: your apps, their own data, your AppImages and Trindle's settings. Press the save button and Trindle makes one backup file in your home folder. Copy it to a USB stick or cloud storage.
# Help topic "Moving to another computer": paragraph 2.
settings-help-topics-moving-p2 = On the other computer, open the same page, press the open button and choose the file. Trindle downloads and installs the apps from Flathub, then puts back their data and your settings. Log out and back in to see every setting.
# Help topic "Moving to another computer": paragraph 3.
settings-help-topics-moving-p3 = The backup is the same kind of file that Trindle Flat makes. It ends in .fmpack. This means Trindle Flat can open it on any Linux computer, and you can choose a backup or app list made by Trindle Flat here. The name Trindle Flat on the page links to its website. Apps already on this computer keep their own settings. Only apps the backup installs get their settings back.
# Help topic "Moving to another computer": tip 4, shown in a box of its own.
settings-help-topics-moving-tip4 = Your documents, pictures and other files aren't in the backup, so copy those over yourself. Wi-Fi passwords and your GitHub key aren't included either.

## Topic: Connecting to Wi-Fi (src/settings/help_topics.rs)

# Title of the Help topic "Connecting to Wi-Fi".
settings-help-topics-wifi-title = Connecting to Wi-Fi
# Words people might search for the Help topic "Connecting to Wi-Fi" by, in lower case, separated by spaces.
settings-help-topics-wifi-keywords = wifi wi-fi wireless internet network connect password ethernet cable
# Help topic "Connecting to Wi-Fi": step 1 of the numbered steps at block 1.
settings-help-topics-wifi-step1-1 = Open the ring and click the network icon in the middle, or open Settings and choose Network.
# Help topic "Connecting to Wi-Fi": step 2 of the numbered steps at block 1.
settings-help-topics-wifi-step1-2 = Make sure the Wi-Fi switch is on.
# Help topic "Connecting to Wi-Fi": step 3 of the numbered steps at block 1.
settings-help-topics-wifi-step1-3 = Find your network in the list and press Connect.
# Help topic "Connecting to Wi-Fi": step 4 of the numbered steps at block 1.
settings-help-topics-wifi-step1-4 = Type the Wi-Fi password and press Join.
# Help topic "Connecting to Wi-Fi": caption and spoken name of picture 2 (settings-network.png).
settings-help-topics-wifi-pic2 = The Network page
# Help topic "Connecting to Wi-Fi": tip 3, shown in a box of its own.
settings-help-topics-wifi-tip3 = Trindle connects by itself to a network you've joined before. Press Forget this network twice to stop that.

## Topic: Sound, speakers and microphone (src/settings/help_topics.rs)

# Title of the Help topic "Sound, speakers and microphone".
settings-help-topics-sound-title = Sound, speakers and microphone
# Words people might search for the Help topic "Sound, speakers and microphone" by, in lower case, separated by spaces.
settings-help-topics-sound-keywords = sound audio volume speakers headphones microphone mic mute test quiet loud switcher ring input output
# Help topic "Sound, speakers and microphone": paragraph 1.
settings-help-topics-sound-p1 = Open Settings and choose Audio. Pick which speakers or headphones to use under Speakers and headphones, and which microphone under Microphone.
# Help topic "Sound, speakers and microphone": caption and spoken name of picture 2 (settings-audio.png).
settings-help-topics-sound-pic2 = The Audio page
# Help topic "Sound, speakers and microphone": step 1 of the numbered steps at block 3.
settings-help-topics-sound-step3-1 = Press Play a test sound to hear a short chime through the chosen speakers.
# Help topic "Sound, speakers and microphone": step 2 of the numbered steps at block 3.
settings-help-topics-sound-step3-2 = Speak and watch the Level bar. If it moves, the microphone is picking up your voice.
# Help topic "Sound, speakers and microphone": step 3 of the numbered steps at block 3.
settings-help-topics-sound-step3-3 = Drag the Volume slider, or switch Mute on.
# Help topic "Sound, speakers and microphone": paragraph 4.
settings-help-topics-sound-p4 = To change the volume or the microphone without opening Settings, click the sound icon in the middle of the ring and use its round panel.
# Help topic "Sound, speakers and microphone": tip 5, shown in a box of its own.
settings-help-topics-sound-tip5 = Scrolling over the sound icon in the ring also changes the volume.

## Topic: Sounds for logging in, the charger and the battery (src/settings/help_topics.rs)

# Title of the Help topic "Sounds for logging in, the charger and the battery".
settings-help-topics-sounds-title = Sounds for logging in, the charger and the battery
# Words people might search for the Help topic "Sounds for logging in, the charger and the battery" by, in lower case, separated by spaces.
settings-help-topics-sounds-keywords = sounds chime startup login logout shut down charger plugged unplugged battery alert off quiet change
# Help topic "Sounds for logging in, the charger and the battery": paragraph 1.
settings-help-topics-sounds-p1 = Open Settings and choose Sounds. Each of these has its own sound: logging in; logging out, restarting or shutting down; plugging in the charger; unplugging the charger; and the battery running low.
# Help topic "Sounds for logging in, the charger and the battery": step 1 of the numbered steps at block 2.
settings-help-topics-sounds-step2-1 = Choose another sound from the list next to one of them. It plays so you can hear it.
# Help topic "Sounds for logging in, the charger and the battery": step 2 of the numbered steps at block 2.
settings-help-topics-sounds-step2-2 = Choose Off to keep that one quiet.
# Help topic "Sounds for logging in, the charger and the battery": step 3 of the numbered steps at block 2.
settings-help-topics-sounds-step2-3 = Click the speaker to hear the sound again.

## Topic: Bluetooth headphones, mice and keyboards (src/settings/help_topics.rs)

# Title of the Help topic "Bluetooth headphones, mice and keyboards".
settings-help-topics-bluetooth-title = Bluetooth headphones, mice and keyboards
# Words people might search for the Help topic "Bluetooth headphones, mice and keyboards" by, in lower case, separated by spaces.
settings-help-topics-bluetooth-keywords = bluetooth pair headphones earbuds mouse keyboard speaker connect
# Help topic "Bluetooth headphones, mice and keyboards": step 1 of the numbered steps at block 1.
settings-help-topics-bluetooth-step1-1 = Open Settings and choose Bluetooth. Make sure Bluetooth is on.
# Help topic "Bluetooth headphones, mice and keyboards": step 2 of the numbered steps at block 1.
settings-help-topics-bluetooth-step1-2 = Put your device into pairing mode (see its instructions).
# Help topic "Bluetooth headphones, mice and keyboards": step 3 of the numbered steps at block 1.
settings-help-topics-bluetooth-step1-3 = Press Look for devices, then press Pair beside your device when it appears.
# Help topic "Bluetooth headphones, mice and keyboards": step 4 of the numbered steps at block 1.
settings-help-topics-bluetooth-step1-4 = If the other device shows a number, check that it matches and press Yes, it matches.
# Help topic "Bluetooth headphones, mice and keyboards": caption and spoken name of picture 2 (settings-bluetooth.png).
settings-help-topics-bluetooth-pic2 = The Bluetooth page

## Topic: Screen size and displays (src/settings/help_topics.rs)

# Title of the Help topic "Screen size and displays".
settings-help-topics-display-title = Screen size and displays
# Words people might search for the Help topic "Screen size and displays" by, in lower case, separated by spaces.
settings-help-topics-display-keywords = screen display size bigger smaller text zoom scale resolution monitor rotate night light warm blue light evening eyes
# Help topic "Screen size and displays": paragraph 1.
settings-help-topics-display-p1 = If things look too small or too large, open Settings and choose Display, then change Size. 100% is the standard size for your screen.
# Help topic "Screen size and displays": caption and spoken name of picture 2 (settings-display.png).
settings-help-topics-display-pic2 = The Display page
# Help topic "Screen size and displays": paragraph 3.
settings-help-topics-display-p3 = After a change, a message asks if you want to keep it. If you do nothing for fifteen seconds, the screen goes back to how it was, so a wrong choice can't leave you stuck.
# Help topic "Screen size and displays": paragraph 4.
settings-help-topics-display-p4 = Night light, at the bottom of the same page, makes the screen warmer in the evening, with less of the blue light that can keep you awake. Switch it on and it follows sunset and sunrise, which Trindle works out from your time zone, so it never asks where you are. Choose Set times instead to pick when it starts and ends. Move Warmth to the right for a warmer screen.

## Topic: NVIDIA graphics, and making them faster (src/settings/help_topics.rs)

# Title of the Help topic "NVIDIA graphics, and making them faster".
settings-help-topics-nvidia-title = Making NVIDIA graphics faster
# Words people might search for the Help topic "NVIDIA graphics, and making them faster" by, in lower case, separated by spaces.
settings-help-topics-nvidia-keywords = nvidia graphics card driver nouveau open slow speed gpu secure boot proprietary geforce
# Help topic "NVIDIA graphics, and making them faster": paragraph 1.
settings-help-topics-nvidia-p1 = On every NVIDIA card, Trindle starts with nouveau, the open-source driver that comes with Linux. You don't need to install or decide anything. The desktop works from the first start.
# Help topic "NVIDIA graphics, and making them faster": paragraph 2.
settings-help-topics-nvidia-p2 = On an older card, that's all you need. On a card from roughly 2022 onwards, the open driver can't raise the card's speed above its lowest setting. The desktop still works but feels sluggish, and video may stutter. NVIDIA's own driver fixes that.
# Help topic "NVIDIA graphics, and making them faster": tip 3, shown in a box of its own.
settings-help-topics-nvidia-tip3 = If your computer feels fine, leave it alone. Changing a graphics driver is the most common way to end up with a screen that won't come on.
# Help topic "NVIDIA graphics, and making them faster": paragraph 4.
settings-help-topics-nvidia-p4 = To install NVIDIA's own driver, open a terminal from the ring and type the two lines below, one at a time. If apt says a package "is not installable", install Trindle's updates first, then type both lines again:
# Help topic "NVIDIA graphics, and making them faster": step 1 of the numbered steps at block 5.
settings-help-topics-nvidia-step5-1 = sudo apt update
# Help topic "NVIDIA graphics, and making them faster": step 2 of the numbered steps at block 5.
settings-help-topics-nvidia-step5-2 = sudo apt install nvidia-driver
# Help topic "NVIDIA graphics, and making them faster": step 3 of the numbered steps at block 5.
settings-help-topics-nvidia-step5-3 = Don't restart yet. Read the next part first.
# Help topic "NVIDIA graphics, and making them faster": paragraph 6.
settings-help-topics-nvidia-p6 = Debian builds the driver for your kernel as it installs. This takes a few minutes and installs a compiler, which is a tool for building software. The driver rebuilds itself whenever the kernel is updated.
# Help topic "NVIDIA graphics, and making them faster": paragraph 10.
settings-help-topics-nvidia-p10 = To go back to the open driver at any time, run sudo apt remove nvidia-driver, then restart.

## Topic: Making the screen bigger where you are looking (src/settings/help_topics.rs)

# Title of the Help topic "Making the screen bigger where you are looking".
settings-help-topics-magnifier-title = Making the screen bigger where you are looking
# Words people might search for the Help topic "Making the screen bigger where you are looking" by, in lower case, separated by spaces.
settings-help-topics-magnifier-keywords = magnifier magnify zoom bigger larger reading glasses eyesight sight small print loupe accessibility
# Help topic "Making the screen bigger where you are looking": paragraph 1.
settings-help-topics-magnifier-p1 = The magnifier shows a magnified circle wherever you press, so you can read small print without changing anything on the screen itself.
# Help topic "Making the screen bigger where you are looking": step 1 of the numbered steps at block 2.
settings-help-topics-magnifier-step2-1 = Click the Magnifier circle in the ring.
# Help topic "Making the screen bigger where you are looking": step 2 of the numbered steps at block 2.
settings-help-topics-magnifier-step2-2 = Press and hold anywhere on the screen with a finger or the mouse button. A circle appears that shows everything under it larger.
# Help topic "Making the screen bigger where you are looking": step 3 of the numbered steps at block 2.
settings-help-topics-magnifier-step2-3 = Keep holding and move about. The circle follows you, so you can read along a line.
# Help topic "Making the screen bigger where you are looking": step 4 of the numbered steps at block 2.
settings-help-topics-magnifier-step2-4 = Let go and the circle disappears. Press the magnifier button again when you've finished.
# Help topic "Making the screen bigger where you are looking": tip 3, shown in a box of its own.
settings-help-topics-magnifier-tip3 = While the magnifier is on, the screen underneath stays still and can't be clicked, so you can't set anything off by accident while you're reading.

## Topic: Picking a colour from the screen (src/settings/help_topics.rs)

# Title of the Help topic "Picking a colour from the screen".
settings-help-topics-eye-dropper-title = Picking a colour from the screen
# Words people might search for the Help topic "Picking a colour from the screen" by, in lower case, separated by spaces.
settings-help-topics-eye-dropper-keywords = eye dropper eyedropper colour color pick picker hex code copy swatch screen
# Help topic "Picking a colour from the screen": paragraph 1.
settings-help-topics-eye-dropper-p1 = The eye dropper reads the colour of any single dot on the screen and copies its code, such as #FBC711, so you can paste it into a document or a design.
# Help topic "Picking a colour from the screen": step 1 of the numbered steps at block 2.
settings-help-topics-eye-dropper-step2-1 = Click the Eye dropper circle in the ring, the one with a dropper on it.
# Help topic "Picking a colour from the screen": step 2 of the numbered steps at block 2.
settings-help-topics-eye-dropper-step2-2 = The screen stays still, and a magnified circle appears by the pointer so you can aim at one dot.
# Help topic "Picking a colour from the screen": step 3 of the numbered steps at block 2.
settings-help-topics-eye-dropper-step2-3 = Click the dot you want. Press Esc to cancel instead.
# Help topic "Picking a colour from the screen": step 4 of the numbered steps at block 2.
settings-help-topics-eye-dropper-step2-4 = The code is copied at once and added to the top of the list, newest first.
# Help topic "Picking a colour from the screen": tip 3, shown in a box of its own.
settings-help-topics-eye-dropper-tip3 = Colours taken earlier stay in the list, so you can copy one again later without picking it again.

## Topic: Taking a picture of the screen (src/settings/help_topics.rs)

# Title of the Help topic "Taking a picture of the screen".
settings-help-topics-screenshots-title = Taking a picture of the screen
# Words people might search for the Help topic "Taking a picture of the screen" by, in lower case, separated by spaces.
settings-help-topics-screenshots-keywords = screenshot screen shot print prtsc super shift s picture capture grab snip area part copy paste clipboard
# Help topic "Taking a picture of the screen": paragraph 1.
settings-help-topics-screenshots-p1 = Press the Print key. If your keyboard doesn't have one, press Super, Shift and S. You can also click the Screenshot circle (the camera) in the ring. The ring closes first so it isn't in the picture. Then the screen stays still and goes a little dimmer. Drag across the part you want, or use the two round buttons at the bottom: one for the whole screen, and one to close. Rest the pointer on a button to see its name.
# Help topic "Taking a picture of the screen": step 1 of the numbered steps at block 2.
settings-help-topics-screenshots-step2-1 = For the whole screen, press the screen button.
# Help topic "Taking a picture of the screen": step 2 of the numbered steps at block 2.
settings-help-topics-screenshots-step2-2 = For a part, press the dotted square, then drag across the part you want. The picture is taken the moment you let go.
# Help topic "Taking a picture of the screen": step 3 of the numbered steps at block 2.
settings-help-topics-screenshots-step2-3 = Changed your mind? Press the cross, or Esc.
# Help topic "Taking a picture of the screen": paragraph 3.
settings-help-topics-screenshots-p3 = Trindle saves the picture in Pictures, in a folder called Screenshots. The file name has the date and time. Trindle also copies the picture, so you can paste it straight into an email or a document. Nothing shows when it works. You only see a message if there's a problem.
# Help topic "Taking a picture of the screen": tip 4, shown in a box of its own.
settings-help-topics-screenshots-tip4 = Shift and Print take the whole screen at once, without asking.

## Topic: Having the screen read aloud (src/settings/help_topics.rs)

# Title of the Help topic "Having the screen read aloud".
settings-help-topics-screen-reader-title = Having the screen read aloud
# Words people might search for the Help topic "Having the screen read aloud" by, in lower case, separated by spaces.
settings-help-topics-screen-reader-keywords = screen reader blind read aloud speak speech voice sight eyesight accessibility
# Help topic "Having the screen read aloud": paragraph 1.
settings-help-topics-screen-reader-p1 = No screen reader is installed, so the Read the screen aloud switch in Settings, Appearance, under Reading the screen, is greyed out, with a note saying so.
# Help topic "Having the screen read aloud": paragraph 2.
settings-help-topics-screen-reader-p2 = When a screen reader is ready, pressing Super, Alt and S together will turn it on and off from anywhere. That works even the very first time the computer starts, when there's nothing to see or click. Until then, those keys open Settings at that switch.

## Topic: Wallpaper (src/settings/help_topics.rs)

# Title of the Help topic "Wallpaper".
settings-help-topics-wallpaper-title = Wallpaper
# Words people might search for the Help topic "Wallpaper" by, in lower case, separated by spaces.
settings-help-topics-wallpaper-keywords = wallpaper background picture photo slideshow change tiles pictures folder add remove skip
# Help topic "Wallpaper": paragraph 1.
settings-help-topics-wallpaper-p1 = Open Settings and choose Wallpaper. The pictures Trindle comes with are shown as small tiles, and the one on screen now is in the large preview on the right.
# Help topic "Wallpaper": step 1 of the numbered steps at block 2.
settings-help-topics-wallpaper-step2-1 = Press a tile to put that picture on the screen.
# Help topic "Wallpaper": step 2 of the numbered steps at block 2.
settings-help-topics-wallpaper-step2-2 = Press it again to leave it out. The tile dims, and that picture is skipped from then on. Press once more to bring it back.
# Help topic "Wallpaper": step 3 of the numbered steps at block 2.
settings-help-topics-wallpaper-step2-3 = Add your own with Choose a picture, or use Choose a folder for a folder of your own.
# Help topic "Wallpaper": step 4 of the numbered steps at block 2.
settings-help-topics-wallpaper-step2-4 = Switch on Change the picture now and then, and choose how often. Only the pictures you haven't left out are used.
# Help topic "Wallpaper": tip 3, shown in a box of its own.
settings-help-topics-wallpaper-tip3 = Leaving a picture out never deletes it. The tile stays there, dimmed, so you can change your mind.
# Help topic "Wallpaper": caption and spoken name of picture 4 (settings-wallpaper.png).
settings-help-topics-wallpaper-pic4 = The Wallpaper page

## Topic: Colours, icons and text (src/settings/help_topics.rs)

# Title of the Help topic "Colours, icons and text".
settings-help-topics-look-title = Colours, icons and text
# Words people might search for the Help topic "Colours, icons and text" by, in lower case, separated by spaces.
settings-help-topics-look-keywords = colour colours color colors accent theme dark light mode sunshine bright font text icons cursor pointer look
# Help topic "Colours, icons and text": paragraph 1.
settings-help-topics-look-p1 = Open Settings and choose Appearance. Under Theme, a theme that has both a dark and a light look has two buttons beside its picture, Dark and Light. Press Light when the screen is hard to see in bright sunshine, and Dark again later. Everything changes at once, except apps installed from Apps, which change the next time they open. A theme with only one look, such as Midnight (dark) or Paper (light), has no such buttons. To change between dark and light there, choose another theme.
# Help topic "Colours, icons and text": paragraph 2.
settings-help-topics-look-p2 = The highlight colour is used for buttons, switches and to show where you are. Pick one of the twenty colours or type your own.
# Help topic "Colours, icons and text": caption and spoken name of picture 3 (settings-appearance.png).
settings-help-topics-look-pic3 = The Appearance page
# Help topic "Colours, icons and text": paragraph 4.
settings-help-topics-look-p4 = Under Icons, choose Circles or Squares. Both are the Trindle Icons: each shows the app's own mark in white on its own colour, round or on a rounded square, and looks the same in light and dark. Trindle starts with circles. If the set doesn't have an icon for an app yet, the app's own picture is shown on a white circle or square, so it still fits in.
# Help topic "Colours, icons and text": paragraph 5.
settings-help-topics-look-p5 = The same page changes the font and the size of the pointer. You can also add a font of your own: see adding fonts.
# Help topic "Colours, icons and text": paragraph 6.
settings-help-topics-look-p6 = A theme changes the whole look at once: see themes. To change more things by hand, such as shadows, corners or the background of Apps, see changing the look with a style sheet.

## Topic: Adding fonts of your own (src/settings/help_topics.rs)

# Title of the Help topic "Adding fonts of your own".
settings-help-topics-themes-title = Adding fonts of your own
# Words people might search for the Help topic "Adding fonts of your own" by, in lower case, separated by spaces.
settings-help-topics-themes-keywords = font fonts ttf otf ttc zip add upload install typeface letters
# Help topic "Adding fonts of your own": paragraph 1.
settings-help-topics-themes-p1 = In Settings, Appearance, you can add a font you've downloaded, so you're not stuck with the ones that come with Trindle. Add it under Text, and it appears in the Font list there.
# Help topic "Adding fonts of your own": paragraph 2.
settings-help-topics-themes-p2 = A font is a .ttf, .otf or .ttc file, or a zip containing some. Trindle copies every font it finds into your own fonts folder, and they then appear in the Font list. To install fonts for everyone who uses the computer, use Apps instead.
# Help topic "Adding fonts of your own": tip 3, shown in a box of its own.
settings-help-topics-themes-tip3 = Nothing you add is sent anywhere, and nothing already on the computer is replaced.

## Topic: Themes: a whole look from one file (src/settings/help_topics.rs)

# Title of the Help topic "Themes: a whole look from one file".
settings-help-topics-theme-files-title = Themes: a whole look from one file
# Words people might search for the Help topic "Themes: a whole look from one file" by, in lower case, separated by spaces.
settings-help-topics-theme-files-keywords = theme themes look skin file css add share make own glass dark light keep go back undo thirty seconds footer safe default trial
# Help topic "Themes: a whole look from one file": paragraph 1.
settings-help-topics-theme-files-p1 = A theme is a whole look, including colours, corners, sizes and spacing, kept in one file. In Settings, Appearance, Theme, every theme is shown as a small picture drawn from its own colours. Trindle's own look, a deep night blue, is always first in the list and can never be removed. Midnight, which comes with Trindle, is black and grey with no blue in it. That is how Trindle's own look used to be.
# Help topic "Themes: a whole look from one file": step 1 of the numbered steps at block 2.
settings-help-topics-theme-files-step2-1 = Press a theme to try it. Every Trindle window changes at once. If a theme has both a dark and a light look, Dark and Light appear beside its picture. Press one of those instead to try the theme in that look. On the theme you already use, they change the look straight away, with no need to confirm.
# Help topic "Themes: a whole look from one file": step 2 of the numbered steps at block 2.
settings-help-topics-theme-files-step2-2 = A Keep box appears along the bottom of the window. It stays in sight however far down the page you are. Press Keep within 30 seconds to keep the new look. If you do nothing, or press Undo now or Escape, your old look comes back by itself, everywhere, even if Settings has been closed.
# Help topic "Themes: a whole look from one file": step 3 of the numbered steps at block 2.
settings-help-topics-theme-files-step2-3 = To add a theme somebody has shared, press Add a theme and choose its .css file. Trindle checks it first. If it can't use the theme, it shows which line is wrong and why.
# Help topic "Themes: a whole look from one file": paragraph 3.
settings-help-topics-theme-files-p3 = A theme file is written like Trindle's own sheet (see changing the look with a style sheet). It has a :root { "{" } { "}" } block holding any of the same names, such as --trindle-page: #101820;. At the top it can say who it is, using --trindle-theme-name, --trindle-theme-about (one line), --trindle-theme-author and --trindle-theme-version. It can say which looks it has with --trindle-theme-look: dark; light; or both;. Anything it leaves out stays as Trindle has it. Themes you add are kept in the themes folder, inside the trindle folder in your .config folder. Removing one from there removes it from the list.
# Help topic "Themes: a whole look from one file": paragraph 4.
settings-help-topics-theme-files-p4 = A theme with both looks keeps them in the one file. Its dark values are written in :root { "{" } { "}" } as usual, and its light ones in a second block, @media (prefers-color-scheme: light) { "{" } :root { "{" } { "}" } { "}" }, exactly as Trindle's own sheet does. Give the light block every colour and shadow the dark look sets, or the dark ones show through in the light look. Settings then shows Dark and Light beside the theme's picture. A theme that says dark or light has only that look, is always drawn in it, and has no such buttons. If a theme doesn't say, Trindle treats it as having both looks when it has a light block, and as dark when it has none. Other programs are told the look your theme is really drawn in, so a light theme makes them light.
# Help topic "Themes: a whole look from one file": paragraph 5.
settings-help-topics-theme-files-p5 = A theme is safe to try, whoever made it. Trindle only reads it as a list of values and never runs it, and it can't import another file or reach the internet. Trindle only takes values it knows, and keeps each within its limits. So no theme can hide a button, a question or a warning, or make text too small to read. Words stay clear enough to read whatever colours a theme gives, red stays red, and your highlight colour stays your own. A theme changes only your own windows. The login screen always uses Trindle's own look.
# Help topic "Themes: a whole look from one file": tip 6, shown in a box of its own.
settings-help-topics-theme-files-tip6 = If a look ever goes wrong and you can't find your way back, open a terminal, or a text console with Ctrl, Alt and F3, and type: trindle-settings --theme default

## Topic: Changing the look with a style sheet (src/settings/help_topics.rs)

# Title of the Help topic "Changing the look with a style sheet".
settings-help-topics-theming-title = Changing the look with a style sheet
# Words people might search for the Help topic "Changing the look with a style sheet" by, in lower case, separated by spaces.
settings-help-topics-theming-keywords = theme theming style sheet stylesheet css colours colors shadows corners rounder bigger text size buttons background page store look custom own retheme
# Help topic "Changing the look with a style sheet": paragraph 1.
settings-help-topics-theming-p1 = Everything Trindle draws comes from one style sheet, written the way web pages are styled. That includes colours, shadows, corners, the space between things, the size of buttons, and the size and weight of words. You, or an AI you ask, can change any of it with a sheet of your own.
# Help topic "Changing the look with a style sheet": step 1 of the numbered steps at block 2.
settings-help-topics-theming-step2-1 = Create a file called style.css in the trindle folder inside your .config folder. Folders only shows it when you show hidden files.
# Help topic "Changing the look with a style sheet": step 2 of the numbered steps at block 2.
settings-help-topics-theming-step2-2 = Include only what you want to change, inside :root { "{" } { "}" }. For example, --trindle-page: #101010; gives every window a near-black page, --trindle-highlight: #4bae4f; makes buttons green, and --trindle-card-corner: 16; makes cards rounder.
# Help topic "Changing the look with a style sheet": step 3 of the numbered steps at block 2.
settings-help-topics-theming-step2-3 = Save it. Every Trindle window, the ring and the title bars change straight away.
# Help topic "Changing the look with a style sheet": paragraph 3.
settings-help-topics-theming-p3 = Every name you can use is listed in Trindle's own sheet, /usr/share/trindle/style.css. Beside each one is a note on what it does and its limits, such as (6 to 18). Changes for the light look are written only in a @media (prefers-color-scheme: light) { "{" } :root { "{" } { "}" } { "}" } block, as in Trindle's sheet. Shadows are written as a web page writes them, such as 0 8px 20px 2px rgba(0, 0, 0, 0.8). Cards have a gentle shadow to begin with. --trindle-card-shadow: none; makes them flat, and --trindle-edge: transparent; takes away their fine edge.
# Help topic "Changing the look with a style sheet": paragraph 4.
settings-help-topics-theming-p4 = Nothing you write can break a window. Trindle ignores names it doesn't know, replaces a value it can't read with its own, and keeps sizes within their limits. Colours are up to you, and Trindle keeps them exactly as you wrote them. It changes its own words so you can still read them on your pages, cards and title bars. A colour set for the light look only applies in the light look. A see-through colour is checked over both black and white, because Trindle can't know what will show through it. If no colour of words can be read on all of yours, Trindle uses black or white. Three things always stay readable: red words about a failure, words on your highlight colour, and password boxes, which always use Trindle's own look. If your colours are too hard to read, Settings, Appearance says "Some colours are hard to read." and leaves them as you wrote them. If words you set yourself become hard to read, delete the file and Trindle's own look comes back at once. The file must be an ordinary text file of at most 256 KB. If it isn't, Trindle doesn't use it, and Settings, Appearance tells you.
# Help topic "Changing the look with a style sheet": paragraph 5.
settings-help-topics-theming-p5 = Your own sheet is applied on top of whichever theme is chosen, so a small change of yours stays when you change theme. To make a whole look that you can keep or share, make a theme instead: see themes.
# Help topic "Changing the look with a style sheet": tip 6, shown in a box of its own.
settings-help-topics-theming-tip6 = If you set --trindle-highlight in your sheet, it overrides the highlight colour chosen in Settings, Appearance.

## Topic: Voice control (src/settings/help_topics.rs)

# Title of the Help topic "Voice control".
settings-help-topics-voice-title = Voice control
# Words people might search for the Help topic "Voice control" by, in lower case, separated by spaces.
settings-help-topics-voice-keywords = voice speak talk microphone commands whisper listen say
# Help topic "Voice control": paragraph 1.
settings-help-topics-voice-p1 = Voice control is an optional extra. To add it, open Settings, choose Ring, and press Add voice control. It takes a few minutes and about 150 MB.
# Help topic "Voice control": paragraph 2.
settings-help-topics-voice-p2 = Then open the ring, tap its middle anywhere there's no button, and speak. Trindle stops listening when you stop talking, or you can tap again to stop it. This works with a finger as well as the mouse. Here are some things you can say:
# Help topic "Voice control": step 1 of the numbered steps at block 3.
settings-help-topics-voice-step3-1 = An app's name, such as “Firefox” or “folders”.
# Help topic "Voice control": step 2 of the numbered steps at block 3.
settings-help-topics-voice-step3-2 = “Switch to” an app, or “close window”.
# Help topic "Voice control": step 3 of the numbered steps at block 3.
settings-help-topics-voice-step3-3 = “Louder”, “quieter”, “mute”.
# Help topic "Voice control": step 4 of the numbered steps at block 3.
settings-help-topics-voice-step3-4 = “Wi-Fi settings”, “battery settings”, or any other settings page.
# Help topic "Voice control": step 5 of the numbered steps at block 3.
settings-help-topics-voice-step3-5 = “Install” and an app's name, to search for it in Apps.
# Help topic "Voice control": step 6 of the numbered steps at block 3.
settings-help-topics-voice-step3-6 = “Next wallpaper”, “screenshot”, “what time is it”.
# Help topic "Voice control": tip 4, shown in a box of its own.
settings-help-topics-voice-tip4 = Trindle understands your voice on this computer, and nothing is sent anywhere. You can switch voice control off on the Ring page without removing it.

## Topic: Folders, USB sticks and phones (src/settings/help_topics.rs)

# Title of the Help topic "Folders, USB sticks and phones".
settings-help-topics-files-title = Folders, USB sticks and phones
# Words people might search for the Help topic "Folders, USB sticks and phones" by, in lower case, separated by spaces.
settings-help-topics-files-keywords = folders files file manager usb stick drive phone camera nas network share sharing samba disks eject format extract unpack zip compress archive rar 7z tar iso disc image password terminal f4 root wallpaper copy path location preview space bar emblem badge resize rotate picture columns music photo pdf keys keyboard shortcuts tab tabs new window
# Help topic "Folders, USB sticks and phones": paragraph 1.
settings-help-topics-files-p1 = Folders is the file manager, one of the apps round the ring. USB sticks, phones and cameras you plug in appear on its left, and so do shared folders on your local network, under Network.
# Help topic "Folders, USB sticks and phones": paragraph 2.
settings-help-topics-files-p2 = Folders opens with one pane showing your Home folder, with Downloads and Documents pinned on the left. The Single and Dual switch at the top left (or F3) adds a second pane. The second pane shows Recent unless you chose another place in Folders' own settings. Open those settings with the cog at the end of the top bar (or Ctrl and comma). There you can set where each pane starts, whether to show tick boxes for selecting files, whether to show file endings, and whether a drive opens as soon as you plug it in. Each pane has its own row of tabs above its list. The top bar only has buttons that change how the folder looks, plus Undo and New folder. Everything you can do with the files you've selected, such as Rename, Cut, Copy, Move to Trash, Delete permanently and Wipe securely, is in the right-click menu, and each one still has its key.
# Help topic "Folders, USB sticks and phones": paragraph 3.
settings-help-topics-files-p3 = In the right-click menu, Open with another app lists the apps that can open the file. Double-click one, or press Enter on it, to open the file with it straight away. The tick does the same.
# Help topic "Folders, USB sticks and phones": paragraph 4.
settings-help-topics-files-p4 = When you plug in a USB stick or a memory card, a line with an Open button appears in the ring. Press Open to see its files in Folders. If the drive is locked, it asks for the passphrase first. Folders also lists the drive under Drives whether or not you press Open, with a round Eject button that removes it safely. To have Folders open drives for you, switch on "Open drives when plugged in" in Folders' own settings, under Drives. A drive plugged in while the screen is locked waits until you unlock it.
# Help topic "Folders, USB sticks and phones": paragraph 5.
settings-help-topics-files-p5 = Right-click in Folders for more: Extract here and Extract to a folder for archives, Compress to make a .zip or a .tar, Open in Terminal, Open as Root, Copy location, and Set as wallpaper on a picture. Folders opens archives itself. Double-click a .zip, .7z, .rar or .tar, a disc image (.iso), or almost any other kind, and it opens like a folder, so you can look inside and take out only what you want. If a .zip or .7z is locked with a password, Folders asks for it. It doesn't remember the password, so it asks again next time. Folders can't open a .rar locked with a password, and it tells you so.
# Help topic "Folders, USB sticks and phones": paragraph 6.
settings-help-topics-files-p6 = Press the space bar on a file to look at it without opening it, and press it again to close the preview. F4 opens a terminal in the folder you're in. Right-click a folder or a file and choose Pin to the ring to keep it round the middle of the ring with your favourite apps, where one click opens it. To unpin it, right-click it there. Right-click pictures to resize or rotate them. In a file's Properties you can give it a small emblem. In a folder's Properties you can share it with other computers on the same local network (on a café or office network, that means everyone else on it). The first time, Folders switches on Samba, which asks for your password, and sharing works once the computer has restarted. In the list view, right-click the column headings to add music, photo and PDF details.
# Help topic "Folders, USB sticks and phones": paragraph 7.
settings-help-topics-files-p7 = Each name in the path along the top has a small arrow after it. Press the arrow to see the folders inside that folder, then choose one to go there. Middle-click a folder, or hold Ctrl and double-click it, to open it in a new tab behind the one you're on. Middle-click a tab to close it. Drag a folder between two of the pinned folders on the left to pin it there, and drag the pinned folders up and down to put them in the order you like. Under the Find box are three lists, Kind, Size and Changed. Choose from them to narrow what's shown straight away.
# Help topic "Folders, USB sticks and phones": paragraph 8.
settings-help-topics-files-p8 = The keys are the usual ones for a file manager. Alt and Left go back, Alt and Right forward, and Alt and Up or Backspace up one folder. Ctrl and L, or Alt and D, let you type a place. Ctrl and T opens a tab, and Ctrl and W closes it (and the window, when it's the last tab). Ctrl and Tab moves to the next tab, and Ctrl, Shift and Tab to the one before. Ctrl and N opens another window. Ctrl and F finds, F2 renames, F5 refreshes the folder, Delete moves a file to the Trash and Shift and Delete deletes it permanently. Ctrl, Shift and N makes a folder. Ctrl and A selects all, Ctrl, Shift and A selects none, and Ctrl and I swaps what's selected with what isn't. Alt and Enter opens Properties. F3 switches between one pane and two, and F4 opens a terminal.
# Help topic "Folders, USB sticks and phones": paragraph 9.
settings-help-topics-files-p9 = Drives shows every drive and how much space is free. You'll find it in All apps and in Settings, About this computer. From there you can open a USB stick, eject it safely before pulling it out, or format it. Trindle never offers to format the drive the system runs on.
# Help topic "Folders, USB sticks and phones": caption and spoken name of picture 10 (settings-disks.png).
settings-help-topics-files-pic10 = The Drives window

## Topic: Apps that come as an AppImage (src/settings/help_topics.rs)

# Title of the Help topic "Apps that come as an AppImage".
settings-help-topics-appimages-title = Apps that come as an AppImage
# Words people might search for the Help topic "Apps that come as an AppImage" by, in lower case, separated by spaces.
settings-help-topics-appimages-keywords = appimage app image download portable website install
# Help topic "Apps that come as an AppImage": paragraph 1.
settings-help-topics-appimages-p1 = Some apps are downloaded from their makers' websites as a single file ending in .AppImage. Open Apps, choose Installed, press the plus beside Add an AppImage, then pick the file. Trindle copies it into the .appimage folder in your home folder, and it appears in the ring with its own name and picture.
# Help topic "Apps that come as an AppImage": paragraph 2.
settings-help-topics-appimages-p2 = It's listed on the same page with every other app. To remove it, press its bin twice. The copy you downloaded stays where it was.

## Topic: Older programs: Steam, Wine games and Java apps (src/settings/help_topics.rs)

# Title of the Help topic "Older programs: Steam, Wine games and Java apps".
settings-help-topics-older-programs-title = Older programs: Steam, Wine games and Java apps
# Words people might search for the Help topic "Older programs: Steam, Wine games and Java apps" by, in lower case, separated by spaces.
settings-help-topics-older-programs-keywords = older programs x11 xwayland steam wine games java old apps allow switch watch spy
# Help topic "Older programs: Steam, Wine games and Java apps": paragraph 1.
settings-help-topics-older-programs-p1 = Most apps on Trindle draw their windows the newer way. Some older programs, such as Steam and its games, Wine games and Java apps, still draw them the older way, using a helper called Xwayland. On other desktops, all older programs share one helper, so any of them can watch the others' windows and what you type into them.
# Help topic "Older programs: Steam, Wine games and Java apps": paragraph 2.
settings-help-topics-older-programs-p2 = Trindle gives each older app a helper of its own, so none can see another. This applies to each one you installed in its own folder, and each Flathub app that needs it. Programs installed for the whole computer share one helper. A helper only starts when an older program opens and stops when it closes, so it uses no memory the rest of the time.
# Help topic "Older programs: Steam, Wine games and Java apps": tip 3, shown in a box of its own.
settings-help-topics-older-programs-tip3 = If you never use older programs, switch off Allow older programs in Settings, Permissions. They won't open while it's off.

## Topic: Printing (src/settings/help_topics.rs)

# Title of the Help topic "Printing".
settings-help-topics-printers-title = Printing
# Words people might search for the Help topic "Printing" by, in lower case, separated by spaces.
settings-help-topics-printers-keywords = print printer printing test page
# Help topic "Printing": paragraph 1.
settings-help-topics-printers-p1 = Open Settings and choose Printers. Trindle finds printers on your local network by itself. Press Add this printer beside yours. To check it works, press Print a test page.
# Help topic "Printing": caption and spoken name of picture 2 (settings-printers.png).
settings-help-topics-printers-pic2 = The Printers page

## Topic: People, passwords and pictures (src/settings/help_topics.rs)

# Title of the Help topic "People, passwords and pictures".
settings-help-topics-users-title = People, passwords and pictures
# Words people might search for the Help topic "People, passwords and pictures" by, in lower case, separated by spaces.
settings-help-topics-users-keywords = users accounts people password picture photo avatar administrator add delete
# Help topic "People, passwords and pictures": paragraph 1.
settings-help-topics-users-p1 = Open Settings and choose Users. You can change your password, choose a picture of yourself that appears at the login screen, and add accounts for other people.
# Help topic "People, passwords and pictures": caption and spoken name of picture 2 (settings-users.png).
settings-help-topics-users-pic2 = The Users page
# Help topic "People, passwords and pictures": tip 3, shown in a box of its own.
settings-help-topics-users-tip3 = You need an administrator's password to change accounts.

## Topic: Language and keyboard (src/settings/help_topics.rs)

# Title of the Help topic "Language and keyboard".
settings-help-topics-language-title = Language and keyboard
# Words people might search for the Help topic "Language and keyboard" by, in lower case, separated by spaces.
settings-help-topics-language-keywords = language keyboard layout british american typing region formats date currency mouse touchpad pointer speed tap click scroll natural left handed
# Help topic "Language and keyboard": paragraph 1.
settings-help-topics-language-p1 = In Settings, Keyboard and mouse, you choose the keyboard layout, and a second layout to switch to. In Settings, Region and language, you choose the language of menus and how dates, money and numbers are written. That takes effect the next time you log in.
# Help topic "Language and keyboard": paragraph 2.
settings-help-topics-language-p2 = The same Keyboard and mouse page sets how fast the pointer and scrolling go, which way scrolling moves the page, and whether the mouse buttons are swapped for left-handed use. On a laptop it also has the touchpad settings: tap to click, and switching the touchpad off while you type. Each change works straight away.
# Help topic "Language and keyboard": caption and spoken name of picture 3 (settings-keyboard.png).
settings-help-topics-language-pic3 = The Keyboard and mouse page
# Help topic "Language and keyboard": caption and spoken name of picture 4 (settings-region.png).
settings-help-topics-language-pic4 = The Region and language page

## Topic: Battery, sleep and shutting down (src/settings/help_topics.rs)

# Title of the Help topic "Battery, sleep and shutting down".
settings-help-topics-power-title = Battery, sleep and shutting down
# Words people might search for the Help topic "Battery, sleep and shutting down" by, in lower case, separated by spaces.
settings-help-topics-power-keywords = battery power charge sleep suspend shut down turn off restart log out screen
# Help topic "Battery, sleep and shutting down": paragraph 1.
settings-help-topics-power-p1 = Settings, Power shows the battery and the power mode. It also shows how long you can be away before the screen dims, locks and switches off, and before the computer sleeps. Once locking is switched on (see below), Super and L together lock the screen straight away.
# Help topic "Battery, sleep and shutting down": paragraph 2.
settings-help-topics-power-p2 = Locking is off to begin with. Switch on Lock the screen under Locking on the same page. Then choose how long the computer is left alone before it locks: only when it sleeps, or from 1 minute to 2 hours. Your password unlocks it. If you like, you can add a four-digit PIN as well, which is quicker. You choose how long after locking the PIN works, from 15 minutes to 2 hours. After that time, or after five wrong tries, it asks for your password. When the screen locks because the computer was left alone, it shows only the time, drifting about, until you come back. Nothing else on your screen shows until you're back in. Logging in always needs your password.
# Help topic "Battery, sleep and shutting down": caption and spoken name of picture 3 (settings-power.png).
settings-help-topics-power-pic3 = The Power page
# Help topic "Battery, sleep and shutting down": paragraph 4.
settings-help-topics-power-p4 = When the battery runs down to 10%, a warning appears in the middle of the screen, once, with a sound. To change the level, choose another one, or Never, under Warn when the battery is down to. When you plug the charger in or pull it out, a card appears for a few seconds, with a sound, so you notice if the cable slips out.
# Help topic "Battery, sleep and shutting down": paragraph 5.
settings-help-topics-power-p5 = To log out, restart or shut down, open the ring and click the power button in the middle. The choices appear in the middle of the ring. Lock and Screensaver happen at once. Sleep, Hibernate, Log out, Restart and Shut down ask twice: the first click fills the button with the highlight colour and shows a tick, so a stray click never ends anything. The cross closes the choices.

## Topic: Apps that start by themselves (src/settings/help_topics.rs)

# Title of the Help topic "Apps that start by themselves".
settings-help-topics-startup-title = Apps that start by themselves
# Words people might search for the Help topic "Apps that start by themselves" by, in lower case, separated by spaces.
settings-help-topics-startup-keywords = autostart startup start login automatically launch delay open when i log in
# Help topic "Apps that start by themselves": paragraph 1.
settings-help-topics-startup-p1 = Settings, Start at login lists the apps that open by themselves when you log in.
# Help topic "Apps that start by themselves": step 1 of the numbered steps at block 2.
settings-help-topics-startup-step2-1 = To add one, press the grid button and pick from the apps installed on this computer, or press the terminal button to type a command or choose a program from the disk.
# Help topic "Apps that start by themselves": step 2 of the numbered steps at block 2.
settings-help-topics-startup-step2-2 = Each one has a delay: At once, or after a few seconds. Use a delay for an app that needs the network or another app to be ready first.
# Help topic "Apps that start by themselves": step 3 of the numbered steps at block 2.
settings-help-topics-startup-step2-3 = Switch an app off to stop it starting without taking it off the list. The bin removes it.

## Topic: Keyboard shortcuts (src/settings/help_topics.rs)

# Title of the Help topic "Keyboard shortcuts".
settings-help-topics-keys-title = Keyboard shortcuts
# Words people might search for the Help topic "Keyboard shortcuts" by, in lower case, separated by spaces.
settings-help-topics-keys-keywords = keys keyboard shortcuts super windows key alt tab screenshot print volume brightness lock terminal close maximise screen reader desktop show
# Help topic "Keyboard shortcuts": paragraph 1.
settings-help-topics-keys-p1 = You can do everything with the mouse. These keys are only quicker ways. Super is the key with the Windows logo on most keyboards.
# Help topic "Keyboard shortcuts": step 1 of the numbered steps at block 2.
settings-help-topics-keys-step2-1 = Super on its own, or Super and Space: open or close the ring.
# Help topic "Keyboard shortcuts": step 2 of the numbered steps at block 2.
settings-help-topics-keys-step2-2 = Super and Enter, or Ctrl, Alt and T: open a Terminal.
# Help topic "Keyboard shortcuts": step 3 of the numbered steps at block 2.
settings-help-topics-keys-step2-3 = Super and I: open Settings.
# Help topic "Keyboard shortcuts": step 4 of the numbered steps at block 2.
settings-help-topics-keys-step2-4 = Super and L: lock the screen, once locking is switched on in Settings, Power.
# Help topic "Keyboard shortcuts": step 5 of the numbered steps at block 2.
settings-help-topics-keys-step2-5 = Super, Alt and S: switch reading the screen aloud on or off.
# Help topic "Keyboard shortcuts": step 6 of the numbered steps at block 2.
settings-help-topics-keys-step2-6 = Super and Q: close the window in front.
# Help topic "Keyboard shortcuts": step 7 of the numbered steps at block 2.
settings-help-topics-keys-step2-7 = Super and Up: make the window fill the screen, or return it to its old size.
# Help topic "Keyboard shortcuts": step 8 of the numbered steps at block 2.
settings-help-topics-keys-step2-8 = Super and Down: minimise the window. You can bring it back from the open windows in the ring.
# Help topic "Keyboard shortcuts": step 9 of the numbered steps at block 2.
settings-help-topics-keys-step2-9 = Super and D: hide every window to show the desktop, and bring them back.
# Help topic "Keyboard shortcuts": step 10 of the numbered steps at block 2.
settings-help-topics-keys-step2-10 = Super and N: move keyboard control to the notifications on the screen. Tab moves between their buttons, Enter or Space presses one, and Escape takes you back.
# Help topic "Keyboard shortcuts": step 11 of the numbered steps at block 2.
settings-help-topics-keys-step2-11 = Alt and Tab: bring each open window to the front in turn.
# Help topic "Keyboard shortcuts": step 12 of the numbered steps at block 2.
settings-help-topics-keys-step2-12 = Print, or Super, Shift and S: take a picture of the screen, or of a part of it. Shift and Print: take the whole screen at once.
# Help topic "Keyboard shortcuts": step 13 of the numbered steps at block 2.
settings-help-topics-keys-step2-13 = The volume, mute and brightness keys work as marked.

## Topic: Drives and USB sticks (src/settings/help_topics.rs)

# Title of the Help topic "Drives and USB sticks".
settings-help-topics-disks-title = Drives and USB sticks
# Words people might search for the Help topic "Drives and USB sticks" by, in lower case, separated by spaces.
settings-help-topics-disks-keywords = drives disk disks drive usb stick memory card format wipe erase eject power off space full storage partition part resize shrink grow health smart check repair mount start-up startup fstab name folder mnt
# Help topic "Drives and USB sticks": paragraph 1.
settings-help-topics-disks-p1 = Drives is a window of its own, in All apps. You can also open it from Settings, About this computer. At the top, one sentence says how much room is left, with a ring for each partition in use. Below that it shows every drive in or plugged into this computer, how full each partition is and, where the drive can tell, how healthy it is and how warm.
# Help topic "Drives and USB sticks": step 1 of the numbered steps at block 2.
settings-help-topics-disks-step2-1 = Plug in a USB stick and it appears in the list.
# Help topic "Drives and USB sticks": step 2 of the numbered steps at block 2.
settings-help-topics-disks-step2-2 = Press the folder to see its files, the play button to mount it (make it available) and the stop button to unmount it.
# Help topic "Drives and USB sticks": step 3 of the numbered steps at block 2.
settings-help-topics-disks-step2-3 = Press eject before pulling it out, so nothing is lost. The power button switches off a USB drive that can be switched off, so it's ready to unplug.
# Help topic "Drives and USB sticks": paragraph 3.
settings-help-topics-disks-p3 = The pencil beside a partition opens the things you can change about it:
# Help topic "Drives and USB sticks": step 1 of the numbered steps at block 4.
settings-help-topics-disks-step4-1 = Name: what it's called here and on other computers.
# Help topic "Drives and USB sticks": step 2 of the numbered steps at block 4.
settings-help-topics-disks-step4-2 = Folder: where its files appear, such as /mnt/backup, instead of a folder named after its number. Leave it empty to let the computer choose.
# Help topic "Drives and USB sticks": step 3 of the numbered steps at block 4.
settings-help-topics-disks-step4-3 = Mount at start-up: the partition is ready whenever the computer starts. If the drive isn't there, the computer starts anyway.
# Help topic "Drives and USB sticks": step 4 of the numbered steps at block 4.
settings-help-topics-disks-step4-4 = Size: make it smaller, or larger into the free space just after it.
# Help topic "Drives and USB sticks": step 5 of the numbered steps at block 4.
settings-help-topics-disks-step4-5 = Faults: look for faults and fix them. You can't use the drive while this runs.
# Help topic "Drives and USB sticks": step 6 of the numbered steps at block 4.
settings-help-topics-disks-step4-6 = Format, and Delete partition.
# Help topic "Drives and USB sticks": paragraph 5.
settings-help-topics-disks-p5 = The three dots beside a drive erase the whole drive and set it up again as one partition. Free space has a pencil too, to make a new partition in it. Choose exFAT for a drive shared with Windows and Macs, ext4 for one used only with Linux, and FAT for a small stick or an older device.
# Help topic "Drives and USB sticks": tip 6, shown in a box of its own.
settings-help-topics-disks-tip6 = Formatting, resizing and deleting each need two clicks and your password. They show the drive's name and size so you can check. Trindle never offers any of them for the drive the computer runs from. Erasing can't be undone, so copy anything you want to keep first.

## Topic: Which app opens what (src/settings/help_topics.rs)

# Title of the Help topic "Which app opens what".
settings-help-topics-defaults-title = Which app opens what
# Words people might search for the Help topic "Which app opens what" by, in lower case, separated by spaces.
settings-help-topics-defaults-keywords = default apps browser email open with pdf pictures music video links
# Help topic "Which app opens what": paragraph 1.
settings-help-topics-defaults-p1 = In Settings, Default apps, you choose which app opens web links, emails, pictures, music, videos, PDF files and folders. Only apps that are installed and can open that kind of file are offered.
# Help topic "Which app opens what": tip 2, shown in a box of its own.
settings-help-topics-defaults-tip2 = No web browser comes with Trindle. Get one from Apps first, and you can then choose it here.

## Topic: Keeping the computer up to date (src/settings/help_topics.rs)

# Title of the Help topic "Keeping the computer up to date".
settings-help-topics-updates-title = Keeping the computer up to date
# Words people might search for the Help topic "Keeping the computer up to date" by, in lower case, separated by spaces.
settings-help-topics-updates-keywords = update updates upgrade security new version debian trindle install firmware bios fwupd
# Help topic "Keeping the computer up to date": paragraph 1.
settings-help-topics-updates-p1 = Everything is updated from one place: the Updates page in Apps.
# Help topic "Keeping the computer up to date": step 1 of the numbered steps at block 2.
settings-help-topics-updates-step2-1 = Open Apps and choose Updates. It lists your apps, Debian and Trindle itself. The number of updates waiting is shown on the Apps button in the ring.
# Help topic "Keeping the computer up to date": step 2 of the numbered steps at block 2.
settings-help-topics-updates-step2-2 = Press the update button on each card. You need your password to update Debian and Trindle, but not apps.
# Help topic "Keeping the computer up to date": step 3 of the numbered steps at block 2.
settings-help-topics-updates-step2-3 = Log out and back in after a Trindle update, so you're running the new version.
# Help topic "Keeping the computer up to date": paragraph 3.
settings-help-topics-updates-p3 = Now and then the makers of your computer, or of one of its parts such as a dock or a touchpad, release new firmware. Firmware is the software built into the hardware itself. When there is some for your computer, a Firmware card appears on the same page, with the new version beside each part. Its update button asks for your password. The computer may need to restart to finish, so save your work and, on a laptop, keep the charger plugged in. When there's nothing new, the card doesn't appear at all.
# Help topic "Keeping the computer up to date": tip 4, shown in a box of its own.
settings-help-topics-updates-tip4 = Nothing installs itself. Updates install only when you choose to install them, in the ring or in Apps. Your settings, apps and files are kept through every update.

## Topic: About Trindle (src/settings/help_topics.rs)

# Title of the Help topic "About Trindle".
settings-help-topics-about-title = About Trindle
# Words people might search for the Help topic "About Trindle" by, in lower case, separated by spaces.
settings-help-topics-about-keywords = about version what is debian compositor wayland smithay rust licence license gpl warranty risk who made
# Help topic "About Trindle": paragraph 1.
settings-help-topics-about-p1 = Trindle is a small desktop for Debian 13. All of its own parts are written in Rust. These are its compositor, which puts every window on the screen, and one program for everything else: the ring, Settings, Apps, and the login and lock screens.
# Help topic "About Trindle": paragraph 2.
settings-help-topics-about-p2 = GTK programs draw their own title bars. For the rest, such as the terminal, Qt programs and older X11 ones, the compositor draws the title bar itself, with round buttons on the right like GTK's own windows have. There's a line to hide the window, a square to make it fill the screen, and a cross to close it.
# Help topic "About Trindle": paragraph 3.
settings-help-topics-about-p3 = Trindle is a rolling release, so there's no timetable of big versions twice a year. Improvements arrive as they're ready, each with its own number. Settings, About this computer shows that number, along with the computer's processor, memory, graphics and so on. You can also change the computer's name there. The page also keeps the last ten start-up times, from the power button to the login screen, and you can save the whole list to Downloads with one click. What the computer is doing right now is in Activity, a window of its own.
# Help topic "About Trindle": paragraph 4.
settings-help-topics-about-p4 = Older programs written for X11 rather than Wayland work too. The X server they need, Xwayland, starts the moment the first of them opens, and uses no memory until then.
# Help topic "About Trindle": paragraph 5.
settings-help-topics-about-p5 = It's free software under the GNU General Public License, version 3, so you may use, change and share it. It comes with no warranty of any kind and you use it at your own risk. Keep a backup of everything that matters.
# Help topic "About Trindle": paragraph 6.
settings-help-topics-about-p6 = The website is trindle.org, and the code is at trindle.org/code.
# Help topic "About Trindle": paragraph 7.
settings-help-topics-about-p7 = Settings, Thanks lists the projects Trindle is built on. Each has a link to its home page, and a heart marks those that accept donations.

## Topic: What is running, and why it is busy (src/settings/help_topics.rs)

# Title of the Help topic "What is running, and why it is busy".
settings-help-topics-activity-title = What's running, and why it's busy
# Words people might search for the Help topic "What is running, and why it is busy" by, in lower case, separated by spaces.
settings-help-topics-activity-keywords = activity task manager system monitor running programs processes processor cpu memory ram busy slow fan hot threads stop close frozen not responding bubbles swap temperature
# Help topic "What is running, and why it is busy": paragraph 1.
settings-help-topics-activity-p1 = Activity is a window of its own that shows what the computer is doing right now. Open it from All apps, under System, or from Settings, About this computer.
# Help topic "What is running, and why it is busy": paragraph 2.
settings-help-topics-activity-p2 = A sentence at the top says how things are, such as "No programs are working hard", or which program is keeping the computer busy. Below it is a picture of the computer. The big circle is the memory, and each program is a bubble in it, as big as the memory it uses. A program that's working glows and beats, faster and brighter the harder it works: blue for a little work, orange for hard work and red for full speed. Round the edge is a bar for each of the processor's threads, which grows and warms as it gets busier.
# Help topic "What is running, and why it is busy": paragraph 3.
settings-help-topics-activity-p3 = Trindle adds together all the parts of a program, so Firefox is one bubble however many tabs it has. Two quiet grey bubbles show the rest: Trindle desktop, which is everything Trindle runs to be the desktop, and Linux and other programs. Trindle's apps, such as Settings and Apps, count as programs, not as part of the desktop.
# Help topic "What is running, and why it is busy": paragraph 4.
settings-help-topics-activity-p4 = Down the side, four rings show the memory, the processor, the disk and the swap file. On a computer with no swap file, the last one shows the processor's temperature instead. In the memory ring, Trindle's own share is in the highlight colour, and your programs and Linux are in blue.
# Help topic "What is running, and why it is busy": step 1 of the numbered steps at block 5.
settings-help-topics-activity-step5-1 = If a program has stopped responding, click its bubble. A card opens beside it, showing what it's doing in plain figures.
# Help topic "What is running, and why it is busy": step 2 of the numbered steps at block 5.
settings-help-topics-activity-step5-2 = Press the stop button on the card. It fills with the highlight colour and shows a tick.
# Help topic "What is running, and why it is busy": step 3 of the numbered steps at block 5.
settings-help-topics-activity-step5-3 = Press it again within three seconds. Trindle asks the program to close, and anything you haven't saved in it may be lost.
# Help topic "What is running, and why it is busy": tip 6, shown in a box of its own.
settings-help-topics-activity-tip6 = Trindle's own programs and system programs have no stop button.

## Topic: Trying Trindle in a virtual machine (src/settings/help_topics.rs)

# Title of the Help topic "Trying Trindle in a virtual machine".
settings-help-topics-virtual-machine-title = Trying Trindle in a virtual machine
# Words people might search for the Help topic "Trying Trindle in a virtual machine" by, in lower case, separated by spaces.
settings-help-topics-virtual-machine-keywords = virtual machine vm virtualbox boxes qemu kvm vmware hyper-v parallels utm slow 3d acceleration graphics
# Help topic "Trying Trindle in a virtual machine": paragraph 1.
settings-help-topics-virtual-machine-p1 = Trindle runs in a virtual machine, and tells you so when it starts in one. You'll see this in the welcome window on the USB stick, and once in a notification after installing. On a real computer it never mentions it.
# Help topic "Trying Trindle in a virtual machine": paragraph 2.
settings-help-topics-virtual-machine-p2 = A virtual machine made without 3D graphics has no graphics chip for Trindle to use, so the processor draws everything instead. Windows, videos and web pages feel slower than they would on a real computer, and Trindle leaves out its animations to help. Switching on 3D acceleration in the virtual machine's display settings makes a big difference. In VirtualBox it's called Enable 3D Acceleration. In GNOME Boxes and virt-manager it's 3D acceleration or Virtio with OpenGL.
# Help topic "Trying Trindle in a virtual machine": tip 3, shown in a box of its own.
settings-help-topics-virtual-machine-tip3 = Judge Trindle's speed on a real computer, because that's where it's meant to run.

## Topic: If something goes wrong (src/settings/help_topics.rs)

# Title of the Help topic "If something goes wrong".
settings-help-topics-trouble-title = If something goes wrong
# Words people might search for the Help topic "If something goes wrong" by, in lower case, separated by spaces.
settings-help-topics-trouble-keywords = problem help stuck frozen crash login error broken recover terminal
# Help topic "If something goes wrong": paragraph 1.
settings-help-topics-trouble-p1 = If an app stops responding, right-click it in the ring and choose Close window. If that doesn't work, or the computer is slow and you don't know why, open Activity. It shows which programs are busy, and its stop button closes a program.
# Help topic "If something goes wrong": paragraph 2.
settings-help-topics-trouble-p2 = If the login screen ever fails to appear, or logging in keeps returning you to it, press Ctrl, Alt and F2 together. An ordinary text login appears, where you can log in and fix things. Ctrl, Alt and F7 takes you back to the graphical screen.
# Help topic "If something goes wrong": tip 3, shown in a box of its own.
settings-help-topics-trouble-tip3 = Trindle keeps a record of each session in .local/state/trindle/session.log in your home folder. It can help you work out what went wrong.

## Topic: A chip with no driver (src/settings/help_topics.rs)

# Title of the Help topic "A chip with no driver".
settings-help-topics-hardware-no-driver-title = A chip with no driver or firmware
# Words people might search for the Help topic "A chip with no driver" by, in lower case, separated by spaces.
settings-help-topics-hardware-no-driver-keywords = driver firmware wifi wi-fi bluetooth sound graphics chip missing no driver mediatek mt7902 not working usb stick
# Help topic "A chip with no driver": paragraph 1.
settings-help-topics-hardware-no-driver-p1 = A chip needs two things to work. One is a driver in the kernel, which is the core of Linux. Many chips also need a firmware file, which is a small program the chip loads when it starts. When you open Settings, Trindle looks at your computer's chips and tells you here if one has neither.
# Help topic "A chip with no driver": paragraph 2.
settings-help-topics-hardware-no-driver-p2 = A very new chip may have no driver in the kernel Debian ships. The MediaTek MT7902 is one. Its Wi-Fi driver first appears in Linux 7.1, and its Bluetooth firmware is in firmware-mediatek 20260916. Trindle ships only Debian's stable kernel, so this chip stays without Wi-Fi until that kernel gains the driver.
# Title of the Help topic "A Bluetooth device will not connect".
settings-help-topics-bluetooth-connect-title = A Bluetooth device won't connect
# Words people might search for the Help topic "A Bluetooth device will not connect" by, in lower case, separated by spaces.
settings-help-topics-bluetooth-connect-keywords = bluetooth connect connecting forget pair again headphones not working paired before old pairing key authentication
# Help topic "A Bluetooth device will not connect": paragraph 1.
settings-help-topics-bluetooth-connect-p1 = Switch the device on and keep it near. If it was paired before and won't connect, or connects and drops at once, the pairing saved on this computer may no longer match the one in the device. This can happen after you reset the device or pair it with another computer.
# Help topic "A Bluetooth device will not connect": paragraph 2.
settings-help-topics-bluetooth-connect-p2 = Press Forget and pair again beside the device, or the bin to forget it without pairing again. Then put the device into pairing mode, as its instructions say, and press Pair beside it when it shows under devices nearby.
# Help topic "A Bluetooth device will not connect": paragraph 3.
settings-help-topics-bluetooth-connect-p3 = If it still fails, the sentence under the device says why. It might say Bluetooth is switched off, the device didn't respond, a number was refused, or the device's sound isn't ready. If a device doesn't respond within half a minute, switch it off and on, then try again.
# Title of the Help topic "Headphones sound basic".
settings-help-topics-bluetooth-codec-title = Headphones sound poor
# Words people might search for the Help topic "Headphones sound basic" by, in lower case, separated by spaces.
settings-help-topics-bluetooth-codec-keywords = bluetooth sound quality codec headphones basic sbc switched off on
# Help topic "Headphones sound basic": paragraph 1.
settings-help-topics-bluetooth-codec-p1 = When Bluetooth is switched off and on, the sound system can miss the better ways of sending sound to headphones, and falls back to a basic one. A moment after Bluetooth comes back, Trindle restarts the Bluetooth part of your sound system for you.
# Help topic "Headphones sound basic": paragraph 2.
settings-help-topics-bluetooth-codec-p2 = If the restart didn't work, log out and in again and the better quality comes back.
# Help topic "A chip with no driver": paragraph 3.
settings-help-topics-hardware-no-driver-p3 = Until then, a USB Wi-Fi stick works on any computer with no setup. Plug it in and it shows in Network. This message doesn't change anything on your computer.

## Topic: Playing games (src/settings/help_topics.rs)

# Title of the Help topic "Playing games".
settings-help-topics-games-title = Playing games
# Words people might search for the Help topic "Playing games" by, in lower case, separated by spaces.
settings-help-topics-games-keywords = games game steam proton windows launcher ubisoft controller gamepad anti-cheat vanguard black screen will not start crash alt-tab hdr
# Help topic "Playing games": paragraph 1.
settings-help-topics-games-p1 = Steam works. Install it from Apps. It runs as a Flatpak, separately from your other apps, and plays Windows games through Proton. Ubisoft Connect has been checked. Other launchers usually work inside Steam, but each can break after its own update. Trindle hasn't checked games made for Linux, DirectX 12 games or older 32-bit games yet.
# Help topic "Playing games": paragraph 2.
settings-help-topics-games-p2 = Trindle raises the memory limit and the file limit that big games need. A game that grabs the mouse keeps it until you press Alt and Tab. A full-screen game stays full screen behind the window you switched to. The screen stays awake while a full-screen game is in front.
# Help topic "Playing games": paragraph 3, before the numbered steps.
settings-help-topics-games-p3 = When a game won't start, try these in order.
# Help topic "Playing games": step 1 of the numbered steps at block 4.
settings-help-topics-games-step4-1 = Close Steam completely and open it again. Games may not start if Steam has been running for days.
# Help topic "Playing games": step 2 of the numbered steps at block 4.
settings-help-topics-games-step4-2 = Wait. The first time a game starts, it builds its shaders, and the screen can stay black for a few minutes.
# Help topic "Playing games": step 3 of the numbered steps at block 4.
settings-help-topics-games-step4-3 = Check the Proton version in the game's Properties, under Compatibility, and try another one.
# Help topic "Playing games": step 4 of the numbered steps at block 4.
settings-help-topics-games-step4-4 = Log in to the game's own launcher, such as Ubisoft Connect. Its window can hide behind the full-screen game. Press Alt and Tab to find it.
# Help topic "Playing games": paragraph 5.
settings-help-topics-games-p5 = Anti-cheat that runs inside the kernel, such as Vanguard, can't run on Linux at all. Easy Anti-Cheat and BattlEye work only for a game whose publisher has allowed Linux.
# Help topic "Playing games": paragraph 6.
settings-help-topics-games-p6 = The keyboard and mouse work. PlayStation, Switch and many Xbox controllers, and Steam Input, may not work in games yet. Trindle has a small service for game controllers, but the parts that use it aren't finished.
# Help topic "Playing games": tip 7.
settings-help-topics-games-tip7 = HDR and variable refresh rate aren't supported yet.
