
# machine translated (Swahili), not yet reviewed by a person.

# Name of the welcome window, and the title of its first page.
welcome-title = Karibu kwenye Trindle

# Title of the second page, about opening the ring.
welcome-page-ring = Fungua duara

# Title of the third page, where the first apps are picked.
welcome-page-apps = Chagua programu zako

# Title of the fourth page, where more apps are picked.
welcome-page-more-apps = Zaidi za kuchagua

# Title of the fifth page, about plugins.
welcome-page-plugins = Programu jalizi huongeza zaidi

# Title of the sixth page, about crash reports.
welcome-page-reports = Saidia kurekebisha Trindle

# Title of the page that shows the coloured dots for what is in use.
welcome-page-dots = Vitone

# Title of the last page, with a few things worth knowing.
welcome-page-know = Vizuri kujua

# Sentence under the title of the first page.
welcome-sentence-hello = Kila unachohitaji kiko katika duara moja, duara.

# Sentence under the title of the second page.
welcome-sentence-ring = Bofya mara mbili ukingoni kabisa mwa skrini, au bonyeza kitufe cha Super.

# Sentence under the title of the third page.
welcome-sentence-apps = Karibu hakuna kitu kilichosakinishwa bado, kwa hiyo chagua programu unazotaka.

# Sentence under the title of the fourth page.
welcome-sentence-more-apps = Chagua zaidi, kisha sakinisha zote ulizochagua kwa kubofya mara moja.

# Sentence under the title of the fifth page.
# SAFETY: needs a human review before it ships
welcome-sentence-plugins = Programu jalizi ni nyongeza ndogo kutoka Programu, na kila moja hufanya tu unachoruhusu.

# Sentence under the title of the sixth page.
welcome-sentence-reports = Tuma ripoti kitu kinapoharibika, ukichagua hivyo.

# Sentence under the title of the dots page.
welcome-sentence-dots = Kitone kinamaanisha programu inatumia kitu chako.

# Sentence under the title of the last page.
welcome-sentence-know = Mambo machache yanayofanya Trindle kuwa tofauti.

# Top right of every page: where the person is in the tour.
welcome-step = Hatua { $n } kati ya { $total }

# Read out by the screen reader when a page opens: its title, where it is in the tour, then its sentence.
welcome-page-spoken = { $title }. Hatua { $n } kati ya { $total }. { $sentence }

# Note written in the file that marks the tour as seen (welcome-done in the settings folder).
welcome-done-file-note = Karibu imeonekana. Iko kwenye Programu zote kama Welcome; futa faili hii ili kuiona tena utakapoingia wakati ujao.

# Name of the round button at the foot that closes the tour.
welcome-close = Funga

# Name of the round button at the foot that goes to the page before.
welcome-back = Rudi nyuma

# Name of the main button at the foot of most pages, that goes to the next page.
welcome-next = Endelea

# Name of the main button at the foot of the apps page once some are picked.
welcome-install = Sakinisha programu zilizochaguliwa

# Name of the main button at the foot of the last page, that ends the tour.
welcome-start = Anza kutumia Trindle

# Foot of the apps pages while the computer is offline.
welcome-offline = Unganisha na intaneti ili kusakinisha programu.

# Name of the round button at the foot that installs the failed apps again.
welcome-retry = Jaribu tena

# Foot while apps are going in: their names.
welcome-installing = Inasakinisha { $names }.

# Foot while apps are going in: their names, then how long is left.
welcome-installing-left = Inasakinisha { $names }, { $left }.

# Foot after apps went in: they are in the ring now.
welcome-installed =
    { $n ->
        [one] { $names } iko kwenye duara sasa.
       *[other] { $names } ziko kwenye duara sasa.
    }

# Red line at the foot when the apps could not be installed, with what Flatpak said.
welcome-failed = Programu hazikuweza kusakinishwa. { $why }

# Said when Flatpak gave no reason for the apps not going in.
welcome-flatpak-refused = Flatpak ilikataa.

# Foot of the apps pages: the apps picked so far.
welcome-chosen = Zilizochaguliwa: { $names }.

