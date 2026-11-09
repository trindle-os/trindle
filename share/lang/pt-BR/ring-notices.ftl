
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Title of the low battery warning card, and the line under Charging when the charger goes in.
ring-battery-at = Bateria em { $pct }%

# Line under the low battery warning's title.
ring-battery-plug-in = Conecte o carregador em breve.

# Title of the card that shows for a few seconds when the charger goes in.
ring-battery-charging = Carregando

# Title of the card that shows for a few seconds when the charger comes out.
ring-battery-unplugged = Carregador desconectado

# Line under the title when the charger comes out.
ring-battery-running = Usando a bateria, em { $pct }%

# Choice on the snooze page of a notification circle (how long to put it off).
ring-snooze-10-minutes = 10 minutos

# Choice on the snooze page of a notification circle.
ring-snooze-1-hour = 1 hora

# Choice on the snooze page of a notification circle.
ring-snooze-6-hours = 6 horas

# Choice on the snooze page of a notification circle.
ring-snooze-next-morning = Amanhã de manhã

# Tip and screen reader name of the snooze choice for ten minutes.
ring-snooze-for-10-minutes = Adiar por 10 minutos

# Tip and screen reader name of the snooze choice for an hour.
ring-snooze-for-1-hour = Adiar por 1 hora

# Tip and screen reader name of the snooze choice for six hours.
ring-snooze-for-6-hours = Adiar por 6 horas

# Tip and screen reader name of the snooze choice for the next morning.
ring-snooze-for-next-morning = Adiar para amanhã de manhã

# Tip of the button on the snooze page that goes back to the notification's message.
ring-act-back = Voltar à mensagem

# Tip of the pin button when the notification is already pinned.
ring-act-unpin = Desafixar

# Tip of the pin button on a notification circle: keeps it until closed.
ring-act-pin = Fixar

# Tip of the button on a notification circle that opens the snooze choices.
ring-act-snooze = Adiar

# SAFETY: needs a human review before it ships
# Tip of the button on the stack of notification circles that clears them all.
ring-act-clear-all = Limpar tudo

# Tip of the copy button for a moment after the message was copied.
ring-act-copied = Copiado

# Tip of the button on a notification circle that copies its message.
ring-act-copy = Copiar a mensagem

# Line under the title of the notice that says one of Trindle's programs crashed.
ring-crash-click = Clique para entregar ao seu assistente.

# Name of Trindle's own main program in the crash notice.
ring-crash-trindle = Trindle

# Name of the desktop part in the crash notice.
ring-crash-desktop = A área de trabalho do Trindle

# Name of the login screen in the crash notice.
ring-crash-greeter = A tela de login do Trindle

# Name of the lock screen in the crash notice.
ring-crash-lock = A tela de bloqueio do Trindle

# Name of Settings in the crash notice.
ring-crash-settings = As Configurações do Trindle

# Name of Apps in the crash notice.
ring-crash-apps = Os Aplicativos do Trindle

# Name of any other Trindle program in the crash notice; the name is the program's own, with spaces for dashes.
ring-crash-other = O { $name } do Trindle

# Title of the notice that says a program crashed.
ring-crash-stopped = { $name } parou inesperadamente

# Notice when a drive's button was pressed and the drive has gone.
ring-drive-gone = Essa unidade não está mais lá

# Title of the notice when a drive could not be opened; the reason follows under it.
ring-drive-not-opened = Não foi possível abrir { $name }

# Choice for what closing the laptop lid does: sleep.
ring-lid-sleep = Suspender

# Choice for what closing the laptop lid does: hibernate.
ring-lid-hibernate = Hibernar

# Choice for what closing the laptop lid does: lock the screen.
ring-lid-lock = Bloquear a tela

# Choice for what closing the laptop lid does: nothing.
ring-lid-nothing = Nada

# Choice for what closing the laptop lid does: shut down.
ring-lid-shut-down = Desligar

# Reason the system lists beside Trindle's hold on the lid switch (shown by tools that list what stops sleep).
ring-lid-reason = Fechar a tampa faz o que Configurações, Energia define

# Notice at the foot of the screen for a few seconds when the magnifier starts.
ring-magnify-hint = Segure em qualquer lugar para ampliar

# Screen reader name of the magnifier's glass.
ring-magnify-name = Lupa

# Tip of the lit button that puts the magnifier away.
ring-magnify-stop = Parar de ampliar

# SAFETY: needs a human review before it ships
# Tip of the switch in the ring that turns the microphone off for every program.
ring-mic-off-tip = Desligar o microfone para todos os programas

# SAFETY: needs a human review before it ships
# Tip of the switch in the ring that turns the microphone back on.
ring-mic-on-tip = Ligar o microfone novamente

# SAFETY: needs a human review before it ships
# Line in the ring when something other than the person pressed the microphone switch on.
ring-mic-own-press = Só um toque seu pode ligar o microfone novamente.

# Line in the ring when the microphone switch could not be changed; the reason follows.
ring-mic-not-changed = Não foi possível mudar o interruptor do microfone: { $why }

# Notice when a notification was clicked and no program could be found for it.
ring-goes-no-program = Nenhum programa encontrado

# Notice when a notification was clicked and it was sent by a program Trindle cannot open.
ring-goes-other-program = Enviada por outro programa

# Notice when the program for a clicked notification could not be started.
ring-goes-not-started = Não foi possível iniciá-lo

# Notice after a click on a crash notification: its details were copied for an assistant.
ring-notes-copied = Copiado para o seu assistente

# Screen reader name of a notification circle.
ring-notes-name = Notificação

# Said when a pinned notification has been held long enough and now follows the pointer.
ring-notes-moving = Movendo. Solte para largar, Escape para devolver.

# Said when a pinned notification has been dropped or moved by keys.
ring-notes-moved = Movida

# Said when Escape put a held notification back.
ring-notes-put-back = Devolvida

# Shown in a notification that is not pinned when a key to move it is pressed.
ring-notes-pin-to-move = Fixe-a para movê-la

# Choice in Settings, Ring, for how long a notification circle stays.
ring-notify-seconds =
    { $n ->
        [one] { $n } segundo
        [many] { $n } segundos
       *[other] { $n } segundos
    }

# Choice in Settings, Ring: the notification circle stays until it is closed.
ring-notify-until-closed = Até você fechar

# Time under a notification from yesterday.
ring-notify-yesterday = Ontem { $clock }

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-right = Embaixo, à direita

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-bottom-left = Embaixo, à esquerda

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-left = Em cima, à esquerda

# Choice in Settings, Ring, for where notification circles stack from.
ring-notify-top-right = Em cima, à direita

# SAFETY: needs a human review before it ships
# Title of the notice when a plugin that asks for permissions has been given none.
ring-plugin-needs = { $name } precisa da sua permissão

# SAFETY: needs a human review before it ships
# Line under the notice that a plugin needs permission: none of the permissions it asks for are given.
ring-plugin-cannot-work = Ainda não pode funcionar: 0 de { $asks } permitidas.

# Said to a screen reader when a notification was copied; $said is what the circle said.
ring-copied-said = Copiado. { $said }



# Title of the notice for ordinary programs that have just reached the network; the program names are under it, and Allow means this is fine.
ring-outside-network = Acessou a rede

# Title of the notice for ordinary programs that have your files open; the program names are under it, and Allow means this is fine.
ring-outside-files = Está com seus arquivos abertos

# Under the title of those two notices: up to three program names, then how many more.
ring-outside-body =
    { $more ->
        [0] { $names }
        [one] { $names } e mais { $more }
        [many] { $names } e mais { $more }
       *[other] { $names } e mais { $more }
    }

