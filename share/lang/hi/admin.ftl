
# machine translated (Hindi), not yet reviewed by a person.

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = यह मान्य लॉगिन नाम नहीं है।

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = यह लॉगिन नाम बहुत लंबा है।

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = { $name } नाम का कोई उपयोगकर्ता नहीं है।

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name } सिस्टम का खाता है, इसलिए Trindle ने उसमें कोई बदलाव नहीं किया।

# Red words in Settings when no password reached the helper that sets it.
# SAFETY: needs a human review before it ships
admin-password-none = कोई पासवर्ड नहीं दिया गया।

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
# SAFETY: needs a human review before it ships
admin-password-short =
    { $n ->
        [one] पासवर्ड में कम से कम { $n } अक्षर होना चाहिए।
       *[other] पासवर्ड में कम से कम { $n } अक्षर होने चाहिए।
    }

# Red words in Settings when a new password has a line break in it.
# SAFETY: needs a human review before it ships
admin-password-line-break = पासवर्ड में नई पंक्ति नहीं हो सकती।

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name } पहले से मौजूद है।

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
# SAFETY: needs a human review before it ships
admin-user-only-admin = { $name } अकेला प्रशासक है, इसलिए उसे प्रशासक ही रहना होगा।

# Red words in Settings when a person tries to delete the account they are signed in to.
# SAFETY: needs a human review before it ships
admin-delete-self = आप अपना खुद का खाता नहीं हटा सकते।

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-delete-picture-kept = Trindle ने { $name } का खाता हटा दिया, पर उनकी लॉगिन स्क्रीन की तस्वीर नहीं हटा सका: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = यह तस्वीर बहुत बड़ी है।

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = यह Trindle की बनाई तस्वीर नहीं है।

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = Trindle लॉगिन स्क्रीन को तस्वीरों के फ़ोल्डर तक पहुँच नहीं दे सका: { $why }

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = यह किसी भाषा का नाम नहीं है।

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Trindle Debian की भाषाओं की सूची नहीं पढ़ सका: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = Debian { $code } भाषा को नहीं पहचानता।

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = कंप्यूटर के नाम में केवल अक्षर, अंक और हाइफ़न हो सकते हैं। वह अधिकतम 63 वर्णों का हो सकता है और हाइफ़न से शुरू या खत्म नहीं हो सकता।

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = चार्जिंग सीमा अभी चालू है, पर कंप्यूटर बंद करने पर वह खो जाएगी: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = पूरी चार्जिंग अभी चालू है, पर अगली बार कंप्यूटर चालू होने पर सीमा लौट आएगी: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-sharing-still-on = दूसरे कंप्यूटर अब भी जुड़ सकते हैं, क्योंकि Trindle साझाकरण बंद नहीं कर सका: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = यह प्लगइन आईडी नहीं है।

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = ऐसा कोई व्यक्ति नहीं है।

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = उस व्यक्ति का कोई फ़ोल्डर नहीं है।

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-plugins-list-not-kept = Trindle अनुमति प्राप्त प्लगइन की सूची सहेज नहीं सका: { $why }।

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = ऐसा कोई प्लगइन नहीं है।

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = कुछ भी अपडेट नहीं हुआ। Trindle यह नहीं पता कर सका कि उसका कौन-सा संस्करण इंस्टॉल है, इसलिए वह अपना अपडेट अलग नहीं रख सका।

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = अपडेट इंस्टॉल हो रहे हैं

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = सफ़ाई हो रही है

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Trindle इंस्टॉल हो रहा है

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = यह किसी पैकेज का नाम नहीं है।

# Red words when the package to remove is not installed.
admin-package-not-installed = वह पैकेज इंस्टॉल नहीं है।

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = यह ऐसा उपकरण नहीं है जिसे fwupd जानता हो।

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Trindle नहीं जानता कि Tailscale किसे चाहिए।

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = ब्लॉकिंग समय पर शुरू नहीं हुई।

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
# SAFETY: needs a human review before it ships
admin-blocking-not-off = ब्लॉकिंग पूरी तरह बंद नहीं है: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = Trindle { $path } को वापस नहीं रख सका: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = यह किसी साइट का नाम नहीं है।

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = ऐसा कोई सर्वर नहीं है।

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool } अपनी ज़रूरत की हर चीज़ मिलने से पहले ही रुक गया, इसलिए काम पूरा नहीं हुआ: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool } विफल रहा।

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = { $path } हटाई नहीं जा सकी: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = कुछ भी नहीं बदला गया, क्योंकि Trindle { $path } को पढ़ नहीं सका: { $why }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Trindle चीज़ों को पहले जैसा भी नहीं कर सका: { $problem }

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = यह पुराने संस्करण रखने की मान्य अवधि नहीं है।

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = पुराने संस्करण नहीं मिल रहे।

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = जहाँ पुराने संस्करणों का फ़ोल्डर होना चाहिए, वहाँ एक फ़ाइल है।

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = पता नहीं किसने माँगा।