# Names of apps when there are more than three: the first two, then how many others.
welcome-named-more = { $first }, { $second } na { $more } zaidi

# Head of the card with the picture of the ring on the first page.
welcome-hello-ring = Duara

# Head of the card on the first page that lists what the ring holds.
welcome-hello-held = Kilichomo ndani yake

# Name of the first thing the ring holds.
welcome-held-programs = Programu zako

# Line under it.
welcome-held-programs-line = Ziko kuzunguka nje.

# Name of the second thing the ring holds.
welcome-held-time = Saa na tarehe

# Line under it.
welcome-held-time-line = Ziko katikati.

# Name of the third thing the ring holds.
welcome-held-switches = Swichi

# Line under it.
welcome-held-switches-line = Sauti, skrini, kuzuia kulala na nishati.

# Head of the card with the picture of the pointer opening the ring.
welcome-ring-pointer = Kwa kielekezi

# Head of the card with the picture of the Super key.
welcome-ring-keyboard = Kwa kibodi

# Note under the picture of the Super key.
welcome-ring-super-note = Kitufe cha Super mara nyingi kina nembo ya Windows.

# Head of a card of apps: web browsers.
welcome-kind-browser = Kivinjari cha wavuti

# Head of a card of apps: email programs.
welcome-kind-email = Barua pepe

# Head of a card of apps: chat programs.
welcome-kind-chat = Mazungumzo

# Head of a card of apps: office programs.
welcome-kind-office = Ofisi

# Head of a card of apps: music and video players.
welcome-kind-media = Muziki na video

# Head of a card of apps: pictures and drawing.
welcome-kind-pictures = Picha na michoro

# Head of a card of apps: notes and writing.
welcome-kind-notes = Maelezo na uandishi

# Head of a card of apps: PDF and documents.
welcome-kind-pdf = PDF na hati

# Shown in a card of apps when none of them is on Flathub.
welcome-kind-empty = Hakuna kati ya hizi kwenye Flathub kwa sasa.

# Beside an app that has been installed.
welcome-app-installed = Imesakinishwa

# Beside an app that is going in now.
welcome-app-installing = Inasakinishwa

# Name of the round button on an app's row that picks it, once picked.
welcome-app-picked = { $name }, imechaguliwa

# Name of the round button on an app's row that picks it.
welcome-app-pick = Chagua { $name }

# Head of the card with the picture of the ring holding plugins.
welcome-plugins-ring = Kwenye duara

# Head of the card that lists the plugins on offer.
welcome-plugins-card = Programu jalizi

# Beside the button that opens Apps at its Plugins page.
welcome-plugins-where = Kwenye Programu, chini ya Programu jalizi.

# Name of the button that opens Apps at its Plugins page.
welcome-plugins-open = Fungua programu jalizi kwenye Programu

# Said in the plugins card while there are none to show.
welcome-plugins-none = Programu jalizi zinazotolewa zimeorodheshwa kwenye Programu.

# Head of the card on the crash reports page.
welcome-reports-card = Ripoti za hitilafu

# The one question about crash reports.
# SAFETY: needs a human review before it ships
welcome-reports-question = Saidia kurekebisha Trindle kwa kutuma ripoti kitu kinapoharibika? Unaiona kwanza kila wakati.

# Note under the question about crash reports.
# SAFETY: needs a human review before it ships
welcome-reports-note = Hakuna kinachotumwa bila wewe kubonyeza. Unaiona ripoti nzima kwanza, na haina faili, manenosiri wala majina ya Wi-Fi.

# Second note under the question about crash reports.
welcome-reports-settings = Unaweza kubadilisha hili wakati wowote kwenye Mipangilio, Ripoti za hitilafu.

# The answer that turns crash reports off.
welcome-reports-no = Hapana

# The answer that turns crash reports on.
welcome-reports-yes = Ndiyo, niulize

# Name the screen reader gives to the choice between the two answers.
welcome-reports-choice = Tuma ripoti kitu kinapoharibika

# Head of a card on the last page.
welcome-know-boxes = Programu huishi kwenye masanduku yake

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-know-boxes-line = Programu huuliza kabla ya kufikia faili, kamera au kipaza sauti chako.

