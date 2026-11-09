
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the Settings page about the keyboard and mouse, in the list and as its heading.
settings-keyboard-title = Teclado e mouse

# Words a person might type into the Settings search to find the keyboard page; separate them with spaces.
settings-keyboard-keywords = layout disposição idioma digitação caps lock teclado virtual botão anel mouse touchpad trackpad ponteiro velocidade toque clique rolagem roda mais lento mais rápido natural canhoto plugin atalho teclas remapear mudar trocar ctrl escape super alt mac botões laterais tocado segurado feito por programas keyd

# Introduction under the heading of the keyboard page, and the words behind the page i.
settings-keyboard-help = Tudo aqui muda na hora. Um segundo layout se alterna com Alt e Shift juntas.

# First sentence at the top of the page; { $layout } is the name of the layout in use.
settings-keyboard-top-typing = Digitando em { $layout }.

# Sentence after it when a second layout is set; { $layout } is the second layout.
settings-keyboard-top-second = Alt e Shift alternam para { $layout }.

# Sentence after it when Caps Lock is set to do nothing.
settings-keyboard-top-caps-nothing = Caps Lock não faz nada.

# Sentence after it when Caps Lock is set to act as another key; { $key } is that key, such as Escape.
settings-keyboard-top-caps-acts = Caps Lock age como { $key }.

# The key Caps Lock acts as, in the middle of a sentence: a second Ctrl key.
settings-keyboard-caps-as-ctrl = outro Ctrl

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-escape = Escape

# The key Caps Lock acts as, in the middle of a sentence.
settings-keyboard-caps-as-backspace = Backspace

# Sentence at the top of the page when the list of keyboard layouts is missing.
settings-keyboard-no-list = A lista de layouts de teclado não está neste computador.

# Title of the card shown when the list of keyboard layouts is missing.
settings-keyboard-no-list-title = Teclado

# Heading of the line in that card.
settings-keyboard-no-list-heading = A lista de layouts de teclado não está disponível

# Small words under that line, telling what to install; the command is typed as it is.
settings-keyboard-no-list-about = Instale com: sudo apt install xkb-data

