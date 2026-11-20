# machine translated (Korean), not yet reviewed by a person.

# Words for the right-click acts of the file chooser and the ring's pinned paths (src/screenshot/file_chooser/lent.rs, src/common/kinds.rs).

# Name given to a folder made with the New folder button; a number follows if the name is taken.
chosen-act-new-folder = 새 폴더

# Red words when a new folder or document could not be made; its name, then the reason.
chosen-act-make-failed = { $base }을(를) 만들 수 없었어요: { $why }.

# Red words under a name being typed when it is empty.
chosen-act-name-empty = 이름을 비워 둘 수 없어요.

# Red words under a name being typed when it has a slash.
chosen-act-name-slash = 이름에 슬래시를 넣을 수 없어요.

# Red words under a name being typed when it is . or ..
chosen-act-name-kept = 시스템이 쓰는 이름이에요.

# Red words under a name being typed when it is too long.
chosen-act-name-long = 이름이 너무 길어요.

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
chosen-inplace-not-renamed = Trindle이 { $name }의 이름을 바꿀 수 없었어요: { $why }.

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
chosen-inplace-taken =
    { $kind ->
        [folder] { $name } 폴더가 여기에 이미 있어요.
       *[other] { $name } 파일이 여기에 이미 있어요.
    }

# First part of the path bar in Folders when the folder is inside the home folder: the home folder.
chosen-home = 홈

# Name of the top of the filesystem, "/", in the path bar, the sidebar and the Storage page: the disk the system is on.
chosen-root-name = 시스템 디스크
