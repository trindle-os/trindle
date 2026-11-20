# machine translated (Korean), not yet reviewed by a person.

# Words of Drives (trindle-drives), the window that shows the drives in or plugged into this computer and changes them: the page, the cards, the floating boxes of changes, and how every change ends. Sizes such as 1.5 GB, percentages and the names of file systems (exFAT, ext4, NTFS, FAT) are written by the program and passed in.

## The page (src/disks/mod.rs)

# Title of the Drives window, the big title at the top of its page, and the title of the card shown when the drive service is missing.
disks-title = 드라이브

# Tip of the round i button beside the title, which opens the longer words about the page.
disks-about-tip = 이 페이지에 대해

# First paragraph of the longer words that open from the i beside the title.
disks-help-1 = 이 컴퓨터에 들어 있거나 연결된 모든 드라이브를 파티션으로 나눈 막대로 보여 줘요. 파티션의 너비는 크기를, 색의 진하기는 찬 정도를 나타내요. 가득 찬 파티션을 쉽게 찾을 수 있어요.

# Second paragraph of the longer words that open from the i beside the title.
disks-help-2 = 그림 아래에는 드라이브마다 카드가 있어요. 파티션을 열거나, 마운트하거나 마운트를 해제하거나, 드라이브를 안전하게 제거하거나, 전원을 끌 수 있어요. 파티션의 연필 버튼으로 이름을 바꾸고, 폴더를 고르고, 시작할 때 마운트하고, 크기를 바꾸고, 검사하고, 복구하고, 포맷하고, 삭제할 수 있어요. 빈 공간의 더하기 버튼은 새 파티션을 만들어요.

# Third paragraph of the longer words that open from the i beside the title.
disks-help-3 = 시스템이 실행되는 드라이브는 볼 수 있지만, 내용을 지우거나 옮기는 변경은 할 수 없어요. 어떤 드라이브든 변경할 때마다 매번 관리자 비밀번호를 물어봐요. 눈 버튼은 드라이브나 파티션을 Folders와 링에서 숨겨요. 여기에는 회색으로 계속 보이고, 잠기지는 않아요.

# Title of the main card, which draws every drive as a bar of its parts.
disks-your-drives = 내 드라이브

# Red sentence at the top of the page when the list of drives could not be read at all.
disks-cannot-read = 지금은 드라이브를 읽을 수 없어요.

# Tip of the round button that asks for the list of drives again, on the card shown when the drive service is missing.
disks-try-again = 다시 시도

# Tip of the round button that asks for the list of drives again, shown with the line that says no drives were found.
disks-look-again = 다시 찾기

# Line shown on the main card when the computer lists no drive at all.
disks-no-drives = 드라이브를 찾지 못했어요. 하나 연결하면 여기에 나타나요.

# Name of the program that is missing, in the red words when the drive service cannot be reached.
disks-missing-service = 드라이브 서비스

# Name of the program that is missing, in the red words when the system bus cannot be reached.
disks-missing-bus = 시스템 버스

# Red words on the card shown when what Drives needs is not there. { $what } is the name of the missing program; { $package } is the package to install.
disks-missing = { $what }을(를) 사용할 수 없어요. 설치하려면 다음을 실행하세요: sudo apt install { $package }

## The sentence at the top (src/disks/mod.rs)

# Sentence at the top of the page while the first look at the drives is being taken.
disks-headline-loading = 드라이브를 살펴보는 중…

# Red sentence at the top of the page when a drive says it is failing. { $name } is the drive's name.
disks-headline-failing = { $name }에 고장이 났어요. 보관할 파일을 지금 다른 드라이브로 복사하세요.

# Sentence at the top of the page when a drive has worn-out or damaged spots. { $name } is the drive's name; { $troubles } lists what is wrong.
disks-headline-look = { $name }을(를) 확인해야 해요: { $troubles }.

# Sentence at the top of the page when nothing is wrong. { $plugged } is how many drives are plugged in from outside.
disks-headline-calm =
    { $plugged ->
        [0] 내 드라이브와 그 안의 공간이에요.
       *[other] 내 드라이브와 그 안의 공간이에요. 드라이브 { $plugged }개가 연결되어 있어요.
    }

# Red sentence at the top of the page when the computer's own disk is almost full. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-full =
    { $plugged ->
        [0] 이 컴퓨터가 거의 가득 찼어요. 남은 공간은 { $free }뿐이에요. 곧 파일을 지우거나 옮기세요.
       *[other] 이 컴퓨터가 거의 가득 찼어요. 남은 공간은 { $free }뿐이에요. 드라이브 { $plugged }개가 연결되어 있어요. 곧 파일을 지우거나 옮기세요.
    }

# Sentence at the top of the page when the computer's own disk is filling up. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-getting-full =
    { $plugged ->
        [0] 이 컴퓨터가 차 가고 있어요. 남은 공간은 { $free }이에요.
       *[other] 이 컴퓨터가 차 가고 있어요. 남은 공간은 { $free }이에요. 드라이브 { $plugged }개가 연결되어 있어요.
    }

