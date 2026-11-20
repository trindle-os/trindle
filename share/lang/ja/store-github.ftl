# machine translated (Japanese), not yet reviewed by a person.

# Apps: searching and installing from GitHub (src/store/github.rs).

## GitHub (src/store/github.rs)

# Said wherever the page offers a file from GitHub to install.
store-gh-unchecked = このファイルは安全にダウンロードされましたが、作者が署名を公開していないため、誰も確認していません。
# What a project with nothing to install here is told as.
store-gh-no-files = このプロジェクトのリリースには、このコンピューター向けの AppImage や Flatpak のファイルがありません。
# What a project that has never made a release is told as.
store-gh-no-releases = このプロジェクトには GitHub のリリースがありません。
# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = キーリングでキーを探しています
# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = キーが保存されています
# In the GitHub key's box: no key is stored.
store-gh-keyis-none = キーは保存されていません
# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = このコンピューターにはキーリングがないため、キーを保存できません
# Red words: the key is still in a plain file; why it was not moved follows.
store-gh-keyis-plain = GitHub のキーが、このコンピューターの古い通常のファイルに残っています。Trindle はもうこのファイルを読みません。{ $why }
# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } そのため、キーが保存されていても、Trindle はキーなしで GitHub を使います。
# Red words: a stored key has a space or line break in it.
store-gh-keyis-unsendable = 保存された GitHub のキーに空白か改行が入っているため、Trindle はそのキーを送れません。そのため、キーなしで GitHub を使います。キーを削除して、もう一度貼り付けてください。
# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = ファイルを削除できませんでした。
# Reason the keyring did not take the key.
store-gh-keyis-not-taken = キーリングが保存しなかったため、Trindle はキーなしで GitHub を使います。
# Red words when a pasted key has a space or line break in it.
store-gh-key-spaces = キーは空白のない 1 行です。このキーには改行か空白が入っています。キーだけを貼り付けてください。
# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub は今、応答を止めています。理由: { $which }。{ $when }
# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub は今、応答を止めています。理由: { $which }。{ $when } { $hint }
# Part of the limit words: how long to wait is not known.
store-gh-wait-later = しばらくしてからもう一度お試しください。
# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
       *[other] あと約 { $n } 分で、また応答します。
    }
# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = 検索は 1 分に 30 回までです
# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = 検索は 1 分に 10 回までです
# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = キーで使える 1 時間 5,000 回のリクエストを使い切りました
# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = キーがない場合は、同じアドレスから 1 時間に 60 回までです
# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = GitHub のキーを使うと、上限が 5,000 回に上がります。検索の右上にあるキーのボタンを押してください。
# Red words when GitHub does not accept the key.
store-gh-key-refused = GitHub がキーを受け付けませんでした。検索の右上にあるキーのボタンで、新しいキーを作るか、このキーを削除してください。
# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub が { $code } を返しました。
# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = 上限に達していないのに、GitHub に拒否されました。
# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub に拒否されました: { $why }。
# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
       *[other] 見つかったプロジェクトのうち { $n } 件を確認できなかったため、一覧に出ていません
    }
# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }。
# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }。
# Red words when GitHub could not be reached.
store-gh-unreached = GitHub に接続できませんでした。
# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = GitHub が Flatpak の検索に応答しなかったため、Trindle は AppImage があるプロジェクトだけを検索しました。{ $why }
# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = ダウンロード中
# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = インストール中
# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = ダウンロードが終わりませんでした。{ $why }
# The size of a download that is under a megabyte.
store-size-under = 1 MB 未満
