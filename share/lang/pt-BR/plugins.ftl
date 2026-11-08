
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Red words when plugin.ini cannot be read at the top of a plugin's folder. { $file } is the file's name, plugin.ini, and { $why } is the reason it could not be read.
plugins-check-no-manifest = Não há um { $file } legível no topo da pasta ({ $why }).

# Red words when plugin.ini has no [Plugin] part. { $file } is the file's name, plugin.ini.
plugins-check-no-plugin-part = { $file } não tem a seção [Plugin].

# Red words when plugin.ini is for another interface number. { $n } is the number it names; { $known } is the one this Trindle knows.
plugins-check-interface-other = Interface={ $n } não é uma que este Trindle conhece: ele conhece Interface={ $known }.

# Red words when plugin.ini names no interface number. { $known } is the one this Trindle knows.
plugins-check-interface-missing = [Plugin] precisa de Interface={ $known }.

# Red words when plugin.ini has no usable Id.
plugins-check-id = [Plugin] precisa de um Id com letras, dígitos, pontos e hifens, como org.example.Weather.

# Red words when plugin.ini has no Name.
plugins-check-needs-name = [Plugin] precisa de um Name.

# Red words when plugin.ini has no Version.
plugins-check-needs-version = [Plugin] precisa de uma Version.

# Red words when plugin.ini has a Summary that runs over more than one line.
plugins-check-summary-one-line = O Summary de [Plugin] tem uma só linha.

# Red words when a value of plugin.ini must be plain words on one line. { $group } is the part, { $key } the key and { $max } the most letters allowed.
plugins-plain-line =
    { $max ->
        [one] [{ $group }] { $key } deve ser texto simples em uma linha, com no máximo { $max } caractere.
        [many] [{ $group }] { $key } deve ser texto simples em uma linha, com no máximo { $max } de caracteres.
       *[other] [{ $group }] { $key } deve ser texto simples em uma linha, com no máximo { $max } caracteres.
    }

# Red words when plugin.ini asks for a permission that does not exist. { $name } is what it asked for; { $list } is the permissions there are, with commas between.
plugins-check-permission-unknown = A permissão { $name } não existe: as que existem são { $list }.

# Red words when the [Ring] part has no usable Icon.
plugins-ring-needs-icon = [Ring] precisa de um Icon: um arquivo SVG ou PNG na pasta do plugin.

# Red words when the [Ring] Icon is not a file in the plugin's folder. { $icon } is the file name it gave.
plugins-ring-icon-missing = O Icon de [Ring] indica { $icon }, que não está na pasta do plugin.

# Red words when the [Ring] part has no Label.
plugins-ring-needs-label = [Ring] precisa de um Label, dito quando o ponteiro fica sobre ele.

# Red words when the [Ring] part has no Run.
plugins-ring-needs-run = [Ring] precisa de Run, o que um clique inicia.

# Red words when the [Settings] part names no fields.
plugins-settings-needs-fields = [Settings] precisa de Fields, os nomes de seus campos.

# Red words when the name of a settings field has marks that are not allowed. { $name } is the name.
plugins-field-bad-name = O nome de campo { $name } só pode ter letras, dígitos, hifens e sublinhados.

# Red words when a settings field is named and has no part of its own. { $name } is the field; { $group } is the part it should have.
plugins-field-no-part = O campo { $name } não tem a seção [{ $group }].

# Red words when a part of plugin.ini has no Label. { $group } is the part.
plugins-needs-label = [{ $group }] precisa de um Label.

# Red words when a choice field has too many choices or choices that are too long. { $group } is the part, { $most } the most choices and { $chars } the most letters in one.
plugins-choices-limits =
    { $chars ->
        [one] [{ $group }] As opções são no máximo { $most }, cada uma em texto simples de no máximo { $chars } caractere.
        [many] [{ $group }] As opções são no máximo { $most }, cada uma em texto simples de no máximo { $chars } de caracteres.
       *[other] [{ $group }] As opções são no máximo { $most }, cada uma em texto simples de no máximo { $chars } caracteres.
    }