# Sentence at the end of the top words when keys or buttons have been changed; { $n } is how many.
settings-keyboard-top-changed =
    { $n ->
        [one] Uma tecla foi alterada.
        [many] { $n } de teclas e botões foram alterados.
       *[other] { $n } teclas e botões foram alterados.
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

# Small words under the second layout line.
settings-keyboard-switch-to-about = Alt e Shift juntas alternam entre os dois.

# Words behind the i of the layout card.
settings-keyboard-layout-about = O que cada tecla digita: o layout e uma variante dele, como uma sem teclas mortas. Um segundo layout se alterna com Alt e Shift juntas, e volta do mesmo jeito. Tudo muda na hora.

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
settings-keyboard-caps-acts-as = A tecla Caps Lock age como

# Small words under that line.
settings-keyboard-caps-acts-as-about = Outro Ctrl, Escape e assim por diante.

# Heading of the line, and name of its switch, for the keyboard button in the ring.
settings-keyboard-ring-button = Botão de teclado no anel

# Small words under that line.
settings-keyboard-ring-button-about = Abre um teclado na tela.

# Title of the card with Caps Lock and the ring keyboard button.
settings-keyboard-other-title = Outras teclas

# Words behind the i of that card.
settings-keyboard-other-about = Caps Lock pode agir como outra tecla, como outro Ctrl ou Escape. O botão de teclado fica no centro do anel e abre um teclado na tela, para digitar com o mouse ou uma tela sensível ao toque. Ele aparece na próxima vez que o anel abrir.

# Words under a line when a press that was not the person's own changed nothing.
settings-keyboard-only-yours = Só um toque seu pode mudar isto.

# Words under the change a key line while it waits for a key or mouse button to be pressed.
settings-keyboard-press-now = Pressione a tecla ou o botão do mouse a mudar.

# Words under the change a key line when nothing was pressed in time.
settings-keyboard-nothing-pressed = Nada foi pressionado, então nada foi adicionado.

# Words under a changed key's line asking for what it becomes; the examples are names a person types.
settings-keyboard-type-it = Digite no que ela se torna, como Escape, Ctrl+C ou Nothing.

# Words under the change a key line when the press could not be told apart from another.
settings-keyboard-not-told = Não foi possível distinguir esse toque. Escolha a tecla na lista.

# Choice in the list of keyboards and mice: the change applies on every one.
settings-keyboard-every-device = Todos os teclados e mouses

# Small words under a ready-made swap that the person's own change overrides.
settings-keyboard-swap-over = Também alterado abaixo; essa alteração é a usada.

# Name of the list of keyboards for one changed key; { $key } is the key.
settings-keyboard-which-device = Em que teclado ou mouse { $key } é alterada

# Name of the box for what a changed key becomes; { $key } is the key.
settings-keyboard-becomes = No que { $key } se torna

# Name of the bin button that takes a change away.
# SAFETY: needs a human review before it ships
settings-keyboard-take-away = Remover esta alteração

# Small words under a changed key's line when it changes on every keyboard and mouse.
settings-keyboard-every-device-about = Em todos os teclados e mouses.

# Small words under a line of a file that was not understood; { $n } is its number and { $why } the reason.
settings-keyboard-line-unused = A linha { $n } não é usada: { $why }

# Heading of the line shown when the file of changed keys could not be read.
settings-keyboard-keys-unread = Não foi possível ler as teclas alteradas

# Small words under that line; { $why } is the reason.
settings-keyboard-keys-unread-about = Toda tecla fica como impressa até que se possa: { $why }.

# Name of the list from which a key or button to change is chosen.
settings-keyboard-keys-list = Escolha a tecla ou o botão a mudar

# Name of the button that waits for a key or button to be pressed.
settings-keyboard-keys-press = Pressione a tecla ou o botão a mudar

# Small words under the change a key line.
settings-keyboard-keys-add-about = Pressione-a ou escolha-a na lista.

# Heading of the line for changing a key or button.
settings-keyboard-keys-add = Mudar uma tecla ou botão

# Words under the change a key line when the key chosen is already changed; { $key } is the key.
settings-keyboard-already = { $key } já foi alterada, abaixo.

# Title of the card with changed keys and buttons.
settings-keyboard-keys-title = Teclas e botões

# Words behind the i of the keys and buttons card; the examples are names a person types.
settings-keyboard-keys-about = Qualquer tecla ou botão do mouse pode fazer outra coisa: outra tecla ou botão, teclas seguradas juntas como Ctrl+C, uma sequência de teclas como H, I, nada, ou uma coisa ao toque e outra ao segurar, como Escape ao toque e Ctrl ao segurar. Funciona em todo lugar, desde a tela de login. Ctrl, Alt e uma tecla F sempre chegam a um console de texto, e trindle-settings --keys default digitado ali devolve todas as teclas ao normal.

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-ctrl-caps = Trocar Ctrl e Caps Lock

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-caps-tap = Caps Lock é Escape ao toque, Ctrl ao segurar

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-esc-caps = Trocar Escape e Caps Lock

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-alt-super = Trocar Alt e Super, como em um teclado Mac

# Name of a ready-made swap of keys, as the heading and switch name of its line.
settings-keyboard-swap-side-copy = Os botões laterais do mouse copiam e colam

# Small words under a keyboard or mouse a program made, when it counts as the person.
settings-keyboard-made-yes = Conta como você, como um teclado conectado.

# Small words under a keyboard or mouse a program made, when it does not count as the person.
settings-keyboard-made-no = Digita e joga. Não pode pressionar botões protegidos.

# Name of the switch that lets a program's keyboard or mouse count as the person; { $name } is its name.
# SAFETY: needs a human review before it ships
settings-keyboard-made-switch = { $name } conta como você

# Heading of the line shown when no program has made a keyboard or mouse.
settings-keyboard-made-none = Nenhum até agora

# Small words under that line.
settings-keyboard-made-none-about = Um criado por um programa aparece aqui, para que você o deixe contar como você.

# Title of the card about keyboards and mice made by programs.
settings-keyboard-made-title = Teclados e mouses criados por programas

# Words behind the i of that card.
# SAFETY: needs a human review before it ships
settings-keyboard-made-about = Ferramentas de remapeamento e iniciadores de jogos podem criar um teclado ou mouse próprio. Suas teclas e cliques ainda digitam e jogam, mas não podem pressionar botões protegidos, como Desligar ou Permitir, nem digitar em uma pergunta feita a você. Ligue um aqui para deixá-lo contar como você.

# Name of a game controller that has no name; { $vendor } and { $product } are numbers that identify it.
settings-keyboard-pad-unnamed = Controle { $vendor }:{ $product }

# Name of the switch for a controller's swap; { $name } is the swap and { $title } the controller.
settings-keyboard-pad-swap-switch = { $name } em { $title }

# Name of the box for what a controller's button does; { $title } is the controller.
settings-keyboard-pad-does = O que { $title } faz

# Small words under a controller's changed button; the words Button and what it becomes stand for the two sides of the line typed.
settings-keyboard-pad-type = Digite como Botão = no que ele se torna.

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-a-b = Trocar A e B

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-x-y = Trocar X e Y

# Name of a ready-made swap of a game controller's buttons.
settings-keyboard-pad-swap-stick = Inverter cima e baixo do direcional direito

# Title of the card with the shortcuts plugins ask for.
settings-keyboard-plugin-title = Atalhos de plugins

# Small words under a plugin shortcut that is not in use; { $plugin } is the plugin and { $why } the reason.
settings-keyboard-plugin-refused = { $plugin }. Não em uso: { $why }

# Small words under a plugin shortcut; { $plugin } is the plugin. The example is keys a person types.
settings-keyboard-plugin-about = { $plugin }. Digite as teclas, como Ctrl+Alt+S, ou nada para nenhuma.

# Name of the box for a plugin shortcut's keys; { $label } is the shortcut.
settings-keyboard-plugin-keys-name = Teclas de { $label }

# Said in green under a plugin shortcut after the keys were kept.
settings-keyboard-plugin-saved = Salvo

# Small words under the pointer speed slider when there is a touchpad.
settings-keyboard-both = O touchpad também.

# Title of the mouse card.
settings-keyboard-mouse-title = Mouse

# Words behind the i of the mouse card.
settings-keyboard-mouse-about = A velocidade do ponteiro é quanto o ponteiro anda para um movimento do mouse, e do touchpad também. A velocidade de rolagem é quanto uma página anda a cada volta da roda: Mais lenta, Normal ou Mais rápida, ou em qualquer ponto da barra. Com rolagem natural a página se move no sentido em que a roda gira, como em um celular. Canhoto troca os botões esquerdo e direito.

# Heading and slider name for the pointer speed.
settings-keyboard-pointer-speed = Velocidade do ponteiro

# Heading and slider name for the scroll speed of the wheel or the touchpad.
settings-keyboard-scroll-speed = Velocidade de rolagem

# Small words under the wheel's scroll speed.
settings-keyboard-scroll-wheel-about = Quanto uma página anda por volta da roda.

# Heading of the natural scrolling switch (the page follows the wheel or fingers).
settings-keyboard-natural = Rolagem natural

# Small words under natural scrolling for the mouse wheel.
settings-keyboard-natural-wheel-about = A página segue a roda, como em um celular.

# Heading of the switch that swaps the left and right mouse buttons.
settings-keyboard-left-handed = Canhoto

# Small words under that switch.
settings-keyboard-left-handed-about = Troca os botões esquerdo e direito.

# Heading and switch name for the clicking helper.
settings-keyboard-clicker = Auxiliar de cliques

# Small words under the clicking helper line.
settings-keyboard-clicker-about = Os cliques de um clicador por permanência ou por botão contam como seus.

# Title of the touchpad card.
settings-keyboard-touchpad-title = Touchpad

# Words behind the i of the touchpad card.
settings-keyboard-touchpad-about = A velocidade de rolagem é quanto uma página anda com dois dedos. Tocar para clicar faz um toque leve valer como clique. Com rolagem natural a página se move no sentido dos seus dedos, como em um celular. Desligar ao digitar impede que a palma da mão roçando no touchpad mova o ponteiro enquanto você digita.

# Small words under the touchpad's scroll speed.
settings-keyboard-scroll-touchpad-about = Quanto uma página anda com dois dedos.

# Heading of the switch that makes a light tap count as a click.
settings-keyboard-tap = Tocar para clicar

# Small words under that switch.
settings-keyboard-tap-about = Um toque leve vale como clique.

# Small words under natural scrolling for the touchpad.
settings-keyboard-natural-touchpad-about = A página segue seus dedos, como em um celular.

# Heading of the switch that stops the touchpad moving the pointer while typing.
settings-keyboard-quiet = Desligado ao digitar

# Small words under that switch.
settings-keyboard-quiet-about = Uma palma roçando deixa o ponteiro parado.

# Choice above a scroll speed slider: slower than normal.
settings-keyboard-pace-slower = Mais lenta

# Choice above a scroll speed slider: the normal speed.
settings-keyboard-pace-normal = Normal

# Choice above a scroll speed slider: faster than normal.
settings-keyboard-pace-faster = Mais rápida

# Title of the card about USB ports that were switched off and on.
settings-keyboard-usb-title = Portas USB

# Words behind the i of that card.
settings-keyboard-usb-about = Quando uma porta USB não inicia, o Trindle a desliga e liga uma vez para tentar de novo, e nunca mais de uma vez por porta.

# Red words under the keyboard button switch when it could not be kept and its mark could not be taken away again; { $error } is the first problem and { $undo } the second.
settings-keyboard-button-and-mark = { $error }; e a marca dele não pôde ser removida: { $undo }


# Title of the card with the keys that take a screenshot and show the shortcut list.
settings-keyboard-shortcuts-title = Captura de tela e lista de atalhos

# Name of the shortcut and its line that starts the screenshot.
settings-keyboard-shortcuts-screenshot = Fazer uma captura de tela

# Name of the shortcut and its line that shows every shortcut in a list.
settings-keyboard-shortcuts-list = Mostrar a lista de atalhos

# Small words under each of the two shortcuts.
settings-keyboard-shortcuts-about = Digite as teclas, ou nada para nenhuma.

# Tip of the round button that starts the screenshot now.
settings-keyboard-shortcuts-screenshot-tip = Fazer uma captura de tela agora

# Tip of the round button that shows the shortcut list now.
settings-keyboard-shortcuts-list-tip = Mostrar a lista agora

# Title of the card with the two shortcuts that restart the desktop.
settings-keyboard-restart-title = Reiniciar a área de trabalho

# Name of the shortcut and its line that restarts the ring and the helper parts only.
settings-keyboard-restart-parts = Reiniciar o anel e as partes auxiliares

# Name of the shortcut and its line that restarts everything, the compositor too.
settings-keyboard-restart-all = Reiniciar tudo

# Small words under each of the two restart shortcuts. The example is keys a person types.
settings-keyboard-restart-about = Digite as teclas, como Ctrl+Alt+Escape, ou nada para nenhuma.

# Name of the box for a restart shortcut's keys; { $label } is the shortcut.
settings-keyboard-restart-keys-name = Teclas para { $label }

# Tip of the round button that restarts the ring and the helper parts now; open windows are left as they are.
settings-keyboard-restart-parts-tip = Reiniciar agora. As janelas abertas ficam como estão.

# Tip of the round button that asks whether to restart everything now.
settings-keyboard-restart-all-tip = Reiniciar tudo. Pergunta antes.

# Said under the line after the parts were restarted.
settings-keyboard-restart-parts-done = Reiniciado

# Said in red under a restart shortcut whose keys another shortcut has; { $keys } is the keys.
settings-keyboard-restart-clash = { $keys } já é usado por outro atalho.

# Said in red under a restart shortcut that could not be kept; { $why } is the reason.
settings-keyboard-restart-not-saved = Não salvo: { $why }


# Name of the shortcut and its line that opens the window to report a problem.
settings-keyboard-shortcuts-report = Relatar um problema

# Tip of the round button that opens the report window now.
settings-keyboard-shortcuts-report-tip = Relatar um problema agora

