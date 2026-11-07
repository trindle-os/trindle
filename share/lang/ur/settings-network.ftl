
# machine translated (Urdu), not yet reviewed by a person.

# Said in red when a radio is switched off at a switch or key on the computer; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-hard = { $radio } کمپیوٹر کے اپنے سوئچ یا کلید سے بند ہے۔ اسے چالو کریں، پھر دوبارہ کوشش کریں۔

# Said in red when the system blocks a radio and the block could not be taken off; the radio is Wi-Fi or Bluetooth.
settings-network-rfkill-refused = { $radio } سسٹم نے بلاک کیا ہوا ہے، اور یہ بلاک ہٹایا نہیں جا سکا۔

# Said in red under a Wi-Fi network when its password was typed and not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-typed = یہ پاس ورڈ قبول نہیں ہوا۔ دوبارہ کوشش کریں۔

# Said in red under a Wi-Fi network when the password kept for it was not accepted.
# SAFETY: needs a human review before it ships
settings-network-join-password-kept = اس نیٹ ورک کے لیے محفوظ پاس ورڈ قبول نہیں ہوا۔ ردی کے ڈبے سے نیٹ ورک کو بھلا دیں، پھر دوبارہ شامل ہوں۔

# Said in red under a Wi-Fi network that could not be found.
settings-network-join-not-found = یہ نیٹ ورک نہیں ملا۔ اس کا نام اور یہ کہ یہ حد میں ہے، چیک کریں۔

# Said in red under a Wi-Fi network that let the computer in but gave it no address.
settings-network-join-no-address = نیٹ ورک نے اس کمپیوٹر کو آنے دیا مگر کوئی پتہ نہیں دیا۔ دوبارہ کوشش کریں، یا راؤٹر دوبارہ چالو کریں۔

# Said in red under a Wi-Fi network that did not let the computer join, for any other reason.
settings-network-join-failed = نیٹ ورک نے اس کمپیوٹر کو شامل نہیں ہونے دیا۔ اس کے قریب جائیں، پھر دوبارہ کوشش کریں۔

# Said in red when the password typed is one a Wi-Fi network cannot have.
# SAFETY: needs a human review before it ships
settings-network-bad-password = Wi-Fi نیٹ ورک کا ایسا پاس ورڈ نہیں ہو سکتا۔ اسے چیک کریں، پھر دوبارہ کوشش کریں۔

# Said in red when joining a network failed for no known reason.
settings-network-join-refused = نیٹ ورک میں شامل نہیں ہو سکا۔

# Small line of a shared folder: who may open it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-open = آپ کے پاس ورڈ سے کھلتا ہے

# Small line of a shared folder: who may open and change it with a password.
# SAFETY: needs a human review before it ships
settings-network-access-change = آپ کے پاس ورڈ سے کھلتا اور بدلا جا سکتا ہے

# Small line of a shared folder: anyone at home may open it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-open = گھر میں کوئی بھی کھول سکتا ہے

# Small line of a shared folder: anyone at home may change it.
# SAFETY: needs a human review before it ships
settings-network-access-guests-change = گھر میں کوئی بھی بدل سکتا ہے

# Small line of a shared folder: where it is, then who may do what with it; both are already written.
settings-network-shared-sub = { $place }: { $access }

# Tip of the round button that stops sharing a folder; the folder's name follows.
# SAFETY: needs a human review before it ships
settings-network-stop-sharing = { $name } کی شیئرنگ بند کریں

# State of a network device that NetworkManager does not manage.
settings-network-state-unmanaged = غیر زیر انتظام

# State of a network device that cannot be used.
settings-network-state-unavailable = دستیاب نہیں

# State of a network device that is not connected.
settings-network-state-disconnected = منقطع

# State of a network device that is getting ready to connect.
settings-network-state-preparing = تیار ہو رہا ہے

# State of a network device that is being set up.
settings-network-state-configuring = ترتیب دیا جا رہا ہے

# State of a network device that needs a password.
# SAFETY: needs a human review before it ships
settings-network-state-needs-password = پاس ورڈ چاہیے

# State of a network device that is getting an address.
settings-network-state-address = پتہ حاصل ہو رہا ہے

# State of a network device whose connection is being checked.
settings-network-state-checking = جانچ ہو رہی ہے

# State of a network device that is waiting.
settings-network-state-waiting = انتظار میں

# State of a network device that is connected.
settings-network-state-connected = منسلک

