# machine translated (Russian), not yet reviewed by a person.

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = Снимки

# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = снимки снимок восстановить отменить откат вернуть резервная копия обновление сломалось btrfs snapper меню загрузки grub прежние версии старая версия предыдущая удалённый файл вернуть восстановление домашняя копии

# SAFETY: needs a human review before it ships
# Help line at the foot of the Snapshots page.
settings-snap-help = Снимки сохраняют копию системы такой, какой она была до и после каждого изменения программ и каждый раз, когда вы делаете снимок. Восстановление снимка возвращает всю систему в прежнее состояние после следующего перезапуска. Ваши личные файлы остаются как есть, если у домашней папки есть отдельная часть диска, как её создаёт установщик Trindle. Если компьютер не запускается, выберите «Снимки Trindle» в меню загрузки. «Прежние версии» хранят ежедневную копию личных файлов каждого пользователя отдельно от снимков системы.

# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = Копия не сделана, потому что диск почти заполнен.

# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = сегодня в { $time }

# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = вчера в { $time }

# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = { $date }

# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = Компьютер не перезапустился. Перезапустите его через кольцо.

# Said beside the Take a snapshot line while it works.
settings-snap-taking = Создаётся снимок

# Said beside the set aside line while the space is freed.
settings-snap-freeing = Освобождается…

# Said beside the set aside line once the space is freed.
settings-snap-freed = Освобождено

# Said beside a snapshot while it is restored.
settings-snap-restoring = Восстанавливается

# SAFETY: needs a human review before it ships
# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
settings-snap-restored = Со следующего запуска система будет в состоянии снимка { $n }. Всё, что изменилось с тех пор, отложено, а не потеряно.

# SAFETY: needs a human review before it ships
# Said beside a snapshot while it is deleted.
settings-snap-deleting = Удаляется

# SAFETY: needs a human review before it ships
# Said beside a snapshot once it is deleted.
settings-snap-deleted = Удалено

# SAFETY: needs a human review before it ships
# Said beside a setting while the administrator's password is awaited.
settings-snap-waiting = Ожидание вашего пароля…

# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = Копия делается каждый день

# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = Больше копий не будет

# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = Сохранено

# SAFETY: needs a human review before it ships
# Said beside the earlier copies while they are cleared.
settings-snap-clearing = Очищаются

# SAFETY: needs a human review before it ships
# Said beside the earlier copies once they are cleared.
settings-snap-cleared = Очищено

# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] Копий пока нет
        [one] { $n } копия
        [few] { $n } копии
        [many] { $n } копий
       *[other] { $n } копии
    }

# Under the number of earlier copies when there are none.
settings-snap-first-soon = Первая будет сделана в течение суток.

# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = Они занимают { $size }, измерено { $when }.

# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = Занимаемое ими место измеряется раз в сутки.

# SAFETY: needs a human review before it ships
# Tip of the button that clears all earlier copies, and its label while working.
settings-snap-clear-all = Очистить все сейчас

# Heading of the card about earlier versions of files.
settings-snap-earlier = Прежние версии

# SAFETY: needs a human review before it ships
# Help text of the Earlier versions card (the information button).
settings-snap-earlier-about = Trindle может хранить ежедневную копию файлов всех пользователей. Тогда старую версию файла или случайно удалённый файл можно открыть или вернуть из Folders. В копии каждый человек видит только то, что мог видеть в тот день, поэтому закрытые домашние папки остаются закрытыми. Файл, который тогда был открыт для других, остаётся для них открытым в этой копии, пока копию не удалят. Эти копии хранятся отдельно от снимков системы, и одно не влияет на другое. Они занимают место на диске. Удалённый или изменённый файл занимает место, пока не удалена последняя копия, где он есть. Очистка копий сразу освобождает место для всех, и это нельзя отменить. Пока диск почти заполнен, копии не делаются. Чтобы изменить что-либо из этого, нужен пароль администратора.

# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = Хранить прежние версии ваших файлов

# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = Удалённые файлы хранятся { $time }, затем их место освобождается.

# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = Хранить копию каждого дня

# Name of the list of how long each day's copy is kept.
settings-snap-how-long = Как долго хранится копия каждого дня

# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = Проверка снимков…

# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = Не удалось прочитать снимки.

# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = Вы запустили систему из снимка { $n }, поэтому ничего из того, что вы сделаете сейчас, не сохранится.

# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
        [one] Можно вернуться к { $n } снимку. Самый новый от { $when }.
        [few] Можно вернуться к { $n } снимкам. Самый новый от { $when }.
        [many] Можно вернуться к { $n } снимкам. Самый новый от { $when }.
       *[other] Можно вернуться к { $n } снимкам. Самый новый от { $when }.
    }

# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = Снимков пока нет. Один делается перед каждым изменением программ.

# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, хранится до удаления

# SAFETY: needs a human review before it ships
# Tip of the button that restores the system to a snapshot, and its label while working.
settings-snap-restore-this = Восстановить этот снимок

# SAFETY: needs a human review before it ships
# Tip of the button that deletes a snapshot.
settings-snap-delete-this = Удалить этот снимок

# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = Перезапустить сейчас

# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = Они пока не настроены.

# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = Снимки не настроены. Переустановка пакета trindle настроит их: sudo apt install --reinstall trindle.

# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = другой тип

# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = Снимкам нужна Btrfs. Этот компьютер использует { $fs }.

# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = Снимкам нужна система на Btrfs, а этот компьютер использует { $fs }. Чтобы они были, выберите Btrfs для системы при установке Debian.

# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = На этом компьютере нет снимков.

# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = Нет снимков

# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = На этом компьютере нет снимков

# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = Загрузка…

# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = Не удалось прочитать снимки

# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = Пока нет

# SAFETY: needs a human review before it ships
# Help text of the Snapshots card (the information button).
settings-snap-list-about = Каждый снимок может вернуть всю систему в прежнее состояние после следующего перезапуска. Чтобы восстановить или удалить снимок, нужен пароль администратора.

# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = Сделать снимок сейчас

# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = Для этого перезапустите систему как обычно.

# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
        [one] { $n } старая система отложена после восстановления
        [few] { $n } старые системы отложены после восстановлений
        [many] { $n } старых систем отложено после восстановлений
       *[other] { $n } старой системы отложено после восстановлений
    }

# SAFETY: needs a human review before it ships
# Tip of the button that frees the space the set aside systems use.
settings-snap-free-them = Освободить занимаемое ими место

# Heading of the card for taking a snapshot.
settings-snap-take-one = Сделать снимок

# SAFETY: needs a human review before it ships
# Help text of the Take one card (the information button).
settings-snap-take-about = Сделайте снимок, прежде чем пробовать то, что, возможно, захочется отменить. Сделанные вами снимки хранятся, пока вы их не удалите. Если вы запустили систему из снимка, сначала перезапустите её как обычно, иначе новый снимок не сохранится. Каждое восстановление откладывает систему такой, какой она была сразу перед ним, поэтому ничего не теряется. Если системные файлы заблокированы, их могут менять только установки и обновления, и для каждой из них делается снимок до и после. Чтобы изменить блокировку, нужен пароль администратора.

# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = Запуск из снимка { $n }

# Under the Started from snapshot heading.
settings-snap-nothing-kept = Изменения не сохранятся.

# Heading of a line on the Before restoring card.
settings-snap-files-too = Ваши файлы тоже входят в снимки

# Under the Your files are in the snapshots too heading.
settings-snap-files-back = Восстановление возвращает и ваши файлы.

# Heading of the card about what to know before restoring.
settings-snap-before = Перед восстановлением

# SAFETY: needs a human review before it ships
# Help text of the Before restoring card (the information button).
settings-snap-before-about = Так как вы запустили систему из снимка, ничего из того, что вы делаете сейчас, не сохраняется. Чтобы оставить систему такой, нажмите «Восстановить этот снимок» рядом с ним в списке, затем перезапустите. На этом компьютере домашняя папка может быть частью системы. Тогда восстановление снимка вернёт и ваши личные файлы к тому, какими они были, поэтому сначала скопируйте всё новое в надёжное место.


# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = Системные файлы заблокированы

# Under the System files locked switch.
settings-snap-locked-sub = Разблокируют их только установка и обновление.

# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = Заблокированы

# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = Разблокированы
