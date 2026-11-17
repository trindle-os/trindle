# Settings page "Audio": speakers, microphone, volume, mute and a test sound.

## Page (src/settings/panels/audio.rs)

# Name of the Settings page about sound, in the list and as its heading.
settings-audio-title = Audio
# Words a person might type into the Settings search to find the audio page; separate them with spaces.
settings-audio-keywords = sound volume speakers headphones microphone mic mute test
# Introduction under the heading of the audio page, and the words behind the page i.
settings-audio-help = Choose which speakers or headphones play sound, and which microphone to use.

## Top sentence

# Red sentence at the top of the page when PipeWire, the sound system, is not installed.
settings-audio-top-missing = There is no sound because PipeWire isn't installed.
# Red sentence at the top of the page when PipeWire is installed but not answering.
settings-audio-top-failed = There is no sound because PipeWire, the sound service, isn't responding.
# Red sentence at the top of the page when there are no speakers or headphones.
settings-audio-top-no-output = There are no speakers or headphones.
# Plain sentence at the top of the page while the page is still finding out about sound.
settings-audio-top-listening = Checking for sound…
# Warning at the top of the page when sound is muted; { $device } is the speakers or headphones.
settings-audio-top-muted = { $device } is muted.
# Warning at the top of the page when the volume is at nothing; { $device } is the speakers or headphones.
settings-audio-top-down = The volume for { $device } is all the way down.
# Sentence at the top of the page when all is well; { $device } is the speakers or headphones and { $percent } the volume.
settings-audio-top-playing = Sound is playing through { $device } at { $percent }%.

## Cards

# Title of the card for speakers and headphones.
settings-audio-out-title = Speakers and headphones
# Title of the card for microphones.
settings-audio-in-title = Microphone
# Heading of the line shown when no speakers or headphones are found.
settings-audio-out-none = No speakers found
# Heading of the line shown when no microphone is found.
settings-audio-in-none = No microphone
# Small words under that line for speakers.
settings-audio-out-plug = Plug in speakers or headphones.
# Small words under that line for microphones.
settings-audio-in-plug = Plug in a microphone or headset.
# Heading of the line with the list of devices.
settings-audio-device = Device
# Name of the list of speakers and headphones.
settings-audio-out-device-name = Where sound plays
# Name of the list of microphones.
settings-audio-in-device-name = Microphone
# Heading of the volume line.
settings-audio-volume = Volume
# Name of the volume slider for the speakers.
settings-audio-out-volume-name = Sound volume
# Name of the volume slider for the microphone.
settings-audio-in-volume-name = Microphone volume
# Heading of the mute line.
settings-audio-mute = Mute
# Name of the switch that mutes the sound.
settings-audio-out-mute-name = Mute the sound
# Name of the switch that mutes the microphone.
settings-audio-in-mute-name = Mute the microphone
# Heading of the line that plays a test sound.
settings-audio-test = Test
# Name of the button that plays a test sound.
settings-audio-test-button = Play a test sound
# Words behind the i of the speakers card.
settings-audio-out-about = Pick which speakers or headphones play sound. The volume and mute here work the same as the keys on your keyboard and the sound circle in the ring.
# Heading of the line with the microphone level bar.
settings-audio-level = Level
# Words behind the i of the microphone card.
settings-audio-in-about = Choose which microphone to use. While this page is open, the bar shows the sound it picks up. Trindle doesn't record or keep anything.
# Title of the card shown when PipeWire is not answering.
settings-audio-failed-title = Sound
# Heading of the line in that card.
settings-audio-failed-heading = PipeWire isn't responding
# Small words under that line.
settings-audio-failed-about = Sound may not have started when you logged in.

## Red words

# Red words under the volume line when the volume could not be changed.
settings-audio-no-volume = Trindle couldn't change the volume. Sound may not have started. Log out and back in, then try again.
# Red words under the mute line when the sound could not be muted.
settings-audio-no-mute = Trindle couldn't mute the sound. Sound may not have started. Log out and back in, then try again.
# Red words under the mute line when the sound could not be turned back on.
settings-audio-no-unmute = Trindle couldn't turn the sound back on. Sound may not have started. Log out and back in, then try again.
# Red words under the device line when the device could not be chosen.
settings-audio-not-chosen = Couldn't switch to that device.
# Red words under the test line when the test sound would not play.
settings-audio-test-failed = The test sound didn't play through this device.

## Where sound goes

# A device's name with what kind it is; { $name } is the device and { $kind } one of the kind words below.
settings-audio-with-kind = { $name } ({ $kind })
# Kind of output: a Bluetooth device.
settings-audio-kind-bluetooth = Bluetooth
# Kind of output: a screen or TV on a cable.
settings-audio-kind-hdmi = HDMI
# Kind of output: a device on a USB cable.
settings-audio-kind-usb = USB
# Kind of output: headphones or a headset on a cable.
settings-audio-kind-headphones = headphones
# Kind of output: the computer's own speakers.
settings-audio-kind-speakers = speakers
# Heading of the line shown while Bluetooth plays the sound.
settings-audio-bt-now = Sound is going to Bluetooth
# Tooltip of the button that moves the sound to the speakers; { $device } is the speakers, kind included.
settings-audio-use-speakers = Play through { $device }
# Heading of the line when Bluetooth devices are connected but have no sound output; { $names } lists them.
settings-audio-bt-silent = { $names } is connected, but sound can't play through it.
# Words under that heading.
settings-audio-bt-silent-about = Its sound may not be set up. Disconnect it, then connect it again.
# Red words when the sound devices could not be read.
settings-audio-not-read = Trindle couldn't find the sound devices.