# State of a network device that is disconnecting.
settings-network-state-disconnecting = منقطع ہو رہا ہے

# State of a network device that failed.
settings-network-state-failed = ناکام

# Headline of the page when connected by cable and to a Wi-Fi network.
settings-network-top-both = تار سے منسلک ہے، اور Wi-Fi پر { $name } سے بھی۔

# Headline of the page when connected to a Wi-Fi network; the level is strong, good, fair or weak.
settings-network-top-wifi =
    { $level ->
        [strong] { $name } سے منسلک، مضبوط سگنل۔
        [good] { $name } سے منسلک، اچھا سگنل۔
        [fair] { $name } سے منسلک، درمیانہ سگنل۔
       *[other] { $name } سے منسلک، کمزور سگنل۔
    }

# Headline of the page when connected by cable only.
settings-network-top-cable = تار سے منسلک ہے۔

# Headline of the page when Wi-Fi is off and there is no cable.
settings-network-top-wifi-off = Wi-Fi بند ہے، اس لیے یہ کمپیوٹر منسلک نہیں ہے۔

# Headline of the page when not connected.
settings-network-top-none = منسلک نہیں۔ نیچے سے کوئی نیٹ ورک چنیں۔

# Headline of the page while the networks are looked for.
settings-network-top-looking = نیٹ ورک تلاش کیے جا رہے ہیں…

# Headline of the page when NetworkManager is not running.
settings-network-top-gone = نیٹ ورک دیکھے نہیں جا سکتے: NetworkManager نہیں چل رہا۔

# The word for the system bus in "The system bus is not available".
settings-network-the-bus = سسٹم بس

# Name of the page in the list of settings pages and its heading.
settings-network-title = نیٹ ورک

# Words the settings search looks for on this page (not shown).
settings-network-keywords = وائی فائی wifi wi-fi وائرلیس انٹرنیٹ ایتھرنیٹ تار پاس ورڈ منسلک فائر وال شیئرنگ شیئر پرنٹر فولڈر

# Help text at the top of the page.
settings-network-help = Wi-Fi چالو یا بند کریں، کسی نیٹ ورک سے جڑیں، جو نیٹ ورک اب نہیں چاہیے اسے بھلا دیں، یا دوسرے کمپیوٹرز کو جڑنے دیں۔

# Small line under the Use Wi-Fi switch when joined to a network; the level is strong, good, fair or weak.
settings-network-signal-from =
    { $level ->
        [strong] { $name } سے مضبوط سگنل۔
        [good] { $name } سے اچھا سگنل۔
        [fair] { $name } سے درمیانہ سگنل۔
       *[other] { $name } سے کمزور سگنل۔
    }

# Small line under the Use Wi-Fi switch when the Wi-Fi chip needs a newer Linux.
settings-network-chip = اس کی چپ کو نئے Linux کی ضرورت ہے۔

# Small line under the Use Wi-Fi line when the computer has no working Wi-Fi.
settings-network-no-wifi = اس کمپیوٹر میں کام کرنے والا Wi-Fi نہیں ہے۔

# Small line under the Use Wi-Fi switch when Wi-Fi is off.
settings-network-wifi-off = بند۔ نیٹ ورک سے جڑنے کے لیے اسے چالو کریں۔

# Small line under the Use Wi-Fi switch when Wi-Fi is on and no network is joined.
settings-network-wifi-idle = چالو ہے، اور کسی نیٹ ورک سے جڑا نہیں۔

# Heading of the line with the Wi-Fi switch, and the name of the switch.
settings-network-use-wifi = Wi-Fi استعمال کریں

# Heading of the Wi-Fi card when the computer has no cable.
settings-network-wifi-card = Wi-Fi

# Heading of the Wi-Fi card when the computer also has a cable.
settings-network-wifi-cable-card = Wi-Fi اور تار

