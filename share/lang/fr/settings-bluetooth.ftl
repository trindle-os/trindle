
# machine translated (French), not yet reviewed by a person.

# Said in red under the Bluetooth switch when switching it on failed for no known reason.
settings-bluetooth-on-failed = Impossible d'activer le Bluetooth.

# Said in red under the Bluetooth switch when switching it off failed for no known reason.
settings-bluetooth-off-failed = Impossible de désactiver le Bluetooth.

# Said in red when Bluetooth refuses something because it is not switched on; the first sentence says what could not be done.
settings-bluetooth-cause-not-ready = { $could_not } Le Bluetooth n'est pas activé.

# SAFETY: needs a human review before it ships
# Said in red when the PIN or number typed was wrong; the first sentence says what could not be done.
settings-bluetooth-cause-auth-failed = { $could_not } Le code PIN ou le nombre n'a pas été accepté. Réessayez.

# SAFETY: needs a human review before it ships
# Said in red when the pairing was turned down; the first sentence says what could not be done.
settings-bluetooth-cause-turned-down = { $could_not } L'appairage a été refusé, ici ou sur l'autre appareil.

# Said in red when the other device did not answer in time; the first sentence says what could not be done.
settings-bluetooth-cause-timeout = { $could_not } L'autre appareil n'a pas répondu à temps. Mettez-le en mode d'appairage, puis réessayez.

# Said in red when the same thing is already being done; the first sentence says what could not be done.
settings-bluetooth-cause-in-progress = { $could_not } Trindle est déjà en train de le faire. Patientez un instant.

# Said in red when the device did not answer; the first sentence says what could not be done.
settings-bluetooth-cause-no-answer = { $could_not } L'appareil n'a pas répondu. Vérifiez qu'il est allumé et à proximité, puis réessayez.

# Said in red when the device is not ready to be used; the first sentence says what could not be done.
settings-bluetooth-cause-not-available = { $could_not } L'appareil n'est pas prêt à être utilisé. Mettez-le en mode d'appairage, puis réessayez.

# The word for the system bus in "The system bus is not available".
settings-bluetooth-the-bus = Le bus système

# Headline of the page while Bluetooth is looked for.
settings-bluetooth-top-looking = Vérification du Bluetooth…

# Headline of the page when the Bluetooth service is not running.
settings-bluetooth-top-gone = Le Bluetooth ne fonctionne pas sur cet ordinateur.

# Headline of the page when Bluetooth is off.
settings-bluetooth-top-off = Le Bluetooth est désactivé.

# Headline of the page when Bluetooth is off at a switch or key on the computer itself.
settings-bluetooth-top-hard = Le Bluetooth est coupé par un interrupteur ou une touche de l'ordinateur.

# Headline of the page when the computer has no Bluetooth.
settings-bluetooth-top-none = Cet ordinateur n'a pas de Bluetooth.

# Headline of the page when the computer has an adapter that Bluetooth has not finished setting up.
settings-bluetooth-top-starting = Le Bluetooth démarre…

# Headline of the page when the adapter was there a moment ago and has gone.
settings-bluetooth-top-lost = L'adaptateur Bluetooth a disparu.

# Headline of the page when Bluetooth is on and searching.
settings-bluetooth-top-searching = Activé, recherche d'appareils…

# Headline of the page when Bluetooth is on and nothing is connected.
settings-bluetooth-top-idle = Activé, mais rien n'est connecté.

# Headline of the page when Bluetooth is on and devices are connected; the names are already joined into a list.
settings-bluetooth-top-connected =
    { $n ->
        [one] Activé, et { $names } est connecté.
        [many] Activé, et { $names } sont connectés.
       *[other] Activé, et { $names } sont connectés.
    }

# Help text (i button) of the Bluetooth card.
settings-bluetooth-card-about = Le nom sous l'interrupteur est celui de votre adaptateur Bluetooth. Tant que « Visible par les autres appareils » est activé, les appareils proches peuvent trouver cet ordinateur.

# Heading of the Bluetooth card, of its switch's line, and the name of its switch.
settings-bluetooth-title = Bluetooth

# Heading of the line with the switch that makes this computer visible, and the name of that switch.
settings-bluetooth-visible = Visible par les autres appareils

