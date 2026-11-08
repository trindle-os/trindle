# machine translated (French), not yet reviewed by a person.

# Name of the Settings page about displays, in the list and as its heading.
settings-display-title = Affichage

# Words a person might type into the Settings search to find the display page; separate them with spaces.
settings-display-keywords = écran moniteur résolution taille échelle zoom plus grand plus petit pivoter rotation actualisation lumière de nuit chaud lumière bleue soir yeux

# Introduction under the heading of the display page, and the words behind the page i.
# SAFETY: needs a human review before it ships
settings-display-help = Les changements s'appliquent tout de suite. Si l'écran se met à mal fonctionner, ne faites rien : il revient en arrière tout seul après 15 secondes, sauf si vous appuyez sur Garder. La lumière de nuit rend l'écran plus chaud le soir.

# Question along the bottom of the page while a change of displays waits to be kept.
# SAFETY: needs a human review before it ships
settings-display-foot-keep = Garder cette disposition ?

# Sentence at the top of the page while the displays are being found.
settings-display-looking = Recherche des écrans…

# Sentence at the top of the page when no display was found.
settings-display-none-found = Aucun écran n'a été trouvé.

# First half of the sentence at the top of the page: no display is in use.
settings-display-used-none = Aucun écran n'est utilisé

# First half of the sentence at the top of the page: one display is in use.
settings-display-used-one = Un écran est utilisé

# First half of the sentence at the top of the page; { $n } is how many displays are in use.
settings-display-used-many =
    { $n ->
        [one] { $n } écran utilisé
        [many] { $n } d'écrans utilisés
       *[other] { $n } écrans utilisés
    }

# Second half of the sentence at the top of the page: night light is off.
settings-display-warm-off = la lumière de nuit est désactivée

# Second half of the sentence at the top of the page; { $from } and { $to } are times.
settings-display-warm-times = la lumière de nuit réchauffe l'écran de { $from } à { $to }

# Second half of the sentence at the top of the page: night light follows the sun.
settings-display-warm-sun = la lumière de nuit réchauffe l'écran du coucher au lever du soleil

# Sentence at the top of the page; { $used } says how many displays are in use and { $warm } what night light does.
settings-display-sum-up = { $used } ; { $warm }.

# Choice in the list of rotations: not rotated.
settings-display-turn-normal = Normale

# Choice in the list of rotations: turned a quarter to the right.
settings-display-turn-right = Pivotée à droite

# Choice in the list of rotations: turned a quarter to the left.
settings-display-turn-left = Pivotée à gauche

# Choice in the list of rotations: turned upside down.
settings-display-turn-upside = À l'envers

# Choice in the list of where a display sits: to the right of the other.
settings-display-place-right = À droite de

# Choice in the list of where a display sits: to the left of the other.
settings-display-place-left = À gauche de

# Choice in the list of where a display sits: above the other.
settings-display-place-above = Au-dessus de

# Choice in the list of where a display sits: below the other.
settings-display-place-below = Au-dessous de

# A screen resolution; { $width } and { $height } are the dots across and down.
settings-display-mode = { $width } x { $height }

# A screen resolution with its refresh rate; { $width } and { $height } are the dots across and down and { $hz } the times a second.
settings-display-mode-hz = { $width } x { $height } à { $hz } Hz

# Choice in the list of resolutions that is the screen's own; { $mode } is the resolution.
settings-display-recommended = { $mode }  (recommandée)

# A refresh rate; { $hz } is the times a second.
settings-display-hz = { $hz } Hz

# Choice in the list of sizes that is the standard one; { $percent } is the percent.
settings-display-size-standard = { $percent } %  (standard)

# Name of the list of resolutions; { $display } is the display.
settings-display-resolution-of = Résolution de { $display }

# Name of the list of sizes; { $display } is the display.
settings-display-size-on = Taille sur { $display }

# Name of the list of rotations; { $display } is the display.
settings-display-rotation-of = Rotation de { $display }

# Heading of the resolution line.
settings-display-resolution = Résolution

# Small words under that line.
settings-display-resolution-about = Combien de points elle affiche, et à quelle fréquence.

# Heading of the size line.
settings-display-size = Taille

# Small words under that line.
settings-display-size-about = La grandeur de tout ce qui s'affiche sur cet écran.

# Heading of the rotation line.
settings-display-rotation = Rotation

# Small words under that line.
settings-display-rotation-about = Pour un écran posé sur le côté.

# Heading of the line, and name of its switch, that turns a display on or off.
settings-display-use = Utiliser cet écran

# Small words under that line for the only display in use.
settings-display-use-only = Le seul écran utilisé ne peut pas être éteint.

# Small words under that line otherwise.
settings-display-use-off = Éteint, rien n'y est affiché.

# Words behind the i of a display's card.
# SAFETY: needs a human review before it ships
settings-display-card-about = Les changements s'appliquent tout de suite. Si l'écran se met à mal fonctionner, ne faites rien : il revient en arrière tout seul après 15 secondes, sauf si vous appuyez sur Garder. La résolution recommandée est celle propre à l'écran ; la taille standard est celle que Trindle a choisie pour cet écran.

# Name of the list of where a display sits, and heading of its line when there are more than two displays; { $display } is the display.
settings-display-place-name = Placer { $display }

# Heading of the line for placing the second display, when there are only two.
settings-display-place-it = Le placer

# Small words under a display's placing line; { $display } is the display and { $other } the one it is placed beside.
settings-display-relative = { $display } par rapport à { $other }

# Title of the card about where displays sit.
settings-display-arrangement-title = Disposition

