
# machine translated (Spanish), not yet reviewed by a person.

# Name of the About page in the Settings side bar and its title.
settings-about-title = Acerca de este equipo

# Words the Settings search matches the About page by, in lower case.
settings-about-keywords = hostname computer name network name about system version debian trindle memory ram processor cpu disk graphics screen model kernel rolling release activity running programs busy task manager system monitor disks drives usb space nombre del equipo red acerca sistema versión memoria procesador disco gráficos pantalla modelo núcleo actividad programas en ejecución monitor unidades espacio

# Text behind the info button of the About page.
settings-about-help = El nombre de este equipo tal como lo ven otros equipos de la red, de qué está hecho y cuánto tarda en arrancar. Lo que está haciendo ahora, y qué programas usan la memoria y el procesador, está en Actividad, y las unidades y su espacio en Unidades, cada una en su propia ventana: las flechas de Más detalle las abren.

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
settings-about-name-bad = Solo letras, números y guiones, sin empezar ni terminar con un guion.

# SAFETY: needs a human review before it ships
# Words under the computer name box while the password window is open.
settings-about-name-waiting = Esperando la contraseña...

# Red words under the computer name box when renaming failed.
settings-about-name-failed = No se pudo cambiar el nombre del equipo.

# Words under the computer name box after renaming worked.
settings-about-name-done = Los programas que ya están abiertos verán el nombre nuevo tras el próximo inicio de sesión.

# Placeholder and spoken name of the typing box for the computer's name.
settings-about-name-entry = Nombre del equipo

# Tip of the save button beside the computer name box.
settings-about-rename = Cambiar el nombre de este equipo

# Bold line of the computer name setting.
settings-about-name = Nombre del equipo

# Quiet line under the computer name setting.
settings-about-name-sub = Letras, números y guiones.

# Heading of the card with the computer's name.
settings-about-name-group = Nombre

# SAFETY: needs a human review before it ships
# Text behind the info button of the Name card.
settings-about-name-about = Cómo aparece este equipo en la red y en el indicador de una terminal. Letras, números y guiones, sin empezar ni terminar con un guion. Cambiarlo pide tu contraseña.

# Heading of the card about the computer's hardware.
settings-about-hardware = El equipo

# Text behind the info button of the hardware card.
settings-about-hardware-about = El fabricante y el modelo, el procesador y cuántos hilos ejecuta a la vez, los gráficos y cada pantalla en uso con su tamaño y su frecuencia de actualización.

# Quiet line under the maker and model.
settings-about-hw-computer = El equipo

# Quiet line under the processor.
settings-about-hw-processor = El procesador

# Quiet line under the graphics.
settings-about-hw-graphics = Dibuja las imágenes

# Quiet line under the screens.
settings-about-hw-screen = La pantalla

# Heading of the card about the software.
settings-about-software = El software

# Text behind the info button of the software card.
settings-about-software-about = Trindle es un escritorio sobre Debian, que se ocupa de los programas de debajo. El núcleo es la parte que habla con el hardware, y el compositor propio de Trindle dibuja el escritorio, sobre Wayland.

# Quiet line under the Debian version.
settings-about-sw-debian = Funciona por debajo

# Quiet line under the kernel version.
settings-about-sw-kernel = El núcleo

# Bold line naming Trindle's compositor.
settings-about-sw-compositor = El compositor propio de Trindle

# Quiet line under the compositor line.
settings-about-sw-compositor-sub = Dibuja el escritorio, sobre Wayland

# Bold line saying how many apps are installed; n is the count.
settings-about-sw-apps =
    { $n ->
        [one] { $n } aplicación
        [many] { $n } aplicaciones
       *[other] { $n } aplicaciones
    }

# Quiet line under the number of apps.
settings-about-sw-apps-sub = Instaladas, listas en el anillo

# Heading of the card with the ways to Activity and Drives.
settings-about-more = Más detalle

# Text behind the info button of the More detail card.
settings-about-more-about = Lo que está haciendo ahora, y qué programas usan la memoria y el procesador, está en Actividad. Cada unidad, cuánto se ha llenado y en qué estado está, y las memorias USB, están en Unidades.

# Bold line of the setting that opens Activity.
settings-about-activity = Ver qué se está ejecutando

# Quiet line under the setting that opens Activity.
settings-about-activity-sub = Programas que usan memoria y procesador.

# Tip of the arrow that opens the Activity program.
settings-about-activity-name = Actividad

# Bold line of the setting that opens Drives.
settings-about-drives = Unidades y espacio

# Quiet line under the setting that opens Drives.
settings-about-drives-sub = Cada unidad, cuánto se ha llenado y su estado.

# Tip of the arrow that opens the Drives program.
settings-about-drives-name = Unidades

# Red words under the start-up times line when the file could not be saved.
settings-about-times-failed = No se pudieron guardar los tiempos de arranque en Descargas.

# Words under the start-up times line after the file was saved; file is the file's name.
settings-about-times-saved = En Descargas, como { $file }

# Label of the word button that saves every start-up time.
settings-about-times-save = Guardar

# Spoken name of the Save button for the start-up times.
settings-about-times-save-spoken = Guardar todos los tiempos de arranque en Descargas

# Bold line of the setting that saves the start-up times.
settings-about-times-keep = Todos los arranques, para conservar

# Quiet line under that setting.
settings-about-times-keep-sub = Se guardan en Descargas como una hoja de cálculo.

# The sentence over the chart of the last ten start-up times.
settings-about-times-chart = Cada columna es un arranque, el más reciente a la derecha.

# Heading of the card with the last ten start-up times.
settings-about-times = Los últimos diez arranques

# Text behind the info button of the start-up times card.
settings-about-times-about = Cada arranque se mide desde el botón de encendido hasta la pantalla de inicio de sesión, sin contar el tiempo de escribir la contraseña del disco. El más reciente está arriba.

# Heading of the memory tile.
settings-about-memory = Memoria

# Heading of the disk tile.
settings-about-disk = Disco



# Heading of the card with the switch for looking for updates.
settings-about-updates = Actualizaciones

# Text behind the info button of the updates card.
settings-about-updates-about = Trindle busca versiones nuevas de las aplicaciones, los complementos, el firmware y de sí mismo poco después de que inicies sesión y cada pocas horas, y las correcciones de seguridad de Debian entran todos los días. Si se desactiva, solo busca cuando pulsas Buscar actualizaciones en Aplicaciones, o el botón de actualizaciones del anillo. Las correcciones de seguridad de Debian tienen su propio interruptor; si se desactiva, ya no se instalan por sí solas y los fallos conocidos siguen abiertos hasta que actualices. Cambiar cualquiera de los dos interruptores pide la contraseña de administrador, para que los servicios propios del equipo también vean la decisión.

# Bold line of the switch for looking for updates by itself; also the switch's name for a screen reader.
settings-about-updates-by-itself = Buscar actualizaciones

# Words under the Check for updates switch while the password window is open.
settings-about-updates-waiting = Esperando la contraseña...

# Quiet line under that switch.
settings-about-updates-by-itself-sub = Desactivado, nada busca si no lo pides.



# Bold line of the switch for Debian's security fixes; also the switch's name for a screen reader.
settings-about-security = Actualizaciones de seguridad

# Quiet line under the Security updates switch when it is on.
settings-about-security-sub = Las correcciones de seguridad se instalan solas todos los días.

# Warning line under the Security updates switch when it is off.
settings-about-security-off = Las correcciones de seguridad ya no se instalan solas.