# Red words when ChoicesFrom is not a good file name. { $group } is the part.
plugins-choices-from-name = [{ $group }] ChoicesFrom é um nome de arquivo na pasta do plugin: letras, dígitos, pontos, hifens e sublinhados, sem começar com ponto e sem pastas.

# Red words when a choice field offers fewer than two choices. { $group } is the part.
plugins-choice-needs-choices = [{ $group }] é uma escolha, então precisa de Choices com duas ou mais opções, ou de ChoicesFrom com um arquivo.

# Red words when the default of a choice field is not one of its choices. { $group } is the part.
plugins-choice-default = O Default de [{ $group }] não é uma de suas Choices.

# Red words when a note field names no good file to read. { $group } is the part.
plugins-note-needs-textfrom = [{ $group }] é uma nota, então precisa de TextFrom, um nome de arquivo na pasta do plugin: letras, dígitos, pontos, hifens e sublinhados, sem começar com ponto e sem pastas.

# Red words when a note field has a Default. { $group } is the part.
plugins-note-default = [{ $group }] é uma nota, que não guarda configuração, então não tem Default.

# Red words when a field's Kind is none of those there are. { $group } is the part; { $kind } is what it gave, in quotation marks.
plugins-field-kind = O Kind de [{ $group }] é { $kind }: deve ser switch, text, choice ou note.

# Red words when a switch field has a Default that is not true or false. { $group } is the part.
plugins-switch-default = [{ $group }] é um interruptor, então seu Default é true ou false.

# Red words when the [Background] part is given without the background permission.
plugins-background-needs-permission = [Background] precisa da permissão background em Permissions, na seção [Plugin].

# Red words when a plugin has nothing that makes it do anything.
plugins-hooks-nothing = O plugin não se liga a nada: dê a ele [Ring], [Settings], [Background], [Shortcut] ou a permissão de notificações.

# Red words when a file inside a plugin is a link that leads out of its folder. { $path } is the link's place inside the folder.
plugins-links-outside = { $path } aponta para fora da pasta do plugin.

# Red words when a Run line cannot be read as a command. { $group } is the part.
plugins-run-unreadable = O Run de [{ $group }] não pode ser lido como um comando.

# Red words when a Run line names a program outside the plugin's folder. { $group } is the part; { $program } is the program named.
plugins-run-outside = O Run de [{ $group }] deve indicar um programa dentro da pasta do plugin, não { $program }.

# Red words when a Run line names a program that is not there or cannot run. { $group } is the part; { $program } is the program named.
plugins-run-missing = O Run de [{ $group }] indica { $program }, que não é um programa do plugin (ausente ou não executável).

# Red words when the [Ring] Menu lists too many items. { $n } is how many it lists; { $max } is the most allowed.
plugins-menu-too-many =
    { $n ->
        [one] O Menu de [Ring] tem { $n } item: no máximo { $max } são permitidos.
        [many] O Menu de [Ring] tem { $n } de itens: no máximo { $max } são permitidos.
       *[other] O Menu de [Ring] tem { $n } itens: no máximo { $max } são permitidos.
    }

# Red words when a menu item's id has marks that are not allowed. { $id } is the id.
plugins-menu-bad-id = O id de menu { $id } só pode ter letras, dígitos, hifens e sublinhados.

# Red words when the same menu item is named twice. { $id } is the id.
plugins-menu-twice = O Menu de [Ring] cita { $id } duas vezes.

# Red words when a menu item is named and has no part of its own. { $id } is the item; { $group } is the part it should have.
plugins-menu-no-part = O Menu de [Ring] cita { $id }, que não tem a seção [{ $group }].

# Red words when a menu item or shortcut has a Label that is not one short plain line. { $group } is the part; { $max } is the most letters.
plugins-label-short-line =
    { $max ->
        [one] O Label de [{ $group }] é uma linha curta de no máximo { $max } caractere.
        [many] O Label de [{ $group }] é uma linha curta de no máximo { $max } de caracteres.
       *[other] O Label de [{ $group }] é uma linha curta de no máximo { $max } caracteres.
    }