# Sentence at the top of the page when the computer's own disk has plenty of room. { $free } is the free space; { $plugged } is how many drives are plugged in from outside.
disks-headline-plenty =
    { $plugged ->
        [0] 이 컴퓨터는 공간이 넉넉해요. 남은 공간은 { $free }이에요.
       *[other] 이 컴퓨터는 공간이 넉넉해요. 남은 공간은 { $free }이에요. 드라이브 { $plugged }개가 연결되어 있어요.
    }

## How full the places are (src/disks/mod.rs)

# Title of the card with the ring for the disk the system is on.
disks-ring-system = 이 컴퓨터

# Title of the card with the ring for the place that holds your own files.
disks-ring-files = 내 파일

# Sentence on a ring's card. { $free } is the free space; { $place } is system for the disk the system is on, or files for the place that holds your own files.
disks-ring-still-free =
    { $place ->
        [system] 시스템이 있는 디스크에 { $free }이 남아 있어요.
       *[files] 내 파일을 위한 공간이 { $free } 남아 있어요.
    }

# What a screen reader says for a ring. { $name } is the place; { $full } is how full it is as a percentage; { $free } and { $whole } are sizes.
disks-ring-tip = { $name }: { $full } 사용, { $whole } 중 { $free } 남음

# Words beside a ring, after how full it is. { $whole } is the size of the place.
disks-ring-of = 전체 { $whole }

## The drives together (src/disks/mod.rs)

# Health reading in the line under the picture when no drive has taken a reading yet.
disks-health-none = 드라이브가 아직 상태 정보를 보내지 않았어요

# Health reading in the line under the picture when the one drive that has taken a reading is fine.
disks-health-one = 드라이브 자체 검사 결과 정상이에요

# Health reading in the line under the picture when every drive is fine. { $n } is how many drives say so.
disks-health-all =
    { $n ->
       *[other] 드라이브 { $n }개가 모두 자체 검사를 통과했어요
    }

# Health reading in the line under the picture when some drives need a look. { $n } is how many.
disks-health-look =
    { $n ->
       *[other] { $n }개를 확인해야 해요: 카드를 보세요
    }

# Line under the picture of the drives. { $n } is how many drives there are; { $total } is their size together; { $health } says how healthy they are.
disks-together =
    { $n ->
       *[other] 드라이브 { $n }개, 합계 { $total }. { $health }.
    }

## The map of the drives (src/disks/mod.rs)

# Line under a part of a drive, in the picture and in its row. { $size } is how much is free.
disks-free = { $size } 남음

# Line under a part of a drive in the picture when it holds files but is not mounted.
disks-map-not-mounted = 마운트 안 됨

# Name of the free space between parts, in the picture and as the label of its row.
disks-free-space = 빈 공간

# Name of the only block drawn for a drive that has no disc or card in it.
disks-map-no-disc = 디스크나 카드가 없어요

# Name of the only block drawn for a drive that has nothing on it yet.
disks-map-empty = 아직 아무것도 없어요

# Line beside a drive's name in the picture, and under its name on its card, for a drive that plugs in. { $size } is the drive's size.
disks-drive-removable = { $size }, USB 또는 이동식

# Line beside a drive's name in the picture, and under its name on its card, for a drive inside the computer. { $size } is the drive's size.
disks-drive-internal = { $size }, 컴퓨터 안쪽

# Line beside a drive's name in the picture for a drive that plugs in and that has been hidden. { $size } is the drive's size.
disks-drive-removable-hidden = { $size }, USB 또는 이동식, 숨김

# Line beside a drive's name in the picture for a drive inside the computer that has been hidden. { $size } is the drive's size.
disks-drive-internal-hidden = { $size }, 컴퓨터 안쪽, 숨김

# What a screen reader says for a part in the picture that is filled. { $name } is the part; { $full } is how full it is as a percentage; { $words } is what is free.
disks-spoken-part-full = { $name }, { $full } 사용, { $words }

# What a screen reader says for a part in the picture. { $name } is the part; { $words } says what it is or how much is free.
disks-spoken-part = { $name }, { $words }

# What a screen reader says for a drive in the picture. { $name } is the drive; { $words } is its size and where it is; { $parts } is the list of its parts.
disks-spoken-drive = { $name }, { $words }: { $parts }.

## Parts and drives (src/disks/mod.rs)

# Name given to a part that has no label and no device name.
disks-part-name = 파티션

# Name given to a drive that gives no maker or model.
disks-drive-name = 드라이브

# A drive's name with its size, used in the warnings and under the floating boxes. { $name } is the drive; { $size } is its size.
disks-drive-words = { $name }, { $size }

# Kind of a part that is swap space.
disks-kind-swap = 스왑

# Kind of an encrypted part that is unlocked.
disks-kind-crypto-open = 암호화됨, 잠금 해제됨

# Kind of an encrypted part that is locked.
disks-kind-crypto-locked = 암호화됨, 잠김

# Kind of a part that holds other parts.
disks-kind-container = 파티션을 담고 있어요

# Kind of a part that has no filesystem on it.
disks-kind-none = 파일 시스템 없음

# A part's kind with what the computer uses it for. { $kind } is the kind; { $role } is efi, legacy or loader.
disks-kind-role =
    { $role ->
        [efi] { $kind }, EFI, 컴퓨터가 시작되는 파티션
        [legacy] { $kind }, 오래된 컴퓨터가 시작되는 파티션
       *[loader] { $kind }, 부트 로더 전용 파티션
    }

