# machine translated (Korean), not yet reviewed by a person.

# Settings: the Snapshots page (src/settings/panels/snapshots.rs).

## The Snapshots page (src/settings/panels/snapshots.rs)

# Name of the Snapshots page, in the page list, as its heading and on the card listing the snapshots.
settings-snap-title = 스냅샷
# Words people may search for to find the Snapshots page, separated by spaces.
settings-snap-keywords = 스냅샷 복원 되돌리기 롤백 백업 업데이트 고장 btrfs snapper 부팅 메뉴 grub 이전 버전 예전 버전 삭제한 파일 되돌려 놓기 복구 홈 복사본
# Help line at the foot of the Snapshots page.
settings-snap-help = 스냅샷은 소프트웨어를 바꾸기 전과 후마다, 그리고 직접 만들 때마다 저장되는 시스템의 복사본이에요. 스냅샷을 복원하면 다음에 다시 시작할 때 시스템 전체가 그때 모습으로 돌아가요. Trindle 설치 프로그램이 설정하는 대로 홈 폴더가 디스크의 별도 구역에 있으면 내 파일은 그대로예요. 컴퓨터가 시작되지 않으면 부팅 메뉴에서 Trindle 스냅샷을 고르세요. 이전 버전은 모든 사용자의 파일을 하루에 한 번 복사해 두며, 시스템 스냅샷과는 별개예요.
# Red words under the Earlier versions switch when a daily copy was skipped because the disk is nearly full.
settings-snap-nearly-full = 디스크가 거의 가득 차서 Trindle이 복사본을 만들지 않았어요.
# Time of a snapshot taken today: { $time } is the hour and minute.
settings-snap-today = 오늘 { $time }
# Time of a snapshot taken yesterday: { $time } is the hour and minute.
settings-snap-yesterday = 어제 { $time }
# Time of an older snapshot: { $date } is the day and month.
settings-snap-on = { $date }
# Red words when the computer would not restart on its own after a restore.
settings-snap-not-restart = 컴퓨터가 다시 시작되지 않았어요. 링에서 직접 다시 시작하세요.
# Said beside the Take a snapshot line while it works.
settings-snap-taking = 스냅샷 만드는 중
# Said beside the set aside line while the space is freed.
settings-snap-freeing = 공간 비우는 중…
# Said beside the set aside line once the space is freed.
settings-snap-freed = 비웠어요
# Said beside a snapshot while it is restored.
settings-snap-restoring = 복원하는 중
# Said beside a snapshot once it is restored: { $n } is the snapshot's number.
settings-snap-restored = 다음에 다시 시작하면 시스템이 스냅샷 { $n }번 때 모습이 돼요. 그 뒤에 바뀐 것은 따로 보관되고 사라지지 않아요.
# Said beside a snapshot while it is deleted.
settings-snap-deleting = 삭제하는 중
# Said beside a snapshot once it is deleted.
settings-snap-deleted = 삭제했어요
# Said beside a setting while the administrator's password is awaited.
settings-snap-waiting = 비밀번호를 기다리는 중…
# Said beside the Earlier versions switch once it is turned on.
settings-snap-copy-daily = 하루에 한 번 복사해요
# Said beside the Earlier versions switch once it is turned off.
settings-snap-no-more = 더 이상 복사하지 않아요
# Said beside the list of how long copies are kept once the choice is saved.
settings-snap-kept = 보관 기간
# Said beside the earlier copies while they are cleared.
settings-snap-clearing = 지우는 중
# Said beside the earlier copies once they are cleared.
settings-snap-cleared = 지웠어요
# Heading of the line about the earlier copies: how many there are.
settings-snap-copies =
    { $n ->
        [0] 복사본이 아직 없어요
       *[other] 복사본 { $n }개
    }
