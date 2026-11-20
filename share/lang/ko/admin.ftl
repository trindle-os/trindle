# machine translated (Korean), not yet reviewed by a person.

# Words of the administrator's helper (trindle-admin), which Settings and Apps show when a step behind the administrator's password fails or is refused. The 'usage:' lines it prints to its maker, and the words it hands to other programs to read, are not here.

## Accounts and passwords (src/admin/mod.rs)

# Red words in Settings when a new person's login name has letters, digits or marks that are not allowed.
admin-login-invalid = 올바른 로그인 이름이 아니에요.

# Red words in Settings when a new person's login name is longer than 32 letters.
admin-login-too-long = 로그인 이름이 너무 길어요.

# Red words in Settings when the account asked about does not exist. { $name } is the login name.
admin-user-none = { $name }(이)라는 사용자가 없어요.

# Red words in Settings when the account asked about is the system's own and not a person's. { $name } is the login name.
admin-user-system = { $name }은(는) 시스템 계정이라 Trindle이 바꾸지 않았어요.

# Red words in Settings when no password reached the helper that sets it.
admin-password-none = 비밀번호가 입력되지 않았어요.

# Red words in Settings when a new password is too short. { $n } is the fewest letters allowed.
admin-password-short =
    { $n ->
       *[other] 비밀번호는 { $n }자 이상이어야 해요.
    }

# Red words in Settings when a new password has a line break in it.
admin-password-line-break = 비밀번호에는 줄바꿈을 넣을 수 없어요.

# Red words in Settings when the login name chosen for a new person is taken. { $name } is the login name.
admin-user-exists = { $name }은(는) 이미 있어요.

# Red words in Settings when the only administrator is asked to stop being one. { $name } is the login name.
admin-user-only-admin = { $name }은(는) 유일한 관리자라서 계속 관리자여야 해요.

# Red words in Settings when a person tries to delete the account they are signed in to.
admin-delete-self = 내가 로그인한 계정은 삭제할 수 없어요.

# Red words in Settings when an account was deleted but its picture on the login screen could not be removed. { $name } is the login name; { $why } is the system's reason.
admin-delete-picture-kept = Trindle이 { $name } 계정을 삭제했지만 로그인 화면 사진은 지우지 못했어요: { $why }

# Red words in Settings when the picture chosen for the login screen is too big.
admin-picture-large = 사진이 너무 커요.

# Red words in Settings when the picture chosen for the login screen is not one Trindle made.
admin-picture-not-ours = Trindle이 만든 사진이 아니에요.

# Red words in Settings when the folder of login pictures could not be opened to the login screen. { $why } is the system's reason.
admin-picture-folder = Trindle이 로그인 화면에서 사진 폴더를 열 수 있게 하지 못했어요: { $why }

## Language, computer name and charging (src/admin/mod.rs)

# Red words in Settings when the language chosen has a name that cannot be one.
admin-language-invalid = 언어 이름이 올바르지 않아요.

# Red words in Settings when Debian's list of languages could not be read. { $why } is the system's reason.
admin-language-list = Trindle이 Debian의 언어 목록을 읽지 못했어요: { $why }

# Red words in Settings when the language chosen is not one Debian knows. { $code } is the language's code.
admin-language-unknown = Debian이 { $code } 언어를 알지 못해요.

# Red words in Settings when the computer name typed is not allowed.
admin-hostname-invalid = 컴퓨터 이름에는 글자, 숫자, 하이픈만 쓸 수 있어요. 63자까지 가능하고 하이픈으로 시작하거나 끝날 수 없어요.

# Red words in Settings when the 80% charging limit is on but could not be kept for the next start. { $why } is the system's reason.
admin-charge-limit-lost = 충전 제한은 지금은 켜져 있지만 컴퓨터를 끄면 사라져요: { $why }

# Red words in Settings when charging was set to full but the limit could not be taken off for good. { $why } is the system's reason.
admin-charge-limit-returns = 완전 충전은 지금은 켜져 있지만 컴퓨터를 다시 켜면 제한이 돌아와요: { $why }

