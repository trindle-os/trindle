# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = Instantâneos

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = instantâneos snapshot snapshots restaurar desfazer reverter voltar backup cópia atualização quebrou btrfs snapper menu de boot grub versões anteriores versão antiga arquivo apagado recuperar pasta pessoal cópias

# Help line at the foot of the Snapshots page.
# SAFETY: needs a human review before it ships
settings-snap-help = Um instantâneo é uma cópia do sistema como ele estava antes e depois de cada mudança de software, e sempre que você tirar um. Restaurar um instantâneo devolve o sistema inteiro ao que ele era, na próxima vez que você reiniciar. Seus próprios arquivos ficam como estão se a sua pasta pessoal tem uma parte própria do disco, como o instalador do Trindle configura. Se o computador não iniciar, escolha Instantâneos do Trindle no menu de inicialização. Versões anteriores guarda uma cópia diária dos arquivos de cada pessoa, separada dos instantâneos do sistema.

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = O Trindle não fez uma cópia porque o disco está quase cheio.

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
settings-snap-freeing = Liberando…

# Said beside the set aside line once the space is freed.
# SAFETY: needs a human review before it ships
settings-snap-freed = Liberado

# Said beside a snapshot while it is restored.
# SAFETY: needs a human review before it ships
settings-snap-restoring = Restaurando

# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
# SAFETY: needs a human review before it ships
settings-snap-restored = Quando você reiniciar, o sistema ficará como era no instantâneo { $n }. Tudo o que mudou desde então fica de lado, não se perde.

# Said beside a snapshot while it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleting = Apagando

# Said beside a snapshot once it is deleted.
# SAFETY: needs a human review before it ships
settings-snap-deleted = Apagado

# Said beside a setting while the administrator's password is awaited.
# SAFETY: needs a human review before it ships
settings-snap-waiting = Aguardando a sua senha…

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = Uma cópia é feita a cada dia

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = Sem mais cópias

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
settings-snap-measured-daily = O espaço delas é medido todo dia.

# Tip of the button that clears all earlier copies, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-clear-all = Limpar todas agora

# Heading of the card about earlier versions of files.
settings-snap-earlier = Versões anteriores

# Help text of the Earlier versions card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-earlier-about = O Trindle pode guardar uma cópia diária dos arquivos de todos. Assim você pode abrir, no Folders, uma versão mais antiga de um arquivo, ou recuperar um que apagou por engano. Em uma cópia, cada pessoa só vê o que podia ver naquele dia, então as pastas pessoais privadas continuam privadas. Um arquivo que outras pessoas podiam abrir continua aberto a elas naquela cópia até a cópia ser removida. Estas cópias são separadas dos instantâneos do sistema, e uma não altera a outra. Elas usam espaço em disco. Um arquivo que você apaga ou muda continua ocupando espaço até a última cópia que o guarda ser removida. Limpar as cópias libera o espaço na hora para todos, e não pode ser desfeito. Nenhuma cópia é feita enquanto o disco está quase cheio. Você precisa da senha de um administrador para mudar qualquer uma destas opções.

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = Guardar versões anteriores dos seus arquivos

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = Os arquivos apagados são guardados por { $time } e depois liberados.

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = Guardar a cópia de cada dia por

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = Por quanto tempo a cópia de cada dia é guardada

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = Verificando os instantâneos…

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = Não foi possível ler os instantâneos.

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = Você iniciou a partir do instantâneo { $n }, então nada que você fizer agora é guardado.

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [one] Você pode voltar a { $n } instantâneo, de { $when }.
        [many] Você pode voltar a { $n } de instantâneos. O mais recente é de { $when }.
       *[other] Você pode voltar a { $n } instantâneos. O mais recente é de { $when }.
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = Nenhum instantâneo ainda. Um é tirado antes de cada mudança de software.

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, guardado até ser apagado

# Tip of the button that restores the system to a snapshot, and its label while working.
# SAFETY: needs a human review before it ships
settings-snap-restore-this = Restaurar este instantâneo

# Tip of the button that deletes a snapshot.
# SAFETY: needs a human review before it ships
settings-snap-delete-this = Apagar este instantâneo

# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = Reiniciar agora

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = Ainda não estão configurados.

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = Os instantâneos não estão configurados. Reinstalar o pacote trindle os configura: sudo apt install --reinstall trindle.

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = outro tipo

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = Os instantâneos precisam de Btrfs. Este computador usa { $fs }.

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = Os instantâneos precisam que o sistema esteja em Btrfs, e este computador usa { $fs }. Para ter instantâneos, escolha Btrfs para o sistema ao instalar o Debian.

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = Não há instantâneos neste computador.

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = Nenhum instantâneo

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = Nenhum instantâneo neste computador

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = Carregando…

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = Não foi possível ler os instantâneos

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = Nenhum ainda

# Help text of the Snapshots card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-list-about = Cada instantâneo pode devolver o sistema inteiro ao que era, na próxima vez que você reiniciar. Você precisa da senha de um administrador para restaurar ou apagar um.

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = Tirar um instantâneo agora

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = Reinicie normalmente para fazer isto.

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [one] Um sistema antigo guardado à parte após uma restauração
       *[other] { $n } sistemas antigos guardados à parte após restaurações
    }

# Tip of the button that frees the space the set aside systems use.
# SAFETY: needs a human review before it ships
settings-snap-free-them = Liberar o espaço que usam

# Heading of the card for taking a snapshot.
settings-snap-take-one = Tirar um

# Help text of the Take one card (the information button).
settings-snap-take-about = Tire um instantâneo antes de tentar algo que você possa querer desfazer. Os instantâneos que você tira são guardados até você apagá-los. Se você iniciou a partir de um instantâneo, reinicie normalmente primeiro, senão o seu novo instantâneo não será guardado. Cada restauração deixa de lado o sistema como era logo antes, então nada se perde. Com os arquivos do sistema bloqueados, só instalações e atualizações podem mudá-los, e cada uma recebe um instantâneo antes e depois. Você precisa da senha de um administrador para mudar o bloqueio.

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = Iniciado a partir do instantâneo { $n }

# Under the Started from snapshot heading.
settings-snap-nothing-kept = As mudanças não serão guardadas.

# Heading of a line on the Before restoring card.
settings-snap-files-too = Seus arquivos também estão nos instantâneos

# Under the Your files are in the snapshots too heading.
settings-snap-files-back = Restaurar também devolve seus arquivos.

# Heading of the card about what to know before restoring.
settings-snap-before = Antes de restaurar

# Help text of the Before restoring card (the information button).
# SAFETY: needs a human review before it ships
settings-snap-before-about = Como você iniciou a partir de um instantâneo, nada que você fizer agora é guardado. Para manter o sistema assim, pressione Restaurar este instantâneo ao lado dele na lista e depois reinicie. Neste computador, a pasta pessoal pode fazer parte do sistema. Se for o caso, restaurar um instantâneo também devolve seus próprios arquivos ao que eram naquele momento, então copie antes qualquer coisa nova para um lugar seguro.


# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = Arquivos do sistema bloqueados

# Under the System files locked switch.
settings-snap-locked-sub = Só instalações e atualizações os desbloqueiam.

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = Bloqueados

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = Desbloqueados
