# machine translated (Swahili), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = Hilo si jina sahihi la kuingia.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = Jina hilo la kuingia ni refu mno.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = Hakuna mtumiaji anayeitwa { $name }.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } ni akaunti ya mfumo na imeachwa kama ilivyo.

# Red words in Settings when no password reached the helper that sets it.
# SAFETY: needs a human review before it ships
admin-password-none = Hakuna nenosiri lililotolewa.

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
# SAFETY: needs a human review before it ships
admin-password-short =
    { $n ->
        [one] Nenosiri linahitaji angalau herufi { $n }.
       *[other] Nenosiri linahitaji angalau herufi { $n }.
    }

# Red words in Settings when a new password has a line break in it.
# SAFETY: needs a human review before it ships
admin-password-line-break = Nenosiri haliwezi kuwa na mstari mpya.

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } ipo tayari.

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
# SAFETY: needs a human review before it ships
admin-user-only-admin = { $name } ndiye msimamizi pekee, kwa hiyo anabaki msimamizi.

# Red words in Settings when a person tries to delete the account they are signed in to.
# SAFETY: needs a human review before it ships
admin-delete-self = Huwezi kufuta akaunti yako mwenyewe.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-delete-picture-kept = { $name } amefutwa, lakini picha yake bado imehifadhiwa kwa skrini ya kuingia: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = Picha hiyo ni kubwa mno.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = Hiyo si picha iliyotengenezwa na Trindle.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = Folda ya picha haikuweza kufunguliwa kwa skrini ya kuingia: { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = Hilo si jina la lugha.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Orodha ya lugha ya Debian haikuweza kusomwa: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = { $code } si lugha inayojulikana na Debian.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = Jina la kompyuta ni herufi, namba na vistari, hadi 63, lisianze wala kuishia na kistari.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = Kikomo kimewashwa kwa sasa, lakini kitapotea kompyuta ikizimwa: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = Kuchaji ni kamili kwa sasa, lakini kikomo kitarudi wakati wa kuanza tena: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-sharing-still-on = Kompyuta nyingine bado zinaweza kuunganisha, kwa sababu swichi haikuweza kuzimwa: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = Hiyo si id ya programu jalizi.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = Mtu huyo hajulikani.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = Folda ya mtu huyo haijulikani.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = Orodha ya programu jalizi zilizoruhusiwa haikuweza kuhifadhiwa: { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = Programu jalizi hiyo haikupatikana.

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = Hakuna kilichosasishwa: toleo la Trindle lililopo kwenye kompyuta hii halikuweza kujulikana, kwa hiyo sasisho la Trindle lenyewe halikuweza kuachwa.

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = Inaweka masasisho

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = Inapanga vizuri

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Inaweka Trindle

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = Hilo si jina la kifurushi.

# Red words when the package to remove is not installed.
admin-package-not-installed = Haikufanyika: haijawekwa.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = Hicho si kifaa kinachojulikana na fwupd.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Haikuweza kujulikana nani aliomba Tailscale.

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = Kuzuia hakukuanza kwa wakati.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = Kuzuia hakujazimwa kabisa: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = { $path } haikuweza kurudishwa: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = Hilo si jina la tovuti.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = Hakuna seva kama hiyo.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } ilisimama kabla ya kupewa kila kitu, kwa hiyo hili haliwezi kuitwa limekamilika: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } imeshindwa.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-file-not-removed = { $path } haikuweza kuondolewa: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = Hakuna kilichobadilishwa, kwa sababu { $why }: { $path }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Kurudisha vitu kama vilivyokuwa pia hakukufanikiwa: { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = Huo si muda ambao matoleo ya awali yanaweza kuhifadhiwa.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = Folda ya picha za hali haipo.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = Kuna kitu ambacho si folda mahali picha za hali zinapokwenda.

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = Haikuweza kujulikana nani aliomba.

# Red words when a step that must go through the administrator's password was started another way.
# SAFETY: needs a human review before it ships
admin-pkexec-only = Hii huendeshwa kupitia pkexec tu.

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = Hilo si jina la kompyuta.

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = Hilo si jina la folda iliyoshirikiwa.

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = Hiyo si anwani ya kompyuta.

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = Hiyo si njia ya folda iliyoshirikiwa.

# Red words in Folders when the shared folder to close is not one of the person's own.
# SAFETY: needs a human review before it ships
admin-share-not-yours = Folda zako mwenyewe zilizoshirikiwa tu ndizo zinazoweza kufungwa hapa.

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = Hakuna folda iliyoshirikiwa iliyofunguliwa hapo.

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = { $path } haiko kama Trindle ilivyoiacha, kwa hiyo hakuna kinachounganishwa hapo.

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } haiko kama Trindle ilivyoiacha.

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = Folda nyingi mno zilizoshirikiwa zimefunguliwa.

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } haijawekwa. Inakuja katika { $package } ya Debian.

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = Kuingia hakukuweza kusomwa.

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = Kilichounganishwa kwenye kompyuta hii hakikuweza kusomwa: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = Programu bado ina kitu kimefunguliwa humo. Funga hicho kwanza.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = Kompyuta hiyo haijibu. Itafungwa ukitoka.

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = Hiyo si id ya mtumiaji.

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = Kundi la sambashare halikuweza kutengenezwa.

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Inapakua Samba

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = Inaandaa kushiriki

