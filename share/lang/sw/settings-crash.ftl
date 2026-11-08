# machine translated (Swahili), not yet reviewed by a person.

# Name of the Crash reports page, in the page list and as its heading.
settings-crash-title = Ripoti za hitilafu

# Words people may search for to find the Crash reports page, separated by spaces.
settings-crash-keywords = ripoti hitilafu makosa hitilafu tuma imetumwa faragha telemetry buildmorphic crash reports errors bugs

# Help line at the foot of the Crash reports page.
settings-crash-help = Programu moja ya Trindle ikiacha kufanya kazi, programu jalizi ya ripoti za hitilafu huweka dokezo fupi kwenye kompyuta hii. Haitumi chochote hadi uendeshe amri yake ya kutuma na useme ndiyo kwa kila ripoti, ambayo huonyesha ripoti nzima kwanza. Jina lako na jina lako la GitHub huingia tu ikiwa umeviandika hapa. Kamwe ujumbe, jina la faili, mahali ulipo wala chochote kuhusu programu nyingine. Kuondoa programu jalizi huzuia yote.

# Heading and label of the switch for sending crashes.
settings-crash-crashes = Hitilafu za kuanguka

# Under the Crashes switch.
settings-crash-crashes-short = Mahali kwenye msimbo wa Trindle ilipoanguka.

# Part of the help text of the What is sent card: what Crashes sends.
settings-crash-crashes-long = Hitilafu za kuanguka hutuma programu, toleo lake, aina ya kuanguka na mahali kwenye msimbo wa Trindle ilipotokea.

# Heading and label of the switch for sending errors.
settings-crash-errors = Makosa

# Under the Errors switch.
settings-crash-errors-short = Jina maalum la kosa, na mahali.

# Part of the help text of the What is sent card: what Errors sends.
settings-crash-errors-long = Makosa hutuma programu, toleo lake, jina maalum ambalo Trindle hulipa kosa na mahali kwenye msimbo wake.

# Heading and label of the switch for sending facts about the computer.
settings-crash-computer = Kuhusu kompyuta hii

# Under the About this computer switch.
settings-crash-computer-short = Linux, kichakataji, kumbukumbu, michoro, skrini.

# Part of the help text of the What is sent card: what About this computer adds.
settings-crash-computer-long = Kuhusu kompyuta hii huongeza kwenye kila ripoti: Linux inayotumika, aina ya kichakataji, kumbukumbu iliyopunguzwa, Wayland au X11, mtengenezaji wa michoro, na idadi ya skrini.

# Heading of the card with the switches for what is sent.
settings-crash-what-is-sent = Kinachotumwa

# Help text of the What is sent card: { $switches } is what each switch sends.
settings-crash-what-about = { $switches } Ripoti hazibebi jina isipokuwa uandike jina lako la mtumiaji la Buildmorphic kwenye kisanduku hapa chini, ambalo linakupa sifa tu.

# Red words when a setting could not be changed: the reason is { $why }.
settings-crash-nothing-changed-because = Hakuna kilichobadilishwa, kwa sababu { $why }.

# Red words when the file that keeps the switches was not written by Trindle.
settings-crash-file-not-as-wrote = Hakuna kilichobadilishwa, kwa sababu faili inayoihifadhi haiko kama Trindle ilivyoiandika.

# Red words when there is no folder to keep the settings in.
settings-crash-no-settings-folder = Hakuna folda ya mipangilio.

# Red words when the folder for the settings could not be made.
settings-crash-folder-not-made = Folda ya mipangilio haikuweza kutengenezwa.

# Red words when a setting could not be written to its file.
settings-crash-not-written = Mpangilio haukuweza kuandikwa.

# Name of a sent report that was a crash: the signal is { $signal } and the program { $program }.
settings-crash-what-crash = Kuanguka ({ $signal }) katika { $program }

# Name of a sent report that was an error: the error is { $code } and the program { $program }.
settings-crash-what-error = Kosa { $code } katika { $program }

# In the list of sent reports: the report was taken.
settings-crash-taken = Imepokelewa

# In the list of sent reports: the report was not sent because the bug tracker is closed.
settings-crash-tracker-closed = Haikutumwa: mfuatiliaji umefungwa

# In the list of sent reports: the report was not sent because it was too big.
settings-crash-too-big = Haikutumwa: ni kubwa mno

# In the list of sent reports: the report was not sent because it was refused.
settings-crash-refused = Haikutumwa: ilikataliwa

# In the list of sent reports: the report was not sent because too many came at once.
settings-crash-too-many = Haikutumwa: nyingi mno kwa wakati mmoja

# In the list of sent reports: the report was not sent, no reason known.
settings-crash-not-sent = Haikutumwa

# Said beside the link line once it is saved.
settings-crash-saved = Imehifadhiwa

# Tip of the button that clears the list of sent reports.
# SAFETY: needs a human review before it ships
settings-crash-clear = Futa orodha

# Red words when the list of sent reports could not be cleared.
settings-crash-not-cleared = Orodha haikuweza kufutwa.

# Said beside the list of sent reports once it is cleared.
settings-crash-removed = Imeondolewa

# Heading of the line that holds the clear button.
settings-crash-kept-here = Imehifadhiwa kwenye kompyuta hii tu

# Under the Kept on this computer only heading.
settings-crash-newest-first = Mpya zaidi kwanza, hadi hamsini.