# Help text (i button) of the Wi-Fi card.
settings-network-wifi-about = Wi-Fi اس کمپیوٹر کو تار کے بغیر نیٹ ورک سے جوڑتا ہے۔ بند ہو تو یہ نیٹ ورک تلاش کرنا چھوڑ دیتا ہے اور تھوڑی بیٹری بچاتا ہے۔ جو نیٹ ورک اپنا نام نشر نہیں کرتا وہ حد میں موجود نیٹ ورکس کی فہرست میں کبھی نظر نہیں آتا: اس سے یہاں جڑنے کے لیے اس کا نام بڑے چھوٹے حروف سمیت بالکل ٹھیک لکھیں۔ جب اس کمپیوٹر کی Wi-Fi چپ ایسی ہو جسے اس کے Linux کے کسی ڈرائیور نے ابھی تک نہیں سنبھالا، تو اسے اس سے نئے Linux کی ضرورت ہے جس پر Trindle چل رہا ہے۔ نیٹ ورک تار کی اپنی الگ لائن ہے، جس میں جوڑنے یا منقطع کرنے کا بٹن ہے۔

# Heading of the line with the switch that lets other computers connect, and the name of the switch.
# SAFETY: needs a human review before it ships
settings-network-share-switch = دوسرے کمپیوٹرز کو جڑنے دیں

# Small line under that switch.
settings-network-share-switch-sub = گھر میں فولڈرز اور پرنٹرز شیئر کرنے کے لیے۔

# Line in the Sharing card while the shared folders are read.
settings-network-share-looking = شیئر کیے گئے فولڈرز تلاش کیے جا رہے ہیں…

# Line in the Sharing card when the shared folders could not be read; the reason follows beneath.
settings-network-share-unreadable = شیئر کیے گئے فولڈرز پڑھے نہیں جا سکے

# Line in the Sharing card when folders are shared but the switch is off.
settings-network-not-reachable = ابھی رسائی نہیں

# Small line under it.
settings-network-not-reachable-sub = جب تک یہ بند ہے دوسرے انہیں نہیں کھول سکتے۔

# Line in the Sharing card when no folder is shared.
settings-network-no-shared = کوئی فولڈر شیئر نہیں کیا گیا

# Small line under it.
settings-network-no-shared-sub = شیئر کرنے کے لیے فولڈرز میں اس پر دایاں کلک کریں۔

# Heading of the Sharing card.
settings-network-sharing = شیئرنگ

# Help text (i button) of the Sharing card.
# SAFETY: needs a human review before it ships
settings-network-sharing-about = جب تک دوسرے کمپیوٹرز کو جڑنے دیں بند ہے، نیٹ ورک پر کوئی چیز اس کمپیوٹر تک نہیں پہنچ سکتی جب تک اس نے خود پہلے رابطہ نہ کیا ہو، اس لیے ایک ہی Wi-Fi پر کوئی قسمت آزمائی نہیں کر سکتا۔ گھر کے دوسرے کمپیوٹرز کے ساتھ فولڈر یا پرنٹر شیئر کرنے کے لیے اسے چالو کریں۔ فولڈر شیئر کرنے کے لیے اسے فولڈرز میں دایاں کلک کر کے "نیٹ ورک پر شیئر کریں" چنیں۔ ہر ایک بتاتا ہے کہ کون کیا کر سکتا ہے۔ آپ کے پاس ورڈ کے ساتھ: دوسرے آپ کے شیئرنگ پاس ورڈ سے فائلیں کھول سکتے ہیں، یا کھول اور بدل سکتے ہیں۔ گھر میں کوئی بھی: نیٹ ورک پر کوئی بھی پاس ورڈ کے بغیر فائلیں کھول سکتا ہے، یا کھول اور بدل سکتا ہے۔ سوئچ بند ہونے پر گھر کے دوسرے کمپیوٹرز ان میں سے کچھ بھی نہیں کھول سکتے۔

# Shown under a shared folder while sharing it is stopped.
settings-network-stopping = بند کیا جا رہا ہے…

# Shown beside the sharing switch while the password is asked for.
# SAFETY: needs a human review before it ships
settings-network-waiting = پاس ورڈ کا انتظار ہے...

# Shown beside the sharing switch after other computers were allowed in.
# SAFETY: needs a human review before it ships
settings-network-sharing-allowed = شیئرنگ کی اجازت ہے

# Shown beside the sharing switch after other computers were kept out.
# SAFETY: needs a human review before it ships
settings-network-kept-out = روک دیا گیا

# Said in red when Wi-Fi could not be switched on.
settings-network-on-failed = Wi-Fi چالو نہیں ہو سکا۔

# Said in red when Wi-Fi could not be switched off.
settings-network-off-failed = Wi-Fi بند نہیں ہو سکا۔

# Name of a wired network device that has no name of its own.
settings-network-ethernet = ایتھرنیٹ

