# Settings: the choice of the language Trindle's own programs are written in, on the Region and language page (src/settings/panels/region.rs).

## The language of Trindle's programs (src/settings/panels/region.rs)

# Heading of the card on the Region and language page where the person picks the language Trindle's own programs and Settings are written in.
settings-language-heading = Language of Trindle
# Words under the heading of that card.
settings-language-about = Pick the language Trindle's own programs are written in. Anything not yet translated shows in English.
# Title of the line with the list of languages.
settings-language-pick = Trindle's language
# Tip of the list of languages, and its screen reader name.
settings-language-pick-tip = Choose the language for Trindle's programs
# Words under the list of languages: when a new choice is taken up.
settings-language-next-start = Used by each program when it next starts.
# Entry in the list of languages: use the language the computer is set to.
settings-language-auto = Follow the system
# Title of the group of bars showing how much of each language is translated.
settings-language-how-complete = How much is translated
# Words under that title.
settings-language-how-complete-sub = A full bar means every word is translated.
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
