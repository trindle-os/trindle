
# machine translated (Vietnamese), not yet reviewed by a person.

# Title of the Help topic about approving a driver's key.
# SAFETY: needs a human review before it ships
key-enrol-title = Phê duyệt khóa của trình điều khiển khi khởi động lại

# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = secure boot key mok enroll enrol blue screen password driver nvidia virtualbox camera dkms mokutil black screen khởi động an toàn khóa màn hình xanh mật khẩu trình điều khiển camera màn hình đen

# First paragraph before the steps when the driver is NVIDIA's own.
# SAFETY: needs a human review before it ships
key-enrol-why-nvidia = Trình điều khiển riêng của NVIDIA được tạo ngay trên máy tính này, nên Secure Boot sẽ không tải nó cho đến khi bạn phê duyệt khóa của nó. Hãy làm việc này trước khi khởi động lại, nếu không màn hình có thể vẫn đen.

# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
# SAFETY: needs a human review before it ships
key-enrol-why-addon = Một trình điều khiển bổ sung được tạo ngay trên máy tính này, nên Secure Boot sẽ không tải nó cho đến khi bạn phê duyệt khóa của nó. Hãy làm việc này trước khi khởi động lại.

# Step 1: the password, made before the restart.
# SAFETY: needs a human review before it ships
key-enrol-step1 = Tự đặt một mật khẩu chỉ gồm chữ cái và chữ số. Màn hình xanh dùng bàn phím kiểu Mỹ, nên các ký hiệu có thể nằm ở phím khác. Hãy ghi lại; mật khẩu chỉ dùng một lần.

# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = Khởi động lại. Một màn hình xanh có tiêu đề Enroll MOK hiện ra trước khi Trindle khởi động. Nó tự bỏ qua sau khoảng 10 giây, nên hãy sẵn sàng nhấn một phím.

# Step 3: the first two screens.
key-enrol-step3 = Chọn Enroll MOK, rồi Continue.

# Step 4: the third screen and the password.
# SAFETY: needs a human review before it ships
key-enrol-step4 = Chọn Yes, rồi gõ mật khẩu bạn đã đặt.

# Step 5: the last screen.
key-enrol-step5 = Chọn Reboot. Trình điều khiển sẽ được tải khi máy tính khởi động xong.

# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = Nếu lỡ màn hình xanh, máy tính vẫn khởi động mà không có trình điều khiển, như trước. Không có gì bị hỏng; hãy chạy sudo mokutil --import /var/lib/dkms/mok.pub rồi khởi động lại để được hỏi lại.
