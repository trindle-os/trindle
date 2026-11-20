# machine translated (Japanese), not yet reviewed by a person.

# Words said before a restart that leads into approving a driver's key (the blue Enroll MOK screen), for every path that leads there (src/common/key_enrolment.rs). Shown as steps in the Help topic "secure-boot-key" and wherever a driver that needs a key is offered.

## The steps (src/common/key_enrolment.rs)

# Title of the Help topic about approving a driver's key.
key-enrol-title = 再起動時にドライバーの鍵を承認する
# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = secure boot key mok enroll enrol blue screen password driver nvidia virtualbox camera dkms mokutil black screen
# First paragraph before the steps when the driver is NVIDIA's own.
key-enrol-why-nvidia = NVIDIA 純正のドライバーはこのコンピューターで作られるため、鍵を承認するまで Secure Boot が読み込みません。再起動の前に行ってください。行わないと、画面が真っ暗なままになることがあります。
# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
key-enrol-why-addon = 追加のドライバーはこのコンピューターで作られるため、鍵を承認するまで Secure Boot が読み込みません。再起動の前に行ってください。
# Step 1: the password, made before the restart.
key-enrol-step1 = 英字と数字だけでパスワードを決めます。青い画面は英語配列のキーボードで動くため、記号は別のキーにあることがあります。パスワードは書き留めてください。使うのは一度だけです。
# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = 再起動します。Trindle が始まる前に、Enroll MOK という青い画面が出ます。キーを押せるのは約 10 秒間なので、すぐ押せるようにしておいてください。
# Step 3: the first two screens.
key-enrol-step3 = Enroll MOK を選び、次に Continue を選びます。
# Step 4: the third screen and the password.
key-enrol-step4 = Yes を選び、決めたパスワードを入力します。
# Step 5: the last screen.
key-enrol-step5 = Reboot を選びます。コンピューターが再起動すると、ドライバーが読み込まれます。
# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = 青い画面を見逃しても、コンピューターは以前と同じようにドライバーなしで起動します。何も壊れていません。もう一度確認してほしいときは、sudo mokutil --import /var/lib/dkms/mok.pub を実行して再起動してください。