# One line in the list of kinds a part can be formatted as. { $name } is the name of the kind of file system; { $what } says what it is for.
disks-kind-line = { $name }: { $what }

# What the exFAT kind is for, in the list of kinds.
disks-kind-exfat-for = 모든 컴퓨터

# What the ext4 kind is for, in the list of kinds.
disks-kind-ext4-for = Linux 전용

# What the NTFS kind is for, in the list of kinds.
disks-kind-ntfs-for = Windows

# What the FAT kind is for, in the list of kinds.
disks-kind-vfat-for = 오래된 기기

## Health (src/disks/plan.rs and src/disks/mod.rs)

# Trouble a drive reports about its spare space.
disks-trouble-spare = 여유 공간이 부족해요

# Trouble a drive reports about its heat.
disks-trouble-temperature = 너무 뜨거워요

# Trouble a drive reports about wearing out, and what a drive says when some of its readings are failing.
disks-trouble-wearing-out = 수명이 다해 가요

# Trouble a drive reports when it has made itself read-only.
disks-trouble-readonly = 읽기 전용이 되었어요

# Trouble a drive reports about the memory it uses to back up its work.
disks-trouble-volatile = 메모리 백업이 고장 났어요

# Trouble a drive reports when part of it has made itself read-only.
disks-trouble-pmr = 일부가 읽기 전용이 되었어요

# Trouble a drive reports when it has been used past what its maker expects.
disks-trouble-worn = 예상 수명을 넘겼어요

# Trouble a drive reports about damaged spots. { $n } is how many.
disks-health-damaged =
    { $n ->
       *[other] 손상된 부분 { $n }곳
    }

# A drive's reading of its heat. { $degrees } is the number of degrees.
disks-health-temperature = { $degrees } °C

# A drive's reading of how long it has been used. { $n } is the number of hours; { $hours } is the same number written with its thousands marks.
disks-health-hours =
    { $n ->
       *[other] { $hours }시간 사용
    }

# How a drive says it is when it is failing, on its card.
disks-health-failing = 고장 났어요. 보관할 파일을 지금 옮기세요

# How a drive says it is when something is wrong, on its card. { $troubles } lists what is wrong.
disks-health-attention = 확인이 필요해요: { $troubles }

# How a drive says it is when all is well, on its card.
disks-health-ok = 정상

# The line on a drive's card about its health. { $state } is how it is; { $readings } are the readings that go with it.
disks-health-line = { $state }, { $readings }

## The card of a drive (src/disks/mod.rs)

# Line under a drive's name when its case answers but the drive inside it does not.
disks-no-answer = 응답 없음

# Red words on a drive's card when its case answers but the drive inside it does not.
disks-no-answer-help = 드라이브 케이스는 연결되어 있지만 안의 드라이브가 응답하지 않아요. 뽑고 10초 기다린 뒤, 전용 케이블로 파란색 또는 USB-C 포트에 다시 꽂으세요. 계속 이러면 안의 드라이브가 빠졌거나 고장 났을 수 있어요.

# Tip of the round button on a drive's card that takes the drive out safely.
disks-remove-safely = 안전하게 제거

# Tip of the round button on a drive's card that powers the drive off.
disks-power-off-tip = 전원 끄기

# Tip of the round button on a drive's card that opens the box for erasing the whole drive, and the name of that box's row and its button.
disks-erase-whole = 드라이브 전체 지우기

# Tip of the eye that shows a hidden drive or part again. { $what } is drive or part.
disks-eye-show =
    { $what ->
        [drive] 이 드라이브 보이기
       *[partition] 이 파티션 보이기
    }

# Tip of the eye that hides a drive or part. { $what } is drive or part; { $lock } is the note that hiding does not protect it.
disks-eye-hide =
    { $what ->
        [drive] 이 드라이브 숨기기. { $lock }
       *[partition] 이 파티션 숨기기. { $lock }
    }

# Note on the card of a hidden drive. { $lock } is the note that hiding does not protect it.
disks-hidden-note = Folders와 링에서 숨겨져 있어요. { $lock }

# Note on the card of the drive the system runs from.
disks-system-note = 시스템이 여기서 실행되므로 아무것도 지울 수 없어요.

# Note on the card of a drive that has no partition table.
disks-no-table = 이 드라이브에는 파티션 테이블이 없어서 전체가 비어 있어요.

## The rows of parts (src/disks/mod.rs)

# Part of the line under a part's name, when it holds files but is not mounted.
disks-part-not-mounted = 마운트 안 됨

# Part of the line under a part's name, when it is mounted at start-up. { $dir } is the folder.
disks-part-startup = 시작할 때 { $dir }에 마운트돼요

# Part of the line under a part's name, when it is not mounted at start-up but has a folder. { $dir } is the folder.
disks-part-appears = { $dir }에 나타나요

# Part of the line under a part's name, when it is mounted somewhere. { $dir } is the folder.
disks-part-in = { $dir } 안

# Tip of the round button that opens the folder a part is mounted in.
disks-open = 열기

# Tip of the round button that mounts a part.
disks-mount = 마운트

# Tip of the round button that unmounts a part.
disks-unmount = 마운트 해제

# Tip of the round button on a part that opens the box of changes.
disks-change-it = 변경

