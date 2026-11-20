# machine translated (Japanese), not yet reviewed by a person.

# Words for the right-click acts of the file chooser and the ring's pinned paths (src/screenshot/file_chooser/lent.rs, src/common/kinds.rs).

# Name given to a folder made with the New folder button; a number follows if the name is taken.
chosen-act-new-folder = 新しいフォルダ

# Red words when a new folder or document could not be made; its name, then the reason.
chosen-act-make-failed = { $base } を作れませんでした：{ $why }。

# Red words under a name being typed when it is empty.
chosen-act-name-empty = 名前は空にできません。

# Red words under a name being typed when it has a slash.
chosen-act-name-slash = 名前にスラッシュは使えません。

# Red words under a name being typed when it is . or ..
chosen-act-name-kept = その名前はシステムが使っています。

# Red words under a name being typed when it is too long.
chosen-act-name-long = 名前が長すぎます。

# Red words under the name box, and at the foot of Folders, when a rename fails: the item's name, then the reason.
chosen-inplace-not-renamed = Trindle は { $name } の名前を変えられませんでした：{ $why }。

# Red words under the name box, and at the foot of Folders, when a name is already used in that place; the kind is the word folder or file.
chosen-inplace-taken =
    { $kind ->
        [folder] { $name } というフォルダがここにすでにあります。
       *[other] { $name } というファイルがここにすでにあります。
    }

# First part of the path bar in Folders when the folder is inside the home folder: the home folder.
chosen-home = ホーム

# Name of the top of the filesystem, "/", in the path bar, the sidebar and the Storage page: the disk the system is on.
chosen-root-name = システムディスク