# Red words when a step that must go through the administrator's password was started another way.
admin-pkexec-only = यह केवल pkexec से चलता है।

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = यह किसी कंप्यूटर का नाम नहीं है।

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = यह किसी साझा फ़ोल्डर का नाम नहीं है।

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = यह किसी कंप्यूटर का पता नहीं है।

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = यह किसी साझा फ़ोल्डर का पाथ नहीं है।

# Red words in Folders when the shared folder to close is not one of the person's own.
# SAFETY: needs a human review before it ships
admin-share-not-yours = यहाँ केवल आपके अपने साझा फ़ोल्डर बंद किए जा सकते हैं।

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = वहाँ कोई साझा फ़ोल्डर खुला नहीं है।

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = Trindle के सेट अप करने के बाद किसी ने { $path } को बदल दिया है, इसलिए वहाँ कुछ भी जुड़ा नहीं है।

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path } वैसा नहीं है जैसा Trindle ने छोड़ा था।

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = बहुत ज़्यादा साझा फ़ोल्डर खुले हैं।

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool } इंस्टॉल नहीं है। यह Debian के { $package } पैकेज के साथ आता है।

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = लॉगिन पढ़ा नहीं जा सका।

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = Trindle यह नहीं पढ़ सका कि इस कंप्यूटर पर क्या माउंट है: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
# SAFETY: needs a human review before it ships
admin-share-busy = कोई प्रोग्राम अब भी इस फ़ोल्डर में कुछ इस्तेमाल कर रहा है। प्रोग्राम बंद करें, फिर दोबारा कोशिश करें।

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = वह कंप्यूटर जवाब नहीं दे रहा। लॉग आउट करने पर साझा फ़ोल्डर बंद हो जाएगा।

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = यह उपयोगकर्ता आईडी नहीं है।

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = sambashare समूह नहीं बन सका।

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Samba लाया जा रहा है

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = साझाकरण तैयार किया जा रहा है

# Red words in Settings when a sharing password is set before sharing is set up.
# SAFETY: needs a human review before it ships
admin-sharing-not-set-up = साझाकरण अभी तैयार नहीं है। पहले कोई फ़ोल्डर साझा करें।

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
# SAFETY: needs a human review before it ships
admin-sharing-still-shown = यह कंप्यूटर अब भी नेटवर्क पर दूसरों को दिख रहा है: { $why }

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = यह स्नैपशॉट संख्या नहीं है।

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = सेटिंग्स में लिया गया

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Trindle यह नहीं पता कर सका कि सिस्टम किस डिस्क पर है।

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = सिस्टम अपने अलग Btrfs सबवॉल्यूम में नहीं है, इसलिए Trindle उसे स्नैपशॉट से नहीं बदल सकता।

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Trindle { $url } से हस्ताक्षर कुंजी नहीं ला सका।

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = उस पते ने हस्ताक्षर कुंजी वापस नहीं भेजी।

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = कुछ भी नहीं बदला गया, क्योंकि Trindle { $path } को पढ़ नहीं सका: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = इसकी हस्ताक्षर कुंजी लाई जा रही है

# Second line of the progress while a source is added.
admin-step-adding-source = इसे जोड़ा जा रहा है और इसकी सूची पढ़ी जा रही है

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = स्रोत इस्तेमाल नहीं हो सका। { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = Trindle इस स्रोत का इस्तेमाल नहीं कर सका, इसलिए उसने इसे फिर हटा दिया। { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-tailscale-operator = { $user } अभी इसे चालू या बंद नहीं कर सकता: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = Trindle { $user } के लिए इसे शुरू नहीं कर सका: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-claude-kvm = Trindle { $user } को वर्चुअल मशीन इस्तेमाल करने की अनुमति नहीं दे सका, जो Cowork को चाहिए: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-source-docker = Trindle { $user } को बिना पासवर्ड इसे इस्तेमाल करने की अनुमति नहीं दे सका: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name } इंस्टॉल है, पर Trindle उसकी तैयारी पूरी नहीं कर सका: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = इसे इंस्टॉल किया जा रहा है

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = यह तैयार स्रोतों में से कोई नहीं है।

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name } इस तरह के कंप्यूटर ({ $arch }) के लिए नहीं बना है।

# Last line of the progress while a program is installed.
admin-step-setting-up = इसे तैयार किया जा रहा है

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = हर पता एक सादा वेब पता होना चाहिए जो https:// से शुरू हो।

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = सूट और कंपोनेंट सादे शब्द होने चाहिए।

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = यह पैकेज का नाम नहीं है।