# Placeholder of the typing boxes for a part's name, and label of its row.
disks-name = 이름

# Placeholder of the typing box for the name a new or formatted part is given; it may be left empty.
disks-name-optional = 이름 (선택)

# Tip of the button that saves a part's new name.
disks-rename-tip = 이름 바꾸기

# Note under the Name row of a part.
disks-name-note = 여기와 다른 컴퓨터에서 보이는 이름이에요.

# Placeholder of the typing box for a part's folder.
disks-folder-name = 폴더 이름

# Tip of the button that saves a part's folder.
disks-use-folder = 이 폴더 사용

# Label of the row where a part's folder is chosen.
disks-folder = 폴더

# Note under the Folder row of a part.
disks-folder-note = 파일이 나타나는 곳이에요. 비워 두면 컴퓨터가 골라요.

# Label of the row with the switch for mounting a part at start-up, and the spoken name of that switch.
disks-mount-startup = 시작할 때 마운트

# Note under the row with the switch for mounting a part at start-up.
disks-startup-note = 컴퓨터가 시작될 때마다 마운트해요. 없어도 컴퓨터는 정상적으로 시작돼요.

# Spoken name of the sliders that choose a size in gigabytes, for resizing a part and for making a new one.
disks-size-gb = 크기(GB)

# Note under the Size row. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range = { $least }부터 { $most }까지예요.

# Note under the Size row when the smallest size is not known until the part is mounted. { $least } and { $most } are the smallest and largest sizes.
disks-resize-range-mount = { $least }부터 { $most }까지예요. 마운트하면 얼마나 줄일 수 있는지 볼 수 있어요.

# Tip of the button that resizes a part.
disks-resize-tip = 크기 바꾸기

# Label of the row where a part is resized.
disks-size = 크기

# Said above the Format and Erase buttons at all times. { $words } names what is erased.
disks-format-erases = { $words }의 모든 내용이 지워져요. 되돌릴 수 없어요.

# Warning shown after the first press of the resize button. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning = 한 번 더 누르면 { $drive }의 { $name } 크기를 { $from }에서 { $to }(으)로 바꿔요

# Warning shown after the first press of the resize button when the part gets smaller. { $name } is the part; { $drive } is the drive; { $from } is its size now; { $to } is the size asked for.
disks-resize-warning-shrink = 한 번 더 누르면 { $drive }의 { $name } 크기를 { $from }에서 { $to }(으)로 바꿔요. 줄이는 중에 전원이 꺼지면 파일을 잃을 수 있으니 먼저 파일을 백업하세요.

# Red words under the size when a Windows part was hibernated or left in fast start-up and so is not made smaller. { $name } is the part.
disks-windows-asleep = { $name }의 Windows가 최대 절전 모드이거나 빠른 시작 상태여서, 크기를 줄이면 파일을 잃을 수 있어요. Windows를 시작해 다시 시작을 고른 뒤 다시 시도하세요.

# Label of the row where room is made for a new part under a part that is in use.
disks-room-label = 새 파티션을 위한 공간

# Spoken name of the slider that chooses how much room to make.
disks-room-gb = 공간(GB)

# Tip of the button that makes room under a part that is in use.
disks-room-tip = 공간 만들기

# Warning shown after the first press of the make room button. { $name } is the part; { $drive } is the drive; { $size } is the room asked for.
disks-room-warning = 한 번 더 누르면 { $drive }의 { $name }에서 { $size }를 가져와요. Trindle이 먼저 스냅샷을 만들어요.

# Note under the row that makes room.
disks-room-note = Trindle이 스냅샷을 만든 다음, 사용 중인 파티션을 줄여요. 생긴 공간은 비워 둬요.

# Tip of the button that checks a part for faults.
disks-check-tip = 오류 검사

# Tip of the button that repairs a part.
disks-repair-tip = 복구

# Label of the row where a part is checked and repaired.
disks-faults = 오류

# Warning shown after the first press of the repair button. { $name } is the part; { $drive } is the drive.
disks-repair-warning = 한 번 더 누르면 { $drive }의 { $name }을(를) 복구해요

# Spoken name of the choice of the kind of file system when formatting or making a part.
disks-kind = 종류

# Label of the row where a part is formatted.
disks-format = 포맷

# Tip of the button that formats a part.
disks-format-tip = 포맷

# Warning shown after the first press of the format or erase button. { $words } names what is erased.
disks-format-warning = 한 번 더 누르면 { $words }의 모든 내용을 지워요

# Names a part in the warning before it is formatted. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-part-where = { $drive }의 { $name }, { $size }

# Spoken name of the button that deletes a part, which is also what it says after the first press. { $name } is the part; { $size } is its size; { $drive } is the drive.
disks-delete-name = { $drive }의 { $name }, { $size } 삭제

# Label of the row where a part is deleted.
disks-delete-label = 파티션 삭제

# Tip of the round button on free space that opens the box for making a new part.
disks-new-part-tip = 여기에 새 파티션 만들기

# Label of the row where a new part is made.
disks-new-part = 새 파티션

# Tip of the button that makes a new part.
disks-make-tip = 만들기

# Warning shown after the first press of the button that makes a new part. { $size } is the size asked for; { $drive } is the drive.
disks-make-warning = 한 번 더 누르면 { $drive }에 { $size } 새 파티션을 만들어요