# Tip of the button that disconnects a network or a cable.
# SAFETY: needs a human review before it ships
settings-network-disconnect = منقطع کریں

# Tip of the button that connects a network or a cable.
settings-network-connect = جوڑیں

# Shown under a network or cable while it is being connected.
settings-network-connecting = جڑ رہا ہے...

# Line in the Networks in range card when there are none.
settings-network-none-in-range = حد میں کوئی نیٹ ورک نہیں

# Small line under it.
settings-network-none-in-range-sub = ہر بار یہ صفحہ کھلنے پر دوبارہ تلاش کیا جاتا ہے۔

# Heading of the card that lists Wi-Fi networks in range.
settings-network-in-range = حد میں موجود نیٹ ورکس

# Help text (i button) of the Networks in range card.
# SAFETY: needs a human review before it ships
settings-network-in-range-about = ہر وہ Wi-Fi نیٹ ورک جو یہ کمپیوٹر سن سکتا ہے۔ صفحہ کھلتے وقت، استعمال میں نیٹ ورک سب سے پہلے ہے، پھر یہاں محفوظ شدہ، پھر باقی سگنل کی مضبوطی کے لحاظ سے؛ صفحہ کھلا ہونے تک ہر لائن اپنی جگہ رہتی ہے۔ کسی سے جڑنے کے لیے تیر دبائیں؛ محفوظ نیٹ ورک پہلی بار اپنا پاس ورڈ مانگتا ہے۔ ردی کا ڈبہ محفوظ شدہ نیٹ ورک اور اس کا پاس ورڈ بھلا دیتا ہے۔ کھلا نیٹ ورک مخفی (انکرپٹڈ) نہیں ہوتا، اس لیے قریب کے لوگ اس پر بھیجی گئی چیزیں دیکھ سکتے ہیں۔

# Shown on the Networks in range card while Wi-Fi is off.
settings-network-switch-on = حد میں موجود نیٹ ورکس دیکھنے کے لیے Wi-Fi چالو کریں۔

# Part of a network's small line: it is saved on this computer.
settings-network-saved = محفوظ شدہ

# Part of a network's small line: it needs a password.
settings-network-secured = محفوظ

# Part of a network's small line: it has no password and is not encrypted.
settings-network-open = کھلا، مخفی نہیں

# Part of a joined network's small line: the speed of the link in megabits a second.
settings-network-speed = { $n } Mb/s

# Tip of the bin button that forgets a saved network.
# SAFETY: needs a human review before it ships
settings-network-forget = اس نیٹ ورک کو بھلا دیں

# Said in red when a saved network could not be forgotten.
settings-network-forget-failed = نیٹ ورک بھلایا نہیں جا سکا۔

# Placeholder of the box where a network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-password = پاس ورڈ

# Tip of the button that joins the network.
settings-network-join = شامل ہوں

# Tip of the button that closes the box without joining.
# SAFETY: needs a human review before it ships
settings-network-cancel = منسوخ کریں

# Shown under a network while it is being joined.
settings-network-joining = شامل ہو رہا ہے...

# Said in red when a Wi-Fi password is too short.
# SAFETY: needs a human review before it ships
settings-network-password-short = Wi-Fi پاس ورڈ کم از کم 8 حروف کے ہوتے ہیں۔

# Heading of the line for joining a hidden network, and tip of its button.
settings-network-hidden = پوشیدہ نیٹ ورک میں شامل ہوں

# Small line under it.
settings-network-hidden-sub = اس کا نام بالکل ٹھیک لکھیں، بڑے حروف سمیت۔

# Placeholder of the box where a hidden network's name is typed.
settings-network-hidden-name = نیٹ ورک کا نام

# Placeholder of the box where a hidden network's password is typed.
# SAFETY: needs a human review before it ships
settings-network-hidden-password = پاس ورڈ، اگر ہو

# Said in red when no name was typed for a hidden network.
settings-network-name-needed = نیٹ ورک کا نام ضروری ہے۔

# Shown under a network after it was joined.
settings-network-joined = شامل ہو گیا

# Said in red when a join failed and the settings made for it could not be forgotten; the first sentence says why it failed.
settings-network-kept-anyway = { $words } پھر بھی یہ محفوظ ہو گیا: دوبارہ شامل ہونے سے پہلے ردی کے ڈبے سے اسے بھلا دیں۔

# Said in red when a network would not disconnect.
settings-network-disconnect-failed = یہ منقطع نہیں ہوا۔
