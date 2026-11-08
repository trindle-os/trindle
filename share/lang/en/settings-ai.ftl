# Settings: the AI programs page (src/settings/panels/ai_programs.rs).

## The AI programs page (src/settings/panels/ai_programs.rs)

# Name of the AI programs page, in the page list, as its heading and on the card listing the programs.
settings-ai-title = AI programs
# Words people may search for to find the AI programs page, separated by spaces.
settings-ai-keywords = ai programs claude codex cursor gemini aider agent fence sandbox private folders share ssh keys token sudo
# Help line at the foot of the AI programs page.
settings-ai-help = AI coding programs run in a fence that hides your personal files. Share only the folders they need.
# Headline of the AI programs page.
settings-ai-note = This keeps AI programs away from your personal files in everyday use, for example if one is tricked into collecting your bank statements and sending them out. It is a limited protection, not a guarantee: a determined program, a flaw in the computer's software, or you approving a request or sharing a folder can still let something through.
# Note under the headline of the AI programs page.
settings-ai-inside = Everything an AI program starts, such as installed programs, scripts and other programs, stays inside the fence with it.
# Red words when a change to the fence could not be made.
settings-ai-not-changed = It could not be changed.
# Said beside a setting on the AI programs page once it is saved.
settings-ai-saved = Saved
# Title of the window for choosing a folder to share with AI programs.
settings-ai-choose-folder = Choose a folder to share with AI programs
# Red words when the key for AI programs to reach computers by ssh could not be made.
settings-ai-key-not-made = The key could not be made.
# Label of the switch that limits the network of AI programs, and heading of its line.
settings-ai-limit-net = Limit the network to the sites listed here
# Under the Limit the network heading.
settings-ai-limit-net-sub = Off: the network is open. On: only the sites below.
# Tip of the button that removes a site from the list AI programs may reach.
settings-ai-stop-site = Stop AI programs reaching this site
# Example in the box for typing a site AI programs may reach.
settings-ai-eg-site = e.g. api.anthropic.com
# Tip of the button that adds a site AI programs may reach.
settings-ai-let-site = Let AI programs reach this site
# Heading of the line for adding a site.
settings-ai-add-site = Add a site
# Under the Add a site heading.
settings-ai-add-site-sub = Its name, or *.example.com for every name under it.
# Heading of the card that limits the network of AI programs.
settings-ai-network = Network
# Help text of the Network card (the information button).
settings-ai-network-about = Advanced. Off, AI programs use the network as you do. On, they can reach only the sites below, over secure (https) connections, and the fence has no network of its own. A program's tools must use the proxy settings they are given. Add the site of the AI service, and of any package source an install needs.
# Label of the switch that keeps AI programs inside the fence, and heading of its line.
settings-ai-keep-inside = Keep AI programs inside the fence
# Under the fence switch when it is on.
settings-ai-hides-home = Hides your home folder, except the folders shared below.
# Under the fence switch when it is off.
settings-ai-fence-off = The fence is off. AI programs can now see everything you can.
# Heading of the card with the fence switch.
settings-ai-the-fence = The fence
# Help text of The fence card (the information button).
settings-ai-fence-about = The system stays visible and usable to AI programs. Your home folder is replaced by an empty private home that they keep between runs, with only the folders you share. They cannot see your other programs, your session, your keys or your passwords. A program started some other way than through its name on this list is not fenced. If you have installed the assistant connection (trindle-mcp), a connected assistant can read these without asking you: the titles of your open windows and the apps behind them, your installed apps, running programs and their memory use, memory, disk, battery, uptime and network state, your Trindle settings, up to 500 lines of the system journal for any service, your latest crash notes, and the names of your sound devices, paired Bluetooth devices, nearby Wi-Fi networks and drives. It always asks you before it changes anything.
# Label of the switch that fences one AI program: its name is { $name }.
settings-ai-fence-name = Fence { $name }
# Tip of the button that takes one program off the list: its name is { $name }.
settings-ai-take-off = Take { $name } off the list
# Label of the switch for using a program's existing settings folder: its name is { $name }.
settings-ai-own-for = Use my existing settings folder for { $name }
# Heading of the line with the switch for using a program's existing settings folder.
settings-ai-own = Use my existing settings folder
# Under the existing settings folder heading.
settings-ai-own-sub = It holds this program's past conversations.
# Example in the box for typing the name of an AI program.
settings-ai-eg-program = e.g. claude
# Tip of the button that adds an AI program, and heading of its line.
settings-ai-add-program = Add a program
# Under the Add a program heading.
settings-ai-add-program-sub = Type the name you start it by.
# Help text of the AI programs card (the information button).
settings-ai-programs-about = These names start inside the fence. Use my existing settings folder: off, a program starts with a fresh private one. The programs sign in on their own, inside: your own sign-in is not shared.
# Heading of the line shown when no folder is shared with AI programs.
settings-ai-nothing-shared = Nothing is shared
# Under the Nothing is shared heading.
settings-ai-none-seen = AI programs see none of your folders.
# Label of the switch that lets AI programs only read a shared folder.
settings-ai-only-read = Only let it read this folder
# Tip of the button that stops sharing a folder with AI programs.
settings-ai-make-private = Make this folder private again
# Under a shared folder's path, beside the switch that makes it read only.
settings-ai-only-read-short = Only let it read
# Tip of the button that shares a folder, and heading of its line.
settings-ai-share-folder = Share a folder
# Under the Share a folder heading.
settings-ai-share-folder-sub = Seen at the same place, to read and write.
# Heading of the card listing folders shared with AI programs.
settings-ai-shared = Shared with AI programs
# Help text of the shared folders card (the information button).
settings-ai-shared-about = Private from AI programs is the rule for everything else in your home folder. Hidden folders, and folders that keep keys, passwords or mail, can never be shared. A folder shared here is there the next time the program starts.
# Tip of the button that removes a computer from the list AI programs may reach by ssh.
settings-ai-stop-computer = Stop AI programs reaching this computer
# Example in the box for typing a computer AI programs may reach by ssh.
settings-ai-eg-computer = e.g. nas.local
# Tip of the button that adds a computer AI programs may reach by ssh.
settings-ai-let-computer = Let AI programs reach this computer
# Heading of the line for adding a computer.
settings-ai-add-computer = Add a computer
# Under the Add a computer heading.
settings-ai-add-computer-sub = Type its name.
# Note showing the public key to put on each computer: the key is { $key }.
settings-ai-put-key = Put this key on each computer: { $key }
# Tip of the button that makes a key for AI programs.
settings-ai-make-key = Make the key
# Heading of the line for making a key for AI programs.
settings-ai-make-key-for = Make a key for AI programs
# Under the Make a key heading.
settings-ai-make-key-sub = A separate key, kept outside the folders they can see.
# Heading of the card listing computers AI programs may reach by ssh.
settings-ai-ssh = Computers they may reach by ssh
# Help text of the ssh card (the information button).
settings-ai-ssh-about = Inside the fence, ssh asks Trindle, which uses a key of its own and only for the computers listed here. Your own keys are never shown to AI programs.
# Tip of the button that stops giving AI programs a key.
settings-ai-stop-key = Stop giving AI programs this key
# Under the name of a key that AI programs are given.
settings-ai-given = Given to AI programs
# Example in the box for typing a key to give AI programs: a name, an equals sign and the value.
settings-ai-eg-key = NAME=value
# Tip of the button that gives AI programs a key.
settings-ai-give-key = Give AI programs this key
# Heading of the line for adding a key.
settings-ai-add-key = Add a key
# Under the Add a key heading.
settings-ai-add-key-sub = Written as NAME=value.
# Heading of the card listing keys given to AI programs.
settings-ai-keys = Keys given to AI programs
# Help text of the keys card (the information button).
settings-ai-keys-about = Keys and tokens in your session never reach AI programs. A key typed here is given to them, and only here.
