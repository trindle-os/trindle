# machine translated (Korean), not yet reviewed by a person.

# Apps: moving apps and settings to another computer, and the reasons a backup was refused (src/store/refusal.rs, backup.rs, put_back.rs, flat.rs).

## Why a backup was refused (src/store/refusal.rs)

# A refusal of a restore; the reason, written as the end of a sentence, follows "because".
store-refusal-nothing = 아무것도 복원하지 않았어요. 이유: { $why }.
# A refusal of a restore that went part of the way; the reason follows "because".
store-refusal-part = Trindle이 백업의 일부만 복원했어요. 이유: { $why }.
# The reason when the backup holds a link or something else that would reach outside the folder it goes into.
store-because-outside = 백업에 들어갈 폴더 밖을 가리키는 링크 등이 들어 있어요
# The reason when something on this computer stands where part of the backup needs to go.
store-because-in-the-way = 백업의 일부가 들어갈 자리에 이 컴퓨터의 다른 항목이 이미 있어요
# The reason when there is nowhere to unpack the backup.
store-because-no-unpack = 백업을 풀 곳이 없어요
# The reason when the file is not a Trindle backup.
store-because-not-trindle = 그 파일은 Trindle 백업이 아니에요
# The reason when the backup's list is damaged or too big.
store-because-damaged-list = 백업이 손상되어 Trindle이 목록을 읽을 수 없어요
# The reason when the file is not a Flat backup.
store-because-not-flat = 그 파일은 Flat 백업으로 보이지 않아요
# The reason when the file holds no Flat list of apps.
store-because-no-flat-list = 그 파일에 Flat 앱 목록이 없어요
# The reason when the backup was made by a newer Flat than Trindle can read.
store-because-newer-flat = Trindle이 읽을 수 없는 더 새로운 버전의 Flat으로 만든 백업이에요
# The reason when the file holds no apps.
store-because-no-apps = 그 파일에 앱이 없어요
# The reason when the backup could not be read; the system's own reason follows.
store-because-not-read = 백업을 읽을 수 없어요: { $why }
# The reason when the program gave none.
store-because-none = 이유가 없어요
# The reason when the system said permission was denied.
store-because-permission = Trindle이 그곳에 쓸 수 있는 권한이 없어요
# The reason when the disk is full.
store-because-space = 이 컴퓨터에 남은 공간이 없어요
# The reason when a part of the computer is read-only.
store-because-read-only = 이 컴퓨터의 그 부분은 읽기 전용이에요
# The reason when the person's disk quota is used up.
store-because-quota = 이 계정에 허용된 공간을 다 썼어요
# The reason when a file or program is missing.
store-because-missing = 필요한 파일이나 프로그램이 없어요
# The reason when no reason could be read from the program's last line.
store-because-failed = 실패했어요
# The reason when the backup has no name to be written under.
store-because-no-name = 이름이 없어요
# The reason when a backup's settings for an app reach outside the app's own folder.
store-because-outside-own = 백업에 든 설정이 앱 자체 폴더 밖을 가리켜요
# The reason when something is too big to be read whole.
store-because-too-big = 너무 커서 읽을 수 없어요
# The reason when an app's settings in the backup do not match their checksum.
store-because-checksum = 백업에 든 설정이 손상되었거나 바뀌었어요
# Said when the person's account picture was not put back; the reason follows.
store-refusal-picture-only = 계정 사진을 뺀 모든 것을 복원했어요. { $why }
# Said when something else failed and the account picture was not put back either.
store-refusal-picture-too = { $words } Trindle이 계정 사진도 복원하지 못했어요. { $why }
# Said when the pkexec program could not be started.
store-refusal-pkexec = 비밀번호를 요청할 수 없었어요: { $why }
# Said when the backup could not be read at all.
store-refusal-not-read-from = Trindle이 백업에서 읽을 수 없었어요: { $why }.