# Heading of the card when Bluetooth is off at a switch on the computer itself.
settings-bluetooth-hard-line = Coupé sur l'ordinateur lui-même

# Small line under it.
settings-bluetooth-hard-line-sub = Utilisez l'interrupteur ou la touche de l'ordinateur pour l'activer.

# Heading of the card when the computer has no Bluetooth adapter.
settings-bluetooth-no-adapter = Aucun adaptateur Bluetooth

# Help text (i button) of that card.
settings-bluetooth-no-adapter-about = Trindle n'a pas trouvé d'adaptateur Bluetooth dans cet ordinateur. Si vous en branchez un sur une prise USB, il s'affiche ici tout de suite.

# Line in that card.
settings-bluetooth-none-found = Aucun trouvé

# Line in the no-adapter card while Bluetooth finishes setting up the adapter.
settings-bluetooth-starting = Démarrage

# Line in the no-adapter card when the adapter was there a moment ago.
settings-bluetooth-lost = L'adaptateur a disparu

# Small line under it.
settings-bluetooth-lost-sub = Trindle continue de le chercher et l'affiche ici s'il revient. Sinon, redémarrez l'ordinateur.

# Shown beside the Bluetooth switch while the password is asked for.
settings-bluetooth-waiting = En attente de votre mot de passe…

# Said in red when this computer could not be made visible to other devices.
settings-bluetooth-visible-failed = Trindle n'a pas pu rendre cet ordinateur visible par les autres appareils.

# Said in red when this computer could not be hidden from other devices.
settings-bluetooth-hidden-failed = Trindle n'a pas pu masquer cet ordinateur aux autres appareils.

# Shown on the two device lists while Bluetooth is off.
settings-bluetooth-switch-on-first = Activez le Bluetooth pour les utiliser.

# Line in the My devices card when nothing is paired.
settings-bluetooth-none-paired = Aucun appareil appairé pour l'instant

# Heading of the card that lists paired devices.
settings-bluetooth-mine = Mes appareils

# Help text (i button) of the My devices card.
settings-bluetooth-mine-about = Les appareils appairés avec cet ordinateur, avec leur niveau de batterie quand il peut être lu. Les appareils connectés à l'ouverture de la page viennent en premier, et l'ordre ne change pas tant qu'elle reste ouverte. Si vous oubliez un appareil, il faudra l'appairer à nouveau.

# Line in the Nearby card while the search runs.
settings-bluetooth-looking = Recherche d'appareils…

# Tip of the button that stops the search.
settings-bluetooth-stop-looking = Arrêter la recherche

# Heading of the line that starts the search, and tip of its button.
settings-bluetooth-look = Rechercher des appareils

# Heading of the card that lists devices nearby.
settings-bluetooth-nearby = À proximité

# Help text (i button) of the Nearby card.
settings-bluetooth-nearby-about = Appuyez sur Rechercher des appareils, mettez l'autre appareil en mode d'appairage, puis appuyez sur Appairer à côté de lui. À l'ouverture de la page, l'appareil au signal le plus fort est listé en premier, et chaque nouvel appareil trouvé est ajouté à la fin. Certains appareils affichent un nombre à comparer avec celui d'ici. D'autres demandent un code PIN, souvent indiqué dans leur mode d'emploi, ou 0000.

# Said in red when the search could not be stopped.
settings-bluetooth-stop-failed = Trindle n'a pas pu arrêter la recherche d'appareils.

# Said in red when the search could not be started.
settings-bluetooth-start-failed = Impossible de rechercher des appareils.

# Small line of a device that is connected.
settings-bluetooth-connected = Connecté

# Small line of a paired device that is not connected.
settings-bluetooth-not-connected = Non connecté

# Part of a device's small line: its battery level in per cent.
settings-bluetooth-battery = batterie { $n } %

# SAFETY: needs a human review before it ships
# Tip of the button that pairs a device, and the button of the same name in the pairing question.
settings-bluetooth-pair = Appairer

# Tip of the button that disconnects a device.
settings-bluetooth-disconnect = Déconnecter

# Tip of the button that connects a device.
settings-bluetooth-connect = Connecter

# SAFETY: needs a human review before it ships
# Tip of the bin button that forgets a device.
settings-bluetooth-forget = Oublier cet appareil

