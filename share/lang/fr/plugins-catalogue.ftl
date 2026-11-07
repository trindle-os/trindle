
# machine translated (French), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = Les mises à jour des modules sont suspendues : la liste des modules est endommagée.

# SAFETY: needs a human review before it ships
# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
plugins-catalogue-start-again = Recommencer

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = L'enregistrement n'a pas pu être retiré : { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = Un nom contenait un caractère nul.

# Error text when there is no record name to take away.
plugins-start-no-note = Il n'y a aucune note à retirer.

# SAFETY: needs a human review before it ships
# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
plugins-refused-no-key = Ce Trindle n'a aucune clé pour vérifier son catalogue de modules, il n'en accepte donc aucun.

# SAFETY: needs a human review before it ships
# Red line under Trindle's own plugins when the catalogue came with no signature.
plugins-refused-no-signature = Le catalogue de modules de Trindle n'a pas de signature, il n'a donc pas été utilisé.

# SAFETY: needs a human review before it ships
# Red line under Trindle's own plugins when the signature is the wrong size.
plugins-refused-wrong-size = La signature du catalogue de modules de Trindle n'a pas la taille d'une signature, le catalogue n'a donc pas été utilisé.

# SAFETY: needs a human review before it ships
# Red line under Trindle's own plugins when the signature is not Trindle's.
plugins-refused-not-signed = Le catalogue de modules de Trindle n'est pas signé avec la clé de Trindle, il n'a donc pas été utilisé.

# SAFETY: needs a human review before it ships
# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = Le catalogue de modules de Trindle n'indique pas son numéro, il n'a donc pas été utilisé.

# SAFETY: needs a human review before it ships
# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
plugins-refused-older = Le catalogue de modules de Trindle est plus ancien que celui accepté auparavant (numéro { $is }, après { $after }), il n'a donc pas été utilisé.

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = Le catalogue de modules de Trindle est signé mais illisible, il n'a donc pas été utilisé : { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } Le dernier bon catalogue est affiché.

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = Ce n'est pas du texte.

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = Le catalogue est trop volumineux pour en être un.

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = Ce n'est pas un catalogue de modules : { $file } n'a pas de section [Catalogue] avec son Format.

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = Le catalogue est d'un type plus récent que celui que ce Trindle sait lire (Format={ $format }).

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } est ignoré : { $why ->
        [id] son identifiant n'est pas composé de lettres, de chiffres, de points et de tirets
        [needs] il lui faut un Name et une Version
        [archive] son Archive n'est pas le nom d'un fichier .tar.zst placé à côté du catalogue
        [hash] son SHA256 n'est pas composé de 64 chiffres hexadécimaux
        [permission] il demande une autorisation qui n'existe pas
       *[icon] son Icon n'est pas le nom d'un fichier SVG ou PNG placé à côté du catalogue
    }.

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar n'a pas pu être lancé : { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = L'archive n'a pas pu être décompressée : { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = tar l'a refusée

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = L'archive ne contient pas de { $file } à sa racine, ni dans l'unique dossier à sa racine.

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Aucun dossier temporaire n'a pu être créé : { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl n'a pas pu être lancé : { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = Le téléchargement a échoué : { $why }

# SAFETY: needs a human review before it ships
# Red words when a downloaded plugin is not the file the catalogue lists.
plugins-install-hash = Son fichier n'est pas celui que liste le catalogue (le SHA-256 diffère), rien n'a donc été installé.

# SAFETY: needs a human review before it ships
# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
plugins-install-check = Il ne passe pas la vérification des modules de Trindle, rien n'a donc été installé : { $why }

# SAFETY: needs a human review before it ships
# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = L'archive contient { $found } et non { $wanted }, rien n'a donc été installé.

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Il a été installé, mais la note indiquant qu'il vient du catalogue de Trindle n'a pas pu être écrite : { $why }.

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = Il n'est pas installé, il n'y a donc rien à mettre à jour.