# Words behind the i of that card.
settings-display-arrangement-about = La place des écrans les uns à côté des autres, pour que le pointeur passe de l'un à l'autre en franchissant le bord qu'ils partagent.

# Small words on that card while fewer than two displays are in use.
settings-display-arrangement-waits = Utilisez deux écrans pour en placer un à côté de l'autre.

# Red words when the desktop refused a change of displays and gave no reason.
settings-display-refused = Le bureau l'a refusé.

# Said aloud after the displays changed; { $seconds } is how long until they go back.
# SAFETY: needs a human review before it ships
settings-display-aloud-changed =
    { $seconds ->
        [one] Les écrans ont changé. Retour en arrière dans { $seconds } seconde, sauf si vous appuyez sur Garder.
        [many] Les écrans ont changé. Retour en arrière dans { $seconds } de secondes, sauf si vous appuyez sur Garder.
       *[other] Les écrans ont changé. Retour en arrière dans { $seconds } secondes, sauf si vous appuyez sur Garder.
    }

# Red words when a change of displays could not be undone; { $why } is the desktop's reason.
settings-display-not-undone = Le changement n'a pas pu être annulé. { $why }

# Red words when the arrangement could not be kept for the next login.
settings-display-not-kept = La disposition est utilisée maintenant, mais n'a pas pu être conservée pour la prochaine ouverture de session.

# Heading of the line under the picture of the displays when the desktop refused a change.
settings-display-nothing-changed = Rien n'a été modifié

# Small words under that line.
settings-display-refused-change = Le bureau a refusé le changement.

# Heading of the line under the picture of the displays.
settings-display-apply-at-once = Les changements s'appliquent tout de suite

# Small words under that line; { $seconds } is how long Keep is asked for.
settings-display-keep-asked =
    { $seconds ->
        [one] La question « Garder » est posée en bas de la fenêtre pendant { $seconds } seconde.
        [many] La question « Garder » est posée en bas de la fenêtre pendant { $seconds } de secondes.
       *[other] La question « Garder » est posée en bas de la fenêtre pendant { $seconds } secondes.
    }

# Title of the night light card.
settings-display-night-title = Lumière de nuit

# Words behind the i of that card.
settings-display-night-about = L'écran devient plus chaud le soir, ce qui repose les yeux. Elle suit le coucher et le lever du soleil, calculés d'après le fuseau horaire, ou des heures que vous fixez ; quand le fuseau horaire n'indique pas où se trouve cet ordinateur, les heures fixées sont utilisées.

# Heading of the night light line, and name of its switch.
settings-display-night = Lumière de nuit

# Small words under that line.
settings-display-night-about-line = Des couleurs plus chaudes le soir.

# Small words under the When line; { $zone } is the time zone.
settings-display-when-zone = Coucher et lever du soleil à { $zone }.

# Small words under the When line when the time zone gives no place.
settings-display-when-no-place = Le fuseau horaire n'indique aucun lieu : heures fixées.

# Choice in the When list: from sunset to sunrise.
settings-display-when-sun = Du coucher au lever du soleil

# Choice in the When list: between times the person sets.
settings-display-when-times = Heures fixées

# Heading of the When line, and name of its list.
settings-display-when = Quand

# Heading of the line, and name of its list, for the time night light starts.
settings-display-from = De

# Heading of the line, and name of its list, for the time night light ends.
settings-display-to = À

# Heading of the warmth line, and name of its slider.
settings-display-warmth = Chaleur

# Small words under the warmth line.
settings-display-warmth-about = À quel point l'écran se réchauffe la nuit.

# Small word under the left end of the warmth slider.
settings-display-little = Un peu

# Small word under the right end of the warmth slider.
settings-display-very = Très chaud

# Spoken description of the strip of colours night light runs through.
settings-display-strip = De un peu chaud à très chaud

# Spoken description of the picture of the displays when none is in use.
settings-display-no-display-tip = Aucun écran utilisé

# Spoken description of the picture of the displays when one is in use; { $name } is its name and { $res } its resolution.
settings-display-one-display-tip = { $name }, { $res }

# Spoken description of the picture of the displays when several are in use; { $n } is how many.
settings-display-many-displays-tip =
    { $n ->
        [one] { $n } écran, côte à côte comme disposé
        [many] { $n } d'écrans, côte à côte comme disposés
       *[other] { $n } écrans, côte à côte comme disposés
    }

# Word drawn in the picture of the displays when none is in use.
settings-display-no-display = Aucun écran

# Name of the large picture of the displays.
settings-display-your-displays = Vos écrans

# Small words under Nothing to show on when the desktop listed no displays.
settings-display-listed-none = Le bureau n'a listé aucun écran.

# Title of the card shown when no display was found.
settings-display-none-title = Aucun écran trouvé

# Heading of the line in that card.
settings-display-none-heading = Rien où afficher

# Title of the card about the graphics card.
settings-display-graphics-title = Graphismes

# Words behind the i of that card.
settings-display-graphics-about = Cet ordinateur a une carte NVIDIA, qui utilise le pilote libre. Tout fonctionne. Sur une carte de 2022 environ ou plus récente, ce sera lent, car le pilote libre ne peut pas augmenter la vitesse de la carte. Le pilote propre à NVIDIA peut être installé.

# Heading of the line about an NVIDIA card on the open driver.
settings-display-nvidia = Une carte NVIDIA, avec le pilote libre

# Small words under that line.
settings-display-nvidia-about = Fonctionne ; une carte de 2022 ou plus récente semble lente.

# Words of the link that opens help about the NVIDIA driver.
settings-display-nvidia-help = Que faire
