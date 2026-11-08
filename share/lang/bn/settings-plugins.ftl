
# machine translated (Bengali), not yet reviewed by a person.

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = আপনার প্লাগইন

# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = প্লাগইন অ্যাড-অন এক্সটেনশন ইনস্টল করা সরান সেটিংস plugins add-ons extensions installed remove settings

# Help line at the foot of the Your plugins page.
# SAFETY: needs a human review before it ships
settings-plugins-help = প্লাগইন Trindle-এ কিছু যোগ করে: রিংয়ে একটি আইকন, এখানে সেটিংস, বিজ্ঞপ্তি, বা লগইন থেকে চলা কিছু। প্রতিটি দেয়াল-ঘেরা অবস্থায় চলে, এবং Permissions-এ আপনি অনুমতি না দেওয়া পর্যন্ত কিছুরই অনুমতি পায় না; এই পাতার শিরোনামের শেষের বোতামটি তা খোলে। এখানে প্রতিটি প্লাগইনের কার্ডে তার নিজের সেটিংস থাকে; কার্ডের বিনে দুইবার চাপলে সেই প্লাগইন সরে যায়।

# Red words when the window that asks what a plugin may use could not be opened.
settings-plugins-window-failed = প্লাগইনটি কী ব্যবহার করতে পারে তা জিজ্ঞেস করার জানালা খোলা যায়নি।

# Under About it: the plugin's version number is { $v }.
settings-plugins-version = সংস্করণ { $v }।

# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = সংস্করণ { $v }। { $settled }

# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
settings-plugins-asks-nothing = { $name } কিছুই চায় না, তাই এটি কোনো অনুমতি ছাড়াই দেয়াল-ঘেরা অবস্থায় চলে।

# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
# SAFETY: needs a human review before it ships
settings-plugins-asks-for =
    { $asks ->
        [one] { $name } চায় { $list }; { $asks }টির মধ্যে { $allowed }টি অনুমোদিত। এটি দেয়াল-ঘেরা অবস্থায় চলে, শুধু আপনার দেওয়া অনুমতি নিয়ে।
       *[other] { $name } চায় { $list }; { $asks }টির মধ্যে { $allowed }টি অনুমোদিত। এটি দেয়াল-ঘেরা অবস্থায় চলে, শুধু আপনার দেওয়া অনুমতি নিয়ে।
    }

# Under the line that tells a plugin needs more allowed, beside its Allow button.
# SAFETY: needs a human review before it ships
settings-plugins-needs-more = অনুমতি দিন বোতামে চাপলে এটি যা যা চায় তার প্রতিটি একটি সুইচসহ দেখায়।

# Heading of the line about a plugin on its card.
settings-plugins-about-it = এটি সম্পর্কে

# Tip of the button that changes what a plugin may use.
settings-plugins-change = এটি কী ব্যবহার করতে পারে তা বদলান

# Tip of the button that removes a plugin.
# SAFETY: needs a human review before it ships
settings-plugins-remove = এই প্লাগইন সরান

# Red words when a plugin could not be removed.
settings-plugins-not-removed = এটি সরানো যায়নি।

# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = এখানে লিখুন

# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = { $name }-এর নিজস্ব কোনো সেটিংস নেই।

# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = বর্তমান অবস্থায় { $name }-এর দেখানোর মতো কোনো সেটিংস নেই।

# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] কোনো প্লাগইন ইনস্টল করা নেই।
        [one] একটি প্লাগইন ইনস্টল করা আছে: { $list }।
       *[other] { $n }টি প্লাগইন ইনস্টল করা আছে: { $list }।
    }

# Heading of the card shown when no plugin is installed.
settings-plugins-group = প্লাগইন

# Heading of the line shown when no plugin is installed.
settings-plugins-none = কিছু ইনস্টল করা নেই

# Under the None installed heading.
settings-plugins-none-sub = Apps-এ যোগ করার মতো প্লাগইন আছে।

# Tip of the button at the end of the page's title that opens the Permissions page.
# SAFETY: needs a human review before it ships
settings-plugins-open-permissions = Permissions খুলুন

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = { $name }-কে জিজ্ঞাসা করা হচ্ছে।

# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name } উত্তর দিচ্ছে না। এর সেবা হয়তো বন্ধ আছে।

# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = সংরক্ষিত।

# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = { $name } এটা গ্রহণ করেনি।

# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = { $name } এটা করতে পারেনি।

# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = { $name } এটা করতে পারেনি: { $why }।

