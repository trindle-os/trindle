# machine translated (French), not yet reviewed by a person.

# Name of the Crash reports page, in the page list and as its heading.
settings-crash-title = Rapports de plantage

# Words people may search for to find the Crash reports page, separated by spaces.
settings-crash-keywords = plantage plantages rapports erreurs bogues bugs pannes envoi envoyé confidentialité télémétrie buildmorphic

# Help line at the foot of the Crash reports page.
settings-crash-help = Quand un programme de Trindle plante, le module de rapports de plantage garde une courte note sur cet ordinateur. Il n'envoie rien tant que vous n'avez pas lancé sa commande d'envoi et accepté chaque rapport, qui vous montre d'abord le rapport en entier. Votre nom et votre nom GitHub n'y figurent que si vous les avez saisis ici. Jamais un message, un nom de fichier, l'endroit où vous êtes ni rien sur d'autres programmes. Retirer le module arrête tout.

# Heading and label of the switch for sending crashes.
settings-crash-crashes = Plantages

# Under the Crashes switch.
settings-crash-crashes-short = L'endroit du code de Trindle où le plantage a eu lieu.

# Part of the help text of the What is sent card: what Crashes sends.
settings-crash-crashes-long = Les plantages envoient le programme, sa version, le type de plantage et l'endroit du code de Trindle où il a eu lieu.

# Heading and label of the switch for sending errors.
settings-crash-errors = Erreurs

# Under the Errors switch.
settings-crash-errors-short = Le nom fixe de l'erreur, et où.

# Part of the help text of the What is sent card: what Errors sends.
settings-crash-errors-long = Les erreurs envoient le programme, sa version, le nom fixe que Trindle donne à l'erreur et l'endroit dans son code.

# Heading and label of the switch for sending facts about the computer.
settings-crash-computer = À propos de cet ordinateur

# Under the About this computer switch.
settings-crash-computer-short = Linux, processeur, mémoire, graphismes, écrans.

# Part of the help text of the What is sent card: what About this computer adds.
settings-crash-computer-long = « À propos de cet ordinateur » ajoute à chaque rapport : le Linux utilisé, le type de processeur, la mémoire arrondie à la baisse, Wayland ou X11, le fabricant des graphismes et le nombre d'écrans.

# Heading of the card with the switches for what is sent.
settings-crash-what-is-sent = Ce qui est envoyé

# Help text of the What is sent card: { $switches } is what each switch sends.
settings-crash-what-about = { $switches } Les rapports ne portent aucun nom, sauf si vous saisissez votre nom d'utilisateur Buildmorphic dans la case ci-dessous, ce qui vous donne seulement du crédit.

# Red words when a setting could not be changed: the reason is { $why }.
settings-crash-nothing-changed-because = Rien n'a été modifié, car { $why }.

# Red words when the file that keeps the switches was not written by Trindle.
settings-crash-file-not-as-wrote = Rien n'a été modifié, car le fichier qui le conserve n'est pas tel que Trindle l'a écrit.

# Red words when there is no folder to keep the settings in.
settings-crash-no-settings-folder = Aucun dossier de réglages.

# Red words when the folder for the settings could not be made.
settings-crash-folder-not-made = Le dossier de réglages n'a pas pu être créé.

# Red words when a setting could not be written to its file.
settings-crash-not-written = Le réglage n'a pas pu être enregistré.

# Name of a sent report that was a crash: the signal is { $signal } and the program { $program }.
settings-crash-what-crash = Plantage ({ $signal }) dans { $program }

# Name of a sent report that was an error: the error is { $code } and the program { $program }.
settings-crash-what-error = Erreur { $code } dans { $program }

# In the list of sent reports: the report was taken.
settings-crash-taken = Reçu

# In the list of sent reports: the report was not sent because the bug tracker is closed.
settings-crash-tracker-closed = Non envoyé : suivi fermé

# In the list of sent reports: the report was not sent because it was too big.
settings-crash-too-big = Non envoyé : trop gros

# In the list of sent reports: the report was not sent because it was refused.
settings-crash-refused = Non envoyé : refusé

# In the list of sent reports: the report was not sent because too many came at once.
settings-crash-too-many = Non envoyé : trop à la fois

# In the list of sent reports: the report was not sent, no reason known.
settings-crash-not-sent = Non envoyé

# Said beside the link line once it is saved.
settings-crash-saved = Enregistré

# Tip of the button that clears the list of sent reports.
# SAFETY: needs a human review before it ships
settings-crash-clear = Vider la liste

# Red words when the list of sent reports could not be cleared.
settings-crash-not-cleared = La liste n'a pas pu être vidée.

# Said beside the list of sent reports once it is cleared.
settings-crash-removed = Supprimé

# Heading of the line that holds the clear button.
settings-crash-kept-here = Conservé sur cet ordinateur uniquement

# Under the Kept on this computer only heading.
settings-crash-newest-first = Les plus récents d'abord, jusqu'à cinquante.

# Heading of the line shown when no report has been sent.
settings-crash-nothing-yet = Rien pour le moment

