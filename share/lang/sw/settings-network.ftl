
# machine translated (Swahili), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } imezimwa kwa swichi au kitufe kwenye kompyuta yenyewe. Iwashe, kisha jaribu tena.

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = Mfumo unazuia { $radio }, na Trindle haikuweza kuondoa kizuizi hicho.

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = Nenosiri hilo halikukubaliwa. Lijaribu tena.

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = Nenosiri lililohifadhiwa la mtandao huu halikukubaliwa. Bonyeza kitufe cha pipa ili kusahau mtandao, kisha ujiunge nao tena.

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = Trindle haikupata mtandao huo. Angalia jina lake, na uhakikishe kuwa uko karibu nao.

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = Kompyuta hii ilijiunga na mtandao lakini haikupata anwani. Jaribu tena, au anzisha upya kipanga njia.

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = Kompyuta hii haikuweza kujiunga na mtandao. Sogea karibu na kipanga njia, kisha jaribu tena.

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = Hilo haliwezi kuwa nenosiri la Wi-Fi. Liangalie, kisha jaribu tena.

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = Haikuweza kujiunga na mtandao.

# Small line of a shared folder: who may open it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-open = fungua kwa nenosiri lako

# Small line of a shared folder: who may open and change it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-change = fungua na ubadilishe kwa nenosiri lako

# Small line of a shared folder: anyone on the local network may open it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-open = yeyote kwenye mtandao wa ndani anaweza kufungua

# Small line of a shared folder: anyone on the local network may change it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-change = yeyote kwenye mtandao wa ndani anaweza kubadilisha

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
settings-network-state-configuring = Inaandaliwa

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
        [strong] Imeunganishwa na { $name }. Mawimbi ni yenye nguvu.
        [good] Imeunganishwa na { $name }. Mawimbi ni mazuri.
        [fair] Imeunganishwa na { $name }. Mawimbi ni ya wastani.
       *[other] Imeunganishwa na { $name }. Mawimbi ni hafifu.
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
settings-network-top-gone = Trindle haiwezi kuonyesha mitandao kwa sababu NetworkManager haiendeshwi.

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = Basi ya mfumo

# Name of the page in the list of settings pages and its heading.
settings-network-title = Mtandao

# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi pasiwaya intaneti ethernet kebo nenosiri unganisha firewall kushiriki printa folda wireless internet wired password connect sharing share printer folder

# Help text at the top of the page.
settings-network-help = Washa au zima Wi-Fi, jiunge na mtandao, sahau mtandao usioutaka tena, au ruhusu kompyuta nyingine ziunganishwe.

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] Mawimbi yenye nguvu kutoka { $name }.
        [good] Mawimbi mazuri kutoka { $name }.
        [fair] Mawimbi ya wastani kutoka { $name }.
       *[other] Mawimbi hafifu kutoka { $name }.
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = Wi-Fi inahitaji Linux mpya zaidi.

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = Kompyuta hii haina Wi-Fi inayofanya kazi.

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = Imezimwa. Iwashe ili ujiunge na mtandao.

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = Imewashwa, lakini haijajiunga na mtandao.

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Tumia Wi-Fi

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi na kebo

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi huunganisha kompyuta hii na mtandao bila kebo. Ukiizima, huacha kutafuta mitandao na kuokoa betri kidogo. Mtandao unaoficha jina lake haujitokezi kamwe kwenye orodha ya mitandao iliyo karibu. Ili kujiunga na mtandao kama huo, andika jina lake kama lilivyo, pamoja na herufi kubwa. Baadhi ya chipu za Wi-Fi zinahitaji dereva ambaye toleo la Linux linalotumiwa na Trindle halina bado. Kwa chipu hizo, unahitaji toleo jipya zaidi la Linux. Kebo ya mtandao ina mstari wake, wenye kitufe cha kuunganisha au kukata.

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = Ruhusu kompyuta nyingine ziunganishwe

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = Inatafuta folda zilizoshirikiwa…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = Folda zilizoshirikiwa hazikuweza kusomwa

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = Bado hazipatikani

# Small line under it.
settings-network-not-reachable-sub = Kompyuta nyingine haziwezi kuzifungua wakati kushiriki kumezimwa.

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = Hakuna folda zilizoshirikiwa