# Under the number of earlier copies when there are none.
settings-snap-first-soon = 첫 복사본은 하루 안에 만들어져요.
# Under the number of earlier copies: { $size } is the room they take and { $when } when it was measured.
settings-snap-they-take = { $size }을 차지해요. { $when }에 쟀어요.
# Under the number of earlier copies when their room has not been measured.
settings-snap-measured-daily = 차지하는 공간은 매일 재요.
# Tip of the button that clears all earlier copies, and its label while working.
settings-snap-clear-all = 지금 모두 지우기
# Heading of the card about earlier versions of files.
settings-snap-earlier = 이전 버전
# Help text of the Earlier versions card (the information button).
settings-snap-earlier-about = Trindle은 모든 사용자의 파일을 하루에 한 번 복사해 둘 수 있어요. 그러면 폴더에서 파일의 예전 버전을 열거나, 실수로 삭제한 파일을 되찾을 수 있어요. 복사본에서는 각 사용자가 그날 볼 수 있던 것만 보여서, 개인 홈 폴더는 계속 비공개예요. 그날 다른 사람도 열 수 있던 파일은 복사본이 삭제될 때까지 그 복사본에서 계속 열 수 있어요. 이 복사본은 시스템 스냅샷과 별개라서 서로 영향을 주지 않아요. 디스크 공간은 차지해요. 삭제하거나 바꾼 파일도 그 파일이 든 마지막 복사본이 삭제될 때까지 공간을 차지해요. 복사본을 지우면 모든 사용자의 공간이 바로 비워지고, 되돌릴 수 없어요. 디스크가 거의 가득 차면 복사본을 만들지 않아요. 이것을 바꾸려면 관리자 비밀번호가 필요해요.
# Label of the switch for keeping earlier versions, and heading of its line.
settings-snap-keep-earlier = 내 파일의 이전 버전 보관
# Under the keep earlier versions switch: { $time } is how long deleted files stay, such as a week.
settings-snap-deleted-stay = 삭제한 파일은 { $time } 동안 보관한 뒤 비워요.
# Heading of the line with the list of how long each day's copy is kept.
settings-snap-keep-each = 하루치 복사본 보관 기간
# Name of the list of how long each day's copy is kept.
settings-snap-how-long = 하루치 복사본을 보관하는 기간
# Headline of the Snapshots page while the snapshots are read.
settings-snap-looking = 스냅샷을 확인하는 중…
# Headline of the Snapshots page when the snapshots could not be read.
settings-snap-not-read = 스냅샷을 읽지 못했어요.
# Headline of the Snapshots page when the computer started from a snapshot: { $n } is its number.
settings-snap-started-from = 스냅샷 { $n }번으로 시작했어요. 지금 하는 일은 저장되지 않아요.
# Headline of the Snapshots page: how many snapshots there are and { $when } the time of the newest.
settings-snap-go-back =
    { $n ->
       *[other] 스냅샷 { $n }개로 돌아갈 수 있어요. 가장 최근 것은 { $when }이에요.
    }