# Red words in Settings when a sharing password is set before sharing is set up.
# SAFETY: needs a human review before it ships
admin-sharing-not-set-up = Kushiriki hakujaandaliwa bado. Shiriki folda kwanza.

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-sharing-still-shown = Kompyuta hii bado inaonyeshwa kwa wengine kwenye mtandao: { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = Hiyo si namba ya picha ya hali.

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = Imechukuliwa kwenye Mipangilio

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Haikuweza kujulikana mfumo uko kwenye diski ipi.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = Mfumo haupo kwenye subvolume yake ya Btrfs, kwa hiyo hauwezi kubadilishwa na picha ya hali.

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
# SAFETY: needs a human review before it ships
admin-key-fetch = Ufunguo wa kutia saini haukuweza kupakuliwa kutoka { $url }.

# Red words in Apps when the address given for a signing key gave back something else.
# SAFETY: needs a human review before it ships
admin-key-invalid = Kile anwani hiyo ilichorudisha si ufunguo wa kutia saini.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = Hakuna kilichobadilishwa, kwa sababu { $path } haiwezi kusomwa: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = Inapakua ufunguo wake wa kutia saini

# Second line of the progress while a source is added.
admin-step-adding-source = Inaiongeza na kusoma orodha yake

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = Chanzo hakikuweza kutumika. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = Chanzo hakikuweza kutumika, kwa hiyo kiliondolewa tena. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
admin-source-tailscale-operator = { $user } bado hawezi kuiwasha au kuizima: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = haikuweza kuanzishwa kwa { $user }: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
admin-source-claude-kvm = { $user } hakuweza kuruhusiwa kutumia mashine pepe, ambazo Cowork inahitaji: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-docker = { $user } hakuweza kuruhusiwa kuitumia bila nenosiri: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } imewekwa, lakini maandalizi yake hayakukamilika: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = Inaweka

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = Hicho si mojawapo ya vyanzo vilivyotayarishwa.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } haijatengenezwa kwa aina hii ya kompyuta ({ $arch }).

# Last line of the progress while a program is installed.
admin-step-setting-up = Inaiandaa

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = Anwani lazima ianze na https:// na isiwe na kingine.

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = Suite na components lazima ziwe maneno ya kawaida.

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = Hilo si jina la kifurushi.

# First line of the progress while a source and its program are taken away.
admin-step-removing = Inaondoa

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = Inasoma orodha tena

# Red words in Apps when the source to remove was not added by Trindle.
# SAFETY: needs a human review before it ships
admin-source-not-ours = Ni vyanzo vilivyoongezwa na Trindle tu ndivyo vinavyoondolewa hapa.

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = Chanzo bado kipo: { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
admin-snapshot-missing = Picha ya hali { $n } haipo kwenye diski.

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-not-ready = Hakuna kilichobadilishwa: picha ya hali haikuweza kutayarishwa. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-aside = Mfumo wa sasa haukuweza kuwekwa kando: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-stranded = mfumo wa sasa sasa unaitwa { $aside } na si { $sub }, na kompyuta haitaanza hadi uitwe { $sub } tena: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-snapshots-across = Picha za hali hazikuweza kuhamishwa: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-in-place = Picha ya hali haikuweza kuwekwa mahali pake: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = Orodha ya vilivyowekwa kando haikuweza kusasishwa: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
# SAFETY: needs a human review before it ships
admin-restore-not-removed = Haikuweza kuondoa: { $names }

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = Hakuna kilichohitaji hivi tena, kwa hiyo viliondolewa: { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = Masasisho yote hayakuingia. Yaliyozuiliwa: { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = { $feature } imezimwa: kiendeshi chake ({ $driver }, DKMS) bado hakiwezi kujengwa kwenye kernel mpya.

admin-driver-off = Kiendeshi ({ $driver }, DKMS) bado hakiwezi kujengwa kwenye kernel mpya, kwa hiyo kinachotoa kimezimwa.

# Said once after the drivers that are off.
admin-driver-off-rest = Kernel na masasisho mengine yaliingia.

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = kamera pepe

admin-language-in-use = { $code } ni lugha ambayo kompyuta hii huanzia nayo, kwa hiyo inabaki.



# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = Kompyuta hii haina kupunguza kasi kwa split-lock cha kuzima.

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Diski hiyo haikupatikana, au ina mfumo.

# The drive could not be closed where it was open.
admin-games-drive-busy = Diski inatumika. Funga kilicho wazi juu yake, kisha jaribu tena: { $why }

# Another software change has the read-only system open.
admin-system-busy = Mabadiliko mengine ya programu yanaendelea. Jaribu tena yakimalizika.

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = Hakuna kilichobadilishwa: mfumo haukuweza kufunguliwa: { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = Mfumo haukuweza kufungwa tena. Utafungwa wakati wa kuwasha ijayo: { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = /usr/local haikuweza kuachwa ikiweza kuandikwa: { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = Ni root pekee anayeweza kuendesha hili, na si kupitia pkexec.

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Inahitaji Btrfs.


# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = Picha ya hali iko mahali pake na kompyuta itaanza kutoka humo. Kusafisha mfumo wa zamani hakukukamilika na kutajaribiwa tena wakati wa kuanza kwa pili: { $why }

