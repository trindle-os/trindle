
# machine translated (Swahili), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Kuhusu kompyuta hii

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = jina la mwenyeji jina la kompyuta jina la mtandao kuhusu mfumo toleo debian trindle kumbukumbu ram kichakataji cpu diski michoro skrini modeli kernel shughuli programu zinazoendeshwa kidhibiti kazi kifuatilia mfumo diski usb nafasi hostname computer name network name about system version memory processor disk graphics screen model kernel activity running programs busy task manager system monitor disks drives space

# Text behind the info button of the About page.
settings-about-help = Ukurasa huu unaonyesha jina la kompyuta hii jinsi kompyuta nyingine kwenye mtandao zinavyoliona, imetengenezwa kwa nini, na inachukua muda gani kuanza. Shughuli inaonyesha kompyuta inachofanya sasa hivi na programu zipi zinatumia kumbukumbu na kichakataji. Diski inaonyesha diski zako na nafasi zake. Kila moja hufunguka kwenye dirisha lake. Tumia mishale ya Maelezo zaidi kuzifungua.

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
settings-about-name-bad = Tumia herufi, nambari na vistariungio tu. Usianze wala kuishia na kistariungio.

# Words under the computer name box while the password window is open.
# SAFETY: needs a human review before it ships
settings-about-name-waiting = Inasubiri nenosiri lako…

# Red words under the computer name box when renaming failed.
settings-about-name-failed = Haikuweza kubadilisha jina la kompyuta.

# Words under the computer name box after renaming worked.
settings-about-name-done = Programu ambazo tayari ziko wazi zitatumia jina jipya baada ya kuingia tena.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Jina la kompyuta

# Tip of the save button beside the computer name box.
settings-about-rename = Badilisha jina la kompyuta hii

# Bold line of the computer name setting.
settings-about-name = Jina la kompyuta

# Heading of the card with the computer's name.
settings-about-name-group = Jina

# Text behind the info button of the Name card.
# SAFETY: needs a human review before it ships
settings-about-name-about = Hili ni jina la kompyuta kwenye mtandao na kwenye kidokezo cha Terminal. Tumia herufi, nambari na vistariungio, lakini usianze wala kuishia na kistariungio. Unahitaji kuweka nenosiri lako ili kulibadilisha.

# Heading of the card about the computer's hardware.
settings-about-hardware = Kompyuta

# Text behind the info button of the hardware card.
settings-about-hardware-about = Hapa panaonyesha mtengenezaji na modeli ya kompyuta, kichakataji chake na michoro yake. Pia panaonyesha nyuzi ngapi kichakataji kinaweza kuendesha kwa wakati mmoja, na kila skrini inayotumika pamoja na ukubwa na kasi yake ya kuonyesha upya.

# Heading of the card about the software.
settings-about-software = Programu

# Text behind the info button of the software card.
settings-about-software-about = Trindle ni eneo-kazi linaloendeshwa juu ya Debian, ambayo hutunza programu zilizo chini. Kernel ni sehemu inayofanya kazi na vifaa. Kikusanyaji cha Trindle chenyewe huchora eneo-kazi, kwa kutumia Wayland.

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = Kikusanyaji cha Trindle chenyewe

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] programu { $n }
       *[other] programu { $n }
    }

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Maelezo zaidi

# Text behind the info button of the More detail card.
settings-about-more-about = Shughuli inaonyesha kompyuta inachofanya sasa hivi na programu zipi zinatumia kumbukumbu na kichakataji. Diski inaonyesha kila diski, jinsi ilivyojaa na hali yake, pamoja na USB stick zozote.

# Bold line of the setting that opens Activity.
settings-about-activity = Ona kinachoendeshwa

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Shughuli

# Bold line of the setting that opens Drives.
settings-about-drives = Diski na nafasi

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Diski

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Trindle haikuweza kuhifadhi nyakati za kuanza kwenye Downloads.

# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = Tarehe na saa,Sekunde

# File name of the saved start-up times, in Downloads.
settings-about-times-file = Nyakati za kuanza za Trindle.csv

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = Kwenye Downloads, kama { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = Hifadhi

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Hifadhi nyakati zote za kuanza kwenye Downloads

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Hifadhi muda wa kila kuanza

# Quiet line under that setting.
settings-about-times-keep-sub = Huhifadhiwa kwenye Downloads kama lahajedwali.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Kila safu ni kuanza mara moja. Ya karibuni zaidi iko kulia.

# Heading of the card with the last ten start-up times.
settings-about-times = Mara kumi za mwisho za kuanza

# Text behind the info button of the start-up times card.
settings-about-times-about = Trindle hupima kila kuanza kutoka kitufe cha kuwasha hadi skrini ya kuingia. Muda wa kuandika nenosiri la diski haujahesabiwa. Ya karibuni zaidi iko juu.

# Heading of the memory tile.
settings-about-memory = Kumbukumbu

# Heading of the disk tile.
settings-about-disk = Diski

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = Ripoti za matatizo

# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = Ripoti huenda kwa Buildmorphic, jumuiya inayosaidia kujenga Trindle. Unaweza kujiunga kwa barua pepe tu. Unapata sifa na viwango kwa matatizo unayoripoti kwanza, hata kama huwezi kuyarekebisha. Ripoti ina orodha maalum tu ya maelezo, na hakuna kinachotumwa hadi ubonyeze Tuma. Jina la mtumiaji au jina la GitHub ni hiari, na jina lako la mtumiaji si anwani yako ya barua pepe kamwe.

# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = Toa ofa ya kutuma ripoti kitu kikienda vibaya

# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = Jina la mtumiaji la Buildmorphic

# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = Jina la GitHub

# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = Kuhusu jumuiya

# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
        [one] Imezimwa. Ripoti { $n } iliyokuwa ikisubiri imefutwa.
       *[other] Imezimwa. Ripoti { $n } zilizokuwa zikisubiri zimefutwa.
    }

# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
        [one] Imezimwa. Ripoti { $n } iliyokuwa ikisubiri haikuweza kufutwa.
       *[other] Imezimwa. Ripoti { $n } zilizokuwa zikisubiri hazikuweza kufutwa.
    }


# Heading of the card with the switch for looking for updates.
settings-about-updates = Masasisho

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle hukagua matoleo mapya ya programu, programu jalizi, programu dhibiti na yenyewe muda mfupi baada ya kuingia, kisha kila baada ya saa chache. Ukizima hili, Trindle hukagua tu unapobonyeza Kagua masasisho kwenye Programu, au kitufe cha masasisho kwenye pete. Marekebisho ya usalama ya Debian husakinishwa kila siku na yana swichi yao. Ukizima hiyo, hayasakinishwi yenyewe tena, na mapengo ya usalama yanayojulikana hubaki bila kurekebishwa hadi usasishe. Kubadilisha swichi yoyote kunahitaji nenosiri la msimamizi wako, ili kompyuta nzima itumie uchaguzi wako.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Kagua masasisho

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Inasubiri nenosiri lako…



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Masasisho ya usalama

# Warning line under the Security updates switch when it is off.
settings-about-security-off = Marekebisho ya usalama hayasakinishwi yenyewe tena.

# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = Imezimwa, lakini ripoti zinazosubiri za programu jalizi ya Reporting hazikufutwa.