# Under the Nothing yet heading.
settings-crash-listed-here = Chaque rapport est listé ici, exactement tel qu'il a été envoyé.

# Tip of the button that shows exactly what one report held.
settings-crash-show-sent = Afficher exactement ce qui a été envoyé

# Under a sent report: { $when } is the time, { $n } is how many times it happened (0 when once) and { $answer } how it went.
settings-crash-sent-sub =
    { $n ->
        [0] { $when } : { $answer }
        [one] { $when }, { $n } fois : { $answer }
       *[other] { $when }, { $n } fois : { $answer }
    }

# Heading of the card listing reports that were sent.
settings-crash-sent-so-far = Envoyés jusqu'ici

# Help text of the Sent so far card (the information button).
settings-crash-sent-about = Chaque rapport est listé exactement tel qu'il a été envoyé, le plus récent d'abord, et conservé sur cet ordinateur uniquement. « Reçu » signifie que Buildmorphic l'a. « Non envoyé : suivi fermé » signifie que le suivi de bogues de Trindle n'accepte pas de rapports ; « refusé » signifie qu'il a été refusé car mal formé ; « trop à la fois » signifie que trop de rapports sont arrivés en même temps.

# Headline of the page when the add-on is not installed and the person is asked after a crash.
settings-crash-sum-asked = Après un plantage, on vous demande, avec le rapport entier affiché, si vous voulez l'envoyer.

# Headline of the page when nothing is sent and the person is not asked.
settings-crash-sum-nothing = Rien n'est envoyé, et on ne vous demande rien après un plantage.

# Headline of the page when both crashes and errors are turned off.
settings-crash-sum-none = Rien n'est envoyé : les plantages et les erreurs sont tous deux désactivés.

# Headline of the page: what is reported (crashes, errors or both) and under whose name (named, linked or anonymous) in { $which }, and the Buildmorphic name in { $name }.
settings-crash-sum-reported =
    { $which ->
        [errors] Les erreurs des programmes de Trindle sont signalées de façon anonyme.
        [both] Les plantages et les erreurs des programmes de Trindle sont signalés de façon anonyme.
       *[crashes] Les plantages des programmes de Trindle sont signalés de façon anonyme.
    }

# Label of the switch for asking about problems and sharing reports, and heading of its line.
settings-crash-ask = Demander en cas de problème et partager les rapports

# Under the Ask about problems heading.
# SAFETY: needs a human review before it ships
settings-crash-ask-sub = Désactivé, tout est masqué et les rapports en attente sont supprimés.

# Said beside the Ask switch when it is turned off and no reports were waiting.
settings-crash-off-none = Désactivé. Aucun rapport en attente.

# Said beside the Ask switch when it is turned off: how many waiting reports were deleted.
# SAFETY: needs a human review before it ships
settings-crash-off-deleted =
    { $n ->
        [one] Désactivé. { $n } rapport en attente supprimé.
        [many] Désactivé. { $n } de rapports en attente supprimés.
       *[other] Désactivé. { $n } rapports en attente supprimés.
    }

# Tip of the button that opens the community page.
settings-crash-about-community = À propos de la communauté

# Heading of the line with the community button.
settings-crash-go-to-bm = Les rapports vont à Buildmorphic

# Under the Reports go to Buildmorphic heading.
settings-crash-community = La communauté qui aide à construire Trindle.

# Placeholder of the box for the Buildmorphic username.
settings-crash-username = nom d'utilisateur

# Heading of the line with the username box.
settings-crash-your-username = Votre nom d'utilisateur Buildmorphic

# Under the username heading.
settings-crash-username-sub = Facultatif. Jamais une adresse e-mail.

# Heading of a line about joining with an email address.
settings-crash-join = Rejoindre avec seulement une adresse e-mail

# Under the Join heading.
settings-crash-join-sub = Gagnez du crédit et des niveaux pour les problèmes que vous signalez en premier.

# Heading of a line saying a report holds only a fixed list.
settings-crash-fixed = Un rapport ne contient qu'une liste fixe

# Under the fixed list heading.
settings-crash-fixed-sub = Vous le voyez d'abord, et rien ne part sans votre appui.

# Heading of the card for reports the person sends.
settings-crash-reports-you-send = Rapports que vous envoyez

# Help text of the Reports you send card (the information button).
settings-crash-reports-about = Quand un programme de Trindle plante, une fenêtre peut s'ouvrir pour demander s'il faut envoyer un rapport. Elle affiche d'abord le texte exact : vos mots, le modèle de l'ordinateur et ses graphismes, l'erreur et le journal juste avant. Les fichiers, les mots de passe et les noms de Wi-Fi ne sont jamais lus, et les noms et adresses sont masqués. Vous pouvez aussi ouvrir cette même fenêtre à tout moment avec le bouton « Signaler un problème » de l'anneau.

# Spoken name of the box where the GitHub name is typed.
settings-crash-github-name = nom github

# Heading of the line where the person can give their GitHub name.
settings-crash-github-line = Votre nom GitHub

# Small words under that heading.
settings-crash-github-about = Facultatif. Affiché sous « Signalé par ».

