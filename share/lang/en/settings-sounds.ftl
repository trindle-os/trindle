# Settings page "Sounds" (event sounds), with the names of the sounds and moments from src/common/sounds.rs.

## Page (src/settings/panels/sounds.rs)

# Name of the Settings page about event sounds, in the list and as its heading.
settings-sounds-title = Sounds
# Words a person might type into the Settings search to find the sounds page; separate them with spaces.
settings-sounds-keywords = sound sounds chime startup start up login log in logout shut down shutdown charger plugged unplugged battery alert beep off quiet own custom add file wav ogg mp3
# Introduction under the heading of the sounds page, and the words behind the page i.
settings-sounds-help = Choose a sound for each moment, such as logging in. You can also add your own sounds.
# First choice in each moment's list of sounds: that moment makes no sound.
settings-sounds-off = Off

## Top sentence

# Sentence at the top of the page when no moment makes a sound.
settings-sounds-top-none = Everything is quiet.
# Sentence at the top of the page when every moment makes a sound.
settings-sounds-top-all = All sounds are on.
# Sentence at the top of the page when some moments make a sound; { $n } is how many of the { $total } do.
settings-sounds-top-some =
    { $n ->
        [one] { $n } of { $total } sounds is on. The rest are off.
       *[other] { $n } of { $total } sounds are on. The rest are off.
    }

## Moments and sounds

# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-open = Trindle opening
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-close = Trindle closing
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-sunrise = Sunrise
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-sunset = Sunset
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-rising = Rising
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-falling = Falling
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-alert = Alert
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-bell = Bell
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-glass = Glass
# Name of a sound that comes with Trindle, in the lists of sounds.
settings-sounds-name-pop = Pop
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-login = Logging in
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-logout = Logging out, restarting and shutting down
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-plug = Charger plugged in
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-unplug = Charger unplugged
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-battery = Battery low
# Name of a moment that can make a sound, as the heading of its line and the name of its list.
settings-sounds-moment-notification = A notification appears
# Heading of the logging out line.
settings-sounds-moment-logout-short = Logging out
# Heading of the notifications line.
settings-sounds-moment-notification-short = Notifications
# Added to the Log in and out card's i.
settings-sounds-moment-logout-about = Also restart and shut down.

## Lines and buttons

# Heading and slider name for the volume of the logging in and out sounds.
settings-sounds-volume-coming = Logging in and out volume
# Heading and slider name for the volume of notification sounds.
settings-sounds-volume-notifications = Volume of notifications
# Heading and slider name for the volume of the charger and battery sounds.
settings-sounds-volume-power = Volume of power sounds
# Name of the round button that plays the sound chosen for a moment, or an added sound.
settings-sounds-hear = Play it
# Name of the button that opens a window to add sound files.
settings-sounds-add = Add sound files
# Name of the button that opens the folder where added sounds are kept.
settings-sounds-open-folder = Open the folder for added sounds
# Red words under the add sounds line when the folder could not be opened.
settings-sounds-folder-not-opened = Couldn't open the folder.
# Red words under the add sounds line when the folder for added sounds could not be made.
settings-sounds-folder-not-made = Trindle couldn't make the folder for your sounds.
# Name of the button that plays the sound being added, when one of the same name is already there.
settings-sounds-hear-new = Play the new sound
# Name of the button that plays the sound already there, when one of the same name is being added.
settings-sounds-hear-old = Play the existing sound
# Heading of the line for adding sounds.
settings-sounds-add-heading = Add sounds
# Name of the bin button that removes a sound added by the person.
settings-sounds-remove = Remove this sound
# Heading of the line shown when no sounds have been added.
settings-sounds-none-yet = None added yet

## Red and green words

# Red words under a moment's line when its chosen sound file is gone.
settings-sounds-gone = That sound is no longer on this computer. Choose a different one.
# Red words under a line when a sound would not play.
settings-sounds-would-not-play = The sound didn't play. Check your speakers in Audio.
# Title of the window for choosing sound files.
settings-sounds-chooser-title = Choose sound files
# Name of the filter in the window for choosing sound files that shows only sounds.
settings-sounds-chooser-filter = Sounds
# Red words under the add sounds line when a chosen file is not a sound; { $name } is the file name.
settings-sounds-not-playable = Trindle can't play { $name } because it doesn't recognise the file type.
# Red words under the add sounds line when a file could not be copied in; { $name } is the file name.
settings-sounds-not-copied = Couldn't copy { $name }.
# Green words under the add sounds line after sounds were added; { $n } is how many.
settings-sounds-added =
    { $n ->
        [one] Added { $n } sound. You can now choose it for any moment above.
       *[other] Added { $n } sounds. You can now choose them for any moment above.
    }
# Red words under the question about a sound of the same name when it could not be replaced; { $name } is the file name.
settings-sounds-not-replaced = Couldn't replace { $name }.
# Green words after one sound took the place of the one there; { $name } is the file name.
settings-sounds-replaced-one = Replaced { $name }.
# Green words after several sounds took the place of the ones there; { $n } of { $total } were done.
settings-sounds-replaced-some = Replaced { $n } of { $total }.
# Red words under a sound's line when it could not be removed.
settings-sounds-not-removed = Couldn't remove it.
# Red words under a moment's line when its sound was removed and the moment could not be set back to its own sound.
settings-sounds-not-set-back = Trindle removed the sound, but couldn't put the usual sound back for that moment.

## Cards

# Title of the card with the sounds for logging in and out and notifications.
settings-sounds-coming-title = Log in and out
# Words behind the i of that card.
settings-sounds-coming-about = Each volume goes from 0 to 100 in steps of ten. You hear the sound once when you let go.
# Title of the card with the charger and battery sounds.
settings-sounds-power-title = Power
# Words behind the i of that card.
settings-sounds-power-about = Sounds for plugging in and unplugging the charger, and for a low battery. The volume goes from 0 to 100 in steps of ten. You hear the sound once when you let go.
# Title of the card with the sounds the person added.
settings-sounds-own-title = Your own sounds
# Words behind the i of that card.
settings-sounds-own-about = WAV, OGG, Opus, FLAC or MP3. Trindle copies them into your own folder, so you can move or delete the originals. You can then choose them for any moment, like the others. If you remove one, any moment that used it goes back to its usual sound.
