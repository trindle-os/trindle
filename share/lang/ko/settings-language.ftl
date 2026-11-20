# machine translated (Korean), not yet reviewed by a person.

# Settings: the choice of the language Trindle's own programs are written in, on the Region and language page (src/settings/panels/region.rs).

## The language of Trindle's programs (src/settings/panels/region.rs)

# Heading of the card on the Region and language page where the person picks the language Trindle's own programs and Settings are written in.
settings-language-heading = Trindle 언어
# Words under the heading of that card.
settings-language-about = Trindle 프로그램에 쓸 언어를 고르세요. 아직 번역되지 않은 부분은 영어로 나와요.
# Title of the line with the list of languages.
settings-language-pick = Trindle 언어
# Tip of the list of languages, and its screen reader name.
settings-language-pick-tip = Trindle 프로그램에 쓸 언어 고르기
# Added to the language card's i: when a new choice is taken up.
settings-language-next-start = 설정은 바로 적용되고, 다른 프로그램은 다음에 시작할 때 적용돼요.
# Entry in the list of languages: use the language the computer is set to.
settings-language-auto = 시스템 언어
# Title of the group of bars showing how much of each language is translated.
settings-language-how-complete = 번역된 정도
# Tip and screen reader name of a bar under a language: how many of the English words it has (done and all are counts).
settings-language-share =
    { $all ->
       *[other] 단어 { $all }개 중 { $done }개 번역됨
    }
# Words under a language that holds only the words that differ from English and takes the rest from English.
settings-language-variant = 영어와 다른 단어만 있어요

# Under a language whose words were made by machine and not yet checked by a person.
settings-language-machine = 기계 번역
