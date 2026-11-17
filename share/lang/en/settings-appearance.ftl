# Settings page "Appearance": themes, highlight colour, reading the screen aloud, icons, fonts and the pointer. The names of the palette colours come from the toolkit.

## Page (src/settings/panels/appearance.rs)

# Name of the Settings page about appearance, in the list and as its heading.
settings-appearance-title = Appearance
# Words a person might type into the Settings search to find the appearance page; separate them with spaces.
settings-appearance-keywords = colour colours color colors accent theme themes look skin glass dark light mode sunshine bright icons font cursor pointer screen reader blind read aloud accessibility labels tooltips names pointing
# Introduction under the heading of the appearance page, and the words behind the page i.
settings-appearance-help = Choose light or dark, colours, icons, fonts and the pointer, or switch on the screen reader. Some programs only show a change after you reopen them.
# Question along the bottom of the page while a theme is on trial.
settings-appearance-foot-keep = Keep this look?
# Small words beside the question along the bottom while a theme is on trial.
settings-appearance-foot-escape = Escape undoes it now.

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
settings-appearance-top-unusable = Trindle's own look is in use because the { $theme } theme can't be used. { $why }
# Sentence at the top of the page for a theme that has dark and light looks; { $theme } is the theme, { $look } dark or light, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-theme-look = The { $theme } theme, { $look }, with the { $accent } highlight and { $shape } icons.
# Sentence at the top of the page for a theme with one look; { $theme } is the theme, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-theme = The { $theme } theme, with the { $accent } highlight and { $shape } icons.
# Sentence at the top of the page when no theme is chosen; { $look } is Dark or Light, { $accent } the highlight colour and { $shape } round or square.
settings-appearance-top-look = { $look }, with the { $accent } highlight and { $shape } icons.
# Shape of the icons in the top sentence.
settings-appearance-shape-square = square
# Shape of the icons in the top sentence.
settings-appearance-shape-round = round
# Name of Trindle's own highlight colour, first in the list of colours.
settings-appearance-trindle-yellow = Trindle yellow

## Theme card

# Start of the words behind the i of the reading the screen card.
settings-appearance-reading-says = The screen reader reads out what's on the screen and what you type. Press Super, Alt and S together to switch it on or off.
# Red words under the icon shape line when the icons could not be remade.
settings-appearance-no-icons = Trindle couldn't update all the icons, so some still have their old shape. Choose the set again to retry.
# Small words on a theme's tile when Trindle made some of its colours clearer.
settings-appearance-corrected = Trindle made some text easier to read.
# Said aloud when a theme is tried; { $theme } is its name and { $seconds } how long until the old look comes back.
settings-appearance-aloud-trying =
    { $seconds ->
        [one] Trying the { $theme } theme. The look you had comes back in { $seconds } second unless you press Keep. Press Escape to go back now.
       *[other] Trying the { $theme } theme. The look you had comes back in { $seconds } seconds unless you press Keep. Press Escape to go back now.
    }
# Name of Trindle's own theme when none is named.
settings-appearance-trindle = Trindle
# Said aloud when a theme was kept; { $theme } is its name.
settings-appearance-aloud-kept = Kept the { $theme } theme.
# Red words under the Theme line when the old look could not be written back.
settings-appearance-back-failed = The look you had will come back when the time runs out.
# Said aloud when the look from before is back.
settings-appearance-aloud-back = The look you had is back.
# Title of the window for choosing a theme file.
settings-appearance-chooser-title = Choose a theme
# Name of the filter in that window that shows only themes.
settings-appearance-chooser-filter = Themes
# Green words under the Add a theme line after a theme was added; { $name } is its name.
settings-appearance-added = { $name } is now in the list above.
# Red words under the Theme line when a theme could not be removed.
settings-appearance-not-removed = Couldn't remove the theme.
# Red words under the theme tiles when a theme cannot be used; { $theme } is its name and { $why } the reason.
settings-appearance-cannot-use = Trindle can't use the { $theme } theme. { $why }
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
settings-appearance-theme-about = A theme is a whole look in one file: colours, corners, sizes and spacing. Press a theme to try it. Every Trindle window changes at once. Unless you press Keep, the look you had comes back by itself after 30 seconds, so you're never stuck with a theme you can't read. If a theme has both a dark and a light look, Dark and Light buttons appear beside its picture. Light is easier to see in bright sunshine, and apps from Apps use it the next time they open. A theme with one look has no such buttons. Your highlight colour stays your own in every theme. Trindle only reads a theme you add and never runs it, and it makes any hard-to-read words clearer. To bring back Trindle's own look from a terminal, run trindle-settings --theme default.
# Heading of the line holding the theme tiles.
settings-appearance-theme-line = Theme
# Added after the words behind the i of its card.
settings-appearance-add-theme-about = One .css file, such as one someone has shared with you.
# Words of the link that opens help about what a theme file holds.
settings-appearance-theme-file-help = What's in a theme file

