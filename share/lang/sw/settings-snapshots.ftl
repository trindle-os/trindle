# machine translated (Swahili), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = Vijipicha vya mfumo

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = kijipicha vijipicha snapshots snapshot rejesha tendua rollback backup nakala sasisho update btrfs snapper boot menu grub matoleo ya awali toleo la zamani faili iliyofutwa rudisha nyumbani home copies

# Help line at the foot of the Snapshots page.
settings-snap-help = Mfumo kama ulivyokuwa kabla na baada ya kila mabadiliko ya programu, na kila unapochukua kijipicha. Kurejesha kijipicha huurudisha mfumo mzima kwake kuanzia mwanzo ujao; faili zako mwenyewe huachwa kama zilivyo ikiwa folda ya nyumbani ina sehemu yake ya diski, kama kisakinishi cha Trindle kinavyoitengeneza. Kompyuta isipoanza, chagua vijipicha vya Trindle kwenye menyu ya kuanzisha. Matoleo ya awali huhifadhi nakala ya faili za kila mtu kila siku, mbali na vijipicha vya mfumo.

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = Hakuna nakala iliyochukuliwa: diski imekaribia kujaa.

# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = leo saa { $time }

# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = jana saa { $time }

# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = tarehe { $date }

# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = Kompyuta haikuanza upya. Ianzishe upya kutoka kwenye pete badala yake.

# Said beside the Take a snapshot line while it works.
settings-snap-taking = Inachukua kijipicha

# Said beside the set aside line while the space is freed.
settings-snap-freeing = Inaachilia...

# Said beside the set aside line once the space is freed.
settings-snap-freed = Imeachiliwa

# Said beside a snapshot while it is restored.
# SAFETY: needs a human review before it ships
settings-snap-restoring = Inarejesha

# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
settings-snap-restored = Mfumo ni kijipicha { $n } kuanzia mwanzo ujao. Chochote kilichobadilika tangu hapo kimewekwa kando, hakijapotea.

# Said beside a snapshot while it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleting = Inafuta

# Said beside a snapshot once it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleted = Imefutwa

# Said beside a setting while the administrator's password is awaited.
# SAFETY: needs a human review before it ships
settings-snap-waiting = Inasubiri nenosiri...

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = Nakala huchukuliwa kila siku

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = Hakuna nakala zaidi zinazochukuliwa

# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = Imehifadhiwa

# Said beside the earlier copies while they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-clearing = Inafuta

# Said beside the earlier copies once they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-cleared = Zimefutwa

# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] Bado hakuna nakala
        [one] Nakala moja
       *[other] Nakala { $n }
    }

# Under the number of earlier copies when there are none.
settings-snap-first-soon = Ya kwanza itachukuliwa ndani ya siku moja.

# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = Zinachukua { $size }, zilipimwa { $when }.

# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = Nafasi yake hupimwa mara moja kwa siku.

# Tip of the button that clears all earlier copies, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-clear-all = Zifute zote sasa

# Heading of the card about earlier versions of files.
settings-snap-earlier = Matoleo ya awali

# Help text of the Earlier versions card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-earlier-about = Trindle inaweza kuhifadhi nakala ya faili za kila mtu kama zilivyokuwa kila siku, ili toleo la zamani la faili, au faili iliyofutwa kwa makosa, ifunguliwe au irudishwe kutoka Folda. Kila mtu anaweza kusoma kwenye nakala yale tu aliyoweza kusoma siku hiyo, kwa hiyo nyumbani pa faragha hubaki pa faragha; faili iliyokuwa wazi kwa wengine hubaki wazi kwao kwenye nakala hiyo hadi iondoke. Nakala ziko mbali na vijipicha vya mfumo: kurejesha mfumo hakubadilishi faili zako, na hizi hazibadilishi mfumo. Gharama ni nafasi: faili unayofuta au kubadilisha bado inachukua nafasi yake hadi nakala ya mwisho inayoishikilia iondoke, kwa hiyo kufuta vitu hakuachilii diski mara moja. Kufuta nakala huiachilia sasa, kwa kila mtu kwenye kompyuta hii, na hakuwezi kutenduliwa. Hakuna nakala inayochukuliwa diski ikiwa karibu kujaa. Kubadilisha lolote kati ya haya kunahitaji nenosiri la msimamizi.

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = Hifadhi matoleo ya awali ya faili zako

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = Faili zilizofutwa hubaki kwa { $time } kabla nafasi yake haijaachiliwa.

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = Hifadhi nakala ya kila siku kwa

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = Nakala ya kila siku inahifadhiwa kwa muda gani

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = Inatazama vijipicha…

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = Vijipicha havikuweza kusomwa.

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = Imeanza kutoka kijipicha { $n }, kwa hiyo hakuna unachofanya sasa kinachohifadhiwa.

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [one] Kijipicha { $n } cha kurudi, cha karibuni zaidi kutoka { $when }.
       *[other] Vijipicha { $n } vya kurudi, cha karibuni zaidi kutoka { $when }.
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = Bado hakuna vijipicha. Kimoja huchukuliwa kabla ya kila mabadiliko ya programu.

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, kimehifadhiwa hadi kifutwe

