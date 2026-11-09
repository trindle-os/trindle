
# machine translated (Swahili), not yet reviewed by a person.

# Time since the lists were fetched, when it was under a minute ago.
settings-blocking-ago-now = sasa hivi

# Time since the lists were fetched, in minutes.
settings-blocking-ago-minutes =
    { $n ->
        [one] dakika { $n } iliyopita
       *[other] dakika { $n } zilizopita
    }

# Time since the lists were fetched, in hours.
settings-blocking-ago-hours =
    { $n ->
        [one] saa { $n } iliyopita
       *[other] saa { $n } zilizopita
    }

# Time since the lists were fetched, in days.
settings-blocking-ago-days =
    { $n ->
        [one] siku { $n } iliyopita
       *[other] siku { $n } zilizopita
    }

# Shown beside the Block adverts and bad sites switch while the person is asked for the password.
# SAFETY: needs a human review before it ships
settings-blocking-waiting = Inasubiri nenosiri...

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = Kuzuia kumewashwa

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = Kuzuia kumezimwa

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = Imehifadhiwa

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = Andika jina la tovuti, kama example.com.

# Headline of the page when blocking is off.
settings-blocking-top-off = Kuzuia kumezimwa. Utafutaji hupitia seva za mtandao wako wenyewe, kama awali.

# Headline of the page while blocking is starting.
settings-blocking-top-starting = Kuzuia kunaanza.

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = Matangazo na tovuti mbaya zinazuiwa, na utafutaji unaenda kwa faragha kwa { $provider }.

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = Seva ya faragha haipatikani kwenye mtandao huu, kwa hiyo seva za mtandao wako wenyewe zinatumika kwa sasa. Kuzuia bado kunafanya kazi.

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = Matangazo na tovuti mbaya zinazuiwa. Utafutaji huenda kwa seva za mtandao wako wenyewe, ambazo zinaweza kuuona.

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = Matangazo na tovuti mbaya zinazuiwa. Utafutaji huenda kwa seva za VPN yako wenyewe, kupitia VPN.

# Headline of the page when blocking is on and the lookups are still being set up.
settings-blocking-top-on = Matangazo na tovuti mbaya zinazuiwa.

# Small word in the two tiles while blocking is starting.
settings-blocking-tile-starting = Inaanza

# Small word in the two tiles while blocking is off.
settings-blocking-tile-off = Imezimwa

# Line under the two tiles while blocking is starting.
settings-blocking-tile-starting-note = kuzuia kunaanza

# Line under the two tiles while blocking is off.
settings-blocking-tile-off-note = kuzuia kumezimwa

# Name of the tile that counts the lookups blocked today.
settings-blocking-tile-today = Vilivyozuiwa leo

# Line under the count of lookups blocked today.
settings-blocking-tile-today-note = utafutaji uliozuiwa leo

# Name of the tile about the lists of sites.
settings-blocking-tile-lists = Orodha

# Number of the lists tile while the lists are being fetched.
settings-blocking-tile-fetching = Inapakua

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = Bado hamna

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = orodha za tovuti

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = tovuti zilizoorodheshwa, zilipakuliwa { $ago }

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = Bado hakuna kilichozuiwa

# Small line under "Nothing blocked yet".
settings-blocking-nothing-sub = Tovuti huonekana hapa, mpya zaidi kwanza.

# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
settings-blocking-let-through = Ruhusu tovuti hii kila wakati

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = Zilizozuiwa hivi karibuni

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = Tovuti huonekana hapa zinapozuiwa, mpya zaidi kwanza. Kitufe cha kuongeza (+) huruhusu moja kuanzia sasa. Ni tovuti zilizozuiwa tu zinazoonyeshwa hapa. Tovuti zilizoruhusiwa hazihifadhiwi kamwe.

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = Washa kuzuia ili kuona kinachozuiwa.

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = mf. example.com

# Heading of the line with the box for adding a site.
settings-blocking-add = Ongeza tovuti

# Small line under "Add a site".
settings-blocking-add-sub = Yenyewe na vikoa vyake vyote vidogo havizuiwi kamwe.

# Tip of the minus button on a site that is let through: blocks it again.
settings-blocking-block-again = Zuia tovuti hii tena

# Heading of the card that lists the sites that are let through.
settings-blocking-allowed = Ruhusiwa kila wakati

# Help text (i button) of the Always let through card.
settings-blocking-allowed-about = Tovuti zilizoorodheshwa hapa, na vikoa vyake vyote vidogo, havizuiwi kamwe, hata zikiwa kwenye orodha. Kitufe cha kutoa (-) huzuia moja tena.

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = Kuzuia

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = zuia matangazo vifuatiliaji programu hasidi ulaghai dns utafutaji wa faragha quad9 mullvad cloudflare ruhusu block adverts ads trackers malware scams private lookups allow

# Help text at the top of the page.
settings-blocking-help = Huzuia matangazo, vifuatiliaji na tovuti mbaya zinazojulikana kwa kila kivinjari na programu kwenye kompyuta hii, na hutuma utafutaji wa majina yake kupitia muunganisho uliosimbwa, ili mtandao uliopo usiweze kuyasoma wala kuyabadilisha.

# Text of the switch that turns blocking on or off.
settings-blocking-switch = Zuia matangazo na tovuti mbaya

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = Utafutaji unakoenda

# Help text (i button) of the Blocking card.
settings-blocking-card-about = Hutumia orodha za HaGeZi za matangazo, vifuatiliaji na tovuti mbaya zinazojulikana, zinazopakuliwa mara moja kwa siku. Hii ikiwa imezimwa, hakuna kinachoendeshwa. Utafutaji wa majina hupitia muunganisho uliosimbwa hadi seva ya faragha iliyochaguliwa hapa. Quad9 haihifadhi rekodi ya aliyeuliza. "Mtandao wako" bado huzuia, lakini si wa faragha: seva za mtandao wako wenyewe huona kila utafutaji.

# Small line under the Block adverts and bad sites switch.
settings-blocking-switch-sub = Orodha za HaGeZi, zinazopakuliwa mara moja kwa siku.

# Heading of the line with the private server dropdown.
settings-blocking-private = Seva ya faragha

# Small line under Private server.
settings-blocking-private-sub = Quad9 haihifadhi rekodi ya aliyeuliza.

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = Mtandao wako
