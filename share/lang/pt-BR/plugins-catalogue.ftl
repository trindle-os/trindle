# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Red words on the Plugins page in Apps when updates are paused because the record of the highest catalogue number accepted is damaged. The Start again button goes beside it.
plugins-catalogue-paused = As atualizações de plugins estão pausadas porque a lista de plugins está danificada.

# Button beside the pause words in Apps that clears the damaged record. It asks for the administrator's password.
# SAFETY: needs a human review before it ships
plugins-catalogue-start-again = Recomeçar

# Red words when the record could not be taken away. { $why } is the reason.
plugins-start-not-taken = O Trindle não conseguiu remover o registro: { $why }

# Error text when a name given to take the record away had a nul in it.
plugins-start-nul = Um nome tem um byte nulo.

# Error text when there is no record name to take away.
plugins-start-no-note = Não há nenhum registro para remover.

# Red line under Trindle's own plugins when this Trindle has no key to check the catalogue with.
# SAFETY: needs a human review before it ships
plugins-refused-no-key = O Trindle não pode verificar o catálogo de plugins porque a chave de verificação está faltando.

# Red line under Trindle's own plugins when the catalogue came with no signature.
# SAFETY: needs a human review before it ships
plugins-refused-no-signature = O catálogo de plugins do Trindle não tem assinatura, então o Trindle não o usou.

# Red line under Trindle's own plugins when the signature is the wrong size.
# SAFETY: needs a human review before it ships
plugins-refused-wrong-size = A assinatura do catálogo de plugins do Trindle tem o tamanho errado, então o Trindle não usou o catálogo.

# Red line under Trindle's own plugins when the signature is not Trindle's.
# SAFETY: needs a human review before it ships
plugins-refused-not-signed = O catálogo de plugins do Trindle não está assinado com a chave do Trindle, então o Trindle não o usou.

# Red line under Trindle's own plugins when the catalogue does not say which number it is.
# SAFETY: needs a human review before it ships
plugins-refused-no-number = O catálogo de plugins do Trindle não tem número de versão, então o Trindle não o usou.

# Red line under Trindle's own plugins when the catalogue is older than one accepted before.
# SAFETY: needs a human review before it ships
plugins-refused-older = O Trindle não usou este catálogo de plugins porque ele é mais antigo que um catálogo usado antes.

# Red line under Trindle's own plugins when the catalogue is signed but cannot be read. { $why } says what is wrong with it.
plugins-refused-unreadable = O catálogo de plugins do Trindle está assinado, mas não pode ser lido, então o Trindle não o usou: { $why }

# Red line under Trindle's own plugins: why the catalogue was not used, then that an earlier one is still shown. { $why } is the sentence saying why.
plugins-catalogue-last-good = { $why } O último catálogo válido é mostrado.

# Why a signed catalogue cannot be read: it is not text.
plugins-catalogue-not-text = Não é texto.

# Why a downloaded catalogue cannot be used: it is too big to be one.
plugins-catalogue-too-big = O catálogo é grande demais.

# Why a file cannot be read as a catalogue. { $file } is the catalogue's file name.
plugins-catalogue-not-one = Não é um catálogo de plugins, porque { $file } não tem uma parte [Catalogue] com um Format.

# Why a catalogue cannot be read: it is of a newer kind. { $format } is the number of the kind this Trindle reads.
plugins-catalogue-newer = Este catálogo usa um formato mais novo do que esta versão do Trindle consegue ler. O Trindle aceita Format={ $format }.

# Why one plugin of the catalogue is left out. { $id } is the plugin; { $why } says which part is wrong: id, needs, archive, hash, permission or icon.
plugins-catalogue-left-out =
    { $id } foi deixado de fora: { $why ->
        [id] o id não é composto só de letras, números, pontos e hifens
        [needs] ele precisa de um Name e de uma Version
        [archive] o Archive não é o nome de um arquivo .tar.zst ao lado do catálogo
        [hash] o SHA256 não tem 64 dígitos hexadecimais
        [permission] ele pede uma permissão que não existe
       *[icon] o Icon não é o nome de um arquivo SVG ou PNG ao lado do catálogo
    }.

# Red words when the program that unpacks archives could not be started. { $why } is the reason.
plugins-unpack-tar = O tar não iniciou: { $why }

# Red words when an archive could not be unpacked. { $why } is the first line the unpacking program said.
plugins-unpack-failed = O Trindle não conseguiu extrair o arquivo: { $why }

# Said in place of the unpacking program's own line when it said nothing.
plugins-unpack-tar-refused = motivo desconhecido

# Red words when an archive holds no plugin.ini. { $file } is the file's name, plugin.ini.
plugins-unpack-no-manifest = O arquivo não tem { $file } no nível principal, nem na única pasta do nível principal.

# Red words when a scratch folder for an install could not be made. { $why } is the reason.
plugins-scratch = O Trindle não conseguiu criar uma pasta temporária: { $why }

# Red words when the program that downloads could not be started. { $why } is the reason.
plugins-download-curl = O curl não iniciou: { $why }

# Red words when a download failed. { $why } is what the downloading program said.
plugins-download-failed = Não foi possível baixar: { $why }

# Red words when a downloaded plugin is not the file the catalogue lists.
# SAFETY: needs a human review before it ships
plugins-install-hash = O arquivo baixado não corresponde à soma de verificação (SHA-256) do catálogo, então o Trindle não instalou nada.

# Red words when a downloaded plugin fails the check of its plugin.ini. { $why } is the first thing wrong.
# SAFETY: needs a human review before it ships
plugins-install-check = O Trindle não instalou nada porque o plugin não passou nas verificações: { $why }

# Red words when a downloaded archive holds another plugin than the one asked for. { $found } is the plugin in it; { $wanted } is the one asked for.
# SAFETY: needs a human review before it ships
plugins-install-other-id = O arquivo contém { $found }, não { $wanted }, então o Trindle não instalou nada.

# Red words when a plugin was put in but the note that it came from the catalogue could not be written. { $why } is the reason.
plugins-install-note = O Trindle instalou o plugin, mas não conseguiu registrar que ele veio do catálogo do Trindle: { $why }.

# Red words when an update is asked for a plugin that is not installed.
plugins-update-not-installed = Ele não está instalado, então não há nada para atualizar.
