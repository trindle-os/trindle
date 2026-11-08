# machine translated (Simplified Chinese), not yet reviewed by a person.

# Title of the Help topic about approving a driver's key.
key-enrol-title = 重启时批准驱动程序的密钥

# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = 安全启动 密钥 mok 注册 蓝屏 密码 驱动 nvidia virtualbox 摄像头 dkms mokutil 黑屏 secure boot key enroll enrol

# First paragraph before the steps when the driver is NVIDIA's own.
key-enrol-why-nvidia = NVIDIA 自带的驱动程序是在这台电脑上编译的，所以在您批准它的密钥之前，安全启动不会加载它。请在重启之前完成此操作，否则屏幕可能一直是黑的。

# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
key-enrol-why-addon = 附加驱动程序是在这台电脑上编译的，所以在您批准它的密钥之前，安全启动不会加载它。请在重启之前完成此操作。

# SAFETY: needs a human review before it ships
# Step 1: the password, made before the restart.
key-enrol-step1 = 想一个只含字母和数字的密码。蓝屏使用美式键盘，符号可能在别的按键上。请把它写下来，只会用一次。

# SAFETY: needs a human review before it ships
# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = 重启。Trindle 启动之前会出现一个标题为 Enroll MOK 的蓝屏。大约 10 秒后它会放弃等待，所以请准备好按键。

# SAFETY: needs a human review before it ships
# Step 3: the first two screens.
key-enrol-step3 = 选择 Enroll MOK，然后选择 Continue。

# SAFETY: needs a human review before it ships
# Step 4: the third screen and the password.
key-enrol-step4 = 选择 Yes，然后输入您设置的密码。

# SAFETY: needs a human review before it ships
# Step 5: the last screen.
key-enrol-step5 = 选择 Reboot。电脑重新启动后，驱动程序就会加载。

# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = 如果错过了蓝屏，电脑会像以前一样在没有驱动程序的情况下启动。没有任何损坏；运行 sudo mokutil --import /var/lib/dkms/mok.pub 并重启，即可再次被询问。