## The floating box of changes (src/disks/mod.rs)

# Title of the box for erasing a whole drive. { $name } is the drive.
disks-sheet-erase = { $name } 지우기

# Line under the title of the box for a part. { $size } is the part's size; { $drive } is the drive.
disks-sheet-part = { $drive }의 { $size }

# Title of the box for making a new part in free space.
disks-sheet-new = 새 파티션

# Line under the title of the box for making a new part. { $size } is the free space; { $drive } is the drive.
disks-sheet-gap = { $drive }의 빈 공간 { $size }

# Tip of the round button that closes the box of changes.
disks-close = 닫기

## How a change goes (src/disks/mod.rs)

# Words on a row while a part is being mounted.
disks-mounting = 마운트하는 중

# Words on a row when a part has been mounted.
disks-mounted = 마운트됨

# Words on a row while a part is being unmounted.
disks-unmounting = 마운트 해제하는 중

# Words on a row when a part has been unmounted.
disks-unmounted = 마운트 해제됨

# Words on a drive's row while it is being closed so it can be removed.
disks-closing-it = 닫는 중

# Words on a drive's row when it is safe to remove.
disks-safe-remove = 안전하게 제거할 수 있어요

# Words on a drive's row while it is being powered off.
disks-powering-off = 전원을 끄는 중

# Words on a drive's row when it has been powered off.
disks-off-safe = 안전하게 뽑을 수 있어요

# Words on a row while a part is being renamed.
disks-renaming = 이름을 바꾸는 중

# Words on a row when a part has been renamed.
disks-renamed = 이름이 바뀌었어요

# Words on a row when the person emptied the folder box, so the computer chooses the folder.
disks-folder-auto = 이제 폴더를 자동으로 골라요

# Words on a row when a part has been given a folder. { $dir } is the folder.
disks-folder-appears = { $dir }에 나타나요

# Words on a row when a part is set to mount at start-up. { $dir } is the folder.
disks-startup-on = 시작할 때 { $dir }에 마운트돼요

# Words on a row when a part is set not to mount at start-up.
disks-startup-off = 시작할 때 마운트 안 됨

# Words on a row while a folder or start-up choice is being saved.
disks-saving = 저장하는 중

# Words on a row while a part is being resized.
disks-resizing = 크기를 바꾸는 중

# Words on a row when a part has been resized. { $size } is its new size.
disks-resized = { $size }(으)로 바뀌었어요

# Red words on the row that makes room, when the files to make smaller cannot be found.
disks-room-files-missing = Trindle이 파일을 찾지 못해서 공간을 만들 수 없어요.

# Words on a row while the snapshot is taken before room is made.
disks-snapshot-taking = 스냅샷을 만드는 중

# Red words when the snapshot taken before making room could not be found afterwards.
disks-snapshot-missing = Trindle이 나중에 스냅샷을 찾지 못했어요.

# Red words when the snapshot, or switching off the swap file, before making room failed. { $why } is why, as a sentence.
disks-snapshot-failed = { $why } 크기는 바뀌지 않았어요.

# Red words when things changed while the snapshot was being taken.
disks-snapshot-changed = Trindle이 스냅샷을 만드는 동안 무언가 바뀌어서 크기를 바꾸지 않았어요.

# Words on a row while room is being made.
disks-making-room = 공간을 만드는 중

# Words on a row while the swap file is switched off before room is made.
disks-swap-off-doing = 스왑 파일을 끄는 중

# Words on a row while the swap file is switched back on after room is made.
disks-swap-on-doing = 스왑 파일을 다시 켜는 중

# Red words when the swap file couldn't be switched back on after making room. { $done } is how making room went, { $why } why the swap file is still off, both as sentences.
disks-swap-still-off = { $done } 다시 시작할 때까지 스왑 파일은 꺼진 상태예요. { $why }

# Words on a row when room has been made. { $size } is how much.
disks-room-made = { $size } 공간을 만들었어요

# Words on a row while a part is being repaired.
disks-repairing = 복구하는 중

# Words on a row when a part has been repaired.
disks-repaired = 복구됨

# Red words when some faults could not be put right.
disks-repair-failed = Trindle이 일부 오류를 복구하지 못했어요. 보관할 파일을 다른 드라이브로 복사하세요.

# Words on a row while a part is being checked.
disks-checking = 검사하는 중

# Words on a row when the check found no faults.
disks-no-faults = 오류가 없어요

# Red words when the check found faults.
disks-faults-found = 오류를 찾았어요. 렌치 버튼을 눌러 복구하세요.

# Words on a row while a part or drive is being formatted.
disks-formatting = 포맷하는 중

# Words on a row when a part has been formatted.
disks-formatted = 포맷됨

# Words for the step of erasing a whole drive, in the account of a change that stopped part-way.
disks-erasing-whole = 드라이브 전체 지우기

# Red words when a freshly formatted part could not be found to give it its folder again.
disks-new-part-missing = 새 파티션을 찾을 수 없어요.

# Red words when the drive service could not be reached to give a formatted part its folder again.
disks-service-unreachable = 드라이브 서비스에 연결하지 못했어요.

# Red words when the identifier of a freshly formatted part could not be read.
disks-uuid-unread = Trindle이 새 파티션의 식별자를 읽지 못했어요.

