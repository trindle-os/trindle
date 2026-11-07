
# machine translated (Swahili), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Kuhusu kompyuta hii

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = jina la mwenyeji jina la kompyuta jina la mtandao kuhusu mfumo toleo debian trindle kumbukumbu ram kichakataji cpu diski michoro skrini modeli kernel shughuli programu zinazoendeshwa kidhibiti kazi kifuatilia mfumo diski usb nafasi hostname computer name network name about system version memory processor disk graphics screen model kernel activity running programs busy task manager system monitor disks drives space

# Text behind the info button of the About page.
settings-about-help = Jina la kompyuta hii jinsi kompyuta nyingine kwenye mtandao zinavyoliona, imetengenezwa kwa nini, na inachukua muda gani kuanza. Inachofanya sasa hivi, na programu zipi zinatumia kumbukumbu na kichakataji, vipo kwenye Shughuli, na diski pamoja na nafasi zake kwenye Diski, kila moja ikiwa dirisha lake: mishale ya Maelezo zaidi huzifungua.

# What is shown when a fact about the computer cannot be found.
settings-about-unknown = Haijulikani

# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
        [one] { $model }, uzi { $cores }
       *[other] { $model }, nyuzi { $cores }
    }

# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Michoro ya Intel

# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = Michoro ya AMD

# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = Michoro ya NVIDIA

# The graphics line inside a virtual machine.
settings-about-gpu-virtual = Michoro ya mashine pepe

# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = Michoro

# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height } kwa { $hz } Hz

# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = { $amount } inatumika

# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = { $amount } bure

# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = kati ya { $amount }

# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }, imewashwa kwa { $on }.

# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }, imewashwa kwa { $on }. Diski imejaa karibu kabisa.

# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = sekunde { $n }

# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = dakika { $n }

# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
        [one] dakika { $n }
       *[other] dakika { $n }
    }

# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
        [one] saa { $n }
       *[other] saa { $n }
    }

# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
        [one] siku { $n }
       *[other] siku { $n }
    }

# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = Herufi, nambari na vistariungio tu, bila kuanza wala kuishia na kistariungio.

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = Inasubiri nenosiri...

# Red words under the computer name box when renaming failed.
settings-about-name-failed = Kompyuta haikuweza kubadilishwa jina.

# Words under the computer name box after renaming worked.
settings-about-name-done = Programu ambazo tayari ziko wazi zitaona jina jipya baada ya kuingia tena.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Jina la kompyuta

# Tip of the save button beside the computer name box.
settings-about-rename = Badilisha jina la kompyuta hii

# Bold line of the computer name setting.
settings-about-name = Jina la kompyuta

# Quiet line under the computer name setting.
settings-about-name-sub = Herufi, nambari na vistariungio.

# Heading of the card with the computer's name.
settings-about-name-group = Jina

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = Jinsi kompyuta hii inavyoonekana kwenye mtandao, na kwenye kidokezo cha Terminal. Herufi, nambari na vistariungio, bila kuanza wala kuishia na kistariungio. Kulibadilisha huomba nenosiri lako.

# Heading of the card about the computer's hardware.
settings-about-hardware = Kompyuta

# Text behind the info button of the hardware card.
settings-about-hardware-about = Mtengenezaji na modeli, kichakataji na nyuzi ngapi kinaendesha kwa wakati mmoja, michoro, na kila skrini inayotumika pamoja na ukubwa na kasi yake ya kuonyesha upya.

# Quiet line under the maker and model.
settings-about-hw-computer = Kompyuta

# Quiet line under the processor.
settings-about-hw-processor = Kichakataji

# Quiet line under the graphics.
settings-about-hw-graphics = Huchora picha

# Quiet line under the screens.
settings-about-hw-screen = Skrini

# Heading of the card about the software.
settings-about-software = Programu

