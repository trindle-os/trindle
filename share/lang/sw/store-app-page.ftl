
# machine translated (Swahili), not yet reviewed by a person.

# Under the app's name: its one line, with the maker left out because the app has none to show.
store-app-line-said = { $said }.

# Under the app's name when only the maker is known.
store-app-line-by = Na { $by }.

# Under the app's name: its one line, then who made it.
store-app-line-both = { $said }, na { $by }.

# The state beside the dot while the app is being taken off.
# SAFETY: needs a human review before it ships
store-app-state-removing = Inaondolewa

# The state beside the dot while the app is being fetched.
store-app-state-downloading = Inapakuliwa

# The state beside the dot while Flatpak is putting the fetched app in.
store-app-state-putting-in = Inawekwa

# The state beside the dot while the app's own update goes in.
store-app-state-updating = Inasasishwa

# The state beside the dot for an installed app whose updates are held back and whose version is not known.
store-app-state-held = Imesakinishwa, masasisho yamezuiwa

# The state beside the dot for an installed app whose updates are held back.
store-app-state-held-version = Imesakinishwa, masasisho yamezuiwa kwenye toleo { $version }

# The state beside the dot for an installed app with an update waiting; the end says what waits.
store-app-state-waiting = Imesakinishwa, { $waits }

# The same, with the installed version.
store-app-state-waiting-version = Imesakinishwa, toleo { $version }, { $waits }

# The state beside the dot for an installed app that is up to date.
store-app-state-current = Imesakinishwa, imesasishwa

# The same, with the installed version.
store-app-state-current-version = Imesakinishwa, toleo { $version }, imesasishwa

# The state beside the dot for an app that is not installed and whose size is not known.
store-app-state-absent = Haijasakinishwa

# The same, with the size of the download.
store-app-state-absent-size = Haijasakinishwa, { $size } za kupakua

# Word beside the spinner while the app is being taken off.
# SAFETY: needs a human review before it ships
store-app-busy-removing = Inaondolewa

# Word beside the spinner while the app is being put in.
store-app-busy-installing = Inasakinishwa

# Tip and spoken name of the round button that gives up a download.
store-app-stop = Simamisha

# Tip and spoken name of the round button that takes the app off.
# SAFETY: needs a human review before it ships
store-app-remove = Ondoa

# Round button beside Update or Release that opens the app.
store-app-open-round = Fungua

# Main button that releases an app whose updates are held back.
store-app-release = Achia

# Main button that updates the app.
store-app-update = Sasisha

# Main button that opens the app.
store-app-open = Fungua

# Main button that installs the app.
store-app-install = Sakinisha

# Button that opens the app's website.
store-app-website = Tovuti

# In the picture frame when the app has no pictures.
store-app-no-pictures = Programu hii haina picha.

# In the picture frame when the picture could not be fetched.
store-app-picture-missed = Picha hii haikuweza kupakuliwa.

# In the picture frame while the picture is being fetched.
store-app-picture-fetching = Inapakua picha…

# In the picture frame when the computer has no internet.
store-app-picture-offline = Picha haziwezi kupakuliwa wakati kompyuta hii haijaunganishwa na intaneti.

# Between the picture buttons: which picture, when it has no caption.
store-app-picture-of = Picha { $at } kati ya { $n }

# Between the picture buttons: which picture, then its caption.
store-app-picture-of-caption = Picha { $at } kati ya { $n }: { $caption }

# Tip of the round button that shows the picture before.
store-app-picture-earlier = Picha iliyotangulia

# Tip of the round button that shows the picture after.
store-app-picture-later = Picha inayofuata

# Licence shown in Facts for an app that is not open source.
store-app-proprietary = Miliki

# Name of the Facts card.
store-app-facts = Taarifa

# Facts line: the size of the download.
store-app-fact-download = Ukubwa wa kupakua

# Facts line: where the app comes from.
store-app-fact-source = Chanzo

# Facts line: the version.
store-app-fact-version = Toleo

# Facts line: the licence of an open source app.
store-app-fact-licence = Leseni

# Facts line: the licence of an app that is not open source.
store-app-fact-licence-closed = Leseni, si chanzo huria

# Facts line: who made the app.
store-app-fact-made-by = Imetengenezwa na

# Under the stars: how many people rated the app.
store-app-rating-from = { $stars }  kutoka { $n ->
        [one] ukadiriaji { $count }
       *[other] ukadiriaji { $count }
    }

# Under the stars when nobody has rated the app.
store-app-rating-none = Bado hakuna ukadiriaji.

# Under the stars when ratings cannot be fetched.
store-app-rating-none-offline = Hakuna ukadiriaji wa kuonyesha.

# Under the stars while the ratings are being fetched.
store-app-rating-looking = Inatafuta ukadiriaji…

# Last line of the Rating card: where the ratings come from.
store-app-rating-source = Kutoka huduma huria ya maoni ya GNOME.

# Last line of the Rating card when there is no internet.
store-app-rating-offline = Kompyuta hii haijaunganishwa na intaneti.

# Name of the Rating card.
store-app-rating = Ukadiriaji

# Text of the About card when the app says nothing of itself.
store-app-no-description = Programu hii haina maelezo.

# Name of the About card.
store-app-about = Kuhusu

# Heading of a review that has no title.
store-app-review-no-title = Hakuna kichwa

# Under What people say when nobody has written.
store-app-people-none = Bado hakuna aliyeandika kuhusu programu hii.

# Under What people say when there is no internet.
store-app-people-offline = Wanachosema watu hakiwezi kupakuliwa wakati kompyuta hii haijaunganishwa na intaneti.

# Under What people say while reviews are being fetched.
store-app-people-looking = Inatafuta wanachosema watu…

# Name of the card of reviews.
store-app-people = Wanachosema watu

# Facts line: when the app's project was made.
store-app-fact-created = Ilitengenezwa

