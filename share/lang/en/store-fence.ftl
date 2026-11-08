# Apps: the fence AI programs run in, and the window that asks the person (src/store/fence.rs, fence_window.rs).

## The fence's settings (src/store/fence.rs)

# Red words under the box that names a program for the fence when the name cannot be used.
store-fence-bad-name = That is not a name a program can have here.
# Red words when a fence setting could not be written to its file.
store-fence-not-saved = It could not be saved.
# Red words when the folder chosen to share does not exist.
store-fence-no-folder = Choose a folder that is there.
# Red words when the whole home folder is chosen to share.
store-fence-whole-home = The whole home folder stays private.
# Red words when a folder that holds the home folder is chosen to share.
store-fence-holds-home = It holds your home folder, so it stays private.
# Red words when a folder outside the home folder is chosen to share.
store-fence-outside-home = Only folders inside your home folder can be shared.
# Red words when a hidden folder is chosen to share.
store-fence-hidden = Hidden folders always stay private.
# Red words when a folder that keeps passwords or mail is chosen to share.
store-fence-passwords = It keeps passwords or mail, so it stays private.
# Red words when a program has no folder of settings of its own to use.
store-fence-no-settings = There is no settings folder to use for that program.
# Red words under the box that names a site AI programs may reach, when the name is not one.
store-fence-bad-site = Type a site's name, such as api.anthropic.com, or *.example.com for every name under it.
# Red words under the box that names a computer AI programs may reach by ssh, when the name is not one.
store-fence-bad-computer = Type a computer's name, such as nas.local.
# Red words under the box of settings for AI programs, when a line is not NAME=value.
store-fence-bad-env = Write each as NAME=value.
# Red words under the box of settings for AI programs, when a name has no capitals.
store-fence-bad-env-capitals = Write each as NAME=value, with capitals.

## The window that asks about an AI program (src/store/fence_window.rs)

# Words above the question when an AI program asks to use a folder.
store-fence-ask-share = An AI program asks to use this folder. It cannot see it now.
# Words above the question when an AI program asks to run something as the administrator.
store-fence-ask-admin = An AI program asks to run this as the administrator. It runs outside the fence and can see everything you can.
# Small heading of the window that asks about an AI program.
store-fence-ai-program = AI program
# Words beside the switch that lets an AI program only read a folder, and the switch's spoken name.
store-fence-read-only = Only let it read this folder
# Note under the question about a folder.
store-fence-sees-next = It sees the folder the next time the program starts.
# Tip of the round button that refuses a folder.
store-fence-no = No
# Tip of the round button that shares a folder until the session ends.
store-fence-share-session = Share for this session only
# Main button that shares a folder for good.
store-fence-share-always = Share always
# Tip of the round button that refuses to run something as the administrator.
store-fence-dont-allow = Don't allow
# Main button that lets something run as the administrator.
store-fence-allow = Allow
