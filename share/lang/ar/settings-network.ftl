
# machine translated (Arabic), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = تم إيقاف { $radio } بمفتاح أو زر على الحاسوب نفسه. شغّله ثم حاول مرة أخرى.

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = { $radio } محظور من النظام، وتعذّرت إزالة الحظر.

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = لم تُقبل كلمة المرور هذه. حاول مرة أخرى.

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = لم تُقبل كلمة المرور المحفوظة لهذه الشبكة. انسَ الشبكة بسلة المهملات، ثم انضم إليها من جديد.

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = تعذّر العثور على تلك الشبكة. تحقق من اسمها ومن أنها في النطاق.

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = سمحت الشبكة لهذا الحاسوب بالدخول لكنها لم تعطه عنوانًا. حاول مرة أخرى، أو أعد تشغيل الراوتر.

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = لم تسمح الشبكة لهذا الحاسوب بالانضمام. اقترب منها ثم حاول مرة أخرى.

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = كلمة المرور هذه لا يمكن أن تكون لشبكة Wi-Fi. تحقق منها ثم حاول مرة أخرى.

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = تعذّر الانضمام إلى الشبكة.

# Small line of a shared folder: who may open it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-open = يُفتح بكلمة مرورك

# Small line of a shared folder: who may open and change it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-change = يُفتح ويُغيَّر بكلمة مرورك

# Small line of a shared folder: anyone at home may open it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-open = يستطيع أي شخص في المنزل فتحه

# Small line of a shared folder: anyone at home may change it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-change = يستطيع أي شخص في المنزل تغييره

# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }: { $access }

# Tip of the round button that stops sharing a folder; the folder's name follows.
# SAFETY: needs a human review before it ships
settings-network-stop-sharing = أوقف مشاركة { $name }

# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = غير مُدار

# State of a network device that cannot be used.
settings-network-state-unavailable = غير متاح

# State of a network device that is not connected.
settings-network-state-disconnected = غير متصل

# State of a network device that is getting ready to connect.
settings-network-state-preparing = قيد التحضير

# State of a network device that is being set up.
settings-network-state-configuring = قيد الإعداد

# State of a network device that needs a password.
# SAFETY: needs a human review before it ships
settings-network-state-needs-password = يحتاج إلى كلمة مرور

# State of a network device that is getting an address.
settings-network-state-address = يحصل على عنوان

# State of a network device whose connection is being checked.
settings-network-state-checking = قيد الفحص

# State of a network device that is waiting.
settings-network-state-waiting = في الانتظار

# State of a network device that is connected.
settings-network-state-connected = متصل

# State of a network device that is disconnecting.
settings-network-state-disconnecting = يقطع الاتصال

# State of a network device that failed.
settings-network-state-failed = فشل

# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = متصل بالكابل، وبالشبكة { $name } عبر Wi-Fi.

# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] متصل بالشبكة { $name }، الإشارة قوية.
        [good] متصل بالشبكة { $name }، الإشارة جيدة.
        [fair] متصل بالشبكة { $name }، الإشارة متوسطة.
       *[other] متصل بالشبكة { $name }، الإشارة ضعيفة.
    }

# Headline of the page when connected by cable only.
settings-network-top-cable = متصل بالكابل.

# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Wi-Fi متوقف، لذلك هذا الحاسوب غير متصل.

# Headline of the page when not connected.
settings-network-top-none = غير متصل. اختر شبكة أدناه.

# Headline of the page while the networks are looked for.
settings-network-top-looking = جارٍ البحث عن شبكات…

# Headline of the page when NetworkManager is not running.
settings-network-top-gone = لا يمكن عرض الشبكات: NetworkManager لا يعمل.

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = ناقل النظام

# Name of the page in the list of settings pages and its heading.
settings-network-title = الشبكة

# Words the settings search looks for on this page (not shown).
settings-network-keywords = wifi wi-fi لاسلكي إنترنت ethernet كابل سلكي كلمة مرور اتصال جدار حماية مشاركة طابعة مجلد

# Help text at the top of the page.
settings-network-help = شغّل Wi-Fi أو أوقفه، وانضم إلى شبكة، وانسَ شبكة لم تعد تريدها، أو اسمح لحواسيب أخرى بالاتصال.

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] إشارة قوية من { $name }.
        [good] إشارة جيدة من { $name }.
        [fair] إشارة متوسطة من { $name }.
       *[other] إشارة ضعيفة من { $name }.
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = شريحته تحتاج إلى Linux أحدث.

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = لا يوجد Wi-Fi يعمل على هذا الحاسوب.

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = متوقف. شغّله للانضمام إلى شبكة.

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = يعمل، وغير منضم إلى شبكة.

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = استخدم Wi-Fi

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi والكابل

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = يصل Wi-Fi هذا الحاسوب بشبكة دون كابل. وعند إيقافه يتوقف عن البحث عن الشبكات ويوفّر قليلًا من البطارية. الشبكة التي لا تبث اسمها لا تظهر أبدًا في قائمة الشبكات في النطاق: انضم إليها من هنا بكتابة اسمها بالضبط، بما في ذلك الأحرف الكبيرة. وإذا كانت شريحة Wi-Fi في هذا الحاسوب مما لم يدعمه أي برنامج تشغيل في Linux الخاص به بعد، فهي تحتاج إلى Linux أحدث من الذي يعمل عليه Trindle. وللكابل الشبكي سطر خاص به، فيه زر للاتصال أو قطع الاتصال.

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = اسمح لحواسيب أخرى بالاتصال