# Head of a card on the last page.
welcome-know-updates = Masasisho ya usalama hujisakinisha yenyewe

# Line in that card.
welcome-know-updates-line = Kila siku, bila kuuliza. Masasisho mengine yanakusubiri.

# Head of a card on the last page.
welcome-know-ring = Mipangilio na Programu ziko kwenye duara

# Line in that card.
welcome-know-ring-line = Katikati yake, juu ya saa.

# Head of a card on the last page.
welcome-know-help = Msaada uko kwenye Mipangilio

# Line in that card.
welcome-know-help-line = Ziara hii iko kwenye Programu zote kama Welcome, wakati wowote ukiitaka tena.

# What the screen reader says for the picture of the ring on a clear screen.
welcome-picture-hello = Skrini ndogo isiyo na kitu isipokuwa duara katikati: saa katikati yake, programu pendwa kuizunguka.

# What the screen reader says for the picture of the ring holding plugins.
welcome-picture-plugins = Duara likiwa wazi kwenye skrini ndogo, na ikoni za programu jalizi chache mfululizo katikati yake, chini ya saa, mahali ikoni ya programu jalizi hukaa ikishasakinishwa.

# What the screen reader says for the picture of the pointer opening the ring.
welcome-picture-dots = Michoro miwili midogo ya mahali nukta zinapoonekana: kwenye kichupo cha dirisha ndani ya pete, na juu katikati ya skrini.

welcome-dots-where-tab = Kwenye kichupo cha dirisha

welcome-dots-where-corner = Juu katikati ya skrini

welcome-picture-ring = Skrini ndogo. Kielekezi kinahamia ukingo wa juu kabisa na kubofya mara mbili, nalo duara hufunguka katikati.

# What the screen reader says for the picture of the Super key.
welcome-picture-super = Kitufe cha Super kikibonyezwa, ambacho hufungua duara.

# Title of the first page, which asks how much the person knows.
welcome-page-level = Unajua kiasi gani?

# Sentence under that title.
welcome-sentence-level = Chagua kilicho karibu zaidi, na ziara itaacha unachokijua.

# Name of the answer for someone who has not used Linux.
welcome-level-new = Mgeni wa Linux

# Line in that answer's card.
welcome-level-new-line = Kila ukurasa, pamoja na kile ambacho kila kompyuta ya Linux hufanya.

# Name of the answer for someone who has used Linux.
welcome-level-linux = Nimewahi kutumia Linux

# Line in that answer's card.
welcome-level-linux-line = Ni vya Trindle tu. Misingi ya Linux inarukwa.

# Name of the answer for someone who has used Trindle.
welcome-level-trindle = Nimewahi kutumia Trindle

# Line in that answer's card.
welcome-level-trindle-line = Inafunga ziara sasa na haionyeshi tena.

# Title of the page about files and folders.
welcome-page-files = Faili zako

# Sentence under that title.
welcome-sentence-files = Kila kitu chako kiko kwenye folda yako ya nyumbani.

# Title of the page about getting software.
welcome-page-software = Kupata programu

# Sentence under that title.
welcome-sentence-software = Programu hutoka kwenye Programu, si kwenye faili za usanidi unazopakua.

# Title of the page about the administrator password.
# SAFETY: needs a human review before it ships
welcome-page-admin = Nenosiri la msimamizi

# Sentence under that title.
# SAFETY: needs a human review before it ships
welcome-sentence-admin = Mabadiliko kwa kompyuta nzima huomba nenosiri kwanza.

# Title of the page about snapshots.
welcome-page-snapshots = Picha za mfumo

# Sentence under that title.
welcome-sentence-snapshots = Trindle huweka nakala za mfumo, hivyo sasisho linaweza kutenguliwa.

# Title of the page about the person's own words.
welcome-page-words = Maneno yako mwenyewe

# Sentence under that title.
welcome-sentence-words = Badilisha neno lolote Trindle inaloonyesha, au ongeza lugha yako mwenyewe.

# Head of a card on the files page.
welcome-files-home = Folda ya nyumbani

# Line in that card.
welcome-files-home-line = Nyaraka, Vipakuliwa na Picha viko ndani yake. Folda hufunguka hapo.

