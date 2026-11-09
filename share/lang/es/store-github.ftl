
# machine translated (Spanish), not yet reviewed by a person.

# SAFETY: needs a human review before it ships
# Said wherever the page offers a file from GitHub to install.
store-gh-unchecked = Descargado de forma segura, pero nadie ha comprobado este archivo: su autor no publica ninguna firma.

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
store-gh-keyis-nokeyring = Este equipo no tiene llavero, así que no se puede guardar ninguna clave

# SAFETY: needs a human review before it ships
# Red words: the key is still in a plain file; why it was not moved follows.
store-gh-keyis-plain = Tu clave de GitHub sigue en un archivo de texto sin cifrar en este equipo y se usa desde ahí: no se pudo pasar al llavero. { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } Por eso se consulta a GitHub sin clave, haya o no una guardada.

# SAFETY: needs a human review before it ships
# Red words: a stored key has a space or line break in it.
store-gh-keyis-unsendable = Hay una clave de GitHub guardada, pero no se puede enviar: tiene un espacio o un salto de línea dentro, como ocurre con una clave pegada en dos partes. Por eso se consulta a GitHub sin clave. Elimínala y pega la clave de nuevo.

# SAFETY: needs a human review before it ships
# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = No se pudo eliminar el archivo.

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = El llavero no la aceptó.

# SAFETY: needs a human review before it ships
# Red words when a pasted key has a space or line break in it.
store-gh-key-spaces = Una clave es una sola línea sin espacios, y esta tiene un salto de línea o un espacio dentro. Pega solo la clave.

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub ha dejado de responder por ahora: { $which }. { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub ha dejado de responder por ahora: { $which }. { $when } { $hint }

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
store-gh-limit-search-key = responde a 30 búsquedas por minuto

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = responde a 10 búsquedas por minuto

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = responde a 5000 consultas por hora con tu clave, y se han agotado

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = responde a 60 consultas por hora desde una misma dirección sin clave

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = Una clave de GitHub lo eleva a 5000: pulsa la llave de arriba a la derecha en Buscar en GitHub.

# SAFETY: needs a human review before it ships
# Red words when GitHub does not accept the key.
store-gh-key-refused = GitHub no aceptó la clave. Crea una nueva o elimínala: pulsa la llave de arriba a la derecha en Buscar en GitHub.

# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub respondió { $code }.

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = GitHub rechazó esto, aunque no ha agotado sus respuestas.

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub rechazó esto: { $why }.

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [one] No se pudo examinar 1 proyecto encontrado, así que no aparece en la lista
        [many] No se pudieron examinar { $n } de proyectos encontrados, así que no aparecen en la lista
       *[other] No se pudieron examinar { $n } proyectos encontrados, así que no aparecen en la lista
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }.

# Red words when GitHub could not be reached.
store-gh-unreached = No se pudo conectar con GitHub.

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = Solo se buscaron proyectos con un AppImage: GitHub no respondió a la búsqueda de Flatpak. { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = Descargando

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = Instalando

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = La descarga no terminó. { $why }

# The size of a download that is under a megabyte.
store-size-under = menos de 1 MB

