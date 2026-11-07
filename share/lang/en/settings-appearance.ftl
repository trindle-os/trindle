# Settings page "Appearance": themes, highlight colour, reading the screen aloud, icons, fonts and the pointer. The names of the palette colours come from the toolkit.

## Page (src/settings/panels/appearance.rs)

# Name of the Settings page about appearance, in the list and as its heading.
settings-appearance-title = Appearance
# Words a person might type into the Settings search to find the appearance page; separate them with spaces.
settings-appearance-keywords = colour colours color colors accent theme themes look skin glass dark light mode sunshine bright icons font cursor pointer screen reader blind read aloud accessibility labels tooltips names pointing
# Introduction under the heading of the appearance page, and the words behind the page i.
settings-appearance-help = Light or dark, colours, icons, fonts, the pointer and reading the screen aloud. Some programs only notice after they are reopened.
# Question along the bottom of the page while a theme is on trial.
settings-appearance-foot-keep = Keep this look?
# Small words beside the question along the bottom while a theme is on trial.
settings-appearance-foot-escape = Escape goes back now.

## Looks and the top sentence

# Name of the dark look, on a theme's button and in the top sentence.
settings-appearance-look-dark = Dark
# Name of the light look, on a theme's button and in the top sentence.
settings-appearance-look-light = Light
# The dark look in the middle of a sentence, so in lower case where the language does that.
settings-appearance-look-dark-lower = dark
# The light look in the middle of a sentence, so in lower case where the language does that.
settings-appearance-look-light-lower = light
# Spoken name of a theme's Dark or Light button; { $name } is the theme and { $look } the look.
settings-appearance-theme-spoken = { $name }, { $look }
# Spoken description of the small picture of a theme; { $name } is the theme and { $look } its look, such as dark.
settings-appearance-sample-spoken = How { $name } looks, { $look }
# Said aloud after Dark or Light was chosen; { $look } is dark or light.
settings-appearance-aloud-look = The { $look } look.
# Sentence at the top of the page when a theme cannot be used; { $theme } is its name and { $why } the reason.
settings-appearance-top-unusable = Trindle's own look, because the { $theme } theme cannot be used. { $why }
# Sentence at the top of the page for a theme that has dark and light looks; { $theme } is the theme, { $look } dark or light, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-theme-look = The { $theme } theme, { $look }, with the { $accent } highlight and { $shape } icons.
# Sentence at the top of the page for a theme with one look; { $theme } is the theme, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-theme = The { $theme } theme, with the { $accent } highlight and { $shape } icons.
# Sentence at the top of the page when no theme is chosen; { $look } is Dark or Light, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-look = { $look }, with the { $accent } highlight and { $shape } icons.
# Shape of the icons in the top sentence.
settings-appearance-shape-round = round
# Name of Trindle's own highlight colour, first in the list of colours.
settings-appearance-trindle-yellow = Trindle yellow

## Theme card

# Start of the words behind the i of the reading the screen card.
settings-appearance-reading-says = Says what is on the screen and what you type. Super, Alt and S together also turn it on and off.
# Red words under the icon shape line when the icons could not be remade.
settings-appearance-no-icons = The icons could not be remade, so some keep their old shape. Choose the set again to try once more.
# Small words on a theme's tile when Trindle made some of its colours clearer.
settings-appearance-corrected = Some colours were made clearer to read.
# Said aloud when a theme is tried; { $theme } is its name and { $seconds } how long until the old look comes back.
settings-appearance-aloud-trying =
    { $seconds ->
        [one] Trying the { $theme } theme. The look you had comes back in { $seconds } second unless you press Keep. Escape goes back now.
       *[other] Trying the { $theme } theme. The look you had comes back in { $seconds } seconds unless you press Keep. Escape goes back now.
    }
