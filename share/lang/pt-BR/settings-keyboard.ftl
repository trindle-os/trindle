
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = Teclado e mouse

# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = layout disposição idioma digitação caps lock teclado virtual botão anel mouse touchpad trackpad ponteiro velocidade toque clique rolagem roda mais lento mais rápido natural canhoto plugin atalho teclas remapear mudar trocar ctrl escape super alt mac botões laterais tocado segurado feito por programas keyd

# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = Tudo aqui muda na hora. Você pode alternar para um segundo layout pressionando Alt e Shift juntas.

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = Digitando em { $layout }.

# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = Alt e Shift alternam para { $layout }.

# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Caps Lock não faz nada.

# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Caps Lock é { $key }.

# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = outro Ctrl

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Backspace

# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = Este computador não tem a lista de layouts de teclado.

# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = Teclado

# Heading of the line in that card.
settings-keyboard-no-list-heading = A lista de layouts de teclado não está disponível

# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = Para instalar, execute: sudo apt install xkb-data

# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
        [one] Você mudou uma tecla ou botão.
       *[other] Você mudou { $n } teclas e botões.
    }

# Title of the card with the keyboard layout.
settings-keyboard-layout-title = Layout

# Heading of the layout line, and the name of its list.
settings-keyboard-layout = Layout

# Heading of the variant line, and the name of its list.
settings-keyboard-variant = Variante

# First choice in the list of variants: the layout without a variant.
settings-keyboard-standard = Padrão

# First choice in the list of second layouts: no second layout.
settings-keyboard-none = Nenhum

# Heading of the line, and the name of its list, for a second layout.
settings-keyboard-switch-to = Alternar para

# Words behind the i of the layout card.
settings-keyboard-layout-about = Isto define o que cada tecla digita. Você pode escolher um layout e uma variante dele. Pressione Alt e Shift juntas para alternar para um segundo layout, e de novo para voltar. As alterações valem na hora.

# Green words under a layout line once the new layout is in use.
settings-keyboard-in-use = Em uso agora

# Choice in the Caps Lock list: Caps Lock does its own job.
settings-keyboard-caps-key = Caps Lock

# Choice in the Caps Lock list: it acts as a second Ctrl key.
settings-keyboard-caps-ctrl = Outro Ctrl

# Choice in the Caps Lock list: it acts as Escape.
settings-keyboard-caps-escape = Escape

# Choice in the Caps Lock list: it acts as Backspace.
settings-keyboard-caps-backspace = Backspace

# Choice in the Caps Lock list: it does nothing.
settings-keyboard-caps-nothing = Nada

# Heading of the line, and the name of its list, for what Caps Lock acts as.
settings-keyboard-caps-acts-as = Usar Caps Lock como

# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = Botão de teclado no anel

# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = Outras teclas

# Words behind the i of that card.
settings-keyboard-other-about = Caps Lock pode funcionar como outra tecla, como um segundo Ctrl ou Escape. O botão de teclado fica no meio do anel e abre um teclado na tela. Você pode usá-lo para digitar com o mouse ou uma tela sensível ao toque. Ele aparece na próxima vez que o anel abrir.

# Words under a line when a press that was not the person's own changed nothing.
settings-keyboard-only-yours = Um programa não pode mudar isto.

# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = Pressione a tecla ou o botão do mouse a mudar.

# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = Você não pressionou nada, então nada foi adicionado.

# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = Digite a tecla a usar no lugar, como Escape ou Ctrl+C, ou digite Nothing.

# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = O Trindle não conseguiu identificar qual era essa tecla. Escolha a tecla na lista.

# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = Todos os teclados e mouses

# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = Você mesmo alterou esta tecla abaixo, e essa alteração é a usada.

# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = Alterar { $key } em

# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = Alterar { $key } para

# Name of the bin button that takes a change away.
# SAFETY: needs a human review before it ships
settings-keyboard-take-away = Remover esta alteração

# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = A linha { $n } foi ignorada: { $why }

# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = Não foi possível ler as teclas alteradas

# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = Suas teclas funcionam normalmente até o Trindle conseguir ler as alterações. { $why }

# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = Escolha a tecla ou o botão a mudar

# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = Pressione a tecla ou o botão a mudar

# Heading of the line for changing a key or button.
settings-keyboard-keys-add = Mudar uma tecla ou botão

# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = Você já alterou { $key }.

# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = Teclas e botões

# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = Qualquer tecla ou botão do mouse pode fazer outra coisa. Ela pode virar outra tecla ou botão, ou teclas pressionadas juntas, como Ctrl+C, ou teclas pressionadas uma após a outra, como H, I. Também pode não fazer nada, ou fazer uma coisa ao toque e outra ao segurar, como Escape ao toque e Ctrl ao segurar. Isto funciona em todo lugar, a partir da tela de login. Ctrl, Alt e uma tecla F sempre abrem um console de texto, e digitar trindle-settings --keys default ali devolve todas as teclas ao normal.

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = Trocar Ctrl e Caps Lock

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Caps Lock: toque para Escape, segure para Ctrl

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = Trocar Escape e Caps Lock

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = Trocar Alt e Super, como em um teclado Mac

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = Os botões laterais do mouse copiam e colam

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
# SAFETY: needs a human review before it ships
settings-keyboard-made-switch = Confiar em { $name }

# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = Nenhum até agora

# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = Teclados e mouses criados por programas

# Words behind the i of that card.
# SAFETY: needs a human review before it ships
settings-keyboard-made-about = Ferramentas de remapeamento e iniciadores de jogos podem criar seu próprio teclado ou mouse. Suas teclas e cliques ainda podem digitar e jogar, mas não podem pressionar botões protegidos, como Desligar ou Permitir, nem responder a uma pergunta feita a você. Ligue um aqui para deixá-lo contar como você.

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = Controle { $vendor }:{ $product }

# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $name } em { $title }

# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = O que { $title } faz

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = Trocar A e B

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = Trocar X e Y

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = Inverter cima e baixo do direcional direito

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = Atalhos de plugins

# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin } não é usado: { $why }

# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = Teclas de { $label }

# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = Salvo

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = Vale para o touchpad também.

# Title of the mouse card.
settings-keyboard-mouse-title = Mouse

# Words behind the i of the mouse card.
settings-keyboard-mouse-about = A velocidade do ponteiro é o quanto o ponteiro se move quando você move o mouse. Vale para o touchpad também. A velocidade de rolagem é o quanto uma página anda a cada volta da roda. Escolha Mais lenta, Normal ou Mais rápida, ou ajuste em qualquer ponto da barra. Com rolagem natural, a página se move no sentido em que a roda gira, como em um celular. Canhoto troca os botões esquerdo e direito.

# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = Velocidade do ponteiro

# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = Velocidade de rolagem

# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = Rolagem natural

# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = Canhoto

# Heading and switch name for the clicking helper.
settings-keyboard-clicker = Auxiliar de cliques

# Small words under the clicking helper line.
settings-keyboard-clicker-about = Os cliques de um clicador por permanência ou por botão contam como seus.

# Title of the touchpad card.
settings-keyboard-touchpad-title = Touchpad

# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = A velocidade de rolagem é o quanto uma página anda quando você rola com dois dedos. Tocar para clicar faz um toque leve valer como clique. Com rolagem natural, a página se move no sentido dos seus dedos, como em um celular. Desligar ao digitar impede que a palma da mão mova o ponteiro enquanto você digita.

# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = Tocar para clicar

# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = Desligado ao digitar

# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = Mais lenta

# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = Normal

# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = Mais rápida

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = Portas USB

# Words behind the i of that card.
settings-keyboard-usb-about = Se uma porta USB não inicia, o Trindle a desliga e liga uma vez para tentar de novo. Ele nunca faz isso mais de uma vez por porta.

# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error } O Trindle também não conseguiu desfazer a alteração: { $undo }


# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = Capturas de tela, relatórios e atalhos

# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = Fazer uma captura de tela

# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = Mostrar a lista de atalhos

# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = Fazer uma captura de tela agora

# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = Mostrar a lista agora

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = Reiniciar a área de trabalho

# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = Reiniciar o anel e os auxiliares

# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = Reiniciar tudo

# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = Teclas para { $label }

# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = Reiniciar agora. As janelas abertas continuam como estão.

# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = Perguntar e depois reiniciar tudo

# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = Reiniciado

# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = Outro atalho já usa { $keys }.

# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = Não salvo. { $why }


# Name of the shortcut and its line that opens the window to report a problem.
settings-keyboard-shortcuts-report = Relatar um problema

# Tip of the round button that opens the report window now.
settings-keyboard-shortcuts-report-tip = Relatar um problema agora
# Shown beside a change that was not saved because the file of changed keys could not be read.
settings-keyboard-keys-not-saved-unread = Não salvo. O arquivo das teclas alteradas não pôde ser lido, por isso foi deixado como está.