# Red words in Settings when a radio's block would not come off. { $words } is why; { $said } is what the system added.
admin-radio-refused = { $words } { $said }

## Sharing and plugins through the firewall (src/admin/mod.rs)

# Red words in Settings when the switch that lets other computers connect could not be put off. { $why } is the system's reason.
admin-sharing-still-on = Trindle이 공유를 끄지 못해서 다른 컴퓨터가 계속 연결할 수 있어요: { $why }

# Red words in Settings when a plugin's id is not one.
admin-plugin-id-invalid = 플러그인 ID가 올바르지 않아요.

# Red words in Settings when the person a firewall step is for is not known.
admin-person-unknown = 그런 사람이 없어요.

# Red words in Settings when the folder of the person a step is for is not known.
admin-person-folder-unknown = 그 사람의 폴더가 없어요.

# Red words in Settings when the list of plugins let through the firewall could not be kept. { $why } is the system's reason.
admin-plugins-list-not-kept = Trindle이 허용된 플러그인 목록을 저장하지 못했어요: { $why }.

# Red words in Settings when the plugin a firewall step is for was not found.
admin-plugin-not-found = 그런 플러그인이 없어요.

## Updates, apps, firmware and Tailscale (src/admin/mod.rs)

# Red words in Settings and Apps when Debian's updates were not installed because Trindle's own version could not be read.
admin-update-version-unknown = 업데이트하지 않았어요. Trindle이 설치된 자기 버전을 알 수 없어서 자기 업데이트를 빼놓을 수 없었어요.

# First line of the progress while Debian's updates go in.
admin-step-installing-updates = 업데이트 설치 중

# Last line of the progress after updates go in: what nothing needs any more is taken away.
admin-step-tidying = 정리 중

# First line of the progress while Trindle's own update goes in.
admin-step-installing-trindle = Trindle 설치 중

# Red words when the name of the package to remove is not a package's name.
admin-package-invalid = 패키지 이름이 올바르지 않아요.

# Red words when the package to remove is not installed.
admin-package-not-installed = 그 패키지는 설치되어 있지 않아요.

# Red words in Settings when the firmware device asked for is not one fwupd knows.
admin-device-invalid = fwupd가 아는 장치가 아니에요.

# Red words when installing a package failed and fetching the lists failed too. { $first } is why the install failed; { $lists } is why the lists failed.
admin-install-and-lists = { $first }; { $lists }

# Red words in Settings when Tailscale cannot be turned on because it is not known who asked.
admin-tailscale-who = Trindle이 누가 Tailscale을 원하는지 알 수 없어요.

## Blocking sites and tools it runs (src/admin/mod.rs)

# Red words in Settings when the site blocking did not start in time.
admin-blocking-late = 차단이 제때 시작되지 않았어요.

# Red words in Settings when turning site blocking off left something on. { $why } lists what.
admin-blocking-not-off = 차단이 완전히 꺼지지 않았어요: { $why }

# Part of the red words when turning site blocking off could not put a file back. { $path } is the file; { $why } is the system's reason.
admin-file-not-put-back = Trindle이 { $path }을(를) 되돌려 놓지 못했어요: { $why }

# Red words in Settings when the site typed to never block is not a site name.
admin-site-invalid = 사이트 이름이 올바르지 않아요.

# Red words in Settings when the private server chosen for lookups is not one.
admin-server-unknown = 그런 서버가 없어요.

# Red words when a tool a step runs could not be started. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-error = { $tool }: { $why }

# Red words when a tool stopped before it was given everything. { $tool } is the tool's name; { $why } is the system's reason.
admin-tool-stopped = { $tool }이(가) 필요한 것을 다 받기 전에 멈춰서 작업이 끝나지 않았어요: { $why }

# Red words when a tool a step runs failed and said nothing. { $tool } is the tool's name.
admin-tool-failed = { $tool }이(가) 실패했어요.