# Name of Trindle's own theme when none is named.
settings-appearance-trindle = Trindle
# Said aloud when a theme was kept; { $theme } is its name.
settings-appearance-aloud-kept = The { $theme } theme is kept.
# Red words under the Theme line when the old look could not be written back.
settings-appearance-back-failed = The look you had comes back when the time runs out.
# Said aloud when the look from before is back.
settings-appearance-aloud-back = The look you had is back.
# Title of the window for choosing a theme file.
settings-appearance-chooser-title = Choose a theme
# Name of the filter in that window that shows only themes.
settings-appearance-chooser-filter = Themes
# Green words under the Add a theme line after a theme was added; { $name } is its name.
settings-appearance-added = { $name } is in the list above. Press it to use it.
# Red words under the Theme line when a theme could not be removed.
settings-appearance-not-removed = The theme could not be removed.
# Red words under the theme tiles when a theme cannot be used; { $theme } is its name and { $why } the reason.
settings-appearance-cannot-use = The { $theme } theme cannot be used. { $why }
# Heading of the line saying a theme is tried first.
settings-appearance-tried-first = A theme is tried first
# Small words under that line; { $seconds } is how long Keep is asked for.
settings-appearance-tried-first-about =
    { $seconds ->
        [one] Keep is asked at the foot of the window for { $seconds } second.
       *[other] Keep is asked at the foot of the window for { $seconds } seconds.
    }
# Name of the bin button that removes one of the person's own themes.
settings-appearance-remove-theme = Remove this theme
# Name of the button that chooses a theme; { $name } is the theme.
settings-appearance-use-theme = Use { $name }
# Small words on a theme's tile naming its author; { $author } is the name.
settings-appearance-by = By { $author }
# Name of the button that adds a theme, and heading of its line.
settings-appearance-add-theme = Add a theme
# Spoken name of the turning ring shown while themes are being found.
settings-appearance-looking = Looking for themes
# Title of the Theme card.
settings-appearance-theme-card = Theme
# Words behind the i of the Theme card.
settings-appearance-theme-about = A theme is a whole look in one file: colours, corners, sizes and room. Press one to try it: every Trindle window changes at once, and the look you had comes back by itself in 30 seconds unless you press Keep, so a theme that cannot be read never stays. A theme that has a dark look and a light one has Dark and Light beside its picture; Light is easier to see in bright sunshine, and apps from Apps take it the next time they open. A theme with one look has no such buttons. Your highlight colour stays your own in every theme. A theme you add is only read, never run, and words it would make hard to read are made clearer. From a terminal, trindle-settings --theme default always brings Trindle's own look back.
# Heading of the line holding the theme tiles.
settings-appearance-theme-line = Theme
# Small words under that line.
settings-appearance-theme-line-about = The whole look. Dark or Light is beside a theme with both.
# Small words under the Add a theme line.
settings-appearance-add-theme-about = One .css file, such as one somebody has shared.
# Words of the link that opens help about what a theme file holds.
settings-appearance-theme-file-help = What a theme file holds

## Colour

# Red words under the colour box when what was typed is not a colour.
settings-appearance-hex-bad = Six of 0-9 and a-f, such as #fbc711.
# Spoken description of the small dot that shows the colour typed.
settings-appearance-dot = The colour typed
# Placeholder of the box for typing a colour code.
settings-appearance-hex-code = Hex code
# Spoken name of the box for typing a colour as a hex code; the example is a code.
settings-appearance-hex-spoken = Any colour, as a hex code such as #fbc711
# Heading of the line for typing any colour.
settings-appearance-any-colour = Any colour
# Small words under that line.
settings-appearance-any-colour-about = Six or eight digits, such as #fbc711.
# Title of the Colour card.
settings-appearance-colour-title = Colour
# Words behind the i of the Colour card.
settings-appearance-colour-about = The highlight marks buttons, switches, the filling ring and where you are. Trindle's own windows change at once; the ring on its next opening. A colour too dark to see on a card is lightened until it can be read, and in a light look one too pale is darkened. Dark or Light is chosen on a theme's own tile, under Theme.
# Heading of the line with the highlight colour swatches.
settings-appearance-highlight = Highlight
# Small words under that line.
settings-appearance-highlight-about = Buttons, switches and where you are.

## Reading the screen

