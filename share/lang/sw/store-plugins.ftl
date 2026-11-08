
# machine translated (Swahili), not yet reviewed by a person.

# Tip of the round button that opens a plugin's settings.
store-plugins-open-settings = Fungua mipangilio yake

# Red words under the plugin catalogue when starting again did not work; the reason follows.
store-plugins-not-done = Haikufanyika: { $why }

# Line of the built-in Pocketkey row.

# Said on a plugin's row when its update asks for a new permission and so waits for the person.
# SAFETY: needs a human review before it ships
store-plugins-waits-for-you = Sasisho hili linahitaji ruhusa mpya. Linakusubiri.

# On a plugin's row: the version it has and the one that is ready.
store-plugins-waiting = Toleo { $mine } kwa sasa, { $new } liko tayari

# On a plugin's row: the same, and what the new one asks for that the old one did not; the list and the note that it waits follow.
# SAFETY: needs a human review before it ships
store-plugins-waiting-more = Toleo { $mine } kwa sasa, { $new } liko tayari. Linaomba zaidi ya hapo awali: { $more }. { $waits }

# Said after who is putting updates in, on the Updates and Plugins pages.
store-plugins-come-back = Vitufe vilivyo hapa vitarudi litakapomaliza.

# Line under the title of the Plugins page while the lists are read.
store-plugins-looking = Inatafuta programu jalizi…

# Line under the title of the Plugins page when there are no plugins at all.
store-plugins-none = Hakuna programu jalizi zilizosakinishwa, na hakuna zinazotolewa bado.

# Line under the title of the Plugins page when none are installed and some are on offer.
store-plugins-none-ready =
    { $n ->
        [one] Hakuna programu jalizi zilizosakinishwa bado; 1 iko tayari kusakinishwa.
       *[other] Hakuna programu jalizi zilizosakinishwa bado; { $n } ziko tayari kusakinishwa.
    }

# How many plugins are installed, as part of a line.
store-plugins-count-installed =
    { $n ->
        [one] programu jalizi 1 imesakinishwa
       *[other] programu jalizi { $n } zimesakinishwa
    }

# How many plugins have an update waiting, as part of a line.
store-plugins-count-have-update =
    { $n ->
        [one] 1 ina sasisho
       *[other] { $n } zina sasisho
    }

# How many plugins have an update held back, as part of a line.
store-plugins-count-held =
    { $n ->
        [one] sasisho 1 limezuiliwa
       *[other] masasisho { $n } yamezuiliwa
    }

# Line under the title of the Plugins page: how many are installed, with some updates held back.
store-plugins-line-held = { $installed }; { $held }.

# Line under the title of the Plugins page: how many are installed.
store-plugins-line-installed = { $installed }.

# Line under the title of the Plugins page: all are up to date.
store-plugins-line-current = { $installed }, zote zimesasishwa.

# Line under the title of the Plugins page: some have updates waiting.
store-plugins-line-updates = { $installed }, na { $have }.

# The same, with some updates held back.
store-plugins-line-updates-held = { $installed }, na { $have }; { $held }.

# On a plugin's card when it asks for nothing.
store-plugins-asks-nothing = Haiombi chochote.

# On a plugin's card: what it asks for.
# SAFETY: needs a human review before it ships
store-plugins-asks = Inachoomba: { $list }.

# Title of the Plugins page.
store-plugins-title = Programu jalizi

# On a plugin's card: the new version is ready and held back is not said.
store-plugins-version-ready = Toleo { $mine } limesakinishwa; { $new } liko tayari.

# On a plugin's card: its installed version.
store-plugins-version-installed = Toleo { $version }, limesakinishwa.

# On a plugin's card: the version on offer.
store-plugins-version-offered = Toleo { $version }.

# On a plugin's card: the version held back, with the end of the sentence.
store-plugins-version-held = { $words }.

# Tip of the round button that installs a plugin.
store-plugins-install = Sakinisha { $name }

# Tip of the round button that releases a plugin whose updates are held back.
store-plugins-release = Achilia { $name }

# Tip of the round button that updates a plugin to a version.
store-plugins-update-to = Sasisha hadi toleo { $version }

# Heading of the card that says where plugins come from.
store-plugins-where = Programu jalizi zinatoka wapi

# Information on the card that says where plugins come from.
store-plugins-come-from = Programu jalizi hutoka kwenye orodha ya Trindle yenyewe tu, inayosomwa kutoka trindle.org kila ukurasa huu unapofunguliwa. Ili kutoa moja, iwasilishe: kila moja hujaribiwa kabla ya kuanza kutumika.

# Red words under a plugin that went in but whose background part is not running; the reason follows.
store-plugins-not-running = Imesakinishwa, lakini sehemu yake ya usuli haifanyi kazi: { $why }

# Red words under a plugin that could not be removed; the reason follows.
store-plugins-not-removed = Haikuweza kuondolewa: { $why }