# Red words when a file could not be taken away. { $path } is the file; { $why } is the system's reason.
admin-file-not-removed = { $path }을(를) 지우지 못했어요: { $why }

# Red words when a system file could not be read, so nothing was changed. { $why } is the system's reason; { $path } is the file.
admin-file-not-read = Trindle이 { $path }을(를) 읽지 못해서 아무것도 바꾸지 않았어요: { $why }

# Red words when a step failed and what it had done could not be put back either. { $why } is the first failure; { $problem } is the second.
admin-undone-failed = { $why } Trindle이 원래대로 되돌리지도 못했어요: { $problem }

## Earlier versions of the home folder (src/admin/home_snapshots.rs)

# Red words in Settings when the time chosen for keeping earlier versions is not one on offer.
admin-home-keep-invalid = 이전 버전을 보관하는 기간이 올바르지 않아요.

# Red words in Settings when the folder the earlier versions are kept in is not there.
admin-home-folder-missing = 이전 버전을 찾을 수 없어요.

# Red words in Settings when something that is not a folder is where the earlier versions would go.
admin-home-folder-blocked = 이전 버전 폴더가 있어야 할 자리에 파일이 있어요.

## Shared folders on other computers (src/admin/shares.rs)

# Red words in Folders when it is not known who asked for a step.
admin-caller-unknown = 누가 요청했는지 알 수 없어요.

# Red words when a step that must go through the administrator's password was started another way.
admin-pkexec-only = pkexec로만 실행할 수 있어요.

# Red words in Folders when the name of the computer to connect to is not a computer's name.
admin-share-computer-invalid = 컴퓨터 이름이 올바르지 않아요.

# Red words in Folders when the name of the shared folder is not one.
admin-share-name-invalid = 공유 폴더 이름이 올바르지 않아요.

# Red words in Folders when the address of the computer is not an address.
admin-share-address-invalid = 컴퓨터 주소가 올바르지 않아요.

# Red words in Folders when the path of the shared folder is not one.
admin-share-path-invalid = 공유 폴더 경로가 올바르지 않아요.

# Red words in Folders when the shared folder to close is not one of the person's own.
admin-share-not-yours = 여기서는 내 공유 폴더만 닫을 수 있어요.

# Red words in Folders when no shared folder is open at the place named.
admin-share-not-open = 그 자리에 열린 공유 폴더가 없어요.

# Red words in Folders when the folder shares open in has been changed by something else. { $path } is the folder.
admin-share-folder-changed = Trindle이 설정한 뒤에 다른 것이 { $path }을(를) 바꿔서 아무것도 연결하지 않았어요.

# Red words in Folders when the private folder the login is kept in has been changed by something else. { $path } is the folder.
admin-share-private-changed = { $path }이(가) Trindle이 둔 상태가 아니에요.

# Red words when a folder could not be made. { $path } is the folder; { $why } is the system's reason.
admin-path-error = { $path }: { $why }

# Red words in Folders when no more shared folders can be opened.
admin-share-too-many = 열려 있는 공유 폴더가 너무 많아요.

# Red words in Folders when the program that connects to a share is not installed. { $tool } is its name; { $package } is the Debian package it comes in.
admin-tool-missing = { $tool }이(가) 설치되어 있지 않아요. Debian의 { $package } 패키지에 들어 있어요.

# Red words in Folders when the login for a share could not be read.
admin-login-unreadable = 로그인 정보를 읽지 못했어요.

# Red words in Folders when the list of what is open on this computer could not be read. { $why } is the system's reason.
admin-mounts-unreadable = Trindle이 이 컴퓨터에 연결된 항목을 읽지 못했어요: { $why }

# Red words in Folders when a shared folder cannot be closed because a program is using it.
admin-share-busy = 프로그램이 아직 이 폴더의 무언가를 쓰고 있어요. 프로그램을 닫고 다시 시도하세요.