# Heading of the Sharing card.
settings-network-sharing = Kushiriki

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = Wakati Ruhusu kompyuta nyingine ziunganishwe imezimwa, kompyuta nyingine kwenye mtandao zinaweza kufikia hii tu ikiwa kompyuta hii iliwasiliana nazo kwanza. Hii inazuia yeyote aliye kwenye Wi-Fi ile ile asijaribu kuingia. Iwashe ili kushiriki folda au printa na kompyuta nyingine nyumbani. Ili kushiriki folda, ibofye kulia kwenye Folders na uchague Shiriki kwenye mtandao. Kila folda iliyoshirikiwa huonyesha nani anaweza kufanya nini. Kwa nenosiri lako: wengine wanaweza kufungua faili, au kufungua na kubadilisha, ikiwa wana nenosiri lako la kushiriki. Yeyote nyumbani: yeyote kwenye mtandao anaweza kufungua faili, au kufungua na kubadilisha, bila nenosiri. Swichi ikiwa imezimwa, kompyuta nyingine nyumbani haziwezi kufungua yoyote kati yake.

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = Inasimamisha…

# Shown beside the sharing switch while the password is asked for.
# SAFETY: needs a human review before it ships
settings-network-waiting = Inasubiri nenosiri lako…

# Shown beside the sharing switch after other computers were allowed in.
# SAFETY: needs a human review before it ships
settings-network-sharing-allowed = Kushiriki kumewashwa

# Shown beside the sharing switch after other computers were kept out.
# SAFETY: needs a human review before it ships
settings-network-kept-out = Zimezuiwa

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
settings-network-connecting = Inaunganisha…

# Line in the Networks in range card when there are none.
settings-network-none-in-range = Hakuna mitandao iliyo karibu

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = Mitandao iliyo karibu

# Help text (i button) of the Networks in range card.
# SAFETY: needs a human review before it ships
settings-network-in-range-about = Kila mtandao wa Wi-Fi ambao kompyuta hii inaweza kuupata. Unapofungua ukurasa, mtandao unaoutumia huja kwanza, kisha mitandao iliyohifadhiwa, kisha mingine kwa nguvu ya mawimbi. Mpangilio haubadiliki ukurasa ukiwa wazi. Bonyeza mshale ili kujiunga na mtandao. Mtandao uliolindwa huomba nenosiri lake mara ya kwanza. Kitufe cha pipa husahau mtandao uliohifadhiwa na nenosiri lake. Mtandao wazi haujasimbwa, kwa hiyo walio karibu wanaweza kuona unachotuma kupitia huo.

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
settings-network-joining = Inajiunga…

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = Manenosiri ya Wi-Fi yana angalau herufi 8.

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = Jiunge na mtandao uliofichwa

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = Jina la mtandao

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = Nenosiri, ikiwa lipo

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = Andika jina la mtandao.

# Shown under a network after it was joined.
settings-network-joined = Imejiunga

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } Mtandao ulihifadhiwa hata hivyo. Usahau kwa kitufe cha pipa kabla ya kujiunga tena.

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = Haikuweza kukatwa.


# Heading of the card that lists the ports opened by hand.
settings-network-ports = Milango iliyofunguliwa

# Help text (i button) of the Open ports card.
settings-network-ports-about = Kila mlango umefungwa isipokuwa kompyuta hii iliwasiliana kwanza na ile nyingine. Programu inayosubiri wengine waunganishe, kama seva ya mchezo au zana ya kusawazisha, haiwezi kufikiwa hadi ufungue mlango wake hapa. Mlango uliofunguliwa huruhusu kompyuta yoyote kwenye mtandao wa ndani ule ule wa kompyuta hii, lakini kamwe si intaneti pana. Kwenye mtandao wa mkahawa, hoteli au ofisi, hiyo inajumuisha kila mtu aliye pale. Ili kufungua au kufunga mlango, unahitaji nenosiri la msimamizi.

# Line in the Open ports card when no port is open.
settings-network-ports-none = Hakuna milango iliyofunguliwa

# Tooltip of the button that closes an open port. { $port } is like TCP 37666.
settings-network-port-close = Funga { $port }

# Heading of the line that opens a port.
settings-network-port-add = Fungua mlango

# Placeholder in the box where the port number is typed.
settings-network-port-number = Nambari ya mlango

# Tooltip of the choice of TCP, UDP or both.
settings-network-port-protocol = Itifaki

# Choice: the TCP protocol only. Usually left as TCP.
settings-network-port-tcp = TCP

# Choice: the UDP protocol only. Usually left as UDP.
settings-network-port-udp = UDP

# Choice: both TCP and UDP.
settings-network-port-both = Zote mbili

# Button that opens the typed port.
settings-network-port-open = Fungua

# Red words when what was typed is not a port number.
settings-network-port-not-a-port = Andika nambari kutoka 1 hadi 65535.

# Red words when the port is already open.
settings-network-port-already = Tayari umefunguliwa.

# Said under the line once the port is open.
settings-network-port-opened = Umefunguliwa

settings-network-username = Inahitaji jina la mtumiaji

settings-network-username-unsupported = Mtandao huu unahitaji jina la mtumiaji pamoja na nenosiri. Trindle bado haiwezi kujiunga na mitandao ya aina hiyo.
