# machine translated (Korean), not yet reviewed by a person.

# Settings: the Users page (src/settings/panels/users.rs).

## The Users page (src/settings/panels/users.rs)

# Name of the Users page, in the page list and as its heading.
settings-users-title = 사용자
# Words people may search for to find the Users page, separated by spaces.
settings-users-keywords = 계정 사람 비밀번호 사진 그림 관리자
# Help line at the foot of the Users page.
settings-users-help = 사람을 추가하거나 비밀번호를 바꾸거나 계정을 삭제하려면 관리자 비밀번호가 필요해요.
# Beside a change that is waiting for the administrator's password to be typed.
settings-users-waiting = 비밀번호를 기다리는 중…
# Headline of the Users page when nobody on the computer is an administrator.
settings-users-sum-nobody-admin = 관리자가 없어서 아무도 모두를 위해 설치하거나 바꿀 수 없어요.
# Headline of the Users page when you are the only person and an administrator.
settings-users-sum-only-you = 이 컴퓨터는 나만 쓰고, 내가 관리자예요.
# Headline of the Users page when one person who is not you uses the computer.
settings-users-sum-only-one = 이 컴퓨터는 한 사람만 써요.
# Headline of the Users page: how many people use the computer, and you are the one administrator.
settings-users-sum-you-admin =
    { $n ->
       *[other] { $n }명이 이 컴퓨터를 쓰고, 내가 관리자예요.
    }
# Headline of the Users page: how many people use the computer, and one of them is the administrator.
settings-users-sum-one-admin =
    { $n ->
       *[other] { $n }명이 이 컴퓨터를 쓰고, 그중 한 명이 관리자예요.
    }
# Headline of the Users page: how many people use the computer and how many are administrators.
settings-users-sum-admins =
    { $n ->
       *[other] { $n }명이 이 컴퓨터를 쓰고, 그중 { $a }명이 관리자예요.
    }