# Words on a row while a part is being deleted.
disks-deleting = 삭제하는 중

# Words on a row when a part has been deleted.
disks-deleted = 삭제됨

# Words on a row while a new part is being made.
disks-making-it = 만드는 중

# Words on a row when a new part has been made.
disks-made = 만들었어요

# Red words under an eye when hiding or showing could not be kept. { $why } is the system's reason.
disks-hide-failed = 저장하지 못했어요: { $why }

# Words on a row while a change is being made. { $doing } says what is being done.
disks-working = { $doing }…

# Words on a row while a change is being made, after the window was asked to close. { $doing } says what is being done; { $closing } says the window stays open.
disks-working-stays = { $doing }… { $closing }

# Red words when the password could not be checked for. { $words } is that sentence; { $why } is the reason.
disks-not-sure-why = { $words } ({ $why })

## Decisions about names and sizes (src/disks/plan.rs)

# Red words under the folder name box when it is empty.
disks-name-empty = 먼저 이름을 지어 주세요.

# Red words under the folder name box when it is too long. { $max } is the most letters allowed.
disks-name-too-long =
    { $max ->
       *[other] 이름이 너무 길어요. { $max }자 이내로 써 주세요.
    }

# Red words under the folder name box when it has letters that are not allowed.
disks-name-letters = 글자, 숫자, - 와 _ 만 쓸 수 있어요 (공백 없음).

# Red words under the folder name box when it starts with a dash.
disks-name-start = 글자나 숫자로 시작하세요.

# Red words under the folder name box when the folder is taken. { $dir } is the folder.
disks-name-taken = { $dir }은(는) 이미 사용 중이에요.

# Red words under a part's name box when the name is too long for its kind of file system. { $max } is the most letters allowed.
disks-label-too-long =
    { $max ->
       *[other] 이 파티션에는 이름이 너무 길어요. { $max }자 이내로 쓰세요.
    }

# Red words under a part's name box when a FAT name has marks that are not allowed.
disks-label-fat-marks = FAT 이름에는 * ? . , ; : / \ | + = < > [ ] " 를 쓸 수 없어요.

# Red words under the Size row when the kind of part must be mounted before it can be resized. { $way } is larger or smaller.
disks-resize-mount-first =
    { $way ->
        [larger] 먼저 마운트하세요. 이 종류의 파티션은 마운트된 상태에서만 크게 만들 수 있어요.
       *[smaller] 먼저 마운트하세요. 이 종류의 파티션은 마운트된 상태에서만 작게 만들 수 있어요.
    }

# Red words under the Size row when the kind of part cannot be resized. { $way } is larger or smaller.
disks-resize-cannot =
    { $way ->
        [larger] 이 종류는 크게 만들 수 없어요.
       *[smaller] 이 종류는 작게 만들 수 없어요.
    }

## What a change says and what became of it (src/disks/plan.rs)

# Red words when the password box was closed with no password. Said whole, as a sentence.
disks-no-password = 비밀번호를 입력하지 않아서 아무것도 바뀌지 않았어요.

# Red words when the password box was closed with no password, after some steps of a change were already done.
disks-no-password-part-way = 비밀번호를 입력하지 않아서 작업이 중간에 멈췄어요.

# Red words when another change is pressed while one is still being made.
disks-change-going = 변경하는 중이에요.
disks-one-at-a-time = 다른 변경이 아직 진행 중이에요. 끝난 뒤에 다시 시도하세요.

# Red words when it could not be made sure that the password would be asked for.
disks-not-sure = Trindle이 비밀번호를 물어볼 수 있는지 확인하지 못해서 아무것도 바뀌지 않았어요.

# Red words after how a change ended, when the password just given could not be taken back.
disks-standing = Trindle이 방금 입력한 비밀번호를 취소하지 못했어요. 몇 분 동안은 드라이브를 다시 바꿀 때 비밀번호를 묻지 않을 수 있어요. Trindle이 다시 시도하고 있어요.

# Words after how a change ended, when the password just given has been taken back after all.
disks-taken-back = 입력한 비밀번호를 이제 취소했어요. 다음 변경에서는 비밀번호를 물어봐요.

# Red words when a change could not be started because there is no drive service to ask.
disks-no-service = Trindle이 드라이브 서비스에 연결하지 못해서 아무것도 바뀌지 않았어요.

# Red words said the next time Drives opens, about a change it was closed or ended in the middle of.
disks-closed = 이 변경을 하는 동안 드라이브 앱이 닫혔어요.

# Words on the row of a change still being made when the window is asked to close.
disks-stays-open = 이 작업이 끝날 때까지 드라이브 앱이 열려 있어요. 결과를 확인할 수 있어요.

# Red words for a refusal. { $reason } says why, without a full stop.
disks-nothing-changed = { $reason } 때문에 아무것도 바뀌지 않았어요.

# Red words for a refusal that came after some steps of a change were done. { $reason } says why, without a full stop.
disks-part-way = { $reason } 때문에 작업이 중간에 멈췄어요.

# Reason given when the drive service could not be asked, without a full stop.
disks-service-not-asked = 드라이브 서비스에 요청하지 못했어요

