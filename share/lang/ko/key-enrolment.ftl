# machine translated (Korean), not yet reviewed by a person.

# Words said before a restart that leads into approving a driver's key (the blue Enroll MOK screen), for every path that leads there (src/common/key_enrolment.rs). Shown as steps in the Help topic "secure-boot-key" and wherever a driver that needs a key is offered.

## The steps (src/common/key_enrolment.rs)

# Title of the Help topic about approving a driver's key.
key-enrol-title = 다시 시작할 때 드라이버 키 승인하기
# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = 보안 부팅 키 mok enroll enrol 파란 화면 비밀번호 드라이버 nvidia virtualbox 카메라 dkms mokutil 검은 화면 secure boot key blue screen password driver black screen
# First paragraph before the steps when the driver is NVIDIA's own.
key-enrol-why-nvidia = NVIDIA의 드라이버는 이 컴퓨터에서 직접 만들어져서, 키를 승인하기 전에는 보안 부팅이 불러오지 않아요. 다시 시작하기 전에 꼭 하세요. 안 하면 화면이 검게 남을 수 있어요.
# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
key-enrol-why-addon = 추가 드라이버는 이 컴퓨터에서 직접 만들어져서, 키를 승인하기 전에는 보안 부팅이 불러오지 않아요. 다시 시작하기 전에 하세요.
# Step 1: the password, made before the restart.
key-enrol-step1 = 영문자와 숫자만으로 비밀번호를 만드세요. 파란 화면은 미국 자판 배열을 쓰므로 기호는 다른 키에 있을 수 있어요. 비밀번호를 적어 두세요. 딱 한 번만 필요해요.
# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = 다시 시작하세요. Trindle이 시작되기 전에 Enroll MOK이라는 파란 화면이 나타나요. 키를 누를 시간은 약 10초라서 미리 준비하세요.
# Step 3: the first two screens.
key-enrol-step3 = Enroll MOK을 고른 다음 Continue를 고르세요.
# Step 4: the third screen and the password.
key-enrol-step4 = Yes를 고른 다음 만들어 둔 비밀번호를 입력하세요.
# Step 5: the last screen.
key-enrol-step5 = Reboot를 고르세요. 컴퓨터가 다시 시작되면 드라이버가 불러와져요.
# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = 파란 화면을 놓치면 컴퓨터는 전과 같이 드라이버 없이 시작해요. 고장 난 것은 아니에요. 다시 묻게 하려면 sudo mokutil --import /var/lib/dkms/mok.pub 를 실행하고 다시 시작하세요.
