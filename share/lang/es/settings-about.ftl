
# machine translated (Spanish), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Acerca de este equipo

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = hostname computer name network name about system version debian trindle memory ram processor cpu disk graphics screen model kernel rolling release activity running programs busy task manager system monitor disks drives usb space nombre del equipo red acerca sistema versión memoria procesador disco gráficos pantalla modelo núcleo actividad programas en ejecución monitor unidades espacio

# Text behind the info button of the About page.
settings-about-help = Esta página muestra el nombre de este equipo tal como lo ven otros equipos de la red, de qué está hecho y cuánto tarda en arrancar. Actividad muestra lo que hace el equipo ahora y qué programas usan la memoria y el procesador. Unidades muestra tus unidades y su espacio. Cada una se abre en su propia ventana. Usa las flechas de Más detalle para abrirlas.

# What is shown when a fact about the computer cannot be found.
settings-about-unknown = Desconocido

# The processor line; model is its name and cores how many threads it runs at once.
settings-about-processor =
    { $cores ->
        [one] { $model }, { $cores } hilo
        [many] { $model }, { $cores } hilos
       *[other] { $model }, { $cores } hilos
    }

# The graphics line for an Intel graphics chip.
settings-about-gpu-intel = Gráficos Intel

# The graphics line for an AMD graphics chip.
settings-about-gpu-amd = Gráficos AMD

# The graphics line for an NVIDIA graphics chip.
settings-about-gpu-nvidia = Gráficos NVIDIA

# The graphics line inside a virtual machine.
settings-about-gpu-virtual = Gráficos de máquina virtual

# The graphics line for a graphics chip of a kind Trindle does not name.
settings-about-gpu-other = Gráficos

# One screen in use: its size in pixels and how many times a second it redraws.
settings-about-screen = { $width } × { $height } a { $hz } Hz

# Figure on the memory tile: how much memory is in use now; amount is like 5.0 GB.
settings-about-in-use = { $amount } en uso

# Figure on the disk tile: how much disk is free; amount is like 290 GB.
settings-about-free = { $amount } libres

# Words under the figure on the memory and disk tiles: the whole size; amount is like 16 GB.
settings-about-of = de { $amount }

# The sentence at the top of About; version is Trindle's version and on is how long the computer has been on, like "3 hours".
settings-about-said = Trindle { $version }, encendido desde hace { $on }.

# The same sentence at the top of About when the disk is nearly full.
settings-about-said-full = Trindle { $version }, encendido desde hace { $on }. El disco está casi lleno.

# How long a start took, under a minute, in the list of starts; n is like 9.8.
settings-about-boot-seconds = { $n } segundos

# How long a start took, a minute or more, in the list of starts; n is like 1:05.2.
settings-about-boot-minutes = { $n } minutos

# How long the computer has been switched on, under an hour.
settings-about-up-minutes =
    { $n ->
        [one] { $n } minuto
        [many] { $n } minutos
       *[other] { $n } minutos
    }

# How long the computer has been switched on, under two days.
settings-about-up-hours =
    { $n ->
        [one] { $n } hora
        [many] { $n } horas
       *[other] { $n } horas
    }

# How long the computer has been switched on, two days or more.
settings-about-up-days =
    { $n ->
        [one] { $n } día
        [many] { $n } días
       *[other] { $n } días
    }

# Red words under the computer name box when the name is not allowed.
settings-about-name-bad = Usa solo letras, números y guiones. No empieces ni termines con un guion.

# SAFETY: needs a human review before it ships
# Words under the computer name box while the password window is open.
settings-about-name-waiting = Esperando tu contraseña…

# Red words under the computer name box when renaming failed.
settings-about-name-failed = No se pudo cambiar el nombre del equipo.

# Words under the computer name box after renaming worked.
settings-about-name-done = Los programas que ya están abiertos usarán el nombre nuevo después de que vuelvas a iniciar sesión.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Nombre del equipo

# Tip of the save button beside the computer name box.
settings-about-rename = Cambiar el nombre de este equipo

# Bold line of the computer name setting.
settings-about-name = Nombre del equipo

# Heading of the card with the computer's name.
settings-about-name-group = Nombre

# SAFETY: needs a human review before it ships
# Text behind the info button of the Name card.
settings-about-name-about = Este es el nombre del equipo en la red y en el indicador de una terminal. Usa letras, números y guiones, pero no empieces ni termines con un guion. Para cambiarlo tienes que escribir tu contraseña.

# Heading of the card about the computer's hardware.
settings-about-hardware = El equipo

# Text behind the info button of the hardware card.
settings-about-hardware-about = Aquí se ven el fabricante y el modelo del equipo, su procesador y sus gráficos. También se ve cuántos hilos puede ejecutar a la vez el procesador, y cada pantalla en uso con su tamaño y su frecuencia de actualización.

# Heading of the card about the software.
settings-about-software = El software

