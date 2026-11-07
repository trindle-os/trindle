# Settings: the Date and time page (src/settings/panels/datetime.rs).

## The Date and time page (src/settings/panels/datetime.rs)

# Name of the Date and time page, in the page list and as its heading.
settings-datetime-title = Date and time
# Words people may search for to find the Date and time page, separated by spaces.
settings-datetime-keywords = time zone clock date
# Help line at the foot of the Date and time page.
settings-datetime-help = The time zone and automatic time need an administrator's password.
# Spoken name of the picture of the clock at the top of the page.
settings-datetime-time-now = The time now
# Name of a way to write the ring's clock: hours 00 to 23.
settings-datetime-24 = 24 hour
# Name of a way to write the ring's clock: hours 1 to 12 with am or pm.
settings-datetime-12 = 12 hour
# Name of a way to write the ring's clock: hours 00 to 23 with seconds.
settings-datetime-24s = 24 hour, with seconds
# Name of the box for the day when the date is set by hand.
settings-datetime-day = Day
# Name of the box for the month when the date is set by hand.
settings-datetime-month = Month
# Name of the box for the year when the date is set by hand.
settings-datetime-year = Year
# Name of the box for the hour when the time is set by hand.
settings-datetime-hour = Hour
# Name of the box for the minute when the time is set by hand.
settings-datetime-minute = Minute
# Red words when the time could not be set.
settings-datetime-no-time = The time could not be set.
# Red words when the time was set by hand while automatic time is on.
settings-datetime-keeps-right = The clock keeps itself right just now. Turn that off above, then set the time by hand.
# Red words when the system's message bus could not be reached.
settings-datetime-no-bus = Could not reach the system bus.
# Red words when the time service does not answer.
settings-datetime-not-answering = The time service is not answering.
# Red words when automatic time could not be turned on.
settings-datetime-ntp-on-failed = The clock could not be told to keep itself right.
# Red words when automatic time could not be turned off.
settings-datetime-ntp-off-failed = Automatic time could not be turned off.
# Red words when the time zone could not be changed.
settings-datetime-zone-failed = The time zone could not be changed.
# Red words when the date typed by hand is not a real date.
settings-datetime-not-real = That is not a real date.
# Headline of the page before the time service has answered: { $time } is the time and { $date } the date.
settings-datetime-it-is = It is { $time } on { $date }.
# Headline of the page when the time service is not answering.
settings-datetime-service-down = The time service is not answering, so the clock cannot be changed here.
# Headline of the page when the clock is automatic and in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-right = It is { $time } on { $date }, and the clock keeps itself right.
# Headline of the page when the clock is automatic but not yet in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-checks = It is { $time } on { $date }. The clock checks itself once it reaches the internet.
# Headline of the page when the clock is set by hand: { $time } is the time and { $date } the date.
settings-datetime-it-is-hand = It is { $time } on { $date }. The clock is set by hand.
# A city and how far its time is from Greenwich.
settings-datetime-city-offset = { $city }, { $offset }
# A city, its part of the world and how far its time is from Greenwich.
settings-datetime-city-region-offset = { $city }, { $region }, { $offset }
# Spoken name of the clock picture: the time, the date and the place.
settings-datetime-tip = It is { $time } on { $date }, { $zone }.
# Help text of the clock card when the time service is not answering or not yet read.
settings-datetime-ring-about = The clock in the middle of the ring is written in 24 or 12 hours, with or without seconds. Only the ring's clock follows this; dates and times in programs follow the formats chosen in Region and language.
# Heading of the card with the clock settings.
settings-datetime-the-clock = The clock
# Heading of the line shown when the time service is not answering.
settings-datetime-service-down-line = The time service is not answering
# Under Set the time automatically when the internet can be used.
settings-datetime-from-internet = From the internet, when connected.
# Under Set the time automatically when the time service is not installed.
settings-datetime-needs-timesyncd = Needs systemd-timesyncd installed.
# Label of the switch for automatic time, and heading of its line.
settings-datetime-set-auto = Set the time automatically
# Name of the list of time zones, and heading of its line.
settings-datetime-time-zone = Time zone
# Tip of the button that sets the date and time typed by hand.
settings-datetime-set-date-time = Set the date and time
# Heading of the line for setting the time by hand.
settings-datetime-by-hand = Set the time by hand
# Under the Set the time by hand heading.
settings-datetime-by-hand-sub = Turn automatic time off first, as it is now.
# Help text of the clock card (the information button) when the time service can keep the time from the internet.
settings-datetime-about = Set automatically, the clock is checked against time servers on the internet whenever the computer is connected, and keeps itself right. Turned off, the date and time are typed in by hand. The time zone also tells night light when the sun sets. Both need an administrator's password. The clock in the middle of the ring is written in 24 or 12 hours, with or without seconds. Only the ring's clock follows this; dates and times in programs follow the formats chosen in Region and language.
# Help text of the clock card (the information button) when the time service is not installed.
settings-datetime-about-no-ntp = Set automatically, the clock is checked against time servers on the internet whenever the computer is connected, and keeps itself right. Turned off, the date and time are typed in by hand. The time zone also tells night light when the sun sets. Both need an administrator's password. Keeping the time from the internet needs systemd-timesyncd: sudo apt install systemd-timesyncd The clock in the middle of the ring is written in 24 or 12 hours, with or without seconds. Only the ring's clock follows this; dates and times in programs follow the formats chosen in Region and language.
# An item in the list of ways to write the ring's clock: { $time } is the time written that way and { $what } its name.
settings-datetime-clock-option = { $time }  ({ $what })
# Name of the list of ways to write the ring's clock.
settings-datetime-show-as = Show it as
# Heading of the line for the ring's clock.
settings-datetime-rings-clock = The ring's clock
# Under The ring's clock heading.
settings-datetime-rings-clock-sub = How the ring writes the time.