# Red words in Folders when a shared folder's computer does not answer while it is being closed.
admin-share-not-answering = 그 컴퓨터가 응답하지 않아요. 로그아웃하면 공유 폴더가 닫혀요.

# Red words when the number given for a person is not a user number.
admin-user-id-invalid = 사용자 ID가 올바르지 않아요.

## Sharing folders from this computer (src/admin/sharing.rs)

# Red words in Settings when the group that may share folders could not be made.
admin-sharing-group = sambashare 그룹을 만들지 못했어요.

# First line of the progress while sharing is set up: the sharing software is fetched.
admin-step-fetching-samba = Samba 가져오는 중

# Last line of the progress while sharing is set up.
admin-step-setting-up-sharing = 공유 설정 중

# Red words in Settings when a sharing password is set before sharing is set up.
admin-sharing-not-set-up = 공유가 아직 설정되지 않았어요. 먼저 폴더를 공유하세요.

# Red words in Settings when this computer could not be hidden from others on the network. { $why } is the system's reason.
admin-sharing-still-shown = 이 컴퓨터가 아직 네트워크의 다른 컴퓨터에 보여요: { $why }

## Snapshots of the system (src/admin/snapshots.rs)

# Red words in Settings when the snapshot asked for has a number that cannot be one.
admin-snapshot-number-invalid = 스냅샷 번호가 올바르지 않아요.

# Name given to a snapshot when the person did not type one; it is shown in the list of snapshots.
admin-snapshot-description = 설정에서 만듦

# Red words in Settings when it could not be told which disk the system is on.
admin-disk-unknown = Trindle이 시스템이 어느 디스크에 있는지 알 수 없어요.

# Red words in Settings when the system cannot be swapped for a snapshot because it is not in a subvolume of its own.
admin-system-subvolume = 시스템이 자체 Btrfs 서브볼륨에 있지 않아서 Trindle이 스냅샷으로 바꿀 수 없어요.

## Extra software sources and their programs (src/admin/sources.rs)

# Red words in Apps when a source's signing key could not be fetched. { $url } is the address.
admin-key-fetch = Trindle이 { $url }에서 서명 키를 가져오지 못했어요.

# Red words in Apps when the address given for a signing key gave back something else.
admin-key-invalid = 그 주소에서 서명 키가 오지 않았어요.

# Red words in Apps when a source's file could not be read, so nothing was changed. { $path } is the file; { $why } is the system's reason.
admin-source-file-unreadable = Trindle이 { $path }을(를) 읽지 못해서 아무것도 바꾸지 않았어요: { $why }

# First line of the progress while a source is added: the signing key is fetched.
admin-step-fetching-key = 서명 키 가져오는 중

# Second line of the progress while a source is added.
admin-step-adding-source = 추가하고 목록 읽는 중

# Red words in Apps when a source could not be used and an earlier one was kept. { $why } is the reason.
admin-source-unusable = 출처를 사용할 수 없어요. { $why }

# Red words in Apps when a source could not be used and so was taken away again. { $why } is the reason.
admin-source-unusable-removed = Trindle이 출처를 사용할 수 없어서 다시 제거했어요. { $why }

# Red words in Apps when the person could not be let switch Tailscale on or off. { $user } is the login name; { $why } is the reason.
admin-source-tailscale-operator = { $user }은(는) 아직 켜거나 끌 수 없어요: { $why }

# Red words in Apps when Syncthing could not be started for a person. { $user } is the login name; { $why } is the reason.
admin-source-syncthing = Trindle이 { $user }에게 시작해 주지 못했어요: { $why }

# Red words in Apps when the person could not be let use virtual machines, which Claude's Cowork needs. { $user } is the login name; { $why } is the reason.
admin-source-claude-kvm = Trindle이 { $user }이(가) Cowork에 필요한 가상 머신을 쓸 수 있게 하지 못했어요: { $why }

# Red words in Apps when the person could not be let use Docker without a password. { $user } is the login name; { $why } is the reason.
admin-source-docker = Trindle이 { $user }이(가) 비밀번호 없이 쓸 수 있게 하지 못했어요: { $why }