# Small line under that switch.
settings-network-share-switch-sub = لمشاركة المجلدات والطابعات في المنزل.

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = جارٍ البحث عن المجلدات المشتركة…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = تعذّرت قراءة المجلدات المشتركة

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = لا يمكن الوصول إليها بعد

# Small line under it.
settings-network-not-reachable-sub = لا يستطيع الآخرون فتحها ما دام المفتاح متوقفًا.

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = لا توجد مجلدات مشتركة

# Small line under it.
settings-network-no-shared-sub = انقر بالزر الأيمن على مجلد في المجلدات لمشاركته.

# Heading of the Sharing card.
settings-network-sharing = المشاركة

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = ما دام خيار السماح لحواسيب أخرى بالاتصال متوقفًا، لا يستطيع أي شيء على الشبكة الوصول إلى هذا الحاسوب ما لم يطلب هو ذلك أولًا، فلا يستطيع أحد على شبكة Wi-Fi نفسها أن يجرّب حظه. شغّله لمشاركة مجلد أو طابعة مع حواسيب أخرى في المنزل. لمشاركة مجلد، انقر عليه بالزر الأيمن في المجلدات واختر المشاركة على الشبكة. ويحدد كل مجلد من يستطيع فعل ماذا. بكلمة مرورك: يستطيع الآخرون فتح الملفات، أو فتحها وتغييرها، بكلمة مرور المشاركة الخاصة بك. أي شخص في المنزل: يستطيع أي شخص على الشبكة فتح الملفات، أو فتحها وتغييرها، دون كلمة مرور. ولا تستطيع حواسيب المنزل الأخرى فتح أي منها ما دام المفتاح متوقفًا.

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = جارٍ الإيقاف…

# Shown beside the sharing switch while the password is asked for.
# SAFETY: needs a human review before it ships
settings-network-waiting = بانتظار كلمة المرور...

# Shown beside the sharing switch after other computers were allowed in.
# SAFETY: needs a human review before it ships
settings-network-sharing-allowed = المشاركة مسموحة

# Shown beside the sharing switch after other computers were kept out.
settings-network-kept-out = ممنوع الدخول

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = تعذّر تشغيل Wi-Fi.

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = تعذّر إيقاف Wi-Fi.

# Name of a wired network device that has no name of its own.
settings-network-ethernet = Ethernet

# Tip of the button that disconnects a network or a cable.
settings-network-disconnect = قطع الاتصال

# Tip of the button that connects a network or a cable.
settings-network-connect = اتصال

# Shown under a network or cable while it is being connected.
settings-network-connecting = جارٍ الاتصال...

# Line in the Networks in range card when there are none.
settings-network-none-in-range = لا توجد شبكات في النطاق

# Small line under it.
settings-network-none-in-range-sub = يُعاد البحث في كل مرة تُفتح فيها هذه الصفحة.

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = الشبكات في النطاق

# Help text (i button) of the Networks in range card.
# SAFETY: needs a human review before it ships
settings-network-in-range-about = كل شبكة Wi-Fi يستطيع هذا الحاسوب سماعها. عند فتح الصفحة تأتي أولًا الشبكة المستخدمة، ثم المحفوظة هنا، ثم الباقي حسب قوة إشارتها؛ ويبقى كل سطر في مكانه ما دامت الصفحة مفتوحة. اضغط على الأسهم للانضمام إلى شبكة؛ والشبكة المحمية تطلب كلمة مرورها أول مرة. وتنسى سلة المهملات الشبكة المحفوظة وكلمة مرورها. الشبكة المفتوحة غير مشفّرة، فقد يرى من حولك ما يُرسل عبرها.

# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = شغّل Wi-Fi لرؤية الشبكات في النطاق.

# Part of a network's small line: it is saved on this computer.
settings-network-saved = محفوظة

# Part of a network's small line: it needs a password.
# SAFETY: needs a human review before it ships
settings-network-secured = محمية

# Part of a network's small line: it has no password and is not encrypted.
# SAFETY: needs a human review before it ships
settings-network-open = مفتوحة، غير مشفّرة

# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } ميغابت/ث

# Tip of the bin button that forgets a saved network.
# SAFETY: needs a human review before it ships
settings-network-forget = انسَ هذه الشبكة

# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = تعذّر نسيان الشبكة.

# Placeholder of the box where a network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-password = كلمة المرور

# Tip of the button that joins the network.
settings-network-join = انضمام

# Tip of the button that closes the box without joining.
# SAFETY: needs a human review before it ships
settings-network-cancel = إلغاء

# Shown under a network while it is being joined.
settings-network-joining = جارٍ الانضمام...

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = كلمات مرور Wi-Fi لا تقل عن 8 أحرف.

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = انضم إلى شبكة مخفية

# Small line under it.
settings-network-hidden-sub = اكتب اسمها بالضبط، بما في ذلك الأحرف الكبيرة.

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = اسم الشبكة

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = كلمة المرور، إن وُجدت

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = اسم الشبكة مطلوب.

# Shown under a network after it was joined.
settings-network-joined = تم الانضمام

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } ومع ذلك حُفظت: انسَها بسلة المهملات قبل الانضمام من جديد.

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = رفضت قطع الاتصال.

