# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = Instantâneos

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = instantâneos snapshot snapshots restaurar desfazer reverter voltar backup cópia atualização quebrou btrfs snapper menu de boot grub versões anteriores versão antiga arquivo apagado recuperar pasta pessoal cópias

# Help line at the foot of the Snapshots page.
# SAFETY: needs a human review before it ships
settings-snap-help = O sistema como estava antes e depois de cada mudança de software, e sempre que você tirar um. Restaurar um devolve o sistema inteiro àquele estado a partir da próxima inicialização; seus próprios arquivos ficam como estão quando a pasta pessoal tem uma parte própria do disco, como o instalador do Trindle a cria. Se o computador não iniciar, escolha Instantâneos do Trindle no menu de inicialização. Versões anteriores guarda uma cópia dos arquivos de cada pessoa a cada dia, à parte dos instantâneos do sistema.

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = Nenhuma cópia foi feita: o disco está quase cheio.

# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = hoje às { $time }

# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = ontem às { $time }

# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = em { $date }

# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = O computador não reiniciou. Reinicie-o pelo anel.

# Said beside the Take a snapshot line while it works.
settings-snap-taking = Tirando um instantâneo

# Said beside the set aside line while the space is freed.
# SAFETY: needs a human review before it ships
settings-snap-freeing = Liberando...

# Said beside the set aside line once the space is freed.
# SAFETY: needs a human review before it ships
settings-snap-freed = Liberado

# Said beside a snapshot while it is restored.
# SAFETY: needs a human review before it ships
settings-snap-restoring = Restaurando

# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
settings-snap-restored = O sistema será o instantâneo { $n } a partir da próxima inicialização. Tudo o que mudou desde então fica de lado, não se perde.

# Said beside a snapshot while it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleting = Apagando

# Said beside a snapshot once it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleted = Apagado

# Said beside a setting while the administrator's password is awaited.
# SAFETY: needs a human review before it ships
settings-snap-waiting = Aguardando a senha...

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = Uma cópia é feita a cada dia

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = Nenhuma outra é feita

# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = Guardado

# Said beside the earlier copies while they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-clearing = Limpando

# Said beside the earlier copies once they are cleared.
# SAFETY: needs a human review before it ships
settings-snap-cleared = Limpo

# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] Nenhuma cópia ainda
        [one] Uma cópia
        [many] { $n } de cópias
       *[other] { $n } cópias
    }

# Under the number of earlier copies when there are none.
settings-snap-first-soon = A primeira é feita em até um dia.

# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = Ocupam { $size }, medido { $when }.

# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = O espaço delas é medido uma vez por dia.

# Tip of the button that clears all earlier copies, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-clear-all = Limpar todas agora

# Heading of the card about earlier versions of files.
settings-snap-earlier = Versões anteriores

# Help text of the Earlier versions card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-earlier-about = O Trindle pode guardar uma cópia dos arquivos de todos como estavam a cada dia, para que uma versão mais antiga de um arquivo, ou um apagado por engano, possa ser aberta ou recolocada em Pastas. Cada pessoa só pode ler em uma cópia o que podia ler naquele dia, então uma pasta pessoal privada continua privada; um arquivo que estava aberto a outros continua aberto a eles naquela cópia até ela sumir. As cópias são à parte dos instantâneos do sistema: restaurar o sistema nunca muda seus arquivos, e elas nunca mudam o sistema. O custo é espaço: um arquivo que você apaga ou muda continua ocupando seu espaço até sumir a última cópia que o guarda, então apagar coisas não libera o disco na hora. Limpar as cópias libera agora, para todos neste computador, e não pode ser desfeito. Nenhuma cópia é feita enquanto o disco está quase cheio. Mudar qualquer coisa disso precisa da senha de um administrador.

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = Guardar versões anteriores dos seus arquivos

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = Os arquivos apagados ficam por { $time } antes de seu espaço ser liberado.

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = Guardar a cópia de cada dia por

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = Por quanto tempo a cópia de cada dia é guardada

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = Olhando os instantâneos…

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = Não foi possível ler os instantâneos.

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = Iniciado a partir do instantâneo { $n }, então nada feito agora é guardado.

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [one] { $n } instantâneo para voltar, o mais recente de { $when }.
        [many] { $n } de instantâneos para voltar, o mais recente de { $when }.
       *[other] { $n } instantâneos para voltar, o mais recente de { $when }.
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = Nenhum instantâneo ainda. Um é tirado antes de cada mudança de software.

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, guardado até ser apagado

