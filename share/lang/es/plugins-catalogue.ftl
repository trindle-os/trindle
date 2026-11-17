# machine translated (Spanish), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = Las actualizaciones de complementos están en pausa: la lista de complementos está dañada.

# SAFETY: needs a human review before it ships
# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
plugins-catalogue-start-again = Empezar de nuevo

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = Trindle no pudo quitar el registro: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = Un nombre contiene un byte nulo.

# Error text when there is no record name to take away.
plugins-start-no-note = No hay ningún registro que quitar.

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
plugins-refused-no-key = Trindle no puede comprobar el catálogo de complementos porque falta su clave de verificación.

# Red line under Trindle's own plugins when the catalogue came with no signature.
plugins-refused-no-signature = El catálogo de complementos de Trindle no tiene firma, así que Trindle no lo usó.

# Red line under Trindle's own plugins when the signature is the wrong size.
plugins-refused-wrong-size = La firma del catálogo de complementos de Trindle tiene un tamaño incorrecto, así que Trindle no usó el catálogo.

# Red line under Trindle's own plugins when the signature is not Trindle's.
plugins-refused-not-signed = El catálogo de complementos de Trindle no está firmado con la clave de Trindle, así que Trindle no lo usó.

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
plugins-refused-no-number = El catálogo de complementos de Trindle no tiene número de versión, así que Trindle no lo usó.

# Red line under Trindle's own plugins when the catalogue is older than one accepted before. { $is } is its number; { $after } is the number accepted before.
plugins-refused-older = Trindle no usó este catálogo de complementos porque su número de versión, { $is }, es anterior al aceptado antes, { $after }.

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = El catálogo de complementos de Trindle está firmado pero no se puede leer, así que Trindle no lo usó: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } Se muestra el último que era válido.

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = No es texto.

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = El catálogo es demasiado grande.

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = No es un catálogo de complementos, porque { $file } no tiene una parte [Catalogue] con un Format.

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = Este catálogo usa un formato más reciente de lo que esta versión de Trindle puede leer. Trindle admite Format={ $format }.

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    Se omite { $id }: { $why ->
        [id] su id no está formado solo por letras, dígitos, puntos y guiones
        [needs] necesita un Name y una Version
        [archive] su Archive no es el nombre de un archivo .tar.zst junto al catálogo
        [hash] su SHA256 no tiene 64 dígitos hexadecimales
        [permission] pide un permiso que no existe
       *[icon] su Icon no es el nombre de un archivo SVG o PNG junto al catálogo
    }.

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = tar no se inició: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = Trindle no pudo descomprimir el archivo: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = motivo desconocido

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = El archivo no tiene { $file } en su nivel superior, ni en la única carpeta de su nivel superior.

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = Trindle no pudo crear una carpeta temporal: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = curl no se inició: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = No se pudo descargar: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
plugins-install-hash = El archivo descargado no coincide con la suma de comprobación del catálogo (SHA-256), así que Trindle no instaló nada.

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
plugins-install-check = Trindle no instaló nada porque el complemento no superó sus comprobaciones: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
plugins-install-other-id = El archivo contiene { $found }, no { $wanted }, así que Trindle no instaló nada.

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = Trindle instaló el complemento, pero no pudo registrar que procede del catálogo de Trindle: { $why }.

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = No está instalado, así que no hay nada que actualizar.

