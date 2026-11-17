# Settings page "Wallpaper": a plain colour, a picture, or a folder of pictures that change by themselves.

## Page (src/settings/panels/wallpaper.rs)

# Name of the Settings page about the wallpaper, in the list and as its heading.
settings-wallpaper-title = Wallpaper
# Words a person might type into the Settings search to find the wallpaper page; separate them with spaces.
settings-wallpaper-keywords = background picture slideshow photo
# Introduction under the heading of the wallpaper page, and the words behind the page i.
settings-wallpaper-help = Use a plain colour, one picture, or a folder of pictures that change as often as you choose.

## Lists

# Choice in the list of how long each picture stays.
settings-wallpaper-every-10s = 10 seconds
# Choice in the list of how long each picture stays.
settings-wallpaper-every-30s = 30 seconds
# Choice in the list of how long each picture stays.
settings-wallpaper-every-1m = 1 minute
# Choice in the list of how long each picture stays.
settings-wallpaper-every-5m = 5 minutes
# Choice in the list of how long each picture stays.
settings-wallpaper-every-15m = 15 minutes
# Choice in the list of how long each picture stays.
settings-wallpaper-every-30m = 30 minutes
# Choice in the list of how long each picture stays.
settings-wallpaper-every-1h = 1 hour
# Choice in the list of how long each picture stays.
settings-wallpaper-every-1d = 1 day
# Choice in the list of how the picture fits the screen.
settings-wallpaper-fit-fill = Fill the screen
# Choice in the list of how the picture fits the screen.
settings-wallpaper-fit-fit = Fit the whole picture
# Choice in the list of how the picture fits the screen.
settings-wallpaper-fit-center = Centre, actual size
# Choice in the list of how the picture fits the screen.
settings-wallpaper-fit-stretch = Stretch
# Choice in the list of how the picture fits the screen.
settings-wallpaper-fit-tile = Tile
# Choice in the list of the order pictures change in: at random.
settings-wallpaper-order-shuffle = Shuffled
# Choice in the list of the order pictures change in: by file name.
settings-wallpaper-order-name = By file name
# Red words when the program that draws the wallpaper would not start.
settings-wallpaper-no-changer = Trindle couldn't start the program that changes the wallpaper.

## Sentences about the pictures

# Under the large picture, when counting; { $used } of { $all } pictures are used.
settings-wallpaper-count =
    { $all ->
        [one] { $used } of { $all } picture
       *[other] { $used } of { $all } pictures
    }
# Under the large picture; { $folder } is a folder with no pictures in it.
settings-wallpaper-empty-folder = { $folder } has no pictures in it
# Under the large picture, for the pictures that come with Trindle; { $used } of { $all } are used.
settings-wallpaper-count-own =
    { $all ->
        [one] { $used } of { $all } picture that comes with Trindle
       *[other] { $used } of { $all } pictures that come with Trindle
    }
# Under the large picture, for a folder; { $used } of { $all } pictures are used and { $folder } is the folder.
settings-wallpaper-count-folder =
    { $all ->
        [one] { $used } of { $all } picture from { $folder }
       *[other] { $used } of { $all } pictures from { $folder }
    }
# Under the large picture, for one of Trindle's own pictures; { $name } is its name.
settings-wallpaper-file-own = { $name }, one of the pictures that come with Trindle
# Under the large picture when there is only a colour.
settings-wallpaper-plain-below = A plain colour, set below.
# Under the large picture when the picture or folder has gone.
settings-wallpaper-gone-below = The picture or folder is no longer there. Choose another.
# Sentence about the wallpaper when it is a plain colour.
settings-wallpaper-plain = Just a plain colour.
# Sentence about the wallpaper when Trindle's own pictures change; { $every } is how often, { $used } of { $all } pictures are used and { $extra } is any added sentences, each after a space.
settings-wallpaper-changing-own =
    { $all ->
        [one] A new picture every { $every }, from { $used } of the { $all } picture that comes with Trindle.{ $extra }
       *[other] A new picture every { $every }, from { $used } of the { $all } pictures that come with Trindle.{ $extra }
    }
# Sentence about the wallpaper when a folder's pictures change; { $every } is how often, { $used } of { $all } pictures are used, { $folder } is the folder's name and { $extra } is any added sentences, each after a space.
settings-wallpaper-changing-folder =
    { $all ->
        [one] A new picture every { $every }, from { $used } of the { $all } picture in { $folder }.{ $extra }
       *[other] A new picture every { $every }, from { $used } of the { $all } pictures in { $folder }.{ $extra }
    }