# Small words under the read the screen aloud switch when a screen reader is installed.
settings-appearance-reader-on = Super, Alt and S turn it on and off too.
# Heading of the line, and name of its switch, for reading the screen aloud.
settings-appearance-reader = Read the screen aloud
# Heading of the line, and name of its switch, for keeping labels while pointing.
settings-appearance-labels = Keep labels while pointing
# Small words under that line.
settings-appearance-labels-about = A button's name stays while you point at it.
# Title of the Reading the screen card.
settings-appearance-reading-title = Reading the screen
# Words behind the i of that card; { $says } is the first sentences, from the key reading-says.
settings-appearance-reading-about = { $says } A button's name usually goes after a second and a half; kept, it stays for as long as the pointer is on it. While the screen is read aloud, every button shows its name under it.

## Icons

# Name of the set of round icons.
settings-appearance-icons-circles = Circles
# Words beside the turning ring while the icons are remade.
settings-appearance-icons-working = Changing the icons
# Green words under the shape line after the icons were remade.
settings-appearance-icons-changed = Changed
# Name of the button that chooses an icon set; { $name } is the set, such as Circles.
settings-appearance-icons-set = { $name } icons
# Heading of the line for the shape of the icons.
settings-appearance-shape = Shape
# Small words under that line.
settings-appearance-shape-about = The Trindle Icons, round or square.
# Title of the Icons card.
settings-appearance-icons-title = Icons
# Words behind the i of the Icons card.
settings-appearance-icons-about = The Trindle Icons come round or square. An app they do not draw gets one made to match, and the ring redraws with the new shape.

## Text

# Name of the list of font families, and heading of the font line.
settings-appearance-font = Font
# Name of the font line's choice before the font in use has been read, so a screen reader has a word to say.
settings-appearance-font-default = Default
# Name of the list of font sizes.
settings-appearance-font-size = Font size
# Small words under the font line.
settings-appearance-font-about = The words in programs' windows.
# Name of the button that adds fonts, and heading of its line.
settings-appearance-add-fonts = Add fonts
# Small words under the Add fonts line.
settings-appearance-add-fonts-about = A .ttf or .otf file, or a zip of them.
# Words of the link that opens help about what a font file must contain.
settings-appearance-fonts-help = What the file must contain
# Title of the Text card.
settings-appearance-text-title = Text
# Words behind the i of the Text card.
settings-appearance-text-about = The font programs use for their words, and its size. Fonts added here, a .ttf or .otf file or a zip of them, join the font list for you alone; some programs only notice after they are reopened.
# Title of the window for choosing font files.
settings-appearance-chooser-fonts = Choose fonts
# Red words when a font could not be replaced; { $name } is the file name.
settings-appearance-font-not-replaced = { $name } could not be replaced.
# Green words after one font took the place of the one there; { $name } is the file name.
settings-appearance-font-replaced-one = Replaced { $name }. Programs started from now on use it.
# Green words after several fonts took the place of the ones there; { $n } of { $total } were done.
settings-appearance-font-replaced-some = Replaced { $n } of { $total }. Programs started from now on use them.
# Green words after fonts were added; { $n } is how many.
settings-appearance-fonts-added =
    { $n ->
        [one] { $n } font is in the list above. Choose it above.
       *[other] { $n } fonts are in the list above. Choose it above.
    }
# Green words after added fonts were already in the list.
settings-appearance-fonts-in-list = Those fonts are in the list already.

## Pointer

# Choice in the list of cursor sizes: the normal size.
settings-appearance-cursor-normal = Normal
# Choice in the list of cursor sizes: a large size.
settings-appearance-cursor-large = Large
# Choice in the list of cursor sizes: the largest size.
settings-appearance-cursor-larger = Larger
# Spoken name of a picture of a cursor size; { $size } is its name, such as Large.
settings-appearance-pointer-spoken = { $size } pointer
# Name of the standard cursor theme (the one every system has).
settings-appearance-cursor-standard = Standard
# Name of the list of cursor themes, and heading of its line.
settings-appearance-cursor = Cursor
# Name of the list of cursor sizes, and heading of its line.
settings-appearance-cursor-size = Cursor size
# Small words under the cursor line.
settings-appearance-cursor-about = How the pointer is drawn.
# Small words under the cursor size line.
settings-appearance-cursor-size-about = Larger is easier to find.
# Title of the Pointer card.
settings-appearance-pointer-title = Pointer