# Heading of the line shown when no report has been sent.
settings-crash-nothing-yet = Bado hakuna

# Under the Nothing yet heading.
settings-crash-listed-here = Kila ripoti inaorodheshwa hapa, kama ilivyotumwa hasa.

# Tip of the button that shows exactly what one report held.
settings-crash-show-sent = Onyesha kilichotumwa hasa

# Under a sent report: { $when } is the time, { $n } is how many times it happened (0 when once) and { $answer } how it went.
settings-crash-sent-sub =
    { $n ->
        [0] { $when }: { $answer }
        [one] { $when }, mara { $n }: { $answer }
       *[other] { $when }, mara { $n }: { $answer }
    }

# Heading of the card listing reports that were sent.
settings-crash-sent-so-far = Zilizotumwa hadi sasa

# Help text of the Sent so far card (the information button).
settings-crash-sent-about = Kila ripoti inaorodheshwa kama ilivyotumwa hasa, mpya zaidi kwanza, na huhifadhiwa kwenye kompyuta hii tu. Imepokelewa inamaanisha Buildmorphic inayo. Haikutumwa: mfuatiliaji umefungwa inamaanisha mfuatiliaji wa hitilafu wa Trindle haupokei ripoti; ilikataliwa inamaanisha ilikataliwa kwa kuwa imetengenezwa vibaya; nyingi mno inamaanisha nyingi mno zilifika kwa wakati mmoja.

# Headline of the page when the add-on is not installed and the person is asked after a crash.
settings-crash-sum-asked = Baada ya hitilafu unaulizwa, ukionyeshwa ripoti nzima, kama utaituma.

# Headline of the page when nothing is sent and the person is not asked.
settings-crash-sum-nothing = Hakuna kinachotumwa, na hauulizwi baada ya hitilafu.

# Headline of the page when both crashes and errors are turned off.
settings-crash-sum-none = Hakuna kinachotumwa: hitilafu za kuanguka na makosa vyote vimezimwa.

# Headline of the page: what is reported (crashes, errors or both) and under whose name (named, linked or anonymous) in { $which }, and the Buildmorphic name in { $name }.
settings-crash-sum-reported =
    { $which ->
        [errors] Makosa ya programu za Trindle huripotiwa bila kutaja jina.
        [both] Hitilafu za kuanguka na makosa ya programu za Trindle huripotiwa bila kutaja jina.
       *[crashes] Hitilafu za kuanguka za programu za Trindle huripotiwa bila kutaja jina.
    }

# Label of the switch for asking about problems and sharing reports, and heading of its line.
settings-crash-ask = Uliza kuhusu matatizo na kushiriki ripoti

# Under the Ask about problems heading.
# SAFETY: needs a human review before it ships
settings-crash-ask-sub = Kuzima huficha yote na kufuta ripoti zinazosubiri.

# Said beside the Ask switch when it is turned off and no reports were waiting.
settings-crash-off-none = Imezimwa. Hakukuwa na ripoti zilizosubiri.

# Said beside the Ask switch when it is turned off: how many waiting reports were deleted.
# SAFETY: needs a human review before it ships
settings-crash-off-deleted =
    { $n ->
        [one] Imezimwa. Ripoti { $n } iliyosubiri imefutwa.
       *[other] Imezimwa. Ripoti { $n } zilizosubiri zimefutwa.
    }

# Tip of the button that opens the community page.
settings-crash-about-community = Kuhusu jumuiya

# Heading of the line with the community button.
settings-crash-go-to-bm = Ripoti huenda Buildmorphic

# Under the Reports go to Buildmorphic heading.
settings-crash-community = Jumuiya inayosaidia kujenga Trindle.

# Placeholder of the box for the Buildmorphic username.
settings-crash-username = jina la mtumiaji

# Heading of the line with the username box.
settings-crash-your-username = Jina lako la mtumiaji la Buildmorphic

# Under the username heading.
settings-crash-username-sub = Si lazima. Kamwe si anwani ya barua pepe.

# Heading of a line about joining with an email address.
settings-crash-join = Jiunge kwa barua pepe tu

# Under the Join heading.
settings-crash-join-sub = Pata sifa na viwango kwa matatizo unayoripoti kwanza.

# Heading of a line saying a report holds only a fixed list.
settings-crash-fixed = Ripoti ina orodha maalum tu

# Under the fixed list heading.
settings-crash-fixed-sub = Unaiona kwanza, na hakuna kinachotumwa bila wewe kubonyeza.

# Heading of the card for reports the person sends.
settings-crash-reports-you-send = Ripoti unazotuma

# Help text of the Reports you send card (the information button).
settings-crash-reports-about = Programu moja ya Trindle ikiharibika, dirisha linaweza kufunguka likiuliza kama utume ripoti. Linaonyesha maandishi kamili kwanza: maneno yako, modeli ya kompyuta na michoro, kosa na kumbukumbu za kabla yake. Faili, nenosiri na majina ya Wi-Fi hazisomwi kamwe, na majina na anwani hufutwa. Unaweza pia kufungua dirisha hilohilo kutoka kitufe cha Ripoti tatizo cha pete wakati wowote.

# Spoken name of the box where the GitHub name is typed.
settings-crash-github-name = jina la github

# Heading of the line where the person can give their GitHub name.
settings-crash-github-line = Jina lako la GitHub

# Small words under that heading.
settings-crash-github-about = Si lazima. Huonyeshwa kama Imeripotiwa na.