# Red words when a change stopped part-way for a reason that was not a refusal. { $why } is the reason, as a sentence.
disks-failed-part-way = { $why } 작업이 중간에 멈췄어요.

# Full stop put after words that end without one. { $words } are the words.
disks-sentence = { $words }.

# List of steps in the account of a change that stopped part-way, for the steps that were done. { $steps } lists them.
disks-list-done = 완료: { $steps }.

# List of steps in the account of a change that stopped part-way, for the step that failed. { $steps } lists it.
disks-list-failed = 실패: { $steps }.

# List of steps in the account of a change that stopped part-way, for the steps that were not done. { $steps } lists them.
disks-list-not-done = 하지 않음: { $steps }.

# List of steps in the account of a change that was cut off, for the step that may or may not have been done. { $steps } lists it.
disks-list-not-known = 알 수 없음: { $steps }.

# Red words about a part that was unmounted for a change that did not mount it again. { $dev } is the part.
disks-left-unmounted = { $dev }을(를) 이 작업을 위해 마운트 해제했고, 지금은 마운트되어 있지 않아요.

# Red words about a part that was unmounted for a change that did nothing to the drive. { $dev } is the part.
disks-still-unmounted = { $dev }을(를) 이 작업을 위해 마운트 해제했고, 아직 마운트 해제된 상태예요. 해당 행에서 마운트하세요.

# Red words about a part that the step that stopped may have unmounted. { $dev } is the part.
disks-may-be-unmounted = { $dev }이(가) 마운트 해제되었을 수 있어요. 해당 행을 확인하세요.

# Red words about a part that could not be mounted again because that needs a password. { $dev } is the part.
disks-put-back-password = { $dev }을(를) 이 작업을 위해 마운트 해제했어요. 해당 행에서 다시 마운트하고, 물어보면 비밀번호를 입력하세요.

# Red words about a part that could not be mounted again. { $dev } is the part; { $why } is why.
disks-put-back-failed = { $dev }을(를) 이 작업을 위해 마운트 해제했는데 다시 마운트하지 못했어요({ $why }). 해당 행에서 마운트하세요.

# Red words under a freshly formatted part whose folder could not be kept. { $dir } is the folder; { $why } is why, as a sentence.
disks-folder-not-kept = 포맷은 끝났지만 Trindle이 { $dir } 폴더를 다시 지정하지 못했어요. { $why } 해당 행에서 폴더를 다시 고르세요.

# Red words when a drive is in use and cannot be unmounted.
disks-busy = 아직 무언가 사용 중이에요. 이 드라이브의 파일을 보여 주는 창이나 프로그램을 닫고 다시 시도하세요.

# Red words when there is not enough room for what is on a part at the size asked for.
disks-no-room = 그 크기는 이미 들어 있는 파일에 비해 너무 작아요.

## Steps of a change, in the account of one that stopped part-way (src/disks/plan.rs)

# Step of a change. { $dev } is the part.
disks-step-unmount = { $dev } 마운트 해제

# Step of a change. { $dev } is the part.
disks-step-mount = { $dev } 다시 마운트

# Step of a change.
disks-step-eject = 드라이브 분리 준비

# Step of a change.
disks-step-power-off = 드라이브 전원 끄기

# Step of a change. { $dev } is the part.
disks-step-format = { $dev } 포맷

# Step of a change.
disks-step-create = 파티션 만들기

# Step of a change. { $dev } is the part.
disks-step-delete = { $dev } 삭제

# Step of a change. { $dev } is the part.
disks-step-resize-part = { $dev } 크기 조정

# Step of a change. { $dev } is the part.
disks-step-resize = { $dev }에 있는 내용 크기 조정

# Step of a change. { $dev } is the part.
disks-step-repair = { $dev } 복구

# Step of a change. { $dev } is the part.
disks-step-check = { $dev } 검사

# Step of a change. { $dev } is the part.
disks-step-rename = { $dev } 이름 변경

# Step of a change. { $dev } is the part.
disks-step-fstab = { $dev }의 폴더와 시작 설정 저장

# Step of a change that has no words of its own. { $method } is what the drive service was asked; { $dev } is the part.
disks-step-other = { $dev }의 { $method } 단계

## Making room under a part that is in use (src/disks/room.rs)

# Red words under the Make room row for a part of a kind that cannot be made smaller in use, and that can when it is not in use. { $kind } is the kind of file system.
disks-room-ext-offline = 이 { $kind } 파티션은 사용 중이 아닐 때만 줄일 수 있어서 Trindle이 여기서 공간을 만들 수 없어요.

# Red words under the Make room row for an xfs part.
disks-room-xfs-fixed = xfs 파티션은 줄일 수 없어요.

# Red words under the Make room row for any other kind of part in use. { $kind } is the kind of file system.
disks-room-other-kind = 이 { $kind } 파티션은 사용 중에는 줄일 수 없어요.

# Red words under the Make room row when a snapshot cannot be taken of the part.
disks-room-no-snapshot = Trindle은 시스템 자체 디스크의 스냅샷만 만들 수 있어서 여기서 공간을 만들 수 없어요.

# Red words under the Make room row.
disks-room-not-in-table = 드라이브의 파티션 표에 있는 파티션이 아니라서 Trindle이 여기서 공간을 만들 수 없어요.

