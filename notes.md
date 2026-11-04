# To Do 

- change in "edit projects" implementieren
- Settings einbauen
- > manual input -> struct anpassen
- > notifications -> struct anpassen
- ssh einbauen -> struct anpassen
- die "pick_folder" Funktion muss leider für alle OS einzeln gebaut werden damit sie ordentlich funktioniert


# Ideas
- Automatische übertragung über ftp oder ssh? vielleicht nutzlos
- NAME: **File Moth**
- Die System Notes bekommen als Image einen Mottenkopf!!!
- **Settings:** (Notifications disable), (enable manual input/disable file picker)
- Verschlüsselung iwie unterstützen maybe 
- passkey für einlogen ins programm
- **archiv mode:** es werden komprimierte datein gesichert + option das mehrere snapshots gespeichert werden

# Knowledge
- what is Serealization: https://medium.com/@murataslan1/serialization-and-deserialization-in-rust-a-comprehensive-guide-3eb249ae8ac6

# Dameon mode

Die Optionen könnten so aussehen: 
**There are multiple options how to use the daemon mode**
- Safe x times / day
- Safe x times / week 
- Safe x times / month
- Safe at XX:XX every day
- Safe every Weekday x at XX:XX
- Safe every XX.XX. of the Month at the time XX:XX

Damit das program weiß wann es schauen muss welche projekte gesichert werden, wird ein array geführt, in dem die sich ergebenden speicherzeiten gespeichert werden. 
z.B.: [00:00, 06:00, 12:00, 18:00]
oder [00:00, 06:00, 07:30, 12:00, 15:09, 18:00]
Es gibt also einen array der alle speciherzeiten speichert.
Zu diesen Zeiten schaut das program welche projekte zu diesen zeiten gespeichert werden müssen. 
-> Dadurch muss nicht jede minute geschaut werden ob ein projekt gespeichert werden muss.

# Log & Config Files
- Für alles OS muss ich passende Ordner finden wo die Logs und die Config hinkommen

**Mac**
- https://stackoverflow.com/questions/15677388/default-location-for-configuration-files-macos 
- https://developer.apple.com/library/archive/documentation/General/Conceptual/MOSXAppProgrammingGuide/AppRuntime/AppRuntime.html

# Homebrew
- nicht unbedingt notwendig aber wär fresh wenns app da gibt
- Anleitung:   https://docs.brew.sh/Adding-Software-to-Homebrew