# Red words in Apps when a program was installed but its setting up did not finish. { $name } is the program; { $why } is the reason.
admin-source-set-up-failed = { $name }은(는) 설치됐지만 Trindle이 설정을 끝내지 못했어요: { $why }

# Line of the progress while a program is installed.
admin-step-installing-it = 설치 중

# Red words in Apps when the source asked for is not one of the ready-made ones.
admin-source-unknown = 준비된 출처 중 하나가 아니에요.

# Red words in Apps when a program is not made for this kind of computer. { $name } is the program; { $arch } is the kind of processor.
admin-source-arch = { $name }은(는) 이 종류의 컴퓨터({ $arch })용이 아니에요.

# Last line of the progress while a program is installed.
admin-step-setting-up = 설정 중

# Red words in Apps when the address of a custom source is not an https address.
admin-source-address = 주소는 https://로 시작하는 일반 웹 주소여야 해요.

# Red words in Apps when the suite or components of a custom source are not plain words.
admin-source-words = 스위트와 구성 요소는 일반 단어여야 해요.

# Red words in Apps when the package named for a custom source is not a package name.
admin-source-package = 패키지 이름이 올바르지 않아요.

# First line of the progress while a source and its program are taken away.
admin-step-removing = 제거 중

# Last line of the progress after a source is added or taken away: the lists are read again.
admin-step-reading-lists = 목록 다시 읽는 중

# Red words in Apps when the source to remove was not added by Trindle.
admin-source-not-ours = 여기서는 Trindle이 추가한 출처만 제거할 수 있어요.

# Red words in Apps when a source could not be taken away. { $why } is the reason.
admin-source-still-there = 출처가 아직 남아 있어요: { $why }

## Restoring a snapshot (src/admin/snapshots.rs)

# Red words in Settings when the snapshot to restore is not on the disk. { $n } is the snapshot's number.
admin-snapshot-missing = 스냅샷 { $n }이(가) 디스크에 없어요.

# Red words in Settings when a restore did not begin because the snapshot could not be made ready. { $why } is the reason.
admin-restore-not-ready = Trindle이 스냅샷을 준비하지 못해서 아무것도 바꾸지 않았어요. { $why }

# Red words in Settings when the present system could not be set aside during a restore. { $why } is the reason.
admin-restore-aside = Trindle이 현재 시스템을 따로 옮겨 두지 못했어요: { $why }

# Red words in Settings when a restore failed and the present system could not be put back under its name. { $aside } is the name it now has; { $sub } is the name it needs; { $why } is the reason.
admin-restore-stranded = 현재 시스템의 이름이 { $sub } 대신 { $aside }(으)로 되어 있어요. 이름을 { $sub }(으)로 되돌리기 전에는 컴퓨터가 시작되지 않아요: { $why }

# Red words in Settings when a restore swapped the systems but could not finish tidying up. { $why } is the reason.
admin-restore-finish-later = 스냅샷이 적용됐고 컴퓨터는 그것으로 시작해요. Trindle이 이전 시스템 정리를 끝내지 못했어요. 컴퓨터를 다시 켤 때 다시 시도해요: { $why }

# Red words in Settings when the earlier snapshots could not be moved into the restored system. { $why } is the reason.
admin-restore-snapshots-across = Trindle이 스냅샷을 옮기지 못했어요: { $why }

# Red words in Settings joining two reasons a restore could not be undone. { $first } and { $second } are the two reasons.
admin-both-failures = { $first }; { $second }

# Red words in Settings when the restored system could not be put in place. { $why } is the reason.
admin-restore-in-place = Trindle이 스냅샷을 제자리에 두지 못했어요: { $why }

# Red words in Settings when the list of systems set aside could not be brought up to date. { $why } is the reason.
admin-restore-list = Trindle이 따로 옮겨 둔 시스템 목록을 갱신하지 못했어요: { $why }

