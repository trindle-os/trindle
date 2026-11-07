# machine translated (Vietnamese), not yet reviewed by a person.

# Said wherever the page offers a file from GitHub to install.
store-gh-unchecked = Đã tải về an toàn, nhưng chưa ai kiểm tra tệp này: tác giả không công bố chữ ký.

# What a project with nothing to install here is told as.
store-gh-no-files = Các bản phát hành của dự án này không có tệp AppImage hay Flatpak cho máy này.

# What a project that has never made a release is told as.
store-gh-no-releases = GitHub không có bản phát hành nào cho dự án này.

# In the GitHub key's box: the keyring is being asked.
store-gh-keyis-looking = Đang tìm khóa trong chùm khóa

# In the GitHub key's box: a key is stored.
store-gh-keyis-stored = Đã lưu một khóa

# In the GitHub key's box: no key is stored.
store-gh-keyis-none = Chưa lưu khóa nào

# In the GitHub key's box: there is no keyring.
store-gh-keyis-nokeyring = Máy này không có chùm khóa nên không thể lưu khóa

# Red words: the key is still in a plain file; why it was not moved follows.
store-gh-keyis-plain = Khóa GitHub của bạn vẫn nằm trong một tệp thường trên máy này và đang được dùng từ đó: không chuyển được vào chùm khóa. { $why }

# Red words: the keyring could not say; why is given first.
store-gh-keyis-hidden = { $why } Vì vậy GitHub sẽ được hỏi mà không dùng khóa, dù đã lưu khóa hay chưa.

# Red words: a stored key has a space or line break in it.
# SAFETY: needs a human review before it ships
store-gh-keyis-unsendable = Đã lưu một khóa GitHub nhưng không gửi đi được: bên trong có dấu cách hoặc ngắt dòng, như khi dán khóa thành hai đoạn. Vì vậy GitHub sẽ được hỏi mà không dùng khóa. Hãy xóa khóa rồi dán lại.

# Reason the plain file holding the key could not be removed.
store-gh-keyis-file-stays = Không xóa được tệp đó.

# Reason the keyring did not take the key.
store-gh-keyis-not-taken = Chùm khóa không nhận khóa.

# Red words when a pasted key has a space or line break in it.
store-gh-key-spaces = Khóa chỉ gồm một dòng và không có dấu cách, còn khóa này có ngắt dòng hoặc dấu cách bên trong. Hãy chỉ dán riêng khóa.

# GitHub's limit was reached: which limit, when it answers again.
store-gh-limit = GitHub tạm ngừng trả lời: { $which }. { $when }

# The same, with where the key goes that raises the limit.
store-gh-limit-hint = GitHub tạm ngừng trả lời: { $which }. { $when } { $hint }

# Part of the limit words: how long to wait is not known.
store-gh-wait-later = Hãy thử lại sau.

# Part of the limit words: how long until GitHub answers again.
store-gh-wait-minutes =
    { $n ->
        [1] GitHub sẽ trả lời lại sau khoảng một phút.
       *[other] GitHub sẽ trả lời lại sau khoảng { $n } phút.
    }

# Part of the limit words: the limit of searches with a key.
store-gh-limit-search-key = chỉ trả lời 30 lượt tìm mỗi phút

# Part of the limit words: the limit of searches without a key.
store-gh-limit-search = chỉ trả lời 10 lượt tìm mỗi phút

# Part of the limit words: the limit of questions with a key.
store-gh-limit-key = chỉ trả lời 5.000 câu hỏi mỗi giờ với khóa của bạn, và đã dùng hết

# Part of the limit words: the limit of questions without a key.
store-gh-limit-plain = chỉ trả lời 60 câu hỏi mỗi giờ từ một địa chỉ khi không có khóa

# Part of the limit words: how the key raises the limit.
store-gh-limit-key-hint = Khóa GitHub nâng mức đó lên 5.000: nhấn biểu tượng chìa khóa ở góc trên bên phải của Tìm trên GitHub.

# Red words when GitHub does not accept the key.
# SAFETY: needs a human review before it ships
store-gh-key-refused = GitHub không chấp nhận khóa. Hãy tạo khóa mới, hoặc xóa nó: nhấn biểu tượng chìa khóa ở góc trên bên phải của Tìm trên GitHub.

# Red words when GitHub answered with a code and no words.
store-gh-answered = GitHub trả lời { $code }.

# Red words when GitHub refused a question without saying why.
store-gh-refused-plain = GitHub từ chối việc này, dù chưa hết lượt trả lời.

# Red words when GitHub refused a question; its reason is given.
store-gh-refused = GitHub từ chối việc này: { $why }.

# Red words: projects found could not be looked at.
store-gh-unread =
    { $n ->
        [1] Có 1 dự án tìm thấy không xem được nên không được liệt kê
       *[other] Có { $n } dự án tìm thấy không xem được nên không được liệt kê
    }

# Red words: the above made a sentence.
store-gh-unread-sentence = { $which }.

# Red words: the above and why the last could not be looked at.
store-gh-unread-why = { $which }: { $why }.

# Red words when GitHub could not be reached.
store-gh-unreached = Không kết nối được với GitHub.

# Red words over what a search found when only half of it was answered; why GitHub did not answer follows.
store-gh-half = Chỉ tìm các dự án có AppImage: GitHub không trả lời lượt tìm Flatpak. { $why }

# Word on a GitHub file's dial while it downloads.
store-gh-step-downloading = Đang tải

# Word on a GitHub file's dial while it is put in.
store-gh-step-installing = Đang cài

# Red words when a download did not finish; what curl said follows.
store-gh-download-failed = Việc tải chưa hoàn tất. { $why }

# The size of a download that is under a megabyte.
store-size-under = dưới 1 MB