# Red words when a menu item has no Run. { $group } is the part.
plugins-menu-needs-run = [{ $group }] precisa de Run, o que escolhê-lo inicia.

# Red words when a plugin other than Flow has a PinUnlock part. { $group } is the part's name, PinUnlock.
plugins-pin-group-only-flow = [{ $group }] é só para o Flow do catálogo do Trindle, que pede a permissão de controle.

# Red words when the PinUnlock part has no Run. { $group } is the part's name, PinUnlock.
plugins-pin-group-needs-run = [{ $group }] precisa de Run, o comando que recebe o PIN.

# Red words when a ShowWhen is too long. { $group } is the part; { $max } is the most letters.
plugins-showwhen-long =
    { $max ->
        [one] O ShowWhen de [{ $group }] tem no máximo { $max } caractere.
        [many] O ShowWhen de [{ $group }] tem no máximo { $max } de caracteres.
       *[other] O ShowWhen de [{ $group }] tem no máximo { $max } caracteres.
    }

# Red words when a ShowWhen is not written field=value. { $group } is the part.
plugins-showwhen-form = O ShowWhen de [{ $group }] é escrito campo=valor, como ShowWhen=mode=home: um outro campo e o único valor que ele deve ter.

# Red words when the field a ShowWhen names has marks in its name that are not allowed. { $group } is the part.
plugins-showwhen-field-name = O ShowWhen de [{ $group }] é escrito campo=valor, e o nome do campo só pode ter letras, dígitos, hifens e sublinhados.

# Red words when the value a ShowWhen needs is missing or not plain. { $group } is the part; { $max } is the most letters.
plugins-showwhen-value =
    { $max ->
        [one] O ShowWhen de [{ $group }] precisa de um valor em texto simples em uma linha, com no máximo { $max } caractere.
        [many] O ShowWhen de [{ $group }] precisa de um valor em texto simples em uma linha, com no máximo { $max } de caracteres.
       *[other] O ShowWhen de [{ $group }] precisa de um valor em texto simples em uma linha, com no máximo { $max } caracteres.
    }

# Red words when a ShowWhen names the field it is in. { $group } is the part.
plugins-showwhen-own = O ShowWhen de [{ $group }] cita o próprio campo: deve citar outro campo, anterior.

# Red words when a ShowWhen names a field that is not there. { $group } is the part; { $other } is the field named.
plugins-showwhen-unknown = O ShowWhen de [{ $group }] cita o campo { $other }, que não está nos Fields deste plugin.

# Red words when a ShowWhen names a field that comes later. { $group } is the part; { $other } is the field named.
plugins-showwhen-later = O ShowWhen de [{ $group }] cita o campo { $other }, que vem depois dele em Fields: deve ser um campo anterior.

# Red words when a ShowWhen names a field that has a ShowWhen too. { $group } is the part; { $other } is the field named.
plugins-showwhen-chained = O ShowWhen de [{ $group }] cita o campo { $other }, que tem um ShowWhen próprio: um campo só pode depender de outro que não tenha.

# Red words when a ShowWhen names a switch and a value that is not true or false. { $group } is the part; { $other } is the switch.
plugins-showwhen-switch = O ShowWhen de [{ $group }] cita o interruptor { $other }, então seu valor é true ou false.

# Red words when a ShowWhen gives a value that the choice field it names does not offer. { $group } is the part; { $value } is the value, in quotation marks; { $other } is the field.
plugins-showwhen-not-choice = O ShowWhen de [{ $group }] tem o valor { $value }, que não é uma das Choices de { $other }.

# Red words when a ShowWhen names a note field. { $group } is the part; { $other } is the note.
plugins-showwhen-note = O ShowWhen de [{ $group }] cita a nota { $other }, que não guarda configuração: um campo só pode depender de um interruptor, uma escolha ou um texto.

# Red words when the ports a plugin asks to listen on are not good. { $why } says what is wrong with them.
plugins-listen-fault = [Rede] Portas de escuta: { $why }

# Red words when a plugin names ports to listen on without the network permission.
plugins-listen-needs-network = [Network] Listen precisa da permissão network em Permissions, na seção [Plugin].