# Sentence about the wallpaper when its folder has no pictures; { $folder } is the folder.
settings-wallpaper-empty-folder-choose = The folder { $folder } has no pictures in it. Choose another.
# Added to the sentence about the wallpaper when a folder holds more than Trindle looks through.
settings-wallpaper-more-than-looked-at = The folder has more pictures than Trindle can check, so some may be left out.
# Sentence about the wallpaper for one of Trindle's own pictures; { $name } is its name.
settings-wallpaper-file-own-said = { $name }, one of the pictures that come with Trindle.
# Sentence about the wallpaper for one chosen picture; { $name } is its name, written to read in the middle of a sentence.
settings-wallpaper-file-one = { $name } is always shown.
# Sentence about the wallpaper when the chosen picture cannot be used; { $why } is the reason.
settings-wallpaper-file-refused = Trindle isn't using that picture. { $why } Trindle's own picture is shown instead.
# Sentence about the wallpaper when the picture has gone.
settings-wallpaper-file-gone = The picture isn't there any more. Choose a picture or a folder.

## Lines and buttons

# Said in green under a line after its choice was kept.
settings-wallpaper-saved = Saved
# Said in green under the next picture line after the picture changed.
settings-wallpaper-changed = Changed
# Small word drawn in the corner of a moving picture's tile.
settings-wallpaper-animated = Animated
# Title of the window for choosing a picture.
settings-wallpaper-chooser-title = Choose a picture
# Name of the filter in that window that shows only pictures.
settings-wallpaper-chooser-filter = Pictures
# Title of the window for choosing a folder of pictures.
settings-wallpaper-chooser-folder = Choose a folder of pictures
# Spoken description of the large picture when the wallpaper is a plain colour.
settings-wallpaper-plain-name = A plain colour
# Name of the large picture of what is on the screen.
settings-wallpaper-on-screen = On the screen
# Title of the Wallpaper card.
settings-wallpaper-card-title = Wallpaper
# Heading of the wallpaper line, and name of its switch.
settings-wallpaper-wallpaper = Wallpaper
# Heading of the line for choosing where the picture comes from.
settings-wallpaper-picture = Picture
# Name of the round button that chooses one picture.
settings-wallpaper-choose-picture-button = Choose a picture
# Name of the round button that chooses a folder of pictures.
settings-wallpaper-choose-folder = Choose a folder
# Name of the round button that chooses Trindle's own pictures.
settings-wallpaper-own-pictures = Trindle's own pictures
# Heading of the fit line.
settings-wallpaper-fit = Fit
# Name of the list of how the picture fits the screen.
settings-wallpaper-fit-name = How it fits the screen
# Heading of the colour line.
settings-wallpaper-colour = Colour
# Small words under the colour line when a picture covers it.
settings-wallpaper-colour-hidden = Hidden by the picture.
# Placeholder and name of the box for typing the colour as a hex code; the example is a code.
settings-wallpaper-colour-hex = A hex code, such as #09090b
# Words behind the i of the Wallpaper card.
settings-wallpaper-card-about = Use a plain colour, one picture, or a folder of pictures that change as often as you choose. The colour shows on its own when there is no picture, and around a picture that doesn't cover the screen. Type it as a hex code, such as #09090b.
# Small words on the cards that are greyed while the wallpaper is off.
settings-wallpaper-waits = Switch on Wallpaper to use these.
# Heading of the line, and name of its switch, for changing the picture now and then.
settings-wallpaper-rotate = Change the picture now and then
# Heading of the line, and name of its list, for how long each picture stays.
settings-wallpaper-every = Change every
# Heading of the line, and name of its list, for the order pictures change in.
settings-wallpaper-order = Order
# Heading of the line, and name of its button, for moving to the next picture.
settings-wallpaper-next = Next picture
# Title of the Changing pictures card.
settings-wallpaper-changing-title = Changing pictures
# Words behind the i of that card.
settings-wallpaper-changing-about = Shows every picture in a folder, one after another, as often as you choose here, shuffled or by file name.
# Words behind the i of the Pictures card when pictures change by themselves.
settings-wallpaper-pictures-rotating = Trindle uses every picture that isn't dimmed. Press a picture to skip it, and press it again to bring it back. Nothing is deleted. The large picture is the one on screen now.
# Words behind the i of the Pictures card when one picture is on.
settings-wallpaper-pictures-one = Press a picture to use it.
# Title of the Pictures card.
settings-wallpaper-pictures-title = Pictures
# Spoken description of the empty frame shown when there are no pictures.
settings-wallpaper-no-pictures-name = No pictures yet
# Heading of the line shown when there are no pictures to choose from.
settings-wallpaper-no-pictures = No pictures to choose from