# Red words under the Make room row.
disks-room-no-online-shrink = 드라이브 서비스가 사용 중인 Btrfs를 줄일 수 없어서 Trindle이 공간을 만들 수 없어요.

# Red words under the Make room row.
disks-room-holds-parts = 안에 다른 파티션이 있어서 Trindle이 여기서 공간을 만들 수 없어요.

# Red words under the Make room row.
disks-room-not-gpt = 이 드라이브에 GPT 파티션 표가 없어서 Trindle이 여기서 공간을 만들 수 없어요.

# Red words under the Make room row.
disks-room-mount-first = 먼저 마운트하세요. Btrfs는 마운트된 상태에서 줄여요.

# Red words under the Make room row.
disks-room-mounts-not-one = 마운트되어 있지 않아서 Trindle이 공간을 만들 수 없어요.

# Red words under the Make room row.
disks-room-read-only = 읽기 전용으로 마운트되어 있어서 Trindle이 공간을 만들 수 없어요.

# Red words under the Make room row.
disks-room-read-only-unknown = Trindle이 읽기 전용인지 확인하지 못해서 공간을 만들 수 없어요.

# Red words under the Make room row.
disks-room-many-drives = 파일이 드라이브 여러 개에 걸쳐 있어서 Trindle이 공간을 만들 수 없어요.

# Red words under the Make room row.
disks-room-drives-unknown = Trindle이 파일이 드라이브 몇 개에 걸쳐 있는지 확인하지 못해서 공간을 만들 수 없어요.

# Red words under the Make room row.
disks-room-swap-unknown = Trindle이 스왑 파일이 있는지 확인하지 못해서 공간을 만들 수 없어요.

# Red words under the Make room row.
disks-room-used-unknown = Trindle이 사용 중인 양을 읽지 못해서 공간을 만들 수 없어요.

# Red words under the Make room row.
disks-room-area-unknown = 드라이브 서비스가 파일 시스템 크기를 알려 주지 않아서 Trindle이 확인하거나 공간을 만들 수 없어요.

# Red words under the Make room row.
disks-room-open-missing = Trindle이 잠금 해제된 사본을 찾지 못해서 공간을 만들 수 없어요.

# Red words under the Make room row.
disks-room-encryption-sizes = 암호화에 대해 받은 크기가 서로 맞지 않아서 Trindle이 공간을 만들 수 없어요.

# Red words under the Make room row.
disks-room-sizes = 받은 크기가 서로 맞지 않아서 Trindle이 공간을 만들 수 없어요.

# Red words under the Make room row. { $used } is how much the files use.
disks-room-too-little = { $used }이(가) 들어 있고 그보다 많이 남겨 두어야 해서 비울 수 있는 공간이 너무 적어요.

# Red words under the Make room row.
disks-room-sizes-unread = 크기를 읽을 수 없었어요.

# Red words under the Make room row when the room asked for is out of range. { $least } and { $most } are the smallest and largest room.
disks-room-range = { $least }부터 { $most }까지 공간을 만들 수 있어요.

# Red words when making room was stopped. { $what } says what was found wrong, without a full stop.
disks-room-stopped = { $what } 때문에 Trindle이 멈췄어요.

# What was found wrong while making room.
disks-room-check-unmounted = 파일이 더 이상 마운트되어 있지 않아요

# What was found wrong while making room. { $is } is the size the part has; { $should } is the size it should have.
disks-room-check-part = 파티션이 { $is }인데 { $should }여야 해요

# What was found wrong while making room.
disks-room-check-open = 암호화의 잠금 해제된 사본이 맞는 크기가 아니에요

# What was found wrong while making room. { $is } is the size the files' area has; { $should } is the size it should have.
disks-room-check-area = 파일 시스템이 { $is }인데 { $should }여야 해요

# What was found wrong while making room.
disks-room-check-area-small = 파일 시스템이 사용 중인 양보다 작아요

# What was found wrong while making room.
disks-room-check-fit = 파일 시스템이 더 이상 맞지 않아요

# Red words when making room was stopped because the drive service no longer lists something. { $what } is part, copy or files.
disks-room-gone =
    { $what ->
        [partition] 드라이브 서비스 목록에 파티션이 더 이상 없어서 작업을 멈췄어요.
        [copy] 드라이브 서비스 목록에 잠금 해제된 사본이 더 이상 없어서 작업을 멈췄어요.
       *[files] 드라이브 서비스 목록에 파일 시스템이 더 이상 없어서 작업을 멈췄어요.
    }

# Step of making room, in the account of one that stopped part-way.
disks-room-step-files = 파일 시스템 줄이기

# Step of making room, in the account of one that stopped part-way.
disks-room-step-open = 암호화의 잠금 해제된 사본 줄이기

# Step of making room, in the account of one that stopped part-way.
disks-room-step-part = 파티션 줄이기

# Step of making room, in the account of one that stopped part-way.
disks-room-step-check = 크기 확인

# Description the snapshot taken before making room is saved under, which Settings lists. { $dev } is the part; { $at } is a number that tells it from another.
disks-room-snapshot-note = { $dev }에 공간을 만들기 전({ $at })

# Red words when the swap file is back on after making room, but Trindle met a problem doing it. { $done } is how making room went, { $why } the problem, both as sentences.
disks-swap-on-but = { $done } { $why }
