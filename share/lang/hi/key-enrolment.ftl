# machine translated (Hindi), not yet reviewed by a person.

# Title of the Help topic about approving a driver's key.
key-enrol-title = रीस्टार्ट पर ड्राइवर की की को मंज़ूरी देना

# Words people might search for that topic by, in lower case, separated by spaces.
key-enrol-keywords = secure boot key mok enroll enrol blue screen password driver nvidia virtualbox camera dkms mokutil black screen सिक्योर बूट की ड्राइवर नीली स्क्रीन पासवर्ड

# SAFETY: needs a human review before it ships
# First paragraph before the steps when the driver is NVIDIA's own.
key-enrol-why-nvidia = NVIDIA का अपना ड्राइवर इसी कंप्यूटर पर बनाया जाता है, इसलिए जब तक आप उसकी की को मंज़ूरी नहीं देते, Secure Boot उसे लोड नहीं करेगा। रीस्टार्ट से पहले यह करें, वरना स्क्रीन काली रह सकती है।

# SAFETY: needs a human review before it ships
# First paragraph before the steps when the driver is an add-on driver from the Apps page or the Terminal.
key-enrol-why-addon = एक अतिरिक्त ड्राइवर इसी कंप्यूटर पर बनाया जाता है, इसलिए जब तक आप उसकी की को मंज़ूरी नहीं देते, Secure Boot उसे लोड नहीं करेगा। रीस्टार्ट से पहले यह करें।

# SAFETY: needs a human review before it ships
# Step 1: the password, made before the restart.
key-enrol-step1 = केवल अक्षरों और अंकों वाला एक पासवर्ड सोच लें। नीली स्क्रीन पर अमेरिकी कीबोर्ड होता है, इसलिए चिह्न दूसरी कुंजियों पर हो सकते हैं। इसे लिख लें; यह केवल एक बार काम आता है।

# SAFETY: needs a human review before it ships
# Step 2: the restart and how long the blue screen waits.
key-enrol-step2 = रीस्टार्ट करें। Trindle शुरू होने से पहले Enroll MOK शीर्षक वाली एक नीली स्क्रीन आती है। यह लगभग 10 सेकंड बाद हार मान लेती है, इसलिए कोई कुंजी दबाने के लिए तैयार रहें।

# SAFETY: needs a human review before it ships
# Step 3: the first two screens.
key-enrol-step3 = Enroll MOK चुनें, फिर Continue।

# SAFETY: needs a human review before it ships
# Step 4: the third screen and the password.
key-enrol-step4 = Yes चुनें, फिर वह पासवर्ड लिखें जो आपने बनाया था।

# SAFETY: needs a human review before it ships
# Step 5: the last screen.
key-enrol-step5 = Reboot चुनें। कंप्यूटर वापस आने पर ड्राइवर लोड हो जाता है।

# Tip after the steps: what happens when the blue screen is missed and how to be asked again.
key-enrol-missed = नीली स्क्रीन छूट जाए तो कंप्यूटर पहले की तरह ड्राइवर के बिना शुरू होता है। कुछ टूटा नहीं है; दोबारा पूछे जाने के लिए sudo mokutil --import /var/lib/dkms/mok.pub चलाएँ और रीस्टार्ट करें।
