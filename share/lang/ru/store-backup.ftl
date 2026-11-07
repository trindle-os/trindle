# machine translated (Russian), not yet reviewed by a person.

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
# SAFETY: needs a human review before it ships
store-refusal-nothing = Ничего не восстановлено, потому что { $why }.

# A refusal of a restore that went part of the way; the reason follows "because".
# SAFETY: needs a human review before it ships
store-refusal-part = Часть резервной копии не восстановлена, потому что { $why }.

# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
# SAFETY: needs a human review before it ships
store-because-outside = в резервной копии есть ссылка или что-то ещё, что выходило бы за пределы папки, куда её распаковывают

# The reason when something on this computer stands where part of the backup needs to go.
# SAFETY: needs a human review before it ships
store-because-in-the-way = на этом компьютере уже есть что-то там, куда должна попасть часть резервной копии

# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = негде распаковать резервную копию

# The reason when the backup file cannot be read.
store-because-unreadable = не удалось прочитать файл резервной копии

# The reason when the file is not a Trindle backup.
store-because-not-trindle = этот файл не является резервной копией Trindle

# The reason when the backup's list is damaged or too big.
store-because-damaged-list = резервная копия повреждена, и её список не удалось прочитать

# The reason when the file is not a Flat backup.
store-because-not-flat = этот файл не похож на резервную копию Flat

# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = в этом файле нет списка приложений Flat

# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = она создана более новым Flat, чем Trindle может прочитать

# The reason when the file holds no apps.
store-because-no-apps = в этом файле нет приложений

# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = не удалось прочитать резервную копию: { $why }

# The reason when the program gave none.
store-because-none = причина не указана

# The reason when the system said permission was denied.
# SAFETY: needs a human review before it ships
store-because-permission = этот компьютер не позволил Trindle записать туда

# The reason when the disk is full.
store-because-space = на этом компьютере не осталось места

# The reason when a part of the computer is read-only.
store-because-read-only = эта часть компьютера доступна только для чтения

# The reason when the person's disk quota is used up.
store-because-quota = место, выделенное для этой учётной записи, закончилось

# The reason when a file or program is missing.
store-because-missing = не хватает нужного ему файла или программы

# The reason when no reason could be read from the program's last line.
store-because-failed = сбой

# The reason when the backup has no name to be written under.
store-because-no-name = у неё нет имени

# The reason when a backup's settings for an app reach outside the app's own folder.
# SAFETY: needs a human review before it ships
store-because-outside-own = её настройки в резервной копии выходят за пределы её собственной папки

# The reason when something is too big to be read whole.
store-because-too-big = она слишком велика, чтобы прочитать целиком

# The reason when an app's settings in the backup do not match their checksum.
# SAFETY: needs a human review before it ships
store-because-checksum = её настройки в резервной копии не совпадают с контрольной суммой

# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = Восстановлено, кроме картинки вашей учётной записи. { $why }

# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } Картинка вашей учётной записи тоже не восстановлена. { $why }

# Said when the pkexec program could not be started.
store-refusal-pkexec = Не удалось запустить pkexec: { $why }

# Said when the backup could not be read at all.
store-refusal-not-read-from = Не удалось прочитать из резервной копии: { $why }.

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = Резервная копия не создана, потому что { $why }.

# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = пока ничего

# Beside the size of a part of the backup: about how big it is.
store-backup-about = около { $size }

# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Восстановление настроек Trindle и AppImage...

# Step shown while the settings and data are put back.
store-backup-step-data = Восстановление настроек и данных...

# Step shown while apps are installed from Flathub.
store-backup-step-installing =
    { $count ->
        [one] Установка приложений из Flathub: { $count }... это может занять время.
        [few] Установка приложений из Flathub: { $count }... это может занять время.
        [many] Установка приложений из Flathub: { $count }... это может занять время.
       *[other] Установка приложений из Flathub: { $count }... это может занять время.
    }

# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = Настройки и данные восстановлены, но эти приложения не установлены, потому что Flatpak отказал: { $failed }.

# The same, when other apps were also left out for another reason.
store-not-installed-both = Настройки и данные восстановлены, но эти приложения не установлены, потому что Flatpak отказал: { $failed }. Также не установлены: { $refused }.

# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = Настройки и данные восстановлены, но эти приложения не установлены: { $refused }.

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
        [one] Пропущено приложений: { $n }, их имена недопустимы.
        [few] Пропущено приложений: { $n }, их имена недопустимы.
        [many] Пропущено приложений: { $n }, их имена недопустимы.
       *[other] Пропущено приложений: { $n }, их имена недопустимы.
    }

# Said beside an app whose source is not secure: the source's name is given.
# SAFETY: needs a human review before it ships
store-flat-not-secure = источник { $name } не зашифрован и не был принят, поэтому не использовался

# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = источника { $name } нет на этом компьютере, а резервная копия не говорит, где он

# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = настройки { $name } (потому что их копия в резервной копии не прошла проверки)

# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (потому что имя его источника недопустимо)

# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (потому что { $why })

# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (потому что Flatpak отказал: { $why })

# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = всё (потому что негде распаковать резервную копию)

# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = то, что разрешено каждому приложению

# Said among what could not be done: one app's settings.
store-flat-app-settings = настройки { $name }

# Said when an app's settings could not be put back and the old folder was kept.
# SAFETY: needs a human review before it ships
store-flat-kept-as = { $why }. То, что было там раньше, сохранено как { $path }: { $error }

# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = Установка { $name } ({ $at } из { $total })...

# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = Восстановление настроек { $name } ({ $at } из { $total })...

# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
        [one] Установлено приложений: { $n }.
        [few] Установлено приложений: { $n }.
        [many] Установлено приложений: { $n }.
       *[other] Установлено приложений: { $n }.
    }

# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings = Настройки восстановлены для: { $n }.

# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
        [one] Уже были установлены и оставлены как есть: { $n }.
        [few] Уже были установлены и оставлены как есть: { $n }.
        [many] Уже были установлены и оставлены как есть: { $n }.
       *[other] Уже были установлены и оставлены как есть: { $n }.
    }

# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = Не восстановлено: { $list }

# Name of Flat's backup program, which Trindle reads backups from.
store-flat-name = Trindle Flat

# Said among the apps that were not installed: the app and why.
store-backup-app-why = { $id } ({ $why })