# First line of the progress while a source and its program are taken away.
admin-step-removing = इसे हटाया जा रहा है

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = सूचियाँ फिर से पढ़ी जा रही हैं

# Red words in Apps when the source to remove was not added by Trindle.
# SAFETY: needs a human review before it ships
admin-source-not-ours = यहाँ केवल वे स्रोत हटाए जा सकते हैं जो Trindle ने जोड़े थे।

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = स्रोत अब भी मौजूद है: { $why }

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = स्नैपशॉट { $n } डिस्क पर नहीं है।

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-not-ready = कुछ भी नहीं बदला गया, क्योंकि Trindle स्नैपशॉट तैयार नहीं कर सका। { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-aside = Trindle मौजूदा सिस्टम को अलग नहीं रख सका: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-stranded = मौजूदा सिस्टम का नाम अब { $sub } के बजाय { $aside } है, और जब तक उसका नाम फिर { $sub } नहीं होता, कंप्यूटर शुरू नहीं होगा: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-snapshots-across = Trindle स्नैपशॉट दूसरी ओर नहीं ले जा सका: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
# SAFETY: needs a human review before it ships
admin-restore-in-place = Trindle स्नैपशॉट को उसकी जगह पर नहीं रख सका: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = Trindle अलग रखे गए सिस्टमों की सूची अपडेट नहीं कर सका: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
# SAFETY: needs a human review before it ships
admin-restore-not-removed = { $names } हटाए नहीं जा सके

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
# SAFETY: needs a human review before it ships
admin-tidied-away = Trindle ने इन्हें हटा दिया, क्योंकि अब इनकी किसी को ज़रूरत नहीं थी: { $list }।

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = सभी अपडेट इंस्टॉल नहीं हुए। ये रोके गए: { $list }।

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off.
admin-driver-off-known = { $feature } बंद है, क्योंकि उसका ड्राइवर ({ $driver }, DKMS) अभी नए Linux कर्नेल के लिए नहीं बन पा रहा।

admin-driver-off = ड्राइवर ({ $driver }, DKMS) अभी नए Linux कर्नेल के लिए नहीं बन पा रहा। जिन सुविधाओं को इसकी ज़रूरत है, वे बंद हैं।

# Said once after the drivers that are off.
admin-driver-off-rest = कर्नेल और बाकी अपडेट इंस्टॉल हो गए।

# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = वर्चुअल कैमरा

admin-language-in-use = { $code } वह भाषा है जिसमें यह कंप्यूटर शुरू होता है, इसलिए इसे हटाया नहीं जा सकता।

# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = इस कंप्यूटर में बंद करने के लिए कोई स्प्लिट-लॉक धीमापन नहीं है।

# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Trindle को वह ड्राइव नहीं मिली, या वह सिस्टम ड्राइव है।

# The drive could not be closed where it was open.
admin-games-drive-busy = ड्राइव उपयोग में है। उस पर खुली फ़ाइलें या प्रोग्राम बंद करें, फिर दोबारा कोशिश करें: { $why }

# Another software change has the read-only system open.
admin-system-busy = सॉफ़्टवेयर में एक और बदलाव चल रहा है। उसके पूरा होने पर दोबारा कोशिश करें।

# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = कुछ भी नहीं बदला गया, क्योंकि Trindle सिस्टम को अनलॉक नहीं कर सका: { $why }

# The read-only system could not be locked again after a change.
admin-system-left-open = Trindle सिस्टम को दोबारा लॉक नहीं कर सका। अगली बार चालू होने पर वह लॉक हो जाएगा: { $why }

# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = Trindle /usr/local को लिखने योग्य नहीं रख सका: { $why }

# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = इसे केवल root चला सकता है, और pkexec के ज़रिए नहीं।

# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Btrfs चाहिए।

# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = स्नैपशॉट अपनी जगह पर है, और कंप्यूटर उसी से शुरू होगा। Trindle पुराने सिस्टम की सफ़ाई पूरी नहीं कर सका। वह कंप्यूटर के अगली बार चालू होने पर फिर कोशिश करेगा: { $why }

# Red words in Settings when too many ports are open already. { $most } is a number.
admin-ports-too-many = आप ज़्यादा से ज़्यादा { $most } पोर्ट खोल सकते हैं।

# Red words in Settings when the list of open ports could not be kept. { $why } is the system's reason.
admin-ports-not-kept = Trindle खुले पोर्ट की सूची सहेज नहीं सका: { $why }।

# Red words in Settings when the firewall could not be switched off. { $why } is the system's reason.
admin-firewall-still-on = फ़ायरवॉल अब भी चालू है: { $why }

admin-plugin-ports-changed = यह प्लगइन अब उन पोर्ट से अलग पोर्ट माँग रहा है जिनकी आपने अनुमति दी थी। उन्हें खोलने के लिए इसे फिर से अनुमति दें।

admin-source-not-you = प्रोग्राम केवल उसी व्यक्ति के लिए जोड़ा जा सकता है जो उसे माँगता है।

# Red words when a snapshot is put back and the name for the system set aside is already used.
admin-restore-name-taken = एक और रिस्टोर ने अभी-अभी यही नाम इस्तेमाल किया। कुछ सेकंड रुकें, फिर दोबारा कोशिश करें।

# Said when a shared folder can't be disconnected. { $path } is the folder, { $error } the system's reason.
admin-share-not-closed = { $path } को अलग नहीं किया जा सका: { $error }

# Said when shared folders don't answer while they are being disconnected. { $n } is how many.
admin-shares-left =
    { $n ->
        [one] 1 साझा फ़ोल्डर ने जवाब नहीं दिया, इसलिए वह अब भी जुड़ा है। कंप्यूटर बंद होने पर वह अलग हो जाएगा।
       *[other] { $n } साझा फ़ोल्डरों ने जवाब नहीं दिया, इसलिए वे अब भी जुड़े हैं। कंप्यूटर बंद होने पर वे अलग हो जाएँगे।
    }

# Red words in Drives when a swap file could not be switched off before making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-off = स्वैप फ़ाइल { $path } बंद नहीं हो सकी: { $why }

# Red words in Drives when a swap file could not be switched back on after making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-on = स्वैप फ़ाइल { $path } दोबारा चालू नहीं हो सकी: { $why }

# Red words when Drives asks about a swap file that the system's own settings don't name.
admin-swap-not-listed = { $path } सिस्टम की स्वैप फ़ाइलों में नहीं है।

# Red words when a swap file, or a folder above it, could be changed or read by someone other than the administrator. { $why } is that file or folder.
admin-swap-not-safe = Trindle स्वैप फ़ाइल { $path } इस्तेमाल नहीं करेगा, क्योंकि दूसरे लोग { $why } को बदल या पढ़ सकते हैं।

# Red words in Drives when, after making room, Trindle couldn't find where the swap file now starts, which waking from hibernation needs.
admin-swap-resume-unread = स्वैप फ़ाइल अब कहाँ से शुरू होती है, यह नहीं मिला, इसलिए हाइबरनेशन से जागना शायद काम न करे।

# Said when the swap file moved and the running system couldn't be told: hibernation is refused until the computer restarts.
admin-swap-resume-restart = रीस्टार्ट करने के बाद हाइबरनेट फिर से काम करेगा।

# Red words in Settings, Hardware, when Use the chip again is pressed while it is already running.
admin-chip-busy = चिप पहले से तैयार हो रही है। इसके पूरा होने तक रुकें।

# Red words in Settings, Hardware, when the disk password typed for Use the chip again is wrong.
admin-chip-wrong-password = यह डिस्क का पासवर्ड नहीं है।

# Red words in Settings, Hardware, after several wrong disk passwords in a row.
admin-chip-wait = बहुत सारे गलत पासवर्ड। एक मिनट रुकें, फिर कोशिश करें।

# Red words in Settings, Hardware, when the chip unlocked the disk at the last start after all.
admin-chip-not-needed = पिछली बार चिप ने डिस्क खोली थी, इसलिए कुछ करना नहीं है।

# Red words in Settings, Hardware, when the disk is not the one the start-up screen noted.
admin-chip-changed = कंप्यूटर शुरू होने के बाद डिस्क बदल गई है। फिर चालू करें, फिर कोशिश करें।

# Red words in Settings, Hardware, when the computer's TPM security chip can't be found.
admin-chip-none = सुरक्षा चिप नहीं मिली। उसे BIOS में चालू करें, फिर कोशिश करें।

# Red words in Settings, Hardware, when clevis, the tool that sets up the chip, is missing.
admin-chip-no-clevis = चिप तैयार करने वाला टूल clevis इंस्टॉल नहीं है।

# Red words in Settings, Hardware, when the disk has no room left for another key.
admin-chip-no-slot = डिस्क में एक और कुंजी के लिए जगह नहीं है।

# Red words in Settings, Hardware, when the chip couldn't be set up. The disk still opens with its password.
admin-chip-not-sealed = चिप तैयार नहीं हो सकी। डिस्क अब भी पासवर्ड से खुलती है।

# Red words in Settings, Hardware, when the chip works again but its old key couldn't be removed.
admin-chip-old-kept = चिप फिर काम करती है, पर उसकी पुरानी कुंजी हटाई नहीं जा सकी।