# Tip of the button that restores the system to a snapshot, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-restore-this = Rejesha mfumo kwenye hiki

# Tip of the button that deletes a snapshot.
# SAFETY: needs a human review before it ships
settings-snap-delete-this = Futa kijipicha hiki

# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = Anzisha upya sasa

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = Bado havijaandaliwa.

# Under the To have them heading when snapshots are not set up.
settings-snap-reinstall = Sakinisha upya kifurushi cha trindle.

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = Vijipicha havijaandaliwa. Kusakinisha upya kifurushi cha trindle kunaviandaa: sudo apt install --reinstall trindle.

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = aina nyingine

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = Vinahitaji Btrfs; kompyuta hii inatumia { $fs }.

# Under the To have them heading when the computer does not use Btrfs.
settings-snap-choose-btrfs = Chagua Btrfs wakati wa kusakinisha Debian.

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = Vijipicha vinahitaji mfumo kwenye Btrfs, na kompyuta hii inatumia { $fs }. Chagua Btrfs kwa mfumo wakati wa kusakinisha Debian ili kuvipata.

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = Hakuna vijipicha kwenye kompyuta hii.

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = Hakuna vijipicha

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = Hakuna vijipicha kwenye kompyuta hii

# Heading of the line saying how to have snapshots.
settings-snap-to-have = Ili kuvipata

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = Inatazama…

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = Vijipicha havikuweza kusomwa

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = Bado hakuna

# Under the None yet heading.
settings-snap-none-line-sub = Kimoja huchukuliwa kabla ya kila mabadiliko ya programu.

# Help text of the Snapshots card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-list-about = Kila kijipicha kinaweza kurejesha mfumo mzima kama ulivyokuwa wakati huo, kuanzia mwanzo ujao, au kufutwa. Vyote viwili vinahitaji kubonyezwa mara mbili na vinahitaji nenosiri la msimamizi. Baada ya kurejesha, kitufe cha kuanzisha upya kinatokea kando yake.

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = Chukua kijipicha sasa

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = Si wakati umeanza kutoka kijipicha.

# Under the Take a snapshot now heading.
settings-snap-kept-until-you = Kinahifadhiwa hadi ukifute.

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [one] Mfumo mmoja uliowekwa kando kwa kurejesha
       *[other] Mifumo { $n } iliyowekwa kando kwa kurejesha
    }

# Under the set aside heading.
settings-snap-free-space = Achilia nafasi mara kila kitu kikiwa sawa.

# Tip of the button that frees the space the set aside systems use.
# SAFETY: needs a human review before it ships
settings-snap-free-them = Achilia nafasi wanayotumia

# Heading of the card for taking a snapshot.
settings-snap-take-one = Chukua kimoja

# Help text of the Take one card (the information button).
settings-snap-take-about = Chukua kijipicha kabla ya kujaribu kitu ambacho unaweza kutaka kukitendua; kile unachochukua kinahifadhiwa hadi ukifute. Kompyuta ikiwa imeanza kutoka kijipicha, hakuna kitakachochukuliwa kitakachohifadhiwa, kwa hiyo anzisha upya kawaida kwanza. Kila kurejesha huweka kando mfumo ulivyokuwa kabla yake, kwa hiyo hakuna kinachopotea; achilia nafasi wanayotumia mara kila kitu kikiwa sawa.

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = Imeanza kutoka kijipicha { $n }

# Under the Started from snapshot heading.
settings-snap-nothing-kept = Hakuna unachofanya sasa kinachohifadhiwa.

# Heading of a line on the Before restoring card.
settings-snap-files-too = Faili zako ziko kwenye vijipicha pia

# Under the Your files are in the snapshots too heading.
# SAFETY: needs a human review before it ships
settings-snap-files-back = Kurejesha pia kunarudisha faili zako.

# Heading of the card about what to know before restoring.
settings-snap-before = Kabla ya kurejesha

# Help text of the Before restoring card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-before-about = Ukiwa umeanza kutoka kijipicha, hakuna unachofanya sasa kinachohifadhiwa: ikiwa mfumo unapaswa kuwa hivi, bonyeza rejesha kwenye orodha, kisha anzisha upya. Kwenye kompyuta hii folda ya nyumbani inaweza kuwa sehemu ya mfumo, na hapo kurejesha kijipicha pia kunarudisha faili zako mwenyewe kama zilivyokuwa wakati huo, kwa hiyo nakili chochote kipya mahali salama kwanza.



# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = Faili za mfumo zimefungwa

# Under the System files locked switch.
settings-snap-locked-sub = Usakinishaji na masasisho pekee ndivyo huzifungua.

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = Zimefungwa

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = Zimefunguliwa