# Text behind the info button of the software card.
settings-about-software-about = Trindle ni eneo-kazi juu ya Debian, ambayo hutunza programu zilizo chini. Kernel ni sehemu inayozungumza na vifaa, na kikusanyaji cha Trindle chenyewe huchora eneo-kazi, kwenye Wayland.

# Quiet line under the Debian version.
settings-about-sw-debian = Huendeshwa chini

# Quiet line under the kernel version.
settings-about-sw-kernel = Kernel

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Kikusanyaji cha Trindle chenyewe

# Quiet line under the compositor line.
settings-about-sw-compositor-sub = Huchora eneo-kazi, kwenye Wayland

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] programu { $n }
       *[other] programu { $n }
    }

# Quiet line under the number of apps.
settings-about-sw-apps-sub = Zimesakinishwa, tayari kwenye duara

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Maelezo zaidi

# Text behind the info button of the More detail card.
settings-about-more-about = Inachofanya sasa hivi, na programu zipi zinatumia kumbukumbu na kichakataji, vipo kwenye Shughuli. Kila diski, jinsi ilivyojaa na hali yake, na USB stick, vipo kwenye Diski.

# Bold line of the setting that opens Activity.
settings-about-activity = Ona kinachoendeshwa

# Quiet line under the setting that opens Activity.
settings-about-activity-sub = Programu zinazotumia kumbukumbu na kichakataji.

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Shughuli

# Bold line of the setting that opens Drives.
settings-about-drives = Diski na nafasi

# Quiet line under the setting that opens Drives.
settings-about-drives-sub = Kila diski, jinsi ilivyojaa na hali yake.

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Diski

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Nyakati za kuanza hazikuweza kuhifadhiwa kwenye Downloads.

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = Kwenye Downloads, kama { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = Hifadhi

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Hifadhi nyakati zote za kuanza kwenye Downloads

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Kila kuanza, kuhifadhi

# Quiet line under that setting.
settings-about-times-keep-sub = Huhifadhiwa kwenye Downloads kama lahajedwali.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Kila safu ni kuanza mara moja, ya karibuni zaidi iko kulia.

# Heading of the card with the last ten start-up times.
settings-about-times = Kuanza kumi kwa mwisho

# Text behind the info button of the start-up times card.
settings-about-times-about = Kila kuanza hupimwa kutoka kitufe cha kuwasha hadi skrini ya kuingia, bila kuhesabu muda wa kuandika nenosiri la diski. Ya karibuni zaidi iko juu.

# Heading of the memory tile.
settings-about-memory = Kumbukumbu

# Heading of the disk tile.
settings-about-disk = Diski


# Heading of the card with the switch for looking for updates.
settings-about-updates = Masasisho

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle hutafuta matoleo mapya ya programu, programu jalizi, programu dhibiti na yenyewe muda mfupi baada ya kuingia na kila baada ya saa chache, na marekebisho ya usalama ya Debian huingia kila siku. Ikizimwa, Trindle hutafuta tu unapobonyeza Kagua masasisho kwenye Programu, au kitufe cha masasisho kwenye pete. Marekebisho ya usalama ya Debian yana swichi yao; ikizimwa, hayasakinishwi yenyewe tena na hitilafu zinazojulikana hubaki wazi hadi usasishe. Kubadilisha swichi yoyote kunaomba nenosiri la msimamizi wako, ili huduma za kompyuta yenyewe pia ziuone uchaguzi huo.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Kagua masasisho

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Inasubiri nenosiri...

# Quiet line under that switch.
settings-about-updates-by-itself-sub = Imezimwa, hakuna kinachotafuta usipoomba.



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Masasisho ya usalama

# Quiet line under the Security updates switch when it is on.
settings-about-security-sub = Marekebisho ya usalama husakinishwa yenyewe kila siku.

# Warning line under the Security updates switch when it is off.
settings-about-security-off = Marekebisho ya usalama hayasakinishwi yenyewe tena.