# Headline of the Snapshots page when there are none yet.
settings-snap-none-yet = 아직 스냅샷이 없어요. 소프트웨어를 바꾸기 전에 하나씩 만들어져요.
# Under a snapshot's number and date when it is kept until deleted: { $what } is what it was taken for.
settings-snap-kept-until-deleted = { $what }, 삭제할 때까지 보관
# Tip of the button that restores the system to a snapshot, and its label while working.
settings-snap-restore-this = 이 스냅샷 복원
# Tip of the button that deletes a snapshot.
settings-snap-delete-this = 이 스냅샷 삭제
# Tip of the button that restarts the computer after a restore.
settings-snap-restart-now = 지금 다시 시작
# Under the heading when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-sub = 아직 설정되지 않았어요.
# Help text of the No snapshots card when snapshots are not set up on a Btrfs computer.
settings-snap-not-set-up-about = 스냅샷이 설정되지 않았어요. trindle 패키지를 다시 설치하면 설정돼요: sudo apt install --reinstall trindle.
# Name used for the kind of disk format when it is not known.
settings-snap-another-kind = 다른 종류
# Under the heading when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs = 스냅샷에는 Btrfs가 필요해요. 이 컴퓨터는 { $fs }를 써요.
# Help text of the No snapshots card when the computer does not use Btrfs: { $fs } is the kind it uses.
settings-snap-need-btrfs-about = 스냅샷을 쓰려면 시스템이 Btrfs에 있어야 하는데, 이 컴퓨터는 { $fs }를 써요. 스냅샷을 쓰려면 Debian을 설치할 때 시스템에 Btrfs를 고르세요.
# Headline of the Snapshots page when the computer has none.
settings-snap-no-snapshots = 이 컴퓨터에는 스냅샷이 없어요.
# Heading of the card shown when the computer has no snapshots.
settings-snap-no-snapshots-title = 스냅샷 없음
# Heading of the line shown when the computer has no snapshots.
settings-snap-no-snapshots-line = 이 컴퓨터에 스냅샷 없음
# Shown in place of the list while the snapshots are read.
settings-snap-looking-short = 불러오는 중…
# Heading of the line shown when the snapshots could not be read; the reason is under it.
settings-snap-not-read-line = 스냅샷을 읽지 못했어요
# Heading of the line shown when there are no snapshots yet.
settings-snap-none-line = 아직 없어요
# Help text of the Snapshots card (the information button).
settings-snap-list-about = 스냅샷마다 다음에 다시 시작할 때 시스템 전체를 그때 모습으로 되돌릴 수 있어요. 복원하거나 삭제하려면 관리자 비밀번호가 필요해요.
# Tip of the button that takes a snapshot now, and heading of its line.
settings-snap-take-now = 지금 스냅샷 만들기
# Under the Take a snapshot now heading when the computer started from a snapshot.
settings-snap-not-while = 이렇게 하려면 평소처럼 다시 시작하세요.
# Heading of the line about systems set aside by restoring: how many there are.
settings-snap-set-aside =
    { $n ->
       *[other] 복원 뒤 따로 보관한 옛 시스템 { $n }개
    }
# Tip of the button that frees the space the set aside systems use.
settings-snap-free-them = 차지한 공간 비우기
# Heading of the card for taking a snapshot.
settings-snap-take-one = 만들기
# Name of the switch that keeps the system's own files read-only.
settings-snap-locked = 시스템 파일 잠금
# Added to the Take one card's i.
settings-snap-locked-sub = 설치와 업데이트만 잠금을 풀어요.
# Said beside the System files locked switch once it is turned on.
settings-snap-locked-now = 잠김
# Said beside the System files locked switch once it is turned off.
settings-snap-unlocked-now = 풀림
# Help text of the Take one card (the information button).
settings-snap-take-about = 되돌리고 싶을 수도 있는 일을 해 보기 전에 스냅샷을 만드세요. 직접 만든 스냅샷은 삭제할 때까지 보관돼요. 스냅샷으로 시작했다면 먼저 평소처럼 다시 시작하세요. 그렇지 않으면 새 스냅샷이 보관되지 않아요. 복원할 때마다 바로 직전의 시스템이 따로 보관되니 잃는 것은 없어요. 시스템 파일을 잠그면 설치와 업데이트만 파일을 바꿀 수 있고, 그때마다 전과 후에 스냅샷이 만들어져요. 잠금을 바꾸려면 관리자 비밀번호가 필요해요.
# Heading of a line on the Before restoring card: { $n } is the number of the snapshot started from.
settings-snap-started-line = 스냅샷 { $n }번으로 시작함
# Under the Started from snapshot heading.
settings-snap-nothing-kept = 바뀐 내용은 저장되지 않아요.
# Heading of a line on the Before restoring card.
settings-snap-files-too = 내 파일도 스냅샷에 있어요
# Under the Your files are in the snapshots too heading.
settings-snap-files-back = 복원하면 내 파일도 되돌아가요.
# Heading of the card about what to know before restoring.
settings-snap-before = 복원하기 전에
# Help text of the Before restoring card (the information button).
settings-snap-before-about = 스냅샷으로 시작했기 때문에 지금 하는 일은 저장되지 않아요. 시스템을 이대로 두려면 목록에서 그 스냅샷 옆의 이 스냅샷 복원을 누른 뒤 다시 시작하세요. 이 컴퓨터에서는 홈 폴더가 시스템의 일부일 수 있어요. 그렇다면 스냅샷을 복원할 때 내 파일도 그때 모습으로 돌아가니, 새로 만든 파일은 먼저 안전한 곳에 복사하세요.
