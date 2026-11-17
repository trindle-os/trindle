# Settings: the choice of the language Trindle's own programs are written in, on the Region and language page (src/settings/panels/region.rs).

## The language of Trindle's programs (src/settings/panels/region.rs)

# Heading of the card on the Region and language page where the person picks the language Trindle's own programs and Settings are written in.
settings-language-heading = Trindle's language
# Words under the heading of that card.
settings-language-about = Pick the language for Trindle's own programs. Anything that isn't translated yet shows in English.
# Title of the line with the list of languages.
settings-language-pick = Trindle's language
# Tip of the list of languages, and its screen reader name.
settings-language-pick-tip = Choose the language for Trindle's programs
# Added to the language card's i: when a new choice is taken up.
settings-language-next-start = Settings uses it now, other programs the next time they start.
# Entry in the list of languages: use the language the computer is set to.
settings-language-auto = System language
# Title of the group of bars showing how much of each language is translated.
settings-language-how-complete = How much is translated
# Tip and screen reader name of a bar under a language: how many of the English words it has (done and all are counts).
settings-language-share =
    { $all ->
        [one] { $done } of { $all } word translated
       *[other] { $done } of { $all } words translated
    }
# Words under a language that holds only the words that differ from English and takes the rest from English.
settings-language-variant = Only the words that differ from English

# Under a language whose words were made by machine and not yet checked by a person.
settings-language-machine = Machine translated
