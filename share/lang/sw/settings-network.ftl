
# machine translated (Swahili), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } imezimwa kwa swichi au kitufe kwenye kompyuta yenyewe. Iwashe, kisha jaribu tena.

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = { $radio } imezuiwa na mfumo, na kizuizi hakikuweza kuondolewa.

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = Nenosiri hilo halikukubaliwa. Lijaribu tena.

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = Nenosiri lililohifadhiwa kwa mtandao huu halikukubaliwa. Sahau mtandao kwa kutumia pipa, kisha ujiunge nao tena.

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = Mtandao huo haukupatikana. Angalia jina lake, na kwamba uko karibu.

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = Mtandao uliruhusu kompyuta hii lakini haukuipa anwani. Jaribu tena, au anzisha upya kipanga njia.

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = Mtandao haukuruhusu kompyuta hii kujiunga. Sogea karibu nao, kisha jaribu tena.

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = Nenosiri hilo si la aina ambayo mtandao wa Wi-Fi unaweza kuwa nayo. Liangalie, kisha jaribu tena.

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = Mtandao haukuweza kujiunganishwa.

# Small line of a shared folder: who may open it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-open = fungua kwa nenosiri lako

# Small line of a shared folder: who may open and change it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-change = fungua na ubadilishe kwa nenosiri lako

# Small line of a shared folder: anyone at home may open it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-open = yeyote nyumbani anaweza kufungua

# Small line of a shared folder: anyone at home may change it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-change = yeyote nyumbani anaweza kubadilisha

# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }: { $access }

# Tip of the round button that stops sharing a folder; the folder's name follows.
# SAFETY: needs a human review before it ships
settings-network-stop-sharing = Acha kushiriki { $name }

# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = Haisimamiwi

# State of a network device that cannot be used.
settings-network-state-unavailable = Haipatikani

# State of a network device that is not connected.
settings-network-state-disconnected = Imekatwa

# State of a network device that is getting ready to connect.
settings-network-state-preparing = Inajiandaa

# State of a network device that is being set up.
settings-network-state-configuring = Inawekwa mipangilio

# State of a network device that needs a password.
settings-network-state-needs-password = Inahitaji nenosiri

# State of a network device that is getting an address.
settings-network-state-address = Inapata anwani

# State of a network device whose connection is being checked.
settings-network-state-checking = Inakaguliwa

# State of a network device that is waiting.
settings-network-state-waiting = Inasubiri

# State of a network device that is connected.
settings-network-state-connected = Imeunganishwa

# State of a network device that is disconnecting.
settings-network-state-disconnecting = Inakatwa

# State of a network device that failed.
settings-network-state-failed = Imeshindwa

# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = Imeunganishwa kwa kebo, na kwa { $name } kupitia Wi-Fi.

# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] Imeunganishwa na { $name }, mawimbi yenye nguvu.
        [good] Imeunganishwa na { $name }, mawimbi mazuri.
        [fair] Imeunganishwa na { $name }, mawimbi ya wastani.
       *[other] Imeunganishwa na { $name }, mawimbi hafifu.
    }

# Headline of the page when connected by cable only.
settings-network-top-cable = Imeunganishwa kwa kebo.

# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Wi-Fi imezimwa, kwa hiyo kompyuta hii haijaunganishwa.

# Headline of the page when not connected.
settings-network-top-none = Haijaunganishwa. Chagua mtandao hapa chini.

# Headline of the page while the networks are looked for.
settings-network-top-looking = Inatafuta mitandao…

# Headline of the page when NetworkManager is not running.
settings-network-top-gone = Mitandao haiwezi kutazamwa: NetworkManager haiendeshwi.

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = Basi ya mfumo

# Name of the page in the list of settings pages and its heading.
settings-network-title = Mtandao

# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi pasiwaya intaneti ethernet kebo nenosiri unganisha firewall kushiriki printa folda wireless internet wired password connect sharing share printer folder

# Help text at the top of the page.
settings-network-help = Washa au zima Wi-Fi, jiunge na mtandao, sahau ule usiouhitaji tena, au ruhusu kompyuta nyingine ziunganishwe.

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] Mawimbi yenye nguvu kutoka { $name }.
        [good] Mawimbi mazuri kutoka { $name }.
        [fair] Mawimbi ya wastani kutoka { $name }.
       *[other] Mawimbi hafifu kutoka { $name }.
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = Chipu yake inahitaji Linux mpya zaidi.

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = Kompyuta hii haina Wi-Fi inayofanya kazi.

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = Imezimwa. Iwashe ili ujiunge na mtandao.

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = Imewashwa, na haijajiunga na mtandao.

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Tumia Wi-Fi

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi na kebo

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi huunganisha kompyuta hii na mtandao bila kebo. Ikizimwa, huacha kutafuta mitandao na kuokoa betri kidogo. Mtandao usiotangaza jina lake haujitokezi kwenye orodha ya mitandao iliyo karibu: jiunge nao hapa kwa kuandika jina lake kama lilivyo, pamoja na herufi kubwa. Chipu ya Wi-Fi ya kompyuta hii ikiwa ni ile ambayo bado hakuna dereva wa Linux yake aliyeikubali, inahitaji Linux mpya zaidi kuliko ile ambayo Trindle inaendesha. Kebo ya mtandao ina mstari wake, wenye kitufe cha kuunganisha au kukata.

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = Ruhusu kompyuta nyingine ziunganishwe

