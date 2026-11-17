
# machine translated (Spanish), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Said wherever the page offers a file from GitHub to install.
store-gh-unchecked = Este archivo se descargó de forma segura, pero nadie lo ha comprobado, porque su autor no publica ninguna firma.

# What a project with nothing to install here is told as.
store-gh-no-files = Las versiones de este proyecto no tienen ningún archivo AppImage ni Flatpak para este equipo.

# What a project that has never made a release is told as.
store-gh-no-releases = GitHub no tiene versiones de este proyecto.

# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = Buscando una clave en el llavero

# SAFETY: needs a human review before it ships
# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = Hay una clave guardada

# SAFETY: needs a human review before it ships
# In the GitHub key's box: no key is stored.
store-gh-keyis-none = No hay ninguna clave guardada

# SAFETY: needs a human review before it ships
# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = Este equipo no tiene llavero, así que no puede guardar una clave

# Red words: the key is still in a plain file; why it was not moved follows.
# SAFETY: needs a human review before it ships
store-gh-keyis-plain = Tu clave de GitHub sigue en un archivo de texto antiguo en este equipo, que Trindle ya no lee. { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } Por eso Trindle usa GitHub sin clave, aunque haya una guardada.

# SAFETY: needs a human review before it ships
# Red words: a stored key has a space or line break in it.
store-gh-keyis-unsendable = Trindle no puede enviar tu clave de GitHub guardada, porque tiene un espacio o un salto de línea. Por eso Trindle usa GitHub sin clave. Elimina la clave y pégala de nuevo.

# SAFETY: needs a human review before it ships
# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = No se pudo eliminar el archivo.

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = El llavero no la guardó, así que Trindle usa GitHub sin clave.

# SAFETY: needs a human review before it ships
# Red words when a pasted key has a space or line break in it.
store-gh-key-spaces = Una clave es una sola línea sin espacios. Esta tiene un salto de línea o un espacio dentro. Pega solo la clave.

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub ha dejado de responder por ahora, porque { $which }. { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub ha dejado de responder por ahora, porque { $which }. { $when } { $hint }

# Part of the limit words: how long to wait is not known.
store-gh-wait-later = Inténtalo de nuevo más tarde.

# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
        [one] Volverá a responder en aproximadamente un minuto.
        [many] Volverá a responder en aproximadamente { $n } de minutos.
       *[other] Volverá a responder en aproximadamente { $n } minutos.
    }

# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = permite 30 búsquedas por minuto

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = permite 10 búsquedas por minuto

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = has agotado las 5000 consultas por hora que permite tu clave

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = permite 60 consultas por hora desde una misma dirección sin clave

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = Una clave de GitHub lo eleva a 5000. Pulsa el botón de la llave, arriba a la derecha en Buscar.

# SAFETY: needs a human review before it ships
# Red words when GitHub does not accept the key.
store-gh-key-refused = GitHub no aceptó la clave. Crea una nueva o elimina esta con el botón de la llave, arriba a la derecha en Buscar.

# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub devolvió { $code }.

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = GitHub bloqueó esto, aunque no has llegado a su límite.

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub bloqueó esto: { $why }.

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [one] Trindle no pudo examinar 1 proyecto que encontró, así que no aparece en la lista
       *[other] Trindle no pudo examinar { $n } proyectos que encontró, así que no aparecen en la lista
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }.

# Red words when GitHub could not be reached.
store-gh-unreached = No se pudo conectar con GitHub.

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = Trindle solo buscó proyectos con un AppImage, porque GitHub no respondió a la búsqueda de Flatpak. { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = Descargando

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = Instalando

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = La descarga no terminó. { $why }

# The size of a download that is under a megabyte.
store-size-under = menos de 1 MB
