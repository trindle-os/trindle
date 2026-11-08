
# machine translated (Simplified Chinese), not yet reviewed by a person.

# Said wherever the page offers a file from GitHub to install.
store-gh-unchecked = 已通过安全连接下载，但没有人检查过这个文件：它的制作者没有发布签名。

# What a project with nothing to install here is told as.
store-gh-no-files = 这个项目的发布版本中，没有适用于这台电脑的 AppImage 或 Flatpak 文件。

# What a project that has never made a release is told as.
store-gh-no-releases = GitHub 上没有这个项目的发布版本。

# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = 正在密钥环中查找密钥

# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = 已保存密钥

# In the GitHub key's box: no key is stored.
store-gh-keyis-none = 没有保存密钥

# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = 这台电脑上没有密钥环，因此无法保存密钥

# Red words: the key is still in a plain file; why it was not moved follows.
# SAFETY: needs a human review before it ships
store-gh-keyis-plain = 你的 GitHub 密钥仍在这台电脑的一个普通文件里，并从那里使用：无法将它移入密钥环。{ $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } 因此无论是否保存了密钥，向 GitHub 提问时都不会带上密钥。

# Red words: a stored key has a space or line break in it.
# SAFETY: needs a human review before it ships
store-gh-keyis-unsendable = 已保存了一个 GitHub 密钥，但无法发送：它里面有空格或换行，就像分两段粘贴的密钥那样。因此向 GitHub 提问时不会带上密钥。请删除它，然后重新粘贴密钥。

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = 无法删除该文件。

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = 密钥环没有接收它。

# Red words when a pasted key has a space or line break in it.
# SAFETY: needs a human review before it ships
store-gh-key-spaces = 密钥是一行，中间没有空格，而这个密钥里有换行或空格。请只粘贴密钥本身。

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub 暂时不再回答：{ $which }。{ $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub 暂时不再回答：{ $which }。{ $when } { $hint }

# Part of the limit words: how long to wait is not known.
store-gh-wait-later = 请稍后再试。

# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
       *[other] 大约 { $n } 分钟后它会再次回答。
    }

# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = 它每分钟只回答 30 次搜索

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = 它每分钟只回答 10 次搜索

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = 使用你的密钥时它每小时回答 5,000 次提问，而这些次数已用完

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = 不带密钥时，同一个地址每小时它只回答 60 次提问

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = GitHub 密钥可将上限提高到 5,000 次：请按“搜索 GitHub”右上角的钥匙。

# Red words when GitHub does not accept the key.
# SAFETY: needs a human review before it ships
store-gh-key-refused = GitHub 不接受这个密钥。请重新生成一个，或将其删除：请按“搜索 GitHub”右上角的钥匙。

# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub 返回了 { $code }。

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = GitHub 拒绝了这个请求，尽管它的回答次数还没有用完。

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub 拒绝了这个请求：{ $why }。

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
       *[other] 找到的 { $n } 个项目无法查看，因此没有列出
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }。

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }：{ $why }。

# Red words when GitHub could not be reached.
store-gh-unreached = 无法连接到 GitHub。

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = 只搜索了带有 AppImage 的项目：GitHub 没有回应对 Flatpak 的搜索。{ $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = 正在下载

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = 正在安装

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = 下载没有完成。{ $why }

# The size of a download that is under a megabyte.
store-size-under = 不到 1 MB