# Red words when a plugin names ports to listen on without the listen permission.
plugins-listen-needs-listen = [Network] Listen precisa da permissão listen em Permissions, na seção [Plugin].

# Red words when a plugin has the listen permission and names no ports.
plugins-listen-needs-ports = A permissão listen precisa de [Network] Listen, as portas a liberar.

# Red words when the ports to open for a plugin are asked of a plugin.ini for another interface.
plugins-ports-interface = Seu plugin.ini não é de uma interface que este Trindle conhece.

# Red words when the ports to open for a plugin are asked and it names none.
plugins-ports-none = Ele não cita nenhuma porta para escutar.

# Red words when the ports to open for a plugin are asked and its plugin.ini cannot be read.
plugins-ports-unreadable = Não foi possível ler o plugin.ini dele.

# Red words when the ports to open for a plugin are asked and it has not been allowed to listen.
# SAFETY: needs a human review before it ships
plugins-ports-not-allowed = Ele não foi autorizado a escutar.

# Red words when the ports a plugin names are not the ones the person was shown when allowing it to listen.
# SAFETY: needs a human review before it ships
plugins-ports-changed = As portas que ele cita não são as que foram autorizadas. É preciso autorizá-las de novo.

# Headline on a plugin that has been allowed none of what it asks for. { $name } is the plugin; { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-none = { $name } precisa da sua permissão para funcionar: 0 de { $n } permitidas.

# Headline on a plugin that has been allowed some of what it asks for. { $name } is the plugin; { $a } is how many are allowed; { $n } is how many it asks for.
# SAFETY: needs a human review before it ships
plugins-needs-some = { $name } tem só parte da permissão que pede: { $a } de { $n } permitidas.

# Quiet line on a plugin that has been allowed all it asks for. { $n } is how many things it asks for.
# SAFETY: needs a human review before it ships
plugins-settled =
    { $n ->
        [0] Ele não pede nada.
        [one] A única coisa que ele pede está permitida.
        [many] Todas as { $n } coisas que ele pede estão permitidas.
       *[other] Todas as { $n } coisas que ele pede estão permitidas.
    }

# Button that opens the window where a plugin's permissions are allowed.
# SAFETY: needs a human review before it ships
plugins-allow = Permitir

# Said among what a newer version of a plugin asks for that the old one did not: other ports to be reached on. It follows words such as "It asks for more than before:".
plugins-other-ports = ser acessado em outras portas além das de antes

# Red words when the ports of a plugin cannot be opened because another plugin that holds them is no longer installed.
plugins-not-installed = Não instalado.

# Name used for a plugin that cannot be found, in the words about who holds a port.
plugins-another-plugin = outro plugin

# Red words when a plugin that is not installed is asked to open its ports.
plugins-that-not-installed = Esse plugin não está instalado.

# Red words when a permission could not be kept. { $why } is the reason.
plugins-not-saved = Não foi possível salvar: { $why }

# Red words when a plugin's files could not be copied for an install.
plugins-copy-failed = Não foi possível copiar os arquivos do plugin.

# Red words when an update was not put in and the plugin's background part is not running either. { $why } is the first reason; { $not } is why the background part is not running.
plugins-swap-not-running = { $why } A parte em segundo plano dele não está em execução: { $not }

# Words after a colon: a plugin's background part cannot be started because trindle-plugin cannot be found.
plugins-bg-no-program = não foi possível encontrar o trindle-plugin para iniciá-la.

# Words after a colon: a plugin's background part did not start in time. { $n } is how many seconds it was given.
plugins-bg-slow =
    { $n ->
        [one] ela não iniciou em { $n } segundo.
        [many] ela não iniciou em { $n } de segundos.
       *[other] ela não iniciou em { $n } segundos.
    }

# Words after a colon: a plugin's background part was not started because no desktop session is running.
plugins-bg-no-session = não há sessão de área de trabalho em execução para ela se juntar, então não foi iniciada. Entre na sessão primeiro.

