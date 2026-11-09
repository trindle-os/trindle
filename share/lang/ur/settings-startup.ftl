
# machine translated (Urdu), not yet reviewed by a person.

# Name of the Settings page about apps that start at login, in the list and as its heading.
settings-startup-title = لاگ ان پر شروع

# Words a person might type into the Settings search to find the start at login page; separate them with spaces.
settings-startup-keywords = autostart startup start login boot automatically launch delay شروع لاگ ان خودکار تاخیر

# Introduction under the heading of the start at login page, and the words behind the page i.
settings-startup-help = وہ ایپس جو آپ کے لاگ ان کرنے پر خود کھل جاتی ہیں۔ تاخیر سے ایپ پہلے نیٹ ورک یا کسی اور ایپ کا انتظار کر سکتی ہے۔

# Sentence at the top of the page when no app starts at login.
settings-startup-top-none = لاگ ان کرنے پر کچھ بھی خود شروع نہیں ہوتا۔

# Sentence at the top of the page when one app starts at login; { $name } is the app.
settings-startup-top-one = لاگ ان کرنے پر { $name } خود شروع ہوتی ہے۔

# Sentence at the top of the page when several apps start at login; { $n } is how many.
settings-startup-top-many =
    { $n ->
        [one] لاگ ان کرنے پر { $n } ایپ خود شروع ہوتی ہے۔
       *[other] لاگ ان کرنے پر { $n } ایپس خود شروع ہوتی ہیں۔
    }

# Second sentence at the top of the page, after a space, when apps are kept in the list but switched off; { $n } is how many.
settings-startup-top-off =
    { $n ->
        [one] ایک اور فہرست میں ہے، بند حالت میں۔
       *[other] { $n } اور فہرست میں ہیں، بند حالت میں۔
    }

# Choice in the delay list: the app starts as soon as someone logs in.
settings-startup-delay-0 = فوراً

# Choice in the delay list: the app starts a minute after login.
settings-startup-delay-60 = ایک منٹ بعد

# Choice in the delay list: the app starts two minutes after login.
settings-startup-delay-120 = 2 منٹ بعد

# Choice in the delay list; { $n } is the number of seconds to wait after login.
settings-startup-delay-seconds =
    { $n ->
        [one] { $n } سیکنڈ بعد
       *[other] { $n } سیکنڈ بعد
    }

# Name of the delay list on an app's line; { $name } is the app.
settings-startup-delay-for = { $name } کے لیے تاخیر

# Name of the switch on an app's line that keeps it from starting at login without removing it.
settings-startup-switch = لاگ ان پر شروع کریں

# Name of the bin button on an app's line that stops it starting at login.
settings-startup-stop = لاگ ان پر شروع کرنا بند کریں

# Red words under an app's line when it could not be taken off the list.
settings-startup-not-removed = اسے ہٹایا نہیں جا سکا، اس لیے یہ اب بھی لاگ ان پر شروع ہوتی ہے۔

# Red words under the add card when something could not be added.
settings-startup-not-added = اسے شامل نہیں کیا جا سکا۔

# Placeholder of the search box above the list of installed apps.
settings-startup-find = ایپ تلاش کریں

# Name of the plus button that adds an app or a command to what starts at login.
settings-startup-add-this = اسے لاگ ان پر شروع کریں

# Red words under an app in the add list when it was removed from the computer meanwhile.
settings-startup-gone = اسے شامل نہیں کیا جا سکا: ایپ اب انسٹال نہیں ہے۔

# Placeholder of the box for typing a command; the example is a command, not translated words.
settings-startup-command-example = مثلاً syncthing --no-browser

# Name of the button that opens a window to choose a program on the disk.
settings-startup-browse = ڈسک پر سے پروگرام چنیں

# Heading of the line for adding a command.
settings-startup-command = کمانڈ

# Small words under the command line.
settings-startup-command-about = جیسے ٹرمینل میں لکھی جاتی ہے، یا ڈسک پر موجود پروگرام۔

# Red words under the command line when nothing was typed.
settings-startup-command-empty = کمانڈ لکھیں، یا پروگرام چنیں۔

# Title of the window for choosing a program to start at login.
settings-startup-chooser-title = لاگ ان پر شروع کرنے کے لیے پروگرام چنیں

# Heading of the line shown in the list when nothing starts at login.
settings-startup-none-title = کچھ بھی خود شروع نہیں ہوتا

# Small words under that line.
settings-startup-none-about = اس کے ساتھ ایک ایپ شامل کریں۔

# Title of the card listing what starts at login.
settings-startup-list-title = لاگ ان پر شروع ہوتی ہیں

# Words behind the i of the card listing what starts at login.
settings-startup-about = ہر ایپ آپ کے لاگ ان کرنے کے بعد کچھ دیر انتظار کر سکتی ہے تاکہ نیٹ ورک یا کوئی اور ایپ پہلے تیار ہو جائے۔ اس کا سوئچ اسے شروع کیے بغیر یہاں رکھتا ہے؛ ردی کا بٹن اسے لاگ ان پر شروع ہونے سے بالکل روک دیتا ہے۔

# Added to the words behind the i, after a space, for an app whose command is too long for its line; { $name } is the app and { $command } its command.
settings-startup-runs = { $name } یہ چلاتی ہے: { $command }

# Name of the button that opens the list of installed apps.
settings-startup-open-apps = انسٹال شدہ ایپس میں سے چنیں

# Name of the button that opens the box for typing a command.
settings-startup-open-command = کمانڈ لکھیں، یا پروگرام چنیں

# Heading of the line for adding an installed app.
settings-startup-app-heading = انسٹال شدہ ایپ

# Small words under that line.
settings-startup-app-about = ایپس کی فہرست میں سے ایک چنیں۔

# Heading of the line for adding a command or a program.
settings-startup-cmd-heading = کمانڈ یا پروگرام

# Small words under that line.
settings-startup-cmd-about = لکھی ہوئی، یا ڈسک سے چنی ہوئی۔

# Words behind the i of the Add an app card.
settings-startup-add-about = انسٹال شدہ ایپس میں سے چنیں، یا چلانے کے لیے کمانڈ دیں جیسے آپ ٹرمینل میں لکھتے ہیں، یا ڈسک سے کوئی پروگرام چنیں۔ یہ اگلے لاگ ان پر شروع ہوتا ہے، فوراً؛ فہرست میں آنے کے بعد اس کے لیے انتظار چنا جا سکتا ہے۔

# Title of the card for adding an app.
settings-startup-add-title = ایپ شامل کریں