# Red words in Settings when systems set aside could not be removed. { $names } lists them.
admin-restore-not-removed = { $names }을(를) 지우지 못했어요

# Last line of an update's tidying when it took programs away: which, and that nothing needed them. Help address updates-removed.
admin-tidied-away = 더 이상 필요하지 않아서 Trindle이 이것들을 제거했어요: { $list }.

# Red words in Settings and Apps when an update ended with packages still kept back. { $list } names them. Help address updates-kept-back.
admin-update-kept-back = 모든 업데이트가 설치되지는 않았어요. 이것들은 보류됐어요: { $list }.

# Red words after an update when a driver (DKMS) would not build for the new kernel. The kernel and the rest of the update went in. { $driver } is the driver, { $feature } what is off. Help address updates-driver-off. Keep "DKMS) will not build": Help finds its page by those words.
admin-driver-off-known = 새 Linux 커널용 드라이버({ $driver }, DKMS)가 아직 빌드되지 않아서 { $feature }이(가) 꺼져 있어요.
admin-driver-off = 새 Linux 커널용 드라이버({ $driver }, DKMS)가 아직 빌드되지 않아요. 이 드라이버가 필요한 기능은 꺼져 있어요.
# Said once after the drivers that are off.
admin-driver-off-rest = 커널과 나머지 업데이트는 설치됐어요.
# The feature that the v4l2loopback driver gives.
admin-driver-virtual-camera = 가상 카메라

admin-language-in-use = { $code }은(는) 이 컴퓨터가 시작할 때 쓰는 언어라서 제거할 수 없어요.
# The kernel or chip has no split-lock slowing to turn off.
admin-split-lock-none = 이 컴퓨터에는 끌 수 있는 split-lock 감속이 없어요.
# The drive chosen for games is not there, or is the computer's own.
admin-games-drive-not-found = Trindle이 그 드라이브를 찾지 못했거나 시스템 드라이브예요.
# The drive could not be closed where it was open.
admin-games-drive-busy = 드라이브를 쓰는 중이에요. 그 드라이브의 파일과 프로그램을 닫고 다시 시도하세요: { $why }
# Another software change has the read-only system open.
admin-system-busy = 다른 소프트웨어 변경이 진행 중이에요. 끝난 뒤에 다시 시도하세요.
# The read-only system could not be opened for a software change; nothing was changed.
admin-system-not-opened = Trindle이 시스템 잠금을 풀지 못해서 아무것도 바꾸지 않았어요: { $why }
# The read-only system could not be locked again after a change.
admin-system-left-open = Trindle이 시스템을 다시 잠그지 못했어요. 다음에 시작할 때 잠겨요: { $why }
# The system was locked, but /usr/local could not be kept writable on top of it.
admin-system-local-locked = Trindle이 /usr/local을 쓸 수 있게 유지하지 못했어요: { $why }
# A command that opens the system outside a software change, run other than by root itself.
admin-system-root-only = root만 실행할 수 있고 pkexec로는 안 돼요.
# Switching the read-only system on where the system's disk is not Btrfs.
admin-system-needs-btrfs = Btrfs가 필요해요.

# Red words in Settings when too many ports are open already. { $most } is a number.
admin-ports-too-many = 포트는 최대 { $most }개까지 열 수 있어요.

# Red words in Settings when the list of open ports could not be kept. { $why } is the system's reason.
admin-ports-not-kept = Trindle이 열린 포트 목록을 저장하지 못했어요: { $why }.

# Red words in Settings when the firewall could not be switched off. { $why } is the system's reason.
admin-firewall-still-on = 방화벽이 아직 켜져 있어요: { $why }
# Red line when a plugin now asks for other network ports than the ones the administrator's password was typed for.
admin-plugin-ports-changed = 이 플러그인이 허용하신 것과 다른 네트워크 포트를 요청해요. 포트를 열려면 다시 허용하세요.
# Red words when a program is asked to be set up for someone other than the person asking.
admin-source-not-you = 프로그램은 요청한 사람에게만 추가할 수 있어요.
# Red words when a snapshot is put back and the name for the system set aside is already used. 
admin-restore-name-taken = 방금 다른 복원이 같은 이름을 썼어요. 몇 초 기다렸다가 다시 시도하세요.

