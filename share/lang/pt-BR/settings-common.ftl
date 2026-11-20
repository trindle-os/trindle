
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Said beside a line once its change is saved.
settings-common-saved = Salvo

# Said beside a button that worked, when nothing more is known.
settings-common-done = Pronto

# Words that only say "that worked", in lower case and separated by bars. A line saying one of them under a control that has already moved is left out, as the glow says it. Translate each word, and keep the bars.
settings-common-quiet-words = salvo|pronto|em uso agora|alterado|definido|tomado|adicionado|liberado|removido|guardado|instalado|atualizado|entrou|copiado|padrão agora|reproduzido|enviado|conectado|conectando|ligado|desligado|esquecido|para em 80%|carrega até o máximo|inicia ao entrar|guardado, mas não inicia|em dia

# Said beside a button that failed, when nothing more is known.
settings-common-not-done = Não concluído

# Tip of the information button at the top of every Settings page.
settings-common-about-page = Sobre esta página

# Tip of the information button on a card: { $name } is the card's heading as it is written.
settings-common-about-card = Sobre { $name }

# Heading of the question when a name is already taken: { $name } is the name.
settings-common-already-there = { $name } já existe

# Under the question when more of the same kind wait: how many more.
settings-common-same-for-rest-more =
    { $n ->
        [one] Fazer o mesmo com os demais ({ $n } a mais)
        [many] Fazer o mesmo com os demais ({ $n } a mais)
       *[other] Fazer o mesmo com os demais ({ $n } a mais)
    }

# Label of the switch that answers the same way for all that wait.
settings-common-same-for-rest = Fazer o mesmo com os demais

# Tip of the button that keeps the one already there.
settings-common-skip = Pular e manter o que já existe

# SAFETY: needs a human review before it ships
# Button that replaces the one already there.
settings-common-replace = Substituir

# SAFETY: needs a human review before it ships
# Spoken name of the Replace button.
settings-common-replace-spoken = Substituir o que já existe pelo novo

# Heading of the line that shows the new one, in the question about a name already taken.
settings-common-being-brought = O novo

# Heading of the line that shows the one already there.
settings-common-already-line = O que já existe

# SAFETY: needs a human review before it ships
# Heading of the line with the Skip and Replace buttons.
settings-common-replace-question = Substituir o que já existe?

# At the foot of the window while a change is on trial: how many seconds are left.
settings-common-goes-back =
    { $left ->
        [one] Desfazendo em { $left } segundo.
       *[other] Desfazendo em { $left } segundos.
    }

# At the foot of the window while a change is on trial: the seconds left, then { $more } such as what Escape does.
settings-common-goes-back-more =
    { $left ->
        [one] Desfazendo em { $left } segundo. { $more }
       *[other] Desfazendo em { $left } segundos. { $more }
    }

# Button at the foot of the window that undoes the change on trial.
settings-common-go-back-now = Desfazer agora

# Button at the foot of the window that keeps the change on trial.
settings-common-keep = Manter

# Heading of the line shown when a page needs a program that is not installed: { $what } is its name.
settings-common-not-available = { $what } não está disponível

# Under that heading: { $package } is the name of the package to install.
settings-common-install-with = Para instalar, execute: sudo apt install { $package }

# Red words when a web page could not be opened.
settings-common-no-browser = O Trindle não conseguiu abrir a página da web, porque não conseguiu iniciar um navegador.

# Red words when one of Trindle's programs could not be opened: { $name } is its name.
settings-common-could-not-open = Não foi possível abrir { $name }.

# Red words when a change could not be kept and nothing more is known.
settings-common-not-saved = Não foi possível salvar.

# Red words when an action needs the system's message bus and there is none.
settings-common-no-system-bus = O Trindle não conseguiu fazer isso porque uma parte do sistema não está respondendo. Reinicie o computador e tente de novo.

# Red words when an action needs this login's message bus and there is none.
settings-common-no-session-bus = O Trindle não conseguiu fazer isso porque uma parte da sua sessão não está respondendo. Saia e entre de novo, depois tente outra vez.

# Red words when the window for choosing a file or folder could not be opened.
settings-common-no-chooser = O Trindle não conseguiu abrir a janela para escolher um arquivo ou pasta. Saia e entre de novo, depois tente outra vez.

# Cause given in red words after an action failed: the disk is full.
settings-common-why-full = O disco está cheio.

# Cause given in red words after an action failed: the disk is read only.
settings-common-why-read-only = O Trindle não pode gravar no disco agora.

# Cause given in red words after an action failed: the part of the system that does it is not running.
settings-common-why-not-running = A parte do sistema que faz isso não está em execução.

# Cause given in red words after an action failed: no answer in time.
settings-common-why-slow = A resposta demorou demais. Tente de novo.

# SAFETY: needs a human review before it ships
# Cause given in red words after an action failed: the password was not given.
settings-common-why-password = Nenhuma senha foi informada.

# SAFETY: needs a human review before it ships
# Cause given in red words after an action failed: the account is not allowed.
settings-common-why-not-allowed = Esta conta não pode fazer isso.

# Cause given in red words after an action failed: something needed is missing.
settings-common-why-missing = Falta neste computador algo de que isso precisa.

# Red words after an action failed: { $could_not } is what could not be done and { $cause } why.
settings-common-why-then = { $could_not } { $cause }

# A list of names ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-and = { $rest } e { $last }

# What goes between names in a list, before the last two.
settings-common-list-comma = { ", " }

# Word drawn in the middle of that picture.
settings-common-ring-name = O anel

# Red words under the charge limit switch when the laptop's firmware door could not be opened; { $error } is the system's reason.
charge-no-firmware-door = O Trindle não conseguiu acessar os controles de carga do notebook: { $error }

# Red words under the charge limit switch when the laptop's firmware refused the limit.
charge-firmware-refused = O notebook não aceitou o limite de carga.

# Red words under the charge limit switch on a computer that cannot stop charging early.
charge-not-allowed = Este computador não permite interromper a carga antes do fim.

# Red words under the charge limit switch when a battery setting file could not be written; { $path } is the file and { $error } the system's reason.
charge-could-not-set = Não foi possível definir { $path }: { $error }

# Heading of the note, shown by the welcome window and the ring, to someone trying Trindle in a virtual machine.
graphics-vm-title = Experimentando o Trindle em uma máquina virtual

# Note to someone in a virtual machine that has 3D graphics; { $machine } is the machine's name.
graphics-vm-body-fast = O Trindle está rodando em { $machine }. Funciona bem ali, mas uma máquina virtual nunca é tão rápida nem tão fluida quanto um computador de verdade. Avalie a velocidade em um equipamento real.

# Note to someone in a virtual machine without 3D graphics; { $machine } is the machine's name.
graphics-vm-body-slow = O Trindle está rodando em { $machine } sem gráficos 3D. O processador precisa desenhar tudo o que um chip gráfico normalmente desenharia. Janelas, vídeos e páginas da web ficarão mais lentos do que em um computador de verdade, por isso o Trindle dispensa as animações. Se o computador em que ele roda permitir, ative a aceleração 3D nas configurações de vídeo da máquina virtual. Isso faz muita diferença. Nada disso se aplica se você instalar o Trindle em um computador de verdade.
# A list of choices ends with this: { $rest } is the names before the last, { $last } the last.
settings-common-or = { $rest } ou { $last }
