# machine translated (Korean), not yet reviewed by a person.

# Apps: searching and installing from GitHub (src/store/github.rs).

## GitHub (src/store/github.rs)

# Said wherever the page offers a file from GitHub to install.
store-gh-unchecked = 이 파일은 안전하게 내려받았지만, 만든 사람이 서명을 공개하지 않아서 아무도 확인하지 않았어요.
# What a project with nothing to install here is told as.
store-gh-no-files = 이 프로젝트의 릴리스에는 이 컴퓨터용 AppImage나 Flatpak 파일이 없어요.
# What a project that has never made a release is told as.
store-gh-no-releases = 이 프로젝트에는 GitHub 릴리스가 없어요.
# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = 키링에서 키를 찾는 중
# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = 키가 저장되어 있어요
# In the GitHub key's box: no key is stored.
store-gh-keyis-none = 저장된 키가 없어요
# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = 이 컴퓨터에는 키링이 없어서 키를 저장할 수 없어요
# Red words: the key is still in a plain file; why it was not moved follows.
store-gh-keyis-plain = GitHub 키가 이 컴퓨터의 예전 일반 파일에 아직 남아 있어요. Trindle은 이제 그 파일을 읽지 않아요. { $why }
# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } 그래서 저장된 키가 있어도 Trindle은 키 없이 GitHub를 써요.
# Red words: a stored key has a space or line break in it.
store-gh-keyis-unsendable = 저장된 GitHub 키에 공백이나 줄바꿈이 있어서 Trindle이 보낼 수 없어요. 그래서 키 없이 GitHub를 써요. 키를 지우고 다시 붙여 넣으세요.
# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = 파일을 지우지 못했어요.
# Reason the keyring did not take the key.
store-gh-keyis-not-taken = 키링이 키를 저장하지 않아서 Trindle은 키 없이 GitHub를 써요.
# Red words when a pasted key has a space or line break in it.
store-gh-key-spaces = 키는 공백 없이 한 줄이에요. 이 키에는 줄바꿈이나 공백이 들어 있어요. 키만 따로 붙여 넣으세요.
# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub가 지금은 응답을 멈췄어요. { $which }. { $when }
# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub가 지금은 응답을 멈췄어요. { $which }. { $when } { $hint }
# Part of the limit words: how long to wait is not known.
store-gh-wait-later = 나중에 다시 시도하세요.
# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
       *[other] 약 { $n }분 뒤에 다시 응답해요.
    }
# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = 1분에 30번까지 검색할 수 있어요
# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = 1분에 10번까지 검색할 수 있어요
# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = 키로 허용되는 시간당 5,000번의 요청을 다 썼어요
# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = 키 없이는 한 주소에서 시간당 60번까지 요청할 수 있어요
# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = GitHub 키가 있으면 5,000번으로 늘어나요. 검색 화면 오른쪽 위의 키 버튼을 누르세요.
# Red words when GitHub does not accept the key.
store-gh-key-refused = GitHub가 키를 받아 주지 않았어요. 검색 화면 오른쪽 위의 키 버튼으로 새 키를 만들거나 이 키를 지우세요.
# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub가 { $code }을(를) 보냈어요.
# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = 한도에 이르지 않았는데도 GitHub가 이 요청을 막았어요.
# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub가 이 요청을 막았어요: { $why }.
# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
       *[other] 찾은 프로젝트 { $n }개를 확인하지 못해서 목록에 없어요
    }
# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.
# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }.
# Red words when GitHub could not be reached.
store-gh-unreached = GitHub에 연결할 수 없어요.
# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = GitHub가 Flatpak 검색에 응답하지 않아서 Trindle은 AppImage가 있는 프로젝트만 검색했어요. { $why }
# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = 내려받는 중
# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = 설치하는 중
# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = 내려받기가 끝나지 않았어요. { $why }
# The size of a download that is under a megabyte.
store-size-under = 1 MB 미만
