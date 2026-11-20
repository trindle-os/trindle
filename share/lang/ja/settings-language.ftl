# machine translated (Japanese), not yet reviewed by a person.

# Settings: the choice of the language Trindle's own programs are written in, on the Region and language page (src/settings/panels/region.rs).

## The language of Trindle's programs (src/settings/panels/region.rs)

# Heading of the card on the Region and language page where the person picks the language Trindle's own programs and Settings are written in.
settings-language-heading = Trindle の言語
# Words under the heading of that card.
settings-language-about = Trindle のプログラムで使う言語を選びます。まだ翻訳されていない部分は英語で表示されます。
# Title of the line with the list of languages.
settings-language-pick = Trindle の言語
# Tip of the list of languages, and its screen reader name.
settings-language-pick-tip = Trindle のプログラムの言語を選ぶ
# Added to the language card's i: when a new choice is taken up.
settings-language-next-start = 設定はすぐに反映され、ほかのプログラムは次に起動したときに反映されます。
# Entry in the list of languages: use the language the computer is set to.
settings-language-auto = システムの言語
# Title of the group of bars showing how much of each language is translated.
settings-language-how-complete = 翻訳の進み具合
# Tip and screen reader name of a bar under a language: how many of the English words it has (done and all are counts).
settings-language-share =
    { $all ->
       *[other] 英語の { $all } 語のうち { $done } 語を翻訳済み
    }
# Words under a language that holds only the words that differ from English and takes the rest from English.
settings-language-variant = 英語と違う言葉だけ

# Under a language whose words were made by machine and not yet checked by a person.
settings-language-machine = 機械翻訳
