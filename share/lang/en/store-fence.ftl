# Apps: the fence AI programs run in, and the window that asks the person (src/store/fence.rs, fence_window.rs).

## The fence's settings (src/store/fence.rs)

# Red words under the box that names a program for the fence when the name cannot be used.
store-fence-bad-name = That isn't a valid program name.
# Red words when a fence setting could not be written to its file.
store-fence-not-saved = That wasn't saved.
# Red words when the folder chosen to share does not exist.
store-fence-no-folder = Choose a folder that exists.
# Red words when the whole home folder is chosen to share.
store-fence-whole-home = The whole home folder stays private.
# Red words when a folder that holds the home folder is chosen to share.
store-fence-holds-home = This folder contains your home folder, so it stays private.
# Red words when a folder outside the home folder is chosen to share.
store-fence-outside-home = Only folders inside your home folder can be shared.
# Red words when a hidden folder is chosen to share.
store-fence-hidden = Hidden folders always stay private.
# Red words when a folder that keeps passwords or mail is chosen to share.
store-fence-passwords = This folder holds passwords or mail, so it stays private.
# Red words when a program has no folder of settings of its own to use.
store-fence-no-settings = That program has no settings folder to use.
# Red words under the box that names a site AI programs may reach, when the name is not one.
store-fence-bad-site = Type a site's name, such as api.example.com, or *.example.com for every site under that name.
# Red words under the box that names a computer AI programs may reach by ssh, when the name is not one.
store-fence-bad-computer = Type a computer's name, such as nas.local.
# Red words under the box of settings for AI programs, when a line is not NAME=value.
store-fence-bad-env = Write each as NAME=value.
# Red words under the box of settings for AI programs, when a name has no capitals.
store-fence-bad-env-capitals = Write each as NAME=value, with capitals.

## The window that asks about an AI program (src/store/fence_window.rs)

# Words above the question when an AI program asks to use a folder.
store-fence-ask-share = An AI program is asking to use this folder. It can't see it at the moment.
# Words above the question when an AI program asks to run something as the administrator.
store-fence-ask-admin = An AI program is asking to run this as the administrator. It would run outside the fence and could see everything you can.
# Small heading of the window that asks about an AI program.
store-fence-ai-program = AI program
# Words beside the switch that lets an AI program only read a folder, and the switch's spoken name.
store-fence-read-only = Only let it read this folder
# Note under the question about a folder.
store-fence-sees-next = It will see the folder the next time the program starts.
# Tip of the round button that refuses a folder.
store-fence-no = No
# Tip of the round button that shares a folder until the session ends.
store-fence-share-session = Share for this session only
# Main button that shares a folder for good.
store-fence-share-always = Always share
# Tip of the round button that refuses to run something as the administrator.
store-fence-dont-allow = Don't allow
# Main button that lets something run as the administrator.
store-fence-allow = Allow

# Said to an AI program: the command it asked to run as administrator is too long to show in full.
store-fence-ask-too-long = That command is too long to be shown whole, so it was not run.

# Said to an AI program: another request is waiting for the person to answer.
store-fence-ask-busy = Another request is waiting for you to answer it.

# Said to an AI program: the person did not allow its command.
store-fence-ask-refused = You did not allow this.

# Said to an AI program that asked for a program the fence does not offer. { $name } is its name.
store-fence-not-here = { $name } is not available inside the fence.

# Said to an AI program: the command it asked for could not be started outside the fence.
store-fence-not-started = It could not be started outside the fence.

# Said to an AI program that asked to share a folder without giving its full path.
store-fence-share-path = Give the full path of the folder.

# Said to an AI program: the person did not share the folder it asked for.
store-fence-share-refused = You did not share it.

# Said to an AI program: the folder it asked for is shared.
store-fence-shared = Shared. It will be there the next time this program starts.

# Said to an AI program that asked for ssh without naming a computer.
store-fence-ssh-name = Name the computer to reach.

# Said to an AI program that asked for ssh to a name that is not a computer ssh may reach from the fence.
store-fence-ssh-bad = AI programs in the fence can't reach that computer with ssh.

# Said to an AI program: the ssh key for AI programs could not be made. { $why } is the reason.
store-fence-key-failed = The key for AI programs could not be made: { $why }

# Said to an AI program that asked to run something as administrator without naming a command.
store-fence-sudo-name = Name a command to run.

# Printed by a stand-in command inside the fence when the fence cannot reach the outside. { $name } is the command.
store-fence-stand-in-gone = { $name } is not available here.

# Printed by a stand-in command inside the fence for a command it does not offer. { $name } is the command.
store-fence-stand-in-outside = { $name } is not available inside the fence. Ask for it to be run outside.

# Printed by a stand-in command inside the fence when its line to the outside is missing. { $name } is the command.
store-fence-stand-in-line = { $name }: the fence's request line is not there.

# Printed by a stand-in command inside the fence when its request could not be sent. { $name } is the command.
store-fence-stand-in-unsent = { $name }: the request was not sent.

# Printed when the kernel refused the fence's file limits. { $why } is the kernel's reason.
store-fence-landlock = Landlock refused: { $why }

# Printed when the fence could not start. { $why } is the reason.
store-fence-start-sockets = The fence did not start: { $why }.

# Printed when the fence could not make a private home. { $path } is the folder, { $why } the reason.
store-fence-start-home = The fence did not start: { $path }: { $why }

# Printed the first time an AI program starts in the fence.
store-fence-signin = AI programs keep their own logins here, apart from yours. Log in inside the program when it asks.

# Printed when an AI program starts in a folder it cannot see.
store-fence-private = Your folders are private from AI programs. Share one in Settings, AI programs, to work in it.

# Printed when the fence could not start because its system call filter could not be made.
store-fence-start-filter = The fence did not start: the system call filter could not be made.

# Printed when the fence could not start because its network limit could not be set up.
store-fence-start-net = The fence did not start: the network limit could not be set up.

# Printed when the fence could not start because bubblewrap would not run. { $why } is the reason.
store-fence-start-bwrap = The fence did not start: bubblewrap would not run: { $why }

# Printed when the program asked for is not found outside the fence. { $name } is its name.
store-fence-not-found = { $name }: not found outside the fence