# Text behind the info button of the software card.
settings-about-software-about = Trindle es un escritorio que funciona sobre Debian, que se ocupa de los programas de debajo. El núcleo es la parte que trabaja con el hardware. El compositor propio de Trindle dibuja el escritorio con Wayland.

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = El compositor propio de Trindle

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } aplicación
        [many] { $n } aplicaciones
       *[other] { $n } aplicaciones
    }

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Más detalle

# Text behind the info button of the More detail card.
settings-about-more-about = Actividad muestra lo que hace el equipo ahora y qué programas usan la memoria y el procesador. Unidades muestra todas las unidades, cuánto se han llenado y en qué estado están, y las memorias USB.

# Bold line of the setting that opens Activity.
settings-about-activity = Ver qué se está ejecutando

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Actividad

# Bold line of the setting that opens Drives.
settings-about-drives = Unidades y espacio

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Unidades

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = Trindle no pudo guardar los tiempos de arranque en Descargas.

# First line of the saved start-up times file: the two columns, separated by a comma.
settings-about-times-heading = Fecha y hora,Segundos

# File name of the saved start-up times, in Downloads.
settings-about-times-file = Tiempos de inicio de Trindle.csv

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = En Descargas, como { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = Guardar

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Guardar todos los tiempos de arranque en Descargas

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Guardar cada tiempo de arranque

# Quiet line under that setting.
settings-about-times-keep-sub = Se guardan en Descargas como una hoja de cálculo.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Cada columna es un arranque. El más reciente está a la derecha.

# Heading of the card with the last ten start-up times.
settings-about-times = Últimos diez arranques

# Text behind the info button of the start-up times card.
settings-about-times-about = Trindle mide cada arranque desde el botón de encendido hasta la pantalla de inicio de sesión. No cuenta el tiempo que tardas en escribir la contraseña del disco. El más reciente está arriba.

# Heading of the memory tile.
settings-about-memory = Memoria

# Heading of the disk tile.
settings-about-disk = Disco

# Heading of the card on About that holds the switch and the optional names for problem reports.
settings-about-reports = Informes de problemas

# Text behind the round i of that card. Buildmorphic is the community that helps build Trindle; keep its name.
settings-about-reports-about = Los informes van a Buildmorphic, la comunidad que ayuda a crear Trindle. Puedes unirte solo con una dirección de correo. Ganas crédito y niveles por los problemas que informas primero, aunque no sepas arreglarlos. Un informe contiene solo una lista fija de datos, y no se envía nada hasta que pulsas Enviar. Un nombre de usuario o de GitHub es opcional, y tu nombre de usuario nunca es tu dirección de correo.

# Switch: whether Trindle asks about problems and about sharing reports. Name of the switch for the screen reader too.
settings-about-reports-offer = Ofrecer enviar un informe cuando algo sale mal

# Box for the optional Buildmorphic username, also its name for the screen reader. Buildmorphic is a name.
settings-about-reports-name = Nombre de usuario de Buildmorphic

# Box for the optional GitHub name, also its name for the screen reader. GitHub is a name.
settings-about-reports-github = Nombre de GitHub

# Name of the round button that opens the Buildmorphic community website, and of its line.
settings-about-reports-community = Sobre la comunidad

# Said under the switch when it is turned off and reports that were waiting to be sent were deleted; n is how many.
settings-about-reports-deleted =
    { $n ->
        [one] Desactivado. { $n } informe en espera borrado.
       *[other] Desactivado. { $n } informes en espera borrados.
    }

# Said in red under the switch when it is turned off but some waiting reports could not be deleted; n is how many.
settings-about-reports-kept =
    { $n ->
        [one] Desactivado. No se pudo borrar { $n } informe en espera.
       *[other] Desactivado. No se pudieron borrar { $n } informes en espera.
    }



# Heading of the card with the switch for looking for updates.
settings-about-updates = Actualizaciones

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle busca versiones nuevas de las aplicaciones, los complementos, el firmware y de sí mismo poco después de que inicies sesión y luego cada pocas horas. Si lo desactivas, Trindle solo busca cuando pulsas Buscar actualizaciones en Aplicaciones, o el botón de actualizaciones del anillo. Las correcciones de seguridad de Debian se instalan todos los días y tienen su propio interruptor. Si lo desactivas, ya no se instalan por sí solas y los fallos de seguridad conocidos siguen sin corregir hasta que actualices. Cambiar cualquiera de los dos interruptores pide la contraseña de administrador, para que todo el equipo use tu decisión.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Buscar actualizaciones

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Esperando tu contraseña…



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Actualizaciones de seguridad

# Warning line under the Security updates switch when it is off.
settings-about-security-off = Las correcciones de seguridad ya no se instalan solas.

# Said in red under the switch when it is turned off but the Reporting add-on could not delete the reports it keeps; the reason follows.
settings-about-reports-addon-kept = Desactivado, pero no se borraron los informes en espera del complemento Reporting.