# Said in red when a device could not be disconnected.
settings-bluetooth-disconnect-failed = Impossible de le déconnecter.

# Said in red when a device could not be forgotten.
settings-bluetooth-forget-failed = Impossible de l'oublier.

# Shown under a device while it is being paired.
settings-bluetooth-pairing = Appairage…

# Said in red when a device could not be paired.
settings-bluetooth-pair-failed = Impossible de l'appairer.

# Shown under a device while it is being connected.
settings-bluetooth-connecting = Connexion…

# Said in red when a device could not be connected.
settings-bluetooth-connect-failed = Impossible de le connecter.

# Said under a device that paired but could not be set to connect by itself.
settings-bluetooth-not-trusted = Connecté, mais Trindle n'a pas pu le régler pour qu'il se connecte tout seul la prochaine fois. Connectez-le ici quand vous voulez l'utiliser.

# Said under a device that shows a code to be typed on it; the code is already written.
settings-bluetooth-type-code = Saisissez { $code } sur l'appareil, puis appuyez sur Entrée sur celui-ci.

# SAFETY: needs a human review before it ships
# Question under a device that shows a number to compare with the one on this computer.
settings-bluetooth-compare = L'autre appareil affiche-t-il { $code } ?

# SAFETY: needs a human review before it ships
# Tip of the button that answers no to a pairing question.
settings-bluetooth-no = Non

# SAFETY: needs a human review before it ships
# Tip of the button that says the numbers match.
settings-bluetooth-yes-matches = Oui, il correspond

# SAFETY: needs a human review before it ships
# Question under a device that asks to be paired.
settings-bluetooth-pair-question = Appairer avec cet appareil ?

# Placeholder of the box where a device's PIN is typed.
settings-bluetooth-pin = Code PIN

# SAFETY: needs a human review before it ships
# Tip of the button that refuses to type a PIN.
settings-bluetooth-cancel = Annuler

# Help line above the box where a device's PIN is typed.
settings-bluetooth-pin-help = Saisissez le code PIN de cet appareil. Il figure souvent dans le mode d'emploi. Sinon, essayez 0000.

# Words the settings search looks for on this page (not shown).
settings-bluetooth-keywords = headphones mouse keyboard pair devices wireless casque souris clavier appairer appareils sans fil

# Help text at the top of the page.
settings-bluetooth-help = Appuyez sur Rechercher, mettez l'autre appareil en mode d'appairage, puis appuyez sur Appairer à côté de lui.

# Said in red under the Bluetooth switch when restarting the sound service after Bluetooth came back on failed; $why is the system's own reason.
settings-bluetooth-sound-failed = Trindle n'a pas pu redémarrer le son Bluetooth : le son du casque peut donc être moins bon que d'habitude jusqu'à votre prochaine ouverture de session. { $why }

# Tooltip of the help button beside the Bluetooth switch.
settings-bluetooth-sound-help = Pourquoi le son du casque est moins bon après avoir désactivé puis réactivé le Bluetooth


# Said in red when a device is no longer in BlueZ's list; the first sentence says what could not be done.
settings-bluetooth-cause-gone = { $could_not } Cet appareil n'est plus enregistré sur cet ordinateur. Recherchez-le de nouveau.

# Said in red when a paired device's saved pairing is refused; the first sentence says what could not be done.
settings-bluetooth-cause-old-pairing = { $could_not } L'appareil n'accepte plus l'appairage enregistré. Oubliez l'appareil, puis appairez-le de nouveau.

# Said in red under a device that connected and then dropped at once.
settings-bluetooth-dropped = Il s'est connecté, puis a perdu la connexion. L'appairage enregistré est peut-être périmé.

# Tooltip of the button that forgets a paired device and looks for it afresh.
settings-bluetooth-pair-again = Oublier et appairer de nouveau

# Said under Look for devices after a device was forgotten to be paired afresh.
settings-bluetooth-forgotten-pair = L'appareil a été oublié. Mettez-le en mode d'appairage, puis appuyez sur Appairer quand il s'affiche ci-dessous.

# Tooltip of the button that opens Help from a failed connection.
settings-bluetooth-more = En savoir plus
