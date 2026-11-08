# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Tip and name of the round button that closes or puts away a ring panel, a notice or a battery card.
ring-close = Fechar

# Tip and name of a button that opens what is shown (a notification's drive, a notification's program).
ring-open = Abrir

# Name of the line or button that opens Settings from the ring (plugin menus, the ring's own tools).
ring-settings = Configurações

# First line of the ring's --help, printed in a terminal.
ring-cli-usage = Uso:

# Second line of the ring's --help, printed in a terminal: the command and where options go; the command's name is filled in.
ring-cli-command = { $program } [OPÇÃO…]

# Line of the ring's --help, printed in a terminal, above the list of options.
ring-cli-options = Opções:

# Help for the ring's --say option, printed in a terminal.
ring-cli-say = Faz o que estas palavras pedem, como se fossem ditas ao anel (para testar comandos de voz sem microfone)

# Help for the ring's --background option, printed in a terminal.
ring-cli-background = Inicia sem mostrar, para que a primeira abertura seja instantânea

# Help for the ring's --show-desktop option, printed in a terminal.
ring-cli-show-desktop = Guarda todas as janelas para mostrar a área de trabalho, ou as traz de volta (Super e D pedem isto)

# Help for the ring's --listen option, printed in a terminal.
ring-cli-listen = Abre o anel já ouvindo, como um atalho de teclado pode fazer

# Help for the ring's --all option, printed in a terminal.
ring-cli-all = Abre o anel com todos os aplicativos mostrados ao redor

# Help for the ring's --power option, printed in a terminal.
ring-cli-power = Abre o anel em sair, reiniciar e desligar

# Help for the ring's --favourites option, printed in a terminal.
ring-cli-favourites = Abre a página de favoritos nas Configurações

# Help for the ring's --restyle option, printed in a terminal.
ring-cli-restyle = Adota de imediato um novo visual (claro ou escuro, ou um novo destaque) das Configurações

# Help for the ring's --corners option, printed in a terminal.
ring-cli-corners = Cria de novo as áreas de toque nos cantos superiores

# Help for the ring's --notes option, printed in a terminal.
ring-cli-notes = Move o teclado para os círculos de notificação na tela (Super e N pedem isto)

# Error printed in a terminal when the ring is given an option it does not know.
ring-cli-unknown = opção desconhecida { $option }

# Error printed in a terminal when --say is given no words.
ring-cli-missing-say = falta um argumento para --say

# Line in the ring's results when what was typed matches no app or favourite.
ring-find-nothing = Nenhum aplicativo ou favorito tem esse nome

# Tip of the round button on an app's menu that removes the app.
# SAFETY: needs a human review before it ships
ring-uninstall = Desinstalar

# Line that replaces the uninstall button while the app is being removed.
# SAFETY: needs a human review before it ships
ring-uninstalling = Desinstalando

# Line that replaces the uninstall button once the app is removed.
# SAFETY: needs a human review before it ships
ring-uninstalled = Desinstalado

# Tip of the round button on the menu of an app put there by hand: it only leaves the ring.
ring-take-off = Tirar do anel

# Line that replaces that button while the app is being taken off the ring.
ring-taking-off = Tirando

# Line that replaces that button once the app is off the ring.
ring-taken-off = Tirado

# Name of the ring's round button that opens Apps.
ring-tool-apps = Aplicativos

# Name of the ring's round button that opens Activity.
ring-tool-activity = Atividade

# Name of the ring's round button that opens Drives.
ring-tool-drives = Unidades

# Name of the ring's round button that takes a screenshot.
ring-tool-screenshot = Captura de tela

# Name of the ring's round button that opens the clipboard page.
ring-tool-clipboard = Área de transferência

# Name of the ring's round button that opens the notifications page.
ring-tool-notifications = Notificações

# Name of the ring's round button that starts the magnifier.
ring-tool-magnifier = Lupa

# Name of the ring's round button that picks a colour from the screen.
ring-tool-eye-dropper = Conta-gotas

# Name of the ring's round button that opens the report a problem window.
ring-tool-report = Relatar um problema

# Line in the ring when the item that was pressed is not there any more.
ring-gone = Isso não está mais lá.

# Line in the ring when nothing could open the item that was pressed.
ring-could-not-open = Nada conseguiu abrir isso.

# Line in the ring while it listens to a spoken command.
ring-listening = Ouvindo... fale, e ele para quando você parar

# Line in the ring while it works out what was said.
ring-working-out = Entendendo...