# Small line under that switch.
settings-network-share-switch-sub = Ili kushiriki folda na printa nyumbani.

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Inatafuta folda zilizoshirikiwa…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = Folda zilizoshirikiwa hazikuweza kusomwa

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = Bado hazipatikani

# Small line under it.
settings-network-not-reachable-sub = Wengine hawawezi kuzifungua hizi ikiwa imezimwa.

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = Hakuna folda zilizoshirikiwa

# Small line under it.
settings-network-no-shared-sub = Bofya kulia moja kwenye Folda ili kuishiriki.

# Heading of the Sharing card.
settings-network-sharing = Kushiriki

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = Ruhusu kompyuta nyingine ziunganishwe ikiwa imezimwa, hakuna kitu kwenye mtandao kinachoweza kufikia kompyuta hii isipokuwa kama iliomba kwanza, hivyo hakuna aliye kwenye Wi-Fi ile ile anayeweza kujaribu bahati yake. Iwashe ili kushiriki folda au printa na kompyuta nyingine nyumbani. Ili kushiriki folda, ibofye kulia kwenye Folda na uchague Shiriki kwenye mtandao. Kila moja husema nani anaweza kufanya nini. Kwa nenosiri lako: wengine wanaweza kufungua faili, au kufungua na kubadilisha, kwa nenosiri lako la kushiriki. Yeyote nyumbani: yeyote kwenye mtandao anaweza kufungua faili, au kufungua na kubadilisha, bila nenosiri. Kompyuta nyingine nyumbani haziwezi kufungua yoyote kati yake swichi ikiwa imezimwa.

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Inasimamisha…

# Shown beside the sharing switch while the password is asked for.
# SAFETY: needs a human review before it ships
settings-network-waiting = Inasubiri nenosiri...

# Shown beside the sharing switch after other computers were allowed in.
# SAFETY: needs a human review before it ships
settings-network-sharing-allowed = Kushiriki kunaruhusiwa

# Shown beside the sharing switch after other computers were kept out.
# SAFETY: needs a human review before it ships
settings-network-kept-out = Wamezuiwa

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Wi-Fi haikuweza kuwashwa.

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Wi-Fi haikuweza kuzimwa.

# Name of a wired network device that has no name of its own.
settings-network-ethernet = Ethernet

# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = Kata muunganisho

# Tip of the button that connects a network or a cable.
settings-network-connect = Unganisha

# Shown under a network or cable while it is being connected.
settings-network-connecting = Inaunganisha...

# Line in the Networks in range card when there are none.
settings-network-none-in-range = Hakuna mitandao iliyo karibu

# Small line under it.
settings-network-none-in-range-sub = Hutafutwa tena kila ukurasa huu unapofunguliwa.

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Mitandao iliyo karibu

# Help text (i button) of the Networks in range card.
# SAFETY: needs a human review before it ships
settings-network-in-range-about = Kila mtandao wa Wi-Fi ambao kompyuta hii inaweza kuusikia. Ukurasa unapofunguliwa, ule unaotumika uko kwanza, kisha ule uliohifadhiwa hapa, kisha mingine kwa nguvu ya mawimbi yake; mstari hubaki mahali pake ukurasa ukiwa wazi. Bonyeza mishale kujiunga na mmoja; mtandao uliolindwa huomba nenosiri lake mara ya kwanza. Pipa husahau mtandao uliohifadhiwa na nenosiri lake. Mtandao wazi haujasimbwa, kwa hiyo walio karibu wanaweza kuona kinachotumwa kupitia huo.

# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = Washa Wi-Fi ili kuona mitandao iliyo karibu.

# Part of a network's small line: it is saved on this computer.
settings-network-saved = Imehifadhiwa

# Part of a network's small line: it needs a password.
settings-network-secured = Imelindwa

# Part of a network's small line: it has no password and is not encrypted.
settings-network-open = Wazi, haijasimbwa

# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/s

# Tip of the bin button that forgets a saved network.
# SAFETY: needs a human review before it ships
settings-network-forget = Sahau mtandao huu

# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = Mtandao haukuweza kusahaulika.

# Placeholder of the box where a network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-password = Nenosiri

# Tip of the button that joins the network.
settings-network-join = Jiunge

# Tip of the button that closes the box without joining.
# SAFETY: needs a human review before it ships
settings-network-cancel = Ghairi

# Shown under a network while it is being joined.
settings-network-joining = Inajiunga...

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = Manenosiri ya Wi-Fi ni angalau herufi 8.

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Jiunge na mtandao uliofichwa

# Small line under it.
settings-network-hidden-sub = Andika jina lake kama lilivyo, herufi kubwa pia.

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Jina la mtandao

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = Nenosiri, ikiwa lipo

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = Jina la mtandao linahitajika.

# Shown under a network after it was joined.
settings-network-joined = Imejiunga

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } Hata hivyo ilihifadhiwa: isahau kwa pipa kabla ya kujiunga tena.

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = Haikukubali kukatwa.