## Making and putting back a backup (src/store/backup.rs)

# Said when a backup was not made; the reason follows "because".
store-backup-not-made = Trindle이 백업을 만들지 못했어요. 이유: { $why }.
# Beside the size of a part of the backup that is not there yet.
store-backup-nothing-yet = 아직 없음
# Beside the size of a part of the backup: about how big it is.
store-backup-about = 약 { $size }
# Step shown while Trindle's own settings and AppImages are put back.
store-backup-step-trindle = Trindle 설정과 AppImage를 복원하는 중…
# Said after a restore when some apps could not be installed because Flatpak refused them.
store-not-installed-failed = Trindle이 설정과 데이터를 복원했지만, Flatpak이 다음 앱을 설치하지 않았어요: { $failed }.
# Said after a restore when some apps were left out before Flatpak was asked.
store-not-installed-refused = Trindle이 설정과 데이터를 복원했지만, 다음 앱은 설치하지 않았어요: { $refused }.

## Restoring from a Flat backup (src/store/flat.rs)

# Said among what could not be done, when some apps were skipped because their names were not valid.
store-flat-skipped =
    { $n ->
       *[other] 이름이 올바르지 않아서 Trindle이 앱 { $n }개를 건너뛰었어요.
    }
# Said beside an app whose source is not secure: the source's name is given.
store-flat-not-secure = { $name } 소스가 암호화되어 있지 않아서 Trindle이 쓰지 않았어요
# Said when a source is not on this computer and the backup does not say where it is.
store-flat-no-address = { $name } 소스가 이 컴퓨터에 없고, 백업에도 위치가 없어요
# Said among what could not be done: an app's settings failed their checks.
store-flat-settings-checks = { $name }의 설정 (백업의 설정 사본이 검사를 통과하지 못했어요)
# Said among what could not be done: the name of an app's source is not valid.
store-flat-source-name = { $name } (소스 이름이 올바르지 않아요)
# Said among what could not be done: something and the reason it was not put back.
store-flat-failed-because = { $what } (이유: { $why })
# Said among what could not be done: the app Flatpak refused.
store-flat-flatpak-refused = { $name } (Flatpak이 설치하지 못했어요: { $why })
# Said among what could not be done when there was nowhere to unpack the backup.
store-flat-nowhere = 전부 (백업을 풀 곳이 없어요)
# Said among what could not be done: the settings that apply to every app.
store-flat-every-app = 모든 앱의 권한
# Said among what could not be done when a backup's permissions asked for more than the app had: only ones that take access away are put back.
store-flat-grants-left-out = { $what }: Trindle이 접근을 막는 권한만 복원했어요.
# Said among what could not be done: one app's settings.
store-flat-app-settings = { $name }의 설정
# Said when an app's settings could not be put back and the old folder was kept.
store-flat-kept-as = { $why }. 이전에 있던 것은 { $path }(으)로 남겨 두었어요: { $error }
# Step shown for each app being installed, with its place in the list.
store-flat-step-installing = { $name } 설치 중 ({ $total }개 중 { $at }번째)…
# Step shown for each app's settings being put back, with its place in the list.
store-flat-step-settings = { $name }의 설정을 복원하는 중 ({ $total }개 중 { $at }번째)…
# Summary of a restore from a Flat backup: how many apps were installed.
store-flat-summary-installed =
    { $n ->
       *[other] 앱 { $n }개를 설치했어요.
    }
# Summary of a restore: how many apps had their settings put back.
store-flat-summary-settings =
    { $n ->
       *[other] 앱 { $n }개의 설정을 복원했어요.
    }
# Summary of a restore: how many apps were already installed and left alone.
store-flat-summary-already =
    { $n ->
       *[other] 앱 { $n }개는 이미 설치되어 있었어요.
    }
# Summary of a restore: what could not be put back, as a list.
store-flat-summary-failed = 제외됨: { $list }