# Head of a card on the files page.
welcome-files-drives = Hakuna herufi za diski

# Line in that card.
welcome-files-drives-line = Kijiti cha USB au diski nyingine huonekana kwenye Folda unapokichomeka.

# Head of a card on the files page.
welcome-files-hidden = Majina yaliyofichwa

# Line in that card.
welcome-files-hidden-line = Jina linaloanza na nukta, kama .config, limefichwa. Linashikilia mipangilio ya programu.

# Head of a card on the files page.
welcome-files-bin = Pipa la taka

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-files-bin-line = Faili iliyofutwa hungoja kwenye pipa la taka hadi ulimwage.

# Head of a card on the software page.
welcome-software-apps = Mahali pamoja pa programu

# Line in that card.
welcome-software-apps-line = Fungua Programu kutoka kwenye duara ili kutafuta, kusakinisha na kuondoa programu.

# Head of a card on the software page.
welcome-software-exe = Hakuna faili za usanidi

# Line in that card.
welcome-software-exe-line = Faili ya usanidi ya Windows, inayoishia .exe, haiendeshwi hapa. Tafuta programu kwenye Programu.

# Head of a card on the software page.
welcome-software-updates = Masasisho mahali pamoja

# Line in that card.
welcome-software-updates-line = Programu na Mipangilio huweka kila kitu kipya, kwa hiyo hakuna programu inayohitaji kisasishaji chake.

# Head of a card on the software page.
welcome-software-remove = Kuondoa ni safi

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-software-remove-line = Kuondoa programu huiondoa yote na hakuacha chochote nyuma.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-why = Kwa nini huuliza

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-why-line = Kusakinisha kwa kila mtu au kubadilisha diski huathiri kila mtumiaji, kwa hiyo kompyuta huuliza kwanza.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-who = Nenosiri la nani

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-who-line = La msimamizi, kwa kawaida yule unayeingia naye.

# Head of a card on the administrator password page.
# SAFETY: needs a human review before it ships
welcome-admin-not = Wakati haliulizi

# Line in that card.
# SAFETY: needs a human review before it ships
welcome-admin-not-line = Programu unazosakinisha kwa ajili yako mwenyewe na faili zako hazihitaji kamwe.

# Head of a card on the snapshots page.
welcome-snap-taken = Huchukuliwa kwa ajili yako

# Line in that card.
welcome-snap-taken-line = Kwenye mfumo wa Btrfs, moja huchukuliwa kabla na baada ya kila mabadiliko ya programu.

# Head of a card on the snapshots page.
welcome-snap-list = Huorodheshwa kwenye Mipangilio

# Line in that card.
welcome-snap-list-line = Mipangilio, Picha za mfumo huziorodhesha. Kamera huchukua moja sasa.

# Head of a card on the snapshots page.
welcome-snap-boot = Hutumika ikishindwa kuanza

# Line in that card.
welcome-snap-boot-line = Chagua picha za mfumo za Trindle kwenye menyu ya kuwasha, kisha picha ya mfumo.

# Head of a card on the your own words page.
welcome-words-change = Badilisha neno

# Line in that card.
welcome-words-change-line = Tafuta neno lolote Trindle inaloonyesha na uandike lako. "Ifungue, brew" inaweza kuchukua nafasi ya "Bofya hapa kufungua faili".

# Head of a card on the your own words page.
welcome-words-share = Shiriki karatasi

# Line in that card.
welcome-words-share-line = Hamisha maneno yako kama faili, au leta ile aliyokutumia rafiki. Hukaguliwa kabla ya kutumika.

# Head of a card on the your own words page.
welcome-words-all = Kila programu kwa wakati mmoja

# Line in that card.
welcome-words-all-line = Trindle yote husoma maneno yake kutoka mahali pamoja, kwa hiyo badiliko moja huonekana kila mahali.

# Line under the cards on the your own words page, saying where it is done.
welcome-words-where = Iko kwenye Mipangilio, Eneo na lugha. Msaada unaieleza hatua kwa hatua.

# Name of the round button that opens that Settings page.
welcome-words-open = Fungua Eneo na lugha