## Colour

# Red words under the colour box when what was typed is not a colour.
settings-appearance-hex-bad = Use 0-9 and a-f, such as #fbc711.
# Spoken description of the small dot that shows the colour typed.
settings-appearance-dot = Colour you typed
# Placeholder of the box for typing a colour code.
settings-appearance-hex-code = Hex code
# Spoken name of the box for typing a colour as a hex code; the example is a code.
settings-appearance-hex-spoken = Any colour, as a hex code such as #fbc711
# Heading of the line for typing any colour.
settings-appearance-any-colour = Any colour
# Added after the words behind the i of its card.
settings-appearance-any-colour-about = Six or eight digits, like #fbc711.
# Title of the Colour card.
settings-appearance-colour-title = Colour
# Words behind the i of the Colour card.
settings-appearance-colour-about = The highlight marks buttons, switches, the ring as it fills, and where you are. Trindle's own windows change at once, and the ring changes the next time it opens. If a colour is too dark to see on a card, Trindle lightens it until it can be read. In a light look, Trindle darkens a colour that is too pale. Choose Dark or Light on a theme's own tile, under Theme.
# Heading of the line with the highlight colour swatches.
settings-appearance-highlight = Highlight

## Reading the screen

# Heading of the line, and name of its switch, for reading the screen aloud.
settings-appearance-reader = Read the screen aloud
# Heading of the line, and name of its switch, for keeping labels while pointing.
settings-appearance-labels = Keep labels while pointing
# Title of the Reading the screen card.
settings-appearance-reading-title = Reading the screen
# Words behind the i of that card; { $says } is the first sentences, from the key reading-says.
settings-appearance-reading-about = { $says } A button's name usually disappears after a second and a half. If you keep labels, it stays for as long as the pointer is on the button. While the screen is read aloud, every button shows its name under it.

## Icons

# Name of the set of round icons.
settings-appearance-icons-circles = Circles
# Name of the set of square icons.
settings-appearance-icons-squares = Squares
# Words beside the turning ring while the icons are remade.
settings-appearance-icons-working = Changing the icons
# Green words under the shape line after the icons were remade.
settings-appearance-icons-changed = Changed
# Name of the button that chooses an icon set; { $name } is the set, such as Circles.
settings-appearance-icons-set = { $name } icons
# Heading of the line for the shape of the icons.
settings-appearance-shape = Shape
# Title of the Icons card.
settings-appearance-icons-title = Icons
# Words behind the i of the Icons card.
settings-appearance-icons-about = The Trindle Icons come in round or square. If Trindle has no icon for an app, it makes one to match. The ring uses the new shape too.

## Text

# Name of the list of font families, and heading of the font line.
settings-appearance-font = Font
# Name of the font line's choice before the font in use has been read, so a screen reader has a word to say.
settings-appearance-font-default = Default
# Name of the list of font sizes.
settings-appearance-font-size = Font size
# Name of the button that adds fonts, and heading of its line.
settings-appearance-add-fonts = Add fonts
# Words of the link that opens help about what a font file must contain.
settings-appearance-fonts-help = What the file must contain
# Title of the Text card.
settings-appearance-text-title = Text
# Words behind the i of the Text card.
settings-appearance-text-about = The font and size that programs use for their words. Fonts you add here, as .ttf or .otf files or a zip of them, are added to the font list for you only. Some programs only show them after you reopen them.
# Title of the window for choosing font files.
settings-appearance-chooser-fonts = Choose fonts
# Red words when a font could not be replaced; { $name } is the file name.
settings-appearance-font-not-replaced = Couldn't replace { $name }.
# Green words after one font took the place of the one there; { $name } is the file name.
settings-appearance-font-replaced-one = Trindle replaced { $name }. Programs you start from now on will use it.
# Green words after several fonts took the place of the ones there; { $n } of { $total } were done.
settings-appearance-font-replaced-some = Trindle replaced { $n } of { $total } fonts. Programs you start from now on will use them.
# Green words after fonts were added; { $n } is how many.
settings-appearance-fonts-added =
    { $n ->
        [one] { $n } font is now in the list above.
       *[other] { $n } fonts are now in the list above.
    }
# Green words after added fonts were already in the list.
settings-appearance-fonts-in-list = Those fonts are already in the list.

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
settings-appearance-cursor = Pointer
# Name of the list of cursor sizes, and heading of its line.
settings-appearance-cursor-size = Pointer size
# Title of the Pointer card.
settings-appearance-pointer-title = Pointer
