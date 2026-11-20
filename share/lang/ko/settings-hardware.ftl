# machine translated (Korean), not yet reviewed by a person.

# Settings, Hardware page: the computer's devices by kind and which ones have no working driver.

## The page (src/settings/panels/hardware.rs)

# Name of the Hardware page in the Settings side bar and its title.
settings-hardware-title = 하드웨어
# Words the Settings search matches the Hardware page by, in lower case.
settings-hardware-keywords = 하드웨어 장치 기기 드라이버 펌웨어 pci usb 그래픽 소리 네트워크 bluetooth 저장소 카메라 키보드 마우스 없음 문제 칩 hardware devices drivers driver firmware graphics sound network storage camera keyboard mouse missing problem chip
# Text behind the info button of the Hardware page.
settings-hardware-help = Trindle이 볼 수 있는 이 컴퓨터의 모든 부품을 하는 일에 따라 묶어 보여 줘요. 드라이버가 없거나 드라이버에 펌웨어 파일이 없는 부품이 맨 위에 나와요. 드라이버는 시스템에 들어 있어서 여기서 받을 것은 없어요.
# The sentence at the top when every device is fine; n is how many devices there are.
settings-hardware-all-well =
    { $n ->
       *[other] 장치 { $n }개 모두 드라이버가 잘 작동해요.
    }
# The sentence at the top when something is wrong; n is how many problems there are.
settings-hardware-trouble =
    { $n ->
       *[other] 장치 { $n }개를 확인해야 해요.
    }
# Card names, one for each kind of device.
settings-hardware-kind-graphics = 그래픽
settings-hardware-kind-sound = 소리
settings-hardware-kind-network = 네트워크
settings-hardware-kind-bluetooth = Bluetooth
settings-hardware-kind-storage = 저장소
settings-hardware-kind-input = 키보드와 마우스
settings-hardware-kind-camera = 카메라
settings-hardware-kind-other = 기타
# Under a device that works; name is the driver's name.
settings-hardware-driver = 정상 작동. 드라이버: { $name }
# Under a device that works when the driver's name is not known.
settings-hardware-working = 정상 작동
# Under a device that has no driver.
settings-hardware-no-driver = 드라이버 없음
# Under a device whose driver is missing a firmware file.
settings-hardware-no-firmware = 펌웨어 없음
# Under a device with no driver, where none is expected, such as a bridge between two parts.
settings-hardware-idle = 사용 중인 드라이버 없음
# The name of a device that pci.ids and usb.ids do not know; id is its two hex numbers.
settings-hardware-unnamed = 알 수 없는 장치 { $id }

## The security chip card, shown only after the TPM chip did not unlock the encrypted disk at start

# Card name.
settings-hardware-chip = 보안 칩
# The card's line: why the disk asked for its password at start.
settings-hardware-chip-refused = 시작할 때 칩이 디스크 잠금을 풀지 못해서 암호를 물었어요. BIOS 업데이트가 원인일 수 있어요.
# Button that seals the disk's key to the chip again. Must match unlock-chip-refused-fix.
settings-hardware-chip-again = 칩 다시 사용
# Placeholder in the box the disk password is typed in.
settings-hardware-chip-password = 디스크 암호
# Button that closes the password box without changing anything.
settings-hardware-chip-cancel = 취소
# Red words when the button is pressed with the box empty.
settings-hardware-chip-type = 먼저 디스크 암호를 입력하세요.
# Beside the line while it works.
settings-hardware-chip-waiting = 칩을 설정하는 중
# Once it worked.
settings-hardware-chip-done = 다음 시작부터 칩이 디스크 잠금을 풀어요.