# Said when a shared folder can't be disconnected. { $path } is the folder, { $error } the system's reason.
admin-share-not-closed = { $path }의 연결을 끊지 못했어요: { $error }
# Said when shared folders don't answer while they are being disconnected. { $n } is how many.
admin-shares-left =
    { $n ->
       *[other] 공유 폴더 { $n }개가 응답하지 않아서 아직 연결되어 있어요. 컴퓨터를 끌 때 연결이 끊겨요.
    }

# Red words in Drives when a swap file could not be switched off before making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-off = 스왑 파일 { $path }을(를) 끄지 못했어요: { $why }
# Red words in Drives when a swap file could not be switched back on after making room. { $path } is the file, { $why } the system's reason.
admin-swap-not-on = 스왑 파일 { $path }을(를) 다시 켜지 못했어요: { $why }
# Red words when Drives asks about a swap file that the system's own settings don't name.
admin-swap-not-listed = { $path }은(는) 시스템의 스왑 파일이 아니에요.
# Red words when a swap file, or a folder above it, could be changed or read by someone other than the administrator. { $why } is that file or folder.
admin-swap-not-safe = 다른 사람이 { $why }을(를) 바꾸거나 읽을 수 있어서 Trindle이 스왑 파일 { $path }을(를) 쓰지 않아요.
# Red words in Drives when, after making room, Trindle couldn't find where the swap file now starts, which waking from hibernation needs.
admin-swap-resume-unread = 스왑 파일이 지금 어디 있는지 찾지 못해서 최대 절전 모드에서 깨어나지 못할 수 있어요.
# Said when the swap file moved and the running system couldn't be told: hibernation is refused until the computer restarts.
admin-swap-resume-restart = 다시 시작하면 최대 절전 모드가 다시 작동해요.
# Red words in Settings, Hardware, when Use the chip again is pressed while it is already running.
admin-chip-busy = 칩을 이미 설정하는 중이에요. 끝날 때까지 기다리세요.
# Red words in Settings, Hardware, when the disk password typed for Use the chip again is wrong.
admin-chip-wrong-password = 디스크 비밀번호가 아니에요.
# Red words in Settings, Hardware, after several wrong disk passwords in a row.
admin-chip-wait = 잘못된 비밀번호를 너무 많이 입력했어요. 1분 기다렸다가 다시 시도하세요.
# Red words in Settings, Hardware, when the chip unlocked the disk at the last start after all.
admin-chip-not-needed = 지난번 시작 때 칩이 디스크를 열었으니 할 일이 없어요.
# Red words in Settings, Hardware, when the disk is not the one the start-up screen noted.
admin-chip-changed = 컴퓨터를 시작한 뒤 디스크가 바뀌었어요. 다시 시작한 뒤 다시 시도하세요.
# Red words in Settings, Hardware, when the computer's TPM security chip can't be found.
admin-chip-none = 보안 칩을 찾지 못했어요. BIOS에서 켠 뒤 다시 시도하세요.
# Red words in Settings, Hardware, when clevis, the tool that sets up the chip, is missing.
admin-chip-no-clevis = 칩을 설정하는 도구인 clevis가 설치되어 있지 않아요.
# Red words in Settings, Hardware, when the disk has no room left for another key.
admin-chip-no-slot = 디스크에 키를 더 넣을 공간이 없어요.
# Red words in Settings, Hardware, when the chip couldn't be set up. The disk still opens with its password.
admin-chip-not-sealed = 칩을 설정하지 못했어요. 디스크는 비밀번호로 계속 열려요.
# Red words in Settings, Hardware, when the chip works again but its old key couldn't be removed.
admin-chip-old-kept = 칩은 다시 작동하지만 이전 키를 제거하지 못했어요.