# Answer to the spoken command to close windows when none are open.
ring-nothing-to-close = Nada para fechar

# Answer to the spoken question what the time is: the time as 24 hour clock, then the weekday, day and month.
ring-voice-time = São { $time } de { $date }

# Answer to the spoken command to change brightness when the screen has no brightness control.
ring-brightness-fixed = O brilho desta tela não pode ser alterado daqui

# Answer to the spoken command for the on-screen keyboard when it is not installed.
ring-keyboard-missing = O teclado na tela não está instalado

# Screen reader name of the ring of open windows.
ring-open-windows = Janelas abertas

# Tip of the button on a window's menu that brings a minimised window back.
ring-window-restore = Restaurar

# Tip of the button on a window's menu that minimises it.
ring-window-minimise = Minimizar

# Tip of the button on a window's menu that brings a maximised window back to its size.
ring-window-restore-size = Restaurar tamanho

# Tip of the button on a window's menu that maximises it.
ring-window-maximise = Maximizar

# Tip of the button on a window's menu that stops a program that does not answer.
# SAFETY: needs a human review before it ships
ring-window-force-close = Forçar fechamento

# Name and screen reader name of the button in the ring that shows every app.
ring-all-apps = Todos os aplicativos

# Name of the ring's round button that opens the power page.
ring-power = Energia

# Screen reader description of the ring's power button.
ring-power-spoken = Sair, reiniciar ou desligar

# Screen reader name of the ring's results while something is typed.
ring-finding = Procurando { $typed }

# Screen reader name of the time and date in the middle of the ring; pressing it opens Settings, Date and time.
ring-date-time = Data e hora

# Tip of the button on an app's menu that opens another window of an app already running.
ring-new-window = Nova janela

# Tip of the button on an app's menu that closes its windows.
ring-close-window =
    { $n ->
        [one] Fechar janela
        [many] Fechar todas as { $n } janelas
       *[other] Fechar todas as { $n } janelas
    }

# Tip of the star on the menu of a folder or file pinned to the ring: takes it off the ring.
ring-star-unpin = Desafixar do anel

# Tip of the star on the menu of a favourite app: removes it from the favourites.
ring-star-remove = Remover dos favoritos

# Tip of the star on the menu of an app: adds it to the favourites.
ring-star-add = Adicionar aos favoritos

# Screen reader name of the red button that shows why an uninstall failed.
ring-not-done = Não concluído

# Note on an app's menu when the middle of the ring has no room for another favourite; the number is how many it holds.
ring-middle-full =
    O centro comporta { $n }.
    Remova um para abrir espaço.

# Choice on the ring's power page: lock the screen.
ring-power-lock = Bloquear

# Choice on the ring's power page: start the screensaver.
ring-power-screensaver = Protetor de tela

# Choice on the ring's power page: put the computer to sleep.
ring-power-suspend = Suspender

# Choice on the ring's power page: hibernate.
ring-power-hibernate = Hibernar

# Choice on the ring's power page: log out.
# SAFETY: needs a human review before it ships
ring-power-log-out = Sair

# Choice on the ring's power page: restart the computer.
# SAFETY: needs a human review before it ships
ring-power-restart = Reiniciar

# Choice on the ring's power page: turn the computer off.
# SAFETY: needs a human review before it ships
ring-power-shut-down = Desligar

# Grey line in the middle of the ring when there are no favourites.
ring-empty-hint = Clique em Todos os aplicativos e depois clique com o botão direito em um aplicativo para mantê-lo aqui

# Line in the ring when a spoken or typed power choice needs a click to confirm.
ring-power-only-click = Só um clique confirma isto.

# Line in the ring when something other than the person tried to shut the computer down.
ring-power-only-own = Só você pode desligar este computador.

# Line in the ring when the power page closed because a program pressed a key.
ring-power-closed = A página de energia foi fechada: um programa apertou uma tecla, não você.

# Said in the middle of the ring when Close is chosen on a plugin's icon and none of its windows are open.
ring-plugin-not-open = Nada aberto para fechar

ring-startup-add = Iniciar ao entrar

ring-startup-remove = Parar de iniciar ao entrar



# Help for the option that opens the shortcut list, or closes it when it is showing.
ring-cli-shortcuts = Mostrar todos os atalhos de teclado em uma lista, ou guardar a lista (Super e K, a menos que mudado nas Configurações)

# Notice from the new ring after the parts were restarted from the keyboard.
ring-restarted = Reiniciado

# Under the notice above.
ring-restarted-why = O anel e as partes auxiliares iniciaram de novo.

