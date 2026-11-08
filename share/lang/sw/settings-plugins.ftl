# machine translated (Swahili), not yet reviewed by a person.

# Name of the Your plugins page, in the page list and as its heading.
settings-plugins-title = Programu jalizi zako

# Words people may search for to find the Your plugins page, separated by spaces.
settings-plugins-keywords = programu jalizi viongezi viendelezi vimewekwa ondoa mipangilio

# Help line at the foot of the Your plugins page.
# SAFETY: needs a human review before it ships
settings-plugins-help = Programu jalizi huongeza kitu kwenye Trindle: aikoni kwenye pete, mipangilio hapa, arifa au kitu kinachoanza ukiingia. Kila moja hufanya kazi ikiwa imetengwa, bila kuruhusiwa chochote hadi uruhusu kwenye Ruhusa, ambayo kitufe mwishoni mwa kichwa cha ukurasa huu hufungua. Kadi ya kila programu jalizi hapa ina mipangilio yake; pipa kwenye kadi, likibonyezwa mara mbili, huondoa programu jalizi hiyo.

# Red words when the window that asks what a plugin may use could not be opened.
settings-plugins-window-failed = Dirisha linalouliza inachoweza kutumia halikuweza kufunguliwa.

# Under About it: the plugin's version number is { $v }.
settings-plugins-version = Toleo { $v }.

# Under About it when the plugin is allowed all it asks: the version is { $v } and { $settled } says what is allowed.
settings-plugins-version-settled = Toleo { $v }. { $settled }

# Help text of a plugin's card when it asks for nothing: the plugin's name is { $name }.
# SAFETY: needs a human review before it ships
settings-plugins-asks-nothing = { $name } haiombi chochote, kwa hiyo hufanya kazi ikiwa imetengwa bila kuruhusiwa chochote.

# Help text of a plugin's card: its name is { $name }, { $list } is what it asks for, { $allowed } of { $asks } are allowed.
# SAFETY: needs a human review before it ships
settings-plugins-asks-for =
    { $asks ->
        [one] { $name } inaomba { $list }; { $allowed } kati ya { $asks } imeruhusiwa. Hufanya kazi ikiwa imetengwa, na kile tu unachoruhusu.
       *[other] { $name } inaomba { $list }; { $allowed } kati ya { $asks } zimeruhusiwa. Hufanya kazi ikiwa imetengwa, na kile tu unachoruhusu.
    }

# Under the line that tells a plugin needs more allowed, beside its Allow button.
# SAFETY: needs a human review before it ships
settings-plugins-needs-more = Ruhusu huonyesha kila kitu inachoomba, pamoja na swichi.

# Heading of the line about a plugin on its card.
settings-plugins-about-it = Kuhusu programu hii

# Tip of the button that changes what a plugin may use.
# SAFETY: needs a human review before it ships
settings-plugins-change = Badilisha inachoweza kutumia

# Tip of the button that removes a plugin.
# SAFETY: needs a human review before it ships
settings-plugins-remove = Ondoa programu jalizi hii

# Red words when a plugin could not be removed.
settings-plugins-not-removed = Haikuweza kuondolewa.

# Faint words in an empty typing box whose label is too long to show.
settings-plugins-type-here = Andika hapa

# Note on a plugin's card when the plugin has no settings: its name is { $name }.
settings-plugins-no-settings = { $name } haina mipangilio yake.

# Note on a plugin's card when none of its settings show now: its name is { $name }.
settings-plugins-no-settings-now = { $name } haina mipangilio ya kuonyesha kwa sasa.

# Headline of the Your plugins page: how many plugins are installed and { $list } their names.
settings-plugins-installed =
    { $n ->
        [0] Hakuna programu jalizi iliyowekwa.
        [one] Programu jalizi moja imewekwa: { $list }.
       *[other] Programu jalizi { $n } zimewekwa: { $list }.
    }

# Heading of the card shown when no plugin is installed.
settings-plugins-group = Programu jalizi

# Heading of the line shown when no plugin is installed.
settings-plugins-none = Hakuna iliyowekwa

# Under the None installed heading.
settings-plugins-none-sub = Apps ina programu jalizi za kuongeza.

# Tip of the button at the end of the page's title that opens the Permissions page.
settings-plugins-open-permissions = Fungua Ruhusa

# Said at the top of a plugin's card while its service has not yet answered; the plugin's name is {$name}.
settings-plugins-rows-asking = Inamuuliza { $name }.

# Said at the top of a plugin's card, and in red under a row, when its service does not answer; the plugin's name is {$name}.
settings-plugins-rows-silent = { $name } haijibu. Huduma yake inaweza kuwa imesimama.

# Said beside a row after the plugin kept the change.
settings-plugins-rows-saved = Imehifadhiwa.

# Red words under a row when the plugin's service would not take the change; the plugin's name is {$name}.
settings-plugins-rows-refused = { $name } haikukubali hilo.

# Red words under a row when the change failed and the plugin gave no reason; the plugin's name is {$name}.
settings-plugins-rows-failed = { $name } haikuweza kufanya hivyo.

# Red words under a row when the change failed; the plugin's name is {$name} and the reason it gave is {$why}.
settings-plugins-rows-failed-why = { $name } haikuweza kufanya hivyo: { $why }.

