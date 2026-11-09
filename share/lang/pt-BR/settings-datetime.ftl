
# machine translated (Brazilian Portuguese), not yet reviewed by a person.

# Name of the Date and time page, in the page list and as its heading.
settings-datetime-title = Data e hora

# Words people may search for to find the Date and time page, separated by spaces.
settings-datetime-keywords = fuso horário relógio data hora

# Help line at the foot of the Date and time page.
# SAFETY: needs a human review before it ships
settings-datetime-help = O fuso horário e a hora automática precisam da senha de um administrador.

# Spoken name of the picture of the clock at the top of the page.
settings-datetime-time-now = A hora agora

# Name of a way to write the ring's clock: hours 00 to 23.
settings-datetime-24 = 24 horas

# Name of a way to write the ring's clock: hours 1 to 12 with am or pm.
settings-datetime-12 = 12 horas

# Name of a way to write the ring's clock: hours 00 to 23 with seconds.
settings-datetime-24s = 24 horas, com segundos

# Name of the box for the day when the date is set by hand.
settings-datetime-day = Dia

# Name of the box for the month when the date is set by hand.
settings-datetime-month = Mês

# Name of the box for the year when the date is set by hand.
settings-datetime-year = Ano

# Name of the box for the hour when the time is set by hand.
settings-datetime-hour = Hora

# Name of the box for the minute when the time is set by hand.
settings-datetime-minute = Minuto

# Red words when the time could not be set.
settings-datetime-no-time = Não foi possível acertar a hora.

# Red words when the time was set by hand while automatic time is on.
settings-datetime-keeps-right = O relógio está se acertando sozinho agora. Desligue isso acima e depois acerte a hora à mão.

# Red words when the system's message bus could not be reached.
settings-datetime-no-bus = Não foi possível acessar o barramento do sistema.

# Red words when the time service does not answer.
settings-datetime-not-answering = O serviço de hora não está respondendo.

# Red words when automatic time could not be turned on.
settings-datetime-ntp-on-failed = Não foi possível mandar o relógio se acertar sozinho.

# Red words when automatic time could not be turned off.
settings-datetime-ntp-off-failed = Não foi possível desligar a hora automática.

# Red words when the time zone could not be changed.
settings-datetime-zone-failed = Não foi possível mudar o fuso horário.

# Red words when the date typed by hand is not a real date.
settings-datetime-not-real = Essa não é uma data real.

# Headline of the page before the time service has answered: { $time } is the time and { $date } the date.
settings-datetime-it-is = São { $time } de { $date }.

# Headline of the page when the time service is not answering.
settings-datetime-service-down = O serviço de hora não está respondendo, então o relógio não pode ser mudado aqui.

# Headline of the page when the clock is automatic and in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-right = São { $time } de { $date }, e o relógio se acerta sozinho.

# Headline of the page when the clock is automatic but not yet in step: { $time } is the time and { $date } the date.
settings-datetime-it-is-checks = São { $time } de { $date }. O relógio se confere assim que chega à internet.

# Headline of the page when the clock is set by hand: { $time } is the time and { $date } the date.
settings-datetime-it-is-hand = São { $time } de { $date }. O relógio foi acertado à mão.

# A city and how far its time is from Greenwich.
settings-datetime-city-offset = { $city }, { $offset }

# A city, its part of the world and how far its time is from Greenwich.
settings-datetime-city-region-offset = { $city }, { $region }, { $offset }

# Spoken name of the clock picture: the time, the date and the place.
settings-datetime-tip = São { $time } de { $date }, { $zone }.

# Help text of the clock card when the time service is not answering or not yet read.
settings-datetime-ring-about = O relógio no meio do anel é escrito em 24 ou 12 horas, com ou sem segundos. Só o relógio do anel segue isso; datas e horas nos programas seguem os formatos escolhidos em Região e idioma.

# Heading of the card with the clock settings.
settings-datetime-the-clock = O relógio

# Heading of the line shown when the time service is not answering.
settings-datetime-service-down-line = O serviço de hora não está respondendo

# Under Set the time automatically when the internet can be used.
settings-datetime-from-internet = Pela internet, quando conectado.

# Under Set the time automatically when the time service is not installed.
settings-datetime-needs-timesyncd = Precisa do systemd-timesyncd instalado.

# Label of the switch for automatic time, and heading of its line.
settings-datetime-set-auto = Acertar a hora automaticamente

# Name of the list of time zones, and heading of its line.
settings-datetime-time-zone = Fuso horário

# Tip of the button that sets the date and time typed by hand.
settings-datetime-set-date-time = Acertar a data e a hora

# Heading of the line for setting the time by hand.
settings-datetime-by-hand = Acertar a hora à mão

# Under the Set the time by hand heading.
settings-datetime-by-hand-sub = Desligue antes a hora automática, como está agora.

# Help text of the clock card (the information button) when the time service can keep the time from the internet.
# SAFETY: needs a human review before it ships
settings-datetime-about = Em modo automático, o relógio é conferido com servidores de hora na internet sempre que o computador está conectado e se mantém certo. Desligado, a data e a hora são digitadas à mão. O fuso horário também diz à luz noturna quando o sol se põe. Ambos precisam da senha de um administrador. O relógio no meio do anel é escrito em 24 ou 12 horas, com ou sem segundos. Só o relógio do anel segue isso; datas e horas nos programas seguem os formatos escolhidos em Região e idioma.

# Help text of the clock card (the information button) when the time service is not installed.
# SAFETY: needs a human review before it ships
settings-datetime-about-no-ntp = Em modo automático, o relógio é conferido com servidores de hora na internet sempre que o computador está conectado e se mantém certo. Desligado, a data e a hora são digitadas à mão. O fuso horário também diz à luz noturna quando o sol se põe. Ambos precisam da senha de um administrador. Manter a hora pela internet precisa do systemd-timesyncd: sudo apt install systemd-timesyncd O relógio no meio do anel é escrito em 24 ou 12 horas, com ou sem segundos. Só o relógio do anel segue isso; datas e horas nos programas seguem os formatos escolhidos em Região e idioma.

# An item in the list of ways to write the ring's clock: { $time } is the time written that way and { $what } its name.
settings-datetime-clock-option = { $time }  ({ $what })

# Name of the list of ways to write the ring's clock.
settings-datetime-show-as = Mostrar como

# Heading of the line for the ring's clock.
settings-datetime-rings-clock = O relógio do anel

# Under The ring's clock heading.
settings-datetime-rings-clock-sub = Como o anel escreve a hora.
