
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
settings-blocking-waiting = Inasubiri nenosiri lako…

# Shown beside the switch after blocking is turned on.
settings-blocking-done-on = Kuzuia kumewashwa

# Shown beside the switch after blocking is turned off.
settings-blocking-done-off = Kuzuia kumezimwa

# Shown beside a control after a change to blocking was saved.
settings-blocking-saved = Imehifadhiwa

# Red words under the Add a site box when what was typed is not a site name.
settings-blocking-bad-site = Andika jina la tovuti, kama example.com.

# Headline of the page when blocking is off.
settings-blocking-top-off = Kuzuia kumezimwa. Tovuti hutafutwa kupitia seva za mtandao wako wenyewe, kama awali.

# Headline of the page while blocking is starting.
settings-blocking-top-starting = Kuzuia kunaanza.

# Headline of the page when blocking works and lookups go privately to the chosen server.
settings-blocking-top-private = Matangazo na tovuti mbaya zinazuiwa, na tovuti hutafutwa kwa faragha kupitia { $provider }.

# Headline of the page when the private server cannot be reached on this network.
settings-blocking-top-fallback = Trindle haiwezi kufikia seva ya faragha kwenye mtandao huu, kwa hiyo inatumia seva za mtandao wako wenyewe kwa sasa. Kuzuia bado kunafanya kazi.

# Headline of the page when lookups go to the network's own servers.
settings-blocking-top-network = Matangazo na tovuti mbaya zinazuiwa. Tovuti hutafutwa kupitia seva za mtandao wako wenyewe, ambazo zinaweza kuona kila tovuti unayotembelea.

# Headline of the page when lookups go through the VPN.
settings-blocking-top-vpn = Matangazo na tovuti mbaya zinazuiwa. Tovuti hutafutwa kupitia seva za VPN yako wenyewe.

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
settings-blocking-tile-fetching = Inasasisha

# Number of the lists tile when no list has been fetched yet.
settings-blocking-tile-none = Bado hamna

# Line under the lists tile when no list has been fetched yet.
settings-blocking-tile-lists-note = orodha za tovuti

# Line under the lists tile: how many sites are listed and how long ago they were fetched.
settings-blocking-tile-listed-note = tovuti zilizoorodheshwa, zilisasishwa { $ago }

# Line in the Recently blocked card when nothing has been blocked.
settings-blocking-nothing = Bado hakuna kilichozuiwa

# Tip of the plus button on a blocked site, and beside the Add a site box: lets the site through.
settings-blocking-let-through = Ruhusu tovuti hii kila wakati

# Heading of the card that lists recently blocked sites.
settings-blocking-recent = Zilizozuiwa hivi karibuni

# Help text (i button) of the Recently blocked card.
settings-blocking-recent-about = Tovuti huonekana hapa zinapozuiwa, mpya zaidi kwanza. Bonyeza kitufe cha kuongeza (+) ili kuruhusu tovuti kuanzia sasa. Trindle haihifadhi rekodi ya tovuti inazoruhusu.

# Shown on the Recently blocked card while blocking is off.
settings-blocking-recent-waits = Washa kuzuia ili kuona kinachozuiwa.

# Placeholder of the box where a site name is typed.
settings-blocking-site-placeholder = example.com

# Heading of the line with the box for adding a site.
settings-blocking-add = Ongeza tovuti

# Tip of the minus button on a site that is let through: blocks it again.
settings-blocking-block-again = Zuia tovuti hii tena

# Heading of the card that lists the sites that are let through.
settings-blocking-allowed = Ruhusiwa kila wakati

# Help text (i button) of the Always let through card.
settings-blocking-allowed-about = Tovuti hizi, na kila anwani iliyo chini yake, hazizuiwi kamwe, hata zikiwa kwenye orodha. Bonyeza kitufe cha kutoa (-) ili kuzuia tovuti tena.

# Name of the page in the list of settings pages and its heading.
settings-blocking-title = Kuzuia

# Words the settings search looks for on this page (not shown).
settings-blocking-keywords = zuia matangazo vifuatiliaji programu hasidi ulaghai dns utafutaji wa faragha quad9 mullvad cloudflare ruhusu block adverts ads trackers malware scams private lookups allow

# Help text at the top of the page.
settings-blocking-help = Huzuia matangazo, vifuatiliaji na tovuti mbaya zinazojulikana kwa kila kivinjari na programu kwenye kompyuta hii. Pia hutafuta tovuti kupitia muunganisho uliosimbwa, ili mtandao uliopo usiweze kuona wala kubadilisha tovuti unazotafuta.

# Text of the switch that turns blocking on or off.
settings-blocking-switch = Zuia matangazo na tovuti mbaya

# Tip of the dropdown that picks where lookups go.
settings-blocking-where = Utafutaji unakoenda

# Help text (i button) of the Blocking card.
settings-blocking-card-about = Hutumia orodha za HaGeZi za matangazo, vifuatiliaji na tovuti mbaya zinazojulikana, zinazosasishwa mara moja kwa siku. Kuzuia kukiwa kumezimwa, hakuna kinachoendeshwa. Tovuti hutafutwa kupitia muunganisho uliosimbwa hadi seva ya faragha unayochagua hapa. Quad9 haihifadhi rekodi ya aliyeuliza. "Mtandao wako" bado huzuia, lakini si wa faragha, kwa sababu seva za mtandao wako wenyewe huona kila tovuti unayotafuta.

# Heading of the line with the private server dropdown.
settings-blocking-private = Seva ya faragha

# Name of the choice in Private server that uses the network's own servers.
settings-blocking-provider-network = Mtandao wako


# Heading of the Firewall card, first on the Firewall and blocking page.
settings-firewall = Ngome ya moto

# Help text (i button) of the Firewall card.
settings-firewall-about = Ngome ya moto ikiwa imewashwa, kompyuta nyingine haziwezi kuunganisha kwenye hii isipokuwa kompyuta hii ndiyo iliyoanzisha muunganisho. Kwa hiyo hakuna mtu aliye kwenye Wi-Fi ile ile anayeweza kujaribu kuingia. Kuvinjari, masasisho na vichapishi bado hufanya kazi. Programu inayosubiri kompyuta nyingine ziunganishe, kama seva ya mchezo, inahitaji mlango wake ufunguliwe hapa chini. Ngome ya moto ikizimwa, kompyuta yoyote kwenye mtandao inaweza kufikia kila programu inayosubiri miunganisho. Unahitaji nenosiri la msimamizi ili kuiwasha au kuizima.

# Name of the firewall's on and off switch.
settings-firewall-switch = Ngome ya moto

# Small line under the switch while the firewall is off.
settings-firewall-off-sub = Kompyuta nyingine zinaweza kufikia programu zinazozisubiri.

# Said beside the switch once the firewall is on.
settings-firewall-done-on = Ngome ya moto imewashwa

# Said beside the switch once the firewall is off.
settings-firewall-done-off = Ngome ya moto imezimwa

# Heading of the card that blocks adverts and bad sites, on the Firewall and blocking page.
settings-blocking-card = Kuzuia


# Small line under the firewall switch when it is on but its rules are not loaded.
settings-firewall-not-in-force = Ngome ya moto imewashwa lakini haifanyi kazi, kwa sababu kanuni zake hazikupakiwa kompyuta ilipowashwa. Izime kisha uiwashe tena.