# Words after a colon: a plugin's background part was not started because there are several desktop sessions. { $sessions } lists them, with commas between.
plugins-bg-which-session = não dá para saber a qual sessão de área de trabalho ela deve se juntar ({ $sessions }), então não foi iniciada. Execute de novo com WAYLAND_DISPLAY definido para a certa.

# Error text when a folder is where a plugin's file should be.
plugins-folder-in-the-way = há uma pasta onde deveria estar o arquivo

# Error text when a plugin's id is not an id.
plugins-not-an-id = não é o id de um plugin

# Red words when a plugin asks for more shortcuts than are allowed. { $n } is how many it asks for; { $max } is the most allowed.
plugins-shortcut-too-many =
    { $n ->
        [one] Shortcuts de [Plugin] tem { $n } atalho: no máximo { $max } são permitidos.
        [many] Shortcuts de [Plugin] tem { $n } de atalhos: no máximo { $max } são permitidos.
       *[other] Shortcuts de [Plugin] tem { $n } atalhos: no máximo { $max } são permitidos.
    }

# Red words when a shortcut's id has marks that are not allowed. { $id } is the id.
plugins-shortcut-bad-id = O id de atalho { $id } só pode ter letras, dígitos, hifens e sublinhados.

# Red words when the same shortcut is named twice. { $id } is the id.
plugins-shortcut-twice = Shortcuts de [Plugin] cita { $id } duas vezes.

# Red words when a shortcut is named and has no part of its own. { $id } is the shortcut; { $group } is the part it should have.
plugins-shortcut-no-part = Shortcuts de [Plugin] cita { $id }, que não tem a seção [{ $group }].

# Red words when the keys a plugin asks for are already one of Trindle's shortcuts. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-trindle = Em [{ $group }], as Keys { $keys } já são um atalho do Trindle; escolha outras teclas.

# Red words when the keys a plugin asks for are used by another shortcut of the same plugin. { $group } is the part; { $keys } are the keys.
plugins-shortcut-keys-twice = Em [{ $group }], as Keys { $keys } são usadas por outro atalho do plugin.

# Red words when the keys a plugin asks for cannot be read as keys. { $group } is the part; { $why } says what is wrong with them.
plugins-shortcut-keys-needed = [{ $group }] precisa de Keys, como Ctrl+Alt+S. { $why }

# Red words when a shortcut has no Run. { $group } is the part.
plugins-shortcut-needs-run = [{ $group }] precisa de Run, o que as teclas iniciam.

# Red words beside a shortcut in Settings when the keys the person typed are not a shortcut. { $why } says what is wrong with them.
plugins-shortcut-typed-bad = As teclas digitadas não formam um atalho. { $why }

# Red words in Settings when the keys typed are already one of Trindle's shortcuts. { $keys } are the keys.
plugins-shortcut-trindle = { $keys } já é um atalho do Trindle.

# Red words in Settings when the keys typed are already one of the person's own shortcuts. { $keys } are the keys.
plugins-shortcut-yours = { $keys } já é um dos seus próprios atalhos.

# Red words in Settings when the keys typed are already used by another plugin's shortcut. { $keys } are the keys; { $holder } is the plugin and its shortcut.
plugins-shortcut-used = { $keys } já é usado por { $holder }.

# Red words in Settings when a shortcut that is not an installed plugin's is changed.
plugins-shortcut-unknown = Esse não é um atalho de um plugin instalado.

# Red words in Settings when a shortcut was kept but could not be put to use. { $why } is the reason.
plugins-shortcut-not-in-use = Não foi possível colocá-lo em uso ({ $why }).

# Red words in Settings when the file of shortcut choices cannot be read, so nothing was changed. { $why } is the reason.
plugins-shortcut-not-saved-unchanged = Não foi possível salvar: nada foi alterado, porque { $why }.

# Red words in Settings when the file of shortcut choices could not be written. { $why } is the reason.
plugins-shortcut-not-saved = Não foi possível salvar ({ $why }).



# Red words when a plugin other than Trindle's screen reader asks for the accessibility permission.
plugins-accessibility-only-reader = A permissão de acessibilidade é só para o leitor de tela do Trindle que está no catálogo.