# Tip of the button that restores the system to a snapshot, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-restore-this = Restaurar o sistema a este

# Tip of the button that deletes a snapshot.
# SAFETY: needs a human review before it ships
settings-snap-delete-this = Apagar este instantâneo

# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = Reiniciar agora

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = Ainda não estão configurados.

# Under the To have them heading when snapshots are not set up.
settings-snap-reinstall = Reinstale o pacote trindle.

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = Os instantâneos não estão configurados. Reinstalar o pacote trindle os configura: sudo apt install --reinstall trindle.

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = outro tipo

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = Eles precisam de Btrfs; este computador usa { $fs }.

# Under the To have them heading when the computer does not use Btrfs.
settings-snap-choose-btrfs = Escolha Btrfs ao instalar o Debian.

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = Os instantâneos precisam do sistema em Btrfs, e este computador usa { $fs }. Escolha Btrfs para o sistema ao instalar o Debian para tê-los.

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = Não há instantâneos neste computador.

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = Nenhum instantâneo

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = Nenhum instantâneo neste computador

# Heading of the line saying how to have snapshots.
settings-snap-to-have = Para tê-los

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = Verificando…

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = Não foi possível ler os instantâneos

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = Nenhum ainda

# Under the None yet heading.
settings-snap-none-line-sub = Um é tirado antes de cada mudança de software.

# Help text of the Snapshots card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-list-about = Cada instantâneo pode devolver o sistema inteiro ao que era naquele momento, a partir da próxima inicialização, ou ser apagado. Ambos pedem para ser pressionados duas vezes e precisam da senha de um administrador. Depois de uma restauração, aparece ao lado um botão para reiniciar.

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = Tirar um instantâneo agora

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = Não enquanto iniciado a partir de um instantâneo.

# Under the Take a snapshot now heading.
settings-snap-kept-until-you = Guardado até você apagá-lo.

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [one] Um sistema deixado de lado por uma restauração
        [many] { $n } de sistemas deixados de lado por restaurações
       *[other] { $n } sistemas deixados de lado por restaurações
    }

# Under the set aside heading.
# SAFETY: needs a human review before it ships
settings-snap-free-space = Libere o espaço quando tudo estiver bem.

# Tip of the button that frees the space the set aside systems use.
# SAFETY: needs a human review before it ships
settings-snap-free-them = Liberar o espaço que usam

# Heading of the card for taking a snapshot.
settings-snap-take-one = Tirar um

# Help text of the Take one card (the information button).
settings-snap-take-about = Tire um instantâneo antes de tentar algo que você possa querer desfazer; um que você tira é guardado até você apagá-lo. Enquanto o computador está iniciado a partir de um instantâneo, nada que se tirasse seria guardado, então reinicie normalmente antes. Cada restauração deixa de lado o que o sistema era logo antes dela, então nada se perde; libere o espaço que elas usam quando tudo estiver bem.

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = Iniciado a partir do instantâneo { $n }

# Under the Started from snapshot heading.
settings-snap-nothing-kept = Nada feito agora é guardado.

# Heading of a line on the Before restoring card.
settings-snap-files-too = Seus arquivos também estão nos instantâneos

# Under the Your files are in the snapshots too heading.
settings-snap-files-back = Restaurar também devolve seus arquivos.

# Heading of the card about what to know before restoring.
settings-snap-before = Antes de restaurar

# Help text of the Before restoring card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-before-about = Iniciado a partir de um instantâneo, nada feito agora é guardado: se o sistema deve ficar assim, pressione restaurar nele na lista e depois reinicie. Neste computador a pasta pessoal pode fazer parte do sistema, e então restaurar um instantâneo também devolve seus próprios arquivos ao que eram naquele momento, então copie antes qualquer coisa nova para um lugar seguro.


# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = Arquivos do sistema bloqueados

# Under the System files locked switch.
settings-snap-locked-sub = Só instalações e atualizações os desbloqueiam.

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = Bloqueados

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = Desbloqueados