# Under a person's name on their card when the card is yours.
settings-users-this-is-you = 바로 나예요.
# Under a person's name on their card when the card is someone else's: the name shown is their login name.
settings-users-login-name = 로그인 이름
# Tip of the round button on a person's card that picks their picture, and title of the window that opens.
settings-users-choose-picture = 사진 고르기
# Name of the picture type filter in the window for choosing a person's picture.
settings-users-pictures-filter = 사진
# Tip of the round button on a person's card that changes their password.
settings-users-change-password = 비밀번호 바꾸기
# Tip of the round button on a person's card that deletes them and their files.
settings-users-delete = 계정과 파일 삭제
# Said beside a person once they are deleted.
settings-users-deleted = 삭제됨
# Label of the administrator switch and of the line it sits on, on a person's card and in the Add someone form.
settings-users-administrator = 관리자
# Tip of the administrator switch on your own card, and said when you press it: you cannot remove your own administrator rights.
settings-users-cannot-take-away = 내 관리자 권한은 없앨 수 없어요.
# Said beside a person after they are made an administrator.
settings-users-admin-now = 이제 관리자
# Said beside a person after they stop being an administrator.
settings-users-admin-no-longer = 더 이상 관리자 아님
# Help text for a person's card (the information button).
settings-users-person-about = 관리자는 프로그램을 설치하고 모두를 위해 설정을 바꿀 수 있어요. 내 관리자 권한은 없앨 수 없어요. 바꿀 때마다 관리자 비밀번호가 필요해요.
# Typing box in the change password form, for your own current password.
settings-users-current-password = 현재 비밀번호
# Typing box in the change password form for the new password.
settings-users-new-password = 새 비밀번호
# Typing box in the change password form where the new password is typed again.
settings-users-again = 한 번 더 입력
# Button in the change password form that saves the new password.
settings-users-save = 저장
# Spoken name of the Save button in the change password form.
settings-users-save-new = 새 비밀번호 저장
# Tip of the button that closes a form on the Users page without saving.
settings-users-cancel = 취소
# Red words in the change password form when your current password was left empty.
settings-users-need-current = 저장된 로그인이 계속 작동하도록 현재 비밀번호도 입력하세요.
# Red words in the change password form when the new password is too short.
settings-users-at-least-8 = 8자 이상으로 입력하세요.
# Red words in the change password form when the two new passwords differ.
settings-users-no-match = 두 비밀번호가 달라요.
# Said beside a person once their password is changed.
settings-users-password-changed = 비밀번호가 바뀌었어요
# Red words after a password change when the saved logins could not take the new password.
settings-users-not-rekeyed = 비밀번호는 바뀌었지만 저장된 로그인은 아직 예전 비밀번호를 써요.
# Red words when the chosen file is not a picture the page can use.
settings-users-png-please = PNG, JPG, WebP 파일을 고르세요.
# Red words when a person's picture could not be changed.
settings-users-picture-not-changed = 사진을 바꾸지 못했어요.
# Said beside a person once their picture is saved.
settings-users-saved = 저장됨
# Red words when your new picture works at login but could not be copied to your home folder.
settings-users-picture-not-shared = 사진은 로그인 화면에 나오지만, Trindle이 다른 프로그램과 공유하지 못했어요.
# Heading of the form for adding a person.
settings-users-add-someone = 사람 추가
# Help text for the Add someone form (the information button).
settings-users-add-about = 전체 이름, 로그인 이름, 8자 이상의 비밀번호를 입력하세요. 로그인 이름에는 영문 소문자와 숫자만 쓸 수 있고, 문자로 시작해야 해요. 바로 로그인할 수 있어요. 관리자는 프로그램을 설치하고 모두를 위해 설정을 바꿀 수 있어요.
# Heading of the line in the Add someone form that holds the button.
settings-users-new-account = 새 계정
# Typing box in the Add someone form for the person's full name.
settings-users-full-name = 전체 이름
# Typing box in the Add someone form for the login name.
settings-users-login-name-lower = 로그인 이름 (소문자)
# Typing box in the Add someone form for the first password.
settings-users-password = 비밀번호
# Tip of the button that adds the new person.
settings-users-add-this = 이 사람 추가
# Red words when the login name typed is not allowed.
settings-users-login-rules = 로그인 이름은 영문 소문자와 숫자만 쓸 수 있고, 문자로 시작해야 해요.
# Red words when the new person's password is too short.
settings-users-password-8 = 비밀번호는 8자 이상이어야 해요.
# Red words when the login name typed is already taken: the name is { $name }.
settings-users-exists = { $name }은(는) 이미 있어요.
# Said beside the Add someone form once the person is added.
settings-users-added = 추가됨
# Heading of the Saved logins card.
settings-users-saved-logins = 저장된 로그인
# Help text for the Saved logins card (the information button).
settings-users-keyring-about = 프로그램은 메일이나 Wi-Fi 같은 비밀번호를 로그인할 때 열리는 키링에 보관해요. 로그인하면 Trindle이 키링을 자체 키링으로 옮겨요. 옮기지 못했다면 여기서 옮길 수 있어요. 저장된 로그인은 모두 복사돼요. 로그인으로 열리지 않는 키링은 비밀번호를 물을 수 있어요. GNOME의 파일은 그대로 남아요.
# Heading of the line on the Saved logins card when the logins are kept by Trindle's keyring.
settings-users-kept-trindle = Trindle 키링에 있음
# Heading of the line on the Saved logins card when the logins are kept by GNOME's keyring.
settings-users-kept-gnome = GNOME 키링에 있음
# Under the heading when the logins are still kept by GNOME's keyring.
settings-users-could-not-move = 로그인할 때 Trindle이 옮기지 못했어요.
# Tip of the button that moves saved logins to Trindle's keyring.
settings-users-move-keyring = Trindle 키링으로 옮기기
# Typing box for your password in the form for moving saved logins.
settings-users-your-password = 내 비밀번호
# Tip of the button that starts moving the saved logins.
settings-users-go-ahead = 지금 옮기기
# Typing box for the password of one GNOME keyring that stayed shut: the keyring's name is { $name }.
settings-users-password-for = { $name }의 비밀번호
# Red words when the move of saved logins was started without a password.
settings-users-your-password-please = 비밀번호를 입력하세요.
# Said beside the Saved logins line while they are copied.
settings-users-copying = 저장된 로그인을 복사하는 중…
# Said beside the Saved logins line when the copy is done: how many logins were copied.
settings-users-copied = { $n }개 복사됨
# Red words when keyrings stayed shut: the keyrings' names are { $names } and there are { $n } of them.
settings-users-nothing-moved =
    { $n ->
       *[other] 아직 아무것도 옮기지 않았어요. { $names }의 비밀번호를 입력하세요.
    }
# Red words when the move of saved logins stopped unexpectedly.
settings-users-stopped = 끝나지 않았어요.
# Words in place of the people while the accounts are still being read.
settings-users-looking-accounts = 계정을 확인하는 중…
# Heading of the placeholder card shown while the accounts are being read.
settings-users-people = 사람
# Under the Administrator line of the placeholder card.
settings-users-looking = 불러오는 중…
