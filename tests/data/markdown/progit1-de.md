<!--# Getting Started #-->
# Los geht’s #

<!--This chapter will be about getting started with Git.  We will begin at the beginning by explaining some background on version control tools, then move on to how to get Git running on your system and finally how to get it setup to start working with.  At the end of this chapter you should understand why Git is around, why you should use it and you should be all setup to do so.-->

In diesem Kapitel wird es darum gehen, wie man mit Git loslegen kann. Wir werden erläutern, wozu Versionskontrollsysteme gut sind, wie man Git auf verschiedenen Systemen installieren und konfigurieren kann, sodass man in der Lage ist, mit der Arbeit anzufangen. Am Ende dieses Kapitels solltest Du verstehen, wozu Git gut ist, weshalb Du es verwenden solltest und wie Du damit loslegen kannst.

<!--## About Version Control ##-->
## Wozu Versionskontrolle? ##

<!--What is version control, and why should you care? Version control is a system that records changes to a file or set of files over time so that you can recall specific versions later. Even though the examples in this book show software source code as the files under version control, in reality any type of file on a computer can be placed under version control.-->

Was ist  die Versionskontrolle, und warum solltest Du Dich dafür interessieren? Versionskontrollsysteme (VCS) protokollieren Änderungen an einer Datei oder einer Anzahl von Dateien über die Zeit hinweg, so dass man zu jedem Zeitpunkt auf Versionen und Änderungen zugreifen kann. Die Beispiele in diesem Buch verwenden Software Quellcode, tatsächlich aber kannst Du Änderungen an praktisch jeder Art von Datei per Versionskontrolle nachverfolgen.

<!--If you are a graphic or web designer and want to keep every version of an image or layout (which you certainly would), it is very wise to use a Version Control System (VCS). A VCS allows you to: revert files back to a previous state, revert the entire project back to a previous state, review changes made over time, see who last modified something that might be causing a problem, who introduced an issue and when, and more. Using a VCS also means that if you screw things up or lose files, you can generally recover easily. In addition, you get all this for very little overhead.-->

Als ein Grafik- oder Webdesigner zum Beispiel willst Du in der Lage sein, jede Version eines Bildes oder Layouts zurückverfolgen zu können. Es wäre daher sehr ratsam, ein Versionskontrollsystem zu verwenden. Ein solches System erlaubt Dir, einzelne Dateien oder auch ein ganzes Projekt in einen früheren Zustand zurückzuversetzen, nachzuvollziehen, wer zuletzt welche Änderungen vorgenommen hat, die möglicherweise Probleme verursachen, wer eine Änderung ursprünglich vorgenommen hat usw. Ein Versionskontrollsystem für Deine Arbeit zu verwenden, versetzt Dich in die Lage jederzeit zu einem vorherigen, funktionierenden Zustand zurückzugehen, wenn Du vielleicht Mist gebaut oder aus irgendeinem Grunde Dateien verloren hast. All diese Vorteile erhältst Du für einen nur sehr geringen, zusätzlichen Aufwand.

<!--### Local Version Control Systems ###-->
### Lokale Versionskontrollsysteme ###

<!--Many people’s version-control method of choice is to copy files into another directory (perhaps a time-stamped directory, if they’re clever). This approach is very common because it is so simple, but it is also incredibly error prone. It is easy to forget which directory you’re in and accidentally write to the wrong file or copy over files you don’t mean to.-->

Viele Leute kontrollieren Versionen ihrer Arbeit, indem sie einfach Dateien in ein anderes Verzeichnis kopieren (wenn sie clever sind: ein Verzeichnis mit einem Zeitstempel im Namen). Diese Vorgehensweise ist üblich, weil sie so einfach ist. Aber sie ist auch unglaublich fehleranfällig. Man vergisst sehr leicht, in welchem Verzeichnis man sich gerade befindet und kopiert die falschen Dateien oder überschreibt Dateien, die man eigentlich nicht überschreiben wollte.

<!--To deal with this issue, programmers long ago developed local VCSs that had a simple database that kept all the changes to files under revision control (see Figure 1-1).-->

Um diese Arbeit zu erleichtern und sicherer zu machen, haben Programmierer vor langer Zeit Versionskontrollsysteme entwickelt, die alle Änderungen an allen relevanten Dateien in einer lokalen Datenbank verfolgten (siehe Bild 1-1).

<!--Figure 1-1. Local version control diagram.-->

Insert 18333fig0101.png
Bild 1-1. Diagramm: Lokale Versionskontrolle

<!--One of the more popular VCS tools was a system called rcs, which is still distributed with many computers today. Even the popular Mac OS X operating system includes the rcs command when you install the Developer Tools. This tool basically works by keeping patch sets (that is, the differences between files) from one revision to another in a special format on disk; it can then recreate what any file looked like at any point in time by adding up all the patches.-->

Eines der populärsten Versionskontrollsysteme war rcs, und es wird heute immer noch mit vielen Computern ausgeliefert. Z.B. umfasst auch das Betriebssystem Mac OS X den Befehl rcs, wenn Du die Developer Tools installierst. Dieser Befehl arbeitet nach dem Prinzip, dass er für jede Änderung einen Patch (d.h. eine Kodierung der Unterschiede, die eine Änderung an einer oder mehreren Dateien umfasst) in einem speziellen Format in einer Datei auf der Festplatte speichert.

<!--### Centralized Version Control Systems ###-->
### Zentralisierte Versionskontrollsysteme ###

<!--The next major issue that people encounter is that they need to collaborate with developers on other systems. To deal with this problem, Centralized Version Control Systems (CVCSs) were developed. These systems, such as CVS, Subversion, and Perforce, have a single server that contains all the versioned files, and a number of clients that check out files from that central place. For many years, this has been the standard for version control (see Figure 1-2).-->

Das nächste Problem, mit dem Programmierer sich dann konfrontiert sahen, bestand in der Zusammenarbeit mit anderen: Änderungen an dem gleichen Projekt mussten auf verschiedenen Computern, möglicherweise verschiedenen Betriebssystemen vorgenommen werden können. Um dieses Problem zu lösen, wurden Zentralisierte Versionskontrollsysteme (CVCS) entwickelt. Diese Systeme, beispielsweise CVS, Subversion und Perforce, basieren auf einem zentralen Server, der alle versionierten Dateien verwaltet. Wer an diesen Dateien arbeiten will, kann sie von diesem Server abholen („check out“ xxx), auf seinem eigenen Computer bearbeiten und dann wieder auf dem Server abliefern. Diese Art von System war über viele Jahre hinweg der Standard für Versionskontrollsysteme (siehe Bild 1-2).

<!--Figure 1-2. Centralized version control diagram.-->

Insert 18333fig0102.png
Bild 1-2. Diagramm: Zentralisierte Versionskontrollsysteme

<!--This setup offers many advantages, especially over local VCSs. For example, everyone knows to a certain degree what everyone else on the project is doing. Administrators have fine-grained control over who can do what; and it’s far easier to administer a CVCS than it is to deal with local databases on every client.-->

Dieser Aufbau hat viele Vorteile gegenüber Lokalen Versionskontrollsystemen. Zum Beispiel weiß jeder mehr oder weniger genau darüber Bescheid, was andere, an einem Projekt Beteiligte gerade tun. Administratoren haben die Möglichkeit, detailliert festzulegen, wer was tun kann. Und es ist sehr viel einfacher, ein CVCS zu administrieren als lokale Datenbanken auf jedem einzelnen Anwenderrechner zu verwalten.

<!--However, this setup also has some serious downsides. The most obvious is the single point of failure that the centralized server represents. If that server goes down for an hour, then during that hour nobody can collaborate at all or save versioned changes to anything they’re working on. If the hard disk the central database is on becomes corrupted, and proper backups haven’t been kept, you lose absolutely everything—the entire history of the project except whatever single snapshots people happen to have on their local machines. Local VCS systems suffer from this same problem—whenever you have the entire history of the project in a single place, you risk losing everything.-->

Allerdings hat dieser Aufbau auch einige erhebliche Nachteile. Der offensichtlichste Nachteil ist der „Single Point of Failure“, den der zentralisierte Server darstellt. Wenn dieser Server für nur eine Stunde nicht verfügbar ist, dann kann in dieser Stunde niemand in irgendeiner Form mit anderen arbeiten oder versionierte Änderungen an den Dateien speichern, an denen sie momentan arbeiten. Wenn die auf dem zentralen Server verwendete Festplatte beschädigt wird und keine Sicherheitskopien erstellt wurden, dann sind all diese Daten unwiederbringlich verloren – die komplette Historie des Projektes, abgesehen natürlich von dem jeweiligen Zustand, den Mitarbeiter gerade zufällig auf ihrem Rechner haben. Lokale Versionskontrollsysteme haben natürlich dasselbe Problem: wenn man die Historie eines Projektes an einer einzigen, zentralen Stelle verwaltet, riskiert man, sie vollständig zu verlieren, wenn irgendetwas an dieser zentralen Stelle ernsthaft schief läuft.

<!--### Distributed Version Control Systems ###-->
### Verteilte Versionskontrollsysteme ###

<!--This is where Distributed Version Control Systems (DVCSs) step in. In a DVCS (such as Git, Mercurial, Bazaar or Darcs), clients don’t just check out the latest snapshot of the files: they fully mirror the repository. Thus if any server dies, and these systems were collaborating via it, any of the client repositories can be copied back up to the server to restore it. Every checkout is really a full backup of all the data (see Figure 1-3).-->

Und an dieser Stelle kommen verteilte Versionskontrollsysteme (DVCS) ins Spiel. In einem DVCS (wie z.B. Git, Mercurial, Bazaar oder Darcs) erhalten Anwender nicht einfach den jeweils letzten Snapshot des Projektes von einem Server: sie erhalten statt dessen eine vollständige Kopie des Repositories. Auf diese Weise kann, wenn ein Server beschädigt wird, jedes beliebige Repository von jedem beliebigen Anwenderrechner zurück kopiert werden und der Server so wieder hergestellt werden.

<!--Figure 1-3. Distributed version control diagram.-->

Insert 18333fig0103.png
Bild 1-3. Diagramm: Distribuierte Versionskontrolle

<!--Furthermore, many of these systems deal pretty well with having several remote repositories they can work with, so you can collaborate with different groups of people in different ways simultaneously within the same project. This allows you to set up several types of workflows that aren’t possible in centralized systems, such as hierarchical models.-->

Darüber hinaus können derartige Systeme hervorragend mit verschiedenen externen („remote“) Repositories umgehen, sodass man mit verschiedenen Gruppen von Leuten simultan in verschiedenen Weisen zusammenarbeiten kann. Das macht es möglich, verschiedene Arten von Arbeitsabläufen (wie Hierarchien) zu integrieren, was mit zentralisierten Systemen nicht möglich ist.

<!--## A Short History of Git ##-->
## Die Geschichte von Git ##

<!--As with many great things in life, Git began with a bit of creative destruction and fiery controversy. The Linux kernel is an open source software project of fairly large scope. For most of the lifetime of the Linux kernel maintenance (1991–2002), changes to the software were passed around as patches and archived files. In 2002, the Linux kernel project began using a proprietary DVCS system called BitKeeper.-->

Wie viele großartige Dinge im Leben entstand Git aus kreativem Chaos und hitziger Diskussion. Der Linux Kernel ist ein Open Source Software Projekt von erheblichem Umfang. Während der gesamten Entwicklungszeit des Linux Kernels von 1991 bis 2002 wurden Änderungen an der Software in Form von Patches (d.h. Änderungen an bestehendem Code) und archivierten Dateien herumgereicht. 2002 began man dann, ein proprietäres DVCS System mit dem Namen „Bitkeeper“ zu verwenden.

<!--In 2005, the relationship between the community that developed the Linux kernel and the commercial company that developed BitKeeper broke down, and the tool’s free-of-charge status was revoked. This prompted the Linux development community (and in particular Linus Torvalds, the creator of Linux) to develop their own tool based on some of the lessons they learned while using BitKeeper. Some of the goals of the new system were as follows:-->

2005 ging die Beziehung zwischen der Community, die den Linux Kernel entwickelte, und des kommerziell ausgerichteten Unternehmens, das BitKeeper entwickelte, kaputt. Die zuvor ausgesprochene Erlaubnis, BitKeeper kostenlos zu verwenden, wurde widerrufen. Dies war für die Linux Entwickler Community (und besonders für Linus Torvald, der Erfinder von Linux) der Auslöser dafür, ein eigenes Tool zu entwickeln, das auf den Erfahrungen mit BitKeeper basierte. Ziele des neuen Systems waren unter anderem:

<!--*	Speed-->
<!--*	Simple design-->
<!--*	Strong support for non-linear development (thousands of parallel branches)-->
<!--*	Fully distributed-->
<!--*	Able to handle large projects like the Linux kernel efficiently (speed and data size)-->

* Geschwindigkeit
* Einfaches Design
* Gute Unterstützung von nicht-linearer Entwicklung (tausende paralleler Branches, d.h. verschiedener Verzweigungen der Versionen)
* Vollständig verteilt
* Fähig, große Projekte wie den Linux Kernel effektiv zu verwalten (Geschwindigkeit und Datenumfang)

<!--Since its birth in 2005, Git has evolved and matured to be easy to use and yet retain these initial qualities. It’s incredibly fast, it’s very efficient with large projects, and it has an incredible branching system for non-linear development (See Chapter 3).-->

Seit seiner Geburt 2005 entwickelte sich Git kontinuierlich weiter und reifte zu einem System heran, das einfach zu bedienen ist, die ursprünglichen Ziele dabei aber weiter beibehält. Es ist unglaublich schnell, äußerst effizient, wenn es um große Projekte geht, und es hat ein fantastisches Branching Konzept für nicht-lineare Entwicklung (mehr dazu in Kapitel 3).

<!--## Git Basics ##-->
## Git Grundlagen ##

<!--So, what is Git in a nutshell? This is an important section to absorb, because if you understand what Git is and the fundamentals of how it works, then using Git effectively will probably be much easier for you. As you learn Git, try to clear your mind of the things you may know about other VCSs, such as Subversion and Perforce; doing so will help you avoid subtle confusion when using the tool. Git stores and thinks about information much differently than these other systems, even though the user interface is fairly similar; understanding those differences will help prevent you from becoming confused while using it.-->

Was also ist Git, in kurzen Worten? Es ist wichtig, den folgenden Abschnitt zu verstehen, in dem es um die grundlegenden Konzepte von Git geht. Das wird Dich in die Lage versetzen, Git einfacher und effektiver anzuwenden. Versuche Dein vorhandenes Wissen über andere Versionskontrollsysteme, wie Subversion oder Perforce, zu ignorieren, während Du Git kennen lernst. Git speichert und konzipiert Information anders als andere Systeme, auch wenn das Interface relativ ähnlich wirkt. Diese Unterschiede zu verstehen wird Dir helfen, Verwirrung bei der Anwendung von Git zu vermeiden.

<!--### Snapshots, Not Differences ###-->
### Snapshots, nicht Diffs ###

<!--The major difference between Git and any other VCS (Subversion and friends included) is the way Git thinks about its data. Conceptually, most other systems store information as a list of file-based changes. These systems (CVS, Subversion, Perforce, Bazaar, and so on) think of the information they keep as a set of files and the changes made to each file over time, as illustrated in Figure 1-4.-->

Der Hauptunterschied zwischen Git und anderen Versionskontrollsystemen (auch Subversion und vergleichbaren Systemen) besteht in der Art und Weise wie Git Daten betrachtet. Die meisten anderen Systeme speichern Information als eine fortlaufende Liste von Änderungen an Dateien („Diffs“). Diese Systeme (CVS, Subversion, Perforce, Bazaar usw.) betrachten die Informationen, die sie verwalten, als eine Menge von Dateien und die Änderungen, die über die Zeit hinweg an einzelnen Dateien vorgenommen werden. (Siehe Bild 1-4.)

<!--Figure 1-4. Other systems tend to store data as changes to a base version of each file.-->

Insert 18333fig0104.png
Bild 1-4. Andere Systeme speichern Daten als Änderungen an einzelnen Dateien einer Datenbasis

<!--Git doesn’t think of or store its data this way. Instead, Git thinks of its data more like a set of snapshots of a mini filesystem. Every time you commit, or save the state of your project in Git, it basically takes a picture of what all your files look like at that moment and stores a reference to that snapshot. To be efficient, if files have not changed, Git doesn’t store the file again—just a link to the previous identical file it has already stored. Git thinks about its data more like Figure 1-5.-->

Git sieht Daten nicht in dieser Weise. Stattdessen betrachtet Git seine Daten eher als eine Reihe von Snapshots eines Mini-Dateisystems. Jedes Mal, wenn Du committest (d.h. den gegenwärtigen Status Deines Projektes als eine Version in Git speicherst), sichert Git den Zustand sämtlicher Dateien in diesem Moment („Snapshot“) und speichert eine Referenz auf diesen Snapshot. Um dies möglichst effizient und schnell tun zu können, kopiert Git unveränderte Dateien nicht, sondern legt lediglich eine Verknüpfung zu der vorherigen Version der Datei an. Git betrachtet Daten also wie in Bild 1-5 dargestellt.

<!--Figure 1-5. Git stores data as snapshots of the project over time.-->

Insert 18333fig0105.png
Bild 1-5. Git speichert Daten als eine Historie von Snapshots des Projektes.

<!--This is an important distinction between Git and nearly all other VCSs. It makes Git reconsider almost every aspect of version control that most other systems copied from the previous generation. This makes Git more like a mini filesystem with some incredibly powerful tools built on top of it, rather than simply a VCS. We’ll explore some of the benefits you gain by thinking of your data this way when we cover Git branching in Chapter 3.-->

Dies ist ein wichtiger Unterschied zwischen Git und praktisch allen anderen Versionskontrollsystemen. In Git wurden daher fast alle Aspekte der Versionskontrolle neu überdacht, die in anderen Systemen mehr oder weniger von ihren jeweiligen Vorgängergeneration übernommen worden waren. Git arbeitet im Großen und Ganzen eher wie ein (mit einigen unglaublich mächtigen Werkezeugen ausgerüstetes) Mini-Dateisystem, als wie ein gängiges Versionskontrollsystem. Auf einige der Vorteile, die es mit sich bringt, Daten in dieser Weise zu betrachten, werden wir in Kapitel 3 eingehen, wenn wir das Git Branching Konzept diskutieren.

<!--### Nearly Every Operation Is Local ###-->
### Fast jede Operation ist lokal ###

<!--Most operations in Git only need local files and resources to operate — generally no information is needed from another computer on your network.  If you’re used to a CVCS where most operations have that network latency overhead, this aspect of Git will make you think that the gods of speed have blessed Git with unworldly powers. Because you have the entire history of the project right there on your local disk, most operations seem almost instantaneous.-->

Die meisten Operationen in Git benötigen nur die lokalen Dateien und Ressourcen auf Deinem Rechner, um zu funktionieren. D.h. im Allgemeinen werden keine Informationen von einem anderen Rechner im Netzwerk benötigt. Wenn Du mit einem CVCS gearbeitet hast, das für die meisten Operationen einen Network Latency Overhead (also Wartezeiten, die das Netzwerk benötigt werden) hat, dann wirst Du den Eindruck haben, dass die Götter der Geschwindigkeit Git mit unaussprechlichen Fähigkeiten ausgestattet haben. Weil man die vollständige Historie lokal auf dem Rechner hat, werden die allermeisten Operationen ohne jede Verzögerung ausgeführt und sind sehr schnell.

<!--For example, to browse the history of the project, Git doesn’t need to go out to the server to get the history and display it for you—it simply reads it directly from your local database. This means you see the project history almost instantly. If you want to see the changes introduced between the current version of a file and the file a month ago, Git can look up the file a month ago and do a local difference calculation, instead of having to either ask a remote server to do it or pull an older version of the file from the remote server to do it locally.-->

Um beispielsweise die Historie des Projektes zu durchsuchen, braucht Git sie nicht von einem externen Server zu holen – es liest sie einfach aus der lokalen Datenbank. Das heißt, Du siehst die vollständige Projekthistorie ohne jede Verzögerung. Wenn Du sehen willst, worin sich die aktuelle Version einer Datei von einer Version von vor einem Monat unterscheidet, dann kann Git diese Versionen lokal nachschlagen und ihre Unterschiede lokal bestimmen. Es braucht dazu keinen externen Server – weder um Dateien dort nachzuschlagen, noch um Unterschiede dort bestimmen zu lassen.

<!--This also means that there is very little you can’t do if you’re offline or off VPN. If you get on an airplane or a train and want to do a little work, you can commit happily until you get to a network connection to upload. If you go home and can’t get your VPN client working properly, you can still work. In many other systems, doing so is either impossible or painful. In Perforce, for example, you can’t do much when you aren’t connected to the server; and in Subversion and CVS, you can edit files, but you can’t commit changes to your database (because your database is offline). This may not seem like a huge deal, but you may be surprised what a big difference it can make.-->

Dies bedeutet natürlich außerdem, dass es fast nichts gibt, was Du nicht tun kannst, bloß weil Du gerade offline bist oder keinen Zugriff auf ein VPN hast. Wenn Du im Flugzeug oder Zug ein wenig arbeiten willst, kannst Du problemlos Deine Arbeit committen und Deine Arbeit erst auf den Server pushen (hochladen), wenn Du wieder mit dem Internet verbunden bist. Wenn Du zu Hause bist aber nicht auf das VPN zugreifen kannst, kannst Du dennoch arbeiten. Perforce z.B. erlaubt Dir dagegen nicht sonderlich viel zu tun, solange Du nicht mit dem Server verbunden bist. Und in Subversion und CVS kannst Du Dateien zwar ändern, die Änderungen aber nicht in der Datenbank sichern (weil die Datenbank offline ist). Das mag auf den ersten Blick nicht nach einem großen Problem aussehen, aber Du wirst überrascht sein, was für einen großen Unterschied das ausmachen kann.

<!--### Git Has Integrity ###-->
### Git stellt Integrität sicher ###

<!--Everything in Git is check-summed before it is stored and is then referred to by that checksum. This means it’s impossible to change the contents of any file or directory without Git knowing about it. This functionality is built into Git at the lowest levels and is integral to its philosophy. You can’t lose information in transit or get file corruption without Git being able to detect it.-->

In Git werden Änderungen in Checksummen umgerechnet, bevor sie gespeichert werden. Anschließend werden sie mit dieser Checksumme referenziert. Das macht es unmöglich, dass sich die Inhalte von Dateien oder Verzeichnissen ändern, ohne dass Git das mitbekommt. Git basiert auf dieser Funktionalität und sie ist ein integraler Teil von Gits Philosophie. Man kann Informationen deshalb z.B. nicht während der Übermittlung verlieren oder unwissentlich beschädigte Dateien verwenden, ohne dass Git in der Lage wäre, dies festzustellen.

<!--The mechanism that Git uses for this checksumming is called a SHA-1 hash. This is a 40-character string composed of hexadecimal characters (0–9 and a–f) and calculated based on the contents of a file or directory structure in Git. A SHA-1 hash looks something like this:-->

Der Mechanismus, den Git verwendet, um diese Checksummen zu erstellen, heißt SHA-1 Hash. Eine solche Checksumme ist eine 40 Zeichen lange Zeichenkette, die aus hexadezimalen Zeichen besteht und diese wird von Git aus den Inhalten einer Datei oder Verzeichnisstruktur kalkuliert. Ein SHA-1 Hash sieht wie folgt aus:

	24b9da6552252987aa493b52f8696cd6d3b00373

<!--You will see these hash values all over the place in Git because it uses them so much. In fact, Git stores everything not by file name but in the Git database addressable by the hash value of its contents.-->

Dir werden solche Hash Werte überall in Git begegnen, weil es sie so ausgiebig benutzt. Tatsächlich speichert und referenziert Git Informationen über Dateien in der Datenbank nicht nach ihren Dateinamen sondern nach den Hash Werten ihrer Inhalte.

<!--### Git Generally Only Adds Data ###-->
### Git verwaltet fast ausschließlich Daten ###

<!--When you do actions in Git, nearly all of them only add data to the Git database. It is very difficult to get the system to do anything that is not undoable or to make it erase data in any way. As in any VCS, you can lose or mess up changes you haven’t committed yet; but after you commit a snapshot into Git, it is very difficult to lose, especially if you regularly push your database to another repository.-->

Fast alle Operationen, die Du in der täglichen Arbeit mit Git verwendest, fügen Daten jeweils nur zur internen Git Datenbank hinzu. Deshalb ist es sehr schwer, das System dazu zu bewegen, irgendetwas zu tun, das nicht wieder rückgängig zu machen ist, oder dazu, Daten in irgendeiner Form zu löschen. In jedem anderen VCS ist es leicht, Änderungen, die man noch nicht gespeichert hat, zu verlieren oder unbrauchbar zu machen. In Git dagegen ist es schwierig, einen einmal gespeicherten Snapshot zu verlieren, insbesondere wenn man regelmäßig in ein anderes Repository pusht.

<!--This makes using Git a joy because we know we can experiment without the danger of severely screwing things up. For a more in-depth look at how Git stores its data and how you can recover data that seems lost, see Chapter 9.-->

U.a. deshalb macht es so viel Spaß, mit Git zu arbeiten. Man kann mit Änderungen experimentieren, ohne befürchten zu müssen, irgendetwas zu zerstören oder durcheinander zu bringen. Einen tieferen Einblick in die Art, wie Git Daten speichert und wie man Daten, die scheinbar verloren sind, wieder herstellen kann, wird Kapitel 9 gewähren.

<!--### The Three States ###-->
### Die drei Zustände ###

<!--Now, pay attention. This is the main thing to remember about Git if you want the rest of your learning process to go smoothly. Git has three main states that your files can reside in: committed, modified, and staged. Committed means that the data is safely stored in your local database. Modified means that you have changed the file but have not committed it to your database yet. Staged means that you have marked a modified file in its current version to go into your next commit snapshot.-->

Jetzt aufgepasst. Es folgt die wichtigste Information, die Du Dir merken musst, wenn Du Git kennen lernen willst und Fallstricke vermeiden willst. Git definiert drei Haupt-Zustände, in denen sich eine Datei befinden kann: committed, modified („geändert“) und staged („vorgemerkt“). „Committed“ bedeutet, dass die Daten in der lokalen Datenbank gesichert sind. „Modified“ bedeutet, dass die Datei geändert, diese Änderung aber noch nicht committed wurde. „Staged“ bedeutet, dass Du eine geänderte Datei in ihrem gegenwärtigen Zustand für den nächsten Commit vorgemerkt hast.

<!--This leads us to the three main sections of a Git project: the Git directory, the working directory, and the staging area.-->

Das führt uns zu den drei Hauptbereichen eines Git Projektes: das Git Verzeichnis, das Arbeitsverzeichnis und die Staging Area.

<!--Figure 1-6. Working directory, staging area, and git directory.-->

Insert 18333fig0106.png
Bild 1-6. Arbeitsverzeichnis, Staging Area (xxx) und Git Verzeichnis

<!--The Git directory is where Git stores the metadata and object database for your project. This is the most important part of Git, and it is what is copied when you clone a repository from another computer.-->

Das Git Verzeichnis ist der Ort, an dem Git Metadaten und die lokale Datenbank für Dein Projekt speichert. Dies ist der wichtigste Teil von Git, und dieser Teil wird kopiert, wenn Du ein Repository von einem anderen Rechner klonst.

<!--The working directory is a single checkout of one version of the project. These files are pulled out of the compressed database in the Git directory and placed on disk for you to use or modify.-->

Dein Arbeitsverzeichnis ist ein Checkout („Abbild“ xxx) einer spezifischen Version des Projektes. Diese Dateien werden aus der komprimierten Datenbank geholt und auf der Festplatte in einer Form gespeichert, die Du bearbeiten und modifizieren kannst.

<!--The staging area is a simple file, generally contained in your Git directory, that stores information about what will go into your next commit. It’s sometimes referred to as the index, but it’s becoming standard to refer to it as the staging area.-->

Die Staging Area ist einfach eine Datei (normalerweise im Git Verzeichnis), in der vorgemerkt wird, welche Änderungen Dein nächster Commit umfassen soll. Sie wird manchmal auch als „Index“ bezeichnet, aber der Begriff „Staging Area“ ist der gängigere.

<!--The basic Git workflow goes something like this:-->

Der grundlegend Git Arbeitsprozess sieht in etwa so aus:

<!--1. You modify files in your working directory.-->
<!--2. You stage the files, adding snapshots of them to your staging area.-->
<!--3. You do a commit, which takes the files as they are in the staging area and stores that snapshot permanently to your Git directory.-->

1. Du bearbeitest Dateien in Deinem Arbeitsverzeichnis.
2. Du markierst Dateien für den nächsten Commit, indem Du Snapshots zur Staging Area hinzufügst.
3. Du legst den Commit an, wodurch die in der Staging Area vorgemerkten Snapshots dauerhaft im Git Verzeichnis (d.h. der lokalen Datenbank) gespeichert werden.

<!--If a particular version of a file is in the git directory, it’s considered committed. If it’s modified but has been added to the staging area, it is staged. And if it was changed since it was checked out but has not been staged, it is modified. In Chapter 2, you’ll learn more about these states and how you can either take advantage of them or skip the staged part entirely.-->

Wenn eine bestimmte Version einer Datei im Git Verzeichnis liegt, gilt sie als „committed“. Wenn sie geändert und in der Staging Area vorgemerkt ist, gilt sie als „staged“. Und wenn sie geändert, aber noch nicht zur Staging Area hinzugefügt wurde, gilt sie als „modified“. In Kapitel 2 wirst Du mehr über diese Zustände lernen und darüber, wie Du sie sinnvoll einsetzen und wie Du den Zwischenschritt der Staging Area auch einfach überspringen kannst.

<!--## Installing Git ##-->
## Git installieren ##

<!--Let’s get into using some Git. First things first—you have to install it. You can get it a number of ways; the three major ones are to install it from source or to install an existing package for your platform.-->

Lass uns damit anfangen, Git tatsächlich zu verwenden. Der erste Schritt besteht natürlich darin, Git zu installieren und das kann, wie üblich, auf unterschiedliche Weisen geschehen. Die beiden wichtigsten bestehen darin, entweder den Quellcode herunterzuladen und selbst zu kompilieren oder ein fertiges Paket für Dein Betriebssystem zu installieren.

<!--### Installing from Source ###-->
### Vom Quellcode aus installieren ###

<!--If you can, it’s generally useful to install Git from source, because you’ll get the most recent version. Each version of Git tends to include useful UI enhancements, so getting the latest version is often the best route if you feel comfortable compiling software from source. It is also the case that many Linux distributions contain very old packages; so unless you’re on a very up-to-date distro or are using backports, installing from source may be the best bet.-->

Wenn es Dir möglich ist, empfehlen wir, Git vom Quellcode aus zu installieren, weil Du die jeweils neueste Version erhältst. In der Regel bringt jede Version nützliche Verbesserungen (z.B. am Interface), sodass es sich lohnt die jeweils neueste Version zu verwenden – sofern Du natürlich damit klarkommst, Software aus dem Quellcode zu kompilieren. Viele Linux Distributionen umfassen sehr alte Git Versionen. Wenn Du also keine sehr aktuelle Distribution oder Backports (xxx) verwendest, empfehlen wir, diesen Weg in Erwägung ziehen.

<!--To install Git, you need to have the following libraries that Git depends on: curl, zlib, openssl, expat, and libiconv. For example, if you’re on a system that has yum (such as Fedora) or apt-get (such as a Debian based system), you can use one of these commands to install all of the dependencies:-->

Um Git zu installieren, benötigst Du die folgenden Bibliotheken, die von Git verwendet werden: curl, zlib, openssl, expat und libiconv. Wenn Dir auf Deinem System yum (z.B. auf Fedora) oder apt-get (z.B. auf Debian-basierten Systemen) zur Verfügung steht, kannst Du einen der folgenden Befehle verwenden, um diese Abhängigkeiten zu installieren:

	$ yum install curl-devel expat-devel gettext-devel \
	  openssl-devel zlib-devel

	$ sudo apt-get install curl-devel expat-devel gettext-devel \
	  openssl-devel zlib-devel

<!--When you have all the necessary dependencies, you can go ahead and grab the latest snapshot from the Git web site:-->

Nachdem Du die genannten Bibliotheken installiert hast, besorge Dir die aktuelle Version des Git Quellcodes von der Git Webseite:

	http://git-scm.com/download

<!--Then, compile and install:-->

Danach kannst Du dann Git kompilieren und installieren:

	$ tar -zxf git-1.7.2.2.tar.gz
	$ cd git-1.7.2.2
	$ make prefix=/usr/local all
	$ sudo make prefix=/usr/local install

<!--After this is done, you can also get Git via Git itself for updates:-->

Von nun an kannst Du Git mit Hilfe von Git selbst aktualisieren:

	$ git clone git://git.kernel.org/pub/scm/git/git.git

<!--### Installing on Linux ###-->
### Installation unter Linux ###

<!--If you want to install Git on Linux via a binary installer, you can generally do so through the basic package-management tool that comes with your distribution. If you’re on Fedora, you can use yum:-->

Wenn Du Git unter Linux mit einem Installationsprogramm installieren willst, kannst Du das normalerweise mit dem Paketmanager tun, der von Deinem Betriebssystem verwendet wird. Unter Fedora zum Beispiel kannst Du yum verwenden:

	$ yum install git-core

<!--Or if you’re on a Debian-based distribution like Ubuntu, try apt-get:-->

Auf einem Debian-basierten System wie Ubuntu steht Dir apt-get zur Verfügung:

	$ sudo apt-get install git

<!--### Installing on Mac ###-->
### Installation unter Mac OS X ###

<!--There are two easy ways to install Git on a Mac. The easiest is to use the graphical Git installer, which you can download from the SourceForge page (see Figure 1-7):-->

Auf einem Mac kann man Git auf zwei Arten installieren. Der einfachste ist, das grafische Git Installationsprogramm zu verwenden, den man von der SourceForge Webseite herunterladen kann (siehe Bild 1-7)

	http://sourceforge.net/projects/git-osx-installer/

<!--Figure 1-7. Git OS X installer.-->

Insert 18333fig0107.png
Bild 1-7. Git OS X Installationsprogramm

<!--The other major way is to install Git via MacPorts (`http://www.macports.org`). If you have MacPorts installed, install Git via-->

Die andere Möglichkeit ist, Git via MacPorts (http://www.macports.org) zu installieren. Wenn Du MacPorts auf Deinem System hast, installiert der folgende Befehl Git:

	$ sudo port install git-core +svn +doc +bash_completion +gitweb

<!--You don’t have to add all the extras, but you’ll probably want to include +svn in case you ever have to use Git with Subversion repositories (see Chapter 8).-->

Du brauchst die optionalen Features natürlich nicht mit zu installieren, aber es macht Sinn `+svn` zu verwenden, falls Du jemals Git mit einem Subversion Repository verwenden willst.

<!--Homebrew (`http://brew.sh/`) is another alternative to install Git. If you have Homebrew installed, install Git via

	$ brew install git

-->

Alternativ kann man Git auch über Homebrew (`http://brew.sh/`) installieren. Hast Du Homebrew bereits, kannst Du Git einfach über den folgenden Befehl installieren:

	$ brew install git

<!--### Installing on Windows ###-->
### Installation unter Windows ###

<!--Installing Git on Windows is very easy. The msysGit project has one of the easier installation procedures. Simply download the installer exe file from the GitHub page, and run it:-->

Das msysGit Projekt macht die Installation von Git unter Windows sehr einfach. Lade einfach das Installationsprogramm für Windows von der GitHub Webseite herunter und führe es aus:

	http://msysgit.github.com/

<!--After it’s installed, you have both a command-line version (including an SSH client that will come in handy later) and the standard GUI.-->

Danach hast Du sowohl eine Kommandozeilenversion (inklusive eines SSH Clients, der sich später noch als nützlich erweisen wird) als auch die Standard GUI installiert.

<!--Note on Windows usage: you should use Git with the provided msysGit shell (Unix style), it allows to use the complex lines of command given in this book. If you need, for some reason, to use the native Windows shell / command line console, you have to use double quotes instead of simple quotes (for parameters with spaces in them) and you must quote the parameters ending with the circumflex accent (^) if they are last on the line, as it is a continuation symbol in Windows.-->

Hinweis für Windows Benutzer: Du solltest Git mit der in msysGit enthaltenen Shell (Unix Style) ausführen. Dies erlaubt es Dir auch die komplexen Kommandozeilenbefehle aus diesem Buch auszuführen. Wenn Du aus irgendeinem Grund die native Windows Shell, also die Eingabeaufforderung, verwenden musst, müssen Gänsefüßchen, statt einzelnen Anführungszeichen verwendet werden (für Parameter, die ein Leerzeichen enthalten). Außerdem müssen alle Parameter, die mit einem Zirkumflex (^) enden und am Ende einer Zeile stehen, mit Gänsefüßchen umschlossen werden. Der Zirkumflex am Ende einer Zeile teilt Windows sonst mit, dass diese Zeile noch nicht beendet ist und in der nächsten Zeile fortgesetzt werden soll.

<!--## First-Time Git Setup ##-->
## Git konfigurieren ##

<!--Now that you have Git on your system, you’ll want to do a few things to customize your Git environment. You should have to do these things only once; they’ll stick around between upgrades. You can also change them at any time by running through the commands again.-->

Nachdem Du jetzt Git auf Deinem System installiert hast, solltest Du Deine Git Konfiguration anpassen. Das brauchst Du nur einmal zu tun, die Konfiguration bleibt auch bestehen, wenn Du Git auf eine neuere Version aktualisierst. Du kannst sie jederzeit ändern, indem Du die folgenden Befehle einfach noch einmal ausführst.

<!--Git comes with a tool called git config that lets you get and set configuration variables that control all aspects of how Git looks and operates. These variables can be stored in three different places:-->

Git umfasst das Werkzeug `git config`, das Dir erlaubt, Konfigurationswerte zu verändern. Auf diese Weise kannst Du anpassen, wie Git aussieht und arbeitet. Diese Werte sind an drei verschiedenen Orten gespeichert:

<!--*	`/etc/gitconfig` file: Contains values for every user on the system and all their repositories. If you pass the option` -\-system` to `git config`, it reads and writes from this file specifically.-->
<!--*	`~/.gitconfig` file: Specific to your user. You can make Git read and write to this file specifically by passing the `-\-global` option.-->
<!--*	config file in the git directory (that is, `.git/config`) of whatever repository you’re currently using: Specific to that single repository. Each level overrides values in the previous level, so values in `.git/config` trump those in `/etc/gitconfig`.-->

* Die Datei `/etc/gitconfig` enthält Werte, die für jeden Anwender des Systems und all ihre Projekte gelten. Wenn Du `git config` mit der Option `--system` verwendest, wird diese Datei verwendet.
* Die Werte in der Datei `~/.gitconfig` gelten ausschließlich für Dich und all Deine Projekte. Wenn Du `git config` mit der Option `--global` verwendest, wird diese Datei verwendet.
* Die Datei `.git/config` im Git Verzeichnis eines Projektes enthält Werte, die nur für das jeweilige Projekt gelten. Diese Dateien überschreiben Werte aus den jeweils vorhergehenden Dateien in dieser Reihenfolge. D.h. Werte in beispielsweise `.git/config` überschreiben diejenigen in `/etc/gitconfig`.

<!--On Windows systems, Git looks for the `.gitconfig` file in the `$HOME` directory (`%USERPROFILE%` in Windows’ environment), which is `C:\Documents and Settings\$USER` or `C:\Users\$USER` for most people, depending on version (`$USER` is `%USERNAME%` in Windows’ environment). It also still looks for /etc/gitconfig, although it’s relative to the MSys root, which is wherever you decide to install Git on your Windows system when you run the installer.-->

Auf Windows Systemen sucht Git nach der `.gitconfig` Datei im `$HOME` Verzeichnis (für die meisten Leute ist das das Verzeichnis `C:\Dokumente und Einstellungen\$USER`). Es schaut auch immer nach `/etc/gitconfig`, auch wenn dieses relativ zu dem MSys Wurzelverzeichnis ist, welches das ist, wohin Du Git bei der Installation in Windows installiert hast.

<!--### Your Identity ###-->
### Deine Identität ###

<!--The first thing you should do when you install Git is to set your user name and e-mail address. This is important because every Git commit uses this information, and it’s immutably baked into the commits you pass around:-->

Nachdem Du Git installiert hast, solltest Du als erstes Deinen Namen und Deine E-Mail Adresse konfigurieren. Das ist wichtig, weil Git diese Information für jeden Commit verwendet, den Du anlegst, und sie ist unveränderlich in Deine Commits eingebaut (xxx):

	$ git config --global user.name "John Doe"
	$ git config --global user.email johndoe@example.com

<!--Again, you need to do this only once if you pass the `-\-global` option, because then Git will always use that information for anything you do on that system. If you want to override this with a different name or e-mail address for specific projects, you can run the command without the `-\-global` option when you’re in that project.-->

Du brauchst diese Konfiguration, wie schon erwähnt, nur einmal vorzunehmen, wenn Du die `--global` Option verwendest, weil Git diese Information dann für all Deine Projekte verwenden wird. Wenn Du sie für ein spezielles Projekt mit einem anderen Namen oder einer anderen E-Mail Adresse überschreiben willst, kannst Du dazu den Befehl ohne die `--global` Option innerhalb dieses Projektes ausführen.

<!--### Your Editor ###-->
### Dein Editor ###

<!--Now that your identity is set up, you can configure the default text editor that will be used when Git needs you to type in a message. By default, Git uses your system’s default editor, which is generally Vi or Vim. If you want to use a different text editor, such as Emacs, you can do the following:-->

Nachdem Du Deine Identität jetzt konfiguriert hast, kannst Du einstellen, welchen Texteditor Git in Situationen verwenden soll, in denen Du eine Nachricht eingeben musst. Normalerweise verwendet Git den Standard-Texteditor Deines Systems – das ist üblicherweise Vi oder Vim. Wenn Du einen anderen Texteditor, z.B. Emacs, verwenden willst, kannst Du das wie folgt festlegen:

	$ git config --global core.editor emacs

<!--### Your Diff Tool ###-->
### Dein Diff Programm ###

<!--Another useful option you may want to configure is the default diff tool to use to resolve merge conflicts. Say you want to use vimdiff:-->

Eine andere nützliche Einstellung, die Du möglicherweise vornehmen willst, ist welches Diff Programm Git verwendet. Mit diesem Programm kannst Du Konflikte auflösen, die während der Arbeit mit Git manchmal auftreten. Wenn Du beispielsweise vimdiff verwenden willst, kannst Du das so festlegen:

	$ git config --global merge.tool vimdiff

<!--Git accepts kdiff3, tkdiff, meld, xxdiff, emerge, vimdiff, gvimdiff, ecmerge, and opendiff as valid merge tools. You can also set up a custom tool; see Chapter 7 for more information about doing that.-->

Git kann von Hause aus mit den folgenden Diff Programmen arbeiten: kdiff3, tkdiff, meld, xxdiff, emerge, vimdiff, gvimdiff, ecmerge, and opendiff. Außerdem kannst Du ein eigenes Programm aufsetzen. Wir werden in Kapitel 7 darauf eingehen, wie das geht.

<!--### Checking Your Settings ###-->
### Deine Einstellungen überprüfen ###

<!--If you want to check your settings, you can use the `git config -\-list` command to list all the settings Git can find at that point:-->

Wenn Du Deine Einstellungen überprüfen willst, kannst Du mit dem Befehl `git config --list` alle Einstellungen anzuzeigen, die Git an dieser Stelle (z.B. innerhalb eines bestimmten Projektes) bekannt sind:

	$ git config --list
	user.name=Scott Chacon
	user.email=schacon@gmail.com
	color.status=auto
	color.branch=auto
	color.interactive=auto
	color.diff=auto
	...

<!--You may see keys more than once, because Git reads the same key from different files (`/etc/gitconfig` and `~/.gitconfig`, for example). In this case, Git uses the last value for each unique key it sees.-->

Manche Variablen werden möglicherweise mehrfach aufgelistet, weil Git dieselbe Variable in verschiedenen Dateien (z.B. `/etc/gitconfig` und `~/.gitconfig`) findet. In diesem Fall verwendet Git dann den jeweils zuletzt aufgelisteten Wert.

<!--You can also check what Git thinks a specific key’s value is by typing `git config {key}`:-->

Außerdem kannst Du mit dem Befehl `git config {key}` prüfen, welchen Wert Git für einen bestimmten Variablennamen verwendet:

	$ git config user.name
	Scott Chacon

<!--## Getting Help ##-->
## Hilfe finden ##

<!--If you ever need help while using Git, there are three ways to get the manual page (manpage) help for any of the Git commands:-->

Falls Du jemals Hilfe in der Anwendung von Git benötigst, gibt es drei Möglichkeiten, die entsprechende Seite aus der Dokumentation (manpage) für jeden Git Befehl anzuzeigen:

	$ git help <verb>
	$ git <verb> --help
	$ man git-<verb>

<!--For example, you can get the manpage help for the config command by running-->

Beispielsweise erhältst Du die Hilfeseite für den `git config` Befehl so:

	$ git help config

<!--These commands are nice because you can access them anywhere, even offline.-->
<!--If the manpages and this book aren’t enough and you need in-person help, you can try the `#git` or `#github` channel on the Freenode IRC server (irc.freenode.net). These channels are regularly filled with hundreds of people who are all very knowledgeable about Git and are often willing to help.-->

Die „manpage“ Dokumentation ist nützlich, weil Du sie Dir jederzeit anzeigen lassen kannst, auch wenn Du offline bist. Wenn Dir die manpages und dieses Buch nicht ausreichen, kannst Du Deine Fragen auch in den Chaträumen `#git` oder `#github` auf dem Freenode IRC Server (irc.freenode.net) stellen. Diese Räume sind in der Regel sehr gut besucht. Normalerweise findet sich unter den hunderten von Anwendern, die oft sehr viel Erfahrung mit Git haben, irgendjemand, der Deine Fragen gern beantwortet.

<!--## Summary ##-->
## Zusammenfassung ##

<!--You should have a basic understanding of what Git is and how it’s different from the CVCS you may have been using. You should also now have a working version of Git on your system that’s set up with your personal identity. It’s now time to learn some Git basics.-->

Du solltest jetzt ein grundlegendes Verständnis davon haben, was Git ist und wie es sich von anderen CVCS unterscheidet, die Du möglicherweise schon verwendet hast. Du solltest außerdem eine funktionierende Git Version auf Deinem Rechner installiert und konfiguriert haben. Jetzt wird es Zeit, einige Git Grundlagen zu besprechen.
<!--# Git Basics #-->
# Git Grundlagen #

<!--If you can read only one chapter to get going with Git, this is it. This chapter covers every basic command you need to do the vast majority of the things you’ll eventually spend your time doing with Git. By the end of the chapter, you should be able to configure and initialize a repository, begin and stop tracking files, and stage and commit changes. We’ll also show you how to set up Git to ignore certain files and file patterns, how to undo mistakes quickly and easily, how to browse the history of your project and view changes between commits, and how to push and pull from remote repositories.-->

Wenn Du nur ein einziges Kapitel aus diesem Buch lesen willst, um mit Git loslegen zu können, dann lies dieses hier. Wir werden hier auf die grundlegenden Git Befehle eingehen, die Du für den größten Teil Deiner täglichen Arbeit mit Git brauchst. Am Ende des Kapitels solltest Du in der Lage sein, ein neues Repository anzulegen und zu konfigurieren, Dateien zur Versionskontrolle hinzuzufügen und wieder aus ihr zu entfernen, Änderungen in der Staging Area für einen Commit vorzumerken und schließlich einen Commit durchzuführen. Wir werden außerdem besprechen, wie Du Git so konfigurieren kannst, dass es bestimmte Dateien und Dateimuster ignoriert, wie Du Fehler schnell und einfach rückgängig machen, wie Du die Historie Deines Projektes durchsuchen und Änderungen zwischen bestimmten Commits nachschlagen, und wie Du in externe Repositorys herauf- und von dort herunterladen kannst.

<!--## Getting a Git Repository ##-->
## Ein Git Repository anlegen ##

<!--You can get a Git project using two main approaches. The first takes an existing project or directory and imports it into Git. The second clones an existing Git repository from another server.-->

Es gibt grundsätzlich zwei Möglichkeiten, ein Git Repository auf dem eigenen Rechner anzulegen. Erstens kann man ein existierendes Projekt oder Verzeichnis in ein neues Git Repository importieren. Zweitens kann man ein existierendes Repository von einem anderen Rechner, der als Server fungiert, auf den eigenen Rechner klonen.

<!--### Initializing a Repository in an Existing Directory ###-->
### Ein existierendes Verzeichnis als Git Repository initialisieren ###

<!--If you’re starting to track an existing project in Git, you need to go to the project’s directory and type-->

Wenn Du künftige Änderungen an einem bestehenden Projekt auf Deinem Rechner mit Git versionieren und nachverfolgen willst, kannst Du dazu einfach in das jeweilige Verzeichnis wechseln und diesen Befehl ausführen:

	$ git init

<!--This creates a new subdirectory named `.git` that contains all of your necessary repository files — a Git repository skeleton. At this point, nothing in your project is tracked yet. (See *Chapter 9* for more information about exactly what files are contained in the `.git` directory you just created.)-->

Das erzeugt ein Unterverzeichnis `.git`, in dem alle relevanten Git Repository Daten enthalten sind, also ein Git Repository Grundgerüst. Zu diesem Zeitpunkt werden noch keine Dateien in Git versioniert. (In Kapitel 9 werden wir genauer darauf eingehen, welche Dateien im .git Verzeichnis enthalten sind und was ihre Aufgabe ist.)

<!--If you want to start version-controlling existing files (as opposed to an empty directory), you should probably begin tracking those files and do an initial commit. You can accomplish that with a few `git add` commands that specify the files you want to track, followed by a commit:-->

Wenn in Deinem Projekt bereits Dateien vorhanden sind (und es sich nicht nur um ein leeres Verzeichnis handelt), willst Du diese vermutlich zur Versionskontrolle hinzufügen, damit Änderungen daran künftig nachverfolgbar sind. Dazu kannst Du die folgenden Git Befehle ausführen um die Dateien zur Versionskontrolle hinzuzufügen. Anschließend kannst Du Deinen ersten Commit anlegen:

	$ git add *.c
	$ git add README
	$ git commit -m 'initial project version'

<!--We’ll go over what these commands do in just a minute. At this point, you have a Git repository with tracked files and an initial commit.-->

Wir werden gleich noch einmal genauer auf diese Befehle eingehen. Im Moment ist nur wichtig zu verstehen, dass Du jetzt ein Git Repository erzeugt und einen ersten Commit angelegt hast.

<!--### Cloning an Existing Repository ###-->
### Ein existierendes Repository klonen ###

<!--If you want to get a copy of an existing Git repository — for example, a project you’d like to contribute to — the command you need is `git clone`. If you’re familiar with other VCS systems such as Subversion, you’ll notice that the command is `clone` and not `checkout`. This is an important distinction — Git receives a copy of nearly all data that the server has. Every version of every file for the history of the project is pulled down when you run `git clone`. In fact, if your server disk gets corrupted, you can use any of the clones on any client to set the server back to the state it was in when it was cloned (you may lose some server-side hooks and such, but all the versioned data would be there — see *Chapter 4* for more details).-->

Wenn Du eine Kopie eines existierenden Git Repositorys anlegen willst – z.B. um an einem Projekt mitzuarbeiten – dann kannst Du dazu den Befehl `git clone` verwenden. Wenn Du schon mit anderen VCS Sytemen wie Subversion gearbeitet hast, wird Dir auffallen, dass der Befehl `clone` heißt und nicht `checkout`. Dies ist ein wichtiger Unterschied, den Du verstehen solltest. Git lädt eine Kopie aller Daten, die sich im existierenden Repository befinden, auf Deinen Rechner. Mit `git clone` wird jede einzelne Version jeder einzelnen Datei in der Historie des Repositorys heruntergeladen. Wenn ein Repository auf einem Server einmal beschädigt wird (z.B. weil die Festplatte beschädigt wird), kann man tatsächlich jeden beliebigen Klon des Repositorys verwenden, um das Repository auf dem Server wieder in dem Zustand wieder herzustellen, in dem es sich befand, als es geklont wurde. (Es kann passieren, dass man einige auf dem Server vorhandenen Hooks verliert, aber alle versionierten Daten bleiben erhalten. In Kapitel 4 gehen wir darauf noch einmal genauer ein.)

<!--You clone a repository with `git clone [url]`. For example, if you want to clone the Ruby Git library called Grit, you can do so like this:-->

Du kannst ein Repository mit dem Befehl `git clone [url]` klonen. Um beispielsweise das Repository der Ruby Git Bibliothek Grit zu klonen, führst Du den folgenden Befehl aus:

	$ git clone git://github.com/schacon/grit.git

<!--That creates a directory named `grit`, initializes a `.git` directory inside it, pulls down all the data for that repository, and checks out a working copy of the latest version. If you go into the new `grit` directory, you’ll see the project files in there, ready to be worked on or used. If you want to clone the repository into a directory named something other than grit, you can specify that as the next command-line option:-->

Git legt dann ein Verzeichnis `grit` an, initialisiert ein `.git` Verzeichnis darin, lädt alle Daten des Repositorys herunter, und checkt eine Arbeitskopie der letzten Version aus. Wenn Du in das neue `grit` Verzeichnis wechselst, findest Du dort die in diesem Projekt enthaltenen Dateien und kannst sie benutzen oder bearbeiten. Wenn Du das Repository in ein Verzeichnis mit einem anderen Namen als `grit` klonen willst, kannst Du das wie folgt angeben:

	$ git clone git://github.com/schacon/grit.git mygrit

<!--That command does the same thing as the previous one, but the target directory is called `mygrit`.-->

Dieser Befehl tut das gleiche wie der vorhergehende, aber das Zielverzeichnis ist diesmal `mygrit`.

<!--Git has a number of different transfer protocols you can use. The previous example uses the `git://` protocol, but you may also see `http(s)://` or `user@server:/path.git`, which uses the SSH transfer protocol. *Chapter 4* will introduce all of the available options the server can set up to access your Git repository and the pros and cons of each.-->

Git unterstützt eine Reihe unterschiedlicher Übertragungsprotokolle. Das vorhergehende Beispiel verwendet das `git://` Protokoll, aber Du wirst auch auf `http(s)://` oder `user@server:/path.git` treffen, die das SSH Protokoll verwenden. In Kapitel 4 gehen wir auf die verfügbaren Optionen (und deren Vor- und Nachteile) ein, die ein Server hat, um Zugriff auf ein Git Repository zu ermöglichen.

<!--## Recording Changes to the Repository ##-->
## Änderungen am Repository nachverfolgen ##

<!--You have a bona fide Git repository and a checkout or working copy of the files for that project. You need to make some changes and commit snapshots of those changes into your repository each time the project reaches a state you want to record.-->

Du hast jetzt ein voll funktionsfähiges Git Repository und eine Arbeitskopie des Projekts ist in Deinem Verzeichnis ausgecheckt. Du kannst nun die Dateien im Projekt bearbeiten. Immer wenn Dein Projekt einen Zustand erreicht hat, den Du festhalten willst, musst Du diese Änderungen einchecken.

<!--Remember that each file in your working directory can be in one of two states: *tracked* or *untracked*. *Tracked* files are files that were in the last snapshot; they can be *unmodified*, *modified*, or *staged*. *Untracked* files are everything else — any files in your working directory that were not in your last snapshot and are not in your staging area.  When you first clone a repository, all of your files will be tracked and unmodified because you just checked them out and haven’t edited anything.-->

Jede Datei in Deinem Arbeitsverzeichnis kann sich in einem von zwei Zuständen befinden: Änderungen werden verfolgt (engl. tracked) oder nicht (engl. untracked). Alle Dateien, die sich im letzten Snapshot (Commit) befanden, werden in der Versionskontrolle verfolgt. Sie können entweder unverändert (engl. unmodified), modifiziert (engl. modified) oder für den nächsten Commit vorgemerkt (engl. staged) sein. Alle anderen Dateien in Deinem Arbeitsverzeichnis dagegen sind nicht versioniert: das sind all diejenigen Dateien, die nicht schon im letzten Snapshot enthalten waren und die sich nicht in der Staging Area befinden. Wenn Du ein Repository gerade geklont hast, sind alle Dateien versioniert und unverändert – Du hast sie gerade ausgecheckt aber noch nicht verändert.

<!--As you edit files, Git sees them as modified, because you’ve changed them since your last commit. You *stage* these modified files and then commit all your staged changes, and the cycle repeats. This lifecycle is illustrated in Figure 2-1.-->

Sobald Du versionierte Dateien bearbeitest, wird Git sie als modifiziert erkennen, weil Du sie seit dem letzten Commit geändert hast. Du merkst diese geänderten Dateien für den nächsten Commit vor (d.h. Du fügst sie zur Staging Area hinzu bzw. Du stagest sie), legst aus allen markierten Änderungen einen Commit an und der Vorgang beginnt von vorn. Bild 2-1 stellt diesen Zyklus dar:

<!--Figure 2-1. The lifecycle of the status of your files.-->

Insert 18333fig0201.png
Bild 2-1. Zyklus der Grundzustände Deiner Dateien

<!--### Checking the Status of Your Files ###-->
### Den Zustand Deiner Dateien prüfen ###

<!--The main tool you use to determine which files are in which state is the `git status` command. If you run this command directly after a clone, you should see something like this:-->

Das wichtigste Hilfsmittel, um den Zustand zu überprüfen, in dem sich die Dateien in Deinem Repository gerade befinden, ist der Befehl `git status`. Wenn Du diesen Befehl unmittelbar nach dem Klonen eines Repositorys ausführst, sollte er folgende Ausgabe liefern:

	$ git status
	On branch master
	nothing to commit, working directory clean

<!--This means you have a clean working directory — in other words, no tracked files are modified. Git also doesn’t see any untracked files, or they would be listed here. Finally, the command tells you which branch you’re on. For now, that is always `master`, which is the default; you won’t worry about it here. The next chapter will go over branches and references in detail.-->

Dieser Zustand wird auch als sauberes Arbeitsverzeichnis (engl. clean working directory) bezeichnet. Mit anderen Worten, es gibt keine Dateien, die unter Versionskontrolle stehen und seit dem letzten Commit geändert wurden – andernfalls würden sie hier aufgelistet werden. Außerdem teilt Dir der Befehl mit, in welchem Branch Du Dich gerade befindest. In diesem Beispiel ist dies der Branch `master`. Mach Dir darüber im Moment keine Gedanken, wir werden im nächsten Kapitel auf Branches detailliert eingehen.

<!--Let’s say you add a new file to your project, a simple `README` file. If the file didn’t exist before, and you run `git status`, you see your untracked file like so:-->

Sagen wir Du fügst eine neue `README` Datei zu Deinem Projekt hinzu. Wenn die Datei zuvor nicht existiert hat und Du jetzt `git status` ausführst, zeigt Git die bisher nicht versionierte Datei wie folgt an:

	$ vim README
	$ git status
	On branch master
	Untracked files:
	  (use "git add <file>..." to include in what will be committed)
	
	        README

	nothing added to commit but untracked files present (use "git add" to track)

<!--You can see that your new `README` file is untracked, because it’s under the “Untracked files” heading in your status output. Untracked basically means that Git sees a file you didn’t have in the previous snapshot (commit); Git won’t start including it in your commit snapshots until you explicitly tell it to do so. It does this so you don’t accidentally begin including generated binary files or other files that you did not mean to include. You do want to start including README, so let’s start tracking the file.-->

Alle Dateien, die in der Sektion „Untracked files“ aufgelistet werden, sind Dateien, die bisher noch nicht versioniert sind. Dort wird jetzt auch die Datei `README` angezeigt. Mit anderen Worten, die Datei `README` wird in diesem Bereich gelistet, weil sie im letzen Snapshot (Commit) von Git nicht enthalten ist. Git nimmt eine solche Datei nicht automatisch in die Versionskontrolle auf, sondern man muss Git dazu ausdrücklich auffordern. Ansonsten würden generierte Binärdateien oder andere Dateien, die Du nicht in Deinem Repository haben willst, automatisch hinzugefügt werden. Das möchte man in den meisten Fällen vermeiden. Jetzt wollen wir aber Änderungen an der Datei `README` verfolgen und fügen sie deshalb zur Versionskontrolle hinzu.

<!--### Tracking New Files ###-->
### Neue Dateien zur Versionskontrolle hinzufügen ###

<!--In order to begin tracking a new file, you use the command `git add`. To begin tracking the `README` file, you can run this:-->

Um eine neue Datei zur Versionskontrolle hinzuzufügen, verwendest Du den Befehl `git add`. Für Deine neue `README` Datei kannst Du ihn wie folgt ausführen:

	$ git add README

<!--If you run your status command again, you can see that your `README` file is now tracked and staged:-->

Wenn Du den `git status` Befehl erneut ausführst, siehst Du, dass sich Deine `README` Datei jetzt unter Versionskontrolle befindet und für den nächsten Commit vorgemerkt ist (gestaged ist):

	$ git status
	On branch master
	Changes to be committed:
	  (use "git reset HEAD <file>..." to unstage)
	
	        new file:   README
	

<!--You can tell that it’s staged because it’s under the “Changes to be committed” heading. If you commit at this point, the version of the file at the time you ran `git add` is what will be in the historical snapshot. You may recall that when you ran `git init` earlier, you then ran `git add (files)` — that was to begin tracking files in your directory. The `git add` command takes a path name for either a file or a directory; if it’s a directory, the command adds all the files in that directory recursively.-->

Dass die Datei für den nächsten Commit vorgemerkt ist, siehst Du daran, dass sie in der Sektion „Changes to be committed“ aufgelistet ist. Wenn Du jetzt einen Commit anlegst, wird der Snapshot den Zustand der Datei beinhalten, den sie zum Zeitpunkt des Befehls `git add` hatte. Du erinnerst Dich daran, dass Du, als Du vorhin `git init` ausgeführt hast, anschließend `git add` ausgeführt hast: an dieser Stelle hast Du die Dateien in Deinem Verzeichnis der Versionskontrolle hinzugefügt. Der `git add` Befehl akzeptiert einen Pfadnamen einer Datei oder eines Verzeichnisses. Wenn Du ein Verzeichnis angibst, fügt `git add` alle Dateien in diesem Verzeichnis und allen Unterverzeichnissen rekursiv hinzu.

<!--### Staging Modified Files ###-->
### Geänderte Dateien stagen ###

<!--Let’s change a file that was already tracked. If you change a previously tracked file called `benchmarks.rb` and then run your `status` command again, you get something that looks like this:-->

Wenn Du eine bereits versionierte Datei `benchmarks.rb` änderst und den `git status` Befehl ausführst, erhältst Du folgendes:

	$ git status
	On branch master
	Changes to be committed:
	  (use "git reset HEAD <file>..." to unstage)
	
	        new file:   README

	Changes not staged for commit:
	  (use "git add <file>..." to update what will be committed)
	  (use "git checkout -- <file>..." to discard changes in working directory)
	
	        modified:   benchmarks.rb
	

<!--The `benchmarks.rb` file appears under a section named “Changes not staged for commit” — which means that a file that is tracked has been modified in the working directory but not yet staged. To stage it, you run the `git add` command (it’s a multipurpose command — you use it to begin tracking new files, to stage files, and to do other things like marking merge-conflicted files as resolved). Let’s run `git add` now to stage the `benchmarks.rb` file, and then run `git status` again:-->

Die Datei `benchmarks.rb` erscheint in der Sektion „Changes not staged for commit“ – d.h., dass eine versionierte Datei im Arbeitsverzeichnis verändert worden ist, aber noch nicht für den Commit vorgemerkt wurde. Um sie vorzumerken, führst Du den Befehl `git add` aus. (`git add` wird zu verschiedenen Zwecken eingesetzt. Man verwendet ihn, um neue Dateien zur Versionskontrolle hinzuzufügen, Dateien für einen Commit zu markieren und verschiedene andere Dinge – beispielsweise, einen Konflikt aus einem Merge als aufgelöst zu kennzeichnen.)

	$ git add benchmarks.rb
	$ git status
	On branch master
	Changes to be committed:
	  (use "git reset HEAD <file>..." to unstage)
	
	        new file:   README
	        modified:   benchmarks.rb
	

<!--Both files are staged and will go into your next commit. At this point, suppose you remember one little change that you want to make in `benchmarks.rb` before you commit it. You open it again and make that change, and you’re ready to commit. However, let’s run `git status` one more time:-->

Beide Dateien sind nun für den nächsten Commit vorgemerkt. Nehmen wir an, Du willst jetzt aber noch eine weitere Änderung an der Datei `benchmarks.rb` vornehmen, bevor Du den Commit tatsächlich anlegst. Du öffnest die Datei und änderst sie. Jetzt könntest Du den Commit anlegen. Aber zuvor führen wir noch mal `git status` aus:

	$ vim benchmarks.rb
	$ git status
	On branch master
	Changes to be committed:
	  (use "git reset HEAD <file>..." to unstage)
	
	        new file:   README
	        modified:   benchmarks.rb
	
	Changes not staged for commit:
	  (use "git add <file>..." to update what will be committed)
	  (use "git checkout -- <file>..." to discard changes in working directory)
	
	        modified:   benchmarks.rb
	

<!--What the heck? Now `benchmarks.rb` is listed as both staged and unstaged. How is that possible? It turns out that Git stages a file exactly as it is when you run the `git add` command. If you commit now, the version of `benchmarks.rb` as it was when you last ran the `git add` command is how it will go into the commit, not the version of the file as it looks in your working directory when you run `git commit`. If you modify a file after you run `git add`, you have to run `git add` again to stage the latest version of the file:-->

Huch, was ist das? Jetzt wird `benchmarks.rb` sowohl in der Staging Area als auch als geändert aufgelistet. Die Erklärung dafür ist, dass Git eine Datei in exakt dem Zustand für den Commit vormerkt, in dem sie sich befindet, wenn Du den Befehl `git add` ausführst. Wenn Du den Commit jetzt anlegst, wird die Version der Datei `benchmarks.rb` diejenigen Inhalte haben, die sie hatte, als Du `git add` zuletzt ausgeführt hast – nicht diejenigen, die sie in dem Moment hat, wenn Du den Commit anlegst. Wenn Du stattdessen die gegenwärtige Version im Commit haben willst, kannst Du einfach erneut `git add` ausführen:

	$ git add benchmarks.rb
	$ git status
	On branch master
	Changes to be committed:
	  (use "git reset HEAD <file>..." to unstage)
	
	        new file:   README
	        modified:   benchmarks.rb
	

<!--### Ignoring Files ###-->
### Dateien ignorieren ###

<!--Often, you’ll have a class of files that you don’t want Git to automatically add or even show you as being untracked. These are generally automatically generated files such as log files or files produced by your build system. In such cases, you can create a file listing patterns to match them named `.gitignore`.  Here is an example `.gitignore` file:-->

Du wirst in der Regel eine Reihe von Dateien in Deinem Projektverzeichnis haben, die Du nicht versionieren bzw. im Repository haben willst, wie z.B. automatisch generierte Dateien, wie Logdateien oder Dateien, die Dein Build-System erzeugt. In solchen Fällen kannst Du in der Datei `.gitignore` alle Dateien oder Dateimuster angeben, die Du ignorieren willst.

	$ cat .gitignore
	*.[oa]
	*~

<!--The first line tells Git to ignore any files ending in `.o` or `.a` — *object* and *archive* files that may be the product of building your code. The second line tells Git to ignore all files that end with a tilde (`~`), which is used by many text editors such as Emacs to mark temporary files. You may also include a `log`, `tmp`, or `pid` directory; automatically generated documentation; and so on. Setting up a `.gitignore` file before you get going is generally a good idea so you don’t accidentally commit files that you really don’t want in your Git repository.-->

Die erste Zeile weist Git an, alle Dateien zu ignorieren, die mit einem `.o` oder `.a` enden (also Objekt- und Archiv-Dateien, die von Deinem Build-System erzeugt werden). Die zweite Zeile bewirkt, dass alle Dateien ignoriert werden, die mit einer Tilde (`~`) enden. Viele Texteditoren speichern ihre temporären Dateien auf diese Weise, wie bespielsweise Emacs. Du kannst außerdem Verzeichnisse wie `log`, `tmp` oder `pid` hinzufügen, automatisch erzeugte Dokumentation, und so weiter. Es ist normalerweise empfehlenswert, eine `.gitignore` Datei anzulegen, bevor man mit der eigentlichen Arbeit anfängt, damit man nicht versehentlich Dateien ins Repository hinzufügt, die man dort nicht wirklich haben will.

<!--The rules for the patterns you can put in the `.gitignore` file are as follows:-->

Folgende Regeln gelten in einer `.gitignore` Datei:

<!--*	Blank lines or lines starting with `#` are ignored.-->
<!--*	Standard glob patterns work.-->
<!--*	You can end patterns with a forward slash (`/`) to specify a directory.-->
<!--*	You can negate a pattern by starting it with an exclamation point (`!`).-->

*	Leere Zeilen oder Zeilen, die mit `#` beginnen, werden ignoriert.
*	Standard `glob` Muster funktionieren.
*	Du kannst ein Muster mit einem Schrägstrich (`/`) abschließen, um ein Verzeichnis zu deklarieren.
*	Du kannst ein Muster negieren, indem Du ein Ausrufezeichen (`!`) voranstellst.

<!--Glob patterns are like simplified regular expressions that shells use. An asterisk (`*`) matches zero or more characters; `[abc]` matches any character inside the brackets (in this case `a`, `b`, or `c`); a question mark (`?`) matches a single character; and brackets enclosing characters separated by a hyphen(`[0-9]`) matches any character in the range (in this case 0 through 9) .-->

Glob Muster sind vereinfachte reguläre Ausdrücke, die von der Shell verwendet werden. Ein Stern (`*`) bezeichnet „kein oder mehrere Zeichen“; `[abc]` bezeichnet eines der in den eckigen Klammern angegebenen Zeichen (in diesem Fall also `a`, `b` oder `c`); ein Fragezeichen (`?`) bezeichnet ein beliebiges, einzelnes Zeichen; und eckige Klammern mit Zeichen, die von einem Bindestrich getrennt werden (`[0-9]`) bezeichnen ein Zeichen aus der jeweiligen Menge von Zeichen (in diesem Fall also aus der Menge der Zeichen von 0 bis 9).

<!--Here is another example `.gitignore` file:-->

Hier ist ein weiteres Beispiel für eine `.gitignore` Datei:

<!--	# a comment - this is ignored-->
<!--	# no .a files-->
<!--	*.a-->
<!--	# but do track lib.a, even though you're ignoring .a files above-->
<!--	!lib.a-->
<!--	# only ignore the root TODO file, not subdir/TODO-->
<!--	/TODO-->
<!--	# ignore all files in the build/ directory-->
<!--	build/-->
<!--	# ignore doc/notes.txt, but not doc/server/arch.txt-->
<!--	doc/*.txt-->
<!--	# ignore all .txt files in the doc/ directory-->
<!--	doc/**/*.txt-->

	# ein Kommentar - dieser wird ignoriert
	# ignoriert alle Dateien, die mit .a enden
	*.a
	# nicht aber lib.a Dateien (obwohl obige Zeile *.a ignoriert)
	!lib.a
	# ignoriert eine TODO Datei nur im Wurzelverzeichnis, nicht aber
	/TODO
	# ignoriert alle Dateien im build/ Verzeichnis
	build/
	# ignoriert doc/notes.txt, aber nicht doc/server/arch.txt
	doc/*.txt
	# ignoriert alle .txt Dateien unterhalb des doc/ Verzeichnis
	doc/**/*.txt

<!--A `**/` pattern is available in Git since version 1.8.2.-->

Die Kombination `**/` wurde in der Git Version 1.8.2 eingeführt.

<!--### Viewing Your Staged and Unstaged Changes ###-->
### Die Änderungen in der Staging Area durchsehen ###

<!--If the `git status` command is too vague for you — you want to know exactly what you changed, not just which files were changed — you can use the `git diff` command. We’ll cover `git diff` in more detail later; but you’ll probably use it most often to answer these two questions: What have you changed but not yet staged? And what have you staged that you are about to commit? Although `git status` answers those questions very generally, `git diff` shows you the exact lines added and removed — the patch, as it were.-->

Wenn Dir die Ausgabe des Befehl `git status` nicht aussagekräftig genug ist, weil Du exakt wissen willst, was sich geändert hat – und nicht lediglich, welche Dateien geändert wurden – kannst Du den `git diff` Befehl verwenden. Wir werden `git diff` später noch einmal im Detail besprechen, aber Du wirst diesen Befehl in der Regel verwenden wollen, um eine der folgenden, zwei Fragen zu beantworten: Was hast Du geändert, aber noch nicht für einen Commit vorgemerkt? Und welche Änderungen hast Du für einen Commit bereits vorgemerkt? Während `git status` diese Fragen nur mit Dateinamen beantwortet, zeigt Dir `git diff` exakt an, welche Zeilen hinzugefügt, geändert und entfernt wurden. Dies entspricht gewissermaßen einem Patch.

<!--Let’s say you edit and stage the `README` file again and then edit the `benchmarks.rb` file without staging it. If you run your `status` command, you once again see something like this:-->

Nehmen wir an, Du hast die Datei `README` geändert und für einen Commit in der Staging Area vorgemerkt. Dann änderst Du außerdem die Datei `benchmarks.rb`, fügst sie aber noch nicht zur Staging Area hinzu. Wenn Du den `git status` Befehl dann ausführst, zeigt er Dir in etwa Folgendes an:

	$ git status
	On branch master
	Changes to be committed:
	  (use "git reset HEAD <file>..." to unstage)
	
	        new file:   README
	
	Changes not staged for commit:
	  (use "git add <file>..." to update what will be committed)
	  (use "git checkout -- <file>..." to discard changes in working directory)
	
	        modified:   benchmarks.rb
	

<!--To see what you’ve changed but not yet staged, type `git diff` with no other arguments:-->

Um festzustellen, welche Änderungen Du bisher nicht gestaged hast, führe `git diff` ohne irgendwelche weiteren Argumente aus:

	$ git diff
	diff --git a/benchmarks.rb b/benchmarks.rb
	index 3cb747f..da65585 100644
	--- a/benchmarks.rb
	+++ b/benchmarks.rb
	@@ -36,6 +36,10 @@ def main
	           @commit.parents[0].parents[0].parents[0]
	         end

	+        run_code(x, 'commits 1') do
	+          git.commits.size
	+        end
	+
	         run_code(x, 'commits 2') do
	           log = git.commits('master', 15)
	           log.size

<!--That command compares what is in your working directory with what is in your staging area. The result tells you the changes you’ve made that you haven’t yet staged.-->

Dieser Befehl vergleicht die Inhalte Deines Arbeitsverzeichnisses mit den Inhalten Deiner Staging Area. Das Ergebnis zeigt Dir die Änderungen, die Du an Dateien im Arbeitsverzeichnis vorgenommen, aber noch nicht für den nächsten Commit vorgemerkt hast.

<!--If you want to see what you’ve staged that will go into your next commit, you can use `git diff -\-cached`. (In Git versions 1.6.1 and later, you can also use `git diff -\-staged`, which may be easier to remember.) This command compares your staged changes to your last commit:-->

Wenn Du sehen willst, welche Änderungen in der Staging Area und somit für den nächsten Commit vorgesehen sind, kannst Du `git diff --cached` verwenden. (Ab der Version Git 1.6.1 kannst Du außerdem `git diff --staged` verwenden, was vielleicht leichter zu merken ist.) Dieser Befehl vergleicht die Inhalte der Staging Area mit dem letzten Commit:

	$ git diff --cached
	diff --git a/README b/README
	new file mode 100644
	index 0000000..03902a1
	--- /dev/null
	+++ b/README2
	@@ -0,0 +1,5 @@
	+grit
	+ by Tom Preston-Werner, Chris Wanstrath
	+ http://github.com/mojombo/grit
	+
	+Grit is a Ruby library for extracting information from a Git repository

<!--It’s important to note that `git diff` by itself doesn’t show all changes made since your last commit — only changes that are still unstaged. This can be confusing, because if you’ve staged all of your changes, `git diff` will give you no output.-->

Es ist wichtig, im Kopf zu behalten, dass `git diff` nicht alle Änderungen seit dem letzten Commit anzeigt – er zeigt lediglich diejenigen Änderungen an, die noch nicht in der Staging Area sind. Das kann verwirrend sein. Wenn Du all Deine Änderungen bereits für einen Commit vorgemerkt hast, zeigt `git diff` überhaupt nichts an.

<!--For another example, if you stage the `benchmarks.rb` file and then edit it, you can use `git diff` to see the changes in the file that are staged and the changes that are unstaged:-->

Ein anderes Beispiel: Wenn Du Änderungen an der Datei `benchmarks.rb` bereits zur Staging Area hinzugefügt hast und sie dann anschließend noch mal änderst, kannst Du `git diff` verwenden, um diese letzten Änderungen anzuzeigen, die noch nicht in der Staging Area sind:

	$ git add benchmarks.rb
	$ echo '# test line' >> benchmarks.rb
	$ git status
	On branch master
	Changes to be committed:
	  (use "git reset HEAD <file>..." to unstage)
	
	        modified:   benchmarks.rb
	
	Changes not staged for commit:
	  (use "git add <file>..." to update what will be committed)
	  (use "git checkout -- <file>..." to discard changes in working directory)
	
	        modified:   benchmarks.rb
	

<!--Now you can use `git diff` to see what is still unstaged-->

Jetzt kannst Du `git diff` verwenden, um zu sehen, was noch nicht für den nächsten Commit vorgemerkt ist:

	$ git diff
	diff --git a/benchmarks.rb b/benchmarks.rb
	index e445e28..86b2f7c 100644
	--- a/benchmarks.rb
	+++ b/benchmarks.rb
	@@ -127,3 +127,4 @@ end
	 main()

	 ##pp Grit::GitRuby.cache_client.stats
	+# test line

<!--and `git diff -\-cached` to see what you’ve staged so far:-->

und `git diff --cached`, um zu sehen, was für den nächsten Commit vorgesehen ist:

	$ git diff --cached
	diff --git a/benchmarks.rb b/benchmarks.rb
	index 3cb747f..e445e28 100644
	--- a/benchmarks.rb
	+++ b/benchmarks.rb
	@@ -36,6 +36,10 @@ def main
	          @commit.parents[0].parents[0].parents[0]
	        end

	+        run_code(x, 'commits 1') do
	+          git.commits.size
	+        end
	+
	        run_code(x, 'commits 2') do
	          log = git.commits('master', 15)
	          log.size

<!--### Committing Your Changes ###-->
### Einen Commit erzeugen ###

<!--Now that your staging area is set up the way you want it, you can commit your changes. Remember that anything that is still unstaged — any files you have created or modified that you haven’t run `git add` on since you edited them — won’t go into this commit. They will stay as modified files on your disk.-->
<!--In this case, the last time you ran `git status`, you saw that everything was staged, so you’re ready to commit your changes. The simplest way to commit is to type `git commit`:-->

Nachdem Du jetzt alle Änderungen, die Du im nächsten Commit haben willst, in Deiner Staging Area gesammelt hast, kannst Du den Commit anlegen. Denke daran, dass Änderungen, die nicht in der Staging Area sind (also alle Änderungen, die Du vorgenommen hast, seit Du zuletzt `git add` ausgeführt hast), auch nicht in den Commit aufgenommen werden. Sie werden ganz einfach weiterhin als geänderte Dateien im Arbeitsverzeichnis verbleiben. In unserem Beispiel haben wir gesehen, dass alle Änderungen vorgemerkt waren, als wir zuletzt `git status` ausgeführt haben, also können wir den Commit jetzt anlegen. Das geht am einfachsten mit dem Befehl:

	$ git commit

<!--Doing so launches your editor of choice. (This is set by your shell’s `$EDITOR` environment variable — usually vim or emacs, although you can configure it with whatever you want using the `git config -\-global core.editor` command as you saw in *Chapter 1*).-->

Wenn Du diesen Befehl ausführst, wird Git den Texteditor Deiner Wahl starten. (D.h. denjenigen Texteditor, der durch die `$EDITOR` Variable Deiner Shell angegeben wird – normalerweise ist das vim oder emacs, aber Du kannst jeden Editor Deiner Wahl angeben. Wie in Kapitel 1 besprochen, kannst Du dazu `git config --global core.editor` verwenden.)

<!--The editor displays the following text (this example is a Vim screen):-->

Der Editor zeigt in etwa folgenden Text an (dies ist ein Beispiel mit vim):

	# Please enter the commit message for your changes. Lines starting
	# with '#' will be ignored, and an empty message aborts the commit.
	# On branch master
	# Changes to be committed:
	#       new file:   README
	#       modified:   benchmarks.rb
	#
	~
	~
	~
	".git/COMMIT_EDITMSG" 10L, 283C

<!--You can see that the default commit message contains the latest output of the `git status` command commented out and one empty line on top. You can remove these comments and type your commit message, or you can leave them there to help you remember what you’re committing. (For an even more explicit reminder of what you’ve modified, you can pass the `-v` option to `git commit`. Doing so also puts the diff of your change in the editor so you can see exactly what you did.) When you exit the editor, Git creates your commit with that commit message (with the comments and diff stripped out).-->

Du siehst, dass die vorausgefüllte Commit Meldung die Ausgabe des letzten `git status` Befehls als einen Kommentar und darüber eine leere Zeile enthält. Du kannst die Kommentare entfernen und Deine eigene Meldung einfügen. Oder Du kannst sie stehen lassen, damit Du siehst, was im Commit enthalten sein wird. (Um die Änderungen noch detaillierter sehen zu können, kannst Du den Befehl `git commit` mit der Option `-v` verwenden. Das fügt zusätzlich das Diff Deiner Änderungen im Editor ein, sodass Du exakt sehen kannst, was sich im Commit befindet.) Wenn Du den Texteditor beendest, erzeugt Git den Commit mit der gegebenen Meldung (d.h., ohne den Kommentar und das Diff).

<!--Alternatively, you can type your commit message inline with the `commit` command by specifying it after a `-m` flag, like this:-->

Alternativ kannst Du die Commit Meldung direkt mit dem Befehl `git commit` angeben, indem Du die Option `-m` wie folgt verwendest:

	$ git commit -m "Story 182: Fix benchmarks for speed"
	[master 463dc4f] Fix benchmarks for speed
	 2 files changed, 3 insertions(+)
	 create mode 100644 README

<!--Now you’ve created your first commit! You can see that the commit has given you some output about itself: which branch you committed to (`master`), what SHA-1 checksum the commit has (`463dc4f`), how many files were changed, and statistics about lines added and removed in the commit.-->

Du hast jetzt Deinen ersten Commit angelegt! Git zeigt Dir als Rückmeldung einige Details über den neu angelegten Commit an: in welchem Branch er sich befindet (master), welche SHA-1 Checksumme er hat (`463dc4f`, in diesem Fall nur die Kurzform), wie viele Dateien geändert wurden und eine Zusammenfassung über die insgesamt neu hinzugefügten und entfernten Zeilen in diesem Commit.

<!--Remember that the commit records the snapshot you set up in your staging area. Anything you didn’t stage is still sitting there modified; you can do another commit to add it to your history. Every time you perform a commit, you’re recording a snapshot of your project that you can revert to or compare to later.-->

Denke daran, dass jeder neue Commit denjenigen Snapshot aufzeichnet, den Du in der Staging Area vorbereitet hast. Änderungen, die nicht in der Staging Area waren, werden weiterhin als modifizierte Dateien im Arbeitsverzeichnis vorliegen. Jedes Mal wenn Du einen Commit anlegst, zeichnest Du einen Snapshot Deines Projektes auf, zu dem Du zurückkehren oder mit dem Du spätere Änderungen vergleichen kannst.

<!--### Skipping the Staging Area ###-->
### Die Staging Area überspringen ###

<!--Although it can be amazingly useful for crafting commits exactly how you want them, the staging area is sometimes a bit more complex than you need in your workflow. If you want to skip the staging area, Git provides a simple shortcut. Providing the `-a` option to the `git commit` command makes Git automatically stage every file that is already tracked before doing the commit, letting you skip the `git add` part:-->

Obwohl die Staging Area unglaublich nützlich ist, um genau diejenigen Commits anzulegen, die Du in Deiner Projekt Historie haben willst, ist sie manchmal auch ein bisschen umständlich. Git stellt Dir deshalb eine Alternative zur Verfügung, mit der Du die Staging Area überspringen kannst. Wenn Du den Befehl `git commit` mit der Option `-a` ausführst, übernimmt Git automatisch alle Änderungen an dejenigen Dateien, die sich bereits unter Versionskontrolle befinden, in den Commit – sodass Du auf diese Weise den Schritt `git add` weglassen kannst:

	$ git status
	On branch master
	Changes not staged for commit:
	  (use "git add <file>..." to update what will be committed)
	  (use "git checkout -- <file>..." to discard changes in working directory)
	
	        modified:   benchmarks.rb
	
	no changes added to commit (use "git add" and/or "git commit -a")
	$ git commit -a -m 'added new benchmarks'
	[master 83e38c7] added new benchmarks
	 1 files changed, 5 insertions(+)

<!--Notice how you don’t have to run `git add` on the `benchmarks.rb` file in this case before you commit.-->

Beachte, dass Du in diesem Fall `git add` zuvor noch nicht ausgeführt hast, die Änderungen an `benchmarks.rb` aber dennoch in den Commit übernommen werden.

<!--### Removing Files ###-->
### Dateien entfernen ###

<!--To remove a file from Git, you have to remove it from your tracked files (more accurately, remove it from your staging area) and then commit. The `git rm` command does that and also removes the file from your working directory so you don’t see it as an untracked file next time around.-->

Um eine Datei aus der Git Versionskontrolle zu entfernen, muss diese von den verfolgten Dateien (genauer, aus der Staging Area) entfernt werden und dann mit einem Commit bestätigt werden. Der Befehl `git rm` tut genau das – und löscht die Datei außerdem aus dem Arbeitsverzeichnis, sodass sie dort nicht unbeabsichtigt (als eine nun unversionierte Datei) liegen bleibt.

<!--If you simply remove the file from your working directory, it shows up under the “Changes not staged for commit” (that is, _unstaged_) area of your `git status` output:-->

Wenn Du einfach nur eine Datei aus dem Arbeitsverzeichnis löschst, wird sie in der Sektion „Changes not staged for commit“ angezeigt, wenn Du `git status` ausführst:

	$ rm grit.gemspec
	$ git status
	On branch master
	Changes not staged for commit:
	  (use "git add/rm <file>..." to update what will be committed)
	  (use "git checkout -- <file>..." to discard changes in working directory)
	
	        deleted:    grit.gemspec
	
	no changes added to commit (use "git add" and/or "git commit -a")

<!--Then, if you run `git rm`, it stages the file’s removal:-->

Wenn Du jetzt `git rm` ausführst, wird diese Änderung für den nächsten Commit in der Staging Area vorgemerkt:

	$ git rm grit.gemspec
	rm 'grit.gemspec'
	$ git status
	On branch master
	Changes to be committed:
	  (use "git reset HEAD <file>..." to unstage)
	
	        deleted:    grit.gemspec
	

<!--The next time you commit, the file will be gone and no longer tracked. If you modified the file and added it to the index already, you must force the removal with the `-f` option. This is a safety feature to prevent accidental removal of data that hasn’t yet been recorded in a snapshot and that can’t be recovered from Git.-->

Nach dem nächsten Anlegen eines Commits, wird die Datei nicht mehr im Arbeitsverzeichnis liegen und sich nicht länger unter Versionskontrolle befinden. Wenn Du die Datei zuvor geändert und diese Änderung bereits zur Staging Area hinzugefügt hattest, musst Du die Option `-f` verwenden, um zu erzwingen, dass sie gelöscht wird. Dies ist eine Sicherheitsmaßnahme, um zu vermeiden, dass Du versehentlich Daten löschst, die sich bisher noch nicht als Commit Snapshot in der Historie Deines Projektes befinden – und deshalb auch nicht wiederhergestellt werden können.

<!--Another useful thing you may want to do is to keep the file in your working tree but remove it from your staging area. In other words, you may want to keep the file on your hard drive but not have Git track it anymore. This is particularly useful if you forgot to add something to your `.gitignore` file and accidentally staged it, like a large log file or a bunch of `.a` compiled files. To do this, use the `-\-cached` option:-->

Ein anderer Anwendungsfall für `git rm` ist, dass Du eine Datei in Deinem Arbeitsverzeichnis behalten, aber aus der Staging Area nehmen willst. In anderen Worten, Du willst die Datei nicht löschen, sondern aus der Versionskontrolle nehmen. Das könnte zum Beispiel der Fall sein, wenn Du vergessen hattest, eine Datei in `.gitignore` anzugeben und sie versehentlich zur Versionskontrolle hinzugefügt hast, beispielsweise eine große Logdatei oder eine Reihe kompilierter `.a` Dateien. Hierzu kannst Du dann die `--cached` Option verwenden:

	$ git rm --cached readme.txt

<!--You can pass files, directories, and file-glob patterns to the `git rm` command. That means you can do things such as-->

Der `git rm` Befehl akzeptiert Dateien, Verzeichnisse und `glob` Dateimuster. D.h., Du kannst z.B. folgendes tun:

	$ git rm log/\*.log

<!--Note the backslash (`\`) in front of the `*`. This is necessary because Git does its own filename expansion in addition to your shell’s filename expansion. On Windows with the system console, the backslash must be omitted. This command removes all files that have the `.log` extension in the `log/` directory. Or, you can do something like this:-->

Beachte den Backslash (`\`) vor dem Stern (`*`). Er ist nötig, weil Git Dateinamen zusätzlich zur Dateinamen-Expansion Deiner Shell selbst vervollständigt. D.h., dieser Befehl entfernt alle Dateien, die die Erweiterung `.log` haben und sich im `/log` Verzeichnis befinden. Ein anderes Beispiel ist:

	$ git rm \*~

<!--This command removes all files that end with `~`.-->

Dieser Befehl entfernt alle Dateien, die mit einer Tilde (`~`) aufhören.

<!--### Moving Files ###-->
### Dateien verschieben ###

<!--Unlike many other VCS systems, Git doesn’t explicitly track file movement. If you rename a file in Git, no metadata is stored in Git that tells it you renamed the file. However, Git is pretty smart about figuring that out after the fact — we’ll deal with detecting file movement a bit later.-->

Anders als andere VCS Systeme verfolgt Git nicht explizit, ob Dateien verschoben werden. Wenn Du eine Datei umbenennst, werden darüber keine Metadaten in der Historie gespeichert. Stattdessen ist Git schlau genug, solche Dinge im Nachhinein zu erkennen. Wir werden uns damit später noch befassen.

<!--Thus it’s a bit confusing that Git has a `mv` command. If you want to rename a file in Git, you can run something like-->

Es ist allerdings ein bisschen verwirrend, dass Git trotzdem einen `git mv` Befehl kennt. Wenn Du eine Datei umbenennen willst, kannst Du folgendes tun:

	$ git mv file_from file_to

<!--and it works fine. In fact, if you run something like this and look at the status, you’ll see that Git considers it a renamed file:-->

Das funktioniert einwandfrei. Wenn Du diesen Befehl ausführst und danach den `git status` ausführst, zeigt Git an, dass die Datei umbenannt wurde:

	$ git mv README.txt README
	$ git status
	On branch master
	Changes to be committed:
	  (use "git reset HEAD <file>..." to unstage)
	
	        renamed:    README.txt -> README
	

<!--However, this is equivalent to running something like this:-->
Allerdings kannst Du genauso folgendes tun:

	$ mv README.txt README
	$ git rm README.txt
	$ git add README

<!--Git figures out that it’s a rename implicitly, so it doesn’t matter if you rename a file that way or with the `mv` command. The only real difference is that `mv` is one command instead of three — it’s a convenience function. More important, you can use any tool you like to rename a file, and address the add/rm later, before you commit.-->

Git ist clever genug, selbst herauszufinden, dass Du die Datei umbenannt hast. Du brauchst dies also nicht explizit mit dem `git mv` Befehl zu tun. Der einzige Unterschied ist, dass Du mit `git mv` nur einen Befehl, nicht drei, ausführen musst – das ist natürlich etwas bequemer. Darüberhinaus kannst Du aber Dateien auf jede beliebige Art und Weise extern umbenennen und dann später `git add` bzw. `git rm` verwenden, wenn Du einen Commit zusammenstellst.

<!--## Viewing the Commit History ##-->
## Die Commit Historie anzeigen ##

<!--After you have created several commits, or if you have cloned a repository with an existing commit history, you’ll probably want to look back to see what has happened. The most basic and powerful tool to do this is the `git log` command.-->

Nachdem Du einige Commits angelegt oder ein bestehendes Repository geklont hast, willst Du vielleicht wissen, welche Änderungen zuletzt vorgenommen wurden. Der grundlegende und mächtige Befehl, mit dem Du das tun kannst, ist `git log`.

<!--These examples use a very simple project called `simplegit` that I often use for demonstrations. To get the project, run-->

Die folgende Beispiele beziehen sich auf ein sehr simples Repository mit dem Namen „simplegit“, das ich oft für Demonstationszwecke verwende:

	git clone git://github.com/schacon/simplegit-progit.git

<!--When you run `git log` in this project, you should get output that looks something like this:-->

Wenn Du in diesem Projekt `git log` ausführst, solltest Du eine Ausgabe wie die folgende sehen:

	$ git log
	commit ca82a6dff817ec66f44342007202690a93763949
	Author: Scott Chacon <schacon@gee-mail.com>
	Date:   Mon Mar 17 21:52:11 2008 -0700

	    changed the version number

	commit 085bb3bcb608e1e8451d4b2432f8ecbe6306e7e7
	Author: Scott Chacon <schacon@gee-mail.com>
	Date:   Sat Mar 15 16:40:33 2008 -0700

	    removed unnecessary test code

	commit a11bef06a3f659402fe7563abf99ad00de2209e6
	Author: Scott Chacon <schacon@gee-mail.com>
	Date:   Sat Mar 15 10:31:28 2008 -0700

	    first commit

<!--By default, with no arguments, `git log` lists the commits made in that repository in reverse chronological order. That is, the most recent commits show up first. As you can see, this command lists each commit with its SHA-1 checksum, the author’s name and e-mail, the date written, and the commit message.-->

Der Befehl `git log` listet die Historie der Commits eines Projekts in umgekehrter chronologischer Reihenfolge auf, wenn man ihn ohne weitere Argumente ausführt, d.h. die letzten Commits stehen oben. Wie Du sehen kannst wird jeder Commit mit seiner SHA-1 Checksumme, Namen und E-Mail Adresse des Autors, dem Datum und der Commit Meldung aufgelistet.

<!--A huge number and variety of options to the `git log` command are available to show you exactly what you’re looking for. Here, we’ll show you some of the most-used options.-->

Für den Befehl `git log` gibt es eine riesige Anzahl von Optionen, mit denen man sehr genau eingrenzen kann, wonach man in einer Historie sucht. Schauen wir uns also einige der am häufigsten verwendeten Optionen an.

<!--One of the more helpful options is `-p`, which shows the diff introduced in each commit. You can also use `-2`, which limits the output to only the last two entries:-->

Eine sehr nützliche Option ist `-p`. Sie zeigt die Änderungen an, die in einem Commit enthalten sind. Du kannst außerdem -2 angeben, wodurch nur die letzten beiden Einträge angezeigt werden:

	$ git log -p -2
	commit ca82a6dff817ec66f44342007202690a93763949
	Author: Scott Chacon <schacon@gee-mail.com>
	Date:   Mon Mar 17 21:52:11 2008 -0700

	    changed the version number

	diff --git a/Rakefile b/Rakefile
	index a874b73..8f94139 100644
	--- a/Rakefile
	+++ b/Rakefile
	@@ -5,5 +5,5 @@ require 'rake/gempackagetask'
	 spec = Gem::Specification.new do |s|
	     s.name      =   "simplegit"
	-    s.version   =   "0.1.0"
	+    s.version   =   "0.1.1"
	     s.author    =   "Scott Chacon"
	     s.email     =   "schacon@gee-mail.com

	commit 085bb3bcb608e1e8451d4b2432f8ecbe6306e7e7
	Author: Scott Chacon <schacon@gee-mail.com>
	Date:   Sat Mar 15 16:40:33 2008 -0700

	    removed unnecessary test code

	diff --git a/lib/simplegit.rb b/lib/simplegit.rb
	index a0a60ae..47c6340 100644
	--- a/lib/simplegit.rb
	+++ b/lib/simplegit.rb
	@@ -18,8 +18,3 @@ class SimpleGit
	     end

	 end
	-
	-if $0 == __FILE__
	-  git = SimpleGit.new
	-  puts git.show
	-end
	\ No newline at end of file

<!--This option displays the same information but with a diff directly following each entry. This is very helpful for code review or to quickly browse what happened during a series of commits that a collaborator has added.-->

Diese Option zeigt also im Prinzip die gleiche Information wie zuvor, aber zusätzlich zu jedem Eintrag ein Diff. Das ist nützlich, um einen Code Review zu machen oder eben mal eine Reihe von Commits durchzuschauen, die ein Mitarbeiter angelegt hat.

<!--Sometimes it's easier to review changes on the word level rather than on the line level. There is a `-\-word-diff` option available in Git, that you can append to the `git log -p` command to get word diff instead of normal line by line diff. Word diff format is quite useless when applied to source code, but it comes in handy when applied to large text files, like books or your dissertation. Here is an example:-->

Manchmal ist es einfacher Änderungen an Hand der Wörter anstatt zeilenbasiert zu überprüfen. Git bietet dafür die Option `--word-diff`, welche man an den Befehl `git log -p` anhängen kann. Man weist Git damit an, einen Vergleich auf Basis der Wörter anstatt Zeile für Zeile durchzuführen. Dieser Vergleich ist ziemlich nutzlos wenn man Änderungen innerhalb von Quellcode vergleicht. Beim Vergleich von langen Textdateien zeigt er aber seine Stärke. Er bietet sich zum Beispiel für Bücher oder wissenschaftliche Texte an. Hierzu ein Beispiel:

	$ git log -U1 --word-diff
	commit ca82a6dff817ec66f44342007202690a93763949
	Author: Scott Chacon <schacon@gee-mail.com>
	Date:   Mon Mar 17 21:52:11 2008 -0700

	    changed the version number

	diff --git a/Rakefile b/Rakefile
	index a874b73..8f94139 100644
	--- a/Rakefile
	+++ b/Rakefile
	@@ -7,3 +7,3 @@ spec = Gem::Specification.new do |s|
	    s.name      =   "simplegit"
	    s.version   =   [-"0.1.0"-]{+"0.1.1"+}
	    s.author    =   "Scott Chacon"

<!--As you can see, there is no added and removed lines in this output as in a normal diff. Changes are shown inline instead. You can see the added word enclosed in `{+ +}` and removed one enclosed in `[- -]`. You may also want to reduce the usual three lines context in diff output to only one line, as the context is now words, not lines. You can do this with `-U1` as we did in the example above.-->

Wie man in der Ausgabe sehen kann, zeigt dieser Vergleich nicht an, welche Zeilen hinzugekommen und welche entfallen sind. Stattdessen werden Änderungen innerhalb der Zeilen dargestellt. Mit der Sequenz `{+ +}` wird ein neu hinzugekommes Wort gekennzeichnet, mit `[- -]` ein Wort, welches entfernt wurde. Normalerweise zeigt Git bei einem Vergleich drei zusätzliche Zeilen ober- und unterhalb der eigentlichen Änderung an. Bei einem Textvergleich reicht meist eine zusätzliche Zeile. Man kann dies mit der Option `-U1` erreichen, so wie in dem oben gezeigten Beispiel.

<!--You can also use a series of summarizing options with `git log`. For example, if you want to see some abbreviated stats for each commit, you can use the `-\-stat` option:-->

Außerdem gibt es verschiedene Optionen, die nützlich sind, um Dinge zusammenzufassen. Beispielsweise kannst Du eine kurze Statistik über jeden Commit mit der Option `--stat` anzeigen lassen:

	$ git log --stat
	commit ca82a6dff817ec66f44342007202690a93763949
	Author: Scott Chacon <schacon@gee-mail.com>
	Date:   Mon Mar 17 21:52:11 2008 -0700

	    changed the version number

	 Rakefile |    2 +-
	 1 file changed, 1 insertion(+), 1 deletion(-)

	commit 085bb3bcb608e1e8451d4b2432f8ecbe6306e7e7
	Author: Scott Chacon <schacon@gee-mail.com>
	Date:   Sat Mar 15 16:40:33 2008 -0700

	    removed unnecessary test code

	 lib/simplegit.rb |    5 -----
	 1 file changed, 5 deletions(-)

	commit a11bef06a3f659402fe7563abf99ad00de2209e6
	Author: Scott Chacon <schacon@gee-mail.com>
	Date:   Sat Mar 15 10:31:28 2008 -0700

	    first commit

	 README           |    6 ++++++
	 Rakefile         |   23 +++++++++++++++++++++++
	 lib/simplegit.rb |   25 +++++++++++++++++++++++++
	 3 files changed, 54 insertions(+)

<!--As you can see, the `-\-stat` option prints below each commit entry a list of modified files, how many files were changed, and how many lines in those files were added and removed. It also puts a summary of the information at the end.-->
<!--Another really useful option is `-\-pretty`. This option changes the log output to formats other than the default. A few prebuilt options are available for you to use. The `oneline` option prints each commit on a single line, which is useful if you’re looking at a lot of commits. In addition, the `short`, `full`, and `fuller` options show the output in roughly the same format but with less or more information, respectively:-->

Die `--stat` Option zeigt unterhalb jedes Commits eine kurze Statistik über die jeweiligen Änderungen an: welche Dateien geändert wurden und wieviele Zeilen insgesamt hinzugefügt oder entfernt wurden. Eine weitere nützliche Option ist `--pretty`. Diese Option ändert das Format der Ausgabe und es gibt eine Anzahl mitgelieferter Formate. Das `oneline` Format listet jeden Commit in einer einzigen Zeile, was nützlich ist, wenn Du eine große Anzahl von Commits durchsuchen willst. Die `short`, `full` und `fuller` Formate zeigen die Commits in ähnlicher Form an, aber mit jeweils mehr oder weniger Informationen.

	$ git log --pretty=oneline
	ca82a6dff817ec66f44342007202690a93763949 changed the version number
	085bb3bcb608e1e8451d4b2432f8ecbe6306e7e7 removed unnecessary test code
	a11bef06a3f659402fe7563abf99ad00de2209e6 first commit

<!--The most interesting option is `format`, which allows you to specify your own log output format. This is especially useful when you’re generating output for machine parsing — because you specify the format explicitly, you know it won’t change with updates to Git:-->

Eines der interessantesten Formate ist `format`, das Dir erlaubt, Dein eigenes Format zu verwenden. Dies ist inbesondere nützlich, wenn Du die Ausgabe in ein anderes Programm einlesen willst (da Du das Format explizit angibst, kannst Du sicher sein, dass es sich nicht ändert, wenn Du Git auf eine neuere Version aktualisierst):

	$ git log --pretty=format:"%h - %an, %ar : %s"
	ca82a6d - Scott Chacon, 11 months ago : changed the version number
	085bb3b - Scott Chacon, 11 months ago : removed unnecessary test code
	a11bef0 - Scott Chacon, 11 months ago : first commit

<!--Table 2-1 lists some of the more useful options that format takes.-->

Tabelle 2-1 zeigt einige nützliche Optionen, die von `format` akzeptiert werden:

<!--	Option	Description of Output-->
<!--	%H	Commit hash-->
<!--	%h	Abbreviated commit hash-->
<!--	%T	Tree hash-->
<!--	%t	Abbreviated tree hash-->
<!--	%P	Parent hashes-->
<!--	%p	Abbreviated parent hashes-->
<!--	%an	Author name-->
<!--	%ae	Author e-mail-->
<!--	%ad	Author date (format respects the -\-date= option)-->
<!--	%ar	Author date, relative-->
<!--	%cn	Committer name-->
<!--	%ce	Committer email-->
<!--	%cd	Committer date-->
<!--	%cr	Committer date, relative-->
<!--	%s	Subject-->

	Option	Beschreibung
	%H	Commit Hash
	%h	Abgekürzter Commit Hash
	%T	Baum Hash
	%t	Abgekürzter Baum Hash
	%P	Eltern Hashs
	%p	Abgekürzte Eltern Hashs
	%an	Autor Name
	%ae	Autor E-Mail
	%ad	Autor Datum (format akzeptiert eine –-date= Option)
	%ar	Autor Datum, relativ
	%cn	Committer Name
	%ce	Committer E-Mail
	%cd	Committer Datum
	%cr	Committer Datum, relativ
	%s	Betreff

<!--You may be wondering what the difference is between _author_ and _committer_. The _author_ is the person who originally wrote the patch, whereas the _committer_ is the person who last applied the patch. So, if you send in a patch to a project and one of the core members applies the patch, both of you get credit — you as the author and the core member as the committer. We’ll cover this distinction a bit more in *Chapter 5*.-->

Du fragst Dich vielleicht, was der Unterschied zwischen Autor und Committer ist. Der Autor ist diejenige Person, die eine Änderung ursprünglich vorgenommen hat. Der Committer dagegen ist diejenige Person, die den Commit angelegt hat. D.h., wenn Du einen Patch an ein Projekt Team schickst und eines der Team Mitglieder den Patch akzeptiert und verwendet, wird beiden Anerkennung gezollt – sowohl Dir als Autor als auch dem Teammitglied als Comitter. Wir werden auf diese Unterschiedung in Kapitel 5 noch einmal genauer eingehen.

<!--The `oneline` and `format` options are particularly useful with another `log` option called `-\-graph`. This option adds a nice little ASCII graph showing your branch and merge history, which we can see in our copy of the Grit project repository:-->

Die `oneline` und `format` Optionen können außerdem zusammen mit einer weiteren Option `--graph` verwendet werden. Diese Option fügt einen netten kleinen ASCII Graphen hinzu, der die Branch- und Merge-Historie des Projektes anzeigt. Das kannst Du z.B. in Deinem Klon des Grit Projekt Repositorys sehen:

	$ git log --pretty=format:"%h %s" --graph
	* 2d3acf9 ignore errors from SIGCHLD on trap
	*  5e3ee11 Merge branch 'master' of git://github.com/dustin/grit
	|\
	| * 420eac9 Added a method for getting the current branch.
	* | 30e367c timeout code and tests
	* | 5a09431 add timeout protection to grit
	* | e1193f8 support for heads with slashes in them
	|/
	* d6016bc require time for xmlschema
	*  11d191e Merge branch 'defunkt' into local

<!--Those are only some simple output-formatting options to `git log` — there are many more. Table 2-2 lists the options we’ve covered so far and some other common formatting options that may be useful, along with how they change the output of the `log` command.-->

Das sind nur einige eher simple Format Optionen für die Ausgabe von `git log` – es gibt sehr viel mehr davon. Tabelle 2-2 listet diejenigen Optionen auf, die wir bisher besprochen haben, und einige weitere, die besonders nützlich sind:

<!--	Option	Description-->
<!--	-p	Show the patch introduced with each commit.-->
<!--	-\-word-diff	Show the patch in a word diff format.-->
<!--	-\-stat	Show statistics for files modified in each commit.-->
<!--	-\-shortstat	Display only the changed/insertions/deletions line from the -\-stat command.-->
<!--	-\-name-only	Show the list of files modified after the commit information.-->
<!--	-\-name-status	Show the list of files affected with added/modified/deleted information as well.-->
<!--	-\-abbrev-commit	Show only the first few characters of the SHA-1 checksum instead of all 40.-->
<!--	-\-relative-date	Display the date in a relative format (for example, “2 weeks ago”) instead of using the full date format.-->
<!--	-\-graph	Display an ASCII graph of the branch and merge history beside the log output.-->
<!--	-\-pretty	Show commits in an alternate format. Options include oneline, short, full, fuller, and format (where you specify your own format).-->
<!--	-\-oneline	A convenience option short for `-\-pretty=oneline -\-abbrev-commit`.-->

	Option	Beschreibung
	-p	Zeigt den Patch, der einem Commit entspricht.
	--word-diff	Führt den Vergleich Wort für Wort, anstatt Zeile für Zeile aus.
	--stat	Zeigt Statistiken über die in einem Commit geänderten Dateien und eingefügten/entfernten Zeilen.
	--shortstat	Zeigt nur die Kurzstatistik über eingefügte/entfernte Zeilen aus der `--stat` Option.
	--name-only	Zeigt die Liste der geänderte Dateien nach der Commit Information.
	--name-status	Zeigt die Liste der Dateien mit der hinzugefügt/geändert/entfernt Statistik.
	--abbrev-commit	Zeigt nur die ersten Zeichen einer SHA-1 Checksumme, nicht alle 40.
	--relative-date	Zeigt das Datum in relativem Format (z.B. „2 weeks ago“), nicht als vollständiges Datumsformat.
	--graph	Zeigt einen ASCII Graphen der Branch- und Merge-Historie neben der Ausgabe.
	--pretty	Zeigt Commits in einem alternativen Format. Gültige Optionen sind: oneline, short, full, fuller und format (mit dem Du Dein eigenes Format spezifizieren kannst)

<!--### Limiting Log Output ###-->
### Log Daten filtern ###

<!--In addition to output-formatting options, `git log` takes a number of useful limiting options — that is, options that let you show only a subset of commits. You’ve seen one such option already — the `-2` option, which shows only the last two commits. In fact, you can do `-<n>`, where `n` is any integer to show the last `n` commits. In reality, you’re unlikely to use that often, because Git by default pipes all output through a pager so you see only one page of log output at a time.-->

Zusätzlich zu den Formatierungsoptionen für die Ausgabe, akzeptiert `git log` eine Reihe nützlicher Optionen, um die Anzahl der ausgegebenen Commits einzuschränken. Eine solche Option haben wir bereits verwendet: die `-2` Option, die bewirkt, dass nur die letzten beiden Commits angezeigt werden. D.h., Du kannst `-<n>` verwenden, wobei `n` irgendeine ganze Zahl sein kann. Im Alltag wirst Du diese Option vermutlich nicht sehr oft verwenden, weil Git die Ausgabe standardmäßig formatiert, sodass nur jeweils eine Seite anzeigt.

<!--However, the time-limiting options such as `-\-since` and `-\-until` are very useful. For example, this command gets the list of commits made in the last two weeks:-->

Darüber hinaus gibt es noch die hilfreichen Optionen `--since` und `--until`, welche die Ausgabe auf Basis der Zeitangaben eingrenzen. Beispielsweise gibt der folgende Befehl eine Liste aller Commits aus, die in den letzten zwei Wochen angelegt wurden:

	$ git log --since=2.weeks

<!--This command works with lots of formats — you can specify a specific date (“2008-01-15”) or a relative date such as “2 years 1 day 3 minutes ago”.-->

Das funktioniert mit einer Reihe von Formaten. Git akzeptiert sowohl ein vollständiges Datum („2008-01-15“) oder ein relatives Datum wie „2 years 1 day 3 minutes ago“.

<!--You can also filter the list to commits that match some search criteria. The `-\-author` option allows you to filter on a specific author, and the `-\-grep` option lets you search for keywords in the commit messages. (Note that if you want to specify both author and grep options, you have to add `-\-all-match` or the command will match commits with either.)-->

Du kannst außerdem die Liste der Commits nach Suchkriterien filtern. Die `--author` Option erlaubt, nach einem bestimmten Autor zu suchen, und die `--grep` Option nach Stichworten in den Commit Meldungen. (Wenn Du sowohl nach dem Autor als auch nach Stichworten suchen willst, musst Du zusätzlich `--all-match` angeben – andernfalls zeigt der Befehl alle Commits, die entweder das eine oder das andere Kriterium erfüllen.)

<!--The last really useful option to pass to `git log` as a filter is a path. If you specify a directory or file name, you can limit the log output to commits that introduced a change to those files. This is always the last option and is generally preceded by double dashes (`-\-`) to separate the paths from the options.-->

Eine letzte sehr nützliche Option, die von `git log` akzeptiert wird, ist ein Pfad. Wenn Du einen Verzeichnis- oder Dateinamen angibst, kannst Du die Ausgabe auf Commits einschränken, die sich auf die jeweiligen Verzeichnisse oder Dateien beziehen. Der Pfad muss als letztes angegeben und mit einem doppelten Bindestrich (`--`) von den Optionen getrennt werden.

<!--In Table 2-3 we’ll list these and a few other common options for your reference.-->

Tabelle 2-3 zeigt die besprochenen und einige weitere, übliche Optionen:

<!--	Option	Description-->
<!--	-(n)	Show only the last n commits-->
<!--	-\-since, -\-after	Limit the commits to those made after the specified date.-->
<!--	-\-until, -\-before	Limit the commits to those made before the specified date.-->
<!--	-\-author	Only show commits in which the author entry matches the specified string.-->
<!--	-\-committer	Only show commits in which the committer entry matches the specified string.-->

	Option	Beschreibung
	-(n)	Begrenzt die Ausgabe auf die letzten n commits
	--since, --after	Zeigt nur Commits, die nach dem angegebenen Datum angelegt wurden.
	--until, --before	Zeigt nur Commits, die vor dem angegebenen Datum angelegt wurden.
	--author	Zeigt nur Commits, die von dem angegebenen Autor vorgenommen wurden.
	--committer	Zeigt nur Commits, die von dem angegebenen Committer angelegt wurden.

<!--For example, if you want to see which commits modifying test files in the Git source code history were committed by Junio Hamano in the month of October 2008 and were not merges, you can run something like this:-->

Um beispielweise alle Commits aus der Git Quelltext Historie anzuzeigen, die alle der folgende Bedinungen erfüllen:

* Autor des Commits ist Junio Hamano
* Commit Datum Oktober 2008
* Commits, welche Änderungen im Testverzeichnis beinhalten
* Commits, welche keine Merges sind

kannst Du folgenden Befehl verwenden:

	$ git log --pretty="%h - %s" --author=gitster --since="2008-10-01" \
	   --before="2008-11-01" --no-merges -- t/
	5610e3b - Fix testcase failure when extended attribute
	acd3b9e - Enhance hold_lock_file_for_{update,append}()
	f563754 - demonstrate breakage of detached checkout wi
	d1a43f2 - reset --hard/read-tree --reset -u: remove un
	51a94af - Fix "checkout --track -b newbranch" on detac
	b0ad11e - pull: allow "git pull origin $something:$cur

<!--Of the nearly 20,000 commits in the Git source code history, this command shows the 6 that match those criteria.-->

Aus etwa 20.000 Commits in der Git Quellcode Historie, filtert dieser Befehl gerade einmal 6 Commits heraus, die diesen Kriterien entsprechen.

<!--### Using a GUI to Visualize History ###-->
### Grafische Darstellung der Historie ###

<!--If you like to use a more graphical tool to visualize your commit history, you may want to take a look at a Tcl/Tk program called `gitk` that is distributed with Git. Gitk is basically a visual `git log` tool, and it accepts nearly all the filtering options that `git log` does. If you type `gitk` on the command line in your project, you should see something like Figure 2-2.-->

Wenn Dir eine grafische Anzeige der Commit Historie lieber ist, kannst Du das Tcl/Tk Programm `gitk`, welches mit Git ausgeliefert wird, ausprobieren. `gitk` ist im wesentlichen eine grafische Version von `git log` und akzeptiert fast alle Filteroptionen, die `git log` auch akzeptiert. Wenn Du `gitk` in einem Projekt ausführst, siehst Du etwa folgende Ausgabe:

<!--Figure 2-2. The gitk history visualizer.-->

Insert 18333fig0202.png
Bild 2-2. Die gitk Oberfläche

<!--You can see the commit history in the top half of the window along with a nice ancestry graph. The diff viewer in the bottom half of the window shows you the changes introduced at any commit you click.-->

Die Commit Historie wird in der oberen Hälfte des Fensters dargestellt. Daneben ein Graph, der die Branches und Merges zeigt. Nach Auswahl eines Commits, zeigt die Vergleichsanzeige in der unteren Hälfte des Fensters die jeweiligen Änderungen in diesem Commit.

<!--## Undoing Things ##-->
## Änderungen rückgängig machen ##

<!--At any stage, you may want to undo something. Here, we’ll review a few basic tools for undoing changes that you’ve made. Be careful, because you can’t always revert some of these undos. This is one of the few areas in Git where you may lose some work if you do it wrong.-->

Es kommt immer wieder mal vor, dass Du Änderungen rückgängig machen willst. Im Folgenden gehen wir auf einige grundlegende Möglichkeiten dazu ein. Sei allerdings vorsichtig damit, denn Du kannst nicht immer alles wieder herstellen, was Du rückgängig gemacht hast. Dies ist eine der wenigen Situationen in Git, in denen man Daten verlieren kann, wenn man etwas falsch macht.

<!--### Changing Your Last Commit ###-->
### Den letzten Commit ändern ###

<!--One of the common undos takes place when you commit too early and possibly forget to add some files, or you mess up your commit message. If you want to try that commit again, you can run commit with the `-\-amend` option:-->

Manchmal hat man einen Commit zu früh angelegt und möglicherweise vergessen, einige Dateien hinzuzufügen, oder eine falsche Commit Meldung verwendet. Wenn Du den letzten Commit korrigieren willst, kannst Du dazu `git commit` zusammen mit der `--amend` Option verwenden:

	$ git commit --amend

<!--This command takes your staging area and uses it for the commit. If you’ve made no changes since your last commit (for instance, you run this command immediately after your previous commit), then your snapshot will look exactly the same and all you’ll change is your commit message.-->

Dieser Befehl verwendet Deine Staging Area für den Commit. Wenn Du seit dem letzten Commit keine Änderungen vorgenommen hast (z.B. wenn Du den Befehl unmittelbar nach einem Commit ausführst), wird der Snapshot exakt genauso aussehen wie der vorherige – alles, was Du dann änderst, ist die Commit Meldung.

<!--The same commit-message editor fires up, but it already contains the message of your previous commit. You can edit the message the same as always, but it overwrites your previous commit.-->

Der Texteditor startet wie üblich, aber diesmal enthält er bereits die Meldung aus dem vorherigen Commit. Du kannst diese Meldung wie gewohnt bearbeiten, speichern und die vorherige Meldung dadurch überschreiben.

<!--As an example, if you commit and then realize you forgot to stage the changes in a file you wanted to add to this commit, you can do something like this:-->

Wenn Du beispielsweise einen Commit angelegt hast und dann feststellst, dass Du zuvor vergessen hast, die Änderungen in einer bestimmten Datei zur Staging Area hinzuzufügen, kannst Du folgendes tun:

	$ git commit -m 'initial commit'
	$ git add forgotten_file
	$ git commit --amend

<!--After these three commands, you end up with a single commit — the second commit replaces the results of the first.-->

Diese drei Befehle legen einen einzigen neuen Commit an – der letzte Befehl ersetzt dabei das Ergebnis des ersten Befehls.

<!--### Unstaging a Staged File ###-->
### Änderungen aus der Staging Area entfernen ###

<!--The next two sections demonstrate how to wrangle your staging area and working directory changes. The nice part is that the command you use to determine the state of those two areas also reminds you how to undo changes to them. For example, let’s say you’ve changed two files and want to commit them as two separate changes, but you accidentally type `git add *` and stage them both. How can you unstage one of the two? The `git status` command reminds you:-->

Die nächsten zwei Abschnitte gehen darauf ein, wie Du Änderungen in der Staging Area und dem Arbeitsverzeichnis verwalten kannst. Praktischerweise liefert Dir der Befehl `git status`, den Du verwendest, um den Status dieser beiden Bereiche zu überprüfen, zugleich auch einen Hinweis dafür, wie Du Änderungen rückgängig machen kanst. Nehmen wir beispielsweise an, Du hast zwei Dateien geändert und willst sie als zwei seperate Commits anlegen, Du hast aber versehentlich `git add *` ausgeführt und damit beide zur Staging Area hinzugefügt. Wie kannst Du jetzt eine der beiden Änderungen wieder aus der Staging Area nehmen? `git status` gibt Dir einen Hinweis:

	$ git add .
	$ git status
	On branch master
	Changes to be committed:
	  (use "git reset HEAD <file>..." to unstage)
	
	        modified:   README.txt
	        modified:   benchmarks.rb
	

<!--Right below the “Changes to be committed” text, it says "use `git reset HEAD <file>...` to unstage". So, let’s use that advice to unstage the `benchmarks.rb` file:-->

Direkt unter der Zeile „Changes to be committed“ findest Du den Hinweis „use `git reset HEAD <file>...` to unstage“, d.h. „aus der Staging Area zu entfernen“. Wir verwenden nun also diesen Befehl, um die Änderungen an der Datei `benchmarks.rb` aus der Staging Area zu nehmen:

	$ git reset HEAD benchmarks.rb
	Unstaged changes after reset:
	M       benchmarks.rb
	$ git status
	On branch master
	Changes to be committed:
	  (use "git reset HEAD <file>..." to unstage)
	
	        modified:   README.txt
	
	Changes not staged for commit:
	  (use "git add <file>..." to update what will be committed)
	  (use "git checkout -- <file>..." to discard changes in working directory)
	
	        modified:   benchmarks.rb
	

<!--The command is a bit strange, but it works. The `benchmarks.rb` file is modified but once again unstaged.-->

Der Befehl liest sich zunächst vielleicht etwas merkwürdig, aber wie Du siehst, funktioniert er. Die Datei benchmarks.rb ist weiterhin modifiziert, befindet sich aber nicht mehr in der Staging Area.

<!--### Unmodifying a Modified File ###-->
### Eine Änderung an einer Datei rückgängig machen ###

<!--What if you realize that you don’t want to keep your changes to the `benchmarks.rb` file? How can you easily unmodify it — revert it back to what it looked like when you last committed (or initially cloned, or however you got it into your working directory)? Luckily, `git status` tells you how to do that, too. In the last example output, the unstaged area looks like this:-->

Was aber, wenn Du die Änderungen an der Datei `benchmarks.rb` überhaupt nicht beibehalten willst? D.h., wenn Du sie in den Zustand zurückversetzen willst, in dem sie sich befand, als Du den letzten Commit angelegt hast (oder das Repository geklont hast). Das ist einfach, und glücklicherweise zeigt der `git status` Befehl ebenfalls bereits einen Hinweis dafür an. Die obige Ausgabe enthält den folgenden Text:

	Changes not staged for commit:
	  (use "git add <file>..." to update what will be committed)
	  (use "git checkout -- <file>..." to discard changes in working directory)
	
	        modified:   benchmarks.rb
	

<!--It tells you pretty explicitly how to discard the changes you’ve made (at least, the newer versions of Git, 1.6.1 and later, do this — if you have an older version, we highly recommend upgrading it to get some of these nicer usability features). Let’s do what it says:-->

Das sagt ziemlich klar, was wir zu tun haben um die Änderungen an der Datei zu verwerfen (genauer gesagt, Git tut dies seit der Version 1.6.1 – wenn Du eine ältere Version hast, empfehlen wir dir, sie zu aktualisieren). Wir führen den vorgeschlagenen Befehl also aus:

	$ git checkout -- benchmarks.rb
	$ git status
	On branch master
	Changes to be committed:
	  (use "git reset HEAD <file>..." to unstage)
	
	        modified:   README.txt
	

<!--You can see that the changes have been reverted. You should also realize that this is a dangerous command: any changes you made to that file are gone — you just copied another file over it. Don’t ever use this command unless you absolutely know that you don’t want the file. If you just need to get it out of the way, we’ll go over stashing and branching in the next chapter; these are generally better ways to go.-->

Die Änderung wurde also rückgängig gemacht: sie taucht nicht mehr in der Liste der geänderten Dateien auf. Sei Dir bewusst, dass dieser Befehl potentiell gefährlich ist, da er Änderungen an einer Datei vollständig verwirft. Es ist also ratsam, ihn nur dann zu verwenden, wenn Du Dir absolut sicher bist, dass Du die Änderungen nicht mehr brauchst. Für Situationen, in denen Du eine Änderung lediglich vorläufig aus dem Weg räumen willst, werden wir im nächsten Kapitel noch auf Stashing und Branching eingehen – die dazu besser geeignet sind.

<!--Remember, anything that is committed in Git can almost always be recovered. Even commits that were on branches that were deleted or commits that were overwritten with an `-\-amend` commit can be recovered (see *Chapter 9* for data recovery). However, anything you lose that was never committed is likely never to be seen again.-->

Beachte, dass alles was jemals in einem Commit in Git enthalten war, fast immer wieder hergestellt werden kann. Selbst Commits, die sich in gelöschten Branches befanden, oder Commits, die mit einem `--amend` Commit überschrieben wurden, können wieder hergestellt werden (siehe Kapitel 9 für Datenrettung). Allerdings wirst Du Änderungen, die es nie in einen Commit geschafft haben, wahrscheinlich auch nie wieder restaurieren können.

<!--## Working with Remotes ##-->
## Mit externen Repositorys arbeiten ##

<!--To be able to collaborate on any Git project, you need to know how to manage your remote repositories. Remote repositories are versions of your project that are hosted on the Internet or network somewhere. You can have several of them, each of which generally is either read-only or read/write for you. Collaborating with others involves managing these remote repositories and pushing and pulling data to and from them when you need to share work.-->
<!--Managing remote repositories includes knowing how to add remote repositories, remove remotes that are no longer valid, manage various remote branches and define them as being tracked or not, and more. In this section, we’ll cover these remote-management skills.-->

Um mit anderen via Git zusammenzuarbeiten, musst Du wissen, wie Du auf externe (engl. „remote“) Repositorys zugreifen kannst. Remote Repositorys sind Versionen Deines Projektes, die im Internet oder irgendwo in einem anderen Netzwerk gespeichert sind. Du kannst mehrere solcher Repositorys haben und Du kannst jedes davon entweder nur lesen oder lesen und schreiben. Mit anderen via Git zusammenzuarbeiten impliziert, solche Repositorys zu verwalten und Daten aus ihnen herunter- oder heraufzuladen, um Deine Arbeit für andere verfügbar zu machen. Um Remote Repositorys zu verwalten, muss man wissen, wie man sie anlegt und wieder entfernt, wenn sie nicht mehr verwendet werden, wie man externe Branches verwalten und nachverfolgen kann, und mehr. In diesem Kapitel werden wir auf diese Aufgaben eingehen.

<!--### Showing Your Remotes ###-->
### Remote Repositorys anzeigen ###

<!--To see which remote servers you have configured, you can run the `git remote` command. It lists the shortnames of each remote handle you’ve specified. If you’ve cloned your repository, you should at least see *origin* — that is the default name Git gives to the server you cloned from:-->

Der `git remote` Befehl zeigt Dir an, welche externen Server Du für Dein Projekt lokal konfiguriert hast, und listet die Kurzbezeichnungen für diese Remote Repository auf. Wenn Du ein Repository geklont hast, solltest Du mindestens `origin` sehen – welches der Standardname ist, den Git für denjenigen Server vergibt, von dem Du geklont hast:

	$ git clone git://github.com/schacon/ticgit.git
	Cloning into 'ticgit'...
	remote: Reusing existing pack: 1857, done.
	remote: Total 1857 (delta 0), reused 0 (delta 0)
	Receiving objects: 100% (1857/1857), 374.35 KiB | 193.00 KiB/s, done.
	Resolving deltas: 100% (772/772), done.
	Checking connectivity... done.
	$ cd ticgit
	$ git remote
	origin

<!--You can also specify `-v`, which shows you the URL that Git has stored for the shortname to be expanded to:-->

Du kannst außerdem die Option `-v` verwenden, welche für jeden Kurznamen auch die jeweilige URL anzeigt, die Git gespeichert hat:

	$ git remote -v
	origin  git://github.com/schacon/ticgit.git (fetch)
	origin  git://github.com/schacon/ticgit.git (push)

<!--If you have more than one remote, the command lists them all. For example, my Grit repository looks something like this.-->

Wenn Du mehr als ein Remote Repository konfiguriert hast, zeigt der Befehl alle an. Für mein eigenes Grit Repository sieht das beispielsweise wie folgt aus:

	$ cd grit
	$ git remote -v
	bakkdoor  git://github.com/bakkdoor/grit.git
	cho45     git://github.com/cho45/grit.git
	defunkt   git://github.com/defunkt/grit.git
	koke      git://github.com/koke/grit.git
	origin    git@github.com:mojombo/grit.git

<!--This means we can pull contributions from any of these users pretty easily. But notice that only the origin remote is an SSH URL, so it’s the only one I can push to (we’ll cover why this is in *Chapter 4*).-->

D.h., mein lokales Repository kennt die Repositorys von all diesen Leuten und ich kann ihre Beiträge zu meinem Projekt ganz einfach herunterladen und zum Projekt hinzufügen.

<!--### Adding Remote Repositories ###-->
### Remote Repositorys hinzufügen ###

<!--I’ve mentioned and given some demonstrations of adding remote repositories in previous sections, but here is how to do it explicitly. To add a new remote Git repository as a shortname you can reference easily, run `git remote add [shortname] [url]`:-->

Ich habe in vorangegangenen Kapiteln schon Beispiele dafür aufgezeigt, wie man ein Remote Repository hinzufügen kann, aber ich will noch einmal darauf eingehen. Um ein neues Remote Repository mit einem Kurznamen hinzuzufügen, den Du Dir leicht merken kannst, führst Du den Befehl `git remote add [shortname] [url]` aus:

	$ git remote
	origin
	$ git remote add pb git://github.com/paulboone/ticgit.git
	$ git remote -v
	origin	git://github.com/schacon/ticgit.git
	pb	git://github.com/paulboone/ticgit.git

<!--Now you can use the string `pb` on the command line in lieu of the whole URL. For example, if you want to fetch all the information that Paul has but that you don’t yet have in your repository, you can run `git fetch pb`:-->

Jetzt kannst Du den Namen `pb` anstelle der vollständingen URL in verschiedenen Befehlen verwenden. Wenn Du bespielsweise alle Informationen, die in Pauls, aber noch nicht in Deinem eigenen Repository verfügbar sind, herunterladen willst, kannst Du den Befehl `git fetch pb` verwenden:

	$ git fetch pb
	remote: Counting objects: 58, done.
	remote: Compressing objects: 100% (41/41), done.
	remote: Total 44 (delta 24), reused 1 (delta 0)
	Unpacking objects: 100% (44/44), done.
	From git://github.com/paulboone/ticgit
	 * [new branch]      master     -> pb/master
	 * [new branch]      ticgit     -> pb/ticgit

<!--Paul’s master branch is accessible locally as `pb/master` — you can merge it into one of your branches, or you can check out a local branch at that point if you want to inspect it.-->

Pauls master Branch ist jetzt lokal auf Deinem Rechner als `pb/master` verfügbar – Du kannst ihn mit einem Deiner eigenen Branches zusammenführen oder auf einen lokalen Branch wechseln, um damit zu arbeiten.

<!--### Fetching and Pulling from Your Remotes ###-->
### Änderungen aus Remote Repositorys herunterladen und herunterladen inkl. zusammenführen ###

<!--As you just saw, to get data from your remote projects, you can run:-->

Wie Du gerade gesehen hast, kannst Du Daten aus Remote Repositorys herunterladen, indem Du den folgenden Befehl verwendest:

	$ git fetch [remote-name]

<!--The command goes out to that remote project and pulls down all the data from that remote project that you don’t have yet. After you do this, you should have references to all the branches from that remote, which you can merge in or inspect at any time. (We’ll go over what branches are and how to use them in much more detail in *Chapter 3*.)-->

Dieser Befehl lädt alle Daten aus dem Remote Repository herunter, die noch nicht auf Deinem Rechner verfügbar sind. Danach kennt Dein eigenes Repository Verweise auf alle Branches in dem Remote Repository, die Du jederzeit mit Deinen eigenen Branches zusammenführen oder durchschauen kannst. (Wir werden in Kapitel 3 detaillierter darauf eingehen, was genau Branches sind.)

<!--If you clone a repository, the command automatically adds that remote repository under the name *origin*. So, `git fetch origin` fetches any new work that has been pushed to that server since you cloned (or last fetched from) it. It’s important to note that the `fetch` command pulls the data to your local repository — it doesn’t automatically merge it with any of your work or modify what you’re currently working on. You have to merge it manually into your work when you’re ready.-->

Wenn Du ein Repository geklont hast, legt der Befehl automatisch einen Verweis auf dieses Repository unter dem Namen `origin` an. D.h. `git fetch origin` lädt alle Neuigkeiten herunter, die in dem Remote Repository von anderen hinzugefügt wurden, seit Du es geklont hast (oder zuletzt `git fetch` ausgeführt hast). Es ist wichtig, zu verstehen, dass der `git fetch` Befehl Daten lediglich in Dein lokales Repository lädt. Er führt sich mit Deinen eigenen Commits in keiner Weise zusammen (mergt) oder modifiziert, woran Du gerade arbeitest. D.h. Du musst die heruntergeladenen Änderungen anschließend selbst manuell mit Deinen eigenen zusammenführen, wenn Du das willst.

<!--If you have a branch set up to track a remote branch (see the next section and *Chapter 3* for more information), you can use the `git pull` command to automatically fetch and then merge a remote branch into your current branch. This may be an easier or more comfortable workflow for you; and by default, the `git clone` command automatically sets up your local master branch to track the remote master branch on the server you cloned from (assuming the remote has a master branch). Running `git pull` generally fetches data from the server you originally cloned from and automatically tries to merge it into the code you’re currently working on.-->

Wenn Du allerdings einen Branch so aufgesetzt hast, dass er einem Remote Branch „folgt“ (also einen „Tracking Branch“, wir werden im nächsten Abschnitt und in Kapitel 3 noch genauer darauf eingehen), dann kannst Du den Befehl `git pull` verwenden, um automatisch neue Daten herunterzuladen und den externen Branch gleichzeitig mit dem aktuellen, lokalen Branch zusammenzuführen. Das ist oft die bequemere Arbeitsweise. `git clone` setzt Deinen lokalen master Branch deshalb standardmäßig so auf, dass er dem Remote master Branch des geklonten Repositorys folgt (sofern das Remote Repository einen master Branch hat). Wenn Du dann `git pull` ausführst, wird Git die neuen Commits aus dem externen Repository holen und versuchen, sie automatisch mit dem Code zusammenzuführen, an dem Du gerade arbeitest.

<!--### Pushing to Your Remotes ###-->
### Änderungen in ein Remote Repository hochladen ###

<!--When you have your project at a point that you want to share, you have to push it upstream. The command for this is simple: `git push [remote-name] [branch-name]`. If you want to push your master branch to your `origin` server (again, cloning generally sets up both of those names for you automatically), then you can run this to push your work back up to the server:-->

Wenn Du mit Deinem Projekt an einen Punkt gekommen bist, an dem Du es anderen zur Verfügung stellen willst, kannst Du Deine Änderungen in ein gemeinsam genutztes Repository hochladen (engl. „push“). Der Befehl dafür ist einfach: `git push [remote-name] [branch-name]`. Wenn Du Deinen master Branch auf den `origin` Server hochladen willst (noch einmal, wenn Du ein Repository klonst, setzt Git diesen Namen automatisch für dich), dann kannst Du diesen Befehl verwenden:

	$ git push origin master

<!--This command works only if you cloned from a server to which you have write access and if nobody has pushed in the meantime. If you and someone else clone at the same time and they push upstream and then you push upstream, your push will rightly be rejected. You’ll have to pull down their work first and incorporate it into yours before you’ll be allowed to push. See *Chapter 3* for more detailed information on how to push to remote servers.-->

Das funktioniert nur dann, wenn Du Schreibrechte für das jeweilige Repository besitzt und niemand anders in der Zwischenzeit irgendwelche Änderungen hochgeladen hat. Wenn zwei Leute ein Repository zur gleichen Zeit klonen, dann zuerst der eine seine Änderungen hochlädt und der zweite anschließend versucht, das gleiche zu tun, dann wird sein Versuch korrekterweise abgewiesen. In dieser Situation muss man neue Änderungen zunächst herunterladen und mit seinen eigenen zusammenführen, um sie dann erst hochzuladen. In Kapitel 3 gehen wir noch einmal ausführlicher darauf ein.

<!--### Inspecting a Remote ###-->
### Ein Remote Repository durchstöbern ###

<!--If you want to see more information about a particular remote, you can use the `git remote show [remote-name]` command. If you run this command with a particular shortname, such as `origin`, you get something like this:-->

Wenn Du etwas über ein bestimmtes Remote Repository wissen willst, kannst Du den Befehl `git remote show [remote-name]` verwenden. Wenn Du diesen Befehl mit dem entsprechenden Kurznamen, z.B. `origin` verwendest, erhältst Du etwa folgende Ausgabe:

	$ git remote show origin
	* remote origin
	  URL: git://github.com/schacon/ticgit.git
	  Remote branch merged with 'git pull' while on branch master
	    master
	  Tracked remote branches
	    master
	    ticgit

<!--It lists the URL for the remote repository as well as the tracking branch information. The command helpfully tells you that if you’re on the master branch and you run `git pull`, it will automatically merge in the master branch on the remote after it fetches all the remote references. It also lists all the remote references it has pulled down.-->

Das zeigt Dir die URL für das Remote Repository, die Information welche Branches verfolgt werden und welcher Branch aus dem Remote Repository mit Deinem eigenen Master zusammengeführt wird, wenn Du `git pull` ausführst.

<!--That is a simple example you’re likely to encounter. When you’re using Git more heavily, however, you may see much more information from `git remote show`:-->

Dies ist ein eher einfaches Beispiel, das Dir früher oder später so ähnlich über den Weg laufen wird. Wenn Du Git aber täglich verwendest, erhältst Du mit `git remote show` sehr viel mehr Informationen:

	$ git remote show origin
	* remote origin
	  URL: git@github.com:defunkt/github.git
	  Remote branch merged with 'git pull' while on branch issues
	    issues
	  Remote branch merged with 'git pull' while on branch master
	    master
	  New remote branches (next fetch will store in remotes/origin)
	    caching
	  Stale tracking branches (use 'git remote prune')
	    libwalker
	    walker2
	  Tracked remote branches
	    acl
	    apiv2
	    dashboard2
	    issues
	    master
	    postgres
	  Local branch pushed with 'git push'
	    master:master

<!--This command shows which branch is automatically pushed when you run `git push` on certain branches. It also shows you which remote branches on the server you don’t yet have, which remote branches you have that have been removed from the server, and multiple branches that are automatically merged when you run `git pull`.-->

Dieser Befehl zeigt, welcher Branch automatisch hochgeladen werden wird, wenn Du `git push` auf bestimmten Branches ausführst. Er zeigt außerdem, welche Branches es im Remote Repository gibt, die Du selbst noch nicht hast, welche Branches dort gelöscht wurden, und Branches, die automatisch mit lokalen Branches zusammengeführt werden, wenn Du `git pull` ausführst.

<!--### Removing and Renaming Remotes ###-->
### Verweise auf externe Repositorys löschen und umbenennen ###

<!--If you want to rename a reference, in newer versions of Git you can run `git remote rename` to change a remote’s shortname. For instance, if you want to rename `pb` to `paul`, you can do so with `git remote rename`:-->

Wenn Du eine Referenz auf ein Remote Repository umbenennen willst, kannst Du in neueren Git Versionen den Befehl `git remote rename` verwenden, um den Kurznamen zu ändern. Wenn Du beispielsweise `pb` in `paul` umbenennen willst, lautet der Befehl:

	$ git remote rename pb paul
	$ git remote
	origin
	paul

<!--It’s worth mentioning that this changes your remote branch names, too. What used to be referenced at `pb/master` is now at `paul/master`.-->

Beachte dabei, dass dies Deine Branch Namen für Remote Branches ebenfalls ändert. Der Branch, der zuvor mit `pb/master` referenziert werden konnte, heißt jetzt `paul/master`.

<!--If you want to remove a reference for some reason — you’ve moved the server or are no longer using a particular mirror, or perhaps a contributor isn’t contributing anymore — you can use `git remote rm`:-->

Wenn Du eine Referenz aus irgendeinem Grund entfernen willst (z.B. weil Du den Server umgezogen hast oder einen bestimmten Mirror nicht länger verwendest, oder weil jemand vielleicht nicht länger mitarbeitet), kannst Du `git remote rm` verwenden:

	$ git remote rm paul
	$ git remote
	origin

<!--## Tagging ##-->
## Tags ##

<!--Like most VCSs, Git has the ability to tag specific points in history as being important. Generally, people use this functionality to mark release points (`v1.0`, and so on). In this section, you’ll learn how to list the available tags, how to create new tags, and what the different types of tags are.-->

Wie die meisten anderen VCS kann Git bestimmte Punkte in der Historie als besonders wichtig markieren, also taggen. Normalerweise verwendet man diese Funktionalität, um Release Versionen zu markieren (z.B. v1.0). In diesem Abschnitt gehen wir darauf ein, wie Du vorhandene Tags anzeigen und neue Tags erstellen kannst, und worin die Unterschiede zwischen verschiedenen Typen von Tags bestehen.

<!--### Listing Your Tags ###-->
### Vorhandene Tags anzeigen ###

<!--Listing the available tags in Git is straightforward. Just type `git tag`:-->

Um die in einem Repository vorhandenen Tags anzuzeigen, kannst Du den Befehl `git tag` ohne irgendwelche weiteren Optionen verwenden:

	$ git tag
	v0.1
	v1.3

<!--This command lists the tags in alphabetical order; the order in which they appear has no real importance.-->

Dieser Befehl listet die Tags in alphabetischer Reihenfolge auf. Die Reihenfolge ist aber eigentlich nicht so wichtig.

<!--You can also search for tags with a particular pattern. The Git source repo, for instance, contains more than 240 tags. If you’re only interested in looking at the 1.4.2 series, you can run this:-->

Du kannst auch nach Tags mit einem bestimmten Muster suchen. Das Git Quellcode Repository enthält beispielsweise mehr als 240 Tags. Wenn Du nur an denjenigen interessiert bist, die zur Version 1.4.2 gehören, kannst Du folgendes tun:

	$ git tag -l 'v1.4.2.*'
	v1.4.2.1
	v1.4.2.2
	v1.4.2.3
	v1.4.2.4

<!--### Creating Tags ###-->
### Neue Tags anlegen ###

<!--Git uses two main types of tags: lightweight and annotated. A lightweight tag is very much like a branch that doesn’t change — it’s just a pointer to a specific commit. Annotated tags, however, are stored as full objects in the Git database. They’re checksummed; contain the tagger name, e-mail, and date; have a tagging message; and can be signed and verified with GNU Privacy Guard (GPG). It’s generally recommended that you create annotated tags so you can have all this information; but if you want a temporary tag or for some reason don’t want to keep the other information, lightweight tags are available too.-->

Git kennt im wesentlichen zwei Typen von Tags: einfache (engl. lightweight) und kommentierte (engl. annotated) Tags. Ein einfacher Tag ist wie ein Branch, der sich niemals ändert – es ist lediglich ein Zeiger auf einen bestimmten Commit. Kommentierte Tags dagegen werden als vollwertige Objekte in der Git Datenbank gespeichert. Sie haben eine Checksumme, beinhalten Namen und E-Mail Adresse desjenigen, der den Tag angelegt hat, das jeweilige Datum sowie eine Meldung. Sie können überdies mit GNU Privacy Guard (GPG) signiert und verifiziert werden. Generell empfiehlt sich deshalb, kommentierte Tags anzulegen. Wenn man aber aus irgendeinem Grund einen temporären Tag anlegen will, für den all diese zusätzlichen Informationen nicht nötig sind, dann kann man auf einfache Tags zurückgreifen.

<!--### Annotated Tags ###-->
### Kommentierte Tags ###

<!--Creating an annotated tag in Git is simple. The easiest way is to specify `-a` when you run the `tag` command:-->

Einen kommentierten Tag legst Du an, indem Du dem `git tag` Befehl die Option `-a` übergibst:

	$ git tag -a v1.4 -m 'my version 1.4'
	$ git tag
	v0.1
	v1.3
	v1.4

<!--The `-m` specifies a tagging message, which is stored with the tag. If you don’t specify a message for an annotated tag, Git launches your editor so you can type it in.-->

Die Option `-m` gibt dabei wiederum die Meldung an, die zum Tag hinzugefügt wird. Wenn Du keine Meldung angibst, startet Git wie üblich Deinen Editor, sodass Du eine Meldung eingeben kannst.

<!--You can see the tag data along with the commit that was tagged by using the `git show` command:-->

`git show` zeigt Dir dann folgenden Tag zusammen mit dem jeweiligen Commit, auf den der Tag verweist, an:

	$ git show v1.4
	tag v1.4
	Tagger: Scott Chacon <schacon@gee-mail.com>
	Date:   Mon Feb 9 14:45:11 2009 -0800

	my version 1.4

	commit 15027957951b64cf874c3557a0f3547bd83b3ff6
	Merge: 4a447f7... a6b4c97...
	Author: Scott Chacon <schacon@gee-mail.com>
	Date:   Sun Feb 8 19:02:46 2009 -0800

	    Merge branch 'experiment'

<!--That shows the tagger information, the date the commit was tagged, and the annotation message before showing the commit information.-->

Die Ausgabe listet also zunächst die Informationen über denjenigen auf, der den Tag angelegt hat, sowie die Tag Meldung und dann die Commit Informationen selbst.

<!--### Signed Tags ###-->
### Signierte Tags ###

<!--You can also sign your tags with GPG, assuming you have a private key. All you have to do is use `-s` instead of `-a`:-->

Wenn Du einen privaten GPG Schlüssel hast, kannst Du Deine Tags zusätzlich mit GPG signieren. Dazu verwendest Du einfach die Option `-s` anstelle von `-a`:

	$ git tag -s v1.5 -m 'my signed 1.5 tag'
	You need a passphrase to unlock the secret key for
	user: "Scott Chacon <schacon@gee-mail.com>"
	1024-bit DSA key, ID F721C45A, created 2009-02-09

<!--If you run `git show` on that tag, you can see your GPG signature attached to it:-->

Wenn Du jetzt `git show` auf diesen Tag anwendest, siehst Du, dass der Tag Deine GPG Signatur hinterlegt hat:

	$ git show v1.5
	tag v1.5
	Tagger: Scott Chacon <schacon@gee-mail.com>
	Date:   Mon Feb 9 15:22:20 2009 -0800

	my signed 1.5 tag
	-----BEGIN PGP SIGNATURE-----
	Version: GnuPG v1.4.8 (Darwin)

	iEYEABECAAYFAkmQurIACgkQON3DxfchxFr5cACeIMN+ZxLKggJQf0QYiQBwgySN
	Ki0An2JeAVUCAiJ7Ox6ZEtK+NvZAj82/
	=WryJ
	-----END PGP SIGNATURE-----
	commit 15027957951b64cf874c3557a0f3547bd83b3ff6
	Merge: 4a447f7... a6b4c97...
	Author: Scott Chacon <schacon@gee-mail.com>
	Date:   Sun Feb 8 19:02:46 2009 -0800

	    Merge branch 'experiment'

<!--A bit later, you’ll learn how to verify signed tags.-->

Darauf, wie Du signierte Tags verifizieren kannst, werden wir gleich noch eingehen.

<!--### Lightweight Tags ###-->
### Einfache Tags ###

<!--Another way to tag commits is with a lightweight tag. This is basically the commit checksum stored in a file — no other information is kept. To create a lightweight tag, don’t supply the `-a`, `-s`, or `-m` option:-->

Einfache Tags sind die zweite Form von Tags, die Git kennt. Für einen einfachen Tag wird im wesentlichen die jeweilige Commit Prüfsumme, und sonst keine andere Information, in einer Datei gespeichert. Um einen einfachen Tag anzulegen, verwendest Du einfach keine der drei Optionen `-a`, `-s` und `-m`:

	$ git tag v1.4-lw
	$ git tag
	v0.1
	v1.3
	v1.4
	v1.4-lw
	v1.5

<!--This time, if you run `git show` on the tag, you don’t see the extra tag information. The command just shows the commit:-->

Wenn Du jetzt `git show` auf den Tag ausführst, siehst Du keine der zusätzlichen Tag Informationen. Der Befehl zeigt einfach den jeweiligen Commit:

	$ git show v1.4-lw
	commit 15027957951b64cf874c3557a0f3547bd83b3ff6
	Merge: 4a447f7... a6b4c97...
	Author: Scott Chacon <schacon@gee-mail.com>
	Date:   Sun Feb 8 19:02:46 2009 -0800

	    Merge branch 'experiment'

<!--### Verifying Tags ###-->
### Tags verifizieren ###

<!--To verify a signed tag, you use `git tag -v [tag-name]`. This command uses GPG to verify the signature. You need the signer’s public key in your keyring for this to work properly:-->

Um einen signierten Tag zu verifizieren, kannst Du `git tag -v [Tag Name]` verwenden. Dieser Befehl verwendet GPG, um die Signatur mit Hilfe des öffentlichen Schlüssels des Signierenden zu verifizieren – weshalb Du diesen Schlüssel in Deinem Schlüsselbund haben musst:

	$ git tag -v v1.4.2.1
	object 883653babd8ee7ea23e6a5c392bb739348b1eb61
	type commit
	tag v1.4.2.1
	tagger Junio C Hamano <junkio@cox.net> 1158138501 -0700

	GIT 1.4.2.1

	Minor fixes since 1.4.2, including git-mv and git-http with alternates.
	gpg: Signature made Wed Sep 13 02:08:25 2006 PDT using DSA key ID F3119B9A
	gpg: Good signature from "Junio C Hamano <junkio@cox.net>"
	gpg:                 aka "[jpeg image of size 1513]"
	Primary key fingerprint: 3565 2A26 2040 E066 C9A7  4A7D C0C6 D9A4 F311 9B9A

<!--If you don’t have the signer’s public key, you get something like this instead:-->

Wenn Du den öffentlichen Schlüssel des Signierenden nicht in Deinem Schlüsselbund hast, wirst Du statt dessen eine Meldung sehen wie:

	gpg: Signature made Wed Sep 13 02:08:25 2006 PDT using DSA key ID F3119B9A
	gpg: Can't check signature: public key not found
	error: could not verify the tag 'v1.4.2.1'

<!--### Tagging Later ###-->
### Nachträglich taggen ###

<!--You can also tag commits after you’ve moved past them. Suppose your commit history looks like this:-->

Du kannst Commits jederzeit taggen, auch lange Zeit nachdem sie angelegt wurden. Nehmen wir an, Deine Commit Historie sieht wie folgt aus:

	$ git log --pretty=oneline
	15027957951b64cf874c3557a0f3547bd83b3ff6 Merge branch 'experiment'
	a6b4c97498bd301d84096da251c98a07c7723e65 beginning write support
	0d52aaab4479697da7686c15f77a3d64d9165190 one more thing
	6d52a271eda8725415634dd79daabbc4d9b6008e Merge branch 'experiment'
	0b7434d86859cc7b8c3d5e1dddfed66ff742fcbc added a commit function
	4682c3261057305bdd616e23b64b0857d832627b added a todo file
	166ae0c4d3f420721acbb115cc33848dfcc2121a started write support
	9fceb02d0ae598e95dc970b74767f19372d61af8 updated rakefile
	964f16d36dfccde844893cac5b347e7b3d44abbc commit the todo
	8a5cbc430f1a9c3d00faaeffd07798508422908a updated readme

<!--Now, suppose you forgot to tag the project at `v1.2`, which was at the "updated rakefile" commit. You can add it after the fact. To tag that commit, you specify the commit checksum (or part of it) at the end of the command:-->

Nehmen wir an, dass Du vergessen hast, Version v1.2 des Projekts zu taggen und dass dies der Commit „updated rakefile“ gewesen ist. Du kannst diesen jetzt im Nachhinein taggen, indem Du die Checksumme des Commits (oder einen Teil davon) am Ende des Befehls angibst:

	$ git tag -a v1.2 -m 'version 1.2' 9fceb02

<!--You can see that you’ve tagged the commit:-->

Du siehst jetzt, dass Du einen Tag für den Commit angelegt hast:

	$ git tag
	v0.1
	v1.2
	v1.3
	v1.4
	v1.4-lw
	v1.5

	$ git show v1.2
	tag v1.2
	Tagger: Scott Chacon <schacon@gee-mail.com>
	Date:   Mon Feb 9 15:32:16 2009 -0800

	version 1.2
	commit 9fceb02d0ae598e95dc970b74767f19372d61af8
	Author: Magnus Chacon <mchacon@gee-mail.com>
	Date:   Sun Apr 27 20:43:35 2008 -0700

	    updated rakefile
	...

<!--### Sharing Tags ###-->
### Tags veröffentlichen ###

<!--By default, the `git push` command doesn’t transfer tags to remote servers. You will have to explicitly push tags to a shared server after you have created them.  This process is just like sharing remote branches — you can run `git push origin [tagname]`.-->

Der `git push` Befehl lädt Tags nicht von sich aus auf externe Server. Stattdessen muss Du Tags explizit auf einen externen Server hochladen, nachdem Du sie angelegt hast. Der Vorgang entspricht dem  bei Branches: Du kannst den Befehl `git push origin [tagname]` verwenden.

	$ git push origin v1.5
	Counting objects: 50, done.
	Compressing objects: 100% (38/38), done.
	Writing objects: 100% (44/44), 4.56 KiB, done.
	Total 44 (delta 18), reused 8 (delta 1)
	To git@github.com:schacon/simplegit.git
	* [new tag]         v1.5 -> v1.5

<!--If you have a lot of tags that you want to push up at once, you can also use the `-\-tags` option to the `git push` command.  This will transfer all of your tags to the remote server that are not already there.-->

Wenn Du viele Tags auf einmal hochladen willst, kannst Du dem `git push` Befehl außerdem die `--tags` Option übergeben und auf diese Weise sämtliche Tags auf dem Remote Server veröffentlichen, die dort noch nicht bekannt sind.

	$ git push origin --tags
	Counting objects: 50, done.
	Compressing objects: 100% (38/38), done.
	Writing objects: 100% (44/44), 4.56 KiB, done.
	Total 44 (delta 18), reused 8 (delta 1)
	To git@github.com:schacon/simplegit.git
	 * [new tag]         v0.1 -> v0.1
	 * [new tag]         v1.2 -> v1.2
	 * [new tag]         v1.4 -> v1.4
	 * [new tag]         v1.4-lw -> v1.4-lw
	 * [new tag]         v1.5 -> v1.5

<!--Now, when someone else clones or pulls from your repository, they will get all your tags as well.-->

Wenn jetzt jemand anderes das Repository klont oder von dort aktualisiert, wird er all diese Tags ebenfalls erhalten.

<!--## Tips and Tricks ##-->
## Tipps und Tricks ##

<!--Before we finish this chapter on basic Git, a few little tips and tricks may make your Git experience a bit simpler, easier, or more familiar. Many people use Git without using any of these tips, and we won’t refer to them or assume you’ve used them later in the book; but you should probably know how to do them.-->

Bevor wir zum Ende dieses Grundlagenkapitels kommen, möchten wir noch einige Tipps und Tricks vorstellen, die Dir den Umgang mit Git ein bisschen vereinfachen können. Du kannst Git natürlich einsetzen, ohne diese Tipps anzuwenden, und wir werden später in diesem Buch auch nicht darauf Bezug nehmen oder sie voraussetzen. Aber wir finden, Du solltest sie kennen, weil sie einfach nützlich sind.

<!--### Auto-Completion ###-->
### Auto-Vervollständigung ###

<!--If you use the Bash shell, Git comes with a nice auto-completion script you can enable. Download it directly from the Git source code at https://github.com/git/git/blob/master/contrib/completion/git-completion.bash . Copy this file to your home directory, and add this to your `.bashrc` file:-->

Wenn Du die Bash Shell verwendest, dann kannst Du ein Skript für die Git Auto-Vervollständigung einbinden. Du kannst dieses Skript direkt aus den Git Quellen von https://github.com/git/git/blob/master/contrib/completion/git-completion.bash herunterladen. Kopiere diese Datei in Dein Home Verzeichnis  und füge die folgende Zeile in Deine `.bashrc` Datei hinzu:

	source ~/git-completion.bash

<!--If you want to set up Git to automatically have Bash shell completion for all users, copy this script to the `/opt/local/etc/bash_completion.d` directory on Mac systems or to the `/etc/bash_completion.d/` directory on Linux systems. This is a directory of scripts that Bash will automatically load to provide shell completions.-->

Wenn Du Git Auto-Vervollständigung für alle Benutzer Deines Rechners aufsetzen willst, kopiere das Skript in das Verzeichnis `/opt/local/etc/bash_completion.d` (auf Mac OS X Systemen) bzw. `/etc/bash_completion.d/` (auf Linux Systemen). Bash sucht in diesem Verzeichnis nach Erweiterungen für die Autovervollständigung und lädt sie automatisch.

<!--If you’re using Windows with Git Bash, which is the default when installing Git on Windows with msysGit, auto-completion should be preconfigured.-->

Auf Windows Systemen sollte die Autovervollständigung bereits aktiv sein, wenn Du die Git Bash aus dem msysGit Paket verwendest.

<!--Press the Tab key when you’re writing a Git command, and it should return a set of suggestions for you to pick from:-->

Während Du einen Git Befehl eintippst, kannst Du die Tab Taste drücken und Du erhälst eine Auswahl von Vorschlägen, aus denen Du auswählen kannst:

	$ git co<tab><tab>
	commit config

<!--In this case, typing `git co` and then pressing the Tab key twice suggests commit and config. Adding `m<tab>` completes `git commit` automatically.-->

D.h., wenn Du `git co` schreibst und dann die Tab Taste zwei Mal drückst, erhältst Du die Vorschläge `commit` und `config`. Wenn Du Tab nur ein Mal drückst, vervollständigt den Befehl Deine Eingabe direkt zu `git commit`.

<!--This also works with options, which is probably more useful. For instance, if you’re running a `git log` command and can’t remember one of the options, you can start typing it and press Tab to see what matches:-->

Das funktioniert auch mit Optionen – was oftmals noch hilfreicher ist. Wenn Du beispielsweise `git log` verwenden willst und Dich nicht an eine bestimmte Option erinnern kannst, schreibst Du einfach den Befehl und drückst die Tab Taste, um die Optionen anzuzeigen:

	$ git log --s<tab>
	--shortstat  --since=  --src-prefix=  --stat   --summary

<!--That’s a pretty nice trick and may save you some time and documentation reading.-->

Du musst also nicht dauernd die Dokumentation zu Rate ziehen und erspart Dir somit etwas Zeit. Ein toller Trick, nicht wahr?

<!--### Git Aliases ###-->
### Git Aliase ###

<!--Git doesn’t infer your command if you type it in partially. If you don’t want to type the entire text of each of the Git commands, you can easily set up an alias for each command using `git config`. Here are a couple of examples you may want to set up:-->

Git versucht nicht zu erraten, welchen Befehl Du verwenden willst, wenn Du ihn nur teilweise eingibst. Wenn Du lange Befehle nicht immer wieder eintippen willst, kannst Du mit `git config` auf einfache Weise Aliase definieren. Hier einige Beispiele, die Du vielleicht nützlich findest:

	$ git config --global alias.co checkout
	$ git config --global alias.br branch
	$ git config --global alias.ci commit
	$ git config --global alias.st status

<!--This means that, for example, instead of typing `git commit`, you just need to type `git ci`. As you go on using Git, you’ll probably use other commands frequently as well; in this case, don’t hesitate to create new aliases.-->

Das heißt, dass Du z.B. einfach `git ci` anstelle von `git commit` schreiben kannst. Wenn Du Git oft verwendest, werden Dir sicher weitere Befehle begegnen, die Du sehr oft nutzt. In diesem Fall zögere nicht, weitere Aliase zu definieren.

<!--This technique can also be very useful in creating commands that you think should exist. For example, to correct the usability problem you encountered with unstaging a file, you can add your own unstage alias to Git:-->

Diese Technik kann auch dabei helfen, Git Befehle zu definieren, von denen Du denkst, es sollte sie geben:

	$ git config --global alias.unstage 'reset HEAD --'

<!--This makes the following two commands equivalent:-->

Das bewirkt, dass die beiden folgenden Befehle äquivalent sind:

	$ git unstage fileA
	$ git reset HEAD fileA

<!--This seems a bit clearer. It’s also common to add a `last` command, like this:-->

Unser neuer Alias ist wahrscheinlich aussagekräftiger, oder? Ein weiterer, typischer Alias ist der `last` Befehl:

	$ git config --global alias.last 'log -1 HEAD'

<!--This way, you can see the last commit easily:-->

Auf diese Weise kannst Du leicht den letzten Commit nachschlagen:

	$ git last
	commit 66938dae3329c7aebe598c2246a8e6af90d04646
	Author: Josh Goebel <dreamer3@example.com>
	Date:   Tue Aug 26 19:48:51 2008 +0800

	    test for current head

	    Signed-off-by: Scott Chacon <schacon@example.com>

<!--As you can tell, Git simply replaces the new command with whatever you alias it to. However, maybe you want to run an external command, rather than a Git subcommand. In that case, you start the command with a `!` character. This is useful if you write your own tools that work with a Git repository. We can demonstrate by aliasing `git visual` to run `gitk`:-->

Wie Du Dir denken kannst, ersetzt Git ganz einfach den Alias mit dem jeweiligen Befehl, für den er definiert ist. Wenn Du allerdings einen externen Befehl anstelle eines Git Befehls ausführen willst, kannst Du den Befehl mit einem Auführungszeichen (`!`) am Anfang kennzeichnen. Das ist in der Regel nützlich, wenn Du Deine eigenen Hilfsmittel schreibst, um Git zu erweitern. Wir können das demonstrieren, indem wir `git visual` als `gitk` definieren:

	$ git config --global alias.visual '!gitk'

<!--## Summary ##-->
## Zusammenfassung ##

<!--At this point, you can do all the basic local Git operations — creating or cloning a repository, making changes, staging and committing those changes, and viewing the history of all the changes the repository has been through. Next, we’ll cover Git’s killer feature: its branching model.-->

Du solltest jetzt in der Lage sein, die wichtigsten Git Befehle einzusetzen und Repositorys neu zu erzeugen und zu klonen, Änderungen vorzunehmen und zur Staging Area hinzuzufügen, Commits anzulegen und die Historie aller Commits in einem Repository zu durchsuchen. Als nächstes werden wir auf ein herausragendes Feature von Git eingehen: das Branch Konzept.
<!--# Git Branching #-->
# Git Branching #

<!--Nearly every VCS has some form of branching support. Branching means you diverge from the main line of development and continue to do work without messing with that main line. In many VCS tools, this is a somewhat expensive process, often requiring you to create a new copy of your source code directory, which can take a long time for large projects.-->

Nahezu jedes VCS unterstützt eine Form von Branching. Branching bedeutet, dass Du von der Hauptentwicklungslinie abzweigst und unabhängig vom Hauptzweig weiterarbeitest. Bei vielen VCS ist das ein ziemlch aufwendiger Prozess, welcher häufig erfordert, eine Kopie des kompletten Arbeitsverzeichnisses zu erstellen, was bei großen Projekten eine ganze Weile dauern kann.

<!--Some people refer to the branching model in Git as its “killer feature”  , and it certainly sets Git apart in the VCS community. Why is it so special? The way Git branches is incredibly lightweight, making branching operations nearly instantaneous and switching back and forth between branches generally just as fast. Unlike many other VCSs, Git encourages a workflow that branches and merges often, even multiple times in a day. Understanding and mastering this feature gives you a powerful and unique tool and can literally change the way that you develop.-->

Manche Leute bezeichnen Gits Branching-Modell als dessen „Killer-Feature“, was Git zweifellos von dem Rest der VCS-Community abhebt. Was ist das Besondere daran? Die Art und Weise, wie Git Branches behandelt, ist unglaublich leichtgewichtig, wodurch Branch-Operationen nahezu verzögerungsfrei ausgeführt werden und auch das Hin- und Herschalten zwischen einzelnen Entwicklungszweigen im allgemeinen genauso schnell abläuft. Im Gegensatz zu anderen VCS ermutigt Git zu einer Arbeitsweise mit häufigem Branching und Merging – oft mehrmals am Tag. Wenn Du diese Funktion vertstehst und beherrschst, besitzt Du ein mächtiges und einmaliges Werkzeug, welches Deine Art zu entwickeln buchstäblich verändern.

<!--## What a Branch Is ##-->
## Was ist ein Branch? ##

<!--To really understand the way Git does branching, we need to take a step back and examine how Git stores its data. As you may remember from Chapter 1, Git doesn’t store data as a series of changesets or deltas, but instead as a series of snapshots.-->

Um wirklich zu verstehen, wie Git Branching durchführt, müssen wir einen Schritt zurück gehen und untersuchen, wie Git die Daten speichert. Wie Du Dich vielleicht noch an Kapitel 1 erinnerst, speichert Git seine Daten nicht als Serie von Änderungen oder Unterschieden, sondern als Serie von Schnappschüssen.

<!--When you commit in Git, Git stores a commit object that contains a pointer to the snapshot of the content you staged, the author and message metadata, and zero or more pointers to the commit or commits that were the direct parents of this commit: zero parents for the first commit, one parent for a normal commit, and multiple parents for a commit that results from a merge of two or more branches.-->

Wenn Du in Git committest, speichert Git ein sogenanntes Commit-Objekt. Dieses enthält einen Zeiger zu dem Schnappschuss mit den Objekten der Staging-Area, dem Autor, den Commit-Metadaten und einem Zeiger zu den direkten Eltern des Commits. Ein initialer Commit hat keine Eltern-Commits, ein normaler Commit stammt von einem Eltern-Commit ab und ein Merge-Commit, welcher aus einer Zusammenführung von zwei oder mehr Branches resultiert, besitzt ebenso viele Eltern-Commits.

<!--To visualize this, let’s assume that you have a directory containing three files, and you stage them all and commit. Staging the files checksums each one (the SHA-1 hash we mentioned in Chapter 1), stores that version of the file in the Git repository (Git refers to them as blobs), and adds that checksum to the staging area:-->

Um das zu verdeutlichen, lass uns annehmen, Du hast ein Verzeichnis mit drei Dateien, die Du alle zu der Staging-Area hinzufügst und in einem Commit verpackst. Durch das Stagen der Dateien erzeugt Git für jede Datei eine Prüfsumme (der SHA-1 Hash, den wir in Kapitel 1 erwähnt haben), speichert diese Version der Datei im Git-Repository (Git referenziert auf diese als Blobs) und fügt die Prüfsumme der Staging-Area hinzu:

	$ git add README test.rb LICENSE
	$ git commit -m 'initial commit of my project'

<!--Running `git commit` checksums all project directories and stores them as `tree` objects in the Git repository. Git then creates a `commit` object that has the metadata and a pointer to the root project `tree` object so it can re-create that snapshot when needed.-->

Wenn Du einen Commit mit der Anweisung `git commit` erstellst, erzeugt Git für jedes Projektverzeichnis eine Prüfsumme und speichert diese als sogenanntes `tree`-Objekt im Git Repository. Git erzeugt dann ein Commit Objekt, das die Metadaten und den Zeiger zum `tree`-Objekt des Wurzelverzeichnis enthält, um bei Bedarf den Snapshot erneut erzeugen zu können.

<!--Your Git repository now contains five objects: one blob for the contents of each of your three files, one tree that lists the contents of the directory and specifies which file names are stored as which blobs, and one commit with the pointer to that root tree and all the commit metadata. Conceptually, the data in your Git repository looks something like Figure 3-1.-->

Dein Git-Repository enthält nun fünf Objekte: einen Blob für den Inhalt von jeder der drei Dateien, einen Baum, der den Inhalt des Verzeichnisses auflistet und spezifiziert, welcher Dateiname zu welchem Blob gehört, sowie einen Zeiger, der auf die Wurzel des Projektbaumes und die Metadaten des Commits verweist. Prinzipiell sehen Deine Daten im Git Repository wie in Abbildung 3-1 aus.

<!--Figure 3-1. Single commit repository data.-->

Insert 18333fig0301.png
Abbildung 3-1. Repository-Daten eines einzelnen Commits.

<!--If you make some changes and commit again, the next commit stores a pointer to the commit that came immediately before it. After two more commits, your history might look something like Figure 3-2.-->

Wenn Du erneut etwas änderst und wieder einen Commit machst, wird dieser einen Zeiger enthalten, der auf den Vorhergehenden verweist. Nach zwei weiteren Commits könnte die Historie wie in Abbildung 3-2 aussehen.

<!--Figure 3-2. Git object data for multiple commits.-->

Insert 18333fig0302.png
Abbildung 3-2. Git Objektdaten für mehrere Commits.

<!--A branch in Git is simply a lightweight movable pointer to one of these commits. The default branch name in Git is master. As you initially make commits, you’re given a `master` branch that points to the last commit you made. Every time you commit, it moves forward automatically.-->

Ein Branch in Git ist nichts anderes als ein simpler Zeiger auf einen dieser Commits. Der Standardname eines Git-Branches lautet `master`. Mit dem initialen Commit erhältst Du einen `master`-Branch, der auf Deinen letzten Commit zeigt. Mit jedem Commit bewegt er sich automatisch vorwärts.

<!--Figure 3-3. Branch pointing into the commit data’s history.-->

Insert 18333fig0303.png
Abbildung 3-3. Branch, der auf einen Commit in der Historie zeigt.

<!--What happens if you create a new branch? Well, doing so creates a new pointer for you to move around. Let’s say you create a new branch called testing. You do this with the `git branch` command:-->

Was passiert, wenn Du einen neuen Branch erstellst? Nun, zunächst wird ein neuer Zeiger erstellt. Sagen wir, Du erstellst einen neuen Branch mit dem Namen `testing`. Das machst Du mit der Anweisung `git branch`:

	$ git branch testing

<!--This creates a new pointer at the same commit you’re currently on (see Figure 3-4).-->

Dies erzeugt einen neuen Zeiger, der auf den gleichen Commit zeigt, auf dem Du gerade arbeitest (siehe Abbildung 3-4).

<!--Figure 3-4. Multiple branches pointing into the commit’s data history.-->

Insert 18333fig0304.png
Abbildung 3-4. Mehrere Branches zeigen in den Commit-Verlauf

<!--How does Git know what branch you’re currently on? It keeps a special pointer called HEAD. Note that this is a lot different than the concept of HEAD in other VCSs you may be used to, such as Subversion or CVS. In Git, this is a pointer to the local branch you’re currently on. In this case, you’re still on master. The git branch command only created a new branch — it didn’t switch to that branch (see Figure 3-5).-->

Woher weiß Git, welchen Branch Du momentan verwendest? Dafür gibt es einen speziellen Zeiger mit dem Namen HEAD. Berücksichtige, dass dieses Konzept sich grundsätzlich von anderen HEAD-Konzepten anderer VCS, wie Subversion oder CVS, unterscheidet. Bei Git handelt es sich bei HEAD um einen Zeiger, der auf Deinen aktuellen lokalen Branch zeigt. In dem Fall bist Du aber immer noch auf dem `master`-Branch. Die Anweisung `git branch` hat nur einen neuen Branch erstellt, aber nicht zu diesem gewechselt (siehe Abbildung 3-5).

<!--Figure 3-5. HEAD file pointing to the branch you’re on.-->

Insert 18333fig0305.png
Abbildung 3-5. Der HEAD-Zeiger verweist auf Deinen aktuellen Branch.

<!--To switch to an existing branch, you run the `git checkout` command. Let’s switch to the new testing branch:-->

Um zu einem anderen Branch zu wechseln, benutze die Anweisung `git checkout`. Lass uns nun zu unserem neuen Branch `testing` wechseln:

	$ git checkout testing

<!--This moves HEAD to point to the testing branch (see Figure 3-6).-->

Das lässt HEAD neuerdings auf den Branch „testing“ verweisen (siehe Abbildung 3-6).

<!--Figure 3-6. HEAD points to another branch when you switch branches.-->

Insert 18333fig0306.png
Abbildung 3-6. Wenn Du den Branch wechselst, zeigt HEAD auf einen neuen Zweig.

<!--What is the significance of that? Well, let’s do another commit:-->

Und was bedeutet das? Ok, lass uns noch einen weiteren Commit machen:

	$ vim test.rb
	$ git commit -a -m 'made a change'

<!--Figure 3-7 illustrates the result.-->

Abbildung 3-7 verdeutlicht das Ergebnis.

<!--Figure 3-7. The branch that HEAD points to moves forward with each commit.-->

Insert 18333fig0307.png
Abbildung 3-7. Der HEAD-Zeiger schreitet mit jedem weiteren Commit voran.

<!--This is interesting, because now your testing branch has moved forward, but your `master` branch still points to the commit you were on when you ran `git checkout` to switch branches. Let’s switch back to the `master` branch:-->

Das ist interessant, denn Dein Branch `testing` hat sich jetzt voranbewegt, während Dein `master`-Branch immer noch auf den Commit zeigt, welchen Du bearbeitet hast, bevor Du mit `git checkout` den aktuellen Zweig gewechselt hast. Lass uns zurück zu dem `master`-Branch wechseln:

	$ git checkout master

<!--Figure 3-8 shows the result.-->

Abbildung 3-8 zeigt das Ergebnis.

<!--Figure 3-8. HEAD moves to another branch on a checkout.-->

Insert 18333fig0308.png
Abbildung 3-8. HEAD zeigt nach einem Checkout auf einen anderen Branch.

<!--That command did two things. It moved the HEAD pointer back to point to the `master` branch, and it reverted the files in your working directory back to the snapshot that `master` points to. This also means the changes you make from this point forward will diverge from an older version of the project. It essentially rewinds the work you’ve done in your testing branch temporarily so you can go in a different direction.-->

Diese Anweisung hat zwei Dinge veranlasst. Zum einen bewegt es den HEAD-Zeiger zurück zum `master`-Branch, zum anderen setzt es alle Dateien im Arbeitsverzeichnis auf den Bearbeitungsstand des letzten Commits in diesem Zweig zurück. Das bedeutet aber auch, dass nun alle Änderungen am Projekt vollkommen unabhängig von älteren Projektversionen erfolgen. Kurz gesagt, werden alle Änderungen aus dem `testing`-Zweig vorübergehend rückgängig gemacht und Du hast die Möglichkeit, einen vollkommen neuen Weg in der Entwicklung einzuschlagen.

<!--Let’s make a few changes and commit again:-->

Lass uns ein paar Änderungen machen und mit einem Commit festhalten:

	$ vim test.rb
	$ git commit -a -m 'made other changes'

<!--Now your project history has diverged (see Figure 3-9). You created and switched to a branch, did some work on it, and then switched back to your main branch and did other work. Both of those changes are isolated in separate branches: you can switch back and forth between the branches and merge them together when you’re ready. And you did all that with simple `branch` and `checkout` commands.-->

Nun verzweigen sich die Projektverläufe (siehe Abbildung 3-9). Du hast einen Branch erstellt und zu ihm gewechselt, hast ein bisschen gearbeitet, bist zu Deinem Haupt-Zweig zurückgekehrt und hast da was ganz anderes gemacht. Beide Arbeiten existieren vollständig unabhängig voneinander in zwei unterschiedlichen Branches. Du kannst beliebig zwischen den beiden Zweigen wechseln und sie zusammenführen, wenn Du meinst, es wäre soweit. Und das alles hast Du mit simplen `branch` und `checkout`-Anweisungen vollbracht.

<!--Figure 3-9. The branch histories have diverged.-->

Insert 18333fig0309.png
Abbildung 3-9. Die Historie läuft auseinander.

<!--Because a branch in Git is in actuality a simple file that contains the 40 character SHA-1 checksum of the commit it points to, branches are cheap to create and destroy. Creating a new branch is as quick and simple as writing 41 bytes to a file (40 characters and a newline).-->

Branches können in Git spielend erstellt und entfernt werden, da sie nur kleine Dateien sind, die eine 40 Zeichen lange SHA-1 Prüfsumme der Commits enthalten, auf die sie verweisen. Einen neuen Zweig zu erstellen, erzeugt ebenso viel Aufwand wie das Schreiben einer 41 Byte großen Datei (40 Zeichen und einen Zeilenumbruch).

<!--This is in sharp contrast to the way most VCS tools branch, which involves copying all of the project’s files into a second directory. This can take several seconds or even minutes, depending on the size of the project, whereas in Git the process is always instantaneous. Also, because we’re recording the parents when we commit, finding a proper merge base for merging is automatically done for us and is generally very easy to do. These features help encourage developers to create and use branches often.-->

Das steht im krassen Gegensatz zu dem Weg, den die meisten andere VCS Tools beim Thema Branching einschlagen. Diese kopieren oftmals jeden neuen Entwicklungszweig in ein weiteres Verzeichnis, was – je nach Projektgröße – mehrere Minuten in Anspruch nehmen kann, wohingegen Git diese Aufgabe sofort erledigt. Da wir außerdem immer den Ursprungs-Commit festhalten, lässt sich problemlos eine gemeinsame Basis für eine Zusammenführung finden und umsetzen. Diese Eigenschaft soll Entwickler ermutigen Entwicklungszweige häufig zu erstellen und zu nutzen.

<!--Let’s see why you should do so.-->

Lass uns mal sehen, warum Du das machen solltest.

<!--## Basic Branching and Merging ##-->
## Einfaches Branching und Merging ##

<!--Let’s go through a simple example of branching and merging with a workflow that you might use in the real world. You’ll follow these steps:-->

Lass uns das Ganze an einem Beispiel durchgehen, dessen Workflow zum Thema Branching und Zusammenführen Du im echten Leben verwenden kannst. Folge einfach diesen Schritten:

<!--1. Do work on a web site.-->
<!--2. Create a branch for a new story you’re working on.-->
<!--3. Do some work in that branch.-->

1. Arbeite an einer Webseite.
2. Erstelle einen Branch für irgendeine neue Geschichte, an der Du arbeitest.
3. Arbeite in dem Branch.

<!--At this stage, you’ll receive a call that another issue is critical and you need a hotfix. You’ll do the following:-->

In diesem Augenblick kommt ein Anruf, dass ein kritisches Problem aufgetreten ist und sofort gelöst werden muss. Du machst folgendes:

<!--1. Switch back to your production branch.-->
<!--2. Create a branch to add the hotfix.-->
<!--3. After it’s tested, merge the hotfix branch, and push to production.-->
<!--4. Switch back to your original story and continue working.-->

1. Schalte zurück zu Deinem „Produktiv“-Zweig.
2. Erstelle einen Branch für den Hotfix.
3. Nach dem Testen führst Du den Hotfix-Branch mit dem „Produktiv“-Branch zusammen.
4. Schalte wieder auf Deine alte Arbeit zurück und werkel weiter.

<!--### Basic Branching ###-->
### Branching Grundlagen ###

<!--First, let’s say you’re working on your project and have a couple of commits already (see Figure 3-10).-->

Sagen wir, Du arbeitest an Deinem Projekt und hast bereits einige Commits durchgeführt (siehe Abbildung 3-10).

<!--Figure 3-10. A short and simple commit history.-->

Insert 18333fig0310.png
Abbildung 3-10. Eine kurze, einfache Commit-Historie

<!--You’ve decided that you’re going to work on issue #53 in whatever issue-tracking system your company uses. To be clear, Git isn’t tied into any particular issue-tracking system; but because issue #53 is a focused topic that you want to work on, you’ll create a new branch in which to work. To create a branch and switch to it at the same time, you can run the `git checkout` command with the `-b` switch:-->

Du hast Dich dafür entschieden, am Issue #53 des Issue-Trackers XY zu arbeiten. Um eines klarzustellen, Git ist an kein Issue-Tracking-System gebunden. Da der Issue #53 allerdings ein Schwerpunktthema betrifft, wirst Du einen neuen Branch erstellen, um daran zu arbeiten. Um in einem Arbeitsschritt einen neuen Branch zu erstellen und zu aktivieren, kannst Du die Anweisung `git checkout` mit der Option `-b` verwenden:

	$ git checkout -b iss53
	Switched to a new branch 'iss53'

<!--This is shorthand for:-->

Das ist die Kurzform von

	$ git branch iss53
	$ git checkout iss53

<!--Figure 3-11 illustrates the result.-->

Abbildung 3-11 verdeutlicht das Ergebnis.

<!--Figure 3-11. Creating a new branch pointer.-->

Insert 18333fig0311.png
Abbildung 3-11. Erstellung eines neuen Branch-Zeigers

<!--You work on your web site and do some commits. Doing so moves the `iss53` branch forward, because you have it checked out (that is, your HEAD is pointing to it; see Figure 3-12):-->

Du arbeitest an Deiner Web-Seite und machst ein paar Commits. Das bewegt den `iss53`-Branch vorwärts, da Du ihn ausgebucht hast (das heißt, dass Dein HEAD-Zeiger darauf verweist; siehe Abbildung 3-12):

	$ vim index.html
	$ git commit -a -m 'added a new footer [issue 53]'

<!--Figure 3-12. The iss53 branch has moved forward with your work.-->

Insert 18333fig0312.png
Abbildung 3-12. Der `iss53`-Branch hat mit Deiner Arbeit Schritt gehalten.

<!--Now you get the call that there is an issue with the web site, and you need to fix it immediately. With Git, you don’t have to deploy your fix along with the `iss53` changes you’ve made, and you don’t have to put a lot of effort into reverting those changes before you can work on applying your fix to what is in production. All you have to do is switch back to your master branch.-->

Nun bekommst Du einen Anruf, in dem Dir mitgeteilt wird, dass es ein Problem mit der Internet-Seite gibt, das Du umgehend beheben sollst. Bei Git musst Du Deine Fehlerkorrektur nicht zusammen mit den `iss53`-Änderungen einbringen. Und Du musst keine Zeit damit verschwenden, Deine bisherigen Änderungen rückgängig zu machen, bevor Du mit der Fehlerbehebung an der Produktionsumgebung beginnen kannst. Alles, was Du tun musst, ist, zu Deinem master-Branch wechseln.

<!--However, before you do that, note that if your working directory or staging area has uncommitted changes that conflict with the branch you’re checking out, Git won’t let you switch branches. It’s best to have a clean working state when you switch branches. There are ways to get around this (namely, stashing and commit amending) that we’ll cover later. For now, you’ve committed all your changes, so you can switch back to your master branch:-->

Beachte jedoch, dass Dich Git den Branch nur wechseln lässt, wenn bisherige Änderungen in Deinem Arbeitsverzeichnis oder Deiner Staging-Area nicht in Konflikt mit dem Zweig stehen, zu dem Du nun wechseln möchtest. Am besten ist es, wenn ein sauberer Arbeitsstand vorliegt, wenn man den Branch wechselt. Wir werden uns später mit Wegen befassen, dieses Verhalten zu umgehen (namentlich „Stashing“ und „Commit Ammending“). Vorerst aber hast Du Deine Änderungen bereits comitted, sodass Du zu Deinem master-Branch zurückwechseln kannst.

	$ git checkout master
	Switched to branch 'master'

<!--At this point, your project working directory is exactly the way it was before you started working on issue #53, and you can concentrate on your hotfix. This is an important point to remember: Git resets your working directory to look like the snapshot of the commit that the branch you check out points to. It adds, removes, and modifies files automatically to make sure your working copy is what the branch looked like on your last commit to it.-->

Zu diesem Zeitpunkt befindet sich das Arbeitsverzeichnis des Projektes in exakt dem gleichen Zustand, in dem es sich befand, als Du mit der Arbeit an Issue #53 begonnen hast und Du kannst Dich direkt auf Deinen Hotfix konzentrieren. Dies ist ein wichtiger Moment, um sich vor Augen zu halten, dass Git Dein Arbeitsverzeichnis auf den Zustand des Commits, auf den dieser Branch zeigt, zurücksetzt. Es erstellt, entfernt und verändert Dateien automatisch, um sicherzustellen, dass Deine Arbeitskopie haargenau so aussieht wie der Zweig nach Deinem letzten Commit.

<!--Next, you have a hotfix to make. Let’s create a hotfix branch on which to work until it’s completed (see Figure 3-13):-->

Nun hast Du einen Hotfix zu erstellen. Lass uns dazu einen hotfix-Branch erstellen, an dem Du bis zu dessen Fertigstellung arbeitest (siehe Abbildung 3-13):

	$ git checkout -b hotfix
	Switched to a new branch 'hotfix'
	$ vim index.html
	$ git commit -a -m 'fixed the broken email address'
	[hotfix 3a0874c] fixed the broken email address
	 1 files changed, 1 deletion(-)

<!--Figure 3-13. hotfix branch based back at your master branch point.-->

Insert 18333fig0313.png
Abbildung 3-13. Der hotfix-Branch basiert auf dem zurückliegenden master-Branch.

<!--You can run your tests, make sure the hotfix is what you want, and merge it back into your master branch to deploy to production. You do this with the `git merge` command:-->

Du kannst Deine Tests durchführen und sicherstellen, dass der Hotfix seinen Zweck erfüllt und ihn dann mit dem master-Branch zusammenführen, um ihn in die Produktionsumgebung zu integrieren. Das machst Du mit der Anweisung `git merge`:

	$ git checkout master
	$ git merge hotfix
	Updating f42c576..3a0874c
	Fast-forward
	 README | 1 -
	 1 file changed, 1 deletion(-)

<!--You’ll notice the phrase "Fast forward" in that merge. Because the commit pointed to by the branch you merged in was directly upstream of the commit you’re on, Git moves the pointer forward. To phrase that another way, when you try to merge one commit with a commit that can be reached by following the first commit’s history, Git simplifies things by moving the pointer forward because there is no divergent work to merge together — this is called a "fast forward".-->

Du wirst die Mitteilung „Fast Forward“ während des Zusammenführens bemerken. Da der neue Commit direkt von dem ursprünglichen Commit, auf den sich der nun eingebrachte Zweig bezieht, abstammt, bewegt Git einfach den Zeiger weiter. Mit anderen Worten kann Git den neuen Commit durch Verfolgen der Commitabfolge direkt erreichen, dann bewegt es ausschließlich den Branch-Zeiger. Zu einer tatsächlichen Kombination der Commits besteht ja kein Anlass. Dieses Vorgehen wird „Fast Forward“ genannt.

<!--Your change is now in the snapshot of the commit pointed to by the `master` branch, and you can deploy your change (see Figure 3-14).-->

Deine Modifikationen befinden sich nun als Schnappschuss in dem Commit, auf den der `master`-Branch zeigt, diese lassen sich nun veröffentlichen (siehe Abbildung 3-14).

<!--Figure 3-14. Your master branch points to the same place as your hotfix branch after the merge.-->

Insert 18333fig0314.png
Abbildung 3-14. Der master-Branch zeigt nach der Zusammenführung auf den gleichen Commit wie der hotfix-Branch.

<!--After your super-important fix is deployed, you’re ready to switch back to the work you were doing before you were interrupted. However, first you’ll delete the `hotfix` branch, because you no longer need it — the `master` branch points at the same place. You can delete it with the `-d` option to `git branch`:-->

Nachdem Dein superwichtiger Hotfix veröffentlicht wurde, kannst Du Dich wieder Deiner ursprünglichen Arbeit zuwenden. Vorher wird sich allerdings des nun nutzlosen hotfix-Zweiges entledigt, schließlich zeigt der master-Branch ebenfalls auf die aktuelle Version. Du kannst ihn mit der `-d`-Option von `git branch` entfernen:

	$ git branch -d hotfix
	Deleted branch hotfix (was 3a0874c).

<!--Now you can switch back to your work-in-progress branch on issue #53 and continue working on it (see Figure 3-15):-->

Nun kannst Du zu Deinem Issue #53-Branch zurückwechseln und mit Deiner Arbeit fortfahren (Abbildung 3-15):

	$ git checkout iss53
	Switched to branch 'iss53'
	$ vim index.html
	$ git commit -a -m 'finished the new footer [issue 53]'
	[iss53 ad82d7a] finished the new footer [issue 53]
	 1 file changed, 1 insertion(+)

<!--Figure 3-15. Your iss53 branch can move forward independently.-->

Insert 18333fig0315.png
Abbildung 3-15. Dein `iss53`-Branch kann sich unabhängig weiterentwickeln.

<!--It’s worth noting here that the work you did in your `hotfix` branch is not contained in the files in your `iss53` branch. If you need to pull it in, you can merge your `master` branch into your `iss53` branch by running `git merge master`, or you can wait to integrate those changes until you decide to pull the `iss53` branch back into `master` later.-->

An dieser Stelle ist anzumerken, dass die Änderungen an dem `hotfix`-Branch nicht in Deinen `iss53`-Branch eingeflossen sind. Falls nötig, kannst Du den `master`-Branch allerdings mit der Anweisung `git merge master` mit Deinem Branch kombinieren. Oder Du wartest, bis Du den `iss53`-Branch später in den `master`-Branch zurückführst.

<!--### Basic Merging ###-->

### Die Grundlagen des Zusammenführens (Mergen) ###

<!--Suppose you’ve decided that your issue #53 work is complete and ready to be merged into your `master` branch. In order to do that, you’ll merge in your `iss53` branch, much like you merged in your `hotfix` branch earlier. All you have to do is check out the branch you wish to merge into and then run the `git merge` command:-->

Angenommen, Du hast entschieden, dass Deine Arbeiten an issue #53 abgeschlossen sind und das Ganze soweit ist, dass es mit dem `master` Branch zusammengeführt werden kann. Um das zu erreichen, wirst Du Deinen `iss53` Branch in den `master` Branch einfließen lassen, genauso wie Du es mit dem `hotfix` Branch zuvor getan hast. Du musst nur mit der Anweisung `checkout` zum dem Branch zu wechseln, in welchen Du etwas einfließen lassen möchtest und dann die Anweisung `git merge` ausführen:

	$ git checkout master
	$ git merge iss53
	Auto-merging README
	Merge made by the 'recursive' strategy.
	 README | 1 +
	 1 file changed, 1 insertion(+)

<!--This looks a bit different than the `hotfix` merge you did earlier. In this case, your development history has diverged from some older point. Because the commit on the branch you’re on isn’t a direct ancestor of the branch you’re merging in, Git has to do some work. In this case, Git does a simple three-way merge, using the two snapshots pointed to by the branch tips and the common ancestor of the two. Figure 3-16 highlights the three snapshots that Git uses to do its merge in this case.-->

Das sieht ein bisschen anders aus als das `hotfix merge` von vorhin. Hier haben sich die Entwicklungsstränge schon zu einem früheren Zeitpunkt geteilt. Da der `commit` auf dem Branch, in dem Du Dich befindest, kein direkter Nachfolger von dem Branch ist, in den Du das `merge` machst, hat Git einiges zu tun. Dazu macht Git einen 3-Wege `merge`, wobei es die beiden Schnappschüsse verwendet, welche auf die Enden der Branches zeigen, und den gemeinsamen Vorfahren dieser beiden. Abbildung 3-16 zeigt die drei `snapshots`, die Git in diesem Fall für das `merge` verwendet.

<!--Figure 3-16. Git automatically identifies the best common-ancestor merge base for branch merging.-->

Insert 18333fig0316.png
Abbildung 3-16. Git erkennt automatisch den am besten geeigneten gemeinsamen Vorgänger für die Branchzusammenführung.

<!--Instead of just moving the branch pointer forward, Git creates a new snapshot that results from this three-way merge and automatically creates a new commit that points to it (see Figure 3-17). This is referred to as a merge commit and is special in that it has more than one parent.-->

Anstatt einfach den Zeiger vorwärts zu bewegen, erstellt Git einen neuen `snapshot`, der aus dem 3-Wege 'merge' resultiert und erzeugt automatisch einen neuen `commit`, der darauf verweist (siehe Abbildung 3-17). Dies wird auch als `merge commit` bezeichnet und ist ein Spezialfall, weil es mehr als nur einen Elternteil hat.

<!--It’s worth pointing out that Git determines the best common ancestor to use for its merge base; this is different than CVS or Subversion (before version 1.5), where the developer doing the merge has to figure out the best merge base for themselves. This makes merging a heck of a lot easier in Git than in these other systems.-->

Es ist wichtig herauszustellen, dass Git den am besten geeigneten gemeinsamen Vorgänger als Grundlage für das Zusammenführen bestimmt, denn hierin unterscheidet es sich von CVS und Subversion (vor Version 1.5), wo der Entwickler beim Zusammenführen die 'merge' Basis selbst ermitteln muss. In Git ist das Zusammenführen dadurch wesentlich einfacher, als in anderen Systemen.

<!--Figure 3-17. Git automatically creates a new commit object that contains the merged work.-->

Insert 18333fig0317.png
Abbildung 3-17. Git erstellt automatisch ein 'commit', dass die zusammengeführte Arbeit enthält.

<!--Now that your work is merged in, you have no further need for the `iss53` branch. You can delete it and then manually close the ticket in your ticket-tracking system:-->

Jetzt, da wir die Arbeit zusammengeführt haben, ist der `iss53`-Branch nicht mehr notwendig. Du kannst ihn löschen und das Ticket im Ticket-Tracking-System schließen.

	$ git branch -d iss53

<!--### Basic Merge Conflicts ###-->
### Grundlegende Merge-Konflikte ###

<!--Occasionally, this process doesn’t go smoothly. If you changed the same part of the same file differently in the two branches you’re merging together, Git won’t be able to merge them cleanly. If your fix for issue #53 modified the same part of a file as the `hotfix`, you’ll get a merge conflict that looks something like this:-->

Gelegentlich verläuft der Prozess nicht ganz so glatt. Wenn Du an den selben Stellen in den selben Dateien unterschiedlicher Branches etwas geändert hast, kann Git diese nicht sauber zusammenführen. Wenn Dein Fix an 'issue #53' die selbe Stelle in einer Datei verändert hat, die Du auch mit `hotfix` angefasst hast, wirst Du einen 'merge'-Konflikt erhalten, der ungefähr so aussehen könnte:

	$ git merge iss53
	Auto-merging index.html
	CONFLICT (content): Merge conflict in index.html
	Automatic merge failed; fix conflicts and then commit the result.

<!--Git hasn’t automatically created a new merge commit. It has paused the process while you resolve the conflict. If you want to see which files are unmerged at any point after a merge conflict, you can run `git status`:-->

Git hat hier keinen 'merge commit' erstellt. Es hat den Prozess gestoppt, damit Du den Konflikt beseitigen kannst. Wenn Du sehen willst, welche Dateien 'unmerged' aufgrund eines 'merge' Konflikts sind, benutze einfach `git status`:

	$ git status
	On branch master
	You have unmerged paths.
	  (fix conflicts and run "git commit")

	Unmerged paths:
	  (use "git add <file>..." to mark resolution)

	        both modified:      index.html

	no changes added to commit (use "git add" and/or "git commit -a")

<!--Anything that has merge conflicts and hasn’t been resolved is listed as unmerged. Git adds standard conflict-resolution markers to the files that have conflicts, so you can open them manually and resolve those conflicts. Your file contains a section that looks something like this:-->

Alles, was einen 'merge' Konflikt aufweist und nicht gelöst werden konnte, wird als 'unmerged' aufgeführt. Git fügt den betroffenen Dateien Standard-Konfliktlösungsmarker hinzu, sodass Du diese öffnen und den Konflikt manuell lösen kannst. Deine Datei enthält einen Bereich, der so aussehen könnte:

	<<<<<<< HEAD
	<div id="footer">contact : email.support@github.com</div>
	=======
	<div id="footer">
	  please contact us at support@github.com
	</div>
	>>>>>>> iss53

<!--This means the version in HEAD (your master branch, because that was what you had checked out when you ran your merge command) is the top part of that block (everything above the `=======`), while the version in your `iss53` branch looks like everything in the bottom part. In order to resolve the conflict, you have to either choose one side or the other or merge the contents yourself. For instance, you might resolve this conflict by replacing the entire block with this:-->

Das heißt, die Version in HEAD (Deines 'master'-Branches, denn der wurde per 'checkout' aktiviert, als Du das 'merge' gemacht hast) ist der obere Teil des Blocks (alles oberhalb von '======='), und die Version aus dem `iss53`-Branch sieht wie der darunter befindliche Teil aus. Um den Konflikt zu lösen, musst Du Dich entweder für einen der beiden Teile entscheiden oder Du ersetzt den Teil komplett:

	<div id="footer">
	please contact us at email.support@github.com
	</div>

<!--This resolution has a little of each section, and I’ve fully removed the `<<<<<<<`, `=======`, and `>>>>>>>` lines. After you’ve resolved each of these sections in each conflicted file, run `git add` on each file to mark it as resolved. Staging the file marks it as resolved in Git.-->
<!--If you want to use a graphical tool to resolve these issues, you can run `git mergetool`, which fires up an appropriate visual merge tool and walks you through the conflicts:-->

Diese Lösung hat von beiden Teilen etwas und ich habe die Zeilen mit `<<<<<<<`, `=======`, und `>>>>>>>` komplett gelöscht. Nachdem Du alle problematischen Bereiche in allen von dem Konflikt betroffenen Dateien beseitigt hast, führe einfach `git add` für alle betroffenen Dateien aus und markiere sie damit als bereinigt. Dieses 'staging' der Dateien markiert sie für Git als bereinigt.
Wenn Du ein grafisches Tool zur Bereinigung benutzen willst, dann verwende `git mergetool`, welches ein passendes grafisches 'merge'-Tool startet und Dich durch die Konfliktbereiche führt:

	$ git mergetool

	This message is displayed because 'merge.tool' is not configured.
	See 'git mergetool --tool-help' or 'git help config' for more details.
	'git mergetool' will now attempt to use one of the following tools:
	opendiff kdiff3 tkdiff xxdiff meld tortoisemerge gvimdiff diffuse diffmerge ecmerge p4merge araxis bc3 codecompare vimdiff emerge
	Merging:
	index.html

	Normal merge conflict for 'index.html':
	  {local}: modified file
	  {remote}: modified file
	Hit return to start merge resolution tool (opendiff):

<!--If you want to use a merge tool other than the default (Git chose `opendiff` for me in this case because I ran the command on a Mac), you can see all the supported tools listed at the top after “... one of the following tools:”. Type the name of the tool you’d rather use. In Chapter 7, we’ll discuss how you can change this default value for your environment.-->

Wenn Du ein anderes Tool anstelle des Standardwerkzeugs für ein 'merge' verwenden möchtest (Git verwendet in meinem Fall `opendiff`, da ich auf einem Mac arbeite), dann kannst Du alle unterstützten Werkzeuge oben – unter „one of the following tools“ – aufgelistet sehen. Tippe einfach den Namen Deines gewünschten Werkzeugs ein. In Kapitel 7 besprechen wir, wie Du diesen Standardwert in Deiner Umgebung dauerhaft ändern kannst.

<!--After you exit the merge tool, Git asks you if the merge was successful. If you tell the script that it was, it stages the file to mark it as resolved for you.-->

Wenn Du das 'merge' Werkzeug beendest, fragt Dich Git, ob das Zusammenführen erfolgreich war. Wenn Du mit 'Ja' antwortest, wird das Skript diese Dateien als gelöst markieren.

<!--You can run `git status` again to verify that all conflicts have been resolved:-->

Du kannst `git status` erneut ausführen, um zu sehen, ob alle Konflikte gelöst sind:

	$ git status
	On branch master
	Changes to be committed:
	  (use "git reset HEAD <file>..." to unstage)

	        modified:   index.html


<!--If you’re happy with that, and you verify that everything that had conflicts has been staged, you can type `git commit` to finalize the merge commit. The commit message by default looks something like this:-->

Wenn Du zufrieden bist und Du geprüft hast, dass alle Konflikte beseitigt wurden, kannst Du `git commit` ausführen, um den 'merge commit' abzuschließen. Die Standardbeschreibung für diese Art 'commit' sieht wie folgt aus:

	Merge branch 'iss53'

	Conflicts:
	  index.html
	#
	# It looks like you may be committing a merge.
	# If this is not correct, please remove the file
	#       .git/MERGE_HEAD
	# and try again.
	#

<!--You can modify that message with details about how you resolved the merge if you think it would be helpful to others looking at this merge in the future — why you did what you did, if it’s not obvious.-->

Wenn Du glaubst, für zukünftige Betrachter des Commits könnte es hilfreich sein zu erklären, warum Du getan hast, was Du getan hast, dann kannst Du der Commit-Beschreibung noch zusätzliche Informationen hinzufügen, falls es nicht offensichtlich ist.

<!--## Branch Management ##-->
## Branch Management ##

<!--Now that you’ve created, merged, and deleted some branches, let’s look at some branch-management tools that will come in handy when you begin using branches all the time.-->

Du weißt jetzt, wie Du Branches erstellen, zusammenfügen und löschen kannst. Wir schauen uns jetzt noch ein paar recht nützliche Tools für die Arbeit mit Branches an.

<!--The `git branch` command does more than just create and delete branches. If you run it with no arguments, you get a simple listing of your current branches:-->

Die Anweisung `git branch` kann mehr, als nur Branches zu erstellen oder zu löschen. Wenn Du es ohne weitere Argumente ausführst, wird es Dir eine Liste mit Deinen aktuellen Branches anzeigen:

	$ git branch
	  iss53
	* master
	  testing

<!--Notice the `*` character that prefixes the `master` branch: it indicates the branch that you currently have checked out. This means that if you commit at this point, the `master` branch will be moved forward with your new work. To see the last commit on each branch, you can run `git branch -v`:-->

Das `*` vor dem `master`-Branch bedeutet, dass dies der gerade ausgecheckte Branch ist. Wenn Du also jetzt einen Commit erzeugst, wird dieser in den `master`-Branch gehen. Du kannst Dir mit `git branch -v` ganz schnell für jeden Branch den letzten Commit anzeigen lassen:

	$ git branch -v
	  iss53   93b412c fix javascript issue
	* master  7a98805 Merge branch 'iss53'
	  testing 782fd34 add scott to the author list in the readmes

<!--Another useful option to figure out what state your branches are in is to filter this list to branches that you have or have not yet merged into the branch you’re currently on. There are useful `-\-merged` and `-\-no-merged` options available in Git for this purpose. To see which branches are already merged into the branch you’re on, you can run `git branch -\-merged`:-->

Mit einer weiteren nützlichen Option kannst Du herausfinden, in welchem Zustand Deine Branches sind: welche der Branches wurden bereits in den aktuellen Branch gemergt und welche wurden es nicht. Für diesen Zweck gibt es in Git die Optionen `--merged` und `--no-merged`. Um herauszufinden, welche Branches schon in den aktuell ausgecheckten gemergt wurden, kannst Du einfach `git branch --merged` aufrufen:

	$ git branch --merged
	  iss53
	* master

<!--Because you already merged in `iss53` earlier, you see it in your list. Branches on this list without the `*` in front of them are generally fine to delete with `git branch -d`; you’ve already incorporated their work into another branch, so you’re not going to lose anything.-->

Da Du den Branch `iss53` schon gemergt hast, siehst Du ihn in dieser Liste. Alle Branches in dieser Liste, welchen kein `*` voransteht, können ohne Datenverlust mit `git branch -d` gelöscht werden, da sie ja bereits gemergt wurden.

<!--To see all the branches that contain work you haven’t yet merged in, you can run `git branch -\-no-merged`:-->

Um alle Branches zu sehen, welche noch nicht gemergte Änderungen enthalten, kannst Du `git branch --no-merged` aufrufen:

	$ git branch --no-merged
	  testing

<!--This shows your other branch. Because it contains work that isn’t merged in yet, trying to delete it with `git branch -d` will fail:-->

Die Liste zeigt Dir den anderen Branch. Er enthält Arbeit, die noch nicht gemergt wurde. Der Versuch, den Branch mit `git branch -d` zu löschen schlägt fehl:

	$ git branch -d testing
	error: The branch 'testing' is not fully merged.
	If you are sure you want to delete it, run 'git branch -D testing'.

<!--If you really do want to delete the branch and lose that work, you can force it with `-D`, as the helpful message points out.-->

Wenn Du den Branch trotzdem löschen und damit alle Änderungen dieses Branches verlieren willst, kannst Du das mit `git branch -D testing` machen.

<!--## Branching Workflows ##-->
## Branching Workflows ##

<!--Now that you have the basics of branching and merging down, what can or should you do with them? In this section, we’ll cover some common workflows that this lightweight branching makes possible, so you can decide if you would like to incorporate it into your own development cycle.-->

Jetzt, da Du die Grundlagen von 'branching' und 'merging' kennst, fragst Du Dich sicher, was Du damit anfangen kannst. In diesem Abschnitt werden wir uns typische Workflows anschauen, die dieses leichtgewichtige 'branching' möglich macht. Und Du kannst dann entscheiden, ob Du es in Deinem eigenen Entwicklungszyklus verwenden willst.

<!--### Long-Running Branches ###-->
### Langfristige Branches ###

<!--Because Git uses a simple three-way merge, merging from one branch into another multiple times over a long period is generally easy to do. This means you can have several branches that are always open and that you use for different stages of your development cycle; you can merge regularly from some of them into others.-->

Da Git das einfache 3-Wege-'merge' verwendet, ist häufiges Zusammenführen von einem Branch in einen anderen über einen langen Zeitraum generell einfach zu bewerkstelligen. Das heißt, Du kannst mehrere Branches haben, die immer offen sind und die Du für unterschiedliche Stadien Deines Entwicklungszyklus verwendest, und Du kannst regelmäßig welche von diesen mit anderen zusammenführen.

<!--Many Git developers have a workflow that embraces this approach, such as having only code that is entirely stable in their `master` branch — possibly only code that has been or will be released. They have another parallel branch named develop or next that they work from or use to test stability — it isn’t necessarily always stable, but whenever it gets to a stable state, it can be merged into `master`. It’s used to pull in topic branches (short-lived branches, like your earlier `iss53` branch) when they’re ready, to make sure they pass all the tests and don’t introduce bugs.-->

Viele Git Entwickler verfolgen mit ihrem Workflow den Ansatz, nur den stabilen Code in dem `master`-Branch zu halten – möglicherweise auch nur Code, der released wurde oder werden kann. Sie betreiben parallel einen anderen Branch zum Arbeiten oder Testen. Wenn dieser parallele Zweig einen stabilen Status erreicht, kann er mit dem `master`-Branch zusammengeführt werden. Dies findet bei themenbezogenen Branches (kurzfristigen Branches, wie der zuvor genannte `iss53`-Branch) Anwendung, um sicherzustellen, dass dieser die Tests besteht und keine Fehler verursacht.

<!--In reality, we’re talking about pointers moving up the line of commits you’re making. The stable branches are farther down the line in your commit history, and the bleeding-edge branches are farther up the history (see Figure 3-18).-->

Eigentlich reden wir gerade über Zeiger, die in der Reihe Deiner Commits aufsteigen. Die stabilen Branches liegen weiter unten und die bleeding-edge Branches weiter oben in diesem Verlauf (siehe Abbildung 3-18).

<!--Figure 3-18. More stable branches are generally farther down the commit history.-->

Insert 18333fig0318.png
Abbildung 3-18. Stabilere Branches sind generell weiter unten im Entwicklungsverlauf.

<!--It’s generally easier to think about them as work silos, where sets of commits graduate to a more stable silo when they’re fully tested (see Figure 3-19).-->

Es ist leichter, sich die verschiedenen Branches als Arbeitsdepots vorzustellen, in denen Sätze von Commits in stabilere Depots aufsteigen, sobald sie ausreichend getestet wurden (siehe Abbildung 3-19).

<!--Figure 3-19. It may be helpful to think of your branches as silos.-->

Insert 18333fig0319.png
Abbildung 3-19. Es könnte hilfreich sein, sich die Branches als Depots vorzustellen.

<!--You can keep doing this for several levels of stability. Some larger projects also have a `proposed` or `pu` (proposed updates) branch that has integrated branches that may not be ready to go into the `next` or `master` branch. The idea is that your branches are at various levels of stability; when they reach a more stable level, they’re merged into the branch above them.-->
<!--Again, having multiple long-running branches isn’t necessary, but it’s often helpful, especially when you’re dealing with very large or complex projects.-->

Das lässt sich für beliebig viele Stabilitätsabstufungen umsetzen. Manche größeren Projekte haben auch einen `proposed` (vorgeschlagenen) oder `pu` (proposed updates – vorgeschlagene Updates) Zweig mit Branches, die vielleicht noch nicht bereit sind, in den `next`- oder `master`-Branch integriert zu werden. Die Idee dahinter ist, dass Deine Branches verschiedene Stabilitätsabstufungen repräsentieren. Sobald sie eine stabilere Stufe erreichen, werden sie mit dem nächsthöheren Branch zusammengeführt.

Nochmal, langfristig verschiedene Branches parallel laufen zu lassen, ist nicht notwendig, aber oft hilfreich,  insbesondere wenn man es mit sehr großen oder komplexen Projekten zu tun hat.

<!--### Topic Branches ###-->
### Themen-Branches ###

<!--Topic branches, however, are useful in projects of any size. A topic branch is a short-lived branch that you create and use for a single particular feature or related work. This is something you’ve likely never done with a VCS before because it’s generally too expensive to create and merge branches. But in Git it’s common to create, work on, merge, and delete branches several times a day.-->

Themen-Branches sind in jedem Projekt nützlich, egal bei welcher Größe. Ein Themen-Branch ist ein kurzlebiger Zweig, der für eine spezielle Aufgabe oder ähnliche Arbeiten erstellt und benutzt wird. Das ist vielleicht etwas, was Du noch nie zuvor mit einem Versionierungssystem gemacht hast, weil es normalerweise zu aufwändig und mühsam ist, Branches zu erstellen und zusammenzuführen. Mit Git ist es allerdings vollkommen geläufig, mehrmals am Tag Branches zu erstellen, an ihnen zu arbeiten, sie zusammenzuführen und sie anschließend wieder zu löschen.

<!--You saw this in the last section with the `iss53` and `hotfix` branches you created. You did a few commits on them and deleted them directly after merging them into your main branch. This technique allows you to context-switch quickly and completely — because your work is separated into silos where all the changes in that branch have to do with that topic, it’s easier to see what has happened during code review and such. You can keep the changes there for minutes, days, or months, and merge them in when they’re ready, regardless of the order in which they were created or worked on.-->

Du hast das im letzten Abschnitt an den von Dir erstellten `iss53`- und `hotfix`-Branches gesehen. Du hast mehrere Commits auf sie angewendet und sie unmittelbar nach Zusammenführung mit Deinem Hauptzweig gelöscht. Diese Technik erlaubt es Dir, schnell und vollständig den Kontext zu wechseln. Da Deine Arbeit in verschiedene Depots aufgeteilt ist, in denen alle Änderungen unter die Thematik dieses Branches fallen, ist es leichter nachzuvollziehen, was bei Code-Überprüfungen und ähnlichem geschehen ist. Du kannst die Änderungen darin für Minuten, Tage oder Monate aufbewahren und sie einfließen lassen, wenn diese fertig sind, ungeachtet der Reihenfolge, in welcher diese erstellt oder bearbeitet wurden.

<!--Consider an example of doing some work (on `master`), branching off for an issue (`iss91`), working on it for a bit, branching off the second branch to try another way of handling the same thing (`iss91v2`), going back to your master branch and working there for a while, and then branching off there to do some work that you’re not sure is a good idea (`dumbidea` branch). Your commit history will look something like Figure 3-20.-->

Stell Dir vor, Du arbeitest ein bisschen (in `master`), erstellst mal eben einen Branch für einen Fehler (`iss91`), arbeitest an dem für eine Weile, erstellst einen zweiten Branch, um eine andere Problemlösung für den selben Fehler auszuprobieren (`iss91v2`), wechselst zurück zu Deinem master-Branch, arbeitest dort ein bisschen und machst dann einen neuen Branch für etwas, wovon Du nicht weißt, ob es eine gute Idee ist (`dumbidea`-Branch). Dein Commit-Verlauf wird wie in Abbildung 3-20 aussehen.

<!--Figure 3-20. Your commit history with multiple topic branches.-->

Insert 18333fig0320.png
Abbildung 3-20. Dein Commit-Verlauf mit verschiedenen Themen-Branches.

<!--Now, let’s say you decide you like the second solution to your issue best (`iss91v2`); and you showed the `dumbidea` branch to your coworkers, and it turns out to be genius. You can throw away the original `iss91` branch (losing commits C5 and C6) and merge in the other two. Your history then looks like Figure 3-21.-->

Nun, sagen wir, Du hast Dich entschieden, die zweite Lösung des Fehlers (`iss91v2`) zu bevorzugen, außerdem hast den `dumbidea`-Branch Deinen Mitarbeitern gezeigt und es hat sich herausgestellt, dass er genial ist. Du kannst also den ursprünglichen `iss91`-Branch (unter Verlust der Commits C5 und C6) wegwerfen und die anderen beiden einfließen lassen. Dein Verlauf sieht dann aus wie in Abbildung 3-21.

<!--Figure 3-21. Your history after merging in dumbidea and iss91v2.-->

Insert 18333fig0321.png
Abbildung 3-21. Dein Verlauf nach Zusammenführung von `dumbidea` und `iss91v2`.

<!--It’s important to remember when you’re doing all this that these branches are completely local. When you’re branching and merging, everything is being done only in your Git repository — no server communication is happening.-->

Es ist wichtig, sich daran zu erinnern, dass alle diese Branches nur lokal existieren. Wenn Du Verzweigungen schaffst (branchst) und wieder zusammenführst (mergest), findet dies nur in Deinem Git-Repository statt – es findet keine Server-Kommunikation statt.

<!--## Remote Branches ##-->
## Externe Branches ##

<!--Remote branches are references to the state of branches on your remote repositories. They’re local branches that you can’t move; they’re moved automatically whenever you do any network communication. Remote branches act as bookmarks to remind you where the branches on your remote repositories were the last time you connected to them.-->

Externe (Remote) Branches sind Referenzen auf den Zustand der Branches in Deinen externen Repositorys. Sie sind lokale Branches, die Du nicht verändern kannst, sie werden automatisch verändert, wann immer Du eine Netzwerkoperation durchführst. Externe Branches verhalten sich wie Lesezeichen, um Dich daran zu erinnern, an welcher Position sich die Branches in Deinen externen Repositorys befanden, als Du Dich zum letzten Mal mit ihnen verbunden hattest.

<!--They take the form `(remote)/(branch)`. For instance, if you wanted to see what the `master` branch on your `origin` remote looked like as of the last time you communicated with it, you would check the `origin/master` branch. If you were working on an issue with a partner and they pushed up an `iss53` branch, you might have your own local `iss53` branch; but the branch on the server would point to the commit at `origin/iss53`.-->

Externe Branches besitzen die Schreibweise `(Repository)/(Branch)`. Wenn Du beispielsweise wissen möchtest, wie der `master`-Branch in Deinem `origin`-Repository ausgesehen hat, als Du zuletzt Kontakt mit ihm hattest, dann würdest Du den `origin/master`-Branch überprüfen. Wenn Du mit einem Mitarbeiter an einer Fehlerbehebung gearbeitet hast und dieser bereits einen `iss53`-Branch hochgeladen hat, besitzt Du möglicherweise Deinen eigenen lokalen `iss53`-Branch. Der Branch auf dem Server würde allerdings auf den Commit von `origin/iss53` zeigen.

<!--This may be a bit confusing, so let’s look at an example. Let’s say you have a Git server on your network at `git.ourcompany.com`. If you clone from this, Git automatically names it `origin` for you, pulls down all its data, creates a pointer to where its `master` branch is, and names it `origin/master` locally; and you can’t move it. Git also gives you your own `master` branch starting at the same place as origin’s `master` branch, so you have something to work from (see Figure 3-22).-->

Das kann ein wenig verwirrend sein, lass uns also ein Besipiel betrachten. Nehmen wir an, Du hättest in Deinem Netzwerk einen Git-Server mit der Adresse `git.ourcompany.com`. Wenn Du von diesem klonst, nennt Git ihn automatisch `origin` für Dich, lädt all seine Daten herunter, erstellt einen Zeiger zur der Stelle, wo sein `master`-Branch ist und benennt es lokal `origin/master`; und er ist unveränderbar für Dich. Git gibt Dir auch einen eigenen `master`-Branch mit der gleichen Ausgangsposition wie origins `master`-Branch, damit Du einen Punkt für den Beginn Deiner Arbeiten hast (siehe Abbildung 3-22).

<!--Figure 3-22. A Git clone gives you your own master branch and origin/master pointing to origin’s master branch.-->

Insert 18333fig0322.png
Abbildung 3-22. Ein 'git clone' gibt Dir Deinen eigenen `master`-Branch und `origin/master`, welcher auf origins 'master'-Branch zeigt.

<!--If you do some work on your local master branch, and, in the meantime, someone else pushes to `git.ourcompany.com` and updates its master branch, then your histories move forward differently. Also, as long as you stay out of contact with your origin server, your `origin/master` pointer doesn’t move (see Figure 3-23).-->

Wenn Du ein wenig an Deinem lokalen `master`-Branch arbeitest und in der Zwischenzeit ein anderer etwas zu `git.ourcompany.com` herauflädt und damit den `master`-Branch auf dem externen Server aktualisiert, dann entwickeln sich Eure Arbeitsverläufe unterschiedlich. Außerdem bewegt sich Dein `origin/master`-Zeiger nicht, solange Du keinen Kontakt mit Deinem `origin`-Server aufnimmst (siehe Abbildung 3-23).

<!--Figure 3-23. Working locally and having someone push to your remote server makes each history move forward differently.-->

Insert 18333fig0323.png
Abbildung 3-23. Lokal zu arbeiten, während ein anderer auf Deinen externen Server hochlädt, führt zu unterschiedlichen Verläufen.

<!--To synchronize your work, you run a `git fetch origin` command. This command looks up which server origin is (in this case, it’s `git.ourcompany.com`), fetches any data from it that you don’t yet have, and updates your local database, moving your `origin/master` pointer to its new, more up-to-date position (see Figure 3-24).-->

Um Deine Arbeit abzugleichen, führe die Anweisung `git fetch origin` aus. Die Anweisung schlägt nach, welcher Server `orgin` ist (in diesem Fall `git.ourcompany.com`), holt alle Daten, die Dir bisher fehlen und aktualisiert Deine lokale Datenbank, indem es Deinen `orgin/master`-Zeiger auf seine neue aktuellere Position bewegt (siehe Abbildung 3-24).

<!--Figure 3-24. The git fetch command updates your remote references.-->

Insert 18333fig0324.png
Abbildung 3-24. Die Anweisung `git fetch` aktualisiert Deine externen Referenzen.

<!--To demonstrate having multiple remote servers and what remote branches for those remote projects look like, let’s assume you have another internal Git server that is used only for development by one of your sprint teams. This server is at `git.team1.ourcompany.com`. You can add it as a new remote reference to the project you’re currently working on by running the `git remote add` command as we covered in Chapter 2. Name this remote `teamone`, which will be your shortname for that whole URL (see Figure 3-25).-->

Um zu demonstrieren, wie Branches auf verschiedenen Remote-Servern aussehen, stellen wir uns vor, dass Du einen weiteren internen Git-Server besitzt, welcher nur von einem Deiner Sprint-Teams zur Entwicklung genutzt wird. Diesen Server erreichen wir unter `git.team1.ourcompany.com`. Du kannst ihn mit der Git-Anweisung `git remote add`, wie in Kapitel 2 beschrieben, Deinem derzeitigen Arbeitsprojekt als weiteren Quell-Server hinzufügen. Gib dem Remote-Server den Namen `teamone`, welcher nun als Synonym für die ausgeschriebene Internetadresse dient (siehe Abbildung 3-25).

<!--Figure 3-25. Adding another server as a remote.-->

Insert 18333fig0325.png
Abbildung 3-25. Einen weiteren Server als Quelle hinzufügen.

<!--Now, you can run `git fetch teamone` to fetch everything the remote `teamone` server has that you don’t have yet. Because that server has a subset of the data your `origin` server has right now, Git fetches no data but sets a remote branch called `teamone/master` to point to the commit that `teamone` has as its `master` branch (see Figure 3-26).-->

Nun kannst Du einfach `git fetch teamone` ausführen, um alles vom Server zu holen, was Du noch nicht hast. Da der Datenbestand auf dem Teamserver ein Teil der Informationen auf Deinem `origin`-Server ist, holt Git keine Daten, erstellt allerdings einen Remote-Branch namens `teamone/master`, der auf den gleichen Commit wie `teamone`s `master`-Branch zeigt (siehe Abbildung 3-26).

<!--Figure 3-26. You get a reference to teamone’s master branch position locally.-->

Insert 18333fig0326.png
Abbildung 3-26. Du bekommst eine lokale Referenz auf `teamone`s `master`-Branch.

<!--### Pushing ###-->
### Hochladen ###

<!--When you want to share a branch with the world, you need to push it up to a remote that you have write access to. Your local branches aren’t automatically synchronized to the remotes you write to — you have to explicitly push the branches you want to share. That way, you can use private branches for work you don’t want to share, and push up only the topic branches you want to collaborate on.-->

Wenn Du einen Branch mit der Welt teilen möchtest, musst Du ihn auf einen externen Server laden, auf dem Du Schreibrechte besitzt. Deine lokalen Zweige werden nicht automatisch mit den Remote-Servern synchronisiert, wenn Du etwas änderst – Du musst die zu veröffentlichenden Branches explizit hochladen (pushen). Auf diesem Weg kannst Du an privaten Zweigen arbeiten, die Du nicht veröffentlichen möchtest, und nur die Themen-Branches replizieren, an denen Du gemeinsam mit anderen entwickeln möchtest.

<!--If you have a branch named `serverfix` that you want to work on with others, you can push it up the same way you pushed your first branch. Run `git push (remote) (branch)`:-->

Wenn Du einen Zweig namens `serverfix` besitzt, an dem Du mit anderen arbeiten möchtest, dann kannst Du diesen auf dieselbe Weise hochladen wie Deinen ersten Branch. Führe `git push (remote) (branch)` aus:

	$ git push origin serverfix
	Counting objects: 20, done.
	Compressing objects: 100% (14/14), done.
	Writing objects: 100% (15/15), 1.74 KiB, done.
	Total 15 (delta 5), reused 0 (delta 0)
	To git@github.com:schacon/simplegit.git
	 * [new branch]      serverfix -> serverfix

<!--This is a bit of a shortcut. Git automatically expands the `serverfix` branchname out to `refs/heads/serverfix:refs/heads/serverfix`, which means, “Take my serverfix local branch and push it to update the remote’s serverfix branch.” We’ll go over the `refs/heads/` part in detail in Chapter 9, but you can generally leave it off. You can also do `git push origin serverfix:serverfix`, which does the same thing — it says, “Take my serverfix and make it the remote’s serverfix.” You can use this format to push a local branch into a remote branch that is named differently. If you didn’t want it to be called `serverfix` on the remote, you could instead run `git push origin serverfix:awesomebranch` to push your local `serverfix` branch to the `awesomebranch` branch on the remote project.-->

Hierbei handelt es sich um eine Abkürzung. Git erweitert die `serverfix`-Branchbezeichnung automatisch zu `refs/heads/serverfix:refs/heads/serverfix`, was soviel bedeutet wie “Nimm meinen lokalen `serverfix`-Branch und aktualisiere damit den `serverfix`-Branch auf meinem externen Server”. Wir werden den `refs/heads/`-Teil in Kapitel 9 noch näher beleuchten, Du kannst ihn aber in der Regel weglassen. Du kannst auch `git push origin serverfix:serverfix` ausführen, was das Gleiche bewirkt – es bedeutet “Nimm meinen `serverfix` und mach ihn zum externen `serverfix`”. Du kannst dieses Format auch benutzen, um einen lokalen Zweig in einen externen Branch mit anderem Namen zu laden. Wenn Du ihn auf dem externen Server nicht `serverfix` nennen möchtest, könntest Du stattdessen `git push origin serverfix:awesomebranch` ausführen, um Deinen lokalen `serverfix`-Branch in den `awesomebranch`-Zweig in Deinem externen Projekt zu laden.

<!--The next time one of your collaborators fetches from the server, they will get a reference to where the server’s version of `serverfix` is under the remote branch `origin/serverfix`:-->

Das nächste Mal, wenn einer Deiner Mitarbeiter den aktuellen Status des Git-Projektes vom Server abruft, wird er eine Referenz auf den externen Branch `origin/serverfix` unter dem Namen `serverfix` erhalten:

	$ git fetch origin
	remote: Counting objects: 20, done.
	remote: Compressing objects: 100% (14/14), done.
	remote: Total 15 (delta 5), reused 0 (delta 0)
	Unpacking objects: 100% (15/15), done.
	From git@github.com:schacon/simplegit
	 * [new branch]      serverfix    -> origin/serverfix

<!--It’s important to note that when you do a fetch that brings down new remote branches, you don’t automatically have local, editable copies of them. In other words, in this case, you don’t have a new `serverfix` branch — you only have an `origin/serverfix` pointer that you can’t modify.-->

Es ist wichtig festzuhalten, dass Du mit Abrufen eines neuen externen Branches nicht automatisch eine lokale, bearbeitbare Kopie desselben erhältst. Mit anderen Worten, in diesem Fall bekommst Du keinen neuen `serverfix`-Branch – sondern nur einen `origin/serverfix`-Zeiger, den Du nicht verändern kannst.

<!--To merge this work into your current working branch, you can run `git merge origin/serverfix`. If you want your own `serverfix` branch that you can work on, you can base it off your remote branch:-->

Um diese referenzierte Arbeit mit Deinem derzeitigen Arbeitsbranch zusammenzuführen, kannst Du `git merge origin/serverfix` ausführen. Wenn Du allerdings Deine eigene Arbeitskopie des `serverfix`-Branches erstellen möchtest, dann kannst Du diesen auf Grundlage des externen Zweiges erstellen:

	$ git checkout -b serverfix origin/serverfix
	Branch serverfix set up to track remote branch refs/remotes/origin/serverfix.
	Switched to a new branch "serverfix"

<!--This gives you a local branch that you can work on that starts where `origin/serverfix` is.-->

Dies erstellt Dir einen lokalen bearbeitbaren Branch mit der Grundlage des `origin/serverfix`-Zweiges.

<!--### Tracking Branches ###-->
### Tracking Branches ###

<!--Checking out a local branch from a remote branch automatically creates what is called a _tracking branch_. Tracking branches are local branches that have a direct relationship to a remote branch. If you’re on a tracking branch and type `git push`, Git automatically knows which server and branch to push to. Also, running `git pull` while on one of these branches fetches all the remote references and then automatically merges in the corresponding remote branch.-->

Das Auschecken eines lokalen Branches von einem Remote-Branch erzeugt automatisch einen sogenannten _Tracking-Branch_. Tracking Branches sind lokale Branches mit einer direkten Beziehung zu dem Remote-Zweig. Wenn Du Dich in einem Tracking-Branch befindest und `git push` eingibst, weiß Git automatisch, zu welchem Server und Repository es pushen soll. Ebenso führt `git pull` in einem dieser Branches dazu, dass alle entfernten Referenzen gefetched und automatisch in den Zweig gemerged werden.

<!--When you clone a repository, it generally automatically creates a `master` branch that tracks `origin/master`. That’s why `git push` and `git pull` work out of the box with no other arguments. However, you can set up other tracking branches if you wish — ones that don’t track branches on `origin` and don’t track the `master` branch. The simple case is the example you just saw, running `git checkout -b [branch] [remotename]/[branch]`. If you have Git version 1.6.2 or later, you can also use the `-\-track` shorthand:-->

Wenn Du ein Repository klonst, wird automatisch ein `master`-Branch erzeugt, welcher `origin/master` verfolgt. Deshalb können `git push` und `git pull` ohne weitere Argumente aufgerufen werden. Du kannst natürlich auch eigene Tracking-Branches erzeugen – welche die nicht Zweige auf `origin` und dessen `master`-Branch verfolgen. Der einfachste Fall ist das bereits gesehene Beispiel, in welchem Du `git checkout -b [branch] [remotename]/[branch]` ausführst. Mit der Git-Version 1.6.2 oder später kannst Du auch die `--track`-Kurzvariante nutzen:

	$ git checkout --track origin/serverfix
	Branch serverfix set up to track remote branch serverfix from origin.
	Switched to a new branch 'serverfix'

<!--To set up a local branch with a different name than the remote branch, you can easily use the first version with a different local branch name:-->

Um einen lokalen Branch zu erzeugen mit einem anderen Namen als dem des Remote-Branches, kannst Du einfach die erste Variante mit einem neuen lokalen Branch-Namen verwenden:

	$ git checkout -b sf origin/serverfix
	Branch sf set up to track remote branch serverfix from origin.
	Switched to a new branch 'sf'

<!--Now, your local branch `sf` will automatically push to and pull from `origin/serverfix`.-->

Nun wird Dein lokaler Branch `sf` automatisch push und pull auf `origin/serverfix` durchführen.

<!--### Deleting Remote Branches ###-->
### Löschen entfernter Branches ###

<!--Suppose you’re done with a remote branch — say, you and your collaborators are finished with a feature and have merged it into your remote’s `master` branch (or whatever branch your stable codeline is in). You can delete a remote branch using the rather obtuse syntax `git push [remotename] :[branch]`. If you want to delete your `serverfix` branch from the server, you run the following:-->

Stellen wir uns vor, Du bist fertig mit Deinem Remote-Branch – sagen wir Deine Mitarbeiter und Du, Ihr seid fertig mit einer neuen Funktion und habt sie in den entfernten `master`-Branch (oder in welchem Zweig Ihr sonst den stabilen Code ablegt) gemerged. Dann kannst Du den Remote-Branch löschen, indem Du die recht stumpfsinnige Syntax `git push [remotename] :[branch]` benutzt. Wenn Du Deinen `serverfix`-Branch vom Server löschen möchtest, führe folgendes aus:

	$ git push origin :serverfix
	To git@github.com:schacon/simplegit.git
	 - [deleted]         serverfix

<!--Boom. No more branch on your server. You may want to dog-ear this page, because you’ll need that command, and you’ll likely forget the syntax. A way to remember this command is by recalling the `git push [remotename] [localbranch]:[remotebranch]` syntax that we went over a bit earlier. If you leave off the `[localbranch]` portion, then you’re basically saying, “Take nothing on my side and make it be `[remotebranch]`.”-->

Boom. Kein Zweig mehr auf Deinem Server. Du möchtest Dir diese Seite vielleicht markieren, weil Du diese Anweisung noch benötigen wirst und man leicht deren Syntax vergisst. Ein Weg, sich an diese Anweisung zu erinnern, führt über die `git push [remotename] [localbranch]:[remotebranch]`-Snytax, welche wir bereits behandelt haben. Wenn Du den `[localbranch]`-Teil weglässt, dann sagst Du einfach „Nimm nichts von meiner Seite und mach es zu `[remotebranch]`“.

<!--## Rebasing ##-->
## Rebasing ##

<!--In Git, there are two main ways to integrate changes from one branch into another: the `merge` and the `rebase`. In this section you’ll learn what rebasing is, how to do it, why it’s a pretty amazing tool, and in what cases you won’t want to use it.-->

Es gibt in Git zwei Wege, um Änderungen von einem Branch in einen anderen zu überführen: `merge` und `rebase`. In diesem Abschnitt wirst Du erfahren, was Rebasing ist, wie Du es anwendest, warum es ein verdammt abgefahrenes Werkzeug ist und bei welchen Gelegenheiten Du es besser nicht einsetzen solltest.

<!--### The Basic Rebase ###-->
### Der einfache Rebase ###

<!--If you go back to an earlier example from the Merge section (see Figure 3-27), you can see that you diverged your work and made commits on two different branches.-->

Wenn Du zu einem früheren Beispiel aus dem Merge-Kapitel zurückkehrst (siehe Abbildung 3-27), kannst Du sehen, dass Du Deine Arbeit aufgeteilt und Commits auf zwei unterschiedlichen Branches erstellt hast.

<!--Figure 3-27. Your initial diverged commit history.-->

Insert 18333fig0327.png
Abbildung 3-27. Deine initiale Commit-Historie zum Zeitpunkt der Aufteilung.

<!--The easiest way to integrate the branches, as we’ve already covered, is the `merge` command. It performs a three-way merge between the two latest branch snapshots (C3 and C4) and the most recent common ancestor of the two (C2), creating a new snapshot (and commit), as shown in Figure 3-28.-->

Der einfachste Weg, um Zweige zusammenzuführen, ist, wie bereits behandelt, die `merge`-Anweisung. Sie führt einen Drei-Wege-Merge durch zwischen den beiden letzten Branch-Zuständen (C3 und C4) und dem letzen gemeinsamen Vorgänger (C2) der beiden, erstellt einen neuen Schnappschuss (und einen Commit), wie in Abbildung 3-28 dargestellt.

<!--Figure 3-28. Merging a branch to integrate the diverged work history.-->

Insert 18333fig0328.png
Abbildung 3-28. Zusammenführen von Branches, um die verschiedenen Arbeitsfortschritte zu integrieren.

<!--However, there is another way: you can take the patch of the change that was introduced in C3 and reapply it on top of C4. In Git, this is called _rebasing_. With the `rebase` command, you can take all the changes that were committed on one branch and replay them on another one.-->

Allerdings gibt es noch einen anderen Weg: Du kannst den Patch der Änderungen, den wir in C3 eingeführt haben, nehmen und erneut anwenden auf C4. Dieses Vorgehen nennt man in Git _rebasing_. Mit der `rebase`-Anweisung kannst Du alle Änderungen, die an einem Branch vorgenommen wurden, auf einen anderen Branch erneut anwenden.

<!--In this example, you’d run the following:-->

In unserem Beispiel würdest Du folgendes ausführen:

	$ git checkout experiment
	$ git rebase master
	First, rewinding head to replay your work on top of it...
	Applying: added staged command

<!--It works by going to the common ancestor of the two branches (the one you’re on and the one you’re rebasing onto), getting the diff introduced by each commit of the branch you’re on, saving those diffs to temporary files, resetting the current branch to the same commit as the branch you are rebasing onto, and finally applying each change in turn. Figure 3-29 illustrates this process.-->

Dies funktioniert, indem Git zum letzten gemeinsamen Vorfahren der beiden Branches (der, auf dem Du arbeitest, und jener, auf den Du _rebasen_ möchtest) geht, dann die Informationen zu den Änderungen (diffs) sammelt, welche seit dem bei jedem einzelen Commit des aktuellen Branches gemacht wurden, diese in temporären Dateien speichert, den aktuellen Branch auf den gleichen Commit setzt wie den Branch, auf den Du _rebasen_ möchtest und dann alle Änderungen erneut durchführt. Die Abbildung 3-29 bildet diesen Prozess ab.

<!--Figure 3-29. Rebasing the change introduced in C3 onto C4.-->

Insert 18333fig0329.png
Abbildung 3-29. Rebasen der in C3 durchgeführten Änderungen auf C4.

<!--At this point, you can go back to the master branch and do a fast-forward merge (see Figure 3-30).-->

An diesem Punkt kannst Du zurück zum Master-Branch wechseln und einen fast-forward Merge durchführen (siehe Abbildung 3-30).

<!--Figure 3-30. Fast-forwarding the master branch.-->

Insert 18333fig0330.png
Abbildung 3-30. Fast-forward des Master-Branches.

<!--Now, the snapshot pointed to by C3' is exactly the same as the one that was pointed to by C5 in the merge example. There is no difference in the end product of the integration, but rebasing makes for a cleaner history. If you examine the log of a rebased branch, it looks like a linear history: it appears that all the work happened in series, even when it originally happened in parallel.-->

Nun ist der Schnappschuss, auf den C3' zeigt, exakt der gleiche, wie der auf den C5 in dem Merge-Beispiel gezeigt hat. Bei dieser Zusammenführung entsteht kein unterschiedliches Produkt, durch Rebasing ensteht allerdings ein sauberer Verlauf. Bei genauerer Betrachtung der Historie entpuppt sich der Rebased-Branch als linearer Verlauf – es scheint, als sei die ganze Arbeit in einer Serie entstanden, auch wenn sie in Wirklichkeit parallel stattfand.

<!--Often, you’ll do this to make sure your commits apply cleanly on a remote branch — perhaps in a project to which you’re trying to contribute but that you don’t maintain. In this case, you’d do your work in a branch and then rebase your work onto `origin/master` when you were ready to submit your patches to the main project. That way, the maintainer doesn’t have to do any integration work — just a fast-forward or a clean apply.-->

Du wirst das häufig anwenden um sicherzustellen, dass sich Deine Commits sauber in einen Remote-Branch integrieren – möglicherweise in einem Projekt, bei dem Du Dich beteiligen möchtest, Du jedoch nicht der Verantwortliche bist. In diesem Fall würdest Du Deine Arbeiten in einem eigenen Branch erledigen und im Anschluss Deine Änderungen auf `origin/master` rebasen. Dann hätte der Verantwortliche nämlich keinen Aufwand mit der Integration – nur einen Fast-Forward oder eine saubere Integration (= Rebase?).

<!--Note that the snapshot pointed to by the final commit you end up with, whether it’s the last of the rebased commits for a rebase or the final merge commit after a merge, is the same snapshot — it’s only the history that is different. Rebasing replays changes from one line of work onto another in the order they were introduced, whereas merging takes the endpoints and merges them together.-->

Beachte, dass der Schnappschuss, welche auf den letzten Commit zeigt, ob es nun der letzte der Rebase-Commits nach einem Rebase oder der finale Merge-Commit nach einem Merge ist, der selbe Schnappschuss ist, nur der Verlauf ist ein anderer. Rebasing wiederholt einfach die Änderungen einer Arbeitslinie auf einer anderen in der Reihenfolge, in der sie entstanden sind. Im Gegensatz hierzu nimmt Merging die beiden Endpunkte der Arbeitslinien und führt diese zusammen.

<!--### More Interesting Rebases ###-->
### Mehr interessante Rebases ###

<!--You can also have your rebase replay on something other than the rebase branch. Take a history like Figure 3-31, for example. You branched a topic branch (`server`) to add some server-side functionality to your project, and made a commit. Then, you branched off that to make the client-side changes (`client`) and committed a few times. Finally, you went back to your server branch and did a few more commits.-->

Du kannst Deinen Rebase auch auf einem anderen Branch als dem Rebase-Branch anwenden lassen. Nimm zum Beispiel den Verlauf in Abbildung 3-31. Du hattest einen Themen-Branch (`server`) angelegt, um ein paar serverseitige Funktionalitäten zu Deinem Projekt hinzuzufügen, und hast dann einen Commit gemacht. Dann hast Du einen weiteren Branch abgezweigt, um clientseitige Änderungen (`client`) vorzunehmen und dort ein paarmal committed. Zum Schluss hast Du wieder zu Deinem Server-Branch gewechselt und ein paar weitere Commits gebaut.

<!--Figure 3-31. A history with a topic branch off another topic branch.-->

Insert 18333fig0331.png
Abbildung 3-31. Ein Verlauf mit einem Themen-Branch basierend auf einem weiteren Themen-Branch.

<!--Suppose you decide that you want to merge your client-side changes into your mainline for a release, but you want to hold off on the server-side changes until it’s tested further. You can take the changes on client that aren’t on server (C8 and C9) and replay them on your master branch by using the `-\-onto` option of `git rebase`:-->

Angenommen, Du entscheidest Dich, Deine clientseitigen Änderungen für einen Release in die Hauptlinie zu mergen, während Du die serverseitigen Änderungen noch zurückhalten möchtest, bis sie besser getestet wurden. Du kannst einfach die Änderungen am Client, die den Server nicht betreffen, (C8 und C9) mit der `--onto`-Option von `git rebase` erneut auf den Master-Branch anwenden:

	$ git rebase --onto master server client

<!--This basically says, “Check out the client branch, figure out the patches from the common ancestor of the `client` and `server` branches, and then replay them onto `master`.” It’s a bit complex; but the result, shown in Figure 3-32, is pretty cool.-->

Das bedeutet einfach, “Checke den Client-Branch aus, finde die Patches heraus, die auf dem gemeinsamen Vorfahr der `client`- und `server`-Branches basieren und wende sie erneut auf dem `master`-Branch an.” Das ist ein bisschen komplex, aber das Ergebnis – wie in Abbildung 3-32 – ist richtig cool.

<!--Figure 3-32. Rebasing a topic branch off another topic branch.-->

Insert 18333fig0332.png
Abbildung 3-32. Rebasing eines Themen-Branches von einem anderen Themen-Branch.

<!--Now you can fast-forward your master branch (see Figure 3-33):-->

Jetzt kannst Du Deinen Master-Branch fast-forwarden (siehe Abbildung 3-33):

	$ git checkout master
	$ git merge client

<!--Figure 3-33. Fast-forwarding your master branch to include the client branch changes.-->

Insert 18333fig0333.png
Abbildung 3-33. Fast-forwarding Deines Master-Branches um die Client-Branch-Änderungen zu integrieren.

<!--Let’s say you decide to pull in your server branch as well. You can rebase the server branch onto the master branch without having to check it out first by running `git rebase [basebranch] [topicbranch]` — which checks out the topic branch (in this case, `server`) for you and replays it onto the base branch (`master`):-->

Lass uns annehmen, Du entscheidest Dich, Deinen Server-Branch ebenfalls einzupflegen. Du kannst den Server-Branch auf den Master-Branch rebasen, ohne diesen vorher auschecken zu müssen, indem Du die Anweisung `git rebase [Basis-Branch] [Themen-Branch]` ausführst. Sie macht für Dich den Checkout des Themen-Branches (in diesem Fall `server`) und wiederholt ihn auf dem Basis-Branch (`master`):

	$ git rebase master server

<!--This replays your `server` work on top of your `master` work, as shown in Figure 3-34.-->

Das wiederholt Deine `server`-Arbeit auf der Basis der `master`-Arbeit, wie in Abbildung 3-34 ersichtlich.

<!--Figure 3-34. Rebasing your server branch on top of your master branch.-->

Insert 18333fig0334.png
Abbildung 3-34. Rebasing Deines Server-Branches auf Deinen Master-Branch.

<!--Then, you can fast-forward the base branch (`master`):-->

Dann kannst Du den Basis-Branch (`master`) fast-forwarden:

	$ git checkout master
	$ git merge server

<!--You can remove the `client` and `server` branches because all the work is integrated and you don’t need them anymore, leaving your history for this entire process looking like Figure 3-35:-->

Du kannst den `client`- und `server`-Branch nun entfernen, da Du die ganze Arbeit bereits integriert wurde und sie nicht mehr benötigst. Du hinterlässt den Verlauf für den ganzen Prozess wie in Abbildung 3-35:

	$ git branch -d client
	$ git branch -d server

<!--Figure 3-35. Final commit history.-->

Insert 18333fig0335.png
Abbildung 3-35: Endgültiger Commit-Verlauf.

<!--### The Perils of Rebasing ###-->
### Die Gefahren des Rebasings ###

<!--Ahh, but the bliss of rebasing isn’t without its drawbacks, which can be summed up in a single line:-->

Ahh, aber der ganze Spaß mit dem Rebasing kommt nicht ohne seine Schattenseiten, welche in einer einzigen Zeile zusammengefasst werden können:

<!--**Do not rebase commits that you have pushed to a public repository.**-->

**Rebase keine Commits die Du in ein öffentliches Repository hochgeladen hast.**

<!--If you follow that guideline, you’ll be fine. If you don’t, people will hate you, and you’ll be scorned by friends and family.-->

Wenn Du diesem Ratschlag folgst, ist alles in Ordnung. Falls nicht, werden die Leute Dich hassen und Du wirst von Deinen Freunden und Deiner Familie verachtet.

<!--When you rebase stuff, you’re abandoning existing commits and creating new ones that are similar but different. If you push commits somewhere and others pull them down and base work on them, and then you rewrite those commits with `git rebase` and push them up again, your collaborators will have to re-merge their work and things will get messy when you try to pull their work back into yours.-->

Wenn Du Zeug rebased, hebst Du bestehende Commits auf und erstellst stattdessen welche, die zwar ähnlich aber unterschiedlich sind. Wenn Du Commits irgendwohin hochlädst und andere ziehen sich diese herunter und nehmen sie als Grundlage für ihre Arbeit, dann müssen Deine Mitwirkenden ihre Arbeit jedesmal re-mergen, sobald Du Deine Commits mit einem `git rebase` überschreibst und verteilst. Und richtig chaotisch wird es, wenn Du versuchst, deren Arbeit in Deine Commits zu integrieren.

<!--Let’s look at an example of how rebasing work that you’ve made public can cause problems. Suppose you clone from a central server and then do some work off that. Your commit history looks like Figure 3-36.-->

Lass uns mal ein Beispiel betrachten, wie das Rebasen veröffentlichter Arbeit Probleme verursachen kann. Angenommen, Du klonst von einem zentralen Server und werkelst ein bisschen daran rum. Dein Commit-Verlauf sieht wie in Abbildung 3-36 aus.

<!--Figure 3-36. Clone a repository, and base some work on it.-->

Insert 18333fig0336.png
Abbildung 3-36. Klone ein Repository und baue etwas darauf auf.

<!--Now, someone else does more work that includes a merge, and pushes that work to the central server. You fetch them and merge the new remote branch into your work, making your history look something like Figure 3-37.-->

Ein anderer arbeitet unterdessen weiter, macht einen Merge und lädt seine Arbeit auf den zentralen Server. Du fetchst die Änderungen und mergest den neuen Remote-Branch in Deine Arbeit, sodass Dein Verlauf wie in Abbildung 3-37 aussieht.

<!--Figure 3-37. Fetch more commits, and merge them into your work.-->

Insert 18333fig0337.png
Abbildung 3-37. Fetche mehrere Commits und merge sie in Deine Arbeit.

<!--Next, the person who pushed the merged work decides to go back and rebase their work instead; they do a `git push -\-force` to overwrite the history on the server. You then fetch from that server, bringing down the new commits.-->

Als nächstes entscheidet sich die Person, welche den Merge hochgeladen hat, diesen rückgängig zu machen und stattdessen die Commits zu rebasen. Sie macht einen `git push --force`, um den Verlauf auf dem Server zu überschreiben. Du lädst Dir das Ganze dann mit den neuen Commits herunter.

<!--Figure 3-38. Someone pushes rebased commits, abandoning commits you’ve based your work on.-->

Insert 18333fig0338.png
Abbildung 3-38. Jemand pusht rebased Commits und verwirft damit Commits, auf denen Deine Arbeit basiert.

<!--At this point, you have to merge this work in again, even though you’ve already done so. Rebasing changes the SHA-1 hashes of these commits so to Git they look like new commits, when in fact you already have the C4 work in your history (see Figure 3-39).-->

Nun musst Du seine Arbeit erneut in Deine Arbeitslinie mergen, obwohl Du das bereits einmal gemacht hast. Rebasing ändert die SHA-1-Hashes der Commits, weshalb sie für Git wie neue Commits aussehen. In Wirklichkeit hast Du die C4-Arbeit bereits in Deinem Verlauf (siehe Abbildung 3-39).

<!--Figure 3-39. You merge in the same work again into a new merge commit.-->

Insert 18333fig0339.png
Abbildung 3-39. Du mergst die gleiche Arbeit nochmals in einen neuen Merge-Commit.

<!--You have to merge that work in at some point so you can keep up with the other developer in the future. After you do that, your commit history will contain both the C4 and C4' commits, which have different SHA-1 hashes but introduce the same work and have the same commit message. If you run a `git log` when your history looks like this, you’ll see two commits that have the same author date and message, which will be confusing. Furthermore, if you push this history back up to the server, you’ll reintroduce all those rebased commits to the central server, which can further confuse people.-->

Irgendwann musst Du seine Arbeit mergen, damit Du auch zukünftig mit dem anderen Entwickler zusammenarbeiten kannst. Danach wird Dein Commit-Verlauf sowohl den C4 als auch den C4'-Commit enthalten, welche zwar verschiedene SHA-1-Hashes besitzen, aber die gleichen Änderungen und die gleiche Commit-Beschreibung enthalten. Wenn Du so einen Verlauf mit `git log` betrachtest, wirst Du immer zwei Commits des gleichen Autors, zur gleichen Zeit und mit der gleichen Commit-Nachricht sehen. Was ganz schön verwirrend sein wird. Wenn Du diesen Verlauf außerdem auf den Server hochlädst, wirst Du dort alle rebasierten Commits nochmals einführen, was die Leute noch mehr verwirren kann.

<!--If you treat rebasing as a way to clean up and work with commits before you push them, and if you only rebase commits that have never been available publicly, then you’ll be fine. If you rebase commits that have already been pushed publicly, and people may have based work on those commits, then you may be in for some frustrating trouble.-->

Wenn Du rebasing als einen Weg getrachtest, um aufzuräumen und mit Commits zu arbeiten, bevor Du sie hochlädst und wenn Du rebase nur auf Commits anwendest, die noch nie öffentlich zugänglich waren, dann fährst Du goldrichtig. Wenn Du rebase auf Commits anwendest, die bereits veröffentlicht wurden und Leute vielleicht schon ihre Arbeit darauf aufgebaut haben, dann kannst Du Dich auf frustrierenden Ärger gefasst machen.

<!--## Summary ##-->
## Zusammenfassung ##

<!--We’ve covered basic branching and merging in Git. You should feel comfortable creating and switching to new branches, switching between branches and merging local branches together.  You should also be able to share your branches by pushing them to a shared server, working with others on shared branches and rebasing your branches before they are shared.-->

Wir haben einfaches Branching und Merging mit Git behandelt. Du solltest nun gut damit zurecht kommen, Branches zu erstellen, zwischen Branches zu wechseln und lokale Branches mit einem Merge zusammenzuführen. Außerdem solltest Du in der Lage sein, Deine Branches zu veröffentlichen, indem Du sie auf einen zentralen Server lädst, mit anderen auf öffentlichen Branches zusammenzuarbeiten und Deine Branches zu rebasen, bevor sie veröffentlicht werden.
<!--# Git on the Server #-->
# Git auf dem Server #

<!--At this point, you should be able to do most of the day-to-day tasks for which you’ll be using Git. However, in order to do any collaboration in Git, you’ll need to have a remote Git repository. Although you can technically push changes to and pull changes from individuals’ repositories, doing so is discouraged because you can fairly easily confuse what they’re working on if you’re not careful. Furthermore, you want your collaborators to be able to access the repository even if your computer is offline — having a more reliable common repository is often useful. Therefore, the preferred method for collaborating with someone is to set up an intermediate repository that you both have access to, and push to and pull from that. We’ll refer to this repository as a "Git server"; but you’ll notice that it generally takes a tiny amount of resources to host a Git repository, so you’ll rarely need to use an entire server for it.-->

Zum jetzigen Zeitpunkt solltest Du in der Lage sein, die am häufigsten wiederkehrenden Aufgaben mit Git zu lösen. Um eine Zusammenarbeit zu ermöglichen solltest Du jedoch darüber nachdenken, ein externes Repository zur Verfügung stellen. Wenngleich es technisch möglich ist, direkt mit Repositories Anderer zu arbeiten und Änderungen dorthin zu pushen oder von dort zu holen, ist dieses Vorgehen verpöhnt, da es sehr leicht die Arbeit Anderer durcheinander zu bringen. Wenn man nicht vorsichtig ist, verliert man schnell den Überblick darüber wer woran arbeitet. Des weiteren erfordert der direkte Umgang mit Git Repositories, dass diese permanent verfügbar sind. Ein Repository auf dem eigenen Computer ist nur dann für alle Mitentwickler erreichbar, wenn der Computer läuft. Es macht deshalb Sinn, ein zentrales und zuverlässig erreichbares Repository einzurichten. Wir werden dieses Repository im Folgenden als „Git Server“ bezeichnen. Es sollte jedoch schnell klar werden, dass nur minimale Ressourcen notwendig sind, um Git Repositories zu hosten. In den seltensten Fällen ist ein dedizierter Server dafür notwendig.

<!--Running a Git server is simple. First, you choose which protocols you want your server to communicate with. The first section of this chapter will cover the available protocols and the pros and cons of each. The next sections will explain some typical setups using those protocols and how to get your server running with them. Last, we’ll go over a few hosted options, if you don’t mind hosting your code on someone else’s server and don’t want to go through the hassle of setting up and maintaining your own server.-->

Einen Git Server zu betreiben ist einfach. Die erste Entscheidung, die zu treffen ist, ist die des zu verwendenden Protokolls zur Kommunikation mit dem Server. Der erste Teil dieses Kapitels wird deshalb die zur Verfügung stehenden Protokolle und ihre Vor- und Nachteile beschreiben. Darüber hinaus werden einige typische Konfigurationen zum Betreiben eines Git Servers vorgestellt. Falls keine Bedenken bestehen, Code von externen Anbietern hosten zu lassen, werden zuletzt ein paar Optionen für gehostete Git Repositories aufgezeigt. Dies erspart die Mehrarbeit der Einrichtung und Wartung eines eigenen Git Servers.

<!--If you have no interest in running your own server, you can skip to the last section of the chapter to see some options for setting up a hosted account and then move on to the next chapter, where we discuss the various ins and outs of working in a distributed source control environment.-->

Wenn Du kein Interesse am Betreiben eines eigenen Servers hast, kannst Du zum letzten Absatz des Kapitels springen, um ein paar Möglichkeiten zum Einrichten eines gehosteten Accounts zu sehen. Im nächsten Kapitel diskutieren wir verschiedene Vor- und Nachteile vom Arbeiten in einer verteilten Quellcode-Kontroll-Umgebung.

<!--A remote repository is generally a _bare repository_ — a Git repository that has no working directory. Because the repository is only used as a collaboration point, there is no reason to have a snapshot checked out on disk; it’s just the Git data. In the simplest terms, a bare repository is the contents of your project’s `.git` directory and nothing else.-->

Ein externes Repository ist im Allgemeinen ein _einfaches Repository_ – ein Git Repository ohne Arbeitsverzeichnis. Weil das Repository nur als Zusammenarbeitspunkt genutzt wird, gibt es keinen Grund, einen Schnappschuss ausgecheckt auf der Festplatte zu haben; es sind nur die Git Daten. Mit einfachen Begriffen, ein einfaches Repository ist der Inhalt von Deinem `.git` Verzeichnis in Deinem Projekt und nichts anderes.

<!--## The Protocols ##-->
## Die Protokolle ##

<!--Git can use four major network protocols to transfer data: Local, Secure Shell (SSH), Git, and HTTP. Here we’ll discuss what they are and in what basic circumstances you would want (or not want) to use them.-->

Git kann vier wichtige Netzwerk Protokolle zum Datentransfer benutzen: Lokal, Secure Shell (SSH), Git und HTTP. Hier wollen wir diskutieren, was diese Protokolle sind und unter welchen grundlegenden Gegebenheiten Du sie benutzen möchtest (oder auch nicht).

<!--It’s important to note that with the exception of the HTTP protocols, all of these require Git to be installed and working on the server.-->

Es ist wichtig zu beachten, dass alle Protokolle mit Ausnahme von HTTP eine funktionierende Git Installation auf dem Server benötigen.

<!--### Local Protocol ###-->
### Lokales Protokoll ###

<!--The most basic is the _Local protocol_, in which the remote repository is in another directory on disk. This is often used if everyone on your team has access to a shared filesystem such as an NFS mount, or in the less likely case that everyone logs in to the same computer. The latter wouldn’t be ideal, because all your code repository instances would reside on the same computer, making a catastrophic loss much more likely.-->

Am einfachsten ist das _Lokale Protokoll_, wobei das externe Repository in einem anderen Verzeichnis auf der Festplatte ist. Das wird oft genutzt, wenn jeder aus Deinem Team Zugriff zu einem gemeinsamen Dateisystem hat, zum Beispiel ein eingebundenes NFS, oder im unwahrscheinlicheren Fall jeder loggt sich auf bei dem gleichen Computer ein. Das letztere ist nicht ideal, weil alle Code Repository Instanzen auf dem selben Computer wären, ein katastrophaler Verlust wäre wahrscheinlicher.

<!--If you have a shared mounted filesystem, then you can clone, push to, and pull from a local file-based repository. To clone a repository like this or to add one as a remote to an existing project, use the path to the repository as the URL. For example, to clone a local repository, you can run something like this:-->

Wenn Du ein gemeinsames Dateisystem eingebunden hast, kannst Du von einem lokalen Datei-basiertem Repository clonen, pushen und pullen. Um ein Repository wie dieses zu clonen, oder ein externes zu einem bestehenden Projekt hinzuzufügen, benutze den Pfad zu dem Repository als URL. Um zum Beispiel ein lokales Repository zu clonen kannst Du einen Befehl wie diesen nutzen:

	$ git clone /opt/git/project.git

<!--Or you can do this:-->

Oder Du kannst das machen:

	$ git clone file:///opt/git/project.git

<!--Git operates slightly differently if you explicitly specify `file://` at the beginning of the URL. If you just specify the path, and the source and the destination are on the same filesystem, Git tries to hardlink the objects it needs. If they are not on the same filesystem, it will copy the objects it needs using the system's standard copying functionality. If you specify `file://`, Git fires up the processes that it normally uses to transfer data over a network which is generally a lot less efficient method of transferring the data. The main reason to specify the `file://` prefix is if you want a clean copy of the repository with extraneous references or objects left out — generally after an import from another version-control system or something similar (see Chapter 9 for maintenance tasks). We’ll use the normal path here because doing so is almost always faster.-->

Git arbeitet etwas anders, wenn Du am Anfang der URL ausdrücklich `file://` angibst. Wenn Du nur den Pfad angibst, und sowohl die Quelle, als auch das Ziel sich auf dem selben Dateisystem befinden, versucht Git harte Links zu benutzen. Wenn sie sich nicht auf dem selben Dateisystem befinden, kopiert Git die benötigten Objekte mit Hilfe der Standardkopierfunktion des jeweiligen Betriebssystems. Wenn Du `file://` angibst, startet Git den Prozess, den es normalerweise zum Übertragen von Daten über ein Netzwerk verwendet, dass ist gewöhnlich eine wesentlich ineffizientere Methode zum Übertragen der Daten. Der Hauptgrund das `file://`-Präfix anzugeben ist eine saubere Kopie von dem Repository mit fremden Referenzen oder fehlenden Objekten – generell nach einem Import von einem anderen Versionskontrollsystem oder etwas ähnliches (siehe Kapitel 9 für Wartungsarbeiten). Wir benutzen hier den normalen Pfad, weil das fast immer schneller ist.

<!--To add a local repository to an existing Git project, you can run something like this:-->

Um ein lokales Repository zu einem existierenden Git Projekt hinzuzufügen, kannst Du einen Befehl wie diesen ausführen:

	$ git remote add local_proj /opt/git/project.git

<!--Then, you can push to and pull from that remote as though you were doing so over a network.-->

Dann kannst Du zu diesem externen Repository pushen und davon pullen, als würdest Du das über ein Netzwerk machen.

<!--#### The Pros ####-->
#### Die Vorteile ####

<!--The pros of file-based repositories are that they’re simple and they use existing file permissions and network access. If you already have a shared filesystem to which your whole team has access, setting up a repository is very easy. You stick the bare repository copy somewhere everyone has shared access to and set the read/write permissions as you would for any other shared directory. We’ll discuss how to export a bare repository copy for this purpose in the next section, “Getting Git on a Server.”-->

Die Vorteile von Datei-basierten Repositories sind die Einfachheit und das Nutzen bereits bestehender Datei-Berechtigungen und bestehendem Netzwerk-Zugriff. Wenn Du bereits ein gemeinsames Dateisystem hast, zu dem das gesamte Team Zugriff hat, ist das Einrichten eines Repositories sehr einfach. Du exportierst eine Kopie des einfachen Repositories dahin, wo jeder gemeinsamen Zugriff hat und stellst die Lese- und Schreibberechtigungen wie bei jedem anderem gemeinsamen Verzeichnis ein. Wir werden im nächsten Abschnitt „Git auf einen Server bekommen“ diskutieren, wie man die Kopie eines einfachen Repositories für diesen Zweck exportiert.

<!--This is also a nice option for quickly grabbing work from someone else’s working repository. If you and a co-worker are working on the same project and they want you to check something out, running a command like `git pull /home/john/project` is often easier than them pushing to a remote server and you pulling down.-->

Dies ist auch eine nette Möglichkeit zum schnellen Abholen von Änderungen aus dem Arbeitsverzeichnis von jemand anderem. Wenn Du und ein Kollege an dem gleichen Projekt arbeitet und ihr wollt etwas auschecken, dann ein Befehl wie `git pull /home/john/project` ist oft einfacher als das pushen zu einem externen Server und das pullen zurück.

<!--#### The Cons ####-->
#### Die Nachteile ####

<!--The cons of this method are that shared access is generally more difficult to set up and reach from multiple locations than basic network access. If you want to push from your laptop when you’re at home, you have to mount the remote disk, which can be difficult and slow compared to network-based access.-->

Die Nachteile von dieser Methode sind, dass ein gemeinsamer Zugriff im allgemeinen schwieriger einrichten ist und der Zugriff von mehreren Orten ist schwieriger als einfacher Netzwerk Zugriff. Wenn Du von Deinem Laptop zuhause pushen möchtest, musst Du eine entfernte Festplatte einbinden. Das kann schwierig und langsam sein, verglichen mit netzwerk-basiertem Zugriff.

<!--It’s also important to mention that this isn’t necessarily the fastest option if you’re using a shared mount of some kind. A local repository is fast only if you have fast access to the data. A repository on NFS is often slower than the repository over SSH on the same server, allowing Git to run off local disks on each system.-->

Es ist auch wichtig zu erwähnen, dass dies nicht unbedingt die schnellste Möglichkeit ist, wenn Du ein gemeinsames Dateisystem oder ähnliches hast. Ein lokales Repository ist nur dann schnell, wenn Du schnellen Zugriff auf die Daten hast. Ein NFS-basiertes Repository ist oftmals langsamer als ein Repository über SSH auf dem gleichen Server, weil Git über SSH auf jedem System auf den lokalen Festplatten arbeitet.

<!--### The SSH Protocol ###-->
### Das SSH Protokoll ###

<!--Probably the most common transport protocol for Git is SSH. This is because SSH access to servers is already set up in most places — and if it isn’t, it’s easy to do. SSH is also the only network-based protocol that you can easily read from and write to. The other two network protocols (HTTP and Git) are generally read-only, so even if you have them available for the unwashed masses, you still need SSH for your own write commands. SSH is also an authenticated network protocol; and because it’s ubiquitous, it’s generally easy to set up and use.-->

Das vermutlich gebräuchlichste Transport-Protokoll für Git ist SSH. Das hat den Grund, dass der SSH-Zugriff an den meisten Orten bereits eingerichtet ist – und wenn das nicht der Fall ist, einfach zu machen ist. SSH ist außerdem das einzige netzwerk-basierte Protokoll von dem man einfach lesen und darauf schreiben kann. Die beiden anderen Netzwerk-Protokolle (HTTP und Git) sind nur lesbar. Auch wenn sie für die breite Masse sind, brauchst Du trotzdem SSH für Deine Schreib-Befehle. SSH ist auch ein authentifiziertes Netzwerk-Protokoll, und weil es universell ist, ist es im Allgemeinen einfach einzurichten und zu benutzen.

<!--To clone a Git repository over SSH, you can specify ssh:// URL like this:-->

Um ein Git Repository über SSH zu clonen, kannst Du eine ssh:// URL angeben:

	$ git clone ssh://user@server/project.git

<!--Or you can use the shorter scp-like syntax for SSH protocol:-->

Oder Du kannst auch kein Protokoll angeben – Git nimmt SSH an, wenn Du nicht eindeutig bist:

	$ git clone user@server:project.git

<!--You can also not specify a user, and Git assumes the user you’re currently logged in as.-->

Du kannst auch keinen Benutzer angeben, und Git nimmt den Benutzer an, als der Du gerade eingeloggt bist.

<!--#### The Pros ####-->
#### Die Vorteile ####

<!--The pros of using SSH are many. First, you basically have to use it if you want authenticated write access to your repository over a network. Second, SSH is relatively easy to set up — SSH daemons are commonplace, many network admins have experience with them, and many OS distributions are set up with them or have tools to manage them. Next, access over SSH is secure — all data transfer is encrypted and authenticated. Last, like the Git and Local protocols, SSH is efficient, making the data as compact as possible before transferring it.-->

Die Vorteile von SSH sind vielseitig. Erstens, grundlegend musst Du es benutzen, wenn Du authentifizierten Schreib-Zugriff auf Dein Repository über ein Netzwerk haben möchtest. Zweitens, SSH ist relativ einfach einzurichten – SSH-Dämonen sind alltäglich, viele Netzwerk-Administratoren haben Erfahrungen mit ihnen und viele Betriebssysteme sind mit ihnen eingerichtet oder haben Werkzeuge um sie zu verwalten. Als nächstes, Zugriff über SSH ist sicher – der gesamte Daten-Transfer ist verschlüsselt und authentifiziert. Als letztes, wie Git und die lokalen Protokolle, SSH ist effizient, es macht die Daten so kompakt wie möglich bevor es die Daten überträgt.

<!--#### The Cons ####-->
#### Die Nachteile ####

<!--The negative aspect of SSH is that you can’t serve anonymous access of your repository over it. People must have access to your machine over SSH to access it, even in a read-only capacity, which doesn’t make SSH access conducive to open source projects. If you’re using it only within your corporate network, SSH may be the only protocol you need to deal with. If you want to allow anonymous read-only access to your projects, you’ll have to set up SSH for you to push over but something else for others to pull over.-->

Die negative Seite von SSH ist, dass Du Deine Repositories nicht anonym darüber anbieten kannst. Die Leute müssen Zugriff auf Deine Maschine über SSH haben um zuzugreifen, auch mit einem Nur-Lese-Zugriff, was SSH nicht zuträglich zu Open-Source-Projekten macht. Wenn Du es nur innerhalb von Deinem Firmen-Netzwerk benutzt, SSH ist vielleicht das einzige Protokoll mit dem Du arbeiten musst. Wenn Du anonymen Nur-Lese-Zugriff zu Deinen Projekten erlauben willst, musst Du SSH für Dich einsetzen um zu pushen, aber ein anderes Protokoll für andere um zu pullen.

<!--### The Git Protocol ###-->
### Das Git Protokoll ###

<!--Next is the Git protocol. This is a special daemon that comes packaged with Git; it listens on a dedicated port (9418) that provides a service similar to the SSH protocol, but with absolutely no authentication. In order for a repository to be served over the Git protocol, you must create the `git-daemon-export-ok` file — the daemon won’t serve a repository without that file in it — but other than that there is no security. Either the Git repository is available for everyone to clone or it isn’t. This means that there is generally no pushing over this protocol. You can enable push access; but given the lack of authentication, if you turn on push access, anyone on the internet who finds your project’s URL could push to your project. Suffice it to say that this is rare.-->

Als nächstes kommt das Git Protokoll. Das ist ein spezieller Dämon, der zusammen mit Git kommt. Er horcht auf einem bestimmten Port (9418), dieser Service ist vergleichbar mit dem SSH-Protokoll, aber ohne jegliche Authentifizierung. Um ein Repository über das Git Protokoll, musst Du die `git-daemon-export-ok` Datei erstellen – der Dämon bietet kein Repository ohne die Datei darin an – außer dieser Datei gibt es keine Sicherheit. Entweder das Git Repository ist für jeden zum Clonen verfügbar oder halt nicht. Das bedeutet, dass dieses Protokoll generell kein push anbietet. Du kannst push-Zugriff aktivieren; aber ohne Authentifizierung, wenn Du den push-Zugriff aktivierst, kann jeder im Internet, der Deine Projekt-URL findet, zu Deinem Projekt pushen. Ausreichend zu sagen, dass das selten ist.

<!--#### The Pros ####-->
#### Die Vorteile ####

<!--The Git protocol is the fastest transfer protocol available. If you’re serving a lot of traffic for a public project or serving a very large project that doesn’t require user authentication for read access, it’s likely that you’ll want to set up a Git daemon to serve your project. It uses the same data-transfer mechanism as the SSH protocol but without the encryption and authentication overhead.-->

Das Git Protokoll ist das schnellste verfügbare Transfer Protokoll. Wenn Du viel Traffic für ein öffentliches Projekt hast oder ein sehr großes Projekt hast, dass keine Benutzer-Authentifizierung für den Lese-Zugriff voraussetzt, es ist üblich einen Git Dämon einzurichten, der Dein Projekt serviert. Er benutzt den selben Daten-Transfer Mechanismus wie das SSH-Protokoll, aber ohne den Entschlüsselungs- und Authentifizierungs-Overhead.

<!--#### The Cons ####-->
#### Die Nachteile ####

<!--The downside of the Git protocol is the lack of authentication. It’s generally undesirable for the Git protocol to be the only access to your project. Generally, you’ll pair it with SSH access for the few developers who have push (write) access and have everyone else use `git://` for read-only access.-->
<!--It’s also probably the most difficult protocol to set up. It must run its own daemon, which is custom — we’ll look at setting one up in the “Gitosis” section of this chapter — it requires `xinetd` configuration or the like, which isn’t always a walk in the park. It also requires firewall access to port 9418, which isn’t a standard port that corporate firewalls always allow. Behind big corporate firewalls, this obscure port is commonly blocked.-->

Die Negativseite des Git Protokoll ist das Fehlen der Authentifizierung. Es ist generell unerwünscht, dass das Git Protokoll der einzige Zugang zu dem Projekt ist. Im Allgemeinen willst Du es mit SSH-Zugriff für die Entwickler paaren, die push (Schreib) Zugriff haben und jeder andere benutzt `git://` für Nur-Lese-Zugriff.
Es ist vielleicht auch das das schwierigste Protokoll beim Einrichten. Es muss ein eigener Dämon laufen, welcher Git-spezifisch ist – wir wollen im „Gitosis“-Abschnitt in diesem Kapitel schauen, wie man einen einrichtet – es setzt eine `xinetd`-Konfiguration oder ähnliches voraus, das ist nicht immer ein Spaziergang. Es setzt auch einen Firewall-Zugriff auf den Port 9418 voraus, das ist kein Standard-Port, den Firmen-Firewalls immer erlauben. Hinter einer großen Firmen-Firewall ist dieser unklare Port häufig gesperrt.

<!--### The HTTP/S Protocol ###-->
### Das HTTP/S Protokoll ###

<!--Last we have the HTTP protocol. The beauty of the HTTP or HTTPS protocol is the simplicity of setting it up. Basically, all you have to do is put the bare Git repository under your HTTP document root and set up a specific `post-update` hook, and you’re done (See Chapter 7 for details on Git hooks). At that point, anyone who can access the web server under which you put the repository can also clone your repository. To allow read access to your repository over HTTP, do something like this:-->

Als letztes haben wir das HTTP Protokoll. Das Schöne am HTTP bzw. HTTPS Protokoll ist die Einfachheit des Einrichtens. Im Grunde musst Du nur das einfach Git Repository in Dein HTTP Hauptverzeichnis legen und einen speziellen `post-update` hook (xxx) einrichten und schon bist Du fertig (siehe Kapitel 7 für Details zu Git hooks (xxx)). Jetzt kann jeder, der auf den Web-Server mit dem Repository zugreifen kann, das Repository clonen. Um Lese-Zugriff auf das Repository über HTTP zu erlauben, führe die folgenden Befehle aus:

	$ cd /var/www/htdocs/
	$ git clone --bare /path/to/git_project gitproject.git
	$ cd gitproject.git
	$ mv hooks/post-update.sample hooks/post-update
	$ chmod a+x hooks/post-update

<!--That’s all. The `post-update` hook that comes with Git by default runs the appropriate command (`git update-server-info`) to make HTTP fetching and cloning work properly. This command is run when you push to this repository over SSH; then, other people can clone via something like-->

Das ist alles. Der `post-update` hook (xxx) der standardmäßig zusammen mit Git kommt führt den richtigen Befehl aus (`git update-server-info`), damit der HTTP-Server das Repository richtig abruft und klont. Dieser Befehl läuft, wenn Du zu diesem Repository per SSH pusht, andere Leute können dann clonen mit dem Befehl

	$ git clone http://example.com/gitproject.git

<!--In this particular case, we’re using the `/var/www/htdocs` path that is common for Apache setups, but you can use any static web server — just put the bare repository in its path. The Git data is served as basic static files (see Chapter 9 for details about exactly how it’s served).-->

In diesem besonderen Fall benutzen Dir den `/var/www/htdocs`-Pfad, der typisch für Apache-Setups ist, aber Du kannst jeden statischen Web-Server benutzen – nur das einfache Repository in den richtigen Ordner legen. Die Git-Daten werden als einfache statische Dateien serviert (siehe Kapitel 9 für Details, wie es genau serviert wird).

<!--It’s possible to make Git push over HTTP as well, although that technique isn’t as widely used and requires you to set up complex WebDAV requirements. Because it’s rarely used, we won’t cover it in this book. If you’re interested in using the HTTP-push protocols, you can read about preparing a repository for this purpose at `http://www.kernel.org/pub/software/scm/git/docs/howto/setup-git-server-over-http.txt`. One nice thing about making Git push over HTTP is that you can use any WebDAV server, without specific Git features; so, you can use this functionality if your web-hosting provider supports WebDAV for writing updates to your web site.-->

Es ist möglich, Git-Daten auch über HTTP zu pushen, trotzdem wird diese Technik nicht oft eingesetzt und es setzt komplexe WebDAV-Anforderungen voraus. Weil es selten genutzt wird, werden wir das nicht in diesem Buch behandeln. Wenn Du Interesse am HTTP-Push-Protokoll hast, kannst Du das Einrichten unter `http://www.kernel.org/pub/software/scm/git/docs/howto/setup-git-server-over-http.txt` nachlesen. Eine Schöne Sache über Git-Push über HTTP ist, dass Du jeden WebDAV-Server benutzen kannst, ohne spezifische Git-Features; also kannst Du diese Funktionalität nutzen, wenn Dein Web-Hosting-Provider WebDAV unterstützt, um Änderungen auf Deine Webseite zu schreiben.

<!--#### The Pros ####-->
#### Die Vorteile ####

<!--The upside of using the HTTP protocol is that it’s easy to set up. Running the handful of required commands gives you a simple way to give the world read access to your Git repository. It takes only a few minutes to do. The HTTP protocol also isn’t very resource intensive on your server. Because it generally uses a static HTTP server to serve all the data, a normal Apache server can serve thousands of files per second on average — it’s difficult to overload even a small server.-->

Die Positivseite beim Benutzen des HTTP-Protokolls ist, dass es einfach einzurichten ist. Das Ausführen von einer Handvoll notwendiger Befehle ist ein einfacher Weg, um der Welt Lese-Zugriff auf Dein Git-Repository zu geben. Es braucht nur ein paar Minuten. Das HTTP Protokoll benötigt auch nicht viele Ressourcen auf Deinem Server. Es braucht generell nur einen statischen Server um die Daten zu auszuliefern. Ein normaler Apache-Server kann Tausende von Dateien pro Sekunde servieren – es ist schwierig selbst einen kleinen Server zu überlasten.

<!--You can also serve your repositories read-only over HTTPS, which means you can encrypt the content transfer; or you can go so far as to make the clients use specific signed SSL certificates. Generally, if you’re going to these lengths, it’s easier to use SSH public keys; but it may be a better solution in your specific case to use signed SSL certificates or other HTTP-based authentication methods for read-only access over HTTPS.-->

Du kannst Deine Repositories auch als Nur-Lese-Repositories über HTTPS servieren, Du kannst also den Daten-Transfer verschlüsseln. Oder Du kannst so weit gehen, dass die Clients spezifische signierte SSL-Zertifikate benutzen. Im Allgemeinen, wenn Du soweit gehst, ist es einfacher öffentliche SSH-Schlüssel zu benutzen; aber es ist vielleicht für Deinen Fall eine bessere Lösung, signierte SSL-Zertifikate zu benutzen oder andere HTTP-basierte Authentifizierungs-Methoden für Nur-Lese-Zugriff über HTTPS zu benutzen.

<!--Another nice thing is that HTTP is such a commonly used protocol that corporate firewalls are often set up to allow traffic through this port.-->

Eine andere schöne Sache ist, dass HTTP so oft genutzt wird, dass Firmen-Firewalls oft Traffic über den HTTP-Port erlauben.

<!--#### The Cons ####-->
#### Die Nachteile ####

<!--The downside of serving your repository over HTTP is that it’s relatively inefficient for the client. It generally takes a lot longer to clone or fetch from the repository, and you often have a lot more network overhead and transfer volume over HTTP than with any of the other network protocols. Because it’s not as intelligent about transferring only the data you need — there is no dynamic work on the part of the server in these transactions — the HTTP protocol is often referred to as a _dumb_ protocol. For more information about the differences in efficiency between the HTTP protocol and the other protocols, see Chapter 9.-->

Die Unterseite vom Servieren von Deinem Repository über HTTP ist, dass es recht ineffizient für den Client ist. Es braucht im Allgemeinen länger zu clonen oder Daten vom Repository zu holen und Du hast oft wesentlich mehr Netzwerk-Overhead und Transfer-Volumen als mit jedem anderen Netzwerk Protokoll. Weil es nicht so intelligent beim Daten-Transfer ist, um nur die benötigten Daten zu übertragen – es gibt keine dynamische Arbeit auf dem Server bei diesen Aktionen – das HTTP Protokoll wird oft als _dummes_ Protokoll bezeichnet. Für mehr Informationen über die Unterschiede bei der Effizienz zwischen dem HTTP Protokoll und den anderen Protokollen: siehe Kapitel 9.

<!--## Getting Git on a Server ##-->
## Git auf einen Server bekommen ##

<!--In order to initially set up any Git server, you have to export an existing repository into a new bare repository — a repository that doesn’t contain a working directory. This is generally straightforward to do.-->
<!--In order to clone your repository to create a new bare repository, you run the clone command with the `-\-bare` option. By convention, bare repository directories end in `.git`, like so:-->

Um zunächst einen beliebigen Git Server einzurichten, musst Du ein existierendes Repository in ein neues einfaches Repository exportieren – ein Repository, dass kein Arbeitsverzeichnis enthält. Das ist im Allgemeinen einfach zu erledigen.
Um zunächst Dein Repository zu klonen, um ein neues einfaches Repository anzulegen, führst Du den Klonen-Befehl mit der `--bare` Option aus. Per Konvention haben einfache Repository Verzeichnisse die Endung `.git`, wie hier:

	$ git clone --bare my_project my_project.git
	Cloning into bare repository 'my_project.git'...
	done.

<!--The output for this command is a little confusing. Since `clone` is basically a `git init` then a `git fetch`, we see some output from the `git init` part, which creates an empty directory. The actual object transfer gives no output, but it does happen. You should now have a copy of the Git directory data in your `my_project.git` directory.-->

Die Ausgabe für diesen Befehl ist etwas verwirrend. Weil `clone` im Grunde ein `git init` und ein `git fetch` ist, sehen wir eine Ausgabe vom `git init`-Teil, der ein leeres Verzeichnis anlegt. Die eigentliche Objekt-Übertragung erzeugt keine Ausgabe, aber sie findet statt. Du solltest jetzt eine Kopie von den Git-Verzeichnis Daten in Deinem `my_project.git` Verzeichnis haben.

<!--This is roughly equivalent to something like-->

Dies ist entsprechend zu etwas wie

	$ cp -Rf my_project/.git my_project.git

<!--There are a couple of minor differences in the configuration file; but for your purpose, this is close to the same thing. It takes the Git repository by itself, without a working directory, and creates a directory specifically for it alone.-->

Es gibt ein paar kleine Unterschiede in der Konfigurationsdatei, aber für Deine Zwecke ist es nahezu dasselbe. Es nimmt das Git-Repository selbst, ohne ein Arbeitsverzeichnis, und erzeugt ein Verzeichnis speziell für es allein.

<!--### Putting the Bare Repository on a Server ###-->
### Inbetriebnahme des einfachen Repository auf einem Server ###

<!--Now that you have a bare copy of your repository, all you need to do is put it on a server and set up your protocols. Let’s say you’ve set up a server called `git.example.com` that you have SSH access to, and you want to store all your Git repositories under the `/opt/git` directory. You can set up your new repository by copying your bare repository over:-->

Jetzt da Du eine einfache Kopie Deines Repository hast, ist alles was Du tun musst das Aufsetzen auf einem Server und das Einrichten Deiner Protokolle. Angenommen, Du hast einen Server mit dem Namen `git.example.com`, zu dem Du SSH-Zugang hast, und Du möchtest alle Deine Git Repositories im Verzeichnis `/opt/git` speichern. Du kannst Dein neues Repository einrichten, indem Du Dein einfaches Repository dorthin kopierst:

	$ scp -r my_project.git user@git.example.com:/opt/git

<!--At this point, other users who have SSH access to the same server which has read-access to the `/opt/git` directory can clone your repository by running-->

An diesem Punkt können andere Benutzer, die SSH-Zugang zu dem gleichen Server und Lesezugriff auf das `/opt/git` Verzeichnis haben, Dein Repository klonen:

	$ git clone user@git.example.com:/opt/git/my_project.git

<!--If a user SSHs into a server and has write access to the `/opt/git/my_project.git` directory, they will also automatically have push access.  Git will automatically add group write permissions to a repository properly if you run the `git init` command with the `-\-shared` option.-->

Wenn sich ein Benutzer per SSH mit dem Server verbindet und Schreibzugriff zu dem Verzeichnis `/opt/git/my_project.git` hat, wird er automatisch auch Push-Zugriff haben. Git wird automatisch die richtigen Gruppenschreibrechte zu einem Repository hinzufügen, wenn Du den `git init` Befehl mit der `--shared` Option ausführst:

	$ ssh user@git.example.com
	$ cd /opt/git/my_project.git
	$ git init --bare --shared

<!--You see how easy it is to take a Git repository, create a bare version, and place it on a server to which you and your collaborators have SSH access. Now you’re ready to collaborate on the same project.-->

Du siehst wie einfach es ist ein Git Repository zu nehmen, eine einfache Version zu erzeugen, es auf einen Server zu platzieren zu dem Du und Deine Mitarbeiter SSH-Zugriff haben.

<!--It’s important to note that this is literally all you need to do to run a useful Git server to which several people have access — just add SSH-able accounts on a server, and stick a bare repository somewhere that all those users have read and write access to. You’re ready to go — nothing else needed.-->

Es ist wichtig zu beachten, dass dies wortwörtlich alles ist, was Du tun musst, um einen nützlichen Git-Server laufen zu lassen, zu dem mehrere Personen Zugriff haben – füge auf dem Server einfach SSH-fähige Konten und irgendwo ein einfaches Repository hinzu, zu dem alle Benutzer Schreib- und Lesezugriff haben.

<!--In the next few sections, you’ll see how to expand to more sophisticated setups. This discussion will include not having to create user accounts for each user, adding public read access to repositories, setting up web UIs, using the Gitosis tool, and more. However, keep in mind that to collaborate with a couple of people on a private project, all you _need_ is an SSH server and a bare repository.-->

In den nächsten Abschnitten wirst Du sehen, wie man auf anspruchsvollere Konfigurationen erweitert. Diese Diskussion wird beinhalten nicht Benutzerkonten für jeden Benutzer hinzufügen zu müssen, öffentlichen Lese-Zugriff auf Repositories hinzuzufügen, die Einrichtung von Web-UIs, die Benutzung des Gitosis-Tools und weiteres. Aber denke daran, zur Zusammenarbeit mit ein paar Personen an einem privaten Projekt, ist alles was Du _brauchst_ ein SSH-Server und ein einfaches Repository.

<!--### Small Setups ###-->
### Kleine Konfigurationen ###

<!--If you’re a small outfit or are just trying out Git in your organization and have only a few developers, things can be simple for you. One of the most complicated aspects of setting up a Git server is user management. If you want some repositories to be read-only to certain users and read/write to others, access and permissions can be a bit difficult to arrange.-->

Wenn Du eine kleine Ausrüstung hast oder Git gerade in Deinem Unternehmen ausprobierst und nur ein paar Entwickler hast, sind die Dinge einfach für dich. Einer der kompliziertesten Aspekte der Einrichtung eines Git-Servers ist die Benutzerverwaltung. Wenn einige Repositories für bestimmte Benutzer nur lesend zugänglich sein sollen und andere Benutzer Lese/Schreib-Zugriff haben sollen, können Zugriff und Berechtigungen ein bisschen schwierig zu organisieren sein.

<!--#### SSH Access ####-->
#### SSH-Zugriff ####

<!--If you already have a server to which all your developers have SSH access, it’s generally easiest to set up your first repository there, because you have to do almost no work (as we covered in the last section). If you want more complex access control type permissions on your repositories, you can handle them with the normal filesystem permissions of the operating system your server runs.-->

Wenn Du bereits einen Server hast, zu dem alle Entwickler SSH-Zugriff haben, ist es generell einfach, Dein erstes Repository einzurichten, weil Du fast keine Arbeit zu erledigen hast (wie wir im letzen Abschnitt abgedeckt haben). Wenn Du komplexere Zugriffskontroll-Berechtigungen auf Deine Repositories willst, kannst Du diese mit normalen Dateisystem-Berechtigungen des Betriebssystems Deines Servers bewältigen.

<!--If you want to place your repositories on a server that doesn’t have accounts for everyone on your team whom you want to have write access, then you must set up SSH access for them. We assume that if you have a server with which to do this, you already have an SSH server installed, and that’s how you’re accessing the server.-->

Wenn Du Deine Repositories auf einem Server platzieren möchtest, der keine Accounts für jeden aus Deinem Team hat, der Schreibzugriff haben soll, dann musst Du SSH-Zugriff für diese Personen einrichten. Wir nehmen an, wenn Du einen Server hast, mit welchem Du dies tun möchtest, Du bereits einen SSH-Server installiert hast und so greifst Du auf den Server zu.

<!--There are a few ways you can give access to everyone on your team. The first is to set up accounts for everybody, which is straightforward but can be cumbersome. You may not want to run `adduser` and set temporary passwords for every user.-->

Es gibt ein paar Wege allen Mitgliedern Deines Teams Zugriff zu geben. Der erste ist einen Account für jeden einzurichten, was unkompliziert aber mühsam sein kann. Du möchtest vielleicht nicht für jeden Benutzer `adduser` ausführen und ein temporäres Passwort setzen.

<!--A second method is to create a single 'git' user on the machine, ask every user who is to have write access to send you an SSH public key, and add that key to the `~/.ssh/authorized_keys` file of your new 'git' user. At that point, everyone will be able to access that machine via the 'git' user. This doesn’t affect the commit data in any way — the SSH user you connect as doesn’t affect the commits you’ve recorded.-->

Eine zweite Methode ist, einen einzigen ‚git‘-Benutzer auf der Maschine zu erstellen und jeden Benutzer, der Schreibzugriff haben soll, nach einem öffentliche SSH-Schlüssel zu fragen und diesen Schlüssel zu der `~/.ssh/authorized_keys`-Datei Deines neuen ‚git‘ Benutzers hinzuzufügen. Das beeinflusst die Commit-Daten in keiner Weise – der SSH-Benutzer, mit dem Du Dich verbindest, beeinflusst die von Dir aufgezeichneten Commits nicht.

<!--Another way to do it is to have your SSH server authenticate from an LDAP server or some other centralized authentication source that you may already have set up. As long as each user can get shell access on the machine, any SSH authentication mechanism you can think of should work.-->

Ein anderer Weg ist, einen LDAP-Server zur Authentifizierung zu benutzen oder eine andere zentrale Authentifizierungsquelle, die Du vielleicht bereits eingerichtet hast. Solange jeder Benutzer Shell-Zugriff zu der Maschine hat, sollte jede SSH-Authentifizierungsmethode funktionieren, die Du Dir vorstellen kannst.

<!--## Generating Your SSH Public Key ##-->
## Generiere Deinen öffentlichen SSH-Schlüssel ##

<!--That being said, many Git servers authenticate using SSH public keys. In order to provide a public key, each user in your system must generate one if they don’t already have one. This process is similar across all operating systems.-->
<!--First, you should check to make sure you don’t already have a key. By default, a user’s SSH keys are stored in that user’s `~/.ssh` directory. You can easily check to see if you have a key already by going to that directory and listing the contents:-->

Darüber hinaus benutzen viele Git-Server öffentliche SSH-Schlüssel zur Authentifizierung. Um einen öffentlichen Schlüssel bereitzustellen muss jeder Benutzer Deines Systems einen solchen Schlüssel generieren, falls sie noch keinen haben. Dieser Prozess ist bei allen Betriebssystemen ähnlich.
Als erstes solltest Du überprüfen, ob Du nicht schon einen Schlüssel hast. Standardmäßig werden die SSH-Schlüssel der Benutzer in ihrem `~/.ssh`-Verzeichnis gespeichert. Du kannst einfach überprüfen, ob Du einen Schlüssel hast, indem Du in das Verzeichnis gehst und den Inhalt auflistest:

	$ cd ~/.ssh
	$ ls
	authorized_keys2  id_dsa       known_hosts
	config            id_dsa.pub

<!--You’re looking for a pair of files named something and something.pub, where the something is usually `id_dsa` or `id_rsa`. The `.pub` file is your public key, and the other file is your private key. If you don’t have these files (or you don’t even have a `.ssh` directory), you can create them by running a program called `ssh-keygen`, which is provided with the SSH package on Linux/Mac systems and comes with the MSysGit package on Windows:-->

Du suchst nach einem Dateienpaar namens `irgendetwas` und `irgendetwas.pub`, die Datei `irgendetwas` heißt normalerweise `id_dsa` oder `id_rsa`. Die `.pub`-Datei ist Dein öffentlicher Schlüssel, und die andere Datei ist Dein privater Schlüssel. Wenn Du diese Dateien nicht hast (oder gar kein `.ssh`-Verzeichnis hast), kannst Du sie mit dem Ausführen des Programms `ssh-keygen` erzeugen. Das Programm wird mit dem SSH-Paket auf Linux/Mac-Systemen mitgeliefert und kommt mit dem MSysGit-Paket unter Windows:

	$ ssh-keygen
	Generating public/private rsa key pair.
	Enter file in which to save the key (/Users/schacon/.ssh/id_rsa):
	Enter passphrase (empty for no passphrase):
	Enter same passphrase again:
	Your identification has been saved in /Users/schacon/.ssh/id_rsa.
	Your public key has been saved in /Users/schacon/.ssh/id_rsa.pub.
	The key fingerprint is:
	43:c5:5b:5f:b1:f1:50:43:ad:20:a6:92:6a:1f:9a:3a schacon@agadorlaptop.local

<!--First it confirms where you want to save the key (`.ssh/id_rsa`), and then it asks twice for a passphrase, which you can leave empty if you don’t want to type a password when you use the key.-->

Zunächst wird bestätigt, wo Du den Schlüssel speichern möchtest (`.ssh/id_rsa`) und dann wird zweimal nach der Passphrase gefragt, die Du leer lassen kannst, wenn Du kein Passwort bei der Benutzung des Schlüssels eintippen möchtest.

<!--Now, each user that does this has to send their public key to you or whoever is administrating the Git server (assuming you’re using an SSH server setup that requires public keys). All they have to do is copy the contents of the `.pub` file and e-mail it. The public keys look something like this:-->

Jeder Benutzer der dies macht, muss seinen öffentlichen Schlüssel an sich senden oder wer auch immer den Git-Server administriert (angenommen Du benutzt eine SSH-Server Konfiguration, die öffentliche Schlüssel benötigt). Alles was die Benutzer tun müssen ist, den Inhalt der `.pub`-Datei zu kopieren und an Dich per E-Mail zu schicken. Der öffentliche Schlüssel sieht etwa wie folgt aus:

	$ cat ~/.ssh/id_rsa.pub
	ssh-rsa AAAAB3NzaC1yc2EAAAABIwAAAQEAklOUpkDHrfHY17SbrmTIpNLTGK9Tjom/BWDSU
	GPl+nafzlHDTYW7hdI4yZ5ew18JH4JW9jbhUFrviQzM7xlELEVf4h9lFX5QVkbPppSwg0cda3
	Pbv7kOdJ/MTyBlWXFCR+HAo3FXRitBqxiX1nKhXpHAZsMciLq8V6RjsNAQwdsdMFvSlVK/7XA
	t3FaoJoAsncM1Q9x5+3V0Ww68/eIFmb1zuUFljQJKprrX88XypNDvjYNby6vw/Pb0rwert/En
	mZ+AW4OZPnTPI89ZPmVMLuayrD2cE86Z/il8b+gw3r3+1nKatmIkjn2so1d01QraTlMqVSsbx
	NrRFi9wrf+M7Q== schacon@agadorlaptop.local

<!--For a more in-depth tutorial on creating an SSH key on multiple operating systems, see the GitHub guide on SSH keys at `http://github.com/guides/providing-your-ssh-key`.-->

Eine detailliertere Anleitung zur Erstellung eines SSH-Schlüssels unter den verschiedenen Betriebssystemen ist der GitHub-Leitfaden für SSH-Schlüssel unter `http://github.com/guides/providing-your-ssh-key`.

<!--## Setting Up the Server ##-->
## Einrichten des Servers ##

<!--Let’s walk through setting up SSH access on the server side. In this example, you’ll use the `authorized_keys` method for authenticating your users. We also assume you’re running a standard Linux distribution like Ubuntu. First, you create a 'git' user and a `.ssh` directory for that user.-->

Nun kommen wir zur Einrichtung des SSH-Zugangs auf der Server-Seite. In diesem Beispiel verwendest Du die `authorized_keys`-Methode zur Authentifizierung der Benutzer. Wir nehmen auch an, dass Du eine gebräuchliche Linux-Distribution wie Ubuntu verwendest. Zuerst erstellst Du den Benutzer ‚git‘ und ein `.ssh`-Verzeichnis für diesen Benutzer.

	$ sudo adduser git
	$ su git
	$ cd
	$ mkdir .ssh

<!--Next, you need to add some developer SSH public keys to the `authorized_keys` file for that user. Let’s assume you’ve received a few keys by e-mail and saved them to temporary files. Again, the public keys look something like this:-->

Als nächstes ist es nötig, einige öffentliche SSH-Schlüssel der Entwickler zu der `authorized_keys`-Datei des Benutzers hinzuzufügen. Nehmen wir an, dass Du ein paar Schlüssel per E-Mail empfangen hast und diese in temporären Dateien gespeichert hast. Die öffentlichen Schlüssel sehen wieder etwa wie folgt aus:

	$ cat /tmp/id_rsa.john.pub
	ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQCB007n/ww+ouN4gSLKssMxXnBOvf9LGt4L
	ojG6rs6hPB09j9R/T17/x4lhJA0F3FR1rP6kYBRsWj2aThGw6HXLm9/5zytK6Ztg3RPKK+4k
	Yjh6541NYsnEAZuXz0jTTyAUfrtU3Z5E003C4oxOj6H0rfIF1kKI9MAQLMdpGW1GYEIgS9Ez
	Sdfd8AcCIicTDWbqLAcU4UpkaX8KyGlLwsNuuGztobF8m72ALC/nLF6JLtPofwFBlgc+myiv
	O7TCUSBdLQlgMVOFq1I2uPWQOkOWQAHukEOmfjy2jctxSDBQ220ymjaNsHT4kgtZg2AYYgPq
	dAv8JggJICUvax2T9va5 gsg-keypair

<!--You just append them to your `authorized_keys` file:-->

Du hängst sie einfach an Deine `authorized_keys`-Datei an:

	$ cat /tmp/id_rsa.john.pub >> ~/.ssh/authorized_keys
	$ cat /tmp/id_rsa.josie.pub >> ~/.ssh/authorized_keys
	$ cat /tmp/id_rsa.jessica.pub >> ~/.ssh/authorized_keys

<!--Now, you can set up an empty repository for them by running `git init` with the `-\-bare` option, which initializes the repository without a working directory:-->

Jetzt kannst Du einen leeren Ordner für sie anlegen, indem Du den Befehl `git init` mit der Option `--bare` ausführst. Damit wird ein Repository ohne ein Arbeitsverzeichnis erzeugt.

	$ cd /opt/git
	$ mkdir project.git
	$ cd project.git
	$ git --bare init

<!--Then, John, Josie, or Jessica can push the first version of their project into that repository by adding it as a remote and pushing up a branch. Note that someone must shell onto the machine and create a bare repository every time you want to add a project. Let’s use `gitserver` as the hostname of the server on which you’ve set up your 'git' user and repository. If you’re running it internally, and you set up DNS for `gitserver` to point to that server, then you can use the commands pretty much as is:-->

Dann können John, Josie oder Jessica die erste Version ihres Projektes in das Repository hochladen, indem sie es als externes Repository hinzufügen und einen Branch hochladen. Beachte, dass sich bei jeder Projekterstellung jemand mit der Maschine auf eine Shell verbinden muss, um ein einfaches Repository zu erzeugen. Lass uns `gitserver` als Hostnamen des Servers verwenden, auf dem Du den Benutzer ‚git‘ und das Repository eingerichtet hast. Wenn Du den Server intern betreibst und das DNS so eingerichtet hast, dass `gitserver` auf den Server zeigt, dann kannst Du die Befehle ziemlich wie hier benutzen:

	# on Johns computer
	$ cd myproject
	$ git init
	$ git add .
	$ git commit -m 'initial commit'
	$ git remote add origin git@gitserver:/opt/git/project.git
	$ git push origin master

<!--At this point, the others can clone it down and push changes back up just as easily:-->

An diesem Punkt können die anderen das Repository klonen und Änderungen ebenso leicht hochladen:

	$ git clone git@gitserver:/opt/git/project.git
	$ cd project
	$ vim README
	$ git commit -am 'fix for the README file'
	$ git push origin master

<!--With this method, you can quickly get a read/write Git server up and running for a handful of developers.-->

Mit dieser Methode kannst Du schnell für eine Handvoll Entwickler einen Lese/Schreib Git-Server zum Laufen bekommen.

<!--As an extra precaution, you can easily restrict the 'git' user to only doing Git activities with a limited shell tool called `git-shell` that comes with Git. If you set this as your 'git' user’s login shell, then the 'git' user can’t have normal shell access to your server. To use this, specify `git-shell` instead of bash or csh for your user’s login shell. To do so, you’ll likely have to edit your `/etc/passwd` file:-->

Als zusätzliche Vorsichtsmaßnahme kannst Du den Benutzer ‚git‘ so beschränken, dass er nur Git-Aktivitäten mit einem limitierten Shell-Tool namens `git-shell` ausführen kann, dass mit Git kommt. Wenn Du das als Login-Shell des ‚git‘-Benutzers einrichtest, dann hat der Benutzer ‚git‘ keinen normalen Shell-Zugriff auf den Server. Zur Benutzung bestimme `git-shell` anstatt von bash oder csh als Login-Shell Deines Benutzers. Um das zu tun wirst Du wahrscheinlich Deine `/etc/passwd` editieren:

	$ sudo vim /etc/passwd

<!--At the bottom, you should find a line that looks something like this:-->

Am Ende solltest Du eine Zeile finden, die in etwa so aussieht:

	git:x:1000:1000::/home/git:/bin/sh

<!--Change `/bin/sh` to `/usr/bin/git-shell` (or run `which git-shell` to see where it’s installed). The line should look something like this:-->

Ändere `/bin/sh` zu `/usr/bin/git-shell` (oder führe `which git-shell` aus, um zu sehen, wo es installiert ist). Die Zeile sollte in etwa so aussehen:

	git:x:1000:1000::/home/git:/usr/bin/git-shell

<!--Now, the 'git' user can only use the SSH connection to push and pull Git repositories and can’t shell onto the machine. If you try, you’ll see a login rejection like this:-->

Jetzt kann der ‚git‘-Benutzer die SSH-Verbindung nur noch verwenden, um Git-Repositories hochzuladen und herunterzuladen. Der Benutzer kann sich nicht mehr per Shell zur Maschine verbinden. Wenn Du es versuchst, siehst Du eine Login-Ablehnung wie diese:

	$ ssh git@gitserver
	fatal: What do you think I am? A shell?
	Connection to gitserver closed.

<!--## Public Access ##-->
## Öffentlicher Zugang ##

<!--What if you want anonymous read access to your project? Perhaps instead of hosting an internal private project, you want to host an open source project. Or maybe you have a bunch of automated build servers or continuous integration servers that change a lot, and you don’t want to have to generate SSH keys all the time — you just want to add simple anonymous read access.-->

Was ist, wenn Du anonymen Lese-Zugriff zu Deinem Projekt ermöglichen möchtest? Vielleicht möchtest Du ein Open-Source Projekt, anstatt einem privaten, nicht öffentlichen Projekt hosten. Oder Du hast ein paar automatisierte Build-Server oder Continuous Integration Server, die ständig wechseln, und Du möchtest für diese nicht dauernd neue SSH-Schlüssel generieren. Dann wäre es doch schön, wenn ein anonymer Lese-Zugriff zu Deinem Projekt möglich wäre.

<!--Probably the simplest way for smaller setups is to run a static web server with its document root where your Git repositories are, and then enable that `post-update` hook we mentioned in the first section of this chapter. Let’s work from the previous example. Say you have your repositories in the `/opt/git` directory, and an Apache server is running on your machine. Again, you can use any web server for this; but as an example, we’ll demonstrate some basic Apache configurations that should give you an idea of what you might need.-->

Der wahrscheinlich einfachste Weg für kleinere Konfigurationen ist, einen Webserver, in dessen Basisverzeichnis die Git Repositorys liegen, laufen zu lassen und den `post-update` Hook, den wir im ersten Abschnitt dieses Kapitels erwähnt haben, zu aktivieren. Gehen wir vom vorherigen Beispiel aus. Sagen wir, Du hast Deine Repositorys im Verzeichnis `/opt/git` und ein Apache-Server läuft auf Deiner Maschine. Du kannst dafür jeden beliebigen Webserver benutzen, aber in diesem Beispiel demonstrieren wir das Ganze an Hand einer Apache Basis-Konfiguration. Dies sollte Dir eine Vorstellung geben, wie Du es mit dem Webserver Deiner Wahl umsetzen kannst.

<!--First you need to enable the hook:-->

Zuerst musst Du den Hook aktivieren:

	$ cd project.git
	$ mv hooks/post-update.sample hooks/post-update
	$ chmod a+x hooks/post-update

<!--If you’re using a version of Git earlier than 1.6, the `mv` command isn’t necessary — Git started naming the hooks examples with the .sample postfix only recently.-->

Wenn Du eine ältere Git Version als 1.6 benutzt, brauchst Du den `mv`-Befehl nicht auszuführen. Das Namensschema mit der .sample Endung wurde erst bei den neueren Git Versionen eingeführt.

<!--What does this `post-update` hook do? It looks basically like this:-->

Welche Aufgabe hat der `post-update` Hook? Er enthält in etwa folgendes:

	$ cat .git/hooks/post-update
	#!/bin/sh
	#
	# An example hook script to prepare a packed repository for use over
	# dumb transports.
	#
	# To enable this hook, rename this file to "post-update".
	#
	
	exec git-update-server-info

<!--This means that when you push to the server via SSH, Git will run this command to update the files needed for HTTP fetching.-->

Wenn Du via SSH etwas auf den Server hochlädst, wird Git den Befehl `git-update-server-info` ausführen. Dieser Befehl aktualisiert alle Dateien, die benötigt werden, damit das Repository über HTTP geholt (fetch) beziehungsweise geklont werden kann.

<!--Next, you need to add a VirtualHost entry to your Apache configuration with the document root as the root directory of your Git projects. Here, we’re assuming that you have wildcard DNS set up to send `*.gitserver` to whatever box you’re using to run all this:-->

Als nächstes musst Du einen VirtualHost Eintrag zu Deiner Apache-Konfiguration hinzufügen. Das dort angegebene DocumentRoot Verzeichnis muss mit dem Basisverzeichnis Deiner Git Projekte übereinstimmen. In diesem Beispiel gehen wir davon aus, dass ein Wildcard-DNS Eintrag besteht, der dafür sorgt, dass `*.gitserver` auf den Server zeigt, auf dem das Ganze hier läuft:

	<VirtualHost *:80>
	    ServerName git.gitserver
	    DocumentRoot /opt/git
	    <Directory /opt/git/>
	        Order allow, deny
	        allow from all
	    </Directory>
	</VirtualHost>

<!--You’ll also need to set the Unix user group of the `/opt/git` directories to `www-data` so your web server can read-access the repositories, because the Apache instance running the CGI script will (by default) be running as that user:-->

Da die Apache-Instanz, die das CGI-Skript ausführt, standardmäßig unter dem Benutzer `www-data` läuft, musst Du auch die Unix Eigentümer-Gruppe des Verzeichnisses `/opt/git` auf `www-data` setzen. Ansonsten kann Dein Webserver die Repositorys nicht lesen:

	$ chgrp -R www-data /opt/git

<!--When you restart Apache, you should be able to clone your repositories under that directory by specifying the URL for your project:-->

Nach einem Neustart des Apache, solltest Du in der Lage sein, Deine Repositorys innerhalb diesem Verzeichnis zu klonen, indem Du die URL für das jeweilige Projekt angibst.

	$ git clone http://git.gitserver/project.git

<!--This way, you can set up HTTP-based read access to any of your projects for a fair number of users in a few minutes. Another simple option for public unauthenticated access is to start a Git daemon, although that requires you to daemonize the process - we’ll cover this option in the next section, if you prefer that route.-->

Auf diese Art und Weise kannst Du in wenigen Minuten einen HTTP-basierten Lese-Zugriff auf all Deine Projekte für eine große Anzahl von Benutzern ermöglichen. Ein Git Daemon ist eine andere einfache Möglichkeit für einen öffentlichen Zugang. Wenn Du diese Methode bevorzugst, solltest Du Dir den nächsten Abschnitt unbedingt anschauen.

<!--## GitWeb ##-->
## GitWeb ##

<!--Now that you have basic read/write and read-only access to your project, you may want to set up a simple web-based visualizer. Git comes with a CGI script called GitWeb that is commonly used for this. You can see GitWeb in use at sites like `http://git.kernel.org` (see Figure 4-1).-->

Da Du jetzt sowohl einen einfachen Lese- und Schreibzugriff, als auch einen schreibgeschützten Zugang auf Dein Projekt hast, wird es jetzt Zeit eine simple webbasierte Visualisierung dafür einzurichten. Git wird mit einem CGI-Skript namens GitWeb ausgeliefert, welches für diese Aufgabe häufig verwendet wird. Auf Seiten, wie zum Beispiel `http://git.kernel.org`, kannst Du Dir GitWeb in Aktion anschauen (siehe Abbildung 4-1).

<!--Figure 4-1. The GitWeb web-based user interface.-->

Insert 18333fig0401.png
Abbildung 4-1. Die webbasierte Benutzeroberfläche von GitWeb.

<!--If you want to check out what GitWeb would look like for your project, Git comes with a command to fire up a temporary instance if you have a lightweight server on your system like `lighttpd` or `webrick`. On Linux machines, `lighttpd` is often installed, so you may be able to get it to run by typing `git instaweb` in your project directory. If you’re running a Mac, Leopard comes preinstalled with Ruby, so `webrick` may be your best bet. To start `instaweb` with a non-lighttpd handler, you can run it with the `-\-httpd` option.-->

Wenn Du Dir anschauen möchtest, wie GitWeb bei Deinem Projekt ausschauen würde, gibt es dafür eine einfache Möglichkeit. Wenn Du einen einfachen Webserver wie `lighttpd` or `webrick` auf Deinem Server installiert hast, kannst Du mit einem in Git integrierten Kommando eine temporäre Instanz von GitWeb starten. Da `lighttpd` auf vielen Linux Rechnern bereits installiert ist, kannst Du versuchen ihn zum Laufen zu bringen, indem Du das Kommando  `git instaweb` in Deinem Projektverzeichnis ausführst. Bei Mac OS X 10.5 alias Leopard ist Ruby bereits vorinstalliert. Falls Du also einen Mac verwendest, solltest Du es mal mit `webrick` versuchen. Um `instaweb` mit einem anderen Webserver als `lighthttpd` zu starten, kannst Du an den Befehl die Option `--httpd` anhängen.

	$ git instaweb --httpd=webrick
	[2009-02-21 10:02:21] INFO  WEBrick 1.3.1
	[2009-02-21 10:02:21] INFO  ruby 1.8.6 (2008-03-03) [universal-darwin9.0]

<!--That starts up an HTTPD server on port 1234 and then automatically starts a web browser that opens on that page. It’s pretty easy on your part. When you’re done and want to shut down the server, you can run the same command with the `-\-stop` option:-->

Dadurch wird auf dem Port 1234 ein HTPPD Server gestartet. Gleichzeitig wird automatisch Dein Webbrowser mit der entsprechenden Seite geöffnet. Das Ganze gestaltet sich also ziemlich einfach. Wenn Du dann fertig bist und den Server wieder beenden willst, kannst Du das gleiche Kommando mit der Option `--stop` ausführen:

	$ git instaweb --httpd=webrick --stop

<!--If you want to run the web interface on a server all the time for your team or for an open source project you’re hosting, you’ll need to set up the CGI script to be served by your normal web server. Some Linux distributions have a `gitweb` package that you may be able to install via `apt` or `yum`, so you may want to try that first. We’ll walk though installing GitWeb manually very quickly. First, you need to get the Git source code, which GitWeb comes with, and generate the custom CGI script:-->

Wenn Du das Web Interface für Dein Team oder ein von Dir gehostetes Open-Source Projekt dauerhaft zur Verfügung stellen willst, musst Du das CGI-Skript so einrichten, dass es von Deinem normalen Webserver zur Verfügung gestellt werden kann. Bei manchen Linux Distributionen ist ein `gitweb` Paket enthalten, welches Du via `apt` oder `yum` installieren kannst. Vielleicht probierst Du das einfach mal zuerst aus. Wir werden hier nämlich die manuelle Installation von GitWeb nur kurz überfliegen. Zum Starten benötigst Du den Git Quellcode. Dort ist GitWeb enthalten und Du kannst damit Dein angepasstest CGI-Skript erstellen:

	$ git clone git://git.kernel.org/pub/scm/git/git.git
	$ cd git/
	$ make GITWEB_PROJECTROOT="/opt/git" \
	        prefix=/usr gitweb
	$ sudo cp -Rf gitweb /var/www/

<!--Notice that you have to tell the command where to find your Git repositories with the `GITWEB_PROJECTROOT` variable. Now, you need to make Apache use CGI for that script, for which you can add a VirtualHost:-->

Bitte beachte, dass Du dem Kommando angeben musst, wo sich Deine Git Repositorys befinden. Dazu wird die `GITWEB_PROJECTROOT` Variable verwendet. Jetzt musst Du den Apache noch so konfigurieren, dass er CGI für das Skript verwendet. Dazu kannst Du einen VirtualHost einrichten:

	<VirtualHost *:80>
	    ServerName gitserver
	    DocumentRoot /var/www/gitweb
	    <Directory /var/www/gitweb>
	        Options ExecCGI +FollowSymLinks +SymLinksIfOwnerMatch
	        AllowOverride All
	        order allow,deny
	        Allow from all
	        AddHandler cgi-script cgi
	        DirectoryIndex gitweb.cgi
	    </Directory>
	</VirtualHost>

<!--Again, GitWeb can be served with any CGI capable web server; if you prefer to use something else, it shouldn’t be difficult to set up. At this point, you should be able to visit `http://gitserver/` to view your repositories online, and you can use `http://git.gitserver` to clone and fetch your repositories over HTTP.-->

Ich möchte Dich noch einmal darauf hinweisen, dass GitWeb prinzipiell mit jedem CGI-fähigen Webserver funktioniert. Wenn Du einen anderen Webserver bevorzugst, sollte es also kein Problem sein, GitWeb dafür einzurichten. Nach einem Neustart Deines Apache solltest Du jetzt in der Lage sein, Deine Repositorys über die Adresse `http://gitserver/` in GitWeb anzuschauen. Gleichzeitig kannst Du über `http://git.gitserver` Deine Repositorys per HTTP klonen und abholen (im Sinne eines Fetch).

<!--## Gitosis ##-->
## Gitosis ##

<!--Keeping all users’ public keys in the `authorized_keys` file for access works well only for a while. When you have hundreds of users, it’s much more of a pain to manage that process. You have to shell onto the server each time, and there is no access control — everyone in the file has read and write access to every project.-->

Das manuelle Verwalten der öffentlichen Benutzerschlüssel in der Datei `authorized_keys` ist auf Dauer nicht sinnvoll. Wenn Du hunderte von Benutzer verwalten musst, wird dieser Prozess noch viel schwieriger und macht keinen Spaß mehr. Du musst jedes mal über die Shell auf Deinen Server zugreifen und es gibt auch keine Zugriffsberechtigungen. Jeder der in der `authorized_keys` Datei eingetragen ist, hat auf jedes Projekt Lese- und Schreibzugriff.

<!--At this point, you may want to turn to a widely used software project called Gitosis. Gitosis is basically a set of scripts that help you manage the `authorized_keys` file as well as implement some simple access controls. The really interesting part is that the UI for this tool for adding people and determining access isn’t a web interface but a special Git repository. You set up the information in that project; and when you push it, Gitosis reconfigures the server based on that, which is cool.-->

Vielleicht ist es deshalb sinnvoll, dass Du Dich mit dem weit verbreiteten Projekt Gitosis beschäftigst. Gitosis ist im Grunde eine Sammlung von Skripts, die Dir dabei helfen, sowohl die Datei `authorized_keys` zu verwalten, als auch ein paar einfache Zugriffsberechtigungen zu setzen. Das wirklich interessante an diesem Werkzeug ist es, dass die Benutzeroberfläche zum Hinzufügen von Benutzern oder Setzen von Berechtigungen, kein Web Interface ist, sondern ein spezielles Git Repository. Die ganzen Informationen werden in diesem Projekt verwaltet und sobald dieses gepusht wird, konfiguriert Gitosis den Server auf Basis dieser Daten entsprechend um. Das ist ziemlich cool, oder?

<!--Installing Gitosis isn’t the simplest task ever, but it’s not too difficult. It’s easiest to use a Linux server for it — these examples use a stock Ubuntu 8.10 server.-->

Die Installation von Gitosis ist nicht einfach, aber auf jeden Fall machbar. Am einfachsten gestaltet sich die Installation auf einem Linux Server. In unserem Beispiel verwenden wir dafür einen Standard Ubuntu Server in der Version 8.10.

<!--Gitosis requires some Python tools, so first you have to install the Python setuptools package, which Ubuntu provides as python-setuptools:-->

Gitosis setzt einige Python Werkzeuge voraus. Deshalb solltest Du zuerst das Python Setuptools Paket installieren. Unter Ubuntu wird es als python-setuptools zur Verfügung gestellt:

	$ apt-get install python-setuptools

<!--Next, you clone and install Gitosis from the project’s main site:-->

Danach klonst Du Gitosis von der offiziellen Projektseite und installierst es:

	$ git clone https://github.com/tv42/gitosis.git
	$ cd gitosis
	$ sudo python setup.py install

<!--That installs a couple of executables that Gitosis will use. Next, Gitosis wants to put its repositories under `/home/git`, which is fine. But you have already set up your repositories in `/opt/git`, so instead of reconfiguring everything, you create a symlink:-->

Einige ausführbare Dateien, welche von Gitosis benötigt werden, werden mit dem Skript installiert. Außerdem will Gitosis seine Repositorys im Verzeichnis `/home/git` ablegen. Das ist auch ok so. Allerdings liegen unsere Repositorys bereits im Verzeichnis `/opt/git`, aber anstatt alles umzukonfigurieren, erstellst Du einfach eine symbolische Verknüpfung (symlink):

	$ ln -s /opt/git /home/git/repositories

<!--Gitosis is going to manage your keys for you, so you need to remove the current file, re-add the keys later, and let Gitosis control the `authorized_keys` file automatically. For now, move the `authorized_keys` file out of the way:-->

Gitosis wird Deine Schlüssel für Dich verwalten. Deshalb musst Du die bereits vorhandene Datei `authorized_keys` entfernen, die Schlüssel später wieder hinzufügen und Gitosis die automatisierte Verarbeitung dieser Datei überlassen. Zuerst musst Du also die Datei `authorized_keys` aus dem Weg räumen:

	$ mv /home/git/.ssh/authorized_keys /home/git/.ssh/ak.bak

<!--Next you need to turn your shell back on for the 'git' user, if you changed it to the `git-shell` command. People still won’t be able to log in, but Gitosis will control that for you. So, let’s change this line in your `/etc/passwd` file-->

Wenn Du für den Benutzer ‚git‘ die Shell auf die `git-shell` gesetzt hast (siehe Kapitel 4.4), musst Du die normale Shell für diesen Benutzer wieder aktivieren. Den Benutzern wird es danach immer noch nicht möglich sein sich einzuloggen, dafür sorgt Gitosis. Ändere also in Deiner `/etc/passwd` die Zeile

	git:x:1000:1000::/home/git:/usr/bin/git-shell

<!--back to this:-->

zurück in folgendes:

	git:x:1000:1000::/home/git:/bin/sh

<!--Now it’s time to initialize Gitosis. You do this by running the `gitosis-init` command with your personal public key. If your public key isn’t on the server, you’ll have to copy it there:-->

Jetzt wird es Zeit Gitosis zu initialisieren. Dafür musst Du das Kommando `gitosis-init` mit Deinem öffentlichen Schlüssel ausführen. Wenn sich Dein öffentlicher Schlüssel nicht auf dem Server befindet, musst Du ihn vorher dorthin kopieren:

	$ sudo -H -u git gitosis-init < /tmp/id_dsa.pub
	Initialized empty Git repository in /opt/git/gitosis-admin.git/
	Reinitialized existing Git repository in /opt/git/gitosis-admin.git/

<!--This lets the user with that key modify the main Git repository that controls the Gitosis setup. Next, you have to manually set the execute bit on the `post-update` script for your new control repository.-->

Dem Benutzer, mit dem hier angegeben Schlüssel, ist es jetzt möglich, das Git Repository, mit dem Gitosis konfiguriert wird, zu modifizieren. Als nächstes musst Du manuell das Execute-Bit für das Skript `post-update` in Deinem neuen „Verwaltungs“-Repository setzen.

	$ sudo chmod 755 /opt/git/gitosis-admin.git/hooks/post-update

<!--You’re ready to roll. If you’re set up correctly, you can try to SSH into your server as the user for which you added the public key to initialize Gitosis. You should see something like this:-->

Jetzt kann es losgehen. Wenn Du alles richtig eingerichtet hast, kannst Du jetzt versuchen Dich über SSH einzuloggen. Du musst dafür den Benutzer verwenden, dem der öffentliche Schlüssel gehört, den Du in den vorherigen Schritten hinzugefügt hast. Du solltest dann in etwa folgende Ausgabe erhalten:

	$ ssh git@gitserver
	PTY allocation request failed on channel 0
	ERROR:gitosis.serve.main:Need SSH_ORIGINAL_COMMAND in environment.
	  Connection to gitserver closed.

<!--That means Gitosis recognized you but shut you out because you’re not trying to do any Git commands. So, let’s do an actual Git command — you’ll clone the Gitosis control repository:-->

Das bedeutet das Gitosis Dich als Benutzer kennt, aber Dich ausschließt, weil Du nicht versuchst ein Git Kommando auszuführen. Lass uns also ein Git Kommando ausprobieren. Wir klonen dazu das Verwaltungsrepository von Gitosis:

	# on your local computer
	$ git clone git@gitserver:gitosis-admin.git

<!--Now you have a directory named `gitosis-admin`, which has two major parts:-->

Jetzt hast Du auf Deinem Rechner ein Verzeichnis mit dem Namen `gitosis-admin`. Dieses besteht aus hauptsächlich zwei Teilen:

	$ cd gitosis-admin
	$ find .
	./gitosis.conf
	./keydir
	./keydir/scott.pub

<!--The `gitosis.conf` file is the control file you use to specify users, repositories, and permissions. The `keydir` directory is where you store the public keys of all the users who have any sort of access to your repositories — one file per user. The name of the file in `keydir` (in the previous example, `scott.pub`) will be different for you — Gitosis takes that name from the description at the end of the public key that was imported with the `gitosis-init` script.-->

Mit Hilfe der Datei `gitosis.conf` kannst Du die Benutzer, Repositorys und die Zugriffsrechte festlegen. Im Verzeichnis `keydir` werden alle öffentlichen Schlüssel der Benutzer abgelegt, die einen Zugriff auf Deine Repositorys haben sollen und zwar für jeden Benutzer eine einzelne Datei. Der Name der Datei, die sich jetzt bereits im `keydir` Verzeichnis befindet (in diesem Beispiel ist es `scott.pub`), wird bei Dir anders lauten. Die Beschreibung, die sich am Ende des öffentlichen Schlüssels befindet, der beim initialen Import mit dem Skript `gitosis-init` angegeben wurde, wird von Gitosis als Dateiname verwendet.

<!--If you look at the `gitosis.conf` file, it should only specify information about the `gitosis-admin` project that you just cloned:-->

Wenn Du Dir die Datei `gitosis.conf` anschaust, sollten lediglich Daten für das Projekt `gitosis-admin` enthalten sein. `gitosis-admin` ist das Projekt, welches Du gerade geklont hast.

	$ cat gitosis.conf
	[gitosis]

	[group gitosis-admin]
	members = scott
	writable = gitosis-admin

<!--It shows you that the 'scott' user — the user with whose public key you initialized Gitosis — is the only one who has access to the `gitosis-admin` project.-->

In dieser Datei wird Dir angezeigt, dass nur der Benutzer ‚scott‘ — das ist der Anwender mit dessen öffentlichen Schlüssel Gitosis initialisiert wurde — Zugriff auf das Projekt `gitosis-admin` hat.

<!--Now, let’s add a new project for you. You’ll add a new section called `mobile` where you’ll list the developers on your mobile team and projects that those developers need access to. Because 'scott' is the only user in the system right now, you’ll add him as the only member, and you’ll create a new project called `iphone_project` to start on:-->

Lass uns jetzt für Dich ein neues Projekt hinzufügen. Dazu fügen wir eine neue Sektion mit dem Namen `mobile` hinzu, in der wir alle Teammitglieder aus dem „Mobile“-Team und alle Repositorys, die von Ihnen benötigt werden, auflisten. Da ‚scott‘ bisher der einzige Benutzer im System ist, werden wir ihn als einziges Teammitglied hinzufügen. Das erste von uns erzeugte Projekt mit dem wir anfangen, nennt sich `iphone_project`:

	[group mobile]
	members = scott
	writable = iphone_project

<!--Whenever you make changes to the `gitosis-admin` project, you have to commit the changes and push them back up to the server in order for them to take effect:-->

Immer wenn Du Änderungen im Projekt `gitosis-admin` durchführst, musst Du diese Änderungen auch einchecken und auf den Server pushen, damit diese auch eine Wirkung zeigen:

	$ git commit -am 'add iphone_project and mobile group'
	[master 8962da8] add iphone_project and mobile group
	 1 file changed, 4 insertions(+)
	$ git push origin master
	Counting objects: 5, done.
	Compressing objects: 100% (3/3), done.
	Writing objects: 100% (3/3), 272 bytes | 0 bytes/s, done.
	Total 3 (delta 0), reused 0 (delta 0)
	To git@gitserver:gitosis-admin.git
	   fb27aec..8962da8  master -> master

<!--You can make your first push to the new `iphone_project` project by adding your server as a remote to your local version of the project and pushing. You no longer have to manually create a bare repository for new projects on the server — Gitosis creates them automatically when it sees the first push:-->

Du kannst Deinen ersten Push für das neue Projekt `iphone_project` ausführen, indem Du Deinen Server als Remote zu Deinem lokalen Repository hinzufügst und dann auf diesen pushst. Ab jetzt musst Du auf Deinem Server nicht mehr manuell ein Bare Repository für neue Projekte erstellen. Gitosis übernimmt diese Aufgabe für Dich, sobald es den ersten Push erhält:

	$ git remote add origin git@gitserver:iphone_project.git
	$ git push origin master
	Initialized empty Git repository in /opt/git/iphone_project.git/
	Counting objects: 3, done.
	Writing objects: 100% (3/3), 230 bytes | 0 bytes/s, done.
	Total 3 (delta 0), reused 0 (delta 0)
	To git@gitserver:iphone_project.git
	 * [new branch]      master -> master

<!--Notice that you don’t need to specify the path (in fact, doing so won’t work), just a colon and then the name of the project — Gitosis finds it for you.-->

Beachte, dass Du den Pfad nicht angeben musst (es ist sogar so, wenn Du ihn angibst, wird es nicht funktionieren). Gib lediglich ein Doppelpunkt gefolgt vom Namen des Projekts an. Das reicht Gitosis aus, um den richtigen Pfad für Dich zu finden.

<!--You want to work on this project with your friends, so you’ll have to re-add their public keys. But instead of appending them manually to the `~/.ssh/authorized_keys` file on your server, you’ll add them, one key per file, into the `keydir` directory. How you name the keys determines how you refer to the users in the `gitosis.conf` file. Let’s re-add the public keys for John, Josie, and Jessica:-->

Da Du an diesem Projekt nicht alleine, sondern mit Deinen Freunden, arbeiten willst, musst Du deren öffentliche Schlüssel wieder hinzufügen. Aber anstatt diese manuell in der Datei `~/.ssh/authorized_keys` auf Deinem Server einzutragen, fügst Du jeden Schlüssel als einzelne Datei in das Verzeichnis `keydir` hinzu. Der Name der Dateien entspricht den gleichen Namen, auf die Du in der Datei `gitosis.conf` referenzierst. Lass uns für John, Josie und Jessica die öffentlichen Schlüssel hinzufügen:

	$ cp /tmp/id_rsa.john.pub keydir/john.pub
	$ cp /tmp/id_rsa.josie.pub keydir/josie.pub
	$ cp /tmp/id_rsa.jessica.pub keydir/jessica.pub

<!--Now you can add them all to your 'mobile' team so they have read and write access to `iphone_project`:-->

Damit diese Personen Lese- und Schreibzugriff auf das Projekt `iphone_project` haben, musst Du sie dem ‚mobile‘-Team hinzufügen:

	[group mobile]
	members = scott john josie jessica
	writable = iphone_project

<!--After you commit and push that change, all four users will be able to read from and write to that project.-->

Nachdem Du diese Änderung commitet und gepusht hast, können alle vier Benutzer dieses Projekt lesen und schreiben.

<!--Gitosis has simple access controls as well. If you want John to have only read access to this project, you can do this instead:-->

Mit Gitosis kann man auch einfache Zugriffsberechtigungen setzen. Wenn John nur Lesezugriff zum Projekt haben soll, dann kannst Du stattdessen folgendes angeben:

	[group mobile]
	members = scott josie jessica
	writable = iphone_project

	[group mobile_ro]
	members = john
	readonly = iphone_project

<!--Now John can clone the project and get updates, but Gitosis won’t allow him to push back up to the project. You can create as many of these groups as you want, each containing different users and projects. You can also specify another group as one of the members (using `@` as prefix), to inherit all of its members automatically:-->

Mit dieser Konfiguration kann John das Projekt klonen und kann neue Stände herunterladen, aber Gitosis wird jeden Push von ihm ablehnen. Du kannst beliebig viele Gruppen angeben, die jeweils unterschiedliche Benutzer und Projekte enthalten. Ebenso ist es möglich eine andere Gruppe als Mitglied hinzuzufügen damit deren Teammitglieder automatisch miteinbezogen werden. Dazu musst Du bei der Gruppe `@` als Präfix angeben:

	[group mobile_committers]
	members = scott josie jessica

	[group mobile]
	members   = @mobile_committers
	writable  = iphone_project

	[group mobile_2]
	members   = @mobile_committers john
	writable  = another_iphone_project

<!--If you have any issues, it may be useful to add `loglevel=DEBUG` under the `[gitosis]` section. If you’ve lost push access by pushing a messed-up configuration, you can manually fix the file on the server under `/home/git/.gitosis.conf` — the file from which Gitosis reads its info. A push to the project takes the `gitosis.conf` file you just pushed up and sticks it there. If you edit that file manually, it remains like that until the next successful push to the `gitosis-admin` project.-->

Solltest Du Probleme mit Gitosis haben, hilft es Dir vielleicht, wenn Du den Eintrag `loglevel=DEBUG` in der Sektion `[gitosis]` hinzufügst. Wenn Du keinen Push auf das Verwaltungsrepository ausführen kannst und Du Dich damit ausgeschlossen hast, kannst Du die Konfiguration unter `/home/git/.gitosis.conf` manuell bearbeiten. Diese Datei verwendet Gitosis als Konfigurationsdatei. Bei einem Push wird diese durch die im Verwaltungsrepository enthaltene Datei ersetzt. Wenn Du diese Datei also manuell änderst, bleibt diese bis zum nächsten erfolgreichen Push auf das Projekt `gitosis-admin` bestehen.

<!--## Gitolite ##-->
## Gitolite ##

<!--This section serves as a quick introduction to Gitolite, and provides basic installation and setup instructions.  It cannot, however, replace the enormous amount of [documentation][gltoc] that Gitolite comes with.  There may also be occasional changes to this section itself, so you may also want to look at the latest version [here][gldpg].-->

In diesem Abschnitt werde ich einen kurzen Einblick in Gitolite geben und die Basisinstallation und Konfiguration besprechen. Jedoch kann meine kurze Einführung nicht die ausführliche [Dokumentation][gltoc], die Gitolite bietet, ersetzen. Es könnte sein, dass es gelegentlich Änderungen an diesem Abschnitt gibt, deshalb solltest Du auch einen Blick auf die [aktuellste Version][gldpg] wagen.

[gldpg]: http://sitaramc.github.com/gitolite/progit.html
[gltoc]: http://sitaramc.github.com/gitolite/master-toc.html

<!--Gitolite is an authorization layer on top of Git, relying on `sshd` or `httpd` for authentication.  (Recap: authentication is identifying who the user is, authorization is deciding if he is allowed to do what he is attempting to).-->

Gitolite ist als Schicht für die Zugriffsberechtigung oberhalb von Git angeordnet und verwendet `sshd` oder `httpd`  zur Authentifikation. (Kurze Wiederholung: Bei der Authentifizierung wird der Benutzer identifiziert und die Zugriffsberechtigung entscheidet, ob der Benutzer die gewünschte Operation ausführen darf oder nicht).

<!--Gitolite allows you to specify permissions not just by repository, but also by branch or tag names within each repository.  That is, you can specify that certain people (or groups of people) can only push certain "refs" (branches or tags) but not others.-->

Gitolite ermöglicht es Dir die Berechtigungen auf Repository Ebene festzulegen, erlaubt aber zusätzlich auch die Berechtigung auf Ebene von Branches oder Tags innerhalb eines Repositorys zu definieren. Das bedeutet also, dass Du festlegen kannst, dass bestimmte Leute (oder eine Gruppe von Leuten) nur bestimmte „refs“ (Branches oder Tags) pushen können, andere Personen sollen das wiederum nicht können.

<!--### Installing ###-->
### Installation ###

<!--Installing Gitolite is very easy, even if you don’t read the extensive documentation that comes with it.  You need an account on a Unix server of some kind.  You do not need root access, assuming Git, Perl, and an OpenSSH compatible SSH server are already installed.  In the examples below, we will use the `git` account on a host called `gitserver`.-->

Auch ohne Studium der ausführlichen Dokumentation, die Gitolite beiliegt, gestaltet sich die Installtion sehr einfach. Du benötigst dazu einen Account auf irgendeiner Art von Unix Server. Du brauchst keine Root-Rechte, vorausgesetzt Git, Perl und ein OpenSSH kompatibler SSH Server sind bereits installiert. In unserem Beispiel verwenden wir den Benutzer `git` auf einem Host mit dem Namen `gitserver`.

<!--Gitolite is somewhat unusual as far as "server" software goes — access is via SSH, and so every userid on the server is a potential "gitolite host".  We will describe the simplest install method in this article; for the other methods please see the documentation.-->

Gitolite ist in Bezug auf „Server“-Software ein wenig ungewöhnlich — der Zugriff erfolgt per SSH und somit ist jede auf dem Server vorhandene User-Id ein potentieller „gitolite host“. In unserem Beispiel werden wir die einfachste Methode der Installation beschreiben. Die anderen Möglichkeiten können der Dokumentation entnommen werden.

<!--To begin, create a user called `git` on your server and login to this user.  Copy your SSH public key (a file called `~/.ssh/id_rsa.pub` if you did a plain `ssh-keygen` with all the defaults) from your workstation, renaming it to `<yourname>.pub` (we'll use `scott.pub` in our examples).  Then run these commands:-->

Zu Beginn legst Du auf Deinem Server einen Benutzer mit dem Namen `git` an und loggst Dich mit diesem ein. Danach kopierst Du Deinen öffentlichen SSH Schlüssel (die Datei lautet `~/.ssh/id_rsa.pub`, falls Du `ssh-keygen` mit den Standardoptionen ausgeführt hast) von Deiner Workstation auf den Server und nennst ihn entsprechend dem Schema `<yourname>.pub` um (in unserem Beispiel verwenden wir die Datei `scott.pub`). Danach führst Du die folgenden Kommandos aus:

	$ git clone git://github.com/sitaramc/gitolite
	$ gitolite/install -ln
	    # assumes $HOME/bin exists and is in your $PATH
	$ gitolite setup -pk $HOME/scott.pub

<!--That last command creates new Git repository called `gitolite-admin` on the server.-->

Der letzte Befehl erzeugt ein neues Git Repository mit dem Namen `gitolite-admin` auf Deinem Server.

<!--Finally, back on your workstation, run `git clone git@gitserver:gitolite-admin`. And you’re done!  Gitolite has now been installed on the server, and you now have a brand new repository called `gitolite-admin` in your workstation.  You administer your Gitolite setup by making changes to this repository and pushing.-->

Zum Abschluss musst Du auf Deiner Workstation den Befehl `git clone git@gitserver:gitolite-admin` ausführen. Jetzt bist Du im Prinzip fertig. Gitolite ist nun auf Deinem Server installiert und auf Deiner Workstation liegt das neue Repository `gitolite-admin` vor. Du kannst Gitolite nun administrieren, indem Du Änderungen an diesem Repository ausführst und zurück auf den Server pushst.

<!--### Customising the Install ###-->
### Benutzerdefinierte Installation ###

<!--While the default, quick, install works for most people, there are some ways to customise the install if you need to.  Some changes can be made simply by editing the rc file, but if that is not sufficient, there’s documentation on customising Gitolite.-->

Obwohl die schnelle Standardinstallation für die meisten Leute ausreicht, gibt es ein paar Möglichkeiten die Installation an Deine Gegebenheiten anzupassen, falls Du dies für nötig hältst. Teilweise reicht es, die rc Datei zu bearbeiten. Sollte das nicht ausreichen, gibt es genügend Dokumentation, die beschreibt, wie Gitolite angepasst werden kann.

<!--### Config File and Access Control Rules ###-->
### Konfigurationsdateien und Regeln für die Zugangskontrolle ###

<!--Once the install is done, you switch to the `gitolite-admin` clone you just made on your workstation, and poke around to see what you got:-->

Nachdem die Installation abgeschlossen ist, wechselst Du in den `gitolite-admin` Klon auf Deiner Workstation und stöberst dort am besten ein wenig herum:

	$ cd ~/gitolite-admin/
	$ ls
	conf/  keydir/
	$ find conf keydir -type f
	conf/gitolite.conf
	keydir/scott.pub
	$ cat conf/gitolite.conf

	repo gitolite-admin
	    RW+                 = scott

	repo testing
	    RW+                 = @all

<!--Notice that "scott" (the name of the pubkey in the `gitolite setup` command you used earlier) has read-write permissions on the `gitolite-admin` repository as well as a public key file of the same name.-->

Es ist wichtig anzumerken, dass „scott“ (das entspricht dem Name des öffentlichen Schlüssel, den Du beim Ausführen des `gitolite setup` Kommandos angegeben hast) Lese- und Schreibzugriff auf das `gitolite-admin` Repository hat. Zusätzlich existiert eine Datei mit dem gleichen Namen. Diese beinhaltet den öffentlichen Schlüssel.

<!--Adding users is easy.  To add a user called "alice", obtain her public key, name it `alice.pub`, and put it in the `keydir` directory of the clone of the `gitolite-admin` repo you just made on your workstation.  Add, commit, and push the change, and the user has been added.-->

Neue Benutzer hinzuzufügen gestaltet sich einfach. Um einen neuen Anwender mit dem Namen „alice“ hinzuzufügen, benötigst Du ihren öffentlichen Schlüssel. Diesen nennst Du in `alice.pub` um und legst ihn im Verzeichnis `keydir` Deines geklonten `gitolite-admin` Repositorys auf Deiner Workstation ab. Danach stagst Du die Änderungen, commitest diese und pushst sie auf den Server und voi­là, der Benutzer wurde hinzugefügt.

<!--The config file syntax for Gitolite is well documented, so we’ll only mention some highlights here.-->

Die Syntax der Gitolite Konfigurationsdateien ist gut dokumentiert, deshalb gehen wir hier nur auf die wichtigsten Details ein.

<!--You can group users or repos for convenience.  The group names are just like macros; when defining them, it doesn’t even matter whether they are projects or users; that distinction is only made when you *use* the "macro".-->

Einzelne Benutzer oder Repositorys können zur besseren Verwaltung zu Gruppen zusammengefasst werden. Die Gruppennamen verhalten sich wie Makros. Beim Anlegen ist es unabhängig, ob es sich um Projekte oder Benutzer handelt. Diese Festlegung wird erst getroffen, wenn diese „Makros“ verwendet werden.

	@oss_repos      = linux perl rakudo git gitolite
	@secret_repos   = fenestra pear

	@admins         = scott
	@interns        = ashok
	@engineers      = sitaram dilbert wally alice
	@staff          = @admins @engineers @interns

<!--You can control permissions at the "ref" level.  In the following example, interns can only push the "int" branch.  Engineers can push any branch whose name starts with "eng-", and tags that start with "rc" followed by a digit.  And the admins can do anything (including rewind) to any ref.-->

Du kannst die Berechtigungen auf „ref“-Ebene (Branches und Tags) festlegen. Im folgenden Beispiel darf die Gruppe „interns“ nur auf den „int“ Branch pushen. Die Benutzer der Gruppe „engineers“ können jeden Branch pushen, der mit dem Prefix „eng-“ beginnt. Zusätzlich kann diese Gruppe jeden Tag, mit dem Namen „rc“, gefolgt von einer einzelnen Zahl, pushen. Die Benutzer der Gruppe „admins“ können jede Operation für jeden „ref“ durchführen (inklusive Rewind-Operationen).

	repo @oss_repos
	    RW  int$                = @interns
	    RW  eng-                = @engineers
	    RW  refs/tags/rc[0-9]   = @engineers
	    RW+                     = @admins

<!--The expression after the `RW` or `RW+` is a regular expression (regex) that the refname (ref) being pushed is matched against.  So we call it a "refex"!  Of course, a refex can be far more powerful than shown here, so don’t overdo it if you’re not comfortable with Perl regexes.-->

Der Ausdruck hinter `RW` oder `RW+` ist ein regulärer Ausdruck (Regex), gegen den die Referenzen (ref), die gepusht werden, verglichen werden. Wir nennen das auch „Refex“. Mit einem solchen Refex hat man ein sehr mächtiges Werkzeug an der Hand und kann noch viel mehr machen, als hier aufgezeigt ist. Aus diesem Grund solltest Du es aber damit auch nicht übertreiben, wenn Du mit den regulären Ausdrücken aus Perl nicht vertraut bist.

<!--Also, as you probably guessed, Gitolite prefixes `refs/heads/` as a syntactic convenience if the refex does not begin with `refs/`.-->

Wie Du vielleicht bereits vermutest hast, stellt Gitolite den Ausdruck `refs/heads/` den Refex voran, wenn diese nicht mit `refs/` beginnen.

<!--An important feature of the config file’s syntax is that all the rules for a repository need not be in one place.  You can keep all the common stuff together, like the rules for all `oss_repos` shown above, then add specific rules for specific cases later on, like so:-->

Ein wichtige Eigenschaft der Syntax der Konfigurationsdatei ist, dass nicht alle Regeln für ein Repository an einer gemeinsamen Stelle festgehalten werden müssen. Du kannst die ganzen allgemeingültigen Dinge, wie zum Beispiel die oben gezeigten Regeln für alle `oss_repos`, an einer Stelle zusammenfassen und später dann spezifische Regeln für die einzelnen Fälle festlegen. Zum Beispiel folgendermaßen:

	repo gitolite
	    RW+                     = sitaram

<!--That rule will just get added to the ruleset for the `gitolite` repository.-->

Diese Regel gehört dann zum Regelsatz des `gitolite` Repository.

<!--At this point you might be wondering how the access control rules are actually applied, so let’s go over that briefly.-->

An dieser Stelle fragst Du Dich vielleicht, wie die Zugriffsregeln eigentlich angewandt werden. Lass uns das kurz anschauen.

<!--There are two levels of access control in Gitolite.  The first is at the repository level; if you have read (or write) access to *any* ref in the repository, then you have read (or write) access to the repository.-->

Es gibt zwei Ebenen für die Zugriffsberechtigung in Gitolite. Die erste befindet sich auf Repository Ebene. Wenn Du Lese- oder Schreifzugriff auf jede Ref in einem Repository hast, dann kannst Du damit das ganze Repository sowohl lesen, als auch schreiben.

<!--The second level, applicable only to "write" access, is by branch or tag within a repository.  The username, the access being attempted (`W` or `+`), and the refname being updated are known.  The access rules are checked in order of appearance in the config file, looking for a match for this combination (but remember that the refname is regex-matched, not merely string-matched).  If a match is found, the push succeeds.  A fallthrough results in access being denied.-->

Die zweite Ebene bezieht sich auf Branches oder Tags innerhalb eines Repositorys. Auf dieser Ebene kann allerdings nur der Schreibzugriff beschränkt werden. Der Benutzername, die Art des Zugriffs (`W` oder `+`) und der Refname, der aktualisiert wird, sind bekannt. Gitolite prüft, ob einer dieser Regeln auf diese Kombination zutrifft (hierbei ist allerdings zu beachten, dass der Refname mit dem regulären Ausdruck verglichen und kein eins zu eins String-Vergleich durchgeführt wird). Die Zugriffsregeln werden entsprechend der Reihenfolge innerhalb der Konfigurationsdatei abgearbeitet. Wenn eine Kombination zutrifft, kann der Push durchgeführt werden. Trifft keine zu, dann wird der Push verweigert.

<!--### Advanced Access Control with "deny" rules ###-->
### Erweiterte Zugriffsberechtigungen mit „deny“ Regeln ###

<!--So far, we’ve only seen permissions to be one of `R`, `RW`, or `RW+`.  However, Gitolite allows another permission: `-`, standing for "deny".  This gives you a lot more power, at the expense of some complexity, because now fallthrough is not the *only* way for access to be denied, so the *order of the rules now matters*!-->

Bis jetzt waren alle vorgestellten Berechtigungen entweder `R`, `RW`, oder `RW+`. Gitolite kennt aber noch eine weitere Berechtigung: `-`, welche für „deny“, also ablehnen steht. Dies gibt Dir noch viel mehr Möglichkeiten, allerdings auf Kosten der Komplexität, denn ab jetzt ist ein Falltrough nicht die einzige Möglichkeit, wie ein Zugriff auf das Repository abgelehnt wird. Das heißt, die Reihenfolge der aufgestellte Regeln, hat auch eine Bedeutung.

<!--Let us say, in the situation above, we want engineers to be able to rewind any branch *except* master and integ.  Here’s how to do that:-->

Nehmen wir mal an, dass bei unserem bekannten Beispiel, die Gruppe „engineers“ auf alle Branches, außer master und integ, Rewind-Rechte haben soll. Das können wir folgendermaßen erreichen:

	    RW  master integ    = @engineers
	    -   master integ    = @engineers
	    RW+                 = @engineers

<!--Again, you simply follow the rules top down until you hit a match for your access mode, or a deny.  Non-rewind push to master or integ is allowed by the first rule.  A rewind push to those refs does not match the first rule, drops down to the second, and is therefore denied.  Any push (rewind or non-rewind) to refs other than master or integ won’t match the first two rules anyway, and the third rule allows it.-->

Noch einmal zur Wiederholung, man muss jede einzelne Regel von oben nach unten durchgehen und überprüfen, ob eine Regel auf den aktuellen Zugriffsmodus zutrifft oder ob eine Deny-Regel den Zugriff verhindert. Ein Push auf den Branch master oder integ, welcher nicht einem Rewind Push entspricht, wird durch die erste Regel erlaubt. Ein Rewind Push auf diese Refs trifft also auf die erste Regel nicht zu. Deshalb wird die zweite Regel geprüft und auf Grund der Deny-Regel wird der Push verweigert. Jeder Push (unabhängig, ob es sich um einen Rewind Push oder einen normalen Push handelt) auf eine Ref, welche nicht master oder integ entspricht, trifft nicht auf einer der beiden ersten Regeln zu, und wird damit auf Grund der dritten Regel erlaubt.

<!--### Restricting pushes by files changed ###-->
### Ein Push auf Basis von Dateiänderungen einschränken ###

<!--In addition to restricting what branches a user can push changes to, you can also restrict what files they are allowed to touch.  For example, perhaps the Makefile (or some other program) is really not supposed to be changed by just anyone, because a lot of things depend on it or would break if the changes are not done *just right*.  You can tell Gitolite:-->

Neben der Zugriffsbeschränkung auf Basis von Branches, kannst Du genauso verhindern, dass eine Änderung an einer bestimmten Datei gepusht wird. Beispielsweise ist ein Makefile (oder auch andere Programme) nicht dafür geeignet, dass es von jeder x-beliebiegen Person geändert wird. Meist hängen von so einem Makefile viele Dinge ab oder vieles könnte schief laufen, wenn die Änderungen an der Datei nicht korrekt durchgeführt werden würden. Du kannst deshalb Gitolite folgendermaßen konfigurieren:

    repo foo
        RW                      =   @junior_devs @senior_devs

        -   VREF/NAME/Makefile  =   @junior_devs

<!--User who are migrating from the older Gitolite should note that there is a significant change in behaviour with regard to this feature; please see the migration guide for details.-->

Alle Anwender von älteren Gitolite Versionen, die auf eine neue Gitolite Version wechseln, sollten darauf achten, dass sich die neue Version signifikant anders im Bezug auf dieses Feature verhält. Die Umstellungsanleitung (migration guide) weist hier auf weitere Details hin.

<!--### Personal Branches ###-->
### Personenbezogene Branches ###

<!--Gitolite also has a feature called "personal branches" (or rather, "personal branch namespace") that can be very useful in a corporate environment.-->

Gitolite bietet mit den „personal branches“ (genauer „personal branch namespace“) eine weitere Eigenschaft, die im Unternehmensumfeld sehr hilfreich sein kann.

<!--A lot of code exchange in the Git world happens by "please pull" requests.  In a corporate environment, however, unauthenticated access is a no-no, and a developer workstation cannot do authentication, so you have to push to the central server and ask someone to pull from there.-->

Jede Menge Codeänderungen in der Welt von Git passieren, weil jemand einen Pull-Request durchführt. Im Unternehmensumfeld ist ein anonymer Zugriff ein absolutes No-Go und oft kann eine Entwickler Workstation keine Authentifizierung bieten. Deshalb müssen die Änderungen an den zentralen Server gepusht werden und jemand anders muss diese von dort abholen.

<!--This would normally cause the same branch name clutter as in a centralised VCS, plus setting up permissions for this becomes a chore for the admin.-->

Dies würde normalerweise zu dem gleichen Branchnamen-Wirrwarr führen, wie es in zentralisierten Versionskontrollsystemen anzufinden ist. Außerdem wäre es für den Administrator äußerst lästig, die ganzen Berechtigungen dafür zu setzen.

<!--Gitolite lets you define a "personal" or "scratch" namespace prefix for each developer (for example, `refs/personal/<devname>/*`); please see the documentation for details.-->

Gitolite lässt die Definition eines „personal“ oder „scratch“ Namensraum für jeden einzelnen Entwickler zu (zum Beispiel: `refs/personal/<devname>/*`). Die Dokumentation enthält dazu weitere Details.

<!--### "Wildcard" repositories ###-->
### „Wildcard“ Repositorys ###

<!--Gitolite allows you to specify repositories with wildcards (actually Perl regexes), like, for example `assignments/s[0-9][0-9]/a[0-9][0-9]`, to pick a random example.  It also allows you to assign a new permission mode (`C`) which enables users to create repositories based on such wild cards, automatically assigns ownership to the specific user who created it, allows him/her to hand out `R` and `RW` permissions to other users to collaborate, etc.  Again, please see the documentation for details.-->

Mit Platzhaltern (eigentlich Perl reguläre Ausdrücke), wie zum Beispiel `assignments/s[0-9][0-9]/a[0-9][0-9]`, kannst Du in Gitolite auch Repositorys definieren. Außerdem bietet Gitolite eine neuen Berechtigungsmodus (`C`), welcher es den Benutzern ermöglicht, auf Basis dieser Platzhalter, Repositorys zu erzeugen. Dem Benutzer, der das Repository erzeugt hat, wird dieses automatisch zugewiesen, was es ihm oder ihr ermöglicht anderen Benutzern Lese- oder Schreibrechte (`R` und `RW`) zuzuweisen, damit diese zum Projekt beitragen können. Wieder möchte ich Dich darauf hinweisen, dass die Dokumentation weitere Details enthält.

<!--### Other Features ###-->
### Weitere Besonderheiten ###

<!--We’ll round off this discussion with a sampling of other features, all of which, and many more, are described in great detail in the documentation.-->

Ich möchte das Thema Gitolite abschließen, indem ich noch ein paar weitere Besonderheiten kurz anspreche. Diese und viele weitere Features von Gitolite werden ausführlich in der Dokumentation beschrieben.

<!--**Logging**: Gitolite logs all successful accesses.  If you were somewhat relaxed about giving people rewind permissions (`RW+`) and some kid blew away `master`, the log file is a life saver, in terms of easily and quickly finding the SHA that got hosed.-->

**Protokollierung**: Gitolite protokolliert alle Zugriffe, die erfolgreich waren. Wenn Du ein bisschen nachlässig bei der Vergabe von Rewind-Rechten (`RW+`) warst und irgendeiner der Personen mit Rewinde-Rechte dann den `master` zerstört, dann kann Dir die Protokolldatei eine Menge Arbeit ersparen, weil sie Dir hilft, leicht und schnell die SHA Prüfsumme zu finden, die dem Erdboden gleich gemacht wurde.

<!--**Access rights reporting**: Another convenient feature is what happens when you try and just ssh to the server.  Gitolite shows you what repos you have access to, and what that access may be.  Here’s an example:-->

**Zugriffsrechte herausfinden**: Ein anderes praktisches Merkmal von Gitolite lernst Du kennen, wenn Du versuchst Dich über SSH auf dem Server einzuloggen. Gitolite zeigt Dir dann alle Repositorys an, auf die Du Zugriff hast und welche Berechtigung Du für diese hast. Hierzu ein Beispiel:

        hello scott, this is git@git running gitolite3 v3.01-18-g9609868 on git 1.7.4.4

             R     anu-wsd
             R     entrans
             R  W  git-notes
             R  W  gitolite
             R  W  gitolite-admin
             R     indic_web_input
             R     shreelipi_converter

<!--**Delegation**: For really large installations, you can delegate responsibility for groups of repositories to various people and have them manage those pieces independently.  This reduces the load on the main admin, and makes him less of a bottleneck.-->

**Administration aufteilen**: Bei richtig großen Installationen kannst Du die Verantwortlichkeit für verschiedene Gruppen von Repositorys an verschiedene Leute verteilen, damit diese die Repositorys unabhängig verwalten können. Das macht das Leben des Haupt-Administrator leichter und verhindert, dass er der Flaschenhals im System ist.

<!--**Mirroring**: Gitolite can help you maintain multiple mirrors, and switch between them easily if the primary server goes down.-->

**Spiegelung**: Gitolite kann Dir helfen verschiedene Mirrors (Spiegelserver) zu verwalten. Außerdem ist es damit einfach zwischen verschiedenen Mirrors zu wechseln, wenn der primäre Server offline ist.

<!--## Git Daemon ##-->
## Git Daemon ##

<!--For public, unauthenticated read access to your projects, you’ll want to move past the HTTP protocol and start using the Git protocol. The main reason is speed. The Git protocol is far more efficient and thus faster than the HTTP protocol, so using it will save your users time.-->

Wenn Du anonymen, öffentlichen Lesezugriff für Deine Repositorys zur Verfügung stellen willst, solltest Du Dir mal das Git Protokoll, als Ersatz für das HTTP Protokoll anschauen. Der Hauptgrund dafür ist die Geschwindigkeit. Das Git Protokoll ist weitaus effizienter und deshalb viel schneller als das HTTP Protokoll. Wenn Du Deinen Anwendern Zeit ersparen willst, solltest Du es zur Verfügung stellen.

<!--Again, this is for unauthenticated read-only access. If you’re running this on a server outside your firewall, it should only be used for projects that are publicly visible to the world. If the server you’re running it on is inside your firewall, you might use it for projects that a large number of people or computers (continuous integration or build servers) have read-only access to, when you don’t want to have to add an SSH key for each.-->

Ich möchte Dich noch mal darauf hinweisen, dass das Git Protokoll nur für anonymen (also ohne Authentifizierung) Lesezugriff geeignet ist. Wenn Du einen öffentlichen Server betreibst, sollte das Git Protokoll nur für Projekte eingesetzt werden, die öffentlich für den Rest einsehbar sein sollen. Innerhalb Deines eigenen Netzwerks, welches mit einer Firewall abgeschottet ist, ist es sinnvoll, da Du mit dem Git Protokoll einer großen Anzahl von Benutzern und Computern (Continuous Integration oder Build-Server), Lesezugriff zur Verfügung stellen kannst, ohne dass Du für jeden einzelnen Nutzer ein SSH Schlüssel verwalten musst.

<!--In any case, the Git protocol is relatively easy to set up. Basically, you need to run this command in a daemonized manner:-->

Auf jeden Fall ist es sehr einfach das Git Protokoll einzurichten. Im Prinzip musst Du nur folgendes tun:

	git daemon --reuseaddr --base-path=/opt/git/ /opt/git/

<!--`-\-reuseaddr` allows the server to restart without waiting for old connections to time out, the `-\-base-path` option allows people to clone projects without specifying the entire path, and the path at the end tells the Git daemon where to look for repositories to export. If you’re running a firewall, you’ll also need to punch a hole in it at port 9418 on the box you’re setting this up on.-->

Mit `--reuseaddr` teilst Du dem Server mit, dass ein Neustart sofort durchgeführt werden kann, ohne darauf zu warten, dass alte, offene Verbindungen mit einem Timeout abbrechen. Die Option `--base-path` erlaubt es den Nutzern, Projekte zu klonen, ohne den gesamten Pfad angeben zu müssen. Die Pfadangabe als letztes Argument gibt dem Daemon an, wo sich die zu exportierenden Repositorys befinden. Wenn Du eine Firewall eingerichtet hast, musst Du zusätzlich den Port 9418 freischalten.

<!--You can daemonize this process a number of ways, depending on the operating system you’re running. On an Ubuntu machine, you use an Upstart script. So, in the following file-->

Du kannst den Hintergrunddienst für diesen Prozess auf verschiedene Art und Weise einrichten. Das ist natürlich abhängig vom verwendeten Betriebssystem. Auf einem Ubuntu System kannst Du dazu ein Upstart Skript verwenden. Zum Beispiel fügst Du in der folgenden Datei

	/etc/event.d/local-git-daemon

<!--you put this script:-->

das folgende Skript ein (Achtung: In neueren Ubuntu-Versionen lautet der Pfad /etc/init):

	start on startup
	stop on shutdown
	exec /usr/bin/git daemon \
	    --user=git --group=git \
	    --reuseaddr \
	    --base-path=/opt/git/ \
	    /opt/git/
	respawn

<!--For security reasons, it is strongly encouraged to have this daemon run as a user with read-only permissions to the repositories — you can easily do this by creating a new user 'git-ro' and running the daemon as them.  For the sake of simplicity we’ll simply run it as the same 'git' user that Gitosis is running as.-->

Aus Sicherheitsgründen wird dringend empfohlen, dass dieser Daemon als Benutzer ausgeführt wird, welcher nur Lesezugriff auf die betreffenden Repositorys hat. Du stellst das auf einfache Art und Weise sicher, indem Du einen neuen Benutzer ‚git-ro‘ erstellst und den Daemon mit diesem ausführst. Einfachheitshalber verwenden wir hier den Benutzer ‚git‘, den wir auch schon für Gitosis verwendet haben.

<!--When you restart your machine, your Git daemon will start automatically and respawn if it goes down. To get it running without having to reboot, you can run this:-->

Nach einem Neustart des System wird der Git Daemon automatisch starten. Er startet ebenso neu, wenn er unerwartet beendet wird. Der Daemon kann auch ohne einen Neustart gestartet werden:

	initctl start local-git-daemon

<!--On other systems, you may want to use `xinetd`, a script in your `sysvinit` system, or something else — as long as you get that command daemonized and watched somehow.-->

Auf anderen Systemen kannst Du `xinetd`, ein Skript in der `sysvinit`-Umgebung oder irgendetwas anderes verwenden. Du musst nur sicherstellen, dass der Befehl als Hintergrunddienst ausgeführt wird.

<!--Next, you have to tell your Gitosis server which repositories to allow unauthenticated Git server-based access to. If you add a section for each repository, you can specify the ones from which you want your Git daemon to allow reading. If you want to allow Git protocol access for the `iphone_project`, you add this to the end of the `gitosis.conf` file:-->

Als nächstes musst Du dem Gitosis Server mitteilen, für welche Repositorys ein anonymer Zugriff über das Git Protokoll möglich sein soll. Für jedes einzelne Repository kannst Du individuell festlegen, ob der Git Daemon auf dieses Zugriff haben soll. Wenn Du beispielsweise das Git Protokoll für das Projekt `iphone_project` erlauben willst, kannst Du folgendes am Ende der Konfigurationsdatei `gitosis.conf` einfügen:

	[repo iphone_project]
	daemon = yes

<!--When that is committed and pushed up, your running daemon should start serving requests for the project to anyone who has access to port 9418 on your server.-->

Nachdem Du dies eingecheckt und gepusht hast, sollte Dein im Hintergrund laufender Git Daemon die Anfragen aller Benutzer, die Zugriff auf den Port 9418 haben, bearbeiten.

<!--If you decide not to use Gitosis, but you want to set up a Git daemon, you’ll have to run this on each project you want the Git daemon to serve:-->

Wenn Du Dich gegen Gitosis entschieden hast, aber trotzdem dem Git Daemon verwenden willst, musst Du auf dem Server für jedes Projekt, welches der Git Daemon zur Verfügung stellen soll, folgendes ausführen:

	$ cd /path/to/project.git
	$ touch git-daemon-export-ok

<!--The presence of that file tells Git that it’s OK to serve this project without authentication.-->

Wenn diese Datei existiert, erlaubt Git einen anonymen Lesezugriff auf dieses Projekt.

<!--Gitosis can also control which projects GitWeb shows. First, you need to add something like the following to the `/etc/gitweb.conf` file:-->

In Gitosis kann man ebenso einstellen, welche Projekte in GitWeb dargestellt werden sollen. Dazu musst Du als erstes in etwa folgendes in die Konfigurationsdatei `/etc/gitweb.conf` einfügen:

	$projects_list = "/home/git/gitosis/projects.list";
	$projectroot = "/home/git/repositories";
	$export_ok = "git-daemon-export-ok";
	@git_base_url_list = ('git://gitserver');

<!--You can control which projects GitWeb lets users browse by adding or removing a `gitweb` setting in the Gitosis configuration file. For instance, if you want the `iphone_project` to show up on GitWeb, you make the `repo` setting look like this:-->

Um für die einzelnen Projekte festzulegen, dass diese in GitWeb auftauchen, musst Du die Einstellung `gitweb` in der Gitosis Konfigurationsdatei festlegen. Wenn Du beispielsweise willst, dass das Repository `iphone_project` in GitWeb erscheint, muss die Einstellung `repo` in etwa folgendermaßen aussehen:

	[repo iphone_project]
	daemon = yes
	gitweb = yes

<!--Now, if you commit and push the project, GitWeb will automatically start showing the `iphone_project`.-->

Das Projekt `iphone_projekt` wird automatisch in GitWeb angezeigt, sobald Du die Änderungen eingecheckt und auf den Server gepusht hast.

<!--## Hosted Git ##-->
## Git Hosting ##

<!--If you don’t want to go through all of the work involved in setting up your own Git server, you have several options for hosting your Git projects on an external dedicated hosting site. Doing so offers a number of advantages: a hosting site is generally quick to set up and easy to start projects on, and no server maintenance or monitoring is involved. Even if you set up and run your own server internally, you may still want to use a public hosting site for your open source code — it’s generally easier for the open source community to find and help you with.-->

Wenn Du Dir die ganze Arbeit sparen willst, die beim Einrichten eines eigenen Git Servers so anfällt, kannst Du auch einen der vielen Hosting-Anbieter benutzen, um Deine Git Projekte zu verwalten. Wenn Du Dich für diese Option entscheidest, hat das gewisse Vorteile: Die Konfiguration bei den Hosting-Anbietern ist meist sehr schnell durchgeführt und Du kannst sofort mit Deinen Projekten loslegen. Zusätzlich ersparst Du Dir die Wartung und Überwachung Deines eigenen Servers. Auch wenn Du für Deine privaten oder firmeninternen Projekte einen eigenen Server betreibst, sind solche Hosting-Anbieter nützlich, da Du diese dann für Deine Open-Source Projekte verwenden kannst. Dadurch wirst Du innerhalb der Open-Source Community leichter gefunden und es ist einfacher Dir bei Deinen Projekten zu helfen.

<!--These days, you have a huge number of hosting options to choose from, each with different advantages and disadvantages. To see an up-to-date list, check out the following page:-->

Heutzutage stehen Dir viele Anbieter zur Auswahl. Jeder hat seine Vor- und Nachteile. Eine aktuelle Liste von Anbietern findest Du auf der folgenden Seite:

	https://git.wiki.kernel.org/index.php/GitHosting

<!--Because we can’t cover all of them, and because I happen to work at one of them, we’ll use this section to walk through setting up an account and creating a new project at GitHub. This will give you an idea of what is involved.-->

Da wir nicht alle Anbieter vorstellen können und ich zufälligerweise bei einem der Anbieter, nämlich GitHub,  arbeite, werde ich in diesem Kapitel auf diese Plattform näher eingehen. Wir werden die Erstellung eines Accounts und die Erzeugung eines Projekts auf GitHub besprechen. Das sollte Dir einen leichten Einstieg in die Welt von GitHub ermöglichen.

<!--GitHub is by far the largest open source Git hosting site and it’s also one of the very few that offers both public and private hosting options so you can keep your open source and private commercial code in the same place. In fact, we used GitHub to privately collaborate on this book.-->

GitHub ist die mit Abstand größte Open-Source Git Hosting Plattform und bietet als einer der wenigen, sowohl öffentliche und private Hosting-Optionen. Das erlaubt es Dir, Deine Open-Source Projekte und proprietären Code in einer einzelnen Plattform zu verwalten. Sogar für dieses Buch haben wir GitHub benutzt, um gemeinsam unter Ausschluss der Öffentlichkeit daran zu arbeiten.

<!--### GitHub ###-->
### GitHub ###

<!--GitHub is slightly different than most code-hosting sites in the way that it namespaces projects. Instead of being primarily based on the project, GitHub is user centric. That means when I host my `grit` project on GitHub, you won’t find it at `github.com/grit` but instead at `github.com/schacon/grit`. There is no canonical version of any project, which allows a project to move from one user to another seamlessly if the first author abandons the project.-->

GitHub verwaltet und gruppiert Projekte ein wenig anders ein, als andere Code-Hosting Webseiten. GitHub fokussiert sich dabei nicht speziell auf die Projekte, sondern eher auf den Anwender. Dazu ein Beispiel. Wenn ich mein Projekt `grit` auf GitHub einstelle, dann findet man dieses nicht unter `github.com/grit`, sondern unter `github.com/schacon/grit`. Es gibt in diesem Sinne, keine in Stein gemeißelte Stelle an dem das Projekt verwaltet wird. Das bedeutet, man kann ein Projekt nahtlos von einem Benutzer zu einem anderen Benutzer übertragen, wenn dieser zum Beispiel das Projekt aufgibt.

<!--GitHub is also a commercial company that charges for accounts that maintain private repositories, but anyone can quickly get a free account to host as many open source projects as they want. We’ll quickly go over how that is done.-->

Wenn ein Anwender ein geschützes, nicht öffentliches Repository auf GitHub verwalten will, so muss er dafür bezahlen. Damit verdient die Firma GitHub ihr Geld. Aber die Verwaltung und Nutzung für Open-Source Projekte ist kostenlos. Die Anzahl der Projekte ist dabei nicht beschränkt. Der Nutzer muss dazu lediglich einen Account erstellen. Wie das geht, möchte ich hier kurz vorstellen.

<!--### Setting Up a User Account ###-->
## Einrichten eines Benutzeraccounts ###

<!--The first thing you need to do is set up a free user account. If you visit the "Plans and pricing" page at `https://github.com/pricing` and click the "Sign Up" button on the Free account (see Figure 4-2), you’re taken to the signup page.-->

Um loslegen zu können, musst Du Dir einen Benutzeraccount erstellen. Gib dazu die Adresse `https://github.com/pricing` in Deinem Browser ein und wähle den Button „Sign Up“ unter dem „Free account“-Bereich aus (siehe Abbildung 4-2). Danach wirst Du auf die Anmeldeseite weitergeleitet.

<!--Figure 4-2. The GitHub plan page.-->

Insert 18333fig0402.png
Abbildung 4-2. Die Angebotsseite von GitHub.

<!--Here you must choose a username that isn’t yet taken in the system and enter an e-mail address that will be associated with the account and a password (see Figure 4-3).-->

Auf dieser Seite musst Du einen Nutzernamen auswählen, der bisher im System nicht vorhanden ist. Zusätzlich musst Du Deine E-Mail Adresse angeben, die mit Deinem Account verknüpft wird, und ein Passwort angeben (siehe Abbildung 4-3).

<!--Figure 4-3. The GitHub user signup form.-->

Insert 18333fig0403.png
Abbildung 4-3. Das Formular für die GitHub Benutzerregistrierung.

<!--If you have it available, this is a good time to add your public SSH key as well. We covered how to generate a new key earlier, in the "Simple Setups" section. Take the contents of the public key of that pair, and paste it into the SSH Public Key text box. Clicking the "explain ssh keys" link takes you to detailed instructions on how to do so on all major operating systems.-->
<!--Clicking the "I agree, sign me up" button takes you to your new user dashboard (see Figure 4-4).-->

Wenn Du Deinen öffentlichen SSH Schlüssel zur Hand hast, kannst Du diesen auch gleich bei der Registrierung angeben. Die Vorgehensweise zum Generieren eines Schlüssels haben wir bereits im Kapitel 4.3 besprochen. Du musst den Inhalt der öffentlichen Schlüsseldatei kopieren und in das „SSH Public Key“ Formularfeld einfügen. Wenn Du auf den „explain ssh keys“ Link klickst, erhälst Du detailierte Anweisungen zum Ausführen dieses Vorgangs auf verschiedenen Betriebssystemen. Wenn Du auf den „I agree, sign me up“ Button drückst, landest Du in Deinem neuen Benutzer-Dashboard (siehe Abbildung 4-4).

<!--Figure 4-4. The GitHub user dashboard.-->

Insert 18333fig0404.png
Abbildung 4-4. Das GitHub Benutzer-Dashboard.

<!--Next you can create a new repository.-->

Im nächsten Schritt kannst Du ein neues Repository erzeugen.

<!--### Creating a New Repository ###-->
### Erzeugen eines neuen Repository ###

<!--Start by clicking the "create a new one" link next to Your Repositories on the user dashboard. You’re taken to the Create a New Repository form (see Figure 4-5).-->

Um ein neues Repository anzulegen, musst Du auf den „create a new one“-Link, welcher neben Deinen Repositorys auf dem Benutzer-Dashboard angezeigt wird, klicken. Du landest daraufhin im Formular zum Erzeugen eines neuen Repositorys (siehe Abbildung 4-5).

<!--Figure 4-5. Creating a new repository on GitHub.-->

Insert 18333fig0405.png
Abbildung 4-5. Erzeugen eines Repositorys auf GitHub.

<!--All you really have to do is provide a project name, but you can also add a description. When that is done, click the "Create Repository" button. Now you have a new repository on GitHub (see Figure 4-6).-->

Im Prinzip musst Du nur einen Projektnamen und wenn Du es für nötig erachtest eine Beschreibung Deines Projekts angeben. Wenn das erledigt ist, kannst Du auf den „Create Repository“ Button klicken. Du hast soeben Dein erstes Repository auf GitHub erzeugt (siehe Abbildung 4-6).

<!--Figure 4-6. GitHub project header information.-->

Insert 18333fig0406.png
Abbildung 4-6. GitHub Projektinformationen.

<!--Since you have no code there yet, GitHub will show you instructions for how create a brand-new project, push an existing Git project up, or import a project from a public Subversion repository (see Figure 4-7).-->

Da in dem Repository noch kein Code enthalten ist, gibt Dir GitHub ein paar Hinweise, wie Du ein neues Projekt anlegst, wie Du ein bereits vorhandes Git Projekt auf GitHub pusht oder wie man ein bestehendes Subversion Repository in GitHub importieren kann (siehe Abbildung 4-7).

<!--Figure 4-7. Instructions for a new repository.-->

Insert 18333fig0407.png
Abbildung 4-7. Anleitung zum Erzeugen eines neuen Repository.

<!--These instructions are similar to what we’ve already gone over. To initialize a project if it isn’t already a Git project, you use-->

Die Hilfestellung entspricht der Vorgehensweise, die ich bereits in diesem Buch vorgestellt habe. Um ein neues Git Projekt in einem vorhandenen Verzeichnis anzulegen, kannst Du die folgenden Befehle verwenden:

	$ git init
	$ git add .
	$ git commit -m 'initial commit'

<!--When you have a Git repository locally, add GitHub as a remote and push up your master branch:-->

Wenn Du bereits ein lokales Git Repository auf Deinem PC hast, kannst Du GitHub als zusätzlichen Remote hinzufügen und den master Branch pushen:

	$ git remote add origin git@github.com:testinguser/iphone_project.git
	$ git push origin master

<!--Now your project is hosted on GitHub, and you can give the URL to anyone you want to share your project with. In this case, it’s `http://github.com/testinguser/iphone_project`. You can also see from the header on each of your project’s pages that you have two Git URLs (see Figure 4-8).-->

Das war es schon. Dein Projekt ist nun auf GitHub erreichbar. Du kannst jetzt die zugehörige URL an jeden weitergeben, den Du am Projekt teilhaben lassen willst. In unserem Beispiel lautet der Link hierfür `http://github.com/testinguser/iphone_project`. Im oberen Header-Bereich jeder Projektseite werden zwei verschiedene Git URLs angezeigt (siehe Abbildung 4-8).

<!--Figure 4-8. Project header with a public URL and a private URL.-->

Insert 18333fig0408.png
Abbildung 4-8. Projekt Header mit der Angabe der öffentlichen und privaten URL.

<!--The Public Clone URL is a public, read-only Git URL over which anyone can clone the project. Feel free to give out that URL and post it on your web site or what have you.-->

Die „Public Clone URL“ ist eine öffentliche Git URL, die man zum Klonen des Projekts verwenden kann. Über diese URL kann lediglich lesend zugegriffen werden. Ein Schreibzugriff ist nicht möglich. Du kannst diese URL beliebig weiter verteilen und zum Beispiel auch auf Deiner Homepage oder einem anderen Medium veröffentlichen.

<!--The Your Clone URL is a read/write SSH-based URL that you can read or write over only if you connect with the SSH private key associated with the public key you uploaded for your user. When other users visit this project page, they won’t see that URL—only the public one.-->

Die „Your Clone URL“ ist eine auf SSH basierte URL, mit Hilfe derer, vom Projekt gelesen, als auch geschrieben werden kann. Diese URL kann aber nur der Anwender nutzen, der im Besitz des privaten Schlüssel ist, welcher zu dem öffentlichen Schlüssel gehört, der bei dem GitHub Benutzer für dieses Projekt hinterlegt ist (Du hast den öffentlichen Schlüssel bei der Registrierung Deines Accounts angegeben).. Dieser Link wird allerdings nur Dir angezeigt. Andere Benutzer, die Deine Projekte besuchen, können nur die „Public Clone URL“ sehen.

<!--### Importing from Subversion ###-->
### Import von Subversion ###

<!--If you have an existing public Subversion project that you want to import into Git, GitHub can often do that for you. At the bottom of the instructions page is a link to a Subversion import. If you click it, you see a form with information about the import process and a text box where you can paste in the URL of your public Subversion project (see Figure 4-9).-->

Wenn Du bereits ein Subversion Projekt hast und dieses in Git importieren möchtest, kann GitHub Dir diese Aufgabe in vielen Fällen übernehmen. Am Ende der Seite mit den Hilfestellungen ist ein Link, der Dich auf die Seite mit dem Formular zum Importieren eines Subversion Projekts weiterleitet. Du musst in diesem Formular nur die URL des Subversion Projekts angeben (siehe Abbildung 4-9).

<!--Figure 4-9. Subversion importing interface.-->

Insert 18333fig0409.png
Abbildung 4-9. Import-Schnittstelle für Subversion Projekte.

<!--If your project is very large, nonstandard, or private, this process probably won’t work for you. In Chapter 7, you’ll learn how to do more complicated manual project imports.-->

Wenn Dein Projekt sehr groß, nicht standardkonform oder nicht öffentlich einsehbar ist, kann es passieren das dieser Vorgang fehlschlägt. In Kapitel 7 liefere ich die Antwort, wie ein solcher Import, manuell durchgeführt werden kann.

<!--### Adding Collaborators ###-->
### Mitarbeiter hinzufügen ###

<!--Let’s add the rest of the team. If John, Josie, and Jessica all sign up for accounts on GitHub, and you want to give them push access to your repository, you can add them to your project as collaborators. Doing so will allow pushes from their public keys to work.-->

Um gemeinsam an einem Projekt zu arbeiten, kannst Du auch andere Personen für das Projekt freischalten. Wenn Du willst das John, Josie und Jessica ebenso auf das Projekt pushen können, musst Du sie als Mitarbeiter für Dein Projekt freischalten. Voraussetzung hierfür ist natürlich, dass Sie alle einen GitHub Account besitzen. Nachdem Du sie zum Projekt hinzugefügt hast, können sie unter Verwendung ihrer öffentlichen Schlüssel auf das Projekt pushen.

<!--Click the "edit" button in the project header or the Admin tab at the top of the project to reach the Admin page of your GitHub project (see Figure 4-10).-->

Klicke auf die „edit“-Schaltfläche in der Projektübersicht oder wähle den Admin-Tab im oberen Bereich Deines Projekts um zur Administrationsoberfläche zu gelangen (siehe Abbildung 4-10).

<!--Figure 4-10. GitHub administration page.-->

Insert 18333fig0410.png
Abbildung 4-10. GitHub Administrationsoberfläche.

<!--To give another user write access to your project, click the “Add another collaborator” link. A new text box appears, into which you can type a username. As you type, a helper pops up, showing you possible username matches. When you find the correct user, click the Add button to add that user as a collaborator on your project (see Figure 4-11).-->

Um einem anderen Benutzer Schreibrechte zu Deinem Projekt zu gewähren, kannst Du auf den „Add another collaborator“-Link klicken. Daraufhin erscheint ein Eingabefeld, in welches Du die Benutzer eingeben kannst. Während des Tippens, erscheint ein kleines Popup, welches Benutzernamen anzeigt, die Deiner Eingabe entsprechen. Wenn Du den gewünschten Benutzer gefunden hast, kannst Du die „Add“-Schaltfläche betätigen, um diesen Benutzer als Mitarbeiter zu Deinem Projekt hinzuzufügen (siehe Abbildung 4-11).

<!--Figure 4-11. Adding a collaborator to your project.-->

Insert 18333fig0411.png
Abbildung 4-11. Ein Mitarbeiter zu Deinem Projekt hinzufügen.

<!--When you’re finished adding collaborators, you should see a list of them in the Repository Collaborators box (see Figure 4-12).-->

Wenn Du Dein Team fertig zusammengestellt hast, solltest Du eine Liste aller Mitarbeiter im „Repository Collaborators“-Bereich sehen (siehe Abbildung 4.12).

<!--Figure 4-12. A list of collaborators on your project.-->

Insert 18333fig0412.png
Abbildung 4-12. Übersicht über alle Mitarbeiter in Deinem Projekt.

<!--If you need to revoke access to individuals, you can click the "revoke" link, and their push access will be removed. For future projects, you can also copy collaborator groups by copying the permissions of an existing project.-->

Wenn Du einer Person den Zugriff auf Dein Repository entziehen willst, kannst Du auf den „revoke“-Link klicken. Dadurch kann diese Person nicht mehr auf Dein Repository pushen. Für zukünftige Projekte kannst Du die Liste der Benutzer auch für andere Projekte übernehmen.

<!--### Your Project ###-->
### Dein Projekt ###

<!--After you push your project up or have it imported from Subversion, you have a main project page that looks something like Figure 4-13.-->

Nachdem Du das erste mal auf das GitHub Repository gepusht hast oder es von Subversion importiert hast, sieht die Hauptseite Deines GitHub-Projekts entsprechend Abbildung 4-13 aus.

<!--Figure 4-13. A GitHub main project page.-->

Insert 18333fig0413.png
Abbildung 4-13. Beispiel einer Hauptseite eines GitHub-Projekts.

<!--When people visit your project, they see this page. It contains tabs to different aspects of your projects. The Commits tab shows a list of commits in reverse chronological order, similar to the output of the `git log` command. The Network tab shows all the people who have forked your project and contributed back. The Downloads tab allows you to upload project binaries and link to tarballs and zipped versions of any tagged points in your project. The Wiki tab provides a wiki where you can write documentation or other information about your project. The Graphs tab has some contribution visualizations and statistics about your project. The main Source tab that you land on shows your project’s main directory listing and automatically renders the README file below it if you have one. This tab also shows a box with the latest commit information.-->

Wenn andere Dein Projekt in GitHub aufrufen, sehen sie als erstes diese Hauptseite. Die verschiedenen Funktionen, die GitHub für ein Projekt unterstützt, sind in verschiedene Tabs aufgeteilt. Der „Commit“-Tab enthält eine Liste aller Commits in chronologischer Reihenfolge. Dabei steht der neueste Commit ganz oben. Eine ähnliche Liste erhälst Du, wenn Du den `git log` Befehl ausführst. Der „Network“-Tab zeigt alle Benutzer an, die ein Fork von Deinem Projekt erstellt haben und Ihren Teil zum Projekt beigetragen haben. Im „Download“-Tab kannst Du Binärdateien hochladen oder Zip-Archive beziehungsweise Tarballs zur Verfügung stellen, die einem bestimmten Tag Deines Projekts entsprechen. Im „Wiki“-Tab findest Du ein Wiki, welches Du zu Dokumentationszwecken verwenden kannst. Außerdem kannst Du dort andere Informationen über Dein Projekt der Welt mitteilen. Der „Graphs“-Tab stellt Dir eine grafische Übersicht zur Verfügung, die Dir anzeigt, wer und wann jemand etwas zu Deinem Projekt beigetragen hat. Außerdem enthält dieser Tab eine Projekt-Statistik. Der „Source“-Tab, welcher standardmäßig beim Aufruf Deines Projekts angezeigt wird, zeigt das oberste Verzeichnis Deines Repositorys an. Wenn Dein Projekt eine README-Datei enthält, wird der Inhalt dieser Datei unterhalb der Verzeichnisstruktur angezeigt. Zusätzlich zeigt dieser Tab eine Übersicht mit den letzten Commit-Informationen an.

<!--### Forking Projects ###-->
### Fork von einem Projekt erstellen ###

<!--If you want to contribute to an existing project to which you don’t have push access, GitHub encourages forking the project. When you land on a project page that looks interesting and you want to hack on it a bit, you can click the "fork" button in the project header to have GitHub copy that project to your user so you can push to it.-->

Wenn Du an einem bereits vorhandenen Projekt mitarbeiten willst und Du zu diesem keine Schreibrechte hast, bietet Dir GitHub die Möglichkeit einen Fork von diesem Projekt zu erstellen. Wenn Du beim Stöbern durch GitHub bei einem interessanten Projekt landest und Du damit ein bisschen spielen willst, kannst Du die „Fork“-Schaltfläche im Projekt-Header auswählen. GitHub kopiert das gesamte Projekt in Deinen Benutzerbereich. Auf dieses kannst Du jetzt auch pushen.

<!--This way, projects don’t have to worry about adding users as collaborators to give them push access. People can fork a project and push to it, and the main project maintainer can pull in those changes by adding them as remotes and merging in their work.-->

Dadurch das jeder, jedes Projekt kopieren kann, muss sich der Verwalter eines Projekts nicht darum kümmern, das jedem Mitarbeiter die entsprechenden Schreibrechte erhält. Man kann einfach einen Fork erstellen und auf diesen pushen. Der Verwalter des geforkten Projekts kann dieses neue Projekt als neuen Remote hinzufügen und die Änderungen davon holen. Dann kann er diese Änderungen in das Hauptprojekt mergen.

<!--To fork a project, visit the project page (in this case, mojombo/chronic) and click the "fork" button in the header (see Figure 4-14).-->

Um einen Fork von einem Projekt zu erstellen, kannst Du die jeweilige Projektseite besuchen (in diesem Fall mojombo/chronic) und die „Fork“-Schaltfläche im oberen Bereich auswählen (siehe Abbildung 4-14).

<!--Figure 4-14. Get a writable copy of any repository by clicking the "fork" button.-->

Insert 18333fig0414.png
Abbildung 4-14. Durch Betätigen der „Fork“-Schaltfläche erhält man eine Kopie eines Repositorys, auf welches man Schreibrechte hat.

<!--After a few seconds, you’re taken to your new project page, which indicates that this project is a fork of another one (see Figure 4-15).-->

Nach ein paar Sekunden wirst Du zu der neuen Projektseite weitergeleitet. Dort siehst Du auch, dass dieses Projekt ein Fork eines anderen Projekts ist (siehe Abbildung 4-15).

<!--Figure 4-15. Your fork of a project.-->

Insert 18333fig0415.png
Abbildung 4-15. Dein Fork eines Projekts.

<!--### GitHub Summary ###-->
### GitHub Zusammenfassung ###

<!--That’s all we’ll cover about GitHub, but it’s important to note how quickly you can do all this. You can create an account, add a new project, and push to it in a matter of minutes. If your project is open source, you also get a huge community of developers who now have visibility into your project and may well fork it and help contribute to it. At the very least, this may be a way to get up and running with Git and try it out quickly.-->

Das war alles was ich über GitHub berichten möchte. Herausheben möchte ich aber, dass sich die Arbeit mit GitHub sehr einfach gestaltet. Innerhalb kürzester Zeit kannst Du Dir einen Account einrichten, ein neues Projekt hinzufügen und auf dieses pushen. Wenn Dein Projekt ein Open-Source Projekt ist und damit öffentlich einsehbar ist, steht Dir mit GitHub eine riesige Community mit zahlreichen Entwicklern zur Seite, die sich an Deinem Projekt beteiligen können, indem sie einen Fork erstellen. Vielleicht gelingt Dir mit GitHub der Einstieg in die Welt von Git noch einfacher.

<!--## Summary ##-->
## Zusammenfassung ##

<!--You have several options to get a remote Git repository up and running so that you can collaborate with others or share your work.-->

Es stehen Dir viele Möglichkeiten offen, ein Remote-Repository einzurichten, sodass Du mit anderen Entwicklern zusammenarbeiten kannst und Deine Arbeit anderen zur Verfügung stellen kannst.

<!--Running your own server gives you a lot of control and allows you to run the server within your own firewall, but such a server generally requires a fair amount of your time to set up and maintain. If you place your data on a hosted server, it’s easy to set up and maintain; however, you have to be able to keep your code on someone else’s servers, and some organizations don’t allow that.-->

Wenn Du Deinen eigenen Server betreibst, stehen Dir natürlich alle Möglichkeiten offen und Du kannst ihn entsprechend Deiner Anforderungen konfigurieren. Denoch darf man den Zeitaufwand zum Aufsetzen und Warten des Servers nicht unterschätzen. Wenn Du Deine Repositorys bei einem Hosting-Anbieter verwaltest, hält sich der Aufwand für die Einrichtung und Wartung in Grenzen, allerdings vertraust Du Deine Daten auch einem fremden Anbieter an, was manche Firmen oder Organisationen untersagen.

<!--It should be fairly straightforward to determine which solution or combination of solutions is appropriate for you and your organization.-->

Es sollte Dir nicht schwerfallen eine passende Lösung für Dich, Deine Firma oder Deine Organisation zu finden.
<!--# Distributed Git #-->
# Distribuierte Arbeit mit Git (xxx) #

<!--Now that you have a remote Git repository set up as a point for all the developers to share their code, and you’re familiar with basic Git commands in a local workflow, you’ll look at how to utilize some of the distributed workflows that Git affords you.-->

Du hast jetzt ein externes Repository aufgesetzt, sodass alle Mitglieder des Teams ihren Code zur Verfügung stellen können, und Du hast Dich mit den wesentlichen Git Befehlen für die Arbeit in einem lokalen Repository vertraut gemacht. Als nächstes werden wir uns einige Arbeitsabläufe für distribuierte Repositories ansehen, die Git Dir ermöglicht.

<!--In this chapter, you’ll see how to work with Git in a distributed environment as a contributor and an integrator. That is, you’ll learn how to contribute code successfully to a project and make it as easy on you and the project maintainer as possible, and also how to maintain a project successfully with a number of developers contributing.-->

In diesem Kapitel wirst Du lernen, wie Du in einer distribuierten Umgebung als Entwickler Code zu einem Projekt beisteuern, als für Verantwortlicher Code von anderen ins Projekt übernehmen kannst und wie Du das so gestalten kannst, dass es in einem Projekt mit einer großen Anzahl von Entwicklern für alle Beteiligten möglichst einfach ist.

<!--## Distributed Workflows ##-->
## Distribuierte Workflows ##

<!--Unlike Centralized Version Control Systems (CVCSs), the distributed nature of Git allows you to be far more flexible in how developers collaborate on projects. In centralized systems, every developer is a node working more or less equally on a central hub. In Git, however, every developer is potentially both a node and a hub — that is, every developer can both contribute code to other repositories and maintain a public repository on which others can base their work and which they can contribute to. This opens a vast range of workflow possibilities for your project and/or your team, so I’ll cover a few common paradigms that take advantage of this flexibility. I’ll go over the strengths and possible weaknesses of each design; you can choose a single one to use, or you can mix and match features from each.-->

Anders als in zentralisierten Versionskontrollsystemen (CVCS) ermöglicht die Distribuiertheit von Git eine sehr viel flexiblere Zusammenarbeit von Entwicklern. In zentralisierten Systemen fungieren alle Beteiligten als gleichwertige Netzknoten, die in mehr oder weniger der gleichen Weise am zentralen Knotenpunkt (dem zentralen Repository) arbeiten. In Git dagegen ist jeder Beteiligten selbst potentiell zentraler Knotenpunkt. D.h. jeder Entwickler kann sowohl Code zu anderen Repositories beitragen und ein öffentliches Repository zur Verfügung stellen, an dem wiederum andere mitarbeiten. Das ermöglicht eine riesige Anzahl von Möglichkeiten, Arbeitsabläufe zu gestalten, die auf das jeweilige Projekt und/oder Team perfekt zugeschnitten sind. Wir werden auf einige übliche Paradigmen, die diese Flexibilität nutzen, und deren Vor- und Nachteile eingehen. Du kannst daraus ein Modell auswählen, oder Du kannst sie miteinander kombinieren, um sie an Deine eigenen Erfordernisse anzupassen.

<!--### Centralized Workflow ###-->
### Zentralisierter Workflow ###

<!--In centralized systems, there is generally a single collaboration model—the centralized workflow. One central hub, or repository, can accept code, and everyone synchronizes their work to it. A number of developers are nodes — consumers of that hub — and synchronize to that one place (see Figure 5-1).-->

In einem zentralisierten System gibt es grob gesagt ein einziges Modell der Zusammenarbeit. Ein zentraler Knotenpunkt (oder Repository) kann Code von anderen akzeptieren und übernehmen, und alle Beteiligten synchronisieren ihre Arbeit damit. Entwickler fungieren als Knoten, die ihre Arbeit an diesem einen, zentralen Punkt synchronisieren (siehe Bild 5-1).

<!--Figure 5-1. Centralized workflow.-->

Insert 18333fig0501.png
Bild 5-1. Zentralisierter Workflow

<!--This means that if two developers clone from the hub and both make changes, the first developer to push their changes back up can do so with no problems. The second developer must merge in the first one’s work before pushing changes up, so as not to overwrite the first developer’s changes. This concept is true in Git as it is in Subversion (or any CVCS), and this model works perfectly in Git.-->

Das heißt: wenn zwei Entwickler Code aus dem zentralen Repository abholen und beide Änderungen vornehmen, dann kann der erste Entwickler seine Änderungen ohne Probleme im zentralen Repository abliefern. Der zweite Entwickler muss sie zunächst mit den Änderungen des ersten Entwicklers zusammenführen, damit er dessen Arbeit nicht überschreibt. Dieses Konzept trifft sowohl auf Git als auch auf Subversion (und jedes andere CVCS) zu, und es funktioniert in Git perfekt.

<!--If you have a small team or are already comfortable with a centralized workflow in your company or team, you can easily continue using that workflow with Git. Simply set up a single repository, and give everyone on your team push access; Git won’t let users overwrite each other. If one developer clones, makes changes, and then tries to push their changes while another developer has pushed in the meantime, the server will reject that developer’s changes. They will be told that they’re trying to push non-fast-forward changes and that they won’t be able to do so until they fetch and merge.-->
<!--This workflow is attractive to a lot of people because it’s a paradigm that many are familiar and comfortable with.-->

In einem kleinen Team oder einem Team, das mit einem zentralisierten Workflow zufrieden ist, kann man diesen Workflow ohne weiteres mit Git realisieren. Man setzt einfach ein einziges Repository auf und gibt jedem im Team Schreibzugriff („push access“). Git sorgt dann dafür, dass niemand die Arbeit von anderen überschreiben kann. Wenn ein Entwickler das Repository klont, Änderungen vornimmt und dann versucht ins zentrale Repository zu pushen, obwohl jemand anders in der Zwischenzeit Änderungen gepusht hat, dann wird der Server das zurückweisen. Dem Entwickler wird dann mitgeteilt, dass er versucht hat, sogeannte „non-fast-forward“ Änderungen hochzuladen und dass er zuvor die Änderungen des anderen Entwicklers herunterladen und mit seinen zusammenführen muss. Viele Leute mögen diesen Arbeitsablauf, weil sie mit dem Paradigma bereits vertraut sind und sich damit wohl fühlen.


<!--### Integration-Manager Workflow ###-->
### Integration-Manager Workflow ###

<!--Because Git allows you to have multiple remote repositories, it’s possible to have a workflow where each developer has write access to their own public repository and read access to everyone else’s. This scenario often includes a canonical repository that represents the "official" project. To contribute to that project, you create your own public clone of the project and push your changes to it. Then, you can send a request to the maintainer of the main project to pull in your changes. They can add your repository as a remote, test your changes locally, merge them into their branch, and push back to their repository. The process works as follow (see Figure 5-2):-->

Weil Git ermöglicht, eine Vielzahl von externen Repositories zu betreiben, ist es außerdem möglich, einen Arbeitsprozess zu gestalten, in dem jeder Entwickler Schreibzugriff auf sein eigenes öffentliches Repository hat, aber nur Lesezugriff auf die Repositories von allen anderen Beteiligten. In diesem Szenario stellt jedes Repository ein eigenes „offizielles“ Projekt dar. Um zu einem solchen distribuierten Projekt Änderungen beizusteuern, kannst Du einen eigenen, öffentlichen Klon des Projektes anlegen und Deine Änderungen dort publizieren. Anschließend kannst Du den Betreiber des Haupt-Repositories bitten, Deine Änderungen in sein Repository zu übernehmen. Er kann dann Dein Repository als ein externes Repository auf seinem Rechner einrichten, Deine Änderungen lokal testen, sie in einen seiner Branches (z.B. master) mergen und dann in sein öffentliches Repository pushen. Dieser Prozess läuft wie folgt ab (siehe Bild 5-2):

<!--1. The project maintainer pushes to their public repository.-->
<!--2. A contributor clones that repository and makes changes.-->
<!--3. The contributor pushes to their own public copy.-->
<!--4. The contributor sends the maintainer an e-mail asking them to pull changes.-->
<!--5. The maintainer adds the contributor’s repo as a remote and merges locally.-->
<!--6. The maintainer pushes merged changes to the main repository.-->

1.  Der Projekt Betreiber pusht in ein öffentliches Repository.
2.  Ein Mitarbeiter klont das Repository und nimmt Änderungen daran vor.
3.  Der Mitarbeiter pusht diese in sein eigenes öffentliches Repository.
4.  Der Mitarbeiter schickt dem Betreiber eine E-Mail und bittet darum, die Änderungen zu übernehmen.
5.  Der Betreiber richtet das Repository des Mitarbeiters als ein externes Repository ein und führt die Änderungen mit einem seiner eigenen Branches zusammen.
6.  Der Betreiber pusht die zusammengeführten Änderungen in sein öffentliches Repository.

<!--Figure 5-2. Integration-manager workflow.-->

Insert 18333fig0502.png
Bild 5-2. Integration-Manager Workflow

<!--This is a very common workflow with sites like GitHub, where it’s easy to fork a project and push your changes into your fork for everyone to see. One of the main advantages of this approach is that you can continue to work, and the maintainer of the main repository can pull in your changes at any time. Contributors don’t have to wait for the project to incorporate their changes — each party can work at their own pace.-->

Dies ist ein weit verbreiteter Arbeitsablauf wie ihn z.B. auch GitHub ermöglicht, wo man ein Projekt auf sehr einfache Weise forken und seine Änderungen in seinen eigenen Fork pushen kann, um sie anderen zur Verfügung zu stellen. Einer der Hauptvorteile dieser Vorgehensweise ist, dass man an seinem Fork jederzeit weiterarbeiten, der Betreiber des Projektes Änderungen aber auch jederzeit übernehmen kann. Mitarbieter müssen nicht darauf warten, dass der Betreiber Änderungen übernimmt – und jeder Beteiligte kann in seinem eigenen Rhythmus und Tempo arbeiten.

<!--### Dictator and Lieutenants Workflow ###-->
### Diktator und Leutnants Workflow ###

<!--This is a variant of a multiple-repository workflow. It’s generally used by huge projects with hundreds of collaborators; one famous example is the Linux kernel. Various integration managers are in charge of certain parts of the repository; they’re called lieutenants. All the lieutenants have one integration manager known as the benevolent dictator. The benevolent dictator’s repository serves as the reference repository from which all the collaborators need to pull. The process works like this (see Figure 5-3):-->

Dies ist Variante eines Workflows mit zahlreichen Repositories, die normalerweise von sehr großen Projekten mit hunderten von Mitarbeitern verwendet wird. Das bekannteste Beispiel ist wahrscheinlich der Linux Kernel. In diesem Projekt sind zahlreiche Integration Manager, die „Leutnants“, für verschiedene Bereiche des Repositories zuständig. Für sämtliche Leutnants gibt es wiederum einen Integration Manager, der als der „wohlwollende Diktator“ („benevolent dictator“) bezeichnet wird. Das Repository des wohlwollenden Diktators fungiert als das Referenz-Repository aus dem alle Beteiligten ihre eigenen Repositories aktualisieren müssen. Dieser Prozess funktioniert also wie folgt (siehe Bild 5-3)

<!--1. Regular developers work on their topic branch and rebase their work on top of master. The master branch is that of the dictator.-->
<!--2. Lieutenants merge the developers’ topic branches into their master branch.-->
<!--3. The dictator merges the lieutenants’ master branches into the dictator’s master branch.-->
<!--4. The dictator pushes their master to the reference repository so the other developers can rebase on it.-->

1.  Normale Entwickler arbeiten in ihren Arbeitsbranches (xxx) und rebasen (xxx) ihre Änderungen auf der Basis des Master Branches. Der Master Branch ist derjenige des Diktators.
2.  Die Leutnants mergen die Arbeitsbranches der Entwickler in ihre Master Branches.
3.  Der Diktator merged die Master Branches der Leutnants mit seinem eigenen Master Branch zusammen.
4.  Der Diktator pusht seinen Master Branch ins Referenz-Repository, sodass alle ihre Arbeit wiederum damit synchronisieren (rebasen) können.

<!--Figure 5-3. Benevolent dictator workflow.-->

Insert 18333fig0503.png
Bild 5-3. Wohlwollender Diktator Workflow

<!--This kind of workflow isn’t common but can be useful in very big projects or in highly hierarchical environments, as it allows the project leader (the dictator) to delegate much of the work and collect large subsets of code at multiple points before integrating them.-->

Diese Art Workflow ist nicht unbedingt weit verbreitet, aber für große Projekte oder Projekte mit strikten hierarchischen Rollen sehr nützlich, weil der Projektleiter (der Diktator) Arbeit in großem Umfang delegieren und ganze Teilbereiche von Code von verschiedenen Endpunkten zusammensammeln und integrieren kann.

<!--These are some commonly used workflows that are possible with a distributed system like Git, but you can see that many variations are possible to suit your particular real-world workflow. Now that you can (I hope) determine which workflow combination may work for you, I’ll cover some more specific examples of how to accomplish the main roles that make up the different flows.-->

Wir haben jetzt einige übliche Workflows besprochen, die in einem distribuierten System wie Git möglich sind. Natürlich kann man sie mannigfaltig abwandeln und miteinander kombinieren, um sie an ein spezielles reales Projekt und Team anzupassen. Nachdem Du jetzt hoffentlich in der Lage bist, Dir einen Workflow vorzustellen, der für Dich selbst Sinn macht, gehen wir auf einige etwas spezifischere Beispiele ein und darauf, wie man die verschiedenen Rollen umsetzen kann, die die Workflows ausmachen.

<!--## Contributing to a Project ##-->
## An einem Projekt mitarbeiten ##

<!--You know what the different workflows are, and you should have a pretty good grasp of fundamental Git usage. In this section, you’ll learn about a few common patterns for contributing to a project.-->

Du kennst jetzt einige grundlegende Workflow Varianten, und Du solltest ein gutes Verständnis im Umgang mit grundlegenden Git Befehlen haben. In diesem Abschnitt lernst Du wie Du an einem Projekt mitzuarbeiten kannst.

<!--The main difficulty with describing this process is that there are a huge number of variations on how it’s done. Because Git is very flexible, people can and do work together many ways, and it’s problematic to describe how you should contribute to a project — every project is a bit different. Some of the variables involved are active contributor size, chosen workflow, your commit access, and possibly the external contribution method.-->

Diesen Prozess zu beschreiben ist nicht leicht, weil es so viele Variationen gibt. Git ist so unheimlich flexibel, dass Leute auf vielen unterschiedlichen Wegen zusammenarbeiten können, und es ist problematisch, zu erklären, wie Du arbeiten _solltest_, weil jedes Projekt ein bisschen anders ist. Zu den Variablen gehören: die Anzahl der aktiven Mitarbeiter, der Workflow des Projektes, Deine Commit Rechte und möglicherweise eine vorgeschriebene, externe Methode, Änderungen einzureichen.

<!--The first variable is active contributor size. How many users are actively contributing code to this project, and how often? In many instances, you’ll have two or three developers with a few commits a day, or possibly less for somewhat dormant projects. For really large companies or projects, the number of developers could be in the thousands, with dozens or even hundreds of patches coming in each day. This is important because with more and more developers, you run into more issues with making sure your code applies cleanly or can be easily merged. Changes you submit may be rendered obsolete or severely broken by work that is merged in while you were working or while your changes were waiting to be approved or applied. How can you keep your code consistently up to date and your patches valid?-->

Wie viele Mitarbeiter tragen aktive Code zum Projekt bei? Und wie oft? In vielen Fällen findest Du zwei oder drei Entwickler, die täglich einige Commits anlegen, möglicherweise weniger in eher (xxx dormant xxx) Projekten. In wirklich großen Unternehmen oder Projekten können tausende Entwickler involviert sein und täglich dutzende oder sogar hunderte von Patches produzieren. Mit so vielen Mitarbeitern ist es aufwendiger, sicher zu stellen, dass Änderungen sauber mit der Codebase zusammengeführt werden können. Deine Änderungen könnten sich als überflüssig oder dysfunktional (xxx) erweisen, nachdem andere Änderungen übernommen wurden, seit Du angefangen hast, an Deinen eigenen zu arbeiten oder während sie darauf warteten, geprüft und eingefügt (xxx) zu werden.

<!--The next variable is the workflow in use for the project. Is it centralized, with each developer having equal write access to the main codeline? Does the project have a maintainer or integration manager who checks all the patches? Are all the patches peer-reviewed and approved? Are you involved in that process? Is a lieutenant system in place, and do you have to submit your work to them first?-->

Die nächste Variable ist der Workflow, der in diesem Projekt besteht. Ist es ein zentralisierter Workflow, in dem jeder Entwickler Schreibzugriff auf die Hauptentwicklungslinie hat? Hat das Projekt einen Leiter oder Integration Manager, der alle Patches prüft? Werden die Patches durch eine bestimmte Gruppe oder öffentlich, z.B. durch die Community, geprüft? Nimmst Du selbst an diesem Prozess teil? Gibt es ein Leutnant System, in dem Du Deine Arbeit zunächst an einen Leutnant übergibst?

<!--The next issue is your commit access. The workflow required in order to contribute to a project is much different if you have write access to the project than if you don’t. If you don’t have write access, how does the project prefer to accept contributed work? Does it even have a policy? How much work are you contributing at a time? How often do you contribute?-->

Eine weitere Frage ist, welche Commit Rechte Du hast. Wenn Du Schreibrechte hast, sieht der Arbeitsablauf, mit dem Du Änderungen beisteuern kannst, natürlich völlig anders aus, als wenn Du nur Leserechte hast. Und in letzterem Fall: in welcher Form werden Änderungen in diesem Projekt akzeptiert? Gibt es dafür überhaupt eine Richtlinie? Wie umfangreich sind die Änderungen, die Du jeweils beisteuerst? Und wie oft tust Du das?

<!--All these questions can affect how you contribute effectively to a project and what workflows are preferred or available to you. I’ll cover aspects of each of these in a series of use cases, moving from simple to more complex; you should be able to construct the specific workflows you need in practice from these examples.-->

Die Antworten auf diese Fragen können maßgeblich beeinflussen, wie Du effektiv an einem Projekt mitarbeiten kannst und welche Workflows Du zur Auswahl hast. Wir werden verschiedene Aspekte davon in einer Reihe von Fallbeispielen besprechen, wobei wir mit simplen Beispielen anfangen und später komplexere Szenarios besprechen. Du wirst hoffentlich in der Lage sein, aus diesen Beispielen einen eigenen Workflow zu konstruieren, der Deinen Anforderungen entspricht.

<!--### Commit Guidelines ###-->
### Commit Richtlinien ###

<!--Before you start looking at the specific use cases, here’s a quick note about commit messages. Having a good guideline for creating commits and sticking to it makes working with Git and collaborating with others a lot easier. The Git project provides a document that lays out a number of good tips for creating commits from which to submit patches — you can read it in the Git source code in the `Documentation/SubmittingPatches` file.-->

Bevor wir uns verschiedene konkrete Fallbeispiele ansehen, einige kurze Anmerkungen über Commit Meldungen. Gute Richtlinien für Commit Meldungen zu haben und sich danach zu richten, macht die Zusammenarbeit mit anderen und die Arbeit mit Git selbst sehr viel einfacher. Im Git Projekt gibt es ein Dokument mit einer Reihe nützlicher Tipps für das Anlegen von Commits, aus denen man Patches erzeugen will. Schaue im Git Quellcode nach der Datei `Documentation/SubmittingPatches`.

<!--First, you don’t want to submit any whitespace errors. Git provides an easy way to check for this — before you commit, run `git diff -\-check`, which identifies possible whitespace errors and lists them for you. Here is an example, where I’ve replaced a red terminal color with `X`s:-->

Zunächst einmal solltest Du keine Whitespace Fehler (xxx) comitten:

	$ git diff --check
	lib/simplegit.rb:5: trailing whitespace.
	+    @git_dir = File.expand_path(git_dir)XX
	lib/simplegit.rb:7: trailing whitespace.
	+ XXXXXXXXXXX
	lib/simplegit.rb:26: trailing whitespace.
	+    def command(git_cmd)XXXX

<!--If you run that command before committing, you can tell if you’re about to commit whitespace issues that may annoy other developers.-->

Wenn Du diesen Befehl ausführst, bevor Du einen Commit anlegst, warnt er dich, falls in Deinen Änderungen Whitespace Probleme vorliegen, die andere Entwickler ärgern könnten.

<!--Next, try to make each commit a logically separate changeset. If you can, try to make your changes digestible — don’t code for a whole weekend on five different issues and then submit them all as one massive commit on Monday. Even if you don’t commit during the weekend, use the staging area on Monday to split your work into at least one commit per issue, with a useful message per commit. If some of the changes modify the same file, try to use `git add -\-patch` to partially stage files (covered in detail in Chapter 6). The project snapshot at the tip of the branch is identical whether you do one commit or five, as long as all the changes are added at some point, so try to make things easier on your fellow developers when they have to review your changes. This approach also makes it easier to pull out or revert one of the changesets if you need to later. Chapter 6 describes a number of useful Git tricks for rewriting history and interactively staging files — use these tools to help craft a clean and understandable history.-->

Versuche außerdem, Deine Änderungen in logisch zusammenhängende Einheiten zu gruppieren. Wenn möglich, versuche Commits möglichst leichtverständlich (xxx) zu gestalten: arbeite nicht ein ganzes Wochenende lang an fünf verschiedenen Problemen und committe sie dann am Montag als einen einzigen, riesigen Commit. Selbst wenn Du am Wochenende keine Commits angelegt hast, verwende die Staging Area, um Deine Änderungen auf mehrere Commits aufzuteilen, jeweils mit einer verständlichen Meldung. Wenn einige Änderungen dieselbe Datei betreffen, probiere sie mit `git add --patch` nur teilweise zur Staging Area hinzuzufügen (das werden wir in Kapitel 6 noch im Detail besprechen). Der Projekt Snapshot wird am Ende derselbe sein, ob Du nun einen einzigen großen oder mehrere kleine Commits anlegst, daher versuche, es anderen Entwickler zu erleichtern machen, Deine Änderungen zu verstehen. Auf diese Weise machst Du es auch einfacher, einzelne Änderungen später herauszunehmen oder rückgängig zu machen. Kapitel 6 beschreibt eine Reihe nützlicher Git Tricks, die hilfreich sind, um die Historie umzuschreiben oder interaktiv Dateien zur Staging Area hinzuzufügen. Verwende diese Hilfsmittel, um eine sauber und leicht verständliche Historie von Änderungen aufzubauen.

<!--The last thing to keep in mind is the commit message. Getting in the habit of creating quality commit messages makes using and collaborating with Git a lot easier. As a general rule, your messages should start with a single line that’s no more than about 50 characters and that describes the changeset concisely, followed by a blank line, followed by a more detailed explanation. The Git project requires that the more detailed explanation include your motivation for the change and contrast its implementation with previous behavior — this is a good guideline to follow. It’s also a good idea to use the imperative present tense in these messages. In other words, use commands. Instead of "I added tests for" or "Adding tests for," use "Add tests for."-->
<!--Here is a template originally written by Tim Pope at tpope.net:-->

Ein weitere Sache, der Du ein bisschen Aufmerksamkeit schenken solltest, ist die Commit Meldung selbst. Wenn man sich angewöhnt, aussagekräftige und hochwertige Commit Meldungen zu schreiben, macht man sich selbst und anderen das Leben erheblich einfacher. Im allgemeinen sollte eine Commit Meldung mit einer einzelnen Zeile anfangen, die nicht länger als 50 Zeichen sein sollte. Dann sollte eine leere Zeile folgen und schließlich eine ausführlichere Beschreibung der Änderungen.

	Short (50 chars or less) summary of changes

	More detailed explanatory text, if necessary.  Wrap it to about 72
	characters or so.  In some contexts, the first line is treated as the
	subject of an email and the rest of the text as the body.  The blank
	line separating the summary from the body is critical (unless you omit
	the body entirely); tools like rebase can get confused if you run the
	two together.

	Further paragraphs come after blank lines.

	 - Bullet points are okay, too

	 - Typically a hyphen or asterisk is used for the bullet, preceded by a
	   single space, with blank lines in between, but conventions vary here

<!--If all your commit messages look like this, things will be a lot easier for you and the developers you work with. The Git project has well-formatted commit messages — I encourage you to run `git log -\-no-merges` there to see what a nicely formatted project-commit history looks like.-->

Wenn Du Deine Commit Meldungen in dieser Weise formatierst, kannst Du Dir und anderen eine Menge Ärger ersparen. Das Git Projekt selbst hat wohl-formatierte Commit Meldungen. Wir empfehlen, einmal `git log --no-merges` in diesem Repository auszuführen, um einen Eindruck zu erhalten, wie eine gute Commit History eines Projektes aussehen kann.

<!--In the following examples, and throughout most of this book, for the sake of brevity I don’t format messages nicely like this; instead, I use the `-m` option to `git commit`. Do as I say, not as I do.-->

In den folgenden Beispielen hier und fast überall in diesem Buch verwende ich keine derartigen, schön formatierten Meldungen. Stattdessen verwende ich die `-m` Option zusammen mit `git commit`. Also folge meinen Worten, nicht meinem Beispiel.

<!--### Private Small Team ###-->
### Kleine Teams ###

<!--The simplest setup you’re likely to encounter is a private project with one or two other developers. By private, I mean closed source — not read-accessible to the outside world. You and the other developers all have push access to the repository.-->

Das einfachste Setup, mit dem Du zu tun haben wirst, ist ein privates Projekt mit ein oder zwei Entwicklern. Mit „privat“ meine ich, dass es „closed source“, d.h. nicht lesbar für Dritte ist. Alle beteiligten Entwickler haben Schreibzugriff auf das Repository.

<!--In this environment, you can follow a workflow similar to what you might do when using Subversion or another centralized system. You still get the advantages of things like offline committing and vastly simpler branching and merging, but the workflow can be very similar; the main difference is that merges happen client-side rather than on the server at commit time.-->
<!--Let’s see what it might look like when two developers start to work together with a shared repository. The first developer, John, clones the repository, makes a change, and commits locally. (I’m replacing the protocol messages with `...` in these examples to shorten them somewhat.)-->

In einer solchen Umgebung kann man einen ähnlichen Workflow verwenden, wie für Subversion oder ein anderes zentralisiertes System. Du hast dann immer noch Vorteile wie, dass Du offline committen kannst und dass Branching und Merging so unglaublich einfach ist. Der Hauptunterschied ist, dass Merges auf der Client Seite stattfinden und nicht, wenn man committet, auf dem Server. Schauen wir uns an, wie die Arbeit von zwei Entwicklern in einem gemeinsamen Repository abläuft. Der erste Entwickler, John, klont das Repository, nimmt eine Änderung vor und comittet auf seinem Rechner. (Wir kürzen die Beispiele etwas ab und ersetzen die hierfür irrelevanten Protokoll Meldungen mit `xxx`.)

	# John's Machine
	$ git clone john@githost:simplegit.git
	Initialized empty Git repository in /home/john/simplegit/.git/
	...
	$ cd simplegit/
	$ vim lib/simplegit.rb
	$ git commit -am 'removed invalid default value'
	[master 738ee87] removed invalid default value
	 1 files changed, 1 insertions(+), 1 deletions(-)

<!--The second developer, Jessica, does the same thing — clones the repository and commits a change:-->

Der zweite Entwickler, Jessica, tut das gleiche. Sie klont das Repository und committet eine Änderung:

	# Jessica's Machine
	$ git clone jessica@githost:simplegit.git
	Initialized empty Git repository in /home/jessica/simplegit/.git/
	...
	$ cd simplegit/
	$ vim TODO
	$ git commit -am 'add reset task'
	[master fbff5bc] add reset task
	 1 files changed, 1 insertions(+), 0 deletions(-)

<!--Now, Jessica pushes her work up to the server:-->

Jetzt lädt Jessica ihre Arbeit mit `git push` auf den Server:

	# Jessica's Machine
	$ git push origin master
	...
	To jessica@githost:simplegit.git
	   1edee6b..fbff5bc  master -> master

<!--John tries to push his change up, too:-->

John versucht, das selbe zu tun:

	# John's Machine
	$ git push origin master
	To john@githost:simplegit.git
	 ! [rejected]        master -> master (non-fast forward)
	error: failed to push some refs to 'john@githost:simplegit.git'

<!--John isn’t allowed to push because Jessica has pushed in the meantime. This is especially important to understand if you’re used to Subversion, because you’ll notice that the two developers didn’t edit the same file. Although Subversion automatically does such a merge on the server if different files are edited, in Git you must merge the commits locally. John has to fetch Jessica’s changes and merge them in before he will be allowed to push:-->

John darf seine Änderung nicht pushen, weil Jessica in der Zwischenzeit gepushed hat. Dies ist ein Unterschied zu Subversion: wie Du siehst, haben die beiden Entwickler nicht dieselbe Datei bearbeitet. Während Subversion automatisch merged, wenn lediglich verschiedene Dateien bearbeitet wurden, muss man Commits in Git lokal mergen. John muss also Jessicas Änderungen herunterladen und mergen, bevor er dann selbst pushen darf:

	$ git fetch origin
	...
	From john@githost:simplegit
	 + 049d078...fbff5bc master     -> origin/master

<!--At this point, John’s local repository looks something like Figure 5-4.-->

Zu diesem Zeitpunkt sieht Johns lokales Repository jetzt aus wie in Bild 5-4.

<!--Figure 5-4. John’s initial repository.-->

Insert 18333fig0504.png
Bild 5-4. Johns ursprüngliches Repository

<!--John has a reference to the changes Jessica pushed up, but he has to merge them into his own work before he is allowed to push:-->

John hat eine Referenz auf Jessicas Änderungen, aber er muss sie mit seinen eigenen Änderungen mergen, bevor er auf den Server pushen darf:

	$ git merge origin/master
	Merge made by recursive.
	 TODO |    1 +
	 1 files changed, 1 insertions(+), 0 deletions(-)

<!--The merge goes smoothly — John’s commit history now looks like Figure 5-5.-->

Der Merge verläuft glatt: Johns Commit Historie sieht jetzt aus wie in Bild 5-5.

<!--Figure 5-5. John’s repository after merging origin/master.-->

Insert 18333fig0505.png
Johns Repository nach dem Merge mit origin/master

<!--Now, John can test his code to make sure it still works properly, and then he can push his new merged work up to the server:-->

John sollte seinen Code jetzt testen, um sicher zu stellen, dass alles weiterhin funktioniert. Dann kann er seine Arbeit auf den Server pushen:

	$ git push origin master
	...
	To john@githost:simplegit.git
	   fbff5bc..72bbc59  master -> master

<!--Finally, John’s commit history looks like Figure 5-6.-->

Johns Commit Historie sieht schließlich aus wie in Bild 5-6.

<!--Figure 5-6. John’s history after pushing to the origin server.-->

Insert 18333fig0506.png
Johns Commit Historie nach dem pushen auf den origin Server

<!--In the meantime, Jessica has been working on a topic branch. She’s created a topic branch called `issue54` and done three commits on that branch. She hasn’t fetched John’s changes yet, so her commit history looks like Figure 5-7.-->

In der Zwischenzeit hat Jessica auf einem Topic Branch (xxx) gearbeitet. Sie hat einen Topic Branch mit dem Namen `issue54` und darin drei Commits angelegt. Sie hat Johns Änderungen bisher noch nicht herunter geladen, sodass ihre Commit Historie jetzt so aussieht wie in Bild 5-7.

<!--Figure 5-7. Jessica’s initial commit history.-->

Insert 18333fig0507.png
Bild 5-7. Jessicas ursprüngliche Commit Historie

<!--Jessica wants to sync up with John, so she fetches:-->

Jessica will ihre Arbeit jetzt mit John synchronisieren. Also lädt sie seine Änderungen herunter:

	# Jessica's Machine
	$ git fetch origin
	...
	From jessica@githost:simplegit
	   fbff5bc..72bbc59  master     -> origin/master

<!--That pulls down the work John has pushed up in the meantime. Jessica’s history now looks like Figure 5-8.-->

Das lädt die Änderungen, die John in der Zwischenzeit hochgeladen hat. Jessicas Historie entspricht jetzt Bild 5-8.

<!--Figure 5-8. Jessica’s history after fetching John’s changes.-->

Insert 18333fig0508.png
Bild 5-8. Jessicas Historie nachdem sie Johns Änderungen geladen hat

<!--Jessica thinks her topic branch is ready, but she wants to know what she has to merge her work into so that she can push. She runs `git log` to find out:-->

Jessica hat die Arbeit in ihrem Topic Branch abgeschlossen, aber sie will wissen, welche neuen Änderungen es gibt, mit denen sie ihre eigenen mergen muss.

	$ git log --no-merges origin/master ^issue54
	commit 738ee872852dfaa9d6634e0dea7a324040193016
	Author: John Smith <jsmith@example.com>
	Date:   Fri May 29 16:01:27 2009 -0700

	    removed invalid default value

<!--Now, Jessica can merge her topic work into her `master` branch, merge John’s work (`origin/master`) into her `master` branch, and then push back to the server again. First, she switches back to her `master` branch to integrate all this work:-->

Jetzt kann Jessica zunächst ihren Topic Branch `issue54` in ihren `master` Branch mergen, dann Johns Änderungen aus `origin/master` in ihren `master` Branch mergen und schließlich das Resultat auf den `origin` Server pushen. Als erstes wechselt sie zurück auf ihren `master` Branch:

	$ git checkout master
	Switched to branch "master"
	Your branch is behind 'origin/master' by 2 commits, and can be fast-forwarded.

<!--She can merge either `origin/master` or `issue54` first — they’re both upstream, so the order doesn’t matter. The end snapshot should be identical no matter which order she chooses; only the history will be slightly different. She chooses to merge in `issue54` first:-->

Sie kann jetzt entweder `origin/master` oder `issue54` zuerst mergen – sie sind beide „upstream“ (xxx). Der resultierende Snapshot wäre identisch, egal in welcher Reihenfolge sie beide Branches in ihren `master` Branch merged, lediglich die Historie würde natürlich minimal anders aussehen. Jessica entscheidet sich, `issue54` zuerst zu mergen:

	$ git merge issue54
	Updating fbff5bc..4af4298
	Fast forward
	 README           |    1 +
	 lib/simplegit.rb |    6 +++++-
	 2 files changed, 6 insertions(+), 1 deletions(-)

<!--No problems occur; as you can see, it was a simple fast-forward. Now Jessica merges in John’s work (`origin/master`):-->

Das ging glatt, wie Du siehst, war es ein einfacher „fast-forward“ Merge. Als nächstes merged Jessica Johns Änderungen aus `origin/master`:

	$ git merge origin/master
	Auto-merging lib/simplegit.rb
	Merge made by recursive.
	 lib/simplegit.rb |    2 +-
	 1 files changed, 1 insertions(+), 1 deletions(-)

<!--Everything merges cleanly, and Jessica’s history looks like Figure 5-9.-->

Auch hier treten keine Konflikte auf. Jessicas Historie sieht jetzt wie folgt aus (Bild 5-9).

<!--Figure 5-9. Jessica’s history after merging John’s changes.-->

Insert 18333fig0509.png
Bild 5-9. Jessicas Historie nach dem Merge mit Johns Änderungen

<!--Now `origin/master` is reachable from Jessica’s `master` branch, so she should be able to successfully push (assuming John hasn’t pushed again in the meantime):-->

`origin/master` ist jetzt in Jessicas `master` Branch enthalten (xxx reachable xxx), sodass sie in der Lage sein sollte, auf den `origin` Server zu pushen (vorausgesetzt, John hat zwischenzeitlich nicht gepusht):

	$ git push origin master
	...
	To jessica@githost:simplegit.git
	   72bbc59..8059c15  master -> master

<!--Each developer has committed a few times and merged each other’s work successfully; see Figure 5-10.-->

Beide Entwickler haben jetzt einige Male committed und die Arbeit des jeweils anderen erfolgreich mit ihrer eigenen zusammengeführt.

<!--Figure 5-10. Jessica’s history after pushing all changes back to the server.-->

Insert 18333fig0510.png
Bild 5-10. Jessicas Historie nachdem sie sämtliche Änderungen auf den Server gepusht hat

<!--That is one of the simplest workflows. You work for a while, generally in a topic branch, and merge into your `master` branch when it’s ready to be integrated. When you want to share that work, you merge it into your own `master` branch, then fetch and merge `origin/master` if it has changed, and finally push to the `master` branch on the server. The general sequence is something like that shown in Figure 5-11.-->

Dies ist eine der simpelsten Workflow Varianten. Du arbeitest eine Weile, normalerweise in einem Topic Branch, und mergst in Deinen `master` Branch, wenn Du fertig bist. Wenn Du Deine Änderungen anderen zur Verfügung stellen willst, holst Du den aktuellen `origin/master` Branch, mergst Deinen `master` Branch damit und pushst das ganze zurück auf den `origin` Server. Der Ablauf sieht in etwa wie folgt aus (Bild 5-11).

<!--Figure 5-11. General sequence of events for a simple multiple-developer Git workflow.-->

Insert 18333fig0511.png
Bild 5-11. Ablauf eines einfachen Workflows für mehrere Entwickler

<!--### Private Managed Team ###-->
### Teil-Teams mit Integration Manager ###

<!--In this next scenario, you’ll look at contributor roles in a larger private group. You’ll learn how to work in an environment where small groups collaborate on features and then those team-based contributions are integrated by another party.-->

Im folgenden Szenario sehen wir uns die Rollen von Mitarbeitern in einem größeren, nicht öffentlich arbeitenden Team an. Du wirst sehen, wie man in einer Umgebung arbeiten kann, in der kleine Gruppen (z.B. an einzelnen Features) zusammenarbeiten und ihre Ergebnisse dann von einer weiteren Gruppe in die Hauptentwicklungslinie integriert werden.

<!--Let’s say that John and Jessica are working together on one feature, while Jessica and Josie are working on a second. In this case, the company is using a type of integration-manager workflow where the work of the individual groups is integrated only by certain engineers, and the `master` branch of the main repo can be updated only by those engineers. In this scenario, all work is done in team-based branches and pulled together by the integrators later.-->

Sagen wir John und Jessica arbeiten gemeinsam an einem Feature, während Jessica und Josie an einem anderen arbeiten. Das Unternehmen verwendet einen Integration-Manager Workflow, in dem die Arbeit der verschiedenen Gruppen von anderen Mitarbeitern zentral integriert werden – und der `master` Branch nur von diesen letzteren geschrieben werden kann. In diesem Szenario wird sämtliche Arbeit von den Teams in Branches erledigt und dann von den Integration-Manangern zusammengeführt.

<!--Let’s follow Jessica’s workflow as she works on her two features, collaborating in parallel with two different developers in this environment. Assuming she already has her repository cloned, she decides to work on `featureA` first. She creates a new branch for the feature and does some work on it there:-->

Schauen wir uns Jessicas Workflow an, während sie mit jeweils verschiedenen Entwicklern parallel an zwei Features arbeitet. Nehmen wir an, sie hat das Repository bereits geklont und will zuerst an `featureA` arbeiten. Sie legt einen neuen Branch für das Feature an und fängt an, daran zu arbeiten:

	# Jessica's Machine
	$ git checkout -b featureA
	Switched to a new branch "featureA"
	$ vim lib/simplegit.rb
	$ git commit -am 'add limit to log function'
	[featureA 3300904] add limit to log function
	 1 files changed, 1 insertions(+), 1 deletions(-)

<!--At this point, she needs to share her work with John, so she pushes her `featureA` branch commits up to the server. Jessica doesn’t have push access to the `master` branch — only the integrators do — so she has to push to another branch in order to collaborate with John:-->

Jetzt will sie ihre Arbeit John zur Verfügung stellen, der am gleichen Feature arbeiten will, und pusht dazu ihre Commits in ihrem `featureA` Branch auf den Server. Jessica hat keinen Schreibzugriff auf den `master` Branch – den haben nur die Integration Manager – also pusht sie ihren Feature Branch, der nur der Zusammenarbeit mit John dient:

	$ git push origin featureA
	...
	To jessica@githost:simplegit.git
	 * [new branch]      featureA -> featureA

<!--Jessica e-mails John to tell him that she’s pushed some work into a branch named `featureA` and he can look at it now. While she waits for feedback from John, Jessica decides to start working on `featureB` with Josie. To begin, she starts a new feature branch, basing it off the server’s `master` branch:-->

Jessica schickt John eine E-Mail und lässt ihn wissen, dass sie ihre Arbeit in einen Branch `featureA` hochgeladen hat. Während sie jetzt auf Feedback von John wartet, kann Jessica anfangen, an `featureB` zuarbeiten – diesmal gemeinsam mit Josie. Also legt sie einen neuen Feature Branch an, der auf dem gegenwärtigen `master` Branch des `origin` Servers basiert:

	# Jessica's Machine
	$ git fetch origin
	$ git checkout -b featureB origin/master
	Switched to a new branch "featureB"

<!--Now, Jessica makes a couple of commits on the `featureB` branch:-->

Jetzt legt Jessica eine Reihe von Commits im `featureB` Branch an:

	$ vim lib/simplegit.rb
	$ git commit -am 'made the ls-tree function recursive'
	[featureB e5b0fdc] made the ls-tree function recursive
	 1 files changed, 1 insertions(+), 1 deletions(-)
	$ vim lib/simplegit.rb
	$ git commit -am 'add ls-files'
	[featureB 8512791] add ls-files
	 1 files changed, 5 insertions(+), 0 deletions(-)

<!--Jessica’s repository looks like Figure 5-12.-->

Jessicas Repository entspricht jetzt Bild 5-12.

<!--Figure 5-12. Jessica’s initial commit history.-->

Insert 18333fig0512.png
Bild 5-12. Jessicas ursprüngliche Commit Historie

<!--She’s ready to push up her work, but gets an e-mail from Josie that a branch with some initial work on it was already pushed to the server as `featureBee`. Jessica first needs to merge those changes in with her own before she can push to the server. She can then fetch Josie’s changes down with `git fetch`:-->

Jessica könnte ihre Arbeit jetzt hochladen, aber sie hat eine E-Mail von Josie erhalten, dass sie bereits einen Feature Branch `featureBee` für dasselbe Feature auf dem Server angelegt hat. Jessica muss also erst ihre eigenen Änderungen mit diesem Branch mergen und dann dorthin pushen. Sie lädt also Josies Änderungen mit `git fetch` herunter:

	$ git fetch origin
	...
	From jessica@githost:simplegit
	 * [new branch]      featureBee -> origin/featureBee

<!--Jessica can now merge this into the work she did with `git merge`:-->

Jessica kann ihre eigene Arbeit jetzt mit diesen Änderungen zusammenführen:

	$ git merge origin/featureBee
	Auto-merging lib/simplegit.rb
	Merge made by recursive.
	 lib/simplegit.rb |    4 ++++
	 1 files changed, 4 insertions(+), 0 deletions(-)

<!--There is a bit of a problem — she needs to push the merged work in her `featureB` branch to the `featureBee` branch on the server. She can do so by specifying the local branch followed by a colon (:) followed by the remote branch to the `git push` command:-->

Es gibt jetzt ein kleines Problem. Jessica muss die zusammengeführten Änderungen in ihrem `featureB` Branch in den `featureBee` Branch auf dem Server pushen. Das kann sie tun, indem sie sowohl den Namen ihres lokalen Branches als auch des externen Branches angibt, und zwar mit einem Doppelpunkt getrennt:

	$ git push origin featureB:featureBee
	...
	To jessica@githost:simplegit.git
	   fba9af8..cd685d1  featureB -> featureBee

<!--This is called a _refspec_. See Chapter 9 for a more detailed discussion of Git refspecs and different things you can do with them.-->

Das nennt man eine _Refspec_. In Kapitel 9 gehen wir detailliert auf Git Refspecs ein und darauf, was man noch mit ihnen machen kann.

<!--Next, John e-mails Jessica to say he’s pushed some changes to the `featureA` branch and ask her to verify them. She runs a `git fetch` to pull down those changes:-->

Als nächstes schickt John Jessica eine E-Mail. Er schreibt, dass er einige Änderungen in den `featureA` Branch gepusht hat, und bittet sie, diese zu prüfen. Sie führt also `git fetch` aus, um die Änderungen herunter zu laden:

	$ git fetch origin
	...
	From jessica@githost:simplegit
	   3300904..aad881d  featureA   -> origin/featureA

<!--Then, she can see what has been changed with `git log`:-->

Danach kann sie die neuen Änderungen mit `git log` auflisten:

	$ git log origin/featureA ^featureA
	commit aad881d154acdaeb2b6b18ea0e827ed8a6d671e6
	Author: John Smith <jsmith@example.com>
	Date:   Fri May 29 19:57:33 2009 -0700

	    changed log output to 30 from 25

<!--Finally, she merges John’s work into her own `featureA` branch:-->

Schließlich aktualisiert sie ihren eigenen `featureA` Branch mit Johns Änderungen:

	$ git checkout featureA
	Switched to branch "featureA"
	$ git merge origin/featureA
	Updating 3300904..aad881d
	Fast forward
	 lib/simplegit.rb |   10 +++++++++-
	1 files changed, 9 insertions(+), 1 deletions(-)

<!--Jessica wants to tweak something, so she commits again and then pushes this back up to the server:-->

Jessica will eine kleine Änderung vornehmen. Also comittet sie und pusht den neuen Commit auf den Server:

	$ git commit -am 'small tweak'
	[featureA 774b3ed] small tweak
	 1 files changed, 1 insertions(+), 1 deletions(-)
	$ git push origin featureA
	...
	To jessica@githost:simplegit.git
	   3300904..774b3ed  featureA -> featureA

<!--Jessica’s commit history now looks something like Figure 5-13.-->

Jessicas Commit Historie sieht jetzt wie folgt aus (Bild 5-13).

<!--Figure 5-13. Jessica’s history after committing on a feature branch.-->

Insert 18333fig0513.png
Bild 5-13. Jessicas Historie mit dem neuen Commit im Feature Branch

<!--Jessica, Josie, and John inform the integrators that the `featureA` and `featureBee` branches on the server are ready for integration into the mainline. After they integrate these branches into the mainline, a fetch will bring down the new merge commits, making the commit history look like Figure 5-14.-->

Jessica, Josie und John informieren jetzt ihre Integration Manager, dass die Ändeurngen in den Branches `featureA` und `featureBee` fertig sind und in die Hauptlinie in `master` übernommen werden können. Nachdem das geschehen ist, wird `git fetch` die neuen Merge Commits herunter laden und die Commit Historie in etwa wie folgt aussehen (Bild 5-14):

<!--Figure 5-14. Jessica’s history after merging both her topic branches.-->

Insert 18333fig0514.png
Bild 5-14. Jessicas Historie nachdem beide Feature Branches gemerged wurden

<!--Many groups switch to Git because of this ability to have multiple teams working in parallel, merging the different lines of work late in the process. The ability of smaller subgroups of a team to collaborate via remote branches without necessarily having to involve or impede the entire team is a huge benefit of Git. The sequence for the workflow you saw here is something like Figure 5-15.-->

Viele Teams wechseln zu Git, weil es auf einfache Weise ermöglicht, verschiedene Teams parallel an verschiedenen Entwicklungslinien zu arbeiten, die erst später im Prozess integriert werden. Ein riesiger Vorteil von Git besteht darin, dass man in kleinen Teilgruppen über externe Branches zusammenarbeiten kann, ohne dass dazu notwendig wäre, das gesamte Team zu involvieren und möglicherweise aufzuhalten. Der Ablauf dieser Art von Workflow kann wie folgt dargestellt werden (Bild 5-15).

<!--Figure 5-15. Basic sequence of this managed-team workflow.-->

Insert 18333fig0515.png
Bild 5-15. Workflow mit Teil-Teams und Integration Manager

<!--### Public Small Project ###-->
### Kleine, öffentliche Projekte ###

<!--Contributing to public projects is a bit different. Because you don’t have the permissions to directly update branches on the project, you have to get the work to the maintainers some other way. This first example describes contributing via forking on Git hosts that support easy forking. The repo.or.cz and GitHub hosting sites both support this, and many project maintainers expect this style of contribution. The next section deals with projects that prefer to accept contributed patches via e-mail.-->

An öffentlichen Projekten mitzuarbeiten funktioniert ein bisschen anders. Weil man normalerweise keinen Schreibzugriff auf das öffentliche Repository des Projektes hat, muss man mit den Betreibern in anderer Form zusammenarbeiten. Unser erstes Beispiel beschreibt, wie man zu Projekten auf Git Hosts beitragen kann, die es erlauben Forks eines Projektes anzulegen. Z.B. unterstützen die Git Hosting Seiten repo.or.cz und GitHub dieses Feature – und viele Projekt Betreiber akzeptieren Änderungen in dieser Form. Das nächste Beispiel geht dann darauf ein, wie man mit Projekten arbeiten kann, die es bevorzugen, Patches per E-Mail zu erhalten (xxx oder Ticket Tracker, wie z.B. Rails xxx).

<!--First, you’ll probably want to clone the main repository, create a topic branch for the patch or patch series you’re planning to contribute, and do your work there. The sequence looks basically like this:-->

Zunächst wirst vermutlich das Hauptrepository klonen, einen Topic Branch für Deinen Patch anlegen und dann darin arbeiten. Der Prozess sieht dann in etwa so aus:

	$ git clone (url)
	$ cd project
	$ git checkout -b featureA
	$ (work)
	$ git commit
	$ (work)
	$ git commit

<!--You may want to use `rebase -i` to squash your work down to a single commit, or rearrange the work in the commits to make the patch easier for the maintainer to review — see Chapter 6 for more information about interactive rebasing.-->

Es ist wahrscheinlich sinnvoll, `git rebase -i` zu verwenden, um die verschiedenen Commits zu einem einzigen zusammen zu packen („squash“, quetschen) oder um sie in anderer Weise neu zu arrangieren, sodass es für die Projekt Betreiber leichter ist, die Änderungen nach zu vollziehen. In Kapitel 6 gehen wir ausführlicher auf das interaktive `rebase -i` ein.

<!--When your branch work is finished and you’re ready to contribute it back to the maintainers, go to the original project page and click the "Fork" button, creating your own writable fork of the project. You then need to add in this new repository URL as a second remote, in this case named `myfork`:-->

Wenn Dein Branch fertig ist und Du Deine Arbeit den Projekt Betreibern zur Verfügung stellen willst, gehst Du auf die Projekt Seite und klickst auf den „Fork“ Button. Dadurch legst Du Deinen eigenen Fork des Projektes an, in den Du dann schreiben kannst. Die Repository URL dieses Forks musst Du dann als ein zweites, externes Repository („remote“) einrichten. In unserem Beispiel verwenden wir den Namen `myfork`:

	$ git remote add myfork (url)

<!--You need to push your work up to it. It’s easiest to push the remote branch you’re working on up to your repository, rather than merging into your master branch and pushing that up. The reason is that if the work isn’t accepted or is cherry picked, you don’t have to rewind your master branch. If the maintainers merge, rebase, or cherry-pick your work, you’ll eventually get it back via pulling from their repository anyhow:-->

Jetzt kannst Du Deine Änderungen dorthin hochladen. Am besten tust Du das, indem Du Deinen Topic Branch hochlädst (statt ihn in Deinen `master` Branch zu mergen und den dann hochzuladen). Dies deshalb, weil Du, wenn Deine Änderungen nicht akzeptiert werden, Deinen eigenen `master` Branch nicht zurücksetzen musst. Wenn die Projekt Betreiber Deine Änderungen mergen, rebasen oder cherry-picken, landen sie schließlich ohnehin in Deinem `master` Branch.

	$ git push myfork featureA

<!--When your work has been pushed up to your fork, you need to notify the maintainer. This is often called a pull request, and you can either generate it via the website — GitHub has a "pull request" button that automatically messages the maintainer — or run the `git request-pull` command and e-mail the output to the project maintainer manually.-->

Nachdem Du Deine Arbeit in Deinen Fork hochgeladen hast, musst Du die Projekt Betreiber benachrichtigen. Dies wird oft als „pull request“ bezeichnet. Du kannst ihn entweder direkt über die Webseite schicken (GitHub hat dazu einen „pull request“ Button) oder den Git Befehl `git request-pull` verwenden und manuell eine E-Mail an die Projekt Betreiber schicken.

<!--The `request-pull` command takes the base branch into which you want your topic branch pulled and the Git repository URL you want them to pull from, and outputs a summary of all the changes you’re asking to be pulled in. For instance, if Jessica wants to send John a pull request, and she’s done two commits on the topic branch she just pushed up, she can run this:-->

Der `request-pull` Befehl vergleicht denjenigen Branch, für den Deine Änderungen gedacht sind, mit Deinem Topic Branch und gibt eine Übersicht der Änderungen aus. Wenn Jessica zwei Änderungen in einem Topic Branch hat und nun John einen pull request schicken will, kann sie folgendes tun:

	$ git request-pull origin/master myfork
	The following changes since commit 1edee6b1d61823a2de3b09c160d7080b8d1b3a40:
	  John Smith (1):
	        added a new function

	are available in the git repository at:

	  git://githost/simplegit.git featureA

	Jessica Smith (2):
	      add limit to log function
	      change log output to 30 from 25

	 lib/simplegit.rb |   10 +++++++++-
	 1 files changed, 9 insertions(+), 1 deletions(-)

<!--The output can be sent to the maintainer—it tells them where the work was branched from, summarizes the commits, and tells where to pull this work from.-->

Die Ausgabe kann an den Projekt Betreiber geschickt werden – sie sagt klar, auf welchem Branch die Arbeit basiert, gibt eine Zusammenfassung der Änderungen und gibt an, aus welchem Fork oder Repository man die Änderungen herunterladen kann.

<!--On a project for which you’re not the maintainer, it’s generally easier to have a branch like `master` always track `origin/master` and to do your work in topic branches that you can easily discard if they’re rejected.  Having work themes isolated into topic branches also makes it easier for you to rebase your work if the tip of the main repository has moved in the meantime and your commits no longer apply cleanly. For example, if you want to submit a second topic of work to the project, don’t continue working on the topic branch you just pushed up — start over from the main repository’s `master` branch:-->

Wenn Du nicht selbst Betreiber eines bestimmten Projektes bist, ist es im Allgemeinen einfacher, einen Branch `master` immer dem `origin/master` Branch tracken (xxx folgen) zu lassen und eigene Änderungen in Topic Branches vorzunehmen, die man leicht wieder löschen kann, wenn sie nicht akzeptiert werden. Wenn Du Aspekte Deiner Arbeit in Topic Branches isolierst, kannst Du sie außerdem recht leicht auf den letzten Stand des Hauptrepositories rebasen, falls das Hauptrepository in der Zwischenzeit weiter entwickelt wurde und Deine Commits nicht mehr sauber passen. Wenn Du beispielsweise an einem anderen Patch für das Projekt arbeiten willst, verwende dazu nicht weiter den gleichen Topic Branch, den Du gerade in Deinen Fork hochgeladen hast. Lege statt dessen einen neuen Topic Branch an, der wiederum auf dem `master` Branch des Hauptrepositories basiert.

	$ git checkout -b featureB origin/master
	$ (work)
	$ git commit
	$ git push myfork featureB
	$ (email maintainer)
	$ git fetch origin

<!--Now, each of your topics is contained within a silo — similar to a patch queue — that you can rewrite, rebase, and modify without the topics interfering or interdepending on each other as in Figure 5-16.-->

Deine Arbeit an den verschiedenen Patches sind jetzt in Deine Topic Branches isoliert – ähnlich wie in einer Patch Queue – sodass Du die einzelnen Topic Branches neu schreiben, rebasen und ändern kannst, ohne dass sie mit einander in Konflikt geraten (siehe Bild 5-16).

<!--Figure 5-16. Initial commit history with featureB work.-->

Insert 18333fig0516.png
Bild 5-16. Ursprüngliche Commit Historie mit dem `featureB` Branch

<!--Let’s say the project maintainer has pulled in a bunch of other patches and tried your first branch, but it no longer cleanly merges. In this case, you can try to rebase that branch on top of `origin/master`, resolve the conflicts for the maintainer, and then resubmit your changes:-->

Sagen wir, der Projekt Betreiber hat eine Reihe von Änderungen Dritter in das Projekt übernommen und Deine eigenen Änderungen lassen sich jetzt nicht mehr sauber mergen. In diesem Fall kannst Du Deine Änderungen auf dem neuen Stand des `origin/master` Branches rebasen, Konflikte beheben und Deine Arbeit erneut einreichen:

	$ git checkout featureA
	$ git rebase origin/master
	$ git push -f myfork featureA

<!--This rewrites your history to now look like Figure 5-17.-->

Das schreibt Deine Commit Historie neu, sodass sie jetzt so aussieht (Bild 5-17):

<!--Figure 5-17. Commit history after featureA work.-->

Insert 18333fig0517.png
Bild 5-17. Commit Historie nach dem rebase von `featureA`

<!--Because you rebased the branch, you have to specify the `-f` to your push command in order to be able to replace the `featureA` branch on the server with a commit that isn’t a descendant of it. An alternative would be to push this new work to a different branch on the server (perhaps called `featureAv2`).-->

Weil Du den Branch rebased hast, musst Du die Option `-f` verwenden, um den `featureA` Branch auf dem Server zu ersetzen, denn Du hast die Commit Historie umgeschrieben und nun ist ein Commit enthalten, von dem der gegenwärtig letzte Commit des externen Branches nicht abstammt. Eine Alternative dazu wäre, den Branch jetzt in einen neuen externen Branch zu pushen, z.B. `featureAv2`.

<!--Let’s look at one more possible scenario: the maintainer has looked at work in your second branch and likes the concept but would like you to change an implementation detail. You’ll also take this opportunity to move the work to be based off the project’s current `master` branch. You start a new branch based off the current `origin/master` branch, squash the `featureB` changes there, resolve any conflicts, make the implementation change, and then push that up as a new branch:-->

Schauen wir uns noch ein anderes Szenario an: der Projekt Betreiber hat sich Deine Arbeit angesehen und will die Änderungen übernehmen, aber er bittet dich, noch eine Kleinigkeit an der Implementierung zu ändern. Du willst die Gelegenheit außerdem nutzen, um Deine Änderungen neu auf den gegenwärtigen `master` Branch des Projektes zu basieren. Du legst dazu einen neuen Branch von `origin/master` an, übernimmst Deine Änderungen dahin, löst ggf. Konflikte auf, nimmst die angeforderte Änderung an der Implementierung vor und lädst die Änderungen auf den Server:

	$ git checkout -b featureBv2 origin/master
	$ git merge --no-commit --squash featureB
	$ (change implementation)
	$ git commit
	$ git push myfork featureBv2

<!--The `-\-squash` option takes all the work on the merged branch and squashes it into one non-merge commit on top of the branch you’re on. The `-\-no-commit` option tells Git not to automatically record a commit. This allows you to introduce all the changes from another branch and then make more changes before recording the new commit.-->

Die `--squash` Option bewirkt, dass alle Änderungen des Merge Branches (`featureB`) übernommen werden, ohne aber dass zusätzlich ein Merge Commit angelegt wird. Die `--no-commit` Option instruiert Git außerdem, nicht automatisch einen Commit anzulegen. Das erlaubt dir, sämtliche Änderungen aus dem anderen Branch zu übernehmen und dann weitere Änderungen vorzunehmen, bevor Du das Ganze dann in einem neuen Commit speicherst.

<!--Now you can send the maintainer a message that you’ve made the requested changes and they can find those changes in your `featureBv2` branch (see Figure 5-18).-->

Jetzt kannst Du dem Projekt Betreiber eine Nachricht schicken, dass Du die angeforderte Änderung vorgenommen hast und dass er Deine Arbeit in Deinem `featureBv2` Branch finden kann (siehe Bild 5-18).

<!--Figure 5-18. Commit history after featureBv2 work.-->

Insert 18333fig0518.png
Bild 5-18. Commit Historie mit dem neuen `featureBv2` Branch

<!--### Public Large Project ###-->
### Große öffentliche Projekte ###

<!--Many larger projects have established procedures for accepting patches — you’ll need to check the specific rules for each project, because they will differ. However, many larger public projects accept patches via a developer mailing list, so I’ll go over an example of that now.-->

Viele große Projekte haben einen etablierten Prozess, nach dem sie vorgehen, wenn es darum geht, Patches zu akzeptieren. Du musst Dich mit den jeweiligen Regeln vertraut machen, die in jedem Projekt ein bisschen anders sind. Allerdings akzeptieren viele große Projekte Patches per E-Mail über eine Entwickler Mailingliste (xxx oder einen Bugtracker, wie Rails xxx). Deshalb gehen wir auf dieses Beispiel als nächstes ein.

<!--The workflow is similar to the previous use case — you create topic branches for each patch series you work on. The difference is how you submit them to the project. Instead of forking the project and pushing to your own writable version, you generate e-mail versions of each commit series and e-mail them to the developer mailing list:-->

Der Workflow ist ähnlich wie im vorherigen Szenario. Du legst für jeden Patch oder jede Patch Serie einen Topic Branch an, in dem Du arbeitest. Der Unterschied besteht dann darin, auf welchem Wege Du die Änderungen an das Projekt schickst. Statt das Projekt zu forken und Änderungen in Deinen Fork hochzuladen, erzeugst Du eine E-Mail Version Deiner Commits und schickst sie als Patch an die Entwickler Mailingliste.

	$ git checkout -b topicA
	$ (work)
	$ git commit
	$ (work)
	$ git commit

<!--Now you have two commits that you want to send to the mailing list. You use `git format-patch` to generate the mbox-formatted files that you can e-mail to the list — it turns each commit into an e-mail message with the first line of the commit message as the subject and the rest of the message plus the patch that the commit introduces as the body. The nice thing about this is that applying a patch from an e-mail generated with `format-patch` preserves all the commit information properly, as you’ll see more of in the next section when you apply these patches:-->

Jetzt hast Du zwei Commits, die Du an die Mailingliste schicken willst. Du kannst den Befehl `git format-patch` verwenden, um aus diesen Commits Dateien zu erzeugen, die im mbox-Format formatiert sind und die Du per E-Mail verschicken kannst. Dieser Befehl macht aus jedem Commit eine E-Mail Datei. Die erste Zeile der Commit Meldung wird zum Betreff der E-Mail und der Rest der Commit Meldung sowie der Patch des Commits selbst wird zum Text der E-Mail. Das schöne daran ist, dass wenn man einen auf diese Weise erzeugten Patch benutzt, dann bleiben alle Commit Informationen erhalten. Du kannst das in den nächsten Beispielen sehen:

	$ git format-patch -M origin/master
	0001-add-limit-to-log-function.patch
	0002-changed-log-output-to-30-from-25.patch

<!--The `format-patch` command prints out the names of the patch files it creates. The `-M` switch tells Git to look for renames. The files end up looking like this:-->

Der Befehl `git format-patch` zeigt Dir die Namen der Patch Dateien an, die er erzeugt hat. (Die `-M` option weist Git an, nach umbenannten Dateien Ausschau zu halten.) Die Dateien sehen dann so aus:

	$ cat 0001-add-limit-to-log-function.patch
	From 330090432754092d704da8e76ca5c05c198e71a8 Mon Sep 17 00:00:00 2001
	From: Jessica Smith <jessica@example.com>
	Date: Sun, 6 Apr 2008 10:17:23 -0700
	Subject: [PATCH 1/2] add limit to log function

	Limit log functionality to the first 20

	---
	 lib/simplegit.rb |    2 +-
	 1 files changed, 1 insertions(+), 1 deletions(-)

	diff --git a/lib/simplegit.rb b/lib/simplegit.rb
	index 76f47bc..f9815f1 100644
	--- a/lib/simplegit.rb
	+++ b/lib/simplegit.rb
	@@ -14,7 +14,7 @@ class SimpleGit
	   end

	   def log(treeish = 'master')
	-    command("git log #{treeish}")
	+    command("git log -n 20 #{treeish}")
	   end

	   def ls_tree(treeish = 'master')
	--
	1.6.2.rc1.20.g8c5b.dirty

<!--You can also edit these patch files to add more information for the e-mail list that you don’t want to show up in the commit message. If you add text between the `-\-\-` line and the beginning of the patch (the `lib/simplegit.rb` line), then developers can read it; but applying the patch excludes it.-->

Du kannst diese Patch Dateien anschließend bearbeiten, z.B. um weitere Informationen für die Mailingliste hinzuzufügen, die Du nicht in der Commit Meldung haben willst. Wenn Du zusätzlichen Text zwischen der `---` Zeile und dem Anfang des Patches (der Zeile `lib/simplegit.rb` in diesem Fall), dann ist er für den Leser sichtbar, aber Git wird ihn ignorieren, wenn man den Patch verwendet.

<!--To e-mail this to a mailing list, you can either paste the file into your e-mail program or send it via a command-line program. Pasting the text often causes formatting issues, especially with "smarter" clients that don’t preserve newlines and other whitespace appropriately. Luckily, Git provides a tool to help you send properly formatted patches via IMAP, which may be easier for you. I’ll demonstrate how to send a patch via Gmail, which happens to be the e-mail agent I use; you can read detailed instructions for a number of mail programs at the end of the aforementioned `Documentation/SubmittingPatches` file in the Git source code.-->

Um das jetzt an die Mailingliste zu schicken, kannst Du entweder die Datei per copy-and-paste in Dein E-Mail Programm kopieren, als Anhang an eine E-Mail anhängen oder Du kannst die Dateien mit einem Befehlszeilen Programm direkt verschicken. Patches zu kopieren verursacht oft Formatierungsprobleme – insbesondere mit „smarten“ E-Mail Clients, die die Dinge umformatieren, die man einfügt. Zum Glück bringt Git aber ein Tool mit, mit dem man Patches in ihrer korrekten Formatierung über IMAP verschicken kann. In folgendem Beispiel zeige ich, wie man Patches über Gmail verschicken kann, welches der E-Mail Client ist, den ich selbst verwende. Darüberhinaus findest Du ausführliche Beschreibungen für zahlreiche E-Mail Programme am Ende der schon erwähnten Datei `Documentation/SubmittingPatches` im Git Quellcode.

<!--First, you need to set up the imap section in your `~/.gitconfig` file. You can set each value separately with a series of `git config` commands, or you can add them manually; but in the end, your config file should look something like this:-->

Zunächst musst Du die IMAP Sektion in Deiner `~/.gitconfig` Datei ausfüllen. Du kannst jeden Wert separat mit dem Befehl `git config` eingeben oder Du kannst die Datei öffnen und sie manuell eingeben. Im Endeffekt sollte Deine `~/.gitconfig` Datei in etwa so aussehen:

	[imap]
	  folder = "[Gmail]/Drafts"
	  host = imaps://imap.gmail.com
	  user = user@gmail.com
	  pass = p4ssw0rd
	  port = 993
	  sslverify = false

<!--If your IMAP server doesn’t use SSL, the last two lines probably aren’t necessary, and the host value will be `imap://` instead of `imaps://`.-->
<!--When that is set up, you can use `git imap-send` to place the patch series in the Drafts folder of the specified IMAP server:-->

Wenn Dein IMAP Server kein SSL verwendet, kannst Du die letzten beiden Zeilen wahrscheinlich weglassen und der `host` dürfte mit `imap://` und nicht `imaps://` beginnen. Wenn Du diese Einstellungen konfiguriert hast, kannst Du `git imap-send` verwenden, um Deine Patches in den Entwurfsordner des angegebenen IMAP Servers zu kopieren:

	$ cat *.patch |git imap-send
	Resolving imap.gmail.com... ok
	Connecting to [74.125.142.109]:993... ok
	Logging in...
	sending 2 messages
	100% (2/2) done

<!--At this point, you should be able to go to your Drafts folder, change the To field to the mailing list you’re sending the patch to, possibly CC the maintainer or person responsible for that section, and send it off.-->

Jetzt kannst Du in Deinen Entwürfe-Ordner wechseln, die Mailingliste an die Du den Patch senden möchtest im An-Feld setzen, vielleicht noch den Maintainer oder die verantwortliche Person in das CC-Feld einfügen und dann das Ganze losschicken.

<!--You can also send the patches through an SMTP server. As before, you can set each value separately with a series of `git config` commands, or you can add them manually in the sendemail section in your `~/.gitconfig` file:-->

Man kann Patches auch über einen SMTP-Server schicken. Wie im letzen Beispiel, kann man auch hier jeden einzelnen Wert mit einer Reihe von `git config` Kommandos setzen. Oder aber Du änderst die Sektion sendemail in Deiner `~/.gitconfig` Datei manuell:

	[sendemail]
	  smtpencryption = tls
	  smtpserver = smtp.gmail.com
	  smtpuser = user@gmail.com
	  smtpserverport = 587

<!--After this is done, you can use `git send-email` to send your patches:-->

Nach der Änderungen kannst Du mit `git send-email` die Patches abschicken:

	$ git send-email *.patch
	0001-added-limit-to-log-function.patch
	0002-changed-log-output-to-30-from-25.patch
	Who should the emails appear to be from? [Jessica Smith <jessica@example.com>]
	Emails will be sent from: Jessica Smith <jessica@example.com>
	Who should the emails be sent to? jessica@example.com
	Message-ID to be used as In-Reply-To for the first email? y

<!--Then, Git spits out a bunch of log information looking something like this for each patch you’re sending:-->

Git gibt dann für jeden Patch, den Du verschickst, ein paar Log Informationen aus, die in etwa so aussehen:

	(mbox) Adding cc: Jessica Smith <jessica@example.com> from
	  \line 'From: Jessica Smith <jessica@example.com>'
	OK. Log says:
	Sendmail: /usr/sbin/sendmail -i jessica@example.com
	From: Jessica Smith <jessica@example.com>
	To: jessica@example.com
	Subject: [PATCH 1/2] added limit to log function
	Date: Sat, 30 May 2009 13:29:15 -0700
	Message-Id: <1243715356-61726-1-git-send-email-jessica@example.com>
	X-Mailer: git-send-email 1.6.2.rc1.20.g8c5b.dirty
	In-Reply-To: <y>
	References: <y>

	Result: OK

<!--At this point, you should be able to go to your Drafts folder, change the To field to the mailing list you’re sending the patch to, possibly CC the maintainer or person responsible for that section, and send it off.-->

Jetzt solltest Du in den Entwurfsordner Deines E-Mail Clients gehen, als Empfänger Adresse die jeweilige Mailingliste angeben, möglicherweise ein CC an den Projektbetreiber oder einen anderen Verantwortlichen setzen und die E-Mail dann verschicken können.

<!--### Summary ###-->
### Zusammenfassung ###

<!--This section has covered a number of common workflows for dealing with several very different types of Git projects you’re likely to encounter and introduced a couple of new tools to help you manage this process. Next, you’ll see how to work the other side of the coin: maintaining a Git project. You’ll learn how to be a benevolent dictator or integration manager.-->

Wir haben jetzt eine Reihe von Workflows besprochen, die für jeweils sehr verschiedene Arten von Projekten üblich sind und denen Du vermutlich begegnen wirst. Wir haben außerdem einige neue Tools besprochen, die dabei hilfreich sind, diese Workflows umzusetzen. Als nächstes werden wir auf die andere Seite dieser Medaille eingehen: wie Du selbst ein Git Projekt betreiben kannst. Du wirst lernen, wie Du als „wohlwollender Diktator“ oder als Integration Manager arbeiten kannst.

<!--## Maintaining a Project ##-->
## Ein Projekt betreiben ##

<!--In addition to knowing how to effectively contribute to a project, you’ll likely need to know how to maintain one. This can consist of accepting and applying patches generated via `format-patch` and e-mailed to you, or integrating changes in remote branches for repositories you’ve added as remotes to your project. Whether you maintain a canonical repository or want to help by verifying or approving patches, you need to know how to accept work in a way that is clearest for other contributors and sustainable by you over the long run.-->

Neben dem Wissen, das Du brauchst, um zu einem bestehenden Projekt Änderungen beizutragen, wirst Du vermutlich wissen wollen, wie Du selbst ein Projekt betreiben kannst. Dazu willst Du Patches akzeptieren und anwenden, die per `git format-patch` erzeugt und Dir per E-Mail geschickt wurden. Oder Du willst Änderungen aus externen Branches übernehmen, die Du zu Deinem Projekt hinzugefügt hast. Ob Du nun für das Hauptrepository verantwortlich bist oder ob Du dabei helfen willst, Patches zu verifizieren und zu bestätigen – in beiden Fällen musst Du wissen, wie Du Änderungen in einer Weise übernehmen kannst, die für andere Mitarbeiter nachvollziehbar und für Dich selbst tragbar ist.

<!--### Working in Topic Branches ###-->
### In Topic Branches arbeiten ###

<!--When you’re thinking of integrating new work, it’s generally a good idea to try it out in a topic branch — a temporary branch specifically made to try out that new work. This way, it’s easy to tweak a patch individually and leave it if it’s not working until you have time to come back to it. If you create a simple branch name based on the theme of the work you’re going to try, such as `ruby_client` or something similarly descriptive, you can easily remember it if you have to abandon it for a while and come back later. The maintainer of the Git project tends to namespace these branches as well — such as `sc/ruby_client`, where `sc` is short for the person who contributed the work.-->
<!--As you’ll remember, you can create the branch based off your master branch like this:-->

Wenn Du Änderungen von anderen übernehmen willst, ist normalerweise eine gute Idee, sie in einem Topic Branch auszuprobieren – d.h., einem temporären Branch, dessen Zweck nur darin besteht, die jeweiligen Änderungen auszuprobieren. Auf diese Weise ist es einfach, Patches ggf. anzupassen oder sie im Zweifelsfall im Topic Branch liegen zu lassen, wenn sie nicht funktionieren und Du im Moment nicht die Zeit hast, Dich weiter damit zu befassen. Es ist empfehlenswert, Topic Branches Namen zu geben, die gut kommunizieren, worum es sich bei den jeweiligen Änderungen dreht, wie z.B. `ruby_client` oder etwas ähnlich aussagekräftiges, das Dir hilft, Dich daran zu erinnern. Der Projekt Betreiber des Git Projektes selbst vergibt Namensräume für solche Branches – wie z.B. `sc/ruby_client`, wobei `sc` ein Kürzel für den jeweiligen Autor des Patches ist. Wie Du inzwischen weißt, kannst Du einen neuen Branch, der auf dem gegenwärtigen `master` Branch basiert, wie folgt erzeugen (xxx falsch, das stimmt nur, wenn `master` der aktuelle Branch ist xxx):

	$ git branch sc/ruby_client master

<!--Or, if you want to also switch to it immediately, you can use the `checkout -b` command:-->

Oder, wenn außerdem direkt zu dem neuen Branch wechseln willst, kannst Du die `-b` für den `git checkout` Befehl verwenden:

	$ git checkout -b sc/ruby_client master

<!--Now you’re ready to add your contributed work into this topic branch and determine if you want to merge it into your longer-term branches.-->

Nachdem Du jetzt einen Topic Branch angelegt hast, kannst Du die Änderungen zu diesem Branch hinzufügen, um herauszufinden, ob Du sie dauerhaft in einen offiziellen Branch übernehmen willst.

<!--### Applying Patches from E-mail ###-->
### Patches aus E-Mails verwenden ###

<!--If you receive a patch over e-mail that you need to integrate into your project, you need to apply the patch in your topic branch to evaluate it. There are two ways to apply an e-mailed patch: with `git apply` or with `git am`.-->

Wenn Du einen Patch, den Du auf Dein Projekt anwenden willst, per E-Mail erhältst, gibt es zwei Möglichkeiten, das zu tun: `git apply` und `git am`.

<!--#### Applying a Patch with apply ####-->
#### Einen Patch verwenden: git apply ####

<!--If you received the patch from someone who generated it with the `git diff` or a Unix `diff` command, you can apply it with the `git apply` command. Assuming you saved the patch at `/tmp/patch-ruby-client.patch`, you can apply the patch like this:-->

Wenn der Patch mit `git diff` oder dem Unix Befehl `diff` erzeugt wurde, dann kannst Du ihn mit dem Befehl `git apply` anwenden. Nehmen wir an, Du hast den Patch nach `/tmp/patch-ruby-client.patch` gespeichert. Dann kannst Du ihn wie folgt verwenden:

	$ git apply /tmp/patch-ruby-client.patch

<!--This modifies the files in your working directory. It’s almost identical to running a `patch -p1` command to apply the patch, although it’s more paranoid and accepts fewer fuzzy matches than patch. It also handles file adds, deletes, and renames if they’re described in the `git diff` format, which `patch` won’t do. Finally, `git apply` is an "apply all or abort all" model where either everything is applied or nothing is, whereas `patch` can partially apply patchfiles, leaving your working directory in a weird state. `git apply` is overall much more paranoid than `patch`. It won’t create a commit for you — after running it, you must stage and commit the changes introduced manually.-->

Das ändert die Dateien in Deinem Git Arbeitsverzeichnis. Das ist fast das selbe wie wenn Du den Unix Befehl `patch -p1` verwendest. Der Git Befehl ist aber paranoider und akzeptiert nicht so viele unklare Übereinstimmungen. Außerdem kann er mit neu hinzugefügten, gelöschten und umbenannten Dateien umgehen, was der Unix Befehl `patch` nicht kann. Schließlich ist `git apply` ein „alles oder nichts“ Befehl, der entweder alle Änderungen übernimmt oder gar keine (wenn bei einem etwas schief geht), während `patch` Änderungen auch teilweise übernimmt, sodass er Dein Arbeitsverzeichnis gegebenenfalls in einem unbrauchbaren Zustand hinterlässt. `git apply` ist also insgesamt strenger als `patch`. Es legt im übrigen keinen Commit für Dich an. Nachdem Du `git apply` ausgeführt hast, musst Du die Änderungen manuell zur Staging Area hinzufügen und comitten.

<!--You can also use git apply to see if a patch applies cleanly before you try actually applying it — you can run `git apply -\-check` with the patch:-->

Du kannst `git apply --check` verwenden, um zu testen, ob der Patch sauber anwendbar wäre:

	$ git apply --check 0001-seeing-if-this-helps-the-gem.patch
	error: patch failed: ticgit.gemspec:1
	error: ticgit.gemspec: patch does not apply

<!--If there is no output, then the patch should apply cleanly. This command also exits with a non-zero status if the check fails, so you can use it in scripts if you want.-->

Wenn dieser Befehl nichts ausgibt, sollte der Befehl sauber anwendbar sein.

<!--#### Applying a Patch with am ####-->
#### Einen Patch verwenden: git am ####

<!--If the contributor is a Git user and was good enough to use the `format-patch` command to generate their patch, then your job is easier because the patch contains author information and a commit message for you. If you can, encourage your contributors to use `format-patch` instead of `diff` to generate patches for you. You should only have to use `git apply` for legacy patches and things like that.-->

Wenn der Autor des Patches selbst mit Git arbeitet, kann er Dir das Leben leichter machen, indem er `git format-patch` verwender, um seinen Patch zu erzeugen: der Patch wird dann die Commit Informationen über den Autor sowie die Commit Meldung enthalten. Es ist also empfehlenswert, Entwickler darum zu bitten und zu ermutigen, `git format-patch` statt `git diff` zu verwenden. Du wirst dann `git apply` nur sehr selten anwenden müssen (xxx legacy patches ??? xxx)

<!--To apply a patch generated by `format-patch`, you use `git am`. Technically, `git am` is built to read an mbox file, which is a simple, plain-text format for storing one or more e-mail messages in one text file. It looks something like this:-->

Um einen Patch zu verwenden, der mit `git format-patch` erzeugt wurde, benutzt Du den Befehl `git am`. Technisch gesehen ist `git am` ein Befehl, der eine mbox Datei lesen kann, d.h. eine einfache Nur-Text-Datei, die eine oder mehrere E-Mails enthalten kann. Eine solche Datei sieht in etwa wie folgt aus:

	From 330090432754092d704da8e76ca5c05c198e71a8 Mon Sep 17 00:00:00 2001
	From: Jessica Smith <jessica@example.com>
	Date: Sun, 6 Apr 2008 10:17:23 -0700
	Subject: [PATCH 1/2] add limit to log function

	Limit log functionality to the first 20

<!--This is the beginning of the output of the format-patch command that you saw in the previous section. This is also a valid mbox e-mail format. If someone has e-mailed you the patch properly using git send-email, and you download that into an mbox format, then you can point git am to that mbox file, and it will start applying all the patches it sees. If you run a mail client that can save several e-mails out in mbox format, you can save entire patch series into a file and then use git am to apply them one at a time.-->

Das ist der Anfang der Ausgabe des `git format-patch` Befehls, die Du im vorherigen Abschnitt gesehen hast – und außerdem valides mbox E-Mail Format. Wenn Du eine solche Datei im mbox Format erhalten hast und der Absender `git send-email` korrekt verwendet hat, kannst Du `git am` mit der Datei verwenden und alle darin enthaltenen Patches werden auf Dein Projekt angewendet. Wenn Du einen E-Mail Client verwendest, der mehrere E-Mails im mbox Format in einer Datei speichern oder exportieren kann, kannst Du auch eine ganze Reihe von Patches in eine einzige Datei speichern und dann `git am` verwenden, um sie nacheinander anzuwenden.

<!--However, if someone uploaded a patch file generated via `format-patch` to a ticketing system or something similar, you can save the file locally and then pass that file saved on your disk to `git am` to apply it:-->

Wenn jemand einen Patch, der mit `git format-patch` erzeugt wurde, in einem Ticketsystem oder ähnlichem abgelegt hat, kannst Du die Datei lokal speichern und dann ebenfalls `git am` ausführen, um den Patch anzuwenden:

	$ git am 0001-limit-log-function.patch
	Applying: add limit to log function

<!--You can see that it applied cleanly and automatically created the new commit for you. The author information is taken from the e-mail’s `From` and `Date` headers, and the message of the commit is taken from the `Subject` and body (before the patch) of the e-mail. For example, if this patch was applied from the mbox example I just showed, the commit generated would look something like this:-->

Der Patch passte sauber auf die Codebase und hat automatisch einen neuen Commit angelegt. Die Autor Information wurde aus den `From` und `Date` Headern der E-Mail übernommen und die Commit Meldung aus dem Subject und Body der E-Mail. Wenn der Patch z.B. aus dem mbox Beispiel von oben stammt, sieht der resultierende Commit wie folgt aus:

	$ git log --pretty=fuller -1
	commit 6c5e70b984a60b3cecd395edd5b48a7575bf58e0
	Author:     Jessica Smith <jessica@example.com>
	AuthorDate: Sun Apr 6 10:17:23 2008 -0700
	Commit:     Scott Chacon <schacon@gmail.com>
	CommitDate: Thu Apr 9 09:19:06 2009 -0700

	   add limit to log function

	   Limit log functionality to the first 20

<!--The `Commit` information indicates the person who applied the patch and the time it was applied. The `Author` information is the individual who originally created the patch and when it was originally created.-->

Das `Commit` Feld zeigt den Namen desjenigen, der den Patch angewendet hat und `CommitDate` das jeweilige Datum und Uhrzeit. Die Felder `Author` und `AuthorDate` geben an, wer den Commit wann angelegt hat.

<!--But it’s possible that the patch won’t apply cleanly. Perhaps your main branch has diverged too far from the branch the patch was built from, or the patch depends on another patch you haven’t applied yet. In that case, the `git am` process will fail and ask you what you want to do:-->

Es ist allerdings möglich, dass der Patch nicht sauber auf den gegenwärtigen Code passt. Möglicherweise unterscheidet sich der jeweilige Branch inzwischen erheblich von dem Zustand, in dem er sich befand, als die in dem Patch enthaltenen Änderungen geschrieben wurden. In dem Fall wird `git am` fehlschlagen und Dir mitteilen, was zu tun ist:

	$ git am 0001-seeing-if-this-helps-the-gem.patch
	Applying: seeing if this helps the gem
	error: patch failed: ticgit.gemspec:1
	error: ticgit.gemspec: patch does not apply
	Patch failed at 0001.
	When you have resolved this problem run "git am --resolved".
	If you would prefer to skip this patch, instead run "git am --skip".
	To restore the original branch and stop patching run "git am --abort".

<!--This command puts conflict markers in any files it has issues with, much like a conflicted merge or rebase operation. You solve this issue much the same way — edit the file to resolve the conflict, stage the new file, and then run `git am -\-resolved` to continue to the next patch:-->

Der Befehl fügt Konfliktmarkierungen in allen problematischen Dateien ein, so wie bei einem konfligierenden Merge oder Rebase. Und Du kannst den Konflikt in der selben Weise beheben: die Datei bearbeiten, die Änderungen zur Staging Area hinzufügen und dann `git am --resolved` ausführen, um mit dem jeweils nächsten Patch (falls vorhanden) fortzufahren.

	$ (fix the file)
	$ git add ticgit.gemspec
	$ git am --resolved
	Applying: seeing if this helps the gem

<!--If you want Git to try a bit more intelligently to resolve the conflict, you can pass a `-3` option to it, which makes Git attempt a three-way merge. This option isn’t on by default because it doesn’t work if the commit the patch says it was based on isn’t in your repository. If you do have that commit — if the patch was based on a public commit — then the `-3` option is generally much smarter about applying a conflicting patch:-->

Wenn Du willst, dass Git versucht, einen Konflikt etwas intelligenter zu lösen, kannst Du die `-3` Option angeben, sodass Git einen 3-Wege-Merge versucht. Dies ist deshalb nicht der Standard, weil ein 3-Wege-Merge nicht funktioniert, wenn der Commit, auf dem der Patch basiert, nicht Teil Deines Repositories ist. Wenn Du den Commit allerdings in Deiner Historie hast, d.h. wenn der Patch auf einem öffentlichen Commit basiert, dann ist die `-3` Option oft die bessere Variante, um einen konfligierenden Patch anzuwenden:

	$ git am -3 0001-seeing-if-this-helps-the-gem.patch
	Applying: seeing if this helps the gem
	error: patch failed: ticgit.gemspec:1
	error: ticgit.gemspec: patch does not apply
	Using index info to reconstruct a base tree...
	Falling back to patching base and 3-way merge...
	No changes -- Patch already applied.

<!--In this case, I was trying to apply a patch I had already applied. Without the `-3` option, it looks like a conflict.-->

In diesem Fall habe ich versucht, einen Patch anzuwenden, der ich bereits zuvor angewendet hatte. Ohne die `-3` Option würde ich einen Konflikt erhalten.

<!--If you’re applying a number of patches from an mbox, you can also run the `am` command in interactive mode, which stops at each patch it finds and asks if you want to apply it:-->

Wenn Du eine Reihe von Patches aus einer Datei im mbox Format anwendest, kannst Du außerdem den `git am` Befehl in einem interaktiven Modus ausführen. In diesem Modus hält Git bei jedem Patch an und fragt Dich jeweils, ob Du den Patch anwenden willst:

	$ git am -3 -i mbox
	Commit Body is:
	--------------------------
	seeing if this helps the gem
	--------------------------
	Apply? [y]es/[n]o/[e]dit/[v]iew patch/[a]ccept all

<!--This is nice if you have a number of patches saved, because you can view the patch first if you don’t remember what it is, or not apply the patch if you’ve already done so.-->

Das ist praktisch, wenn Du eine ganze Reihe von Patches in einer Datei hast. Du kannst jeweils Patches anzeigen, an die Du Dich nicht erinnern kannst, oder Patches auslassen, z.B. weil Du sie schon zuvor angewendet hattest.

<!--When all the patches for your topic are applied and committed into your branch, you can choose whether and how to integrate them into a longer-running branch.-->

Nachdem Du alle Patches in Deinem Topic Branch angewendet hast, kannst Du die Änderungen durchsehen, testen und entscheiden, ob Du sie in einen dauerhaft bestehenden Branch übernehmen willst.

<!--### Checking Out Remote Branches ###-->
### Checking Out Remote Branches ###

<!--If your contribution came from a Git user who set up their own repository, pushed a number of changes into it, and then sent you the URL to the repository and the name of the remote branch the changes are in, you can add them as a remote and do merges locally.-->

Möglicherweise kommen die Änderungen aber nicht als Patch sondern von einem Git Anwender, der sein eigenes Repository aufgesetzt hat, seine Änderungen dorthin hochgeladen und Dir dann die URL des Repositories und den Namen des Branches geschickt hat. In diesem Fall kannst Du das Repository als ein „remote“ (externes Repository) hinzufügen und die Änderungen lokal mergen.

<!--For instance, if Jessica sends you an e-mail saying that she has a great new feature in the `ruby-client` branch of her repository, you can test it by adding the remote and checking out that branch locally:-->

Wenn Dir z.B. Jessica eine E-Mail schickt und mitteilt, dass sie ein großartiges, neues Feature im `ruby-client` Branch ihres Repositories hat, dann kannst Du das Feature testen, indem Du das Repository als externes Repository Deines Projektes konfigurieren und den Branch lokal auscheckst:

	$ git remote add jessica git://github.com/jessica/myproject.git
	$ git fetch jessica
	$ git checkout -b rubyclient jessica/ruby-client

<!--If she e-mails you again later with another branch containing another great feature, you can fetch and check out because you already have the remote setup.-->

Wenn sie Dir später erneut eine E-Mail mit einem anderen Branch schickt, der ein anderes, großartiges Feature enthält, dann kannst Du diesen Branch direkt herunterladen und auschecken, weil Du das externe Repository noch konfiguriert hast.

<!--This is most useful if you’re working with a person consistently. If someone only has a single patch to contribute once in a while, then accepting it over e-mail may be less time consuming than requiring everyone to run their own server and having to continually add and remove remotes to get a few patches. You’re also unlikely to want to have hundreds of remotes, each for someone who contributes only a patch or two. However, scripts and hosted services may make this easier — it depends largely on how you develop and how your contributors develop.-->

Dies ist insbesondere nützlich, wenn Du mit jemandem regelmäßig zusammen arbeitest. Wenn jemand lediglich gelegentlich einen einzelnen Patch beiträgt, dann ist es wahrscheinlich weniger aufwendig, ihn per E-Mail zu akzeptieren, als von jedem zu erwarten, einen eigenen Server zu betreiben, und selbst ständig externe Repositories hinzuzufügen und zu entfernen. Du wirst kaum hunderte von externen Repositories verwalten wollen, nur um von jedem ein paar Änderungen zu erhalten. Auf der anderen Seite erleichtern Dir Scripts und Hosted Services diesen Prozess. Es hängt also alles davon ab, wie Du selbst und wie Deine Mitarbeiter entwickeln.

<!--The other advantage of this approach is that you get the history of the commits as well. Although you may have legitimate merge issues, you know where in your history their work is based; a proper three-way merge is the default rather than having to supply a `-3` and hope the patch was generated off a public commit to which you have access.-->

Wenn Du mit jemandem nicht regelmäßig zusammen arbeitest, aber trotzdem aus ihrem Repository mergen willst, dann kannst Du dem `git pull` Befehl die URL ihres externen Repositories übergeben. Das lädt den entsprechenden Branch einmalig herunter und merged ihn in Deinen aktuellen Branch, ohne aber die externe URL als eine Referenz zu speichern:

<!--If you aren’t working with a person consistently but still want to pull from them in this way, you can provide the URL of the remote repository to the `git pull` command. This does a one-time pull and doesn’t save the URL as a remote reference:-->

	$ git pull git://github.com/onetimeguy/project.git
	From git://github.com/onetimeguy/project
	 * branch            HEAD       -> FETCH_HEAD
	Merge made by recursive.

<!--### Determining What Is Introduced ###-->
### Neuigkeiten durchsehen ###

<!--Now you have a topic branch that contains contributed work. At this point, you can determine what you’d like to do with it. This section revisits a couple of commands so you can see how you can use them to review exactly what you’ll be introducing if you merge this into your main branch.-->

Du hast jetzt einen Topic Branch, der die neuen Änderungen enthält, und kannst jetzt herausfinden, was Du damit anfangen willst. In diesem Abschnitt gehen wir noch mal auf einige Befehle ein, die nützlich sind, um herauszufinden, welche Änderungen Du übernehmen würdest, wenn Du den Topic Branch in Deinen Hauptbranch mergest.

<!--It’s often helpful to get a review of all the commits that are in this branch but that aren’t in your master branch. You can exclude commits in the master branch by adding the `-\-not` option before the branch name. For example, if your contributor sends you two patches and you create a branch called `contrib` and applied those patches there, you can run this:-->

Es ist in der Regel hilfreich, sich Commits anzusehen, die sich in diesem Branch, nicht aber im `master` Branch befinden. Du kannst Commits aus dem `master` Branch ausschließen, indem Du die `--not` Option verwendest. Wenn Du beispielsweise zwei neue Commits erhalten und sie in einen Topic Branch `contrib` übernommen hast, kannst Du folgendes tun:

	$ git log contrib --not master
	commit 5b6235bd297351589efc4d73316f0a68d484f118
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Fri Oct 24 09:53:59 2008 -0700

	    seeing if this helps the gem

	commit 7482e0d16d04bea79d0dba8988cc78df655f16a0
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Mon Oct 22 19:38:36 2008 -0700

	    updated the gemspec to hopefully work better

<!--To see what changes each commit introduces, remember that you can pass the `-p` option to `git log` and it will append the diff introduced to each commit.-->

Wie Du schon gelernt hast, kannst Du außerdem die Option `-p` verwenden, um zu sehen, welche Diffs die Commits enthalten.

<!--To see a full diff of what would happen if you were to merge this topic branch with another branch, you may have to use a weird trick to get the correct results. You may think to run this:-->

Wenn Du ein vollständiges Diff aller Änderungen sehen willst, die Dein Topic Branch gegenüber z.B. dem `master` Branch enthält, brauchst Du einen Trick. Möglicherweise würdest Du zuerst das hier ausprobieren:

	$ git diff master

<!--This command gives you a diff, but it may be misleading. If your `master` branch has moved forward since you created the topic branch from it, then you’ll get seemingly strange results. This happens because Git directly compares the snapshots of the last commit of the topic branch you’re on and the snapshot of the last commit on the `master` branch. For example, if you’ve added a line in a file on the `master` branch, a direct comparison of the snapshots will look like the topic branch is going to remove that line.-->

Der Befehl gibt Dir ein Diff aus, kann aber irreführend sein. Wenn im `master` Branch Änderungen committed wurden, seit der Branch angelegt wurde, erhältst Du scheinbar merkwürdige Ergebnisse. Das liegt daran, dass Git den Snapshot des letzten Commits des Topic Branches, in dem Du Dich momentan befindest, mit dem letzten Commit des `master` Branches vergleicht. Wenn Du beispielsweise eine Zeile in einer Datei im `master` branch hinzugefügt hast, scheint der direkte Vergleich auszusagen, dass diese Zeile im Topic Branch entfernt wurde.

<!--If `master` is a direct ancestor of your topic branch, this isn’t a problem; but if the two histories have diverged, the diff will look like you’re adding all the new stuff in your topic branch and removing everything unique to the `master` branch.-->

Wenn `master` ein direkter Vorfahr Deines Topic Branches ist, ist das kein Problem. Aber wenn sich die beiden Historien auseinander bewegt (xxx) haben, dann scheint das Diff auszusagen, dass Du alle Neuigkeiten im Topic Branch hinzufügst und alle Neuigkeiten im `master` Branch entfernst.

<!--What you really want to see are the changes added to the topic branch — the work you’ll introduce if you merge this branch with master. You do that by having Git compare the last commit on your topic branch with the first common ancestor it has with the master branch.-->

Was Du aber eigentlich wissen willst, ist welche Änderungen der Topic Branch hinzugefügt hat, d.h. die Änderungen, die Du in den `master` Branch neu einführen würdest, wenn Du den Topic Branch mergest. Dieses Ergebnis erhältst Du, wenn Du den letzten Commit im Topic Branch mit dem letzten Commit vergleichst, den der Topic Branch mit `master` gemeinsam hat.

<!--Technically, you can do that by explicitly figuring out the common ancestor and then running your diff on it:-->

Technisch gesehen könntest Du den letzten gemeinsamen Commit explizit erfragen und dann ein Diff darauf ausführen:

	$ git merge-base contrib master
	36c7dba2c95e6bbb78dfa822519ecfec6e1ca649
	$ git diff 36c7db

<!--However, that isn’t convenient, so Git provides another shorthand for doing the same thing: the triple-dot syntax. In the context of the `diff` command, you can put three periods after another branch to do a `diff` between the last commit of the branch you’re on and its common ancestor with another branch:-->

Das ist natürlich nicht sonderlich bequem, weshalb Git eine Kurzform dafür definiert: die triple-dot Syntax (xxx). Im Context des `git diff` Befehls bewirkt dies, dass Du ein Diff erhältst, das den letzten gemeinsamen Commit der Histories beider angegebener Branches mit dem letzten Commit des zuletzt angegebenen Branches vergleicht:

	$ git diff master...contrib

<!--This command shows you only the work your current topic branch has introduced since its common ancestor with master. That is a very useful syntax to remember.-->

Dieser Befehl zeigt Dir diejenigen Änderungen, die im Topic Branch eingeführt wurden, die aber noch nicht in `master` enthalten sind.

<!--### Integrating Contributed Work ###-->
### Beiträge anderer integrieren ###

<!--When all the work in your topic branch is ready to be integrated into a more mainline branch, the question is how to do it. Furthermore, what overall workflow do you want to use to maintain your project? You have a number of choices, so I’ll cover a few of them.-->

Sobald Du die Änderungen in Deinem Topic Branch in einen dauerhafteren Branch übernehmen willst, fragt sich, wie Du das anstellen kannst. Und welchen generellen Workflow willst Du verwenden, um das Projekt zu pflegen? Wir werden eine Reihe von Möglichkeiten besprechen, die Dir zur Verfügung stehen.

<!--#### Merging Workflows ####-->
#### Merge Workflows ####

<!--One simple workflow merges your work into your `master` branch. In this scenario, you have a `master` branch that contains basically stable code. When you have work in a topic branch that you’ve done or that someone has contributed and you’ve verified, you merge it into your master branch, delete the topic branch, and then continue the process.  If we have a repository with work in two branches named `ruby_client` and `php_client` that looks like Figure 5-19 and merge `ruby_client` first and then `php_client` next, then your history will end up looking like Figure 5-20.-->

Eine einfache Möglichkeit besteht darin, Deine Arbeit einfach in den `master` Branch zu mergen. In diesem Workflow hast Du einen `master` Branch, der eine stabilen Code beinhaltet. Wenn Du Änderungen in einem Topic Branch hast, die von Dir selbst oder jemand anderem geschrieben und die verifiziert sind, dann mergest Du diesen Topic Branch in den `master` Branch, löschst den Topic Branch und fährst mit diesem Prozess so fort. Wenn es ein Repository mit zwei Branches gibt, die `ruby_client` und `php_client` heißen (wie in Bild 5-19), und Du mergest `ruby_client` zuerst und `php_client` danach, dann wird die Historie danach aussehen wie im Bild 5-20.

<!--Figure 5-19. History with several topic branches.-->

Insert 18333fig0519.png
Bild 5-19. Historie mit verschiedenen Topic Branches

<!--Figure 5-20. After a topic branch merge.-->

Insert 18333fig0520.png
Bild 5-20. Nach dem Merge mit verschiedenen Topic Branches

<!--That is probably the simplest workflow, but it’s problematic if you’re dealing with larger repositories or projects.-->

Dies ist vermutlich der einfachste, mögliche Workflow. Er ist allerdings für große Repositories oder Projekte manchmal problematisch.

<!--If you have more developers or a larger project, you’ll probably want to use at least a two-phase merge cycle. In this scenario, you have two long-running branches, `master` and `develop`, in which you determine that `master` is updated only when a very stable release is cut and all new code is integrated into the `develop` branch. You regularly push both of these branches to the public repository. Each time you have a new topic branch to merge in (Figure 5-21), you merge it into `develop` (Figure 5-22); then, when you tag a release, you fast-forward `master` to wherever the now-stable `develop` branch is (Figure 5-23).-->

Wenn Du mehr Entwickler oder ein größeres Projekt hast, wirst Du in der Regel einen Merge Zyklus mit mindestens zwei Phasen verwenden wollen. In einem solchen Workflow hast Du dann zwei dauerhafte Branches, z.B. `master` und `develop`, wobei `master` ausschließlich sehr stabile Releases enthält und neuer Code im `develop` Branch integriert wird. Jedes Mal, wenn Du einen neuen Topic Branch mergen willst, mergest Du ihn nach `develop` (Bild 5-22). Und nur dann, wenn Du einen Release taggen willst, führst Du ein fast-forward (xxx) it `master` bis zum gegenwärtigen, nun stabilen Status des `develop` Branch durch.

<!--Figure 5-21. Before a topic branch merge.-->

Insert 18333fig0521.png
Bild 5-21. Vor dem Topic Branch Merge

<!--Figure 5-22. After a topic branch merge.-->

Insert 18333fig0522.png
Bild 5-22. Nach dem Topic Branch Merge

<!--Figure 5-23. After a topic branch release.-->

Insert 18333fig0523.png
Bild 5-23. Nach dem Topic Branch Release

<!--This way, when people clone your project’s repository, they can either check out master to build the latest stable version and keep up to date on that easily, or they can check out develop, which is the more cutting-edge stuff.-->
<!--You can also continue this concept, having an integrate branch where all the work is merged together. Then, when the codebase on that branch is stable and passes tests, you merge it into a develop branch; and when that has proven itself stable for a while, you fast-forward your master branch.-->

Auf diese Weise kann jeder, der Dein Repository klont, auf einfache Weise Deinen aktuellen `master` Branch verwenden und ihn auf neue Releases aktualisieren. Oder er kann den `develop` Branch ausprobieren, in dem sich die jeweils letzten, brandneuen Änderungen befinden. Du kannst dieses Konzept noch weiterführen, indem Du einen `integrate` Branch pflegst, in den neue Änderungen jeweils integriert werden. Sobald der Code in diesem Branch stabil zu sein scheint und alle Tests durchlaufen (xxx), übernimmst Du die Änderungen in den `develop` Branch. Und wenn sie sich für eine Weile in der Praxis als stabil erwiesen haben, fast-forwardest (xxx) Du den `master` Branch.

<!--#### Large-Merging Workflows ####-->
#### Workflows für umfassende Merges ####

<!--The Git project has four long-running branches: `master`, `next`, and `pu` (proposed updates) for new work, and `maint` for maintenance backports. When new work is introduced by contributors, it’s collected into topic branches in the maintainer’s repository in a manner similar to what I’ve described (see Figure 5-24). At this point, the topics are evaluated to determine whether they’re safe and ready for consumption or whether they need more work. If they’re safe, they’re merged into `next`, and that branch is pushed up so everyone can try the topics integrated together.-->

Das Git Projekt selbst hat view dauerhafte Branches: `master`, `next`, `pu` („proposed updates“, d.h. vorgeschlagene Änderungen) und `maint` („maintenance backports“, d.h. xxx Rückportierungen). Wenn neue Änderungen herein kommen, werden sie in Topic Branches im Projekt Repository gesammelt, ganz ähnlich wie wir gerade besprochen haben (siehe Bild 5-24). Dann wird evaluiert, ob die Änderungen sicher sind und übernommen werden sollen oder ob sie noch weiter bearbeitet werden müssen. Wenn sie übernommen werden sollen, werden sie in den Branch `next` gemerged und dieser Branch wird hochgeladen, sodass jeder ausprobieren kann, wie die neue Codebase funktioniert, nachdem die Änderungen miteinander integriert wurden.

<!--Figure 5-24. Managing a complex series of parallel contributed topic branches.-->

Insert 18333fig0524.png
Bild 5-24. Komplexe, parallel entwickelte Topic Branches verwalten

<!--If the topics still need work, they’re merged into `pu` instead. When it’s determined that they’re totally stable, the topics are re-merged into `master` and are then rebuilt from the topics that were in `next` but didn’t yet graduate to `master`. This means `master` almost always moves forward, `next` is rebased occasionally, and `pu` is rebased even more often (see Figure 5-25).-->

Wenn die Topic Branches noch weiter bearbeitet werden müssen, werden sie statt dessen in den `pu` Branch gemerged. Wenn sie dann stabil sind, werden sie erneut in `master` gemerged und aus denjenigen Änderungen neu aufgebaut, die sich in `next` befanden, es aber noch nicht bis in den `master` Branch geschafft haben (xxx wie jetzt?? xxx). D.h., `master` bewegt sich fast ständig, `next` wird gelegentlich rebased, und `pu` wird noch sehr viel häufiger rebased (siehe Bild 5-25).

<!--Figure 5-25. Merging contributed topic branches into long-term integration branches.-->

Insert 18333fig0525.png
Bild 5-25. Topic Branches in dauerhafte Integrationsbranches mergen

<!--When a topic branch has finally been merged into `master`, it’s removed from the repository. The Git project also has a `maint` branch that is forked off from the last release to provide backported patches in case a maintenance release is required. Thus, when you clone the Git repository, you have four branches that you can check out to evaluate the project in different stages of development, depending on how cutting edge you want to be or how you want to contribute; and the maintainer has a structured workflow to help them vet new contributions.-->

Wenn ein Topic Branch schließlich in `master` gemerged wird, wird er aus dem Repository gelöscht. Das Git Projekt hat außerdem einen `maint` Branch, der jeweils vom letzten Release verzweigt. In diesem Branch werden rückportierte Patches für den Fall gesammelt, dass ein Maintenance Release nötig ist. D.h., wenn Du das Git Projekt Repository klonst, findest Du vier Branches des Projektes in verschiedenen Stadien, die Du jeweils ausprobieren kannst, je nachdem wie hochaktuellen Code Du testen oder wie Du zu dem Projekt beitragen willst. Und der Projekt Betreiber hat auf diese Weise einen klar strukturierten Workflow, der es einfacher macht, neue Beiträge zu prüfen und zu verarbeiten.

<!--#### Rebasing and Cherry Picking Workflows ####-->
#### Rebase und Cherry Picking Workflows ####

<!--Other maintainers prefer to rebase or cherry-pick contributed work on top of their master branch, rather than merging it in, to keep a mostly linear history. When you have work in a topic branch and have determined that you want to integrate it, you move to that branch and run the rebase command to rebuild the changes on top of your current master (or `develop`, and so on) branch. If that works well, you can fast-forward your `master` branch, and you’ll end up with a linear project history.-->

Andere Betreiber bevorzugen, neue Änderungen auf der Basis ihres `master` Branches zu rebasen oder zu cherry-picken statt sie zu mergen, um auf diese Weise eine eher lineare Historie zu erhalten. Wenn Du Änderungen in einem Topic Branch hast, die Du integrieren willst, dann gehst Du in diesen Branch und führst den `rebase` Befehl aus, um diese Änderungen auf der Basis des gegenwärtigen `master` Branches (oder irgendeines anderen, stabileren Branches) neu zu schreiben. Wenn das glatt läuft, kannst Du den `master` Branch fast-forwarden (xxx) und erhältst so eine lineare Projekt Historie.

<!--The other way to move introduced work from one branch to another is to cherry-pick it. A cherry-pick in Git is like a rebase for a single commit. It takes the patch that was introduced in a commit and tries to reapply it on the branch you’re currently on. This is useful if you have a number of commits on a topic branch and you want to integrate only one of them, or if you only have one commit on a topic branch and you’d prefer to cherry-pick it rather than run rebase. For example, suppose you have a project that looks like Figure 5-26.-->

Eine andere Möglichkeit, Commits aus einem Branch in einen anderen zu übernehmen ist der `cherry-pick` Befehl. In Git ist dieser Befehl quasi ein rebase für einen einzelnen Commit. Er nimmt den Patch, der mit dem Commit eingeführt wurde, und versucht, diesen auf den Branch anzuwenden, in dem Du Dich gerade befindest. Das ist nützlich, wenn Du in einem Topic Branch eine Anzahl von Commits hast, aber lediglich einen davon übernehmen willst. Oder wenn Du überhaupt nur einen Commit im Topic Branch hast, diesen aber lieber cherry-picken willst, statt den ganzen Branch zu rebasen. Nehmen wir z.B. an, Du hast ein Projekt, das so aussieht wie in Bild 5-26.

<!--Figure 5-26. Example history before a cherry pick.-->

Insert 18333fig0526.png
Bild 5-26. Beispiel Historie vor einem cherry-pick

<!--If you want to pull commit `e43a6` into your master branch, you can run-->

Wenn Du den Commit `e43a6` in Deinen `master` Branch übernehmen willst, kannst Du folgendes ausführen:

	$ git cherry-pick e43a6fd3e94888d76779ad79fb568ed180e5fcdf
	Finished one cherry-pick.
	[master]: created a0a41a9: "More friendly message when locking the index fails."
	 3 files changed, 17 insertions(+), 3 deletions(-)

<!--This pulls the same change introduced in `e43a6`, but you get a new commit SHA-1 value, because the date applied is different. Now your history looks like Figure 5-27.-->

Das wendet dieselben Änderungen, die in `e43a6` eingeführt wurden, auf den `master` Branch an, aber Du erhältst einen neuen Commit SHA-1 Hash, weil auch das Datum ein anderes ist. Jetzt sieht Deine Historie so aus:

<!--Figure 5-27. History after cherry-picking a commit on a topic branch.-->

Insert 18333fig0527.png
Bild 5-27. Historie nach dem cherry-pick eines Commits aus einem Topic Branch

<!--Now you can remove your topic branch and drop the commits you didn’t want to pull in.-->

Jetzt kannst Du den Topic Branch inklusive der ggf. darin enthaltenen Commits löschen, falls Du sie nicht noch übernehmen willst.

<!--### Tagging Your Releases ###-->
### Releases taggen ###

<!--When you’ve decided to cut a release, you’ll probably want to drop a tag so you can re-create that release at any point going forward. You can create a new tag as I discussed in Chapter 2. If you decide to sign the tag as the maintainer, the tagging may look something like this:-->

Wenn Du einen Release herausgeben willst, ist es empfehlenswert, einen Tag dafür anzulegen, sodass man den jeweiligen Zustand der Historie jederzeit leicht wiederherstellen kann. Wir sind bereits in Kapitel 2 auf Git Tags eingegangen. Wenn Du als Betreiber den neuen Tag signieren willst, könnte das wie folgt aussehen:

	$ git tag -s v1.5 -m 'my signed 1.5 tag'
	You need a passphrase to unlock the secret key for
	user: "Scott Chacon <schacon@gmail.com>"
	1024-bit DSA key, ID F721C45A, created 2009-02-09

<!--If you do sign your tags, you may have the problem of distributing the public PGP key used to sign your tags. The maintainer of the Git project has solved this issue by including their public key as a blob in the repository and then adding a tag that points directly to that content. To do this, you can figure out which key you want by running `gpg -\-list-keys`:-->

Wenn Du Deine Tags signierst, könnte das Problem bestehen, dass Du den jeweiligen öffentlichen PGP key zur Verfügung stellen musst. Der Betreiber des Git Projektes löst das, in dem er den öffentlichen Schlüssel als Inhalt im Repository selbst zur Verfügung stellt und einen Tag hat, der direkt auf diesen Inhalt zeigt. Um das zu tun, musst Du zunächst herausfinden, welchen Schlüssel Du verwenden willst:

	$ gpg --list-keys
	/Users/schacon/.gnupg/pubring.gpg
	---------------------------------
	pub   1024D/F721C45A 2009-02-09 [expires: 2010-02-09]
	uid                  Scott Chacon <schacon@gmail.com>
	sub   2048g/45D02282 2009-02-09 [expires: 2010-02-09]

<!--Then, you can directly import the key into the Git database by exporting it and piping that through `git hash-object`, which writes a new blob with those contents into Git and gives you back the SHA-1 of the blob:-->

Dann kannst Du den Schlüssel direkt in die Git Datenbank importieren, indem Du ihn aus GPG exportierst und die Ausgabe nach `git hash-object` weiterreichst. Das schreibt ein neues Objekt mit dem Schlüssel in die Git Datenbank und gibt Dir einen SHA-1 Hash zurück, der dieses Objekt referenziert:

	$ gpg -a --export F721C45A | git hash-object -w --stdin
	659ef797d181633c87ec71ac3f9ba29fe5775b92

<!--Now that you have the contents of your key in Git, you can create a tag that points directly to it by specifying the new SHA-1 value that the `hash-object` command gave you:-->

Nachdem Du jetzt den Schlüssel im Repository hast, kannst Du einen Tag für den SHA-1 Hash anlegen, den `git hash-object` zurückgegeben hat:

	$ git tag -a maintainer-pgp-pub 659ef797d181633c87ec71ac3f9ba29fe5775b92

<!--If you run `git push -\-tags`, the `maintainer-pgp-pub` tag will be shared with everyone. If anyone wants to verify a tag, they can directly import your PGP key by pulling the blob directly out of the database and importing it into GPG:-->

Wenn Du `git push --tags` ausführst, wird jetzt der `maintainer-pgp-pub` Tag auf den Server geladen, sodass jeder darauf zugreifen kann. Wenn jemand jetzt einen signierten Tag verifizieren will, kann er Deinen öffentlichen PGP Schlüssel direkt aus der Datenbank holen und in seinen Schlüsselbund importieren:

	$ git show maintainer-pgp-pub | gpg --import

<!--They can use that key to verify all your signed tags. Also, if you include instructions in the tag message, running `git show <tag>` will let you give the end user more specific instructions about tag verification.-->

Dieser Schlüssel kann anschließend für alle signierten Tages verwendet werden. Zusätzlich kannst Du Deinen Anwendern in der Tag Meldung erklären, wie sie signierte Tags mit diesem Schlüssel verifizieren können.

<!--### Generating a Build Number ###-->
### Eine Build Nummer generieren ###

<!--Because Git doesn’t have monotonically increasing numbers like 'v123' or the equivalent to go with each commit, if you want to have a human-readable name to go with a commit, you can run `git describe` on that commit. Git gives you the name of the nearest tag with the number of commits on top of that tag and a partial SHA-1 value of the commit you’re describing:-->

Weil Git keine globalen Nummern wie `v123` kennt, die mit jedem Commit monoton hochgezählt werden, kannst Du, um einen leicht lesbaren Bezeichner für einen bestimmten Commit zu erhalten, den Befehl `git describe` auf diesen Befehl ausführen. Git erzeugt dann einen Bezeichner zurück, der den Namen des nächsten Tags enthält, der Anzahl der Commits seit diesem Tag und die ersten Zeichen des SHA-1 Hashs des Commits:

	$ git describe master
	v1.6.2-rc1-20-g8c5b85c

<!--This way, you can export a snapshot or build and name it something understandable to people. In fact, if you build Git from source code cloned from the Git repository, `git -\-version` gives you something that looks like this. If you’re describing a commit that you have directly tagged, it gives you the tag name.-->

Auf diese Weise kannst Du in einer Weise auf einen Commit oder Build verweisen, der für Andere leichter verständlich ist. Wenn Du z.B. Git selbst aus dem Quellcode kompilierst, der sich im Git Projekt Repository befindet, dann gibt `git --version` einen ähnlichen Bezeichner zurück. Wenn Du übrigens `git describe` auf einen Commit ausführst, den Du direkt getagged hast, dann erhältst Du statt dessen den Tag Namen.

<!--The `git describe` command favors annotated tags (tags created with the `-a` or `-s` flag), so release tags should be created this way if you’re using `git describe`, to ensure the commit is named properly when described. You can also use this string as the target of a checkout or show command, although it relies on the abbreviated SHA-1 value at the end, so it may not be valid forever. For instance, the Linux kernel recently jumped from 8 to 10 characters to ensure SHA-1 object uniqueness, so older `git describe` output names were invalidated.-->

Der `git describe` Befehl funktioniert mit kommentierten Tags besser (d.h. Tags, die mit dem `-a` oder `-s` Flag erzeugt wurden), sodass es sich empfiehlt, Release Tags auf diese Weise anzulegen, wenn man `git describe` verwenden will. Du kannst diese Bezeichner auch als Parameter für andere Git Befehle, z.B. `git checkout` oder `git show`, wobei Git allerdings lediglich auf den abgekürzten SHA-1 Hash am Ende achtet, sodass er möglicherweise nicht ewig gültig ist. Das Linux Kernel Projekt beispielsweise erhöhte die Anzahl der Zeichen in abgekürzten Hashes kürzlich von 8 auf 10, um die Eindeutigkeit von SHA-1 Hashes sicherzustellen. Ältere `git describe` Ausgaben wurden damit ungültig.

<!--### Preparing a Release ###-->
### Ein Release vorbereiten ###

<!--Now you want to release a build. One of the things you’ll want to do is create an archive of the latest snapshot of your code for those poor souls who don’t use Git. The command to do this is `git archive`:-->

Du willst jetzt ein Release herausgeben. Dazu willst Du u.a. ein Archiv mit dem letzten Snapshot Deines Codes erzeugen, damit ihn auch arme Seelen herunterladen können, die Git nicht verwenden. Der folgende Befehl hilft Dir dabei:

	$ git archive master --prefix='project/' | gzip > `git describe master`.tar.gz
	$ ls *.tar.gz
	v1.6.2-rc1-20-g8c5b85c.tar.gz

<!--If someone opens that tarball, they get the latest snapshot of your project under a project directory. You can also create a zip archive in much the same way, but by passing the `-\-format=zip` option to `git archive`:-->

Das erzeugt einen Tarball, der den aktuellen Snapshot in Deinem Arbeitsverzeichnis enthält. Du kannst auf die gleiche Weise ein Zip Archiv erzeugen, indem Du `git archive` die `--format=zip` Option übergibst.

	$ git archive master --prefix='project/' --format=zip > `git describe master`.zip

<!--You now have a nice tarball and a zip archive of your project release that you can upload to your website or e-mail to people.-->

Du hast jetzt sowohl einen Tarball als auch ein Zip Archiv Deines Releases. Diese kannst Du z.B. auf Deiner Webseite publizieren oder auch per E-Mail verschicken.

<!--### The Shortlog ###-->
### Das Shortlog ###

<!--It’s time to e-mail your mailing list of people who want to know what’s happening in your project. A nice way of quickly getting a sort of changelog of what has been added to your project since your last release or e-mail is to use the `git shortlog` command. It summarizes all the commits in the range you give it; for example, the following gives you a summary of all the commits since your last release, if your last release was named v1.0.1:-->

Es wird Zeit, den Lesern der Mailingliste zu erklären, was es im Projekt Neues gibt. Der `git shortlog` Befehl ist eine Möglichkeit, schnell eine Art Changelog der Änderungen seit dem letzten Release auszugeben. Er fasst alle Commits in der angegebenen Zeitspanne zusammen. Der folgende Befehl z.B. erzeugt eine Zusammenfassung der Commits seit dem letzten Release, der als `v1.0.1` getagged wurde:

	$ git shortlog --no-merges master --not v1.0.1
	Chris Wanstrath (8):
	      Add support for annotated tags to Grit::Tag
	      Add packed-refs annotated tag support.
	      Add Grit::Commit#to_patch
	      Update version and History.txt
	      Remove stray `puts`
	      Make ls_tree ignore nils

	Tom Preston-Werner (4):
	      fix dates in history
	      dynamic version method
	      Version bump to 1.0.2
	      Regenerated gemspec for version 1.0.2

<!--You get a clean summary of all the commits since v1.0.1, grouped by author, that you can e-mail to your list.-->

Du erhältst eine saubere Auflistung aller Commits seit `v1.0.1`, gruppiert nach Autor. Diese kannst Du z.B. an die Mailingliste schicken oder irgendwie anders publizieren.

<!--## Summary ##-->
## Zusammenfassung ##

<!--You should feel fairly comfortable contributing to a project in Git as well as maintaining your own project or integrating other users’ contributions. Congratulations on being an effective Git developer! In the next chapter, you’ll learn more powerful tools and tips for dealing with complex situations, which will truly make you a Git master.-->

Du solltest Dich jetzt einigermaßen vertraut damit fühlen, sowohl Beiträge bei einem bestehenden Projekt einzureichen als auch selbst ein eigenes Projekt zu betreiben und Beiträge anderer zu integrieren. Herzlichen Glückwunsch, Du bist jetzt ein erfolgreicher Git Entwickler! (xxx hä? xxx) Im nächsten Kapitel wirst Du weitere mächtige Git Werkzeuge und Tipps dafür kennenlernen, mit komplexen Situationen umzugehen – die einen wahren Git Meister aus Dir werden. (xxx aha? xxx)
<!--# Git Tools #-->
# Git Tools #

<!--By now, you’ve learned most of the day-to-day commands and workflows that you need to manage or maintain a Git repository for your source code control. You’ve accomplished the basic tasks of tracking and committing files, and you’ve harnessed the power of the staging area and lightweight topic branching and merging.-->

Bis hierher hast Du die meisten täglichen Kommandos und Arbeitsweisen gelernt, die Du brauchst um ein Git-Repository für Deine Quellcode-Kontrolle, zu benutzen. Du hast die grundlegenden Aufgaben des Nachverfolgens und Eincheckens von Dateien gemeistert und Du hast von der Macht der Staging-Area, des Branching und des Mergens Gebrauch gemacht.

<!--Now you’ll explore a number of very powerful things that Git can do that you may not necessarily use on a day-to-day basis but that you may need at some point.-->

Als Nächstes werden wir einige sehr mächtige Werkzeuge besprechen, die Dir Git zur Verfügung stellt. Du wirst zwar nicht unbedingt jeden Tag verwenden, aber mit Sicherheit an einem bestimmten Punkt gut brauchen können.

<!--## Revision Selection ##-->
## Revision Auswahl ##

<!--Git allows you to specify specific commits or a range of commits in several ways. They aren’t necessarily obvious but are helpful to know.-->

Git erlaubt Dir, Commits auf verschiedenste Art und Weise auszuwählen. Diese sind nicht immer offensichtlich, aber es ist hilfreich diese zu kennen.

<!--### Single Revisions ###-->
### Einzelne Revisionen ###

<!--You can obviously refer to a commit by the SHA-1 hash that it’s given, but there are more human-friendly ways to refer to commits as well. This section outlines the various ways you can refer to a single commit.-->

Du kannst offensichtlich mithilfe des SHA-1-Hashes einen Commit auswählen, aber es gibt auch menschenfreundlichere Methoden, auf einen Commit zu verweisen. Dieser Bereich skizziert die verschiedenen Wege, die man gehen kann, um sich auf ein einzelnen Commit zu beziehen.

<!--### Short SHA ###-->
### Abgekürztes SHA ###

<!--Git is smart enough to figure out what commit you meant to type if you provide the first few characters, as long as your partial SHA-1 is at least four characters long and unambiguous — that is, only one object in the current repository begins with that partial SHA-1.-->

Git ist intelligent genug, den richtigen Commit herauszufinden, wenn man nur die ersten paar Zeichen angibt, aber nur unter der Bedingung, dass der SHA-1-Hash mindestens vier Zeichen lang und einzigartig ist — das bedeutet, dass es nur ein Objekt im derzeitigen Repository gibt, das mit diesem bestimmten SHA-1-Hash beginnt.

<!--For example, to see a specific commit, suppose you run a `git log` command and identify the commit where you added certain functionality:-->

Um zum Beispiel einen bestimmten Commit zu sehen, kann man das `git log` Kommando ausführen und den Commit identifizieren, indem man eine bestimmte Funktionalität hinzugefügt hat:

	$ git log
	commit 734713bc047d87bf7eac9674765ae793478c50d3
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Fri Jan 2 18:32:33 2009 -0800

	    fixed refs handling, added gc auto, updated tests

	commit d921970aadf03b3cf0e71becdaab3147ba71cdef
	Merge: 1c002dd... 35cfb2b...
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Thu Dec 11 15:08:43 2008 -0800

	    Merge commit 'phedders/rdocs'

	commit 1c002dd4b536e7479fe34593e72e6c6c1819e53b
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Thu Dec 11 14:58:32 2008 -0800

	    added some blame and merge stuff

<!--In this case, choose `1c002dd....` If you `git show` that commit, the following commands are equivalent (assuming the shorter versions are unambiguous):-->

In diesem Fall wählt man `1c002dd....`, wenn man diesen Commit mit `git show` anzeigen lassen will, die folgenden Kommandos sind äquivalent (vorausgesetzt die verkürzte Version ist einzigartig):

	$ git show 1c002dd4b536e7479fe34593e72e6c6c1819e53b
	$ git show 1c002dd4b536e7479f
	$ git show 1c002d

<!--Git can figure out a short, unique abbreviation for your SHA-1 values. If you pass `-\-abbrev-commit` to the `git log` command, the output will use shorter values but keep them unique; it defaults to using seven characters but makes them longer if necessary to keep the SHA-1 unambiguous:-->

Git kann auch selber eine Kurzform für Deine einzigartigen SHA-1-Werte erzeugen. Wenn Du `--abbrev-commit` dem `git log` Kommando übergibst, wird es den kürzeren Wert benutzen, diesen aber einzigartig halten; die Standardeinstellung sind sieben Zeichen, aber es werden automatisch mehr benutzt, wenn dies nötig ist, um den SHA-1-Hash eindeutig zu bezeichnen.

	$ git log --abbrev-commit --pretty=oneline
	ca82a6d changed the version number
	085bb3b removed unnecessary test code
	a11bef0 first commit

<!--Generally, eight to ten characters are more than enough to be unique within a project. One of the largest Git projects, the Linux kernel, is beginning to need 12 characters out of the possible 40 to stay unique.-->

Generell kann man sagen, dass acht bis zehn Zeichen mehr als ausreichend in einem Projekt sind, um eindeutig zu bleiben. Eines der größten Git-Projekte, der Linux-Kernel, fängt langsam an 12 von maximal 40 Zeichen zu nutzen, um eindeutig zu bleiben.

<!--### A SHORT NOTE ABOUT SHA-1 ###-->
### Ein kurzer Hinweis zu SHA-1 ###

<!--A lot of people become concerned at some point that they will, by random happenstance, have two objects in their repository that hash to the same SHA-1 value. What then?-->

Eine Menge Leute machen sich Sorgen, dass ab einem zufälligen Punkt zwei Objekte im Repository vorhanden sind, die den gleichen SHA-1-Hashwert haben. Was dann?

<!--If you do happen to commit an object that hashes to the same SHA-1 value as a previous object in your repository, Git will see the previous object already in your Git database and assume it was already written. If you try to check out that object again at some point, you’ll always get the data of the first object.-->

Wenn es passiert, dass bei einem Commit, ein Objekt mit dem gleichen SHA-1-Hashwert im Repository vorhanden ist, wird Git sehen, dass das vorherige Objekt bereits in der Datenbank vorhanden ist und davon ausgehen, dass es bereits geschrieben wurde. Wenn Du versuchst, das Objekt später wieder auszuchecken, wirst Du immer die Daten des ersten Objekts bekommen.

<!--However, you should be aware of how ridiculously unlikely this scenario is. The SHA-1 digest is 20 bytes or 160 bits. The number of randomly hashed objects needed to ensure a 50% probability of a single collision is about 2^80 (the formula for determining collision probability is `p = (n(n-1)/2) * (1/2^160)`). 2^80 is 1.2 x 10^24 or 1 million billion billion. That’s 1,200 times the number of grains of sand on the earth.-->

Allerdings solltest Du Dir bewusst machen, wie unglaublich unwahrscheinlich dieses Szenario ist. Die Länge des SHA-1-Hashs beträgt 20 Bytes oder 160 Bits. Die Anzahl der Objekte, die benötigt werden, um eine 50% Chance einer Kollision zu haben, beträgt ungefähr 2^80 (die Formel zum Berechnen der Kollisionswahrscheinlichkeit lautet `p = (n(n-1)/2) * (1/2^160)`). 2^80 ist somit 1.2 x 10^24 oder eine Trilliarde. Das ist 1200 Mal so viel, wie es Sandkörner auf der Erde gibt.

<!--Here’s an example to give you an idea of what it would take to get a SHA-1 collision. If all 6.5 billion humans on Earth were programming, and every second, each one was producing code that was the equivalent of the entire Linux kernel history (1 million Git objects) and pushing it into one enormous Git repository, it would take 5 years until that repository contained enough objects to have a 50% probability of a single SHA-1 object collision. A higher probability exists that every member of your programming team will be attacked and killed by wolves in unrelated incidents on the same night.-->

Hier ist ein Beispiel, das Dir eine Vorstellung davon gibt, was nötig ist, um in SHA-1 eine Kollision zu bekommen. Wenn alle 6,5 Milliarden Menschen auf der Erde programmieren würden und jeder jede Sekunde Code schreiben würde, der der gesamten Geschichte des Linux-Kernels (1 Million Git-Objekte) entspricht, und diesen dann in ein gigantisches Git-Repository übertragen würde, würde es fünf Jahre dauern, bis das Repository genügend Objekte hätte, um eine 50% Wahrscheinlichkeit für eine einzige SHA-1-Kollision aufzuweisen. Es ist wahrscheinlicher, dass jedes Mitglied Deines Programmierer-Teams, unabhängig voneinander, in einer Nacht von Wölfen angegriffen und getötet wird.

<!--### Branch References ###-->
### Branch-Referenzen ###

<!--The most straightforward way to specify a commit requires that it have a branch reference pointed at it. Then, you can use a branch name in any Git command that expects a commit object or SHA-1 value. For instance, if you want to show the last commit object on a branch, the following commands are equivalent, assuming that the `topic1` branch points to `ca82a6d`:-->

Am direktesten kannst Du einen Commit spezifizieren, wenn eine Branch-Referenz direkt auf ihn zeigt. In dem Fall kannst Du in allen Git-Befehlen, die ein Commit-Objekt oder einen SHA-1-Wert erwarten, stattdessen den Branch-Namen verwenden. Wenn Du z.B. den letzten Commit in einem Branch sehen willst, sind die folgenden Befehle äquivalent (vorausgesetzt der `topic1` Branch zeigt auf den Commit `ca82a6d`):

	$ git show ca82a6dff817ec66f44342007202690a93763949
	$ git show topic1

<!--If you want to see which specific SHA a branch points to, or if you want to see what any of these examples boils down to in terms of SHAs, you can use a Git plumbing tool called `rev-parse`. You can see Chapter 9 for more information about plumbing tools; basically, `rev-parse` exists for lower-level operations and isn’t designed to be used in day-to-day operations. However, it can be helpful sometimes when you need to see what’s really going on. Here you can run `rev-parse` on your branch.-->

Wenn Du sehen willst, auf welchen SHA-1-Wert ein Branch zeigt, oder wie unsere Beispiele intern in SHA-1-Werte übersetzt aussähen, kannst Du den Git-Plumbing-Befehl `rev-parse` verwenden. In Kapitel 9 werden wir genauer auf Plumbing-Befehle eingehen. Kurz gesagt ist `rev-parse` als eine Low-Level-Operation gedacht und nicht dafür, im tagtäglichen Gebrauch eingesetzt zu werden. Aber es kann manchmal hilfreich sein, wenn man wissen muss, was unter der Haube vor sich geht:

	$ git rev-parse topic1
	ca82a6dff817ec66f44342007202690a93763949

<!--### RefLog Shortnames ###-->
### RefLog Kurznamen ###

<!--One of the things Git does in the background while you’re working away is keep a reflog — a log of where your HEAD and branch references have been for the last few months.-->

Eine andere Sache, die während Deiner täglichen Arbeit im Hintergrund passiert ist, dass Git ein sogenanntes Reflog führt, d.h. ein Log darüber, wohin Deine HEAD- und Branch-Referenzen in den letzten Monaten jeweils gezeigt haben.

<!--You can see your reflog by using `git reflog`:-->

Du kannst das Reflog mit `git reflog` anzeigen:

	$ git reflog
	734713b... HEAD@{0}: commit: fixed refs handling, added gc auto, updated
	d921970... HEAD@{1}: merge phedders/rdocs: Merge made by recursive.
	1c002dd... HEAD@{2}: commit: added some blame and merge stuff
	1c36188... HEAD@{3}: rebase -i (squash): updating HEAD
	95df984... HEAD@{4}: commit: # This is a combination of two commits.
	1c36188... HEAD@{5}: rebase -i (squash): updating HEAD
	7e05da5... HEAD@{6}: rebase -i (pick): updating HEAD

<!--Every time your branch tip is updated for any reason, Git stores that information for you in this temporary history. And you can specify older commits with this data, as well. If you want to see the fifth prior value of the HEAD of your repository, you can use the `@{n}` reference that you see in the reflog output:-->

Immer dann, wenn ein Branch in irgendeiner Weise aktualisiert wird oder Du den aktuellen Branch wechselst, speichert Git diese Information ebenso im Reflog wie Commits und anderen Informationen. Wenn Du wissen willst, welches der fünfte Wert vor dem HEAD war, kannst Du die `@{n}` Referenz angeben, die Du in Reflog-Ausgabe sehen kannst:

	$ git show HEAD@{5}

<!--You can also use this syntax to see where a branch was some specific amount of time ago. For instance, to see where your `master` branch was yesterday, you can type-->

Außerdem kannst Du dieselbe Syntax verwenden, um eine Zeitspanne anzugeben. Um z.B. zu sehen, wo Dein `master` Branch gestern war, kannst Du eingeben:

	$ git show master@{yesterday}

<!--That shows you where the branch tip was yesterday. This technique only works for data that’s still in your reflog, so you can’t use it to look for commits older than a few months.-->

Das zeigt Dir, wo der `master` Branch gestern war. Diese Technik funktioniert nur mit Daten, die noch im Reflog sind, d.h. man kann sie nicht für Commits verwenden, die ein älter sind als ein paar Monate.

<!--To see reflog information formatted like the `git log` output, you can run `git log -g`:-->

Um Reflog Informationen in einem Format wie in `git log` anzeigen, kannst Du `git log -g` verwenden:

	$ git log -g master
	commit 734713bc047d87bf7eac9674765ae793478c50d3
	Reflog: master@{0} (Scott Chacon <schacon@gmail.com>)
	Reflog message: commit: fixed refs handling, added gc auto, updated
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Fri Jan 2 18:32:33 2009 -0800

	    fixed refs handling, added gc auto, updated tests

	commit d921970aadf03b3cf0e71becdaab3147ba71cdef
	Reflog: master@{1} (Scott Chacon <schacon@gmail.com>)
	Reflog message: merge phedders/rdocs: Merge made by recursive.
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Thu Dec 11 15:08:43 2008 -0800

	    Merge commit 'phedders/rdocs'

<!--It’s important to note that the reflog information is strictly local — it’s a log of what you’ve done in your repository. The references won’t be the same on someone else’s copy of the repository; and right after you initially clone a repository, you’ll have an empty reflog, as no activity has occurred yet in your repository. Running `git show HEAD@{2.months.ago}` will work only if you cloned the project at least two months ago — if you cloned it five minutes ago, you’ll get no results.-->

Es ist wichtig zu verstehen, dass das Reflog ausschließlich lokale Daten enthält. Es ist ein Log darüber, was Du in Deinem Repository getan hast, und es ist nie dasselbe wie in einem anderen Klon des selben Repositorys. Direkt nachdem Du ein Repository geklont hast, ist das Reflog leer, weil noch keine weitere Aktivität stattgefunden hat. `git show HEAD@{2.months.ago}` funktioniert nur dann, wenn das Projekt mindestens zwei Monate alt ist – wenn Du es vor fünf Minuten erst geklont hast, erhältst Du keine Ergebnisse.

<!--### Ancestry References ###-->
### Vorfahren Referenzen ###

<!--The other main way to specify a commit is via its ancestry. If you place a `^` at the end of a reference, Git resolves it to mean the parent of that commit.
Suppose you look at the history of your project:-->

Außerdem kann man Commits über ihre Vorfahren spezifizieren. Wenn Du ein `^` ans Ende einer Referenz setzt, schlägt Git den direkten Vorfahren dieses Commits nach. Nehmen wir an, Deine Historie sieht so aus:

	$ git log --pretty=format:'%h %s' --graph
	* 734713b fixed refs handling, added gc auto, updated tests
	*   d921970 Merge commit 'phedders/rdocs'
	|\
	| * 35cfb2b Some rdoc changes
	* | 1c002dd added some blame and merge stuff
	|/
	* 1c36188 ignore *.gem
	* 9b29157 add open3_detach to gemspec file list

<!--Then, you can see the previous commit by specifying `HEAD^`, which means "the parent of HEAD":-->

Du kannst jetzt den vorletzten Commit mit `HEAD^` referenzieren, d.h. „den direkten Vorfahren von HEAD“.

	$ git show HEAD^
	commit d921970aadf03b3cf0e71becdaab3147ba71cdef
	Merge: 1c002dd... 35cfb2b...
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Thu Dec 11 15:08:43 2008 -0800

	    Merge commit 'phedders/rdocs'

<!--You can also specify a number after the `^` — for example, `d921970^2` means "the second parent of d921970." This syntax is only useful for merge commits, which have more than one parent. The first parent is the branch you were on when you merged, and the second is the commit on the branch that you merged in:-->

Außerdem kannst Du nach dem `^` eine Zahl angeben. Beispielsweise heißt `d921970^2`: „der zweite Vorfahr von d921970“. Diese Syntax ist allerdings nur für Merge Commits nützlich, die mehr als einen Vorfahren haben. Der erste Vorfahr ist der Branch, auf dem Du Dich beim Merge befandest, der zweite ist der Commit auf dem Branch, den Du gemergt hast.

	$ git show d921970^
	commit 1c002dd4b536e7479fe34593e72e6c6c1819e53b
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Thu Dec 11 14:58:32 2008 -0800

	    added some blame and merge stuff

	$ git show d921970^2
	commit 35cfb2b795a55793d7cc56a6cc2060b4bb732548
	Author: Paul Hedderly <paul+git@mjr.org>
	Date:   Wed Dec 10 22:22:03 2008 +0000

	    Some rdoc changes

<!--The other main ancestry specification is the `~`. This also refers to the first parent, so `HEAD~` and `HEAD^` are equivalent. The difference becomes apparent when you specify a number. `HEAD~2` means "the first parent of the first parent," or "the grandparent" — it traverses the first parents the number of times you specify. For example, in the history listed earlier, `HEAD~3` would be-->

Eine andere Vorfahren-Spezifikation ist `~`. Dies bezieht sich ebenfalls auf den ersten Vorfahren, d.h. `HEAD~` und `HEAD^` sind äquivalent. Einen Unterschied macht es allerdings, wenn Du außerdem eine Zahl angibst. `HEAD~2` bedeutet z.B. „der Vorfahr des Vorfahren von HEAD“ oder „der n-te Vorfahr von HEAD“. Beispielsweise würde `HEAD~3` in der obigen Historie auf den folgenden Commit zeigen:

	$ git show HEAD~3
	commit 1c3618887afb5fbcbea25b7c013f4e2114448b8d
	Author: Tom Preston-Werner <tom@mojombo.com>
	Date:   Fri Nov 7 13:47:59 2008 -0500

	    ignore *.gem

<!--This can also be written `HEAD^^^`, which again is the first parent of the first parent of the first parent:-->

Dasselbe kannst Du mit `HEAD^^^` angeben, was wiederum den „Vorfahren des Vorfahren des Vorfahren“ referenziert:

	$ git show HEAD^^^
	commit 1c3618887afb5fbcbea25b7c013f4e2114448b8d
	Author: Tom Preston-Werner <tom@mojombo.com>
	Date:   Fri Nov 7 13:47:59 2008 -0500

	    ignore *.gem

<!--You can also combine these syntaxes — you can get the second parent of the previous reference (assuming it was a merge commit) by using `HEAD~3^2`, and so on.-->

Du kannst diese Schreibweisen auch kombinieren und z.B. auf den zweiten Vorfahren der obigen Referenz mit `HEAD~3^2` zugreifen.

<!--### Commit Ranges ###-->
### Commit Reihen ###

<!--Now that you can specify individual commits, let’s see how to specify ranges of commits. This is particularly useful for managing your branches — if you have a lot of branches, you can use range specifications to answer questions such as, "What work is on this branch that I haven’t yet merged into my main branch?"-->

Nachdem Du jetzt einzelne Commits spezifizieren kannst, schauen wir uns an, wie man auf Commit-Reihen zugreift. Dies ist vor allem nützlich, um Branches zu verwalten, z.B. wenn man viele Branches hat und solche Fragen beantworten will wie „Welche Änderungen befinden sich in diesem Branch, die ich noch nicht in meinen Hauptbranch gemergt habe?“.

<!--#### Double Dot ####-->
#### Zwei-Punkte-Syntax ####

<!--The most common range specification is the double-dot syntax. This basically asks Git to resolve a range of commits that are reachable from one commit but aren’t reachable from another. For example, say you have a commit history that looks like Figure 6-1.-->

Die gängigste Weise, Commit-Reihen anzugeben, ist die Zwei-Punkte-Notation. Allgemein gesagt liefert Git damit eine Reihe von Commits, die von dem einem Commit aus erreichbar sind, allerdings nicht von dem anderen aus. Nehmen wir z.B. an, Du hättest eine Commit-Historie wie die folgende (Bild 6-1).

<!--Figure 6-1. Example history for range selection.-->

Insert 18333fig0601.png

<!--You want to see what is in your experiment branch that hasn’t yet been merged into your master branch. You can ask Git to show you a log of just those commits with `master..experiment` — that means "all commits reachable by experiment that aren’t reachable by master." For the sake of brevity and clarity in these examples, I’ll use the letters of the commit objects from the diagram in place of the actual log output in the order that they would display:-->

Du willst jetzt herausfinden, welche Änderungen in Deinem `experiment`-Branch sind, die noch nicht in den `master`-Branch gemergt wurden. Dann kannst Du ein Log dieser Commits mit `master..experiment` anzeigen, d.h. „alle Commits, die von `experiment` aus erreichbar sind, aber nicht von `master`“. Um die folgenden Beispiele ein bisschen abzukürzen und deutlicher zu machen, verwende ich für die Commit-Objekte die Buchstaben aus dem Diagramm anstelle der tatsächlichen Log Ausgabe:

	$ git log master..experiment
	D
	C

<!--If, on the other hand, you want to see the opposite — all commits in `master` that aren’t in `experiment` — you can reverse the branch names. `experiment..master` shows you everything in `master` not reachable from `experiment`:-->

Wenn Du allerdings – anders herum – diejenigen Commits anzeigen willst, die in `master`, aber noch nicht in `experiment` sind, kannst Du die Branch-Namen umdrehen: `experiment..master` zeigt „alles in `master`, das nicht in `experiment` enthalten ist“.

	$ git log experiment..master
	F
	E

<!--This is useful if you want to keep the `experiment` branch up to date and preview what you’re about to merge in. Another very frequent use of this syntax is to see what you’re about to push to a remote:-->

Dies ist nützlich, wenn Du vorhast, den `experiment`-Branch zu aktualisieren, und anzeigen willst, was Du dazu mergen wirst. Oder wenn Du vorhast, in ein Remote-Repository zu pushen und sehen willst, welche Commits betroffen sind:

	$ git log origin/master..HEAD

<!--This command shows you any commits in your current branch that aren’t in the `master` branch on your `origin` remote. If you run a `git push` and your current branch is tracking `origin/master`, the commits listed by `git log origin/master..HEAD` are the commits that will be transferred to the server.-->
<!--You can also leave off one side of the syntax to have Git assume HEAD. For example, you can get the same results as in the previous example by typing `git log origin/master..` — Git substitutes HEAD if one side is missing.-->

Dieser Befehl zeigt Dir alle Commits im gegenwärtigen, lokalen Branch, die noch nicht im `master`-Branch des `origin` Repositorys sind. D.h., der Befehl listet diejenigen Commits auf, die auf den Server transferiert würden, wenn Du `git push` benutzt und der aktuelle Branch `origin/master` trackt. Du kannst mit dieser Syntax außerdem eine Seite der beiden Punkte leer lassen. Git nimmt dann an, Du meinst an dieser Stelle HEAD. Z.B. kannst Du dieselben Commits wie im vorherigen Beispiel auch mit `git log origin/master..` anzeigen lassen. Git fügt dann HEAD auf der rechten Seite ein.

<!--#### Multiple Points ####-->
#### Mehrere Bezugspunkte ####

<!--The double-dot syntax is useful as a shorthand; but perhaps you want to specify more than two branches to indicate your revision, such as seeing what commits are in any of several branches that aren’t in the branch you’re currently on. Git allows you to do this by using either the `^` character or `-\-not` before any reference from which you don’t want to see reachable commits. Thus these three commands are equivalent:-->

Die Zwei-Punkte-Syntax ist eine nützliche Abkürzung, aber möglicherweise willst Du mehr als nur zwei Branches angeben, um z.B. herauszufinden, welche Commits in einem beliebigen anderen Branch enthalten sind, ausgenommen in demjenigen, auf dem Du Dich gerade befindest. Dazu kannst Du in Git das `^` Zeichen oder `--not` verwenden, um Commits auszuschließen, die von den angegebenen Referenzen aus erreichbar sind. D.h., die folgenden drei Befehle sind äquivalent:

	$ git log refA..refB
	$ git log ^refA refB
	$ git log refB --not refA

<!--This is nice because with this syntax you can specify more than two references in your query, which you cannot do with the double-dot syntax. For instance, if you want to see all commits that are reachable from `refA` or `refB` but not from `refC`, you can type one of these:-->

Das ist praktisch, weil Du auf diese Weise mehr als nur zwei Referenzen angeben kannst, was mit der Zwei-Punkte-Notation nicht geht. Wenn Du beispielsweise alle Commits sehen willst, die von `refA` oder `refB` erreichbar sind, nicht aber von `refC`, dann kannst Du folgende (äquivalente) Befehle benutzen:

	$ git log refA refB ^refC
	$ git log refA refB --not refC

<!--This makes for a very powerful revision query system that should help you figure out what is in your branches.-->

Damit hast Du ein sehr mächtiges System von Abfragen zur Verfügung, mit denen Du herausfinden kannst, was in welchen Deiner Branches enthalten ist.

<!--#### Triple Dot ####-->
#### Drei-Punkte-Syntax ####

<!--The last major range-selection syntax is the triple-dot syntax, which specifies all the commits that are reachable by either of two references but not by both of them. Look back at the example commit history in Figure 6-1.-->
<!--If you want to see what is in `master` or `experiment` but not any common references, you can run-->

Die letzte wichtige Syntax, mit der man Commit-Reihen spezifizieren kann, ist die Drei-Punkte-Syntax, die alle Commits anzeigt, die in einer der beiden Referenzen enthalten sind, aber nicht in beiden. Schau Dir noch mal die Commit Historie in Bild 6-1 an. Wenn Du diejenigen Commits anzeigen willst, die in den `master`- und `experiment`-Branches, nicht aber in beiden Branches gleichzeitig enthalten sind, dann kannst Du folgendes tun:

	$ git log master...experiment
	F
	E
	D
	C

<!--Again, this gives you normal `log` output but shows you only the commit information for those four commits, appearing in the traditional commit date ordering.-->

Dies gibt Dir wiederum ein normale `log` Ausgabe, aber zeigt nur die Informationen dieser vier Commits – wie üblich sortiert nach dem Commit-Datum.

<!--A common switch to use with the `log` command in this case is `-\-left-right`, which shows you which side of the range each commit is in. This helps make the data more useful:-->

Eine nützliche Option für den `log` Befehl ist in diesem Fall `--left-right`. Sie zeigt Dir an, in welchem der beiden Branches der jeweilige Commit enthalten ist, sodass die Ausgabe noch nützlicher ist:

	$ git log --left-right master...experiment
	< F
	< E
	> D
	> C

<!--With these tools, you can much more easily let Git know what commit or commits you want to inspect.-->

Mit diesen Hilfsmitteln kannst Du noch einfacher und genauer angeben, welche Commits Du nachschlagen willst.

<!--## Interactive Staging ##-->
## Interaktives Stagen ##

<!--Git comes with a couple of scripts that make some command-line tasks easier. Here, you’ll look at a few interactive commands that can help you easily craft your commits to include only certain combinations and parts of files. These tools are very helpful if you modify a bunch of files and then decide that you want those changes to be in several focused commits rather than one big messy commit. This way, you can make sure your commits are logically separate changesets and can be easily reviewed by the developers working with you.-->
<!--If you run `git add` with the `-i` or `-\-interactive` option, Git goes into an interactive shell mode, displaying something like this:-->

Git umfasst eine Reihe von Skripten, die so manche Aufgabe auf der Kommandozeile leichter machen. Im Folgenden schauen wir uns einige interaktive Befehle an, die dabei hilfreich sein können, wenn man Änderungen in vielen Dateien vorgenommen hat, aber nur einige Änderungen gezielt committen will – nicht alles auf einmal in einem riesigen Commit. Auf diese Weise kann man Commits logisch gruppieren und macht es anderen Entwicklern damit leichter, sie zu verstehen. Wenn Du `git add` mit der `-i` oder `--interactive` Option verwendest, geht Git in einen interaktiven Shell-Modus, der in etwa wie folgt aussieht:

	$ git add -i
	           staged     unstaged path
	  1:    unchanged        +0/-1 TODO
	  2:    unchanged        +1/-1 index.html
	  3:    unchanged        +5/-1 lib/simplegit.rb

	*** Commands ***
	  1: status     2: update      3: revert     4: add untracked
	  5: patch      6: diff        7: quit       8: help
	What now>

<!--You can see that this command shows you a much different view of your staging area — basically the same information you get with `git status` but a bit more succinct and informative. It lists the changes you’ve staged on the left and unstaged changes on the right.-->

Wie Du siehst, zeigt dieser Befehl eine andere Ansicht der Staging-Area an – im Wesentlichen also die Information, die Du auch mit `git status` erhältst, aber anders formatiert, kurz und knapp, und informativer. Sie listet alle Änderungen, die in der Staging-Area enthalten sind, auf der linken Seite, und alle anderen Änderungen auf der rechten Seite.

<!--After this comes a Commands section. Here you can do a number of things, including staging files, unstaging files, staging parts of files, adding untracked files, and seeing diffs of what has been staged.-->

Danach folgt eine Liste von Befehlen wie, u.a., Dateien ganz oder teilweise stagen und unstagen, nicht versionskontrollierte Dateien hinzufügen, Diffs der gestageten Änderungen anzeigen etc.

<!--### Staging and Unstaging Files ###-->
### Hinzufügen und Enfernen von Dateien aus der Staging-Area ###

<!--If you type `2` or `u` at the `What now>` prompt, the script prompts you for which files you want to stage:-->

Wenn Du am `What now>` Prompt `2` oder `u` eingibst, wirst Du als Nächstes gefragt, welche Dateien Du stagen willst:

	What now> 2
	           staged     unstaged path
	  1:    unchanged        +0/-1 TODO
	  2:    unchanged        +1/-1 index.html
	  3:    unchanged        +5/-1 lib/simplegit.rb
	Update>>

<!--To stage the TODO and index.html files, you can type the numbers:-->

Um z.B. die TODO und index.html Dateien zu stagen, gibst Du die jeweiligen Zahlen ein:

	Update>> 1,2
	           staged     unstaged path
	* 1:    unchanged        +0/-1 TODO
	* 2:    unchanged        +1/-1 index.html
	  3:    unchanged        +5/-1 lib/simplegit.rb
	Update>>

<!--The `*` next to each file means the file is selected to be staged. If you press Enter after typing nothing at the `Update>>` prompt, Git takes anything selected and stages it for you:-->

Das `*` neben den Dateinamen bedeutet, dass die Datei ausgewählt ist und zur Staging-Area hinzugefügt werden wird, sobald Du (bei einem sonst leeren `Update>>` Prompt) Enter drückst:

	Update>>
	updated 2 paths

	*** Commands ***
	  1: status     2: update      3: revert     4: add untracked
	  5: patch      6: diff        7: quit       8: help
	What now> 1
	           staged     unstaged path
	  1:        +0/-1      nothing TODO
	  2:        +1/-1      nothing index.html
	  3:    unchanged        +5/-1 lib/simplegit.rb

<!--Now you can see that the TODO and index.html files are staged and the simplegit.rb file is still unstaged. If you want to unstage the TODO file at this point, you use the `3` or `r` (for revert) option:-->

Du kannst sehen, dass die TODO und index.html Dateien jetzt gestaget sind, während simplegit.rb immer noch ungestaget ist. Wenn Du die TODO unstagen willst, kannst Du die Option `3` oder `r` (für revert) nutzen:

	*** Commands ***
	  1: status     2: update      3: revert     4: add untracked
	  5: patch      6: diff        7: quit       8: help
	What now> 3
	           staged     unstaged path
	  1:        +0/-1      nothing TODO
	  2:        +1/-1      nothing index.html
	  3:    unchanged        +5/-1 lib/simplegit.rb
	Revert>> 1
	           staged     unstaged path
	* 1:        +0/-1      nothing TODO
	  2:        +1/-1      nothing index.html
	  3:    unchanged        +5/-1 lib/simplegit.rb
	Revert>> [enter]
	reverted one path

<!--Looking at your Git status again, you can see that you’ve unstaged the TODO file:-->

Wenn Du wiederum Deinen Git-Status ansiehst, kannst Du sehen, dass Du die TODO ungestaget hast.

	*** Commands ***
	  1: status     2: update      3: revert     4: add untracked
	  5: patch      6: diff        7: quit       8: help
	What now> 1
	           staged     unstaged path
	  1:    unchanged        +0/-1 TODO
	  2:        +1/-1      nothing index.html
	  3:    unchanged        +5/-1 lib/simplegit.rb

<!--To see the diff of what you’ve staged, you can use the `6` or `d` (for diff) command. It shows you a list of your staged files, and you can select the ones for which you would like to see the staged diff. This is much like specifying `git diff -\-cached` on the command line:-->

Um einen Diff dessen zu sehen, das Du gestaget hast, kannst Du den Befehl `6` oder `d` (für diff) nutzen. Dieser zeigt Dir eine Liste der gestageten Dateien, und Du kannst diejenigen auswählen, von denen Du den gestageten Diff sehen willst. Dies ähnelt sehr dem Befehl `git diff --cached` auf der Kommandozeile.

	*** Commands ***
	  1: status     2: update      3: revert     4: add untracked
	  5: patch      6: diff        7: quit       8: help
	What now> 6
	           staged     unstaged path
	  1:        +1/-1      nothing index.html
	Review diff>> 1
	diff --git a/index.html b/index.html
	index 4d07108..4335f49 100644
	--- a/index.html
	+++ b/index.html
	@@ -16,7 +16,7 @@ Date Finder

	 <p id="out">...</p>

	-<div id="footer">contact : support@github.com</div>
	+<div id="footer">contact : email.support@github.com</div>

	 <script type="text/javascript">

<!--With these basic commands, you can use the interactive add mode to deal with your staging area a little more easily.-->

Mit diesen grundlegenden Befehlen kannst Du den interaktiven Hinzufüge-Modus nutzen, um Dir den Umgang mit Deiner Staging-Area etwas zu erleichtern.

<!--### Staging Patches ###-->
### Patches stagen ###

<!--It’s also possible for Git to stage certain parts of files and not the rest. For example, if you make two changes to your simplegit.rb file and want to stage one of them and not the other, doing so is very easy in Git. From the interactive prompt, type `5` or `p` (for patch). Git will ask you which files you would like to partially stage; then, for each section of the selected files, it will display hunks of the file diff and ask if you would like to stage them, one by one:-->

Es ist für Git auch möglich, bestimmte Teile einer Datei zu stagen und nicht den Rest. Wenn Du z.B. zwei Veränderungen an der simplegit.rb machst und eine davon stagen willst und die andere nicht, ist dies sehr einfach in Git möglich. Wähle `5` oder `p` (für patch) auf dem interaktiven Prompt. Git wird Dich fragen, welche Dateien Du teilweise stagen willst; dann wird es für jeden Abschnitt der gewählten Dateien Diff-Ausschnitte ausgeben und Dich jeweils einzeln fragen, ob Du sie stagen willst.

	diff --git a/lib/simplegit.rb b/lib/simplegit.rb
	index dd5ecc4..57399e0 100644
	--- a/lib/simplegit.rb
	+++ b/lib/simplegit.rb
	@@ -22,7 +22,7 @@ class SimpleGit
	   end

	   def log(treeish = 'master')
	-    command("git log -n 25 #{treeish}")
	+    command("git log -n 30 #{treeish}")
	   end

	   def blame(path)
	Stage this hunk [y,n,a,d,/,j,J,g,e,?]?

<!--You have a lot of options at this point. Typing `?` shows a list of what you can do:-->

Du hast an diesem Punkt viele Optionen. Tippe `?` ein, um eine Liste der Möglichkeiten zu bekommen:

	Stage this hunk [y,n,a,d,/,j,J,g,e,?]? ?
	y - stage this hunk
	n - do not stage this hunk
	a - stage this and all the remaining hunks in the file
	d - do not stage this hunk nor any of the remaining hunks in the file
	g - select a hunk to go to
	/ - search for a hunk matching the given regex
	j - leave this hunk undecided, see next undecided hunk
	J - leave this hunk undecided, see next hunk
	k - leave this hunk undecided, see previous undecided hunk
	K - leave this hunk undecided, see previous hunk
	s - split the current hunk into smaller hunks
	e - manually edit the current hunk
	? - print help

<!--Generally, you’ll type `y` or `n` if you want to stage each hunk, but staging all of them in certain files or skipping a hunk decision until later can be helpful too. If you stage one part of the file and leave another part unstaged, your status output will look like this:-->

Im Allgemeinen, wirst Du `y` oder `n` nutzen, wenn Du jeden Ausschnitt stagen willst, aber alle Ausschnitte in bestimmten Dateien zu stagen oder die Entscheidung für einen Ausschnitt auf später zu verschieben kann auch sehr hilfreich sein. Wenn Du nur einen Teil der Datei stagest und den anderen ungestaget lässt, sieht Deine Status-Ausgabe in etwa so aus:

	What now> 1
	           staged     unstaged path
	  1:    unchanged        +0/-1 TODO
	  2:        +1/-1      nothing index.html
	  3:        +1/-1        +4/-0 lib/simplegit.rb

<!--The status of the simplegit.rb file is interesting. It shows you that a couple of lines are staged and a couple are unstaged. You’ve partially staged this file. At this point, you can exit the interactive adding script and run `git commit` to commit the partially staged files.-->

Der Status der simplegit.rb ist interessant. Er zeigt Dir, dass ein paar Zeilen gestaget und ein paar ungestaget sind. Du hast diese Datei teilweise gestaget. An dieser Stelle kannst Du das interaktive Hinzufüge-Skript verlassen und `git commit` ausführen, um die teilweise gestageten Dateien zu commiten.

<!--Finally, you don’t need to be in interactive add mode to do the partial-file staging — you can start the same script by using `git add -p` or `git add -\-patch` on the command line.-->

Letztendlich musst Du nicht den interaktiven Hinzufüge-Modus nutzen, um Dateien teilweise zu stagen – Du kannst das gleiche Skript starten, indem Du `git add -p` oder `git add --patch` auf der Kommandozeile eingibst.

<!--## Stashing ##-->
## Stashen ##

<!--Often, when you’ve been working on part of your project, things are in a messy state and you want to switch branches for a bit to work on something else. The problem is, you don’t want to do a commit of half-done work just so you can get back to this point later. The answer to this issue is the `git stash` command.-->

Während man an einer bestimmten Funktion eines Projekts arbeitet, ist es oft so, dass man den Branch wechseln will, weil man an etwas anderem weiterarbeiten will. Meist ist dann auch das Arbeitsverzeichnis in einem chaotischen Zustand, da die Funktion noch nicht fertiggestellt ist. Das Problem dabei ist, dass Du Deine halbfertige Arbeit dann auch nicht committen möchtest, um später daran weiter arbeiten zu können. Die Lösung dieses Problems bietet der `git stash` Befehl.

<!--Stashing takes the dirty state of your working directory — that is, your modified tracked files and staged changes — and saves it on a stack of unfinished changes that you can reapply at any time.-->

Beim Stashen werden die aus Deinem Arbeitsverzeichnis noch nicht committeten Änderungen – also Deine geänderten beobachteten Dateien und die in der Staging-Area enthaltenen Dateien – in einem Stack voller unfertiger Änderungen gespeichert. Diese kannst Du dann jederzeit wieder vom Stack holen und auf Dein Arbeitsverzeichnis anwenden.

<!--### Stashing Your Work ###-->
### Stash verwenden ###

<!--To demonstrate, you’ll go into your project and start working on a couple of files and possibly stage one of the changes. If you run `git status`, you can see your dirty state:-->

Um dies zu demonstrieren, gehst Du in Dein Projekt und beginnst an ein paar Dateien zu arbeiten und merkst ein paar dieser Änderungen in der Staging-Area vor. Wenn Du den Befehl `git status` ausführst, siehst Du, dass sich einige Dateien seit dem letzen Commit verändert haben.

	$ git status
	# On branch master
	# Changes to be committed:
	#   (use "git reset HEAD <file>..." to unstage)
	#
	#      modified:   index.html
	#
	# Changes not staged for commit:
	#   (use "git add <file>..." to update what will be committed)
	#
	#      modified:   lib/simplegit.rb
	#

<!--Now you want to switch branches, but you don’t want to commit what you’ve been working on yet; so you’ll stash the changes. To push a new stash onto your stack, run `git stash`:-->

Jetzt kommt der Zeitpunkt, an dem Du den aktuellen Branch wechseln möchtest. Allerdings willst Du den aktuellen Zustand auch nicht committen, da Deine Arbeit noch nicht ganz fertiggestellt ist. Darum legst Du Deine Änderungen jetzt in einem Stash ab. Um diesen neuen Stash auf dem Stack abzulegen, verwendest Du den Befehl `git stash`:

	$ git stash
	Saved working directory and index state \
	  "WIP on master: 049d078 added the index file"
	HEAD is now at 049d078 added the index file
	(To restore them type "git stash apply")

<!--Your working directory is clean:-->

In Deinem Arbeitsverzeichnis befinden sich jetzt keine geänderten Dateien mehr und die Staging-Area ist auch leer:

	$ git status
	# On branch master
	nothing to commit, working directory clean

<!--At this point, you can easily switch branches and do work elsewhere; your changes are stored on your stack. To see which stashes you’ve stored, you can use `git stash list`:-->

In diesem Zustand, kannst Du in beliebig andere Branches wechseln und an etwas anderem weiterarbeiten. Deine Änderungen sind alle in einem Stack gesichert. Um eine Übersicht, der bereits gestashten Änderungen anzusehen, kannst Du den Befehl `git stash list` verwenden:

	$ git stash list
	stash@{0}: WIP on master: 049d078 added the index file
	stash@{1}: WIP on master: c264051... Revert "added file_size"
	stash@{2}: WIP on master: 21d80a5... added number to log

<!--In this case, two stashes were done previously, so you have access to three different stashed works. You can reapply the one you just stashed by using the command shown in the help output of the original stash command: `git stash apply`. If you want to apply one of the older stashes, you can specify it by naming it, like this: `git stash apply stash@{2}`. If you don’t specify a stash, Git assumes the most recent stash and tries to apply it:-->

In diesem Beispiel waren bereits zwei Stashes auf dem Stack vorhanden. Sie wurden zu einem früheren Zeitpunkt gespeichert. Dir stehen jetzt also drei verschiedene Stashes auf dem Stack zur Verfügung. Mit dem Befehl `git stash apply` kannst Du die zuletzt gestashten Änderungen in Deinem Arbeitsverzeichnis wiederherstellen. Git zeigt diesen Befehlsaufruf auch bei Ausführen des Befehls `git stash` als Hilfestellung an. Wenn Du einen der älteren Stashes auf Dein Arbeitsverzeichnis anwenden willst, kannst Du den entsprechenden Stashnamen an den Befehl anhängen: `git stash apply stash@{2}`. Wie Du bereits gesehen hast, verwendet Git die zuletzt gestashten Änderungen und versucht diese im Arbeitsverzeichnis wiederherzustellen, wenn der Stashname nicht angegeben wird:

	$ git stash apply
	# On branch master
	# Changes not staged for commit:
	#   (use "git add <file>..." to update what will be committed)
	#
	#      modified:   index.html
	#      modified:   lib/simplegit.rb
	#

<!--You can see that Git re-modifies the files you uncommitted when you saved the stash. In this case, you had a clean working directory when you tried to apply the stash, and you tried to apply it on the same branch you saved it from; but having a clean working directory and applying it on the same branch aren’t necessary to successfully apply a stash. You can save a stash on one branch, switch to another branch later, and try to reapply the changes. You can also have modified and uncommitted files in your working directory when you apply a stash — Git gives you merge conflicts if anything no longer applies cleanly.-->

Wie Du sehen kannst, stellt Git die Dateien wieder her, die Du in einem Stash gespeichert hast. In dem Beispiel war Dein Arbeitsverzeichnis in einem sauberen Zustand, als Du versucht hast, den Stash zurückzuladen. Ebenso wurde der Stash auf dem gleichen Branch angewandt, der auch beim Stashen der Änderungen ausgecheckt war. Aber es ist nicht zwingend notwendig, dass der gleiche Branch verwendet wird und dass das Arbeitsverzeichnis in einem sauberen Zustand ist, wenn ein Stash zurückgeladen wird. Du kannst die Änderungen in einem Stash ablegen, zu einem anderen Branch wechseln und die Änderungen in diesem neuen Branch wiederherstellen. Es können auch geänderte oder gestagte Dateien im Arbeitsverzeichnis vorhanden sein, während ein Stash zurückgeladen wird. Können die Änderungen nicht ordnungsgemäß zurückgeladen werden, zeigt Git einen entsprechenden Merge-Konflikt an.

<!--The changes to your files were reapplied, but the file you staged before wasn’t restaged. To do that, you must run the `git stash apply` command with a `-\-index` option to tell the command to try to reapply the staged changes. If you had run that instead, you’d have gotten back to your original position:-->

Die Änderungen an den Dateien wurden in Deinem Arbeitsverzeichnis wiederhergestellt. Allerdings ist die Datei, die beim Stashen in der Staging-Area vorhanden war, nicht automatisch wieder in die Staging-Area gewandert. Wenn Du die Option `--index` an den Befehl `git stash apply` anhängst, wird Git versuchen, die Dateien wieder zu stagen. Wenn Du diesen Befehl angewandt hättest, wäre Dein Arbeitsverzeichnis und Deine Staging-Area im exakt gleichen Zustand, wie vor dem Stashen:

	$ git stash apply --index
	# On branch master
	# Changes to be committed:
	#   (use "git reset HEAD <file>..." to unstage)
	#
	#      modified:   index.html
	#
	# Changes not staged for commit:
	#   (use "git add <file>..." to update what will be committed)
	#
	#      modified:   lib/simplegit.rb
	#

<!--The apply option only tries to apply the stashed work — you continue to have it on your stack. To remove it, you can run `git stash drop` with the name of the stash to remove:-->

Mit der Option `apply` wird nur versucht die Änderungen wiederherzustellen. Der Stash an sich bleibt weiterhin auf dem Stack vorhanden. Um diesen zu entfernen, kannst Du den Befehl `git stash drop` zusammen mit dem Namen des Stashes anwenden:

	$ git stash list
	stash@{0}: WIP on master: 049d078 added the index file
	stash@{1}: WIP on master: c264051... Revert "added file_size"
	stash@{2}: WIP on master: 21d80a5... added number to log
	$ git stash drop stash@{0}
	Dropped stash@{0} (364e91f3f268f0900bc3ee613f9f733e82aaed43)

<!--You can also run `git stash pop` to apply the stash and then immediately drop it from your stack.-->

Um den Stash zurückzuführen und gleichzeitig vom Stack zu entfernen, kann der Befehl `git stash pop` verwendet werden.

<!--### Un-applying a Stash ###-->
### Zurückgeführten Stash wieder rückgängig machen ###

<!--In some use case scenarios you might want to apply stashed changes, do some work, but then un-apply those changes that originally came from the stash. Git does not provide such a `stash unapply` command, but it is possible to achieve the effect by simply retrieving the patch associated with a stash and applying it in reverse:-->

Stellen wir uns folgendes Szenario vor: Du wendest die Änderungen eines Stashes wieder auf das Arbeitsverzeichnis an und änderst danach noch ein paar Dateien von Hand. Jetzt möchtest Du die Änderungen, die vom Stash her rühren, aber wieder rückgängig machen. Git besitzt kein Feature, welches dies möglich macht. Allerdings kann man den gleichen Effekt erzeugen, indem man vom betreffenden Stash einen Patch erzeugt und diesen mit der Option `-R` wieder anwendet (Patch rückwärts anwenden).

    $ git stash show -p stash@{0} | git apply -R

<!--Again, if you don’t specify a stash, Git assumes the most recent stash:-->

An dieser Stelle noch einmal der Hinweis, dass Git den zuletzt erstellten Stash verwendet, wenn kein Stashname angegeben wird:

    $ git stash show -p | git apply -R

<!--You may want to create an alias and effectively add a `stash-unapply` command to your Git. For example:-->

Wenn Du dieses Feature öfters benötigst, ist es wahrscheinlich sinnvoll, einen Alias `stash-unapply` in Git dafür anzulegen:

    $ git config --global alias.stash-unapply '!git stash show -p | git apply -R'
    $ git stash apply
    $ #... work work work
    $ git stash-unapply

<!--### Creating a Branch from a Stash ###-->
### Branch auf Basis eines Stashes erzeugen ###

<!--If you stash some work, leave it there for a while, and continue on the branch from which you stashed the work, you may have a problem reapplying the work. If the apply tries to modify a file that you’ve since modified, you’ll get a merge conflict and will have to try to resolve it. If you want an easier way to test the stashed changes again, you can run `git stash branch`, which creates a new branch for you, checks out the commit you were on when you stashed your work, reapplies your work there, and then drops the stash if it applies successfully:-->

Wenn Du einen Teil Deiner Arbeit in einem Stash ablegst, dort eine Weile liegen lässt und danach Deine Arbeit an dem Branch fortsetzt, der auch für den Stash verwendet wurde, hast Du vielleicht später Probleme beim Zurückführen des Stashes. Wenn beim Anwenden des Stashes Dateien modifiziert werden sollen, die Du im bisherigen Verlauf bereits geänderst hast, werden Merge-Konflikte auftreten, die Du manuell auflösen musst. Wenn Du nach einer einfachen Möglichkeit suchst, die gestashten Änderungen separat zu testen, kannst Du den Befehl `git stash branch` verwenden. Dieser Befehl erzeugt einen neuen Branch, checkt den Commit aus, auf dessen Basis der Stash erstellt wurde, und führt den Stash wieder in das Arbeitsverzeichnis zurück. Wenn dabei kein Fehler auftritt, wird der Stash automatisch gelöscht:

	$ git stash branch testchanges
	Switched to a new branch "testchanges"
	# On branch testchanges
	# Changes to be committed:
	#   (use "git reset HEAD <file>..." to unstage)
	#
	#      modified:   index.html
	#
	# Changes not staged for commit:
	#   (use "git add <file>..." to update what will be committed)
	#
	#      modified:   lib/simplegit.rb
	#
	Dropped refs/stash@{0} (f0dfc4d5dc332d1cee34a634182e168c4efc3359)

<!--This is a nice shortcut to recover stashed work easily and work on it in a new branch.-->

Damit ist es auf einfache Art und Weise möglich, die gestashten Änderungen in einem neuen Branch wiederherzustellen und daran weiterzuarbeiten.

<!--## Rewriting History ##-->
## Änderungshistorie verändern ##

<!--Many times, when working with Git, you may want to revise your commit history for some reason. One of the great things about Git is that it allows you to make decisions at the last possible moment. You can decide what files go into which commits right before you commit with the staging area, you can decide that you didn’t mean to be working on something yet with the stash command, and you can rewrite commits that already happened so they look like they happened in a different way. This can involve changing the order of the commits, changing messages or modifying files in a commit, squashing together or splitting apart commits, or removing commits entirely — all before you share your work with others.-->

Beim Arbeiten mit Git kommt es häufig vor, dass man seine Commit-Historie aus irgendeinem Grund noch einmal ändern möchte. Und das Tolle an Git ist, dass es Dir die Möglichkeit bietet, Entscheidungen erst im allerletzten Moment zu treffen. Zum Beispiel bietet Dir Git mit Hilfe der Staging-Area die Möglichkeit, alle Dateien zu sammeln und kurz vor einem Commit zu entscheiden, welche Daten alle in einen Commit wandern sollen. Du kannst auch Deine Dateien, die sich geändert haben, aber noch nicht ins Repository eingepflegt werden sollen, mit dem Stash-Kommando in einem Zwischenspeicher ablegen. Außerdem kannst Du bereits verfasste Commits nachträglich noch einmal ändern, sodass sich die Historie so ändert, als wäre sie ganz anders vorangeschritten. Das kann man zum Beispiel durch Änderung der Reihenfolge der Commits, durch Ändern von Commit-Nachrichten, durch Modifikationen an Dateien innerhalb eines Commits, durch Zusammenfügen zweier Commits zu einem Commit oder durch Löschen eines Commits erreichen. Und das Besondere daran: Das alles, bevor Du Deine Arbeit mit anderen teilst und veröffentlichst.

<!--In this section, you’ll cover how to accomplish these very useful tasks so that you can make your commit history look the way you want before you share it with others.-->

In diesem Kapitel werden wir die nützlichen Arbeitsschritte besprechen, die Dir helfen, Deine Commit-Historie Deinen Wünschen entsprechend zu gestalten, sodass Du Dein Ergebnis danach mit anderen teilen kannst und es damit Deinem gewünschten Ergebnis entspricht.

<!--### Changing the Last Commit ###-->
### Ändern des letzten Commits ###

<!--Changing your last commit is probably the most common rewriting of history that you’ll do. You’ll often want to do two basic things to your last commit: change the commit message, or change the snapshot you just recorded by adding, changing and removing files.-->

Am häufigsten möchte man wahrscheinlich seinen letzten durchgeführten Commit noch einmal nachträglich ändern. Meist sind es zwei Dinge, die man verändern möchte: Änderung der eingegebenen Commit-Nachricht oder den eigentlichen Inhalt des Schnappschusses durch Hinzufügen, Ändern oder Löschen von Dateien.

<!--If you only want to modify your last commit message, it’s very simple:-->

Die letzte Commit-Nachricht noch einmal zu ändern ist sehr einfach:

	$ git commit --amend

<!--That drops you into your text editor, which has your last commit message in it, ready for you to modify the message. When you save and close the editor, the editor writes a new commit containing that message and makes it your new last commit.-->

Nach Eingabe dieses Befehls wird der Texteditor mit dem Inhalt der letzten Commit-Nachricht geöffnet. Jetzt hat man Gelegenheit diesen Text zu ändern. Nach dem Speichern und Schließen des Editors, wird die Commit-Nachricht des letzten Commits entsprechend angepasst. Der alte Commit ist dadurch nicht mehr vorhanden und Du erhältst einen neuen Commit mit dem gleichen Inhalt und Deiner neuen Commit-Nachricht.

<!--If you’ve committed and then you want to change the snapshot you committed by adding or changing files, possibly because you forgot to add a newly created file when you originally committed, the process works basically the same way. You stage the changes you want by editing a file and running `git add` on it or `git rm` to a tracked file, and the subsequent `git commit -\-amend` takes your current staging area and makes it the snapshot for the new commit.-->

Wenn Du Deine Änderungen bereits eingecheckt hast und den Schnappschuss nachträglich durch Hinzufügen oder Ändern von Dateien noch einmal ändern möchtest, läuft das im Prinzip auf die gleiche Art und Weise ab. Meist kommt so etwas vor, weil man vergessen hat, eine neu erstellte Datei zu stagen. Wenn so etwas passiert, kannst Du Folgendes machen: Führe Deine gewünschte Änderungen durch Ändern oder Hinzufügen einer Datei aus und stage dieses Ergebnis mit dem Befehl `git add`. Alternativ kannst Du auch mit dem Befehl `git rm` eine Datei aus dem Repository entfernen. Wenn die Staging-Area Dein gewünschtes Ergebnis enthält, führst Du einfach den Befehl  `git commit --amend` aus. Der neue Commit enthält nun die Änderungen aus dem alten Commit plus die Änderungen aus Deiner Staging-Area.

<!--You need to be careful with this technique because amending changes the SHA-1 of the commit. It’s like a very small rebase — don’t amend your last commit if you’ve already pushed it.-->

Mit dem Befehl `--amend` sollte man vorsichtig umgehen, weil sich mit jeder nachträglichen Modifikation eines Commits auch die SHA-1-Prüfsumme ändert. Das Ändern des letzten Commits hat ein ähnliches Verhalten wie das Durchführen eines Rebase-Befehls. Deshalb sollte man einen Commit niemals nachträglich anpassen, wenn dieser bereits veröffentlicht wurde.

<!--### Changing Multiple Commit Messages ###-->
### Änderung von mehreren Commit-Nachrichten ###

<!--To modify a commit that is farther back in your history, you must move to more complex tools. Git doesn’t have a modify-history tool, but you can use the rebase tool to rebase a series of commits onto the HEAD they were originally based on instead of moving them to another one. With the interactive rebase tool, you can then stop after each commit you want to modify and change the message, add files, or do whatever you wish. You can run rebase interactively by adding the `-i` option to `git rebase`. You must indicate how far back you want to rewrite commits by telling the command which commit to rebase onto.-->

Um einen Commit, der etwas weiter in der Historie zurückliegt, zu ändern, hilft einem der Befehl `--amend` nicht weiter. Man benötigt dazu ein etwas mächtigeres und komplexeres Werkzeug. Für diese Aufgabe kann man den Rebase-Befehl, den wir bereits kennengelernt haben, auf eine etwas andere Art und Weise nutzen. Anstatt den Rebase auf einen HEAD eines anderen Commits auszuführen, führt man den Rebase auf genau dem gleichen Commit aus, auf dem er bereits basiert. Dazu müssen wir nur den interaktiven Modus des Rebase-Befehls nutzen. Dieser bietet einem die Möglichkeit bei jedem Commit, der geändert werden soll, zu stoppen. Dann kann man seine Änderungen an den Dateien oder an der Commit-Nachricht entsprechend einpflegen und mit dem nächsten Commit fortfahren. Um einen interaktiven Rebase durchzuführen, muss man die Option `-i` an den Befehl `git rebase` anhängen. Außerdem musst Du natürlich bestimmen, wie viele Commits Du ändern möchtest. Dazu musst Du den Commit angeben, auf welchem der Rebase basieren soll.

<!--For example, if you want to change the last three commit messages, or any of the commit messages in that group, you supply as an argument to `git rebase -i` the parent of the last commit you want to edit, which is `HEAD~2^` or `HEAD~3`. It may be easier to remember the `~3` because you’re trying to edit the last three commits; but keep in mind that you’re actually designating four commits ago, the parent of the last commit you want to edit:-->

Wenn Du zum Beispiel die letzten drei, oder eine oder mehrere der letzten drei Commit-Nachrichten ändern möchtest, musst Du zusätzlich zu dem Befehl `git rebase -i` den übergeordneten Commit (also dem Commit, der in der Historie genau ein Commit zurückliegt) des letzten Commits, den Du ändern möchtest, angeben. Bei drei Commit-Nachrichten müsste das Argument also `HEAD~2^` beziehungsweise `HEAD~3` lauten. Wahrscheinlich fällt es Dir leichter das Argument `~3` zu merken, weil Du ja schließlich auf die letzten drei Einträge verweisen möchtest. Du solltest Dir aber bewusst sein, dass Du auf den viertältesten Commit verweisen musst, also den übergeordneten Commit, den Du ändern möchtest.

	$ git rebase -i HEAD~3

<!--Remember again that this is a rebasing command — every commit included in the range `HEAD~3..HEAD` will be rewritten, whether you change the message or not. Don’t include any commit you’ve already pushed to a central server — doing so will confuse other developers by providing an alternate version of the same change.-->

Es ist wichtig, dass Du Dir bewusst bist, dass mit diesem Rebase-Befehl jeder Commit im Bereich `HEAD~3..HEAD` geändert wird, unabhängig davon, ob Du die Commit-Nachricht beziehungsweise den Schnappschuss änderst oder nicht. Der Rebase-Befehl sollte nie einen Commit beinhalten, der bereits an einen zentralen Server gepusht worden ist.
Hältst Du Dich nicht daran, werden sich andere Entwickler über Dich ärgern oder wundern, weil es jetzt eine alternative Version der gleichen Änderung gibt.

<!--Running this command gives you a list of commits in your text editor that looks something like this:-->

Wenn Du den Befehl ausführst, erhältst Du eine Reihe von Commits in Deinem Texteditor. Das könnte in etwa folgendermaßen aussehen:

	pick f7f3f6d changed my name a bit
	pick 310154e updated README formatting and added blame
	pick a5f4a0d added cat-file

	# Rebase 710f0f8..a5f4a0d onto 710f0f8
	#
	# Commands:
	#  p, pick = use commit
	#  e, edit = use commit, but stop for amending
	#  s, squash = use commit, but meld into previous commit
	#
	# If you remove a line here THAT COMMIT WILL BE LOST.
	# However, if you remove everything, the rebase will be aborted.
	#

<!--It’s important to note that these commits are listed in the opposite order than you normally see them using the `log` command. If you run a `log`, you see something like this:-->

Vielleicht ist es Dir schon aufgefallen, die Commits werden genau in der umgekehrten Reihenfolge dargestellt, wie sie der `log` Befehl ausgegeben hätte. Wenn Du also den Befehl `log` ausführst, erhält man in etwa die folgende Ausgabe:

	$ git log --pretty=format:"%h %s" HEAD~3..HEAD
	a5f4a0d added cat-file
	310154e updated README formatting and added blame
	f7f3f6d changed my name a bit

<!--Notice the reverse order. The interactive rebase gives you a script that it’s going to run. It will start at the commit you specify on the command line (`HEAD~3`) and replay the changes introduced in each of these commits from top to bottom. It lists the oldest at the top, rather than the newest, because that’s the first one it will replay.-->

Siehst Du den Unterschied? Es ist genau die umgekehrte Reihenfolge. Ein interaktiver Rebase wird nach einem festen Schema, einer Art Skript, durchgeführt und der Texteditor zeigt Dir an, wie dieses Skript genau ablaufen wird. Der Rebase startet bei dem Commit, der in der Kommandozeile angegeben wird (`HEAD~3`) und führt die Änderungen, die durch jeden Commit hinzukommen, von oben nach unten aus. Das bedeutet, dass anstatt des neuesten, der älteste Commit ganz oben steht, weil dieser der erste Commit ist, der bearbeitet wird.

<!--You need to edit the script so that it stops at the commit you want to edit. To do so, change the word pick to the word edit for each of the commits you want the script to stop after. For example, to modify only the third commit message, you change the file to look like this:-->

Du musst das Skript so anpassen, dass es an jedem Commit anhält, den Du ändern möchtest. Dazu musst Du bei jedem Commit, an dem das Skript anhalten soll, das Wort „pick“ mit dem Wort „edit“ ersetzen. Um zum Beispiel die drittälteste Commit-Nachricht zu ändern, müssen die Änderungen am Skript in etwa folgendermaßen aussehen:

	edit f7f3f6d changed my name a bit
	pick 310154e updated README formatting and added blame
	pick a5f4a0d added cat-file

<!--When you save and exit the editor, Git rewinds you back to the last commit in that list and drops you on the command line with the following message:-->

Nachdem Du das Skript gespeichert und den Editor beendet hast, setzt Git nun alle Änderungen bis zum letzten Commit der Liste zurück und zeigt danach in der Kommandozeile in etwa Folgendes an:

	$ git rebase -i HEAD~3
	Stopped at 7482e0d... updated the gemspec to hopefully work better
	You can amend the commit now, with

	       git commit --amend

	Once you’re satisfied with your changes, run

	       git rebase --continue

<!--These instructions tell you exactly what to do. Type-->

Diese Anweisungen zeigen Dir sehr genau, was Du zu tun hast. Gib also den folgenden Befehl ein:

	$ git commit --amend

<!--Change the commit message, and exit the editor. Then, run-->

Im sich öffnenden Texteditor kannst Du jetzt die Commit-Nachricht ändern und danch wieder schließen. Danach führst Du folgenden Befehl aus:

	$ git rebase --continue

<!--This command will apply the other two commits automatically, and then you’re done. If you change pick to edit on more lines, you can repeat these steps for each commit you change to edit. Each time, Git will stop, let you amend the commit, and continue when you’re finished.-->

Der letzte Befehl speichert die letzten beiden Commits automatisch im Repository und der Rebase ist danach abgeschlossen. Wenn Du in einer weiteren Zeile „pick“ mit „edit“ ersetzt hast, kannst Du die oben dargestellten Schritte entsprechend noch einmal ausführen. Git wird nach jedem Commit anhalten und Dir die Möglichkeit bieten, den Commit anzupassen. Danach kannst Du Git auffordern, den Rebase fortzusetzen (`git rebase --continue`).

<!--### Reordering Commits ###-->
### Reihenfolge von Commits verändern ###

<!--You can also use interactive rebases to reorder or remove commits entirely. If you want to remove the "added cat-file" commit and change the order in which the other two commits are introduced, you can change the rebase script from this-->

Mit einem interaktiven Rebase kannst Du ebenso die Reihenfolge von Commits ändern oder sogar komplette Commits löschen. Um den Commit „added cat-file“ zu löschen und die Reihenfolge der beiden anderen Commits zu ändern, kannst Du das vorhandene Skript

	pick f7f3f6d changed my name a bit
	pick 310154e updated README formatting and added blame
	pick a5f4a0d added cat-file

<!--to this:-->

folgendermaßen ändern:

	pick 310154e updated README formatting and added blame
	pick f7f3f6d changed my name a bit

<!--When you save and exit the editor, Git rewinds your branch to the parent of these commits, applies `310154e` and then `f7f3f6d`, and then stops. You effectively change the order of those commits and remove the "added cat-file" commit completely.-->

Nach dem Speichern und Verlassen des Editors, setzt Git nun alle Änderungen bis zum letzten Commit der Liste zurück, speichert den Commit `310154e`, danach den Commit `310154e` und beendet danach den Rebase. Das Ergebnis: Der Commit „added cat-file“ ist aus der Historie verschwunden und die Reihenfolge der beiden restlichen Commits ist getauscht.

<!--### Squashing Commits ###-->
### Mehrere Commits zusammenfassen ###

<!--It’s also possible to take a series of commits and squash them down into a single commit with the interactive rebasing tool. The script puts helpful instructions in the rebase message:-->

Man kann mit einem interaktiven Rebase auch mehrere Commits zu einem einzelnen Commit zusammenfassen. Im Skript der Rebase-Nachricht steht eine Anleitung, wie Du dazu vorgehen musst:

	#
	# Commands:
	#  p, pick = use commit
	#  e, edit = use commit, but stop for amending
	#  s, squash = use commit, but meld into previous commit
	#
	# If you remove a line here THAT COMMIT WILL BE LOST.
	# However, if you remove everything, the rebase will be aborted.
	#

<!--If, instead of "pick" or "edit", you specify "squash", Git applies both that change and the change directly before it and makes you merge the commit messages together. So, if you want to make a single commit from these three commits, you make the script look like this:-->

Wenn Du statt „pick“ oder „edit“, den Befehl „squash“ angibst, führt Git beide Commits zu einem gemeinsamen Commit zusammen und bietet Dir die Möglichkeit, die Commit-Nachricht ebenso entsprechend zu verheiraten. Wenn Du also aus den drei Commits einen einzelnen Commit machen willst, muss Dein Skript folgendermaßen aufgebaut sein:

	pick f7f3f6d changed my name a bit
	squash 310154e updated README formatting and added blame
	squash a5f4a0d added cat-file

<!--When you save and exit the editor, Git applies all three changes and then puts you back into the editor to merge the three commit messages:-->

Nach dem Speichern und Beenden des Editors, führt Git alle drei Änderungen zu einem einzelnen Commit zusammen und öffnet einen Texteditor, der alle drei Commit-Nachrichten enthält:


	# This is a combination of 3 commits.
	# The first commit's message is:
	changed my name a bit

	# This is the 2nd commit message:

	updated README formatting and added blame

	# This is the 3rd commit message:

	added cat-file

<!--When you save that, you have a single commit that introduces the changes of all three previous commits.-->

Du kannst nun die Commit-Nachricht entsprechend anpassen oder auch entsprechend dem vorgeschlagenen Ergebnis belassen. Wenn Du die Commit-Nachricht speicherst, hast Du danach nur noch einen einzelnen Commit, der die letzten drei Commits beinhaltet, in Deiner Historie.

<!--### Splitting a Commit ###-->
### Aufsplitten eines einzelnen Commits ###

<!--Splitting a commit undoes a commit and then partially stages and commits as many times as commits you want to end up with. For example, suppose you want to split the middle commit of your three commits. Instead of "updated README formatting and added blame", you want to split it into two commits: "updated README formatting" for the first, and "added blame" for the second. You can do that in the `rebase -i` script by changing the instruction on the commit you want to split to "edit":-->

Man kann mit Git einen einzelnen Commit auch aufsplitten. Das bedeutet, man setzt den ursprünglichen Commit zurück, fügt dann einen Teil der Änderungen zur Staging-Area hinzu und checkt das Ergebnis ein. Dies kann man unbegrenzt oft wiederholen und so einen einzelnen Commit in mehrere Commits aufteilen. Nehmen wir an, wir möchten den mittleren der beiden Commits aufteilen. Anstatt „updated README formatting and added blame“, möchten wir den Commit in folgende beiden Commits aufteilen: „updated README formatting“ soll das Thema des ersten Commits und „added blame“ soll das Thema des zweiten Commits sein. Dazu kannst Du das angezeigte Skript, welches Dir der Befehl `rebase -i` erzeugt, folgendermaßen anpassen:

	pick f7f3f6d changed my name a bit
	edit 310154e updated README formatting and added blame
	pick a5f4a0d added cat-file

<!--When you save and exit the editor, Git rewinds to the parent of the first commit in your list, applies the first commit (`f7f3f6d`), applies the second (`310154e`), and drops you to the console. There, you can do a mixed reset of that commit with `git reset HEAD^`, which effectively undoes that commit and leaves the modified files unstaged. Now you can take the changes that have been reset, and create multiple commits out of them. Simply stage and commit files until you have several commits, and run `git rebase -\-continue` when you’re done:-->

Nach dem Speichern und Schließen des Editors, setzt Git die Änderungen entsprechend zurück und wendet den ersten (`f7f3f6d`) und zweiten (`310154e`) Commit an und wechselt danach zurück zur Kommandozeile. Jetzt hast Du die Möglichkeit den letzten Commit zurückzusetzen, ohne dass die Änderungen im Arbeitsverzeichnis zurückgesetzt werden. Das heißt, der Commit im Repository wird gelöscht, aber Deine Änderungen im Arbeitsverzeichnis gehen nicht verloren. Um dies durchzuführen, kannst Du den Befehl `git reset HEAD^` verwenden. Jetzt kannst Du die gewünschten Änderungen für den ersten Commit zur Staging-Area hinzufügen und danach einchecken. Diesen Vorgang kannst Du beliebig wiederholen, bis alle Änderungen eingecheckt sind. Wenn Du fertig bist, kannst Du den Rebase mit `git rebase --continue` fortsetzen beziehungsweise abschließen:

	$ git reset HEAD^
	$ git add README
	$ git commit -m 'updated README formatting'
	$ git add lib/simplegit.rb
	$ git commit -m 'added blame'
	$ git rebase --continue

<!--Git applies the last commit (`a5f4a0d`) in the script, and your history looks like this:-->

Git speichert dazu den letzten Commit (`a5f4a0d`) aus dem Skript im Repository. Das Resultat sieht in etwa folgendermaßen aus:

	$ git log -4 --pretty=format:"%h %s"
	1c002dd added cat-file
	9b29157 added blame
	35cfb2b updated README formatting
	f3cc40e changed my name a bit

<!--Once again, this changes the SHAs of all the commits in your list, so make sure no commit shows up in that list that you’ve already pushed to a shared repository.-->

Ich möchte Dich noch einmal darauf hinweisen, dass jede SHA-Prüfsumme von allen Commits aus der Liste geändert werden. Bitte stell also sicher, dass diese Commits in keinem öffentlichen Repository verfügbar sind.

<!--### The Nuclear Option: filter-branch ###-->
### Hol den Vorschlaghammer raus: filter-branch ###

<!--There is another history-rewriting option that you can use if you need to rewrite a larger number of commits in some scriptable way — for instance, changing your e-mail address globally or removing a file from every commit. The command is `filter-branch`, and it can rewrite huge swaths of your history, so you probably shouldn’t use it unless your project isn’t yet public and other people haven’t based work off the commits you’re about to rewrite. However, it can be very useful. You’ll learn a few of the common uses so you can get an idea of some of the things it’s capable of.-->

Es gibt noch eine weitere Möglichkeit, wie man die Historie nach seinen Wünschen anpassen kann. Diese wird oft angewandt, wenn man eine große Zahl von Commits automatisiert mit Hilfe eines Skripts anpassen will. Zum Beispiel kann man damit die E-Mail-Adresse in jedem Commit ändern oder auch eine Datei aus jedem Commit entfernen. Das Werkzeug dazu heißt `filter-branch`. Damit kann man einen riesigen Teil der Historie ändern. Man sollte diesen Befehl also nur verwenden, wenn das Projekt noch nicht weit verbreitet ist, oder andere Personen noch nicht damit begonnen haben an dem Projekt zu arbeiten (also auf Basis der bisherigen Historie neue Branches mit Commits erstellt wurden). Trotzdem kann dieses Werkzeug sehr nützlich sein. Ich möchte hier ein paar der Möglichkeiten dieses Werkzeugs vorstellen.

<!--#### Removing a File from Every Commit ####-->
#### Löschen einer Datei aus jedem Commit ####

<!--This occurs fairly commonly. Someone accidentally commits a huge binary file with a thoughtless `git add .`, and you want to remove it everywhere. Perhaps you accidentally committed a file that contained a password, and you want to make your project open source. `filter-branch` is the tool you probably want to use to scrub your entire history. To remove a file named passwords.txt from your entire history, you can use the `-\-tree-filter` option to `filter-branch`:-->

Dieses Szenario tritt sogar relativ häufig auf. Nehmen wir einmal an, jemand fügt gedankenlos eine große binäre Datei mit `git add .` zum Repository dazu und diese soll aber in keinem der Commits enthalten sein. Oder Du hast aus Versehen eine Datei, welche ein Passwort enthält, zum Repository hinzugefügt und möchtest dieses Repository nun veröffentlichen. `filter-branch` ist dann das Werkzeug Deiner Wahl, um die komplette Historie umzukrempeln. Um eine Datei mit dem Namen „passwords.txt“ aus der kompletten Historie zu löschen, kannst Du die Option `--tree-filter` verwenden:

	$ git filter-branch --tree-filter 'rm -f passwords.txt' HEAD
	Rewrite 6b9b3cf04e7c5686a9cb838c3f36a8cb6a0fc2bd (21/21)
	Ref 'refs/heads/master' was rewritten

<!--The `-\-tree-filter` option runs the specified command after each checkout of the project and then recommits the results. In this case, you remove a file called passwords.txt from every snapshot, whether it exists or not. If you want to remove all accidentally committed editor backup files, you can run something like `git filter-branch -\-tree-filter "rm -f *~" HEAD`.-->

Die Option `--tree-filer` führt den nachfolgenden Befehl nach jedem Auschecken eines Commits des Projekts aus und checkt danach das Ergebnis wieder ein. In diesem Beispiel wird die Datei „passwords.txt“ aus jedem Schnappschuss entfernt, unabhängig davon, ob sie existiert oder nicht. Ein anderes Beispiel wäre es, alle Backup-Dateien eines Texteditors aus dem Repository zu löschen. Dazu kann man in etwa den Befehl `git filter-branch -\-tree-filter "rm -f *~" HEAD` ausführen.

<!--You’ll be able to watch Git rewriting trees and commits and then move the branch pointer at the end. It’s generally a good idea to do this in a testing branch and then hard-reset your master branch after you’ve determined the outcome is what you really want. To run `filter-branch` on all your branches, you can pass `-\-all` to the command.-->

Git informiert Dich über den Fortschritt dieses Vorgangs und Du siehst, wie jeder Commit angepasst wird und der Zeiger auf den Branch auf den letzten Commit gesetzt wird. Es ist empfehlenswert, diesen Befehl in einem Testzweig durchzuführen. Wenn das Ergebnis, wie gewünscht ausfällt, kann man danach den Branch master auf diesen Testzweig setzen. Wenn man an den Befehl `filter-branch` die Option `--all` anfügt, führt Git diesen Vorgang für jeden vorhandenen Zweig aus.

<!--#### Making a Subdirectory the New Root ####-->
#### Aus einem Unterverzeichnis das neue Wurzelverzeichnis machen ####

<!--Suppose you’ve done an import from another source control system and have subdirectories that make no sense (trunk, tags, and so on). If you want to make the `trunk` subdirectory be the new project root for every commit, `filter-branch` can help you do that, too:-->

Wenn man zum Beispiel ein Projekt aus einem anderen Versionskontrollwerkzeug in Git importiert, gibt es dort oft Verzeichnisse, die in Git nicht relevant sind, zum Beispiel trunk, tags, usw.. Wenn man also das Unterverzeichnis `trunk` das neue Wurzelverzeichnis machen will, kann man dies mit Hilfe von `filter-branch` umsetzen:

	$ git filter-branch --subdirectory-filter trunk HEAD
	Rewrite 856f0bf61e41a27326cdae8f09fe708d679f596f (12/12)
	Ref 'refs/heads/master' was rewritten

<!--Now your new project root is what was in the `trunk` subdirectory each time. Git will also automatically remove commits that did not affect the subdirectory.-->

Nach der Ausführung dieses Befehls ist das „trunk“ Verzeichnis das neue Arbeitsverzeichnis. Bei diesem Vorgang entfernt Git außerdem alle Commits, die nicht eine Änderung des „trunk“-Verzeichnisses beinhalten.

<!--#### Changing E-Mail Addresses Globally ####-->
#### E-Mail Adresse in jedem Commit ändern ####

<!--Another common case is that you forgot to run `git config` to set your name and e-mail address before you started working, or perhaps you want to open-source a project at work and change all your work e-mail addresses to your personal address. In any case, you can change e-mail addresses in multiple commits in a batch with `filter-branch` as well. You need to be careful to change only the e-mail addresses that are yours, so you use `-\-commit-filter`:-->

Verflixt, es ist schon wieder passiert. Du hast vergessen, den Befehl `git config` auszuführen und Deinen Namen und E-Mail-Adresse zu setzen, bevor Du mit der Arbeit begonnen hast. Mit `filter-branch` kann man diesen Fehler einfach beheben. Man sollte nur darauf achten, dass man nur seine eigene E-Mail-Adresse ändert. Deshalb verwenden wir die Option `--commit-filter`:

	$ git filter-branch --commit-filter '
	        if [ "$GIT_AUTHOR_EMAIL" = "schacon@localhost" ];
	        then
	                GIT_AUTHOR_NAME="Scott Chacon";
	                GIT_AUTHOR_EMAIL="schacon@example.com";
	                git commit-tree "$@";
	        else
	                git commit-tree "$@";
	        fi' HEAD

<!--This goes through and rewrites every commit to have your new address. Because commits contain the SHA-1 values of their parents, this command changes every commit SHA in your history, not just those that have the matching e-mail address.-->

Dieser Befehl durchforstet das Repository und ersetzt in jedem Commit, dessen E-Mail-Adresse des Autors „schacon@localhost“ lautet, mit der neuen E-Mail-Adresse „schacon@example.com“. Zusätzlich wird der Name des Autors geändert, falls dieser nicht vorher schon „Scott Chacon“ war. Auf Grund der Architektur, dass in Git in jedem Commit die SHA1-Prüfsumme des Vorgänger-Commits enthalten ist, ändert dieser Befehl jeden Commit in Deiner Historie. Die SHA1-Prüfsumme wird sich auch in allen Commits, die nicht die angegebene E-Mail-Adresse enthalten, verändern.

<!--## Debugging with Git ##-->
## Mit Hilfe von Git debuggen ##

<!--Git also provides a couple of tools to help you debug issues in your projects. Because Git is designed to work with nearly any type of project, these tools are pretty generic, but they can often help you hunt for a bug or culprit when things go wrong.-->

Git bietet auch ein paar Werkzeuge, die den Debug-Vorgang bei einem Projekt unterstützen. Da Git so aufgebaut ist, dass es für nahezu jedes Projekt eingesetzt werden kann, sind diese Werkzeuge sehr generisch gehalten. Wenn gewisse Dinge schief laufen, können Dir aber diese Tools oft helfen, den Bug oder den Übeltäter zu finden.

<!--### File Annotation ###-->
### Datei Annotation ###

<!--If you track down a bug in your code and want to know when it was introduced and why, file annotation is often your best tool. It shows you what commit was the last to modify each line of any file. So, if you see that a method in your code is buggy, you can annotate the file with `git blame` to see when each line of the method was last edited and by whom. This example uses the `-L` option to limit the output to lines 12 through 22:-->

Wenn Du nach einem Bug in Deinem Code suchst und gerne wissen willst, wann und warum dieser zum ersten Mal auftrat, dann kann Dir das Werkzeug Datei-Annotation (engl. File Annotation) sicher weiterhelfen. Es kann Dir anzeigen, in welchem Commit die jeweilige Zeile einer Datei zuletzt geändert wurde. Wenn Du also feststellst, dass eine Methode beziehungsweise eine Funktion in Deinem Code nicht mehr das gewünschte Resultat liefert, kannst Du Dir die Datei mit `git blame` genauer ansehen. Nach Aufruf des Befehls zeigt Git Dir an, welche Zeile von welcher Person als letztes geändert wurde, inklusive Datum. Das folgende Beispiel verwendet die Option `-L`, um die Ausgabe auf die Zeilen 12 bis 22 einzuschränken:

	$ git blame -L 12,22 simplegit.rb
	^4832fe2 (Scott Chacon  2008-03-15 10:31:28 -0700 12)  def show(tree = 'master')
	^4832fe2 (Scott Chacon  2008-03-15 10:31:28 -0700 13)   command("git show #{tree}")
	^4832fe2 (Scott Chacon  2008-03-15 10:31:28 -0700 14)  end
	^4832fe2 (Scott Chacon  2008-03-15 10:31:28 -0700 15)
	9f6560e4 (Scott Chacon  2008-03-17 21:52:20 -0700 16)  def log(tree = 'master')
	79eaf55d (Scott Chacon  2008-04-06 10:15:08 -0700 17)   command("git log #{tree}")
	9f6560e4 (Scott Chacon  2008-03-17 21:52:20 -0700 18)  end
	9f6560e4 (Scott Chacon  2008-03-17 21:52:20 -0700 19)
	42cf2861 (Magnus Chacon 2008-04-13 10:45:01 -0700 20)  def blame(path)
	42cf2861 (Magnus Chacon 2008-04-13 10:45:01 -0700 21)   command("git blame #{path}")
	42cf2861 (Magnus Chacon 2008-04-13 10:45:01 -0700 22)  end

<!--Notice that the first field is the partial SHA-1 of the commit that last modified that line. The next two fields are values extracted from that commit—the author name and the authored date of that commit — so you can easily see who modified that line and when. After that come the line number and the content of the file. Also note the `^4832fe2` commit lines, which designate that those lines were in this file’s original commit. That commit is when this file was first added to this project, and those lines have been unchanged since. This is a tad confusing, because now you’ve seen at least three different ways that Git uses the `^` to modify a commit SHA, but that is what it means here.-->

In der ersten Spalte wird die Kurzform der SHA1-Prüfsumme des Commits angezeigt, in welchem diese Zeile zuletzt verändert wurde. Die nächsten beiden Spalten weisen auf den Autor des Commits und wann dieser verfasst wurde, hin. Auf diese Weise kannst Du leicht bestimmen, wer die jeweilige Zeile geändert hat und wann dies durchgeführt wurde. In den nächsten Spalten wird die Zeilennummer und der Inhalt der Zeile angezeigt. Die Zeilen mit der SHA-1-Prüfsumme `^4832fe2` weisen darauf hin, dass diese bereits im ersten Commit vorhanden waren. Das ist also der Commit, in dem die Datei „simplegit.rb“ zum Repository hinzugefügt wurde und die Zeilen deuten damit darauf hin, dass diese bisher nie geändert wurden. Das ist für Dich wahrscheinlich ein bisschen verwirrend, denn nun kennst Du bereits drei Möglichkeiten, wie Git mit dem Zeichen `^` einer SHA-Prüfsumme eine neue Bedeutung gibt. Aber in Zusammenhang mit  `git blame` weist das Zeichen auf den eben geschilderten Sachverhalt hin.

<!--Another cool thing about Git is that it doesn’t track file renames explicitly. It records the snapshots and then tries to figure out what was renamed implicitly, after the fact. One of the interesting features of this is that you can ask it to figure out all sorts of code movement as well. If you pass `-C` to `git blame`, Git analyzes the file you’re annotating and tries to figure out where snippets of code within it originally came from if they were copied from elsewhere. Recently, I was refactoring a file named `GITServerHandler.m` into multiple files, one of which was `GITPackUpload.m`. By blaming `GITPackUpload.m` with the `-C` option, I could see where sections of the code originally came from:-->

Eine weitere herausragende Eigenschaft von Git ist die Tatsache, dass es nicht per se das Umbenennen von Dateien verfolgt. Git speichert immer den jeweiligen Schnappschuss des Dateisystems und versucht erst danach zu bestimmen, welche Dateien umbenannt wurden. Das bietet Dir zum Beispiel die Möglichkeit herauszufinden, wie Code innerhalb des Repositorys hin und her verschoben wurde. Wenn Du also die Option `-C` an `git blame` anfügst, analysiert Git die angegebene Datei und versucht herauszufinden, ob und von wo bestimmte Codezeilen herkopiert wurden. Vor kurzem habe ich ein Refactoring an einer Datei mit dem Namen `GITServerHandler.m` durchgeführt. Dabei habe ich diese Datei in mehrere Dateien aufgeteilt, eine davon war `GITPackUpload.m`. Wenn ich jetzt `git blame` mit der Option `-C` auf die Datei `GITPackUpload.m` ausführe, erhalte ich eine Ausgabe mit den Codezeilen, von denen das Ergebnis ursprünglich stammt:

	$ git blame -C -L 141,153 GITPackUpload.m
	f344f58d GITServerHandler.m (Scott 2009-01-04 141)
	f344f58d GITServerHandler.m (Scott 2009-01-04 142) - (void) gatherObjectShasFromC
	f344f58d GITServerHandler.m (Scott 2009-01-04 143) {
	70befddd GITServerHandler.m (Scott 2009-03-22 144)         //NSLog(@"GATHER COMMI
	ad11ac80 GITPackUpload.m    (Scott 2009-03-24 145)
	ad11ac80 GITPackUpload.m    (Scott 2009-03-24 146)         NSString *parentSha;
	ad11ac80 GITPackUpload.m    (Scott 2009-03-24 147)         GITCommit *commit = [g
	ad11ac80 GITPackUpload.m    (Scott 2009-03-24 148)
	ad11ac80 GITPackUpload.m    (Scott 2009-03-24 149)         //NSLog(@"GATHER COMMI
	ad11ac80 GITPackUpload.m    (Scott 2009-03-24 150)
	56ef2caf GITServerHandler.m (Scott 2009-01-05 151)         if(commit) {
	56ef2caf GITServerHandler.m (Scott 2009-01-05 152)                 [refDict setOb
	56ef2caf GITServerHandler.m (Scott 2009-01-05 153)

<!--This is really useful. Normally, you get as the original commit the commit where you copied the code over, because that is the first time you touched those lines in this file. Git tells you the original commit where you wrote those lines, even if it was in another file.-->

Das ist enorm hilfreich. Für gewöhnlich erhältst Du damit als ursprünglichen Commit, den Commit, von welchem der Code kopiert wurde, da dies der Zeitpunkt war, bei dem diese Zeilen zum ersten Mal angefasst wurden. Git zeigt Dir den ursprünglichen Commit, in dem Du die Zeilen verfasst hast, sogar an, wenn es sich dabei um eine andere Datei handelt.

<!--### Binary Search ###-->
### Das Bisect Werkzeug – Binäre Suche###

<!--Annotating a file helps if you know where the issue is to begin with. If you don’t know what is breaking, and there have been dozens or hundreds of commits since the last state where you know the code worked, you’ll likely turn to `git bisect` for help. The `bisect` command does a binary search through your commit history to help you identify as quickly as possible which commit introduced an issue.-->

`git blame` kann Dir sehr weiterhelfen, wenn Du bereits weißt, an welcher Stelle das Problem liegt. Wenn Du aber nicht weißt, warum gewisse Dinge schief laufen, und es gibt inzwischen Dutzende oder Hunderte von Commits seit dem letzten funktionierenden Stand, dann solltest Du `git bisect` als Hilfestellung verwenden. Der `bisect` Befehl führt eine binäre Suche durch die Commit-Historie durch und hilft Dir auf schnelle Art und Weise die Commits zu bestimmen, die eventuell für das Problem verantwortlich sind.

<!--Let’s say you just pushed out a release of your code to a production environment, you’re getting bug reports about something that wasn’t happening in your development environment, and you can’t imagine why the code is doing that. You go back to your code, and it turns out you can reproduce the issue, but you can’t figure out what is going wrong. You can bisect the code to find out. First you run `git bisect start` to get things going, and then you use `git bisect bad` to tell the system that the current commit you’re on is broken. Then, you must tell bisect when the last known good state was, using `git bisect good [good_commit]`:-->

Nehmen wir zum Beispiel an, dass Du gerade eben Deinen Code in einer Produktivumgebung veröffentlicht hast und auf einmal bekommst Du zahlreiche Fehlerberichte über Probleme, die in Deiner Entwicklungsumgebung nicht aufgetreten sind. Du kannst Dir auch keinen Reim darauf bilden, warum der Code so reagiert. Nachdem Du Dich noch einmal näher mit Deinem Code beschäftigt hast, stellst Du fest, dass Du die Fehlerwirkung reproduzieren kannst, aber Dir ist es immer noch ein Rätsel, was genau schief läuft. Wenn Du vor einem solchen Problem stehst, hilft Dir es bestimmt, wenn Du die Historie in mehrere Teile aufspaltest (engl. bisect: halbieren, zweiteilen). Als erstes startest Du mit dem Befehl `git bisect start`. Danach gibst Du mit dem Befehl `git bisect bad` an, dass der derzeit ausgecheckte Commit den Fehler aufweist. Jetzt braucht Git noch die Information, in welchem Commit das Problem noch nicht aufgetreten ist. Dazu verwendest Du den Befehl `git bisect good [good_commit]`:

	$ git bisect start
	$ git bisect bad
	$ git bisect good v1.0
	Bisecting: 6 revisions left to test after this
	[ecb6e1bc347ccecc5f9350d878ce677feb13d3b2] error handling on repo

<!--Git figured out that about 12 commits came between the commit you marked as the last good commit (v1.0) and the current bad version, and it checked out the middle one for you. At this point, you can run your test to see if the issue exists as of this commit. If it does, then it was introduced sometime before this middle commit; if it doesn’t, then the problem was introduced sometime after the middle commit. It turns out there is no issue here, and you tell Git that by typing `git bisect good` and continue your journey:-->

Nach Ausführen des letzten Befehls, zeigt Git Dir als Erstes an, dass in etwa 12 Commits zwischen der letzten guten Revision (v1.0) und der aktuellen, fehlerhaften Revision liegen. Auf Basis dieser Information hat Git Dir den mittleren Commit ausgecheckt. Jetzt hast Du die Möglichkeit Deine Tests auf Basis des ausgecheckten Stands durchzuführen, um herauszufinden, ob in diesem Commit der Fehler bereits bestand. Wenn der Fehler hier bereits auftritt, dann wurde er in diesem oder in einem der früheren Commits eingefügt. Wenn der Fehler hier noch nicht auftritt, dann wurde er in einem der späteren Commits eingeschleppt. In unserem Beispiel nehmen wir an, dass in diesem Commit der Fehler noch nicht bestand. Das geben wir mit dem Befehl `git bisect good` an und fahren fort:

	$ git bisect good
	Bisecting: 3 revisions left to test after this
	[b047b02ea83310a70fd603dc8cd7a6cd13d15c04] secure this thing

<!--Now you’re on another commit, halfway between the one you just tested and your bad commit. You run your test again and find that this commit is broken, so you tell Git that with `git bisect bad`:-->

Git hat Dir jetzt einen weiteren Commit ausgecheckt, und zwar wieder den mittleren Commit zwischen dem letzten Stand im Repository und dem mittleren Commit aus der letzten Runde. Hier nehmen wir an, dass Du nach Deinen durchgeführten Tests feststellst, dass in diesem Commit der Fehler bereits vorhanden ist. Das müssen wir Git über `git bisect bad` mitteilen:

	$ git bisect bad
	Bisecting: 1 revisions left to test after this
	[f71ce38690acf49c1f3c9bea38e09d82a5ce6014] drop exceptions table

<!--This commit is fine, and now Git has all the information it needs to determine where the issue was introduced. It tells you the SHA-1 of the first bad commit and show some of the commit information and which files were modified in that commit so you can figure out what happened that may have introduced this bug:-->

Git checkt wieder den nächsten mittleren Commit aus und wir stellen fest, dass dieser in Ordnung ist. Ab jetzt hat Git alle notwendigen Informationen um festzustellen, in welchem Commit der Fehler eingebaut wurde. Git zeigt Dir dazu die SHA-1-Prüfsumme des ersten fehlerhaften Commits an. Zusätzlich gibt es noch weitere Commit-Informationen und welche Dateien in diesem Commit geändert wurden, an. Das sollte Dir nun helfen, den Fehler näher zu bestimmen:

	$ git bisect good
	b047b02ea83310a70fd603dc8cd7a6cd13d15c04 is first bad commit
	commit b047b02ea83310a70fd603dc8cd7a6cd13d15c04
	Author: PJ Hyett <pjhyett@example.com>
	Date:   Tue Jan 27 14:48:32 2009 -0800

	    secure this thing

	:040000 040000 40ee3e7821b895e52c1695092db9bdc4c61d1730
	f24d3c6ebcfc639b1a3814550e62d60b8e68a8e4 M  config

<!--When you’re finished, you should run `git bisect reset` to reset your HEAD to where you were before you started, or you’ll end up in a weird state:-->

Wenn Du fertig mit der Fehlersuche bist, solltest Du den Befehl `git bisect reset` ausführen. Dies checkt den ursprünglichen Stand aus, den Du ausgecheckt hattest, bevor Du mit der Fehlersuche begonnen hast:

	$ git bisect reset

<!--This is a powerful tool that can help you check hundreds of commits for an introduced bug in minutes. In fact, if you have a script that will exit 0 if the project is good or non-0 if the project is bad, you can fully automate `git bisect`. First, you again tell it the scope of the bisect by providing the known bad and good commits. You can do this by listing them with the `bisect start` command if you want, listing the known bad commit first and the known good commit second:-->

Wie Du vielleicht gesehen hast, ist dieser Befehl ein mächtiges Werkzeug, um Hunderte von Commits auf schnelle Art und Weise nach einem bestimmten Fehler zu durchsuchen. Besonders nützlich ist es, wenn Du ein Skript hast, welches mit dem Fehlercode Null beendet, wenn das Projekt in Ordnung ist und mit einem Fehlercode größer Null, wenn das Projekt Fehler enthält. Wenn Dir ein solches Skript zur Verfügung steht, kannst Du den bisher manuell durchgeführten Vorgang auch automatisieren. Wie im vorigen Beispiel musst Du Git den zuletzt fehlerfreien Commit und den fehlerhaften Commit angeben. Als verkürzte Schreibweise kannst Du an den Befehl `bisect start` den fehlerhaften und den fehlerfreien Commit angeben:

	$ git bisect start HEAD v1.0
	$ git bisect run test-error.sh

<!--Doing so automatically runs `test-error.sh` on each checked-out commit until Git finds the first broken commit. You can also run something like `make` or `make tests` or whatever you have that runs automated tests for you.-->

Wenn Du die untere, genannte Zeile ausführst, führt Git automatisch nach jedem Auscheckvorgang das Skript `test-error.sh` aus und zwar solange bis es den Commit findet, der als erstes ein fehlerhaftes Ergebnis liefert. Statt eines Skripts kannst Du natürlich auch `make` oder `make tests` oder eine beliebige, andere Testumgebung starten.

<!--## Submodules ##-->
## Submodule ##

<!--It often happens that while working on one project, you need to use another project from within it. Perhaps it’s a library that a third party developed or that you’re developing separately and using in multiple parent projects. A common issue arises in these scenarios: you want to be able to treat the two projects as separate yet still be able to use one from within the other.-->

Oft möchte man während der Arbeit an einem Projekt ein weiteres Projekt darin einbinden und verwenden. Das kann zum Beispiel eine Bibliothek von einer anderen Firma oder vielleicht auch eine selbstentwickelte Bibliothek sein. In einem solchen Szenario tritt dann meistens folgendes Problem auf: Die zwei Projekte sollen unabhängig voneinander entwickelt werden können, aber trotzdem soll es möglich sein, das eine Projekt im anderen zu verwenden.

<!--Here’s an example. Suppose you’re developing a web site and creating Atom feeds. Instead of writing your own Atom-generating code, you decide to use a library. You’re likely to have to either include this code from a shared library like a CPAN install or Ruby gem, or copy the source code into your own project tree. The issue with including the library is that it’s difficult to customize the library in any way and often more difficult to deploy it, because you need to make sure every client has that library available. The issue with vendoring the code into your own project is that any custom changes you make are difficult to merge when upstream changes become available.-->

Dazu ein Beispiel. Nehmen wir an, Du entwickelst gerade eine Webseite, die unter anderem einen Atom-Feed zur Verfügung stellen soll. Anstatt den notwendigen Code zur Auslieferung des Atom-Feeds selber zu schreiben, entscheidest Du Dich für eine geeignete Bibliothek. Dann wirst Du wahrscheinlich den Code in Dein Projekt einbinden müssen, zum Beispiel durch eine CPAN-Installation oder ein Ruby-Gem oder durch Kopieren des Quellcodes in das Arbeitsverzeichnis Deines Projekts. Das Problem beim Einbinden einer Bibliothek ist, dass es schwierig ist, diese an die eigene Bedürfnisse anzupassen. Noch schwieriger gestaltet sich dann die Veröffentlichung des Projekts, da man sicherstellen muss, dass jeder der die Software verwendet, auf die Bibliothek zugreifen kann. Wenn man die Bibliothek im eigenen Projekt projektspezifisch anpasst, hat man meist ein Problem, wenn man eine neue Version der Bibliothek einspielen will.

<!--Git addresses this issue using submodules. Submodules allow you to keep a Git repository as a subdirectory of another Git repository. This lets you clone another repository into your project and keep your commits separate.-->

Git löst dieses Problem mit Hilfe von Submodulen. Mit Hilfe von Submodulen kann man innerhalb eines Git-Repositorys ein weiteres Git-Repository in einem Unterverzeichnis verwalten. Daraus entsteht der Vorteil, dass man ein anderes Repository in das eigene Projekt klonen kann und die Commits der jeweiligen Projekte trennen kann.

<!--### Starting with Submodules ###-->
### Die ersten Schritte mit Submodulen ###

<!--Suppose you want to add the Rack library (a Ruby web server gateway interface) to your project, possibly maintain your own changes to it, but continue to merge in upstream changes. The first thing you should do is clone the external repository into your subdirectory. You add external projects as submodules with the `git submodule add` command:-->

Nehmen wir einmal an, dass Du die Rack-Bibliothek (eine Ruby-Gateway-Schnittstelle für Webserver) zu Deinem Projekt hinzufügen willst. Dabei möchtest Du Deine eigenen Änderungen an dieser Bibliothek nachverfolgen, aber auch weiterhin Änderungen von den Rack-Bibliothek-Entwicklern verwalten und zusammenmergen. Das erste was Du dazu tun musst, ist das Repository der Rack Bibliothek in ein Unterverzeichnis Deines Projekts zu klonen. Diesen Vorgang kannst Du mit Hilfe des Befehls `git submodule add` ausführen:

	$ git submodule add git://github.com/chneukirchen/rack.git rack
	Initialized empty Git repository in /opt/subtest/rack/.git/
	remote: Counting objects: 3181, done.
	remote: Compressing objects: 100% (1534/1534), done.
	remote: Total 3181 (delta 1951), reused 2623 (delta 1603)
	Receiving objects: 100% (3181/3181), 675.42 KiB | 422 KiB/s, done.
	Resolving deltas: 100% (1951/1951), done.

<!--Now you have the Rack project under a subdirectory named `rack` within your project. You can go into that subdirectory, make changes, add your own writable remote repository to push your changes into, fetch and merge from the original repository, and more. If you run `git status` right after you add the submodule, you see two things:-->

Innerhalb Deines Projekts befindet sich nun im Unterverzeichnis `rack` das komplette Rack-Projekt. Man kann jetzt in diesem Verzeichnis Änderungen vornehmen und ein eigenes Remote-Repository mit Schreibrechten, zu welchem man pushen kann, hinzufügen. Ebenso ist es möglich, Änderungen von den Rack-Entwicklern in sein Repository zu laden und diese mit den eigenen Ergebnissen zu mergen. Im Prinzip kann man innerhalb eines Submoduls die gleichen Vorgänge, wie in einem normalen Repository ausführen. Vorher müssen wir aber noch ein paar weitere Dinge zu Submodulen besprechen. Wenn Du gleich nach dem Hinzufügen des Submoduls, den Befehl `git status` ausführst, wirst Du gleich zwei Dinge bemerken:

	$ git status
	# On branch master
	# Changes to be committed:
	#   (use "git reset HEAD <file>..." to unstage)
	#
	#      new file:   .gitmodules
	#      new file:   rack
	#

<!--First you notice the `.gitmodules` file. This is a configuration file that stores the mapping between the project’s URL and the local subdirectory you’ve pulled it into:-->

Als erstes wird Dir die neue Datei `.gitmodules` auffallen. Das ist eine Konfigurationsdatei, welche die Zuordnung der URL des geklonten Projekts und dem lokalen Unterverzeichnis, in welches das Projekt geklont wurde, festlegt:

	$ cat .gitmodules
	[submodule "rack"]
	      path = rack
	      url = git://github.com/chneukirchen/rack.git

<!--If you have multiple submodules, you’ll have multiple entries in this file. It’s important to note that this file is version-controlled with your other files, like your `.gitignore` file. It’s pushed and pulled with the rest of your project. This is how other people who clone this project know where to get the submodule projects from.-->

Wenn Du mehrere Submodule in einem Projekt verwaltest, werden auch mehrere Einträge in dieser Datei auftauchen. Dabei ist es wichtig zu wissen, dass diese Datei zusammen mit all den anderen Dateien aus Deinem Projekt, ebenso in die Versionskontrolle aufgenommen wird, ähnlich wie die Datei `.gitignore`. Die Datei wird so wie der Rest Deines Projekts gepusht und gepullt. Dadurch wissen andere Personen, die Dein Projekt klonen, von welchem Ort sie die Submodule erhalten können.

<!--The other listing in the `git status` output is the rack entry. If you run `git diff` on that, you see something interesting:-->

Die zweite Auffälligkeit bei der Ausgabe von `git status` ist der Eintrag rack. Wenn Du auf diesen Eintrag ein `git diff` durchführst, erhält man in etwa die folgende, interessante Ausgabe:

	$ git diff --cached rack
	diff --git a/rack b/rack
	new file mode 160000
	index 0000000..08d709f
	--- /dev/null
	+++ b/rack
	@@ -0,0 +1 @@
	+Subproject commit 08d709f78b8c5b0fbeb7821e37fa53e69afcf433

<!--Although `rack` is a subdirectory in your working directory, Git sees it as a submodule and doesn’t track its contents when you’re not in that directory. Instead, Git records it as a particular commit from that repository. When you make changes and commit in that subdirectory, the superproject notices that the HEAD there has changed and records the exact commit you’re currently working off of; that way, when others clone this project, they can re-create the environment exactly.-->

Obwohl `rack` ein Unterverzeichnis in Deinem Arbeitsverzeichnis ist, erkennt Git dieses Verzeichnis als Submodul und verfolgt die Änderungen innerhalb dieses Verzeichnisses nicht, wenn Git nicht innerhalb dieses Verzeichnisses aufgerufen wird. Stattdessen erfasst Git, welcher Commit in diesem Repository ausgecheckt ist. Wenn Du also Änderungen in diesem Unterverzeichnis durchführst und eincheckst, kann das Superprojekt erkennen, dass sich der aktuelle HEAD von diesem Projekt geändert hat. Das Superprojekt kann sich jetzt diesen Commit merken. Auf diese Art und Weise ist es möglich, den kompletten Zustand des Projekts mit allen Projekten, die als Submodul hinzugefügt wurden, zu reproduzieren. In der Git-Terminologie wird das Projekt, welches eines oder mehrere Submodule enthält, als Superprojekt bezeichnet.

<!--This is an important point with submodules: you record them as the exact commit they’re at. You can’t record a submodule at `master` or some other symbolic reference.-->

Dabei muss man sich folgender Eigenschaft bewusst sein: Git verwaltet den exakten Commit, der gerade ausgecheckt ist und nicht etwa den Branch oder eine andere Referenz. Git kann also zum Beispiel nicht speichern, dass der aktuelle Stand im Branch `master` enthalten ist.

<!--When you commit, you see something like this:-->

Wenn Du Dein Projekt das erste Mal eincheckst, erhältst Du in etwa folgende Ausgabe:

	$ git commit -m 'first commit with submodule rack'
	[master 0550271] first commit with submodule rack
	 2 files changed, 4 insertions(+), 0 deletions(-)
	 create mode 100644 .gitmodules
	 create mode 160000 rack

<!--Notice the 160000 mode for the rack entry. That is a special mode in Git that basically means you’re recording a commit as a directory entry rather than a subdirectory or a file.-->

Der Mode 160000, der für den rack-Eintrag gilt, ist ein spezieller Mode in Git. Er bedeutet in etwa, dass man einen Commit als Verzeichnis-Eintrag in Git verwaltet und damit nicht wie normalerweise ein Verzeichnis oder eine Datei.

<!--You can treat the `rack` directory as a separate project and then update your superproject from time to time with a pointer to the latest commit in that subproject. All the Git commands work independently in the two directories:-->

Das Verzeichnis `rack` kann man wie ein separates Projekt behandeln und verwenden. Und von Zeit zur Zeit aktualisiert man auch das Superprojekt und speichert damit die letzte Commit-ID des Unterprojekts. Jedes Git-Kommando arbeitet unabhängig in einem der beiden Unterverzeichnisse:

	$ git log -1
	commit 0550271328a0038865aad6331e620cd7238601bb
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Thu Apr 9 09:03:56 2009 -0700

	    first commit with submodule rack
	$ cd rack/
	$ git log -1
	commit 08d709f78b8c5b0fbeb7821e37fa53e69afcf433
	Author: Christian Neukirchen <chneukirchen@gmail.com>
	Date:   Wed Mar 25 14:49:04 2009 +0100

	    Document version change

<!--### Cloning a Project with Submodules ###-->
### Klonen eines Projekts mit den dazugehörigen Submodulen ###

<!--Here you’ll clone a project with a submodule in it. When you receive such a project, you get the directories that contain submodules, but none of the files yet:-->

Als nächstes klonen wir ein Projekt, welches ein Submodul verwendet. Wenn man ein solches Projekt klont, werden die entsprechenden Verzeichnisse, welche ein Submodul enthalten, erstellt. Allerdings enthalten diese Verzeichnisse noch keinen Inhalt:

	$ git clone git://github.com/schacon/myproject.git
	Initialized empty Git repository in /opt/myproject/.git/
	remote: Counting objects: 6, done.
	remote: Compressing objects: 100% (4/4), done.
	remote: Total 6 (delta 0), reused 0 (delta 0)
	Receiving objects: 100% (6/6), done.
	$ cd myproject
	$ ls -l
	total 8
	-rw-r--r--  1 schacon  admin   3 Apr  9 09:11 README
	drwxr-xr-x  2 schacon  admin  68 Apr  9 09:11 rack
	$ ls rack/
	$

<!--The `rack` directory is there, but empty. You must run two commands: `git submodule init` to initialize your local configuration file, and `git submodule update` to fetch all the data from that project and check out the appropriate commit listed in your superproject:-->

Das Verzeichnis `rack` wurde zwar erzeugt, aber es ist leer. Deshalb musst Du zwei Dinge ausführen: `git submodule init`, damit wird die Datei für die lokale Konfiguration initialisiert. Und `git submodule update`, welches die gesamten Daten des Projekts von der im `.gitmodules` angegebenen Quelle holt und den entsprechenden Commit, welcher im Superprojekt hinterlegt ist, auscheckt:

	$ git submodule init
	Submodule 'rack' (git://github.com/chneukirchen/rack.git) registered for path 'rack'
	$ git submodule update
	Initialized empty Git repository in /opt/myproject/rack/.git/
	remote: Counting objects: 3181, done.
	remote: Compressing objects: 100% (1534/1534), done.
	remote: Total 3181 (delta 1951), reused 2623 (delta 1603)
	Receiving objects: 100% (3181/3181), 675.42 KiB | 173 KiB/s, done.
	Resolving deltas: 100% (1951/1951), done.
	Submodule path 'rack': checked out '08d709f78b8c5b0fbeb7821e37fa53e69afcf433'

<!--Now your `rack` subdirectory is at the exact state it was in when you committed earlier. If another developer makes changes to the rack code and commits, and you pull that reference down and merge it in, you get something a bit odd:-->

Nach Ausführung der beiden Befehle befindet sich das Verzeichnis `rack` in genau dem gleichen Zustand, wie wir es ursprünglich eingecheckt haben. Wenn ein anderer Entwickler Änderungen am Rack-Code durchführt, diese eincheckt und Du diese dann pullst und mergst, erhält man einen etwas seltsamen Zustand:

	$ git merge origin/master
	Updating 0550271..85a3eee
	Fast forward
	 rack |    2 +-
	 1 files changed, 1 insertions(+), 1 deletions(-)
	[master*]$ git status
	# On branch master
	# Changes not staged for commit:
	#   (use "git add <file>..." to update what will be committed)
	#   (use "git checkout -- <file>..." to discard changes in working directory)
	#
	#      modified:   rack
	#

<!--You merged in what is basically a change to the pointer for your submodule; but it doesn’t update the code in the submodule directory, so it looks like you have a dirty state in your working directory:-->

Der Merge, der gerade durchgeführt worden ist, ist eigentlich nur eine Aktualisierung des Zeigers auf einen neuen Commit des Submoduls. Der eigentliche Inhalt des Submodul-Verzeichnis wurde allerdings nicht aktualisiert. Das sieht dann so aus, als gäbe es noch nicht eingecheckte Dateien innerhalb Deines Arbeitsverzeichnis:

	$ git diff
	diff --git a/rack b/rack
	index 6c5e70b..08d709f 160000
	--- a/rack
	+++ b/rack
	@@ -1 +1 @@
	-Subproject commit 6c5e70b984a60b3cecd395edd5b48a7575bf58e0
	+Subproject commit 08d709f78b8c5b0fbeb7821e37fa53e69afcf433

<!--This is the case because the pointer you have for the submodule isn’t what is actually in the submodule directory. To fix this, you must run `git submodule update` again:-->

Dieser Zustand tritt auf, weil der Zeiger auf den Commit im Submodul derzeit nicht der Commit ist, welcher im Submodul ausgecheckt ist. Um dies zu beheben, muss man den Befehl `git submodule update` erneut ausführen:

	$ git submodule update
	remote: Counting objects: 5, done.
	remote: Compressing objects: 100% (3/3), done.
	remote: Total 3 (delta 1), reused 2 (delta 0)
	Unpacking objects: 100% (3/3), done.
	From git@github.com:schacon/rack
	   08d709f..6c5e70b  master     -> origin/master
	Submodule path 'rack': checked out '6c5e70b984a60b3cecd395edd5b48a7575bf58e0'

<!--You have to do this every time you pull down a submodule change in the main project. It’s strange, but it works.-->

Dieser Update muss jedes Mal ausgeführt werden, wenn man das Superprojekt pullt und dort ein Änderung in einem Submodul enthalten ist. Es ist vielleicht ein wenig merkwürdig, aber es funktioniert.

<!--One common problem happens when a developer makes a change locally in a submodule but doesn’t push it to a public server. Then, they commit a pointer to that non-public state and push up the superproject. When other developers try to run `git submodule update`, the submodule system can’t find the commit that is referenced, because it exists only on the first developer’s system. If that happens, you see an error like this:-->

Häufig tritt beim Arbeiten mit Submodulen ein Problem bei folgendem Szenario auf: Ein Entwickler führt Änderungen in einem Submodul durch, checkt diese ein, vergisst aber diese Änderungen zum zentralen Server zu pushen. Wenn dann im Superprojekt die Änderung des Submoduls ebenso eingecheckt wird und dieses dann gepusht wird, tritt ein Problem auf. Wenn jetzt andere Entwickler den neuen Stand des Superprojekts holen und den Befehl `git submodule update` ausführen, erhalten sie eine Fehlermeldung, dass der entsprechend referenzierte Commit von dem Submodul nicht gefunden werden konnte. Das passiert weil dieser Commit bei dem zweiten Entwickler noch gar nicht existiert. Wenn ein solcher Fall auftritt, erhält man in etwa folgende Fehlermeldung:

	$ git submodule update
	fatal: reference isn’t a tree: 6c5e70b984a60b3cecd395edd5b48a7575bf58e0
	Unable to checkout '6c5e70b984a60b3cecd395edd5ba7575bf58e0' in submodule path 'rack'

<!--You have to see who last changed the submodule:-->

Dann kann man allerdings herausfinden, wer zum letzten Mal eine Änderung eingecheckt hat:

	$ git log -1 rack
	commit 85a3eee996800fcfa91e2119372dd4172bf76678
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Thu Apr 9 09:19:14 2009 -0700

	    added a submodule reference I will never make public. hahahahaha!

<!--Then, you e-mail that guy and yell at him.-->

Dann kannst Du diesem Entwickler eine E-Mail schreiben und ihn auf seinen Fehler aufmerksam machen.

<!--### Superprojects ###-->
### Superprojekte ###

<!--Sometimes, developers want to get a combination of a large project’s subdirectories, depending on what team they’re on. This is common if you’re coming from CVS or Subversion, where you’ve defined a module or collection of subdirectories, and you want to keep this type of workflow.-->

In manchen großen Projekten möchten die Entwickler die Arbeit in verschiedenen Verzeichnissen aufteilen, sodass das jeweilige Team in diesen Verzeichnissen arbeiten kann. Man trifft diese Vorgehensweise häufig an, wenn ein Team gerade von CVS oder Subversion nach Git gewechselt hat, im alten System ein Modul oder eine Sammlung von solchen Unterverzeichnissen gebildet hat und diesen Arbeitsablauf weiterhin verwenden möchte.

<!--A good way to do this in Git is to make each of the subdirectories a separate Git repository and then create superproject Git repositories that contain multiple submodules. A benefit of this approach is that you can more specifically define the relationships between the projects with tags and branches in the superprojects.-->

In Git kann man diese Vorgehensweise gut abbilden, indem man für jedes Unterverzeichnis ein neues Git-Repository erzeugt. Zusätzlich kann man dann ein Superprojekt erzeugen und die ganzen Git-Repositorys als Submodul hinzufügen. Ein Vorteil dabei ist, dass man mit Hilfe von Tags und Branches im Superprojekt das Verhältnis der einzelnen Submodule zueinander festhalten kann.

<!--### Issues with Submodules ###-->
### Häufige Probleme mit Submodulen ###

<!--Using submodules isn’t without hiccups, however. First, you must be relatively careful when working in the submodule directory. When you run `git submodule update`, it checks out the specific version of the project, but not within a branch. This is called having a detached HEAD — it means the HEAD file points directly to a commit, not to a symbolic reference. The issue is that you generally don’t want to work in a detached HEAD environment, because it’s easy to lose changes. If you do an initial `submodule update`, commit in that submodule directory without creating a branch to work in, and then run `git submodule update` again from the superproject without committing in the meantime, Git will overwrite your changes without telling you.  Technically you won’t lose the work, but you won’t have a branch pointing to it, so it will be somewhat difficult to retrieve.-->

Die Arbeit mit Submodulen verläuft jedoch nicht immer reibungslos. Man muss verhältnismäßig gut aufpassen, wenn man in einem Submodul-Verzeichnis arbeitet. Wenn man nämlich den Befehl `git submodule update` ausführt, checkt Git den entsprechenden Zustand des Commits aus, aber checkt dabei keinen Branch aus. Diesen Zustand nennt man auch `detached HEAD`. Das bedeutet, dass die Datei HEAD direkt auf einen Commit zeigt und nicht, wie sonst üblich, auf eine symbolische Referenz, also zum Beispiel auf einen Branch. Das Problem dabei ist, dass man normalerweise in einem solchen Zustand nicht weiterarbeiten möchte, weil es sehr leicht vorkommen kann, dass Änderungen verloren gehen. Wenn man also den Befehl `git submodule update` aufruft, dann einen Commit in dem entsprechenden Submodul-Verzeichnis ausführt, ohne davor einen Branch auszuchecken, und dann noch einmal `git submodule update` im Superprojekt aufruft, ohne dass man die Änderungen im Submodul im Superprojekt eingecheckt hat, verliert man die ganzen Änderungen, ohne dass Git einen darauf vorher hinweist. Tatsächlich ist es so, dass die Änderungen nicht verloren gehen, aber es gibt keinen Branch, der auf die entsprechenden Commits hinzeigt, und damit kann es schwierig werden, die entsprechenden Commits wiederherzustellen beziehungsweise sichtbar zu machen.

<!--To avoid this issue, create a branch when you work in a submodule directory with `git checkout -b work` or something equivalent. When you do the submodule update a second time, it will still revert your work, but at least you have a pointer to get back to.-->

Um dieses Problem zu vermeiden, sollte man also immer in dem Submodul-Verzeichnis einen neuen Branch mit `git checkout -b work` oder auf eine andere Art und Weise erzeugen. Wenn man dann wieder das Aktualisieren des Submoduls ausführt, wird Git wieder den ursprünglichen Commit auschecken, allerdings hat man jetzt mit dem Branch einen Zeiger auf die neuen Commits und man kann sie leicht wieder auschecken.

<!--Switching branches with submodules in them can also be tricky. If you create a new branch, add a submodule there, and then switch back to a branch without that submodule, you still have the submodule directory as an untracked directory:-->

Wenn ein Projekt ein Submodul enthält und man im Superprojekt zwischen einzelnen Branches hin und her wechseln möchte, kann sich das manchmal auch schwierig gestalten. Wenn man zum Beispiel einen neuen Branch erzeugt, in diesem dann ein Submodul hinzufügt und dann wieder in den ursprünglichen Branch, welcher das Submodul noch nicht enthält, zurückwechselt, hat man im Arbeitsverzeichnis immer noch das Submodul-Verzeichnis, welches auch so dargestellt wird, als ob es von Git noch nicht verfolgt wird:

	$ git checkout -b rack
	Switched to a new branch "rack"
	$ git submodule add git@github.com:schacon/rack.git rack
	Initialized empty Git repository in /opt/myproj/rack/.git/
	...
	Receiving objects: 100% (3184/3184), 677.42 KiB | 34 KiB/s, done.
	Resolving deltas: 100% (1952/1952), done.
	$ git commit -am 'added rack submodule'
	[rack cc49a69] added rack submodule
	 2 files changed, 4 insertions(+), 0 deletions(-)
	 create mode 100644 .gitmodules
	 create mode 160000 rack
	$ git checkout master
	Switched to branch "master"
	$ git status
	# On branch master
	# Untracked files:
	#   (use "git add <file>..." to include in what will be committed)
	#
	#      rack/

<!--You have to either move it out of the way or remove it, in which case you have to clone it again when you switch back—and you may lose local changes or branches that you didn’t push up.-->

In diesem Fall muss man das Verzeichnis entweder an einen anderen Ort verschieben oder löschen. Im letzteren Fall muss man aber wieder das Submodul komplett klonen, wenn man in den anderen Zweig zurückwechselt. Außerdem kann man dabei lokale Änderungen zunichte machen oder Zweige, welche man noch nicht gepusht hat, verlieren.

<!--The last main caveat that many people run into involves switching from subdirectories to submodules. If you’ve been tracking files in your project and you want to move them out into a submodule, you must be careful or Git will get angry at you. Assume that you have the rack files in a subdirectory of your project, and you want to switch it to a submodule. If you delete the subdirectory and then run `submodule add`, Git yells at you:-->

Die letzte Falle, in die viele Leute tappen, tritt auf, wenn man bereits vorhandene Verzeichnisse in Submodule umwandeln will. Wenn man also Dateien, die bereits von Git verwaltet werden, entfernen und in ein entsprechendes Submodul verschieben möchte, muss man vorsichtig sein. Ansonsten können schwer zu behebende Probleme mit Git auftreten. Nehmen wir zum Beispiel an, dass Du die Dateien vom Rack-Projekt in ein Unterverzeichnis Deines Projekts abgelegt hast und diese jetzt aber in ein Submodul verschieben möchtest. Wenn Du das Unterverzeichnis einfach löschst und dann den Befehl `submodule add` ausführst, zeigt Dir Git folgende Fehlermeldung an:

	$ rm -Rf rack/
	$ git submodule add git@github.com:schacon/rack.git rack
	'rack' already exists in the index

<!--You have to unstage the `rack` directory first. Then you can add the submodule:-->

Man muss dann das Verzeichnis `rack` erst aus der Staging-Area entfernen. Danach kann man dann das Submodul erzeugen:

	$ git rm -r rack
	$ git submodule add git@github.com:schacon/rack.git rack
	Initialized empty Git repository in /opt/testsub/rack/.git/
	remote: Counting objects: 3184, done.
	remote: Compressing objects: 100% (1465/1465), done.
	remote: Total 3184 (delta 1952), reused 2770 (delta 1675)
	Receiving objects: 100% (3184/3184), 677.42 KiB | 88 KiB/s, done.
	Resolving deltas: 100% (1952/1952), done.

<!--Now suppose you did that in a branch. If you try to switch back to a branch where those files are still in the actual tree rather than a submodule — you get this error:-->

Wenn wir jetzt annehmen, dass Du diesen Vorgang innerhalb eines Zweigs durchgeführt hast und jetzt auf einen anderen Zweig, in dem das Submodul noch nicht existiert hat und damit die Dateien noch ganz normal im Repository enthalten waren, wechselst, erhält Du folgenden Fehler:

	$ git checkout master
	error: Untracked working tree file 'rack/AUTHORS' would be overwritten by merge.

<!--You have to move the `rack` submodule directory out of the way before you can switch to a branch that doesn’t have it:-->

Dann musst Du das Submodul-Verzeichnis `rack` an einen anderen Ort verschieben, bevor Du diesen Branch auschecken kannst:

	$ mv rack /tmp/
	$ git checkout master
	Switched to branch "master"
	$ ls
	README	rack

<!--Then, when you switch back, you get an empty `rack` directory. You can either run `git submodule update` to reclone, or you can move your `/tmp/rack` directory back into the empty directory.-->

Wenn man dann wieder in den Zweig mit dem Submodul zurückwechseln will, erhält man ein leeres Verzeichnis `rack`. Um dieses zu befüllen, kannst Du entweder `git submodule update` ausführen oder Du kannst Deine Kopie von `/tmp/rack` wieder an den ursprünglichen Ort wiederherstellen.

<!--## Subtree Merging ##-->
## Subtree Merging ##

<!--Now that you’ve seen the difficulties of the submodule system, let’s look at an alternate way to solve the same problem. When Git merges, it looks at what it has to merge together and then chooses an appropriate merging strategy to use. If you’re merging two branches, Git uses a _recursive_ strategy. If you’re merging more than two branches, Git picks the _octopus_ strategy. These strategies are automatically chosen for you because the recursive strategy can handle complex three-way merge situations — for example, more than one common ancestor — but it can only handle merging two branches. The octopus merge can handle multiple branches but is more cautious to avoid difficult conflicts, so it’s chosen as the default strategy if you’re trying to merge more than two branches.-->

Nachdem wir neben den Vor- und Nachteilen beim Arbeiten mit Submodulen kennengelernt haben, möchte ich jetzt noch eine alternative Lösung zeigen, wie man ähnliche Probleme lösen kann. Wenn Git etwas zusammenführt, also mergt, analysiert es die Teile, die es mergen muss. Auf Basis dieser Analyse entscheidet Git sich für eine geeignete Merging-Methode. Wenn man zwei Branches mergt, dann verwendet Git automatisch die sogenannte Recursive-Strategie. Wenn man mehr als zwei Branches mergt, verwendet Git die sogenannte Octopus-Strategie. Diese Strategien werden automatisch für Dich gewählt, weil die Recursive-Strategie normalerweise sehr gut geeignet ist, um einen Drei-Wege-Merge (engl. three-way merge) durchzuführen — zum Beispiel, wenn es mehr als einen gemeinsamen Vorgänger-Commit gibt — aber der Drei-Wege-Merge ist nur für das Mergen von zwei Branches geeignet. Die Octopus-Merge-Strategie kann mehrere Branches zusammenführen, aber es wird dabei vorsichtiger vorgegangen, um schwierig aufzulösende Konflikte zu vermeiden. Aus diesem Grund wird diese Strategie standardmäßig verwendet, wenn man mehr als zwei Branches zusammenführen möchte.

<!--However, there are other strategies you can choose as well. One of them is the _subtree_ merge, and you can use it to deal with the subproject issue. Here you’ll see how to do the same rack embedding as in the last section, but using subtree merges instead.-->

Es gibt jedoch noch weitere Strategien, die man verwenden kann. Einer dieser Strategien ist der sogenannte Subtree-Merge. Dies kann verwendet werden, um unser Problem mit Unterprojekten zu lösen. Ich möchte Dir im Folgenden aufzeigen, wie man das Rack-Projekt aus dem letzten Kapitel einbindet und dabei den Subtree-Merge anstatt der Submodule verwendet.

<!--The idea of the subtree merge is that you have two projects, and one of the projects maps to a subdirectory of the other one and vice versa. When you specify a subtree merge, Git is smart enough to figure out that one is a subtree of the other and merge appropriately — it’s pretty amazing.-->

Das Prinzip, das hinter einem Subtree-Merge steckt, ist, dass man zwei Projekte hat und eines der Projekte wird in ein Unterverzeichnis des anderen Projekts abgebildet. Wenn man ein Subtree-Merge ausführt, ist Git schlau genug, um zu erkennen, dass ein Projekt ein Abbild von einem anderen Projekt ist und es kann den Merge in geeigneter Weise durchführen — das ist wirklich sehr erstaunlich.

<!--You first add the Rack application to your project. You add the Rack project as a remote reference in your own project and then check it out into its own branch:-->

Als erstes musst Du dazu die Rack-Applikation zu Deinem Projekt hinzufügen. Dazu fügst Du das Rack-Projekt als neues Remote-Repository in Deinem Projekt hinzu und checkst dieses in einem separaten Branch aus:

	$ git remote add rack_remote git@github.com:schacon/rack.git
	$ git fetch rack_remote
	warning: no common commits
	remote: Counting objects: 3184, done.
	remote: Compressing objects: 100% (1465/1465), done.
	remote: Total 3184 (delta 1952), reused 2770 (delta 1675)
	Receiving objects: 100% (3184/3184), 677.42 KiB | 4 KiB/s, done.
	Resolving deltas: 100% (1952/1952), done.
	From git@github.com:schacon/rack
	 * [new branch]      build      -> rack_remote/build
	 * [new branch]      master     -> rack_remote/master
	 * [new branch]      rack-0.4   -> rack_remote/rack-0.4
	 * [new branch]      rack-0.9   -> rack_remote/rack-0.9
	$ git checkout -b rack_branch rack_remote/master
	Branch rack_branch set up to track remote branch refs/remotes/rack_remote/master.
	Switched to a new branch "rack_branch"

<!--Now you have the root of the Rack project in your `rack_branch` branch and your own project in the `master` branch. If you check out one and then the other, you can see that they have different project roots:-->

Nach der Ausführung der drei Befehle befindet sich das Rack-Projekt in Deinem Branch `rack_branch` und Dein eigenes Projekt liegt weiterhin im Branch `master`. Wenn man jetzt den jeweiligen Zweig auscheckt, sieht man die unterschiedlichen Inhalte im Wurzelverzeichnis:

	$ ls
	AUTHORS	       KNOWN-ISSUES   Rakefile      contrib	       lib
	COPYING	       README         bin           example	       test
	$ git checkout master
	Switched to branch "master"
	$ ls
	README

<!--You want to pull the Rack project into your `master` project as a subdirectory. You can do that in Git with `git read-tree`. You’ll learn more about `read-tree` and its friends in Chapter 9, but for now know that it reads the root tree of one branch into your current staging area and working directory. You just switched back to your `master` branch, and you pull the `rack` branch into the `rack` subdirectory of your `master` branch of your main project:-->

Jetzt möchten wir das Rack-Projekt in Deinen Branch `master` als Unterverzeichnis hinzufügen. Dies kann man in Git mit dem Befehl `git read-tree` durchführen. In Kapitel 9 werde ich den Befehl `read-tree` und dessen verwandte Befehle näher erläutern. Hier möchte ich nur erklären, dass der Befehl das Wurzelverzeichnis eines Branches in die aktuelle Staging-Area und in das Arbeitsverzeichnis packt. Damit hast Du jetzt zu Deinem Branch `master` zurückgewechselt, den Inhalt des Branches `rack_branch` in das Unterverzeichnis `rack` im Branch `master` Deines Projekts hinterlegt:

	$ git read-tree --prefix=rack/ -u rack_branch

<!--When you commit, it looks like you have all the Rack files under that subdirectory — as though you copied them in from a tarball. What gets interesting is that you can fairly easily merge changes from one of the branches to the other. So, if the Rack project updates, you can pull in upstream changes by switching to that branch and pulling:-->

Wenn Du jetzt einen Commit ausführst, erscheint es einem so, als ob die ganzen Dateien aus dem Rack-Projekt in diesem Unterverzeichnis liegen — als ob man das Projekt aus einem Tarball-Container hineinkopiert hätte. Das Besondere ist jetzt aber, dass man Änderungen zwischen den verschiedenen Branches jetzt einfach zusammenführen kann. Das bedeutet, wenn das Rack-Projekt aktualisiert wird, kann man sich diese Änderungen einfach holen, indem man in diesen Branch wechselt und dort einen Pull durchführt:

	$ git checkout rack_branch
	$ git pull

<!--Then, you can merge those changes back into your master branch. You can use `git merge -s subtree` and it will work fine; but Git will also merge the histories together, which you probably don’t want. To pull in the changes and prepopulate the commit message, use the `-\-squash` and `-\-no-commit` options as well as the `-s subtree` strategy option:-->

Danach kann man diese Änderungen wieder in den Branch master mergen. Wenn man den Befehl `git merge -s subtree` verwendet, sollte dies einwandfrei funktionieren. Allerdings wird Git bei Ausführen dieses Befehls auch die jeweilige Historie mergen, was Du wahrscheinlich nicht haben möchtest. Um nun die Änderungen zu holen und eine entsprechende Commit-Nachricht vorzubereiten, hängt man einfach `--squash`, `--no-commit` und natürlich `-s subtree` als Option an:

	$ git checkout master
	$ git merge --squash -s subtree --no-commit rack_branch
	Squash commit -- not updating HEAD
	Automatic merge went well; stopped before committing as requested

<!--All the changes from your Rack project are merged in and ready to be committed locally. You can also do the opposite — make changes in the `rack` subdirectory of your master branch and then merge them into your `rack_branch` branch later to submit them to the maintainers or push them upstream.-->

Die ganzen Änderungen des Rack-Projekts wurden nun zusammengeführt, Du musst jetzt nur noch einen entsprechenden Commit durchführen. Man kann aber auch genau das Gegenteil machen: Man führt Änderungen im Unterverzeichnis `rack` des Branches master aus und mergt diese dann später in den Zweig `rack_branch`. Diesen kann man dann den Entwicklern des Rack-Projekts zur Verfügung stellen.

<!--To get a diff between what you have in your `rack` subdirectory and the code in your `rack_branch` branch — to see if you need to merge them — you can’t use the normal `diff` command. Instead, you must run `git diff-tree` with the branch you want to compare to:-->

Um die Unterschiede zwischen dem Inhalt in Deinem Verzeichnis `rack` und dem Code im Zweig `rack_branch` anzuzeigen, kann man keinen normalen Vergleich mit `diff` durchführen. Man muss stattdessen den Befehl `git diff-tree` verwenden und als Argument den zu vergleichenden Branch angeben:

	$ git diff-tree -p rack_branch

<!--Or, to compare what is in your `rack` subdirectory with what the `master` branch on the server was the last time you fetched, you can run-->

Um Dein Verzeichnis `rack` mit dem letzten Stand des Branches `master` auf dem Server zu vergleichen, kannst Du folgenden Befehl verwenden:

	$ git diff-tree -p rack_remote/master

<!--## Summary ##-->
## Zusammenfassung ##

<!--You’ve seen a number of advanced tools that allow you to manipulate your commits and staging area more precisely. When you notice issues, you should be able to easily figure out what commit introduced them, when, and by whom. If you want to use subprojects in your project, you’ve learned a few ways to accommodate those needs. At this point, you should be able to do most of the things in Git that you’ll need on the command line day to day and feel comfortable doing so.-->

In diesem Kapitel hast Du viele ausgeklügelte Werkzeuge kennengelernt, die es Dir ermöglichen, Commits und die Staging-Area nach Deinen Vorstellungen zu beeinflussen. Wenn ein Problem in Deinem Projekt auftaucht, solltest Du jetzt leicht bestimmen können, welcher Commit den Fehler verursacht hat, sowie wann und von wem der Fehler begangen wurde. Wenn Du andere Projekte in Deinem Projekt verwenden möchtest, hast Du jetzt mehrere Möglichkeiten kennengelernt, wie Du dies handhaben kannst. An dieser Stelle solltest Du jetzt in der Lage sein, die meisten Dinge, die Du bei der täglichen Arbeit benötigst, in der Kommandozeile durchzuführen, ohne dass Dir dabei Schweißperlen auf der Stirn stehen.
<!--# Customizing Git #-->
# Git individuell einrichten #

<!--So far, I’ve covered the basics of how Git works and how to use it, and I’ve introduced a number of tools that Git provides to help you use it easily and efficiently. In this chapter, I’ll go through some operations that you can use to make Git operate in a more customized fashion by introducing several important configuration settings and the hooks system. With these tools, it’s easy to get Git to work exactly the way you, your company, or your group needs it to.-->

Ich habe nun die grundlegende Funktionsweise und die Benutzung von Git besprochen. Weiterhin habe ich einige Werkzeuge von Git präsentiert, die dem Benutzer ein einfaches und effizientes Arbeiten ermöglichen. In diesem Kapitel werde ich nun auf einige Operationen eingehen, die Du benutzen kannst um die Funktionsweise von Git Deinen persönlichen Bedürfnissen anzupassen. Dazu führe ich einige wichtige Konfigurationseinstellungen ein, sowie verschiedene Einschubmethoden, auch Hooks genannt. Mit diesen Mitteln kann man Git leicht anpassen, sodass es genau Deinen Ansprüchen, des Unternehmens oder des Teams entspricht.

<!--## Git Configuration ##-->
## Git Konfiguration ##

<!--As you briefly saw in the Chapter 1, you can specify Git configuration settings with the `git config` command. One of the first things you did was set up your name and e-mail address:-->

Wie in Kapitel 1 schon kurz beschrieben, kann man die Konfiguration von Git mit Hilfe des Befehls `git config` steuern. Einer Deiner ersten Aktionen war es, Deinen Namen und E-Mail Adresse anzugeben:

	$ git config --global user.name "John Doe"
	$ git config --global user.email johndoe@example.com

<!--Now you’ll learn a few of the more interesting options that you can set in this manner to customize your Git usage.-->

Jetzt wirst Du einige weitere, interessantere Optionen kennenlernen, die Du auf gleiche Art und Weise einsetzen kannst, um Git Deiner Arbeitsumgebung anzupassen.

<!--You saw some simple Git configuration details in the first chapter, but I’ll go over them again quickly here. Git uses a series of configuration files to determine non-default behavior that you may want. The first place Git looks for these values is in an `/etc/gitconfig` file, which contains values for every user on the system and all of their repositories. If you pass the option `-\-system` to `git config`, it reads and writes from this file specifically.-->

In Kapitel 1 hast Du bereits Deine ersten Erfahrungen mit einigen einfachen Einstellparametern von Git gemacht, aber ich möchte sie hier noch einmal kurz wiederholen. Git verwendet eine Reihe von Konfigurationsdateien, um Deine persönliche Einstellungen, welche von den Standard-Einstellungen abweichen, festzuhalten. Zu aller erst prüft Git die Einstellungen in der Datei `/etc/gitconfig`. Diese Datei enthält Werte, welche für alle Benutzer des Systems und deren Repositorys gelten. Wenn Du `git config` mit der Option `--system` benutzt, liest und schreibt Git von genau dieser Datei.

<!--The next place Git looks is the `~/.gitconfig` file, which is specific to each user. You can make Git read and write to this file by passing the `-\-global` option.-->

Als nächstes prüft Git die Datei `~/.gitconfig`, welche nur für den jeweiligen Benutzer gilt. Damit Git diese Datei zum Lesen und Schreiben nutzt, kannst Du die Option `--global` angeben.

<!--Finally, Git looks for configuration values in the config file in the Git directory (`.git/config`) of whatever repository you’re currently using. These values are specific to that single repository. Each level overwrites values in the previous level, so values in `.git/config` trump those in `/etc/gitconfig`, for instance. You can also set these values by manually editing the file and inserting the correct syntax, but it’s generally easier to run the `git config` command.-->

Als Letztes sucht Git in der Konfigurationsdatei im Git Verzeichnis des gerade verwendeten Repositorys (`.git/config`). Die dort enthaltenen Parameter sind nur für dieses einzelne Repository gültig. Jede der erwähnten Ebenen überschreibt die vorhergehende. Das bedeutet, dass z.B. die Einstellungen in der Datei `/etc/gitconfig` von den Einstellungen in der Datei `.git/config` überschrieben werden. Du kannst alle Parameter auch durch manuelles Editieren der jeweiligen Datei setzen bzw. verändern (vorausgesetzt Du verwendest die richtige Syntax). In der Regel ist es aber einfacher den Befehl `git config` zu verwenden.

<!--### Basic Client Configuration ###-->
### Grundlegende Client Konfiguration ###

<!--The configuration options recognized by Git fall into two categories: client side and server side. The majority of the options are client side—configuring your personal working preferences. Although tons of options are available, I’ll only cover the few that either are commonly used or can significantly affect your workflow. Many options are useful only in edge cases that I won’t go over here. If you want to see a list of all the options your version of Git recognizes, you can run-->

Einstellparameter in Git lassen sich in zwei Kategorien aufteilen: Parameter für die Client-Konfiguration und für die Server-Konfiguration. Der Großteil der Einstellungen bezieht sich auf den Client – zur Konfiguration Deines persönlichen Arbeitsablaufs. Auch wenn es eine große Anzahl an Einstellmöglichkeiten gibt, werde ich nur die wenigen besprechen, die sehr gebräuchlich sind oder Deine Arbeitsweise bedeutend beeinflussen können. Viele Optionen sind nur für Spezialfälle interessant, auf die ich hier aber nicht weiter eingehen möchte. Falls Du eine Liste aller Optionen haben willst, kannst Du folgenden Befehl ausführen:

	$ git config --help

<!--The manual page for `git config` lists all the available options in quite a bit of detail.-->

Die Hilfeseite zu `git config` listet alle verfügbaren Optionen sehr detailliert auf.

<!--#### core.editor ####-->
#### core.editor ####

<!--By default, Git uses whatever you’ve set as your default text editor or else falls back to the Vi editor to create and edit your commit and tag messages. To change that default to something else, you can use the `core.editor` setting:-->

In der Grundeinstellung benutzt Git Deinen Standard Texteditor oder greift auf den Vi Editor zurück, um Deine Commit und Tag Nachrichten zu erstellen und zu bearbeiten. Um einen andern Editor als Standard einzurichten kannst Du die Option `core.editor` nutzen:

	$ git config --global core.editor emacs

<!--Now, no matter what is set as your default shell editor variable, Git will fire up Emacs to edit messages.-->

Ab jetzt wird Git immer Emacs starten um Nachrichten zu editieren, unabhängig davon welcher Standard Shell-Editor gesetzt ist.

<!--#### commit.template ####-->
#### commit.template ####

<!--If you set this to the path of a file on your system, Git will use that file as the default message when you commit. For instance, suppose you create a template file at `$HOME/.gitmessage.txt` that looks like this:-->

Wenn Du diese Einstellung auf einen Pfad zu einer Datei auf Deinem System einstellst, wird Git den Inhalt dieser Datei als Standard Commit Nachricht verwenden. Nehmen wir zum Beispiel an, Du erstellst eine Vorlage unter dem Namen `$HOME/.gitmessage.txt`, die den folgenden Inhalt hat:

	subject line

	what happened

	[ticket: X]

<!--To tell Git to use it as the default message that appears in your editor when you run `git commit`, set the `commit.template` configuration value:-->

Damit Git diese Datei als Standard Nachricht benutzt, die in Deinem Editor erscheint, wenn Du `git commit` aufrufst, richte die Option `commit.template` ein:

	$ git config --global commit.template $HOME/.gitmessage.txt
	$ git commit

<!--Then, your editor will open to something like this for your placeholder commit message when you commit:-->

Wenn Du dann das nächste Mal einen Commit durchführst, wird Dein Editor mit etwa der folgenden Nachricht starten:

	subject line

	what happened

	[ticket: X]
	# Please enter the commit message for your changes. Lines starting
	# with '#' will be ignored, and an empty message aborts the commit.
	# On branch master
	# Changes to be committed:
	#   (use "git reset HEAD <file>..." to unstage)
	#
	# modified:   lib/test.rb
	#
	~
	~
	".git/COMMIT_EDITMSG" 14L, 297C

<!--If you have a commit-message policy in place, then putting a template for that policy on your system and configuring Git to use it by default can help increase the chance of that policy being followed regularly.-->

Falls eine Richtlinie für Commit Nachrichten existiert, solltest Du Git so konfigurieren, dass eine Vorlage davon bei einem Commit geladen wird. Dies erhöht die Chance, dass diese Richtlinie auch eingehalten wird.

<!--#### core.pager ####-->
#### core.pager ####

<!--The core.pager setting determines what pager is used when Git pages output such as `log` and `diff`. You can set it to `more` or to your favorite pager (by default, it’s `less`), or you can turn it off by setting it to a blank string:-->

Die Einstellung `core.pager` legt fest, welche Anwendung zur Seitenanzeige benutzt wird, wenn Git Text ausgibt, wie zum Beispiel bei `log` und `diff`. Du kannst es auch auf `more` oder eine andere Seitenanzeige Deiner Wahl (der Standard ist `less`) einstellen, oder Du kannst es mittels eines leeren Strings ganz ausschalten:

	$ git config --global core.pager ''

<!--If you run that, Git will page the entire output of all commands, no matter how long it is.-->

Wenn Du dies ausführst, wird Git immer die komplette Ausgabe aller Befehle anzeigen, egal wie lange sie ist.

<!--#### user.signingkey ####-->
#### user.signingkey ####

<!--If you’re making signed annotated tags (as discussed in Chapter 2), setting your GPG signing key as a configuration setting makes things easier. Set your key ID like so:-->

Falls Du signierte kommentierte Tags erstellst (wie in Kapitel 2 beschrieben), so macht es die Arbeit leichter, wenn Du Deinen GPG Signierschlüssel in Git festlegst. Du kannst Deine Schlüssel ID wie folgt festlegen:

	$ git config --global user.signingkey <gpg-key-id>

<!--Now, you can sign tags without having to specify your key every time with the `git tag` command:-->

Beim Signieren von Tags mit Hilfe von `git tag` musst Du Deinen Schlüssel jetzt nicht mehr angeben. Es reicht folgendes auszuführen:

	$ git tag -s <tag-name>

<!--#### core.excludesfile ####-->
#### core.excludesfile ####

<!--You can put patterns in your project’s `.gitignore` file to have Git not see them as untracked files or try to stage them when you run `git add` on them, as discussed in Chapter 2. However, if you want another file outside of your project to hold those values or have extra values, you can tell Git where that file is with the `core.excludesfile` setting. Simply set it to the path of a file that has content similar to what a `.gitignore` file would have.-->

In Kapitel 2 habe ich bereits beschrieben, wie Du mit Hilfe der projektspezifischen `.gitignore` Datei Git dazu bringst, bestimmte Dateien nicht weiter zu verfolgen beziehungsweise zu stagen, wenn Du den Befehl `git add` verwendest. Falls Du jedoch eine weitere Datei außerhalb Deines Projekts verwenden willst, die diese Werte enthält oder zusätzliche Muster definiert, dann kannst Du Git mit der Option `core.excludesfile` mitteilen, wo sich diese Datei befindet. Trage hier einfach den Pfad zu einer Datei ein, welche entsprechend einer `.gitignore` Datei aufgebaut ist.

<!--#### help.autocorrect ####-->
#### help.autocorrect ####

<!--This option is available only in Git 1.6.1 and later. If you mistype a command in Git, it shows you something like this:-->

Diese Option ist in Git ab Version 1.6.1 verfügbar. Wenn Du in Git einen Befehl falsch schreibst, bekommst Du eine Meldung wie diese:

	$ git com
	git: 'com' is not a git-command. See 'git --help'.

	Did you mean this?
	     commit

<!--If you set `help.autocorrect` to 1, Git will automatically run the command if it has only one match under this scenario.-->

Wenn Du die Option `help.autocorrect` auf 1 setzt, wird Git automatisch den entsprechenden Befehl ausführen, falls es in dieser Situation die einzige passende Alternative ist.

<!--### Colors in Git ###-->
### Farben in Git ###

<!--Git can color its output to your terminal, which can help you visually parse the output quickly and easily. A number of options can help you set the coloring to your preference.-->

Git kann für die Textanzeige im Terminal Farben benutzen, die Dir helfen können, die Ausgabe schnell und einfach zu begreifen. Mit einer Vielzahl von Optionen kannst Du die Farben an Deine Vorlieben anpassen.

<!--#### color.ui ####-->
#### color.ui ####

<!--Git automatically colors most of its output if you ask it to. You can get very specific about what you want colored and how; but to turn on all the default terminal coloring, set `color.ui` to true:-->

Wenn Du Git entsprechend konfigurierst, wird es den Großteil der Ausgaben automatisch farblich darstellen. Du kannst sehr detailliert einstellen, wie und welche Farben verwendet werden sollen, aber um die Standard-Terminalfarben zu aktivieren musst Du `color.ui` auf ‚true‘ setzen:

	$ git config --global color.ui true

<!--When that value is set, Git colors its output if the output goes to a terminal. Other possible settings are false, which never colors the output, and always, which sets colors all the time, even if you’re redirecting Git commands to a file or piping them to another command.-->

Wenn dieser Wert gesetzt wurde, benutzt Git für seine Ausgaben Farben, sofern diese zu einem Terminal geleitet werden. Weitere mögliche Einstellungen sind ‚false‘, wodurch alle Farben deaktiviert werden, sowie ‚always‘, wodurch Farben immer aktiviert sind, selbst wenn Du Git Befehle in eine Datei oder über eine Pipe zu einem anderen Befehl umleitest.

<!--You’ll rarely want `color.ui = always`. In most scenarios, if you want color codes in your redirected output, you can instead pass a `-\-color` flag to the Git command to force it to use color codes. The `color.ui = true` setting is almost always what you’ll want to use.-->

Du wirst selten die Einstellung `color.ui = always` benötigen. In den meisten Fällen in denen Du in Deiner umgeleiteten Ausgabe Farben haben willst, kannst Du stattdessen die Option `--color` in der Kommandozeile benutzen. Damit weist Du Git an, die Farbkodierung für die Ausgabe zu verwenden. Die Einstellung `color.ui = true` sollte aber in den meisten Fällen Deinen Anforderungen genügen.

<!--#### `color.*` ####-->
#### `color.*` ####

<!--If you want to be more specific about which commands are colored and how, Git provides verb-specific coloring settings. Each of these can be set to `true`, `false`, or `always`:-->

Falls Du im Detail einstellen willst, welche Befehle wie gefärbt werden, dann stellt Git Verb-spezifische Farbeinstellungen zur Verfügung. Jede dieser Optionen kann auf `true`, `false`, oder `always` eingestellt werden:

	color.branch
	color.diff
	color.interactive
	color.status

<!--In addition, each of these has subsettings you can use to set specific colors for parts of the output, if you want to override each color. For example, to set the meta information in your diff output to blue foreground, black background, and bold text, you can run-->

Zusätzlich hat jede dieser Einstellungen Unteroptionen, die Du benutzen kannst, um die Farbe für einzelne Teile der Ausgabe festzulegen. Um zum Beispiel die Meta Informationen in Deiner Diff Ausgabe mit blauem, fettem Text auf schwarzem Hintergrund darzustellen, kannst Du folgenden Befehl verwenden:

	$ git config --global color.diff.meta "blue black bold"

<!--You can set the color to any of the following values: normal, black, red, green, yellow, blue, magenta, cyan, or white. If you want an attribute like bold in the previous example, you can choose from bold, dim, ul, blink, and reverse.-->

Du kannst als Farben jeden der folgenden Werte verwenden: `normal`, `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, oder `white`. Falls Du ein Attribut wie z.B. die Fettschrift aus dem vorigen Beispiel verwenden willst, stehen Dir folgende Werte zur Auswahl: `bold`, `dim`, `ul`, `blink`, und `reverse`.

<!--See the `git config` manpage for all the subsettings you can configure, if you want to do that.-->

Auf der Manpage zu `git config` findest Du eine Liste aller Unteroptionen, die Du konfigurieren kannst.

<!--### External Merge and Diff Tools ###-->
### Externe Merge- und Diff-Werzeuge ###

<!--Although Git has an internal implementation of diff, which is what you’ve been using, you can set up an external tool instead. You can also set up a graphical merge conflict-resolution tool instead of having to resolve conflicts manually. I’ll demonstrate setting up the Perforce Visual Merge Tool (P4Merge) to do your diffs and merge resolutions, because it’s a nice graphical tool and it’s free.-->

Bisher hast Du die in Git integrierte Implementierung von diff benutzt, aber Du kannst stattdessen auch eine externe Anwendung verwenden. Du kannst ebenso ein grafisches Merge-Werkzeug zur Auflösung von Konflikten einsetzen, statt diese manuell zu lösen. Ich werde demonstrieren, wie man das grafische Merge-Werkzeug von Perforce (P4Merge) konfiguriert, um Diffs und Merges zu bearbeiten. Ich habe P4Merge gewählt, da es ein freies und gutes grafisches Werkzeug ist.

<!--If you want to try this out, P4Merge works on all major platforms, so you should be able to do so. I’ll use path names in the examples that work on Mac and Linux systems; for Windows, you’ll have to change `/usr/local/bin` to an executable path in your environment.-->

Da P4Merge für die üblichen Plattformen verfügbar ist, sollte es kein Problem sein, es einmal auszuprobieren. In den Beispielen werde ich Pfadnamen nutzen, die auf Mac- und Linux-System funktionieren. Die Windows Benutzer müssen `/usr/local/bin` durch einen Pfad ersetzen, der in der Umgebungsvariable `PATH` gelistet ist.

<!--You can download P4Merge here:-->

Du kannst P4Merge hier herunterladen:

	http://www.perforce.com/product/components/perforce-visual-merge-and-diff-tools

<!--To begin, you’ll set up external wrapper scripts to run your commands. I’ll use the Mac path for the executable; in other systems, it will be where your `p4merge` binary is installed. Set up a merge wrapper script named `extMerge` that calls your binary with all the arguments provided:-->

Als erstes solltest Du einige Wrapper Skripte erstellen um Deine Befehle auszuführen. Ich verwende hier die Pfade, die für einen Mac gelten. Auf anderen Systemen muss der Pfad zur ausführbaren Datei von P4Merge entsprechend angepasst werden. Mit den folgenden Befehlen erzeugen wir ein Skript mit dem Namen `extMerge`, welches die Anwendung mit allen angegebenen Argumenten aufruft:

	$ cat /usr/local/bin/extMerge
	#!/bin/sh
	/Applications/p4merge.app/Contents/MacOS/p4merge $*

<!--The diff wrapper checks to make sure seven arguments are provided and passes two of them to your merge script. By default, Git passes the following arguments to the diff program:-->

Das Wrapper Skript für den Diff Befehl stellt sicher, dass es mit sieben Parametern aufgerufen wird und leitet zwei von diesen an das Merge Skript weiter. Standardmäßig übergibt Git die folgenden Argumente an das Diff-Werkzeug:

	path old-file old-hex old-mode new-file new-hex new-mode

<!--Because you only want the `old-file` and `new-file` arguments, you use the wrapper script to pass the ones you need.-->

Da nur die Parameter `old-file` und `new-file` benötigt werden, verwenden wir das Wrapper Skript um nur die notwendigen Parameter weiterzugeben.

	$ cat /usr/local/bin/extDiff
	#!/bin/sh
	[ $# -eq 7 ] && /usr/local/bin/extMerge "$2" "$5"

<!--You also need to make sure these tools are executable:-->

Außerdem muss sichergestellt werden, dass die Skripte ausführbar sind::

	$ sudo chmod +x /usr/local/bin/extMerge
	$ sudo chmod +x /usr/local/bin/extDiff

<!--Now you can set up your config file to use your custom merge resolution and diff tools. This takes a number of custom settings: `merge.tool` to tell Git what strategy to use, `mergetool.*.cmd` to specify how to run the command, `mergetool.trustExitCode` to tell Git if the exit code of that program indicates a successful merge resolution or not, and `diff.external` to tell Git what command to run for diffs. So, you can either run four config commands-->

Jetzt kannst Du Git so konfigurieren, dass es Deine persönlichen Merge- und Diff-Werkzeuge benutzt. Dazu sind einige weitere Einstellungen nötig: `merge.tool`, um die von Git verwendete Merge Strategie festzulegen, `mergetool.*.cmd`, um festzulegen, wie der Befehl auszuführen ist, `mergetool.trustExitCode`, damit Git weiß, ob der Exit-Code des Programms eine erfolgreiche Merge Auflösung anzeigt oder nicht, und `diff.external`, um einzustellen welches Diff Kommando Git benutzen soll. Du kannst also entweder die vier folgenden Befehle ausführen

	$ git config --global merge.tool extMerge
	$ git config --global mergetool.extMerge.cmd \
	    'extMerge "$BASE" "$LOCAL" "$REMOTE" "$MERGED"'
	$ git config --global mergetool.trustExitCode false
	$ git config --global diff.external extDiff

<!--or you can edit your `~/.gitconfig` file to add these lines:-->

oder Du bearbeitest Deine `~/.gitconfig` Datei und fügst dort folgende Zeilen hinzu:

	[merge]
	  tool = extMerge
	[mergetool "extMerge"]
	  cmd = extMerge \"$BASE\" \"$LOCAL\" \"$REMOTE\" \"$MERGED\"
	  trustExitCode = false
	[diff]
	  external = extDiff

<!--After all this is set, if you run diff commands such as this:-->

Nach Setzen dieser Einstellungen und beim Ausführen eines Diff Befehls wie den folgenden:

	$ git diff 32d1776b1^ 32d1776b1

<!--Instead of getting the diff output on the command line, Git fires up P4Merge, which looks something like Figure 7-1.-->

wird Git P4Merge starten, anstatt den Vergleich in der Kommandozeile auszugeben. Abbildung 7-1 zeigt hierzu ein Beispiel.

<!--Figure 7-1. P4Merge.-->

Insert 18333fig0701.png
Abbildung 7-1. P4Merge

<!--If you try to merge two branches and subsequently have merge conflicts, you can run the command `git mergetool`; it starts P4Merge to let you resolve the conflicts through that GUI tool.-->

Wenn Du versuchst zwei Branches zu mergen und dabei Merge Konflikte auftreten, kannst Du den Befehl `git mergetool` ausführen. Das Kommando startet P4Merge und erlaubt es Dir, die Konflikte mit Hilfe des grafischen Werkzeugs aufzulösen.

<!--The nice thing about this wrapper setup is that you can change your diff and merge tools easily. For example, to change your `extDiff` and `extMerge` tools to run the KDiff3 tool instead, all you have to do is edit your `extMerge` file:-->

Das Tolle an dem Wrapper Ansatz ist, dass Du Deine Diff- und Merge-Werkzeuge sehr leicht wechseln kannst. Wenn Du zum Beispiel für `extDiff` und `extMerge` statt P4Merge, KDiff3 verwenden willst, musst Du lediglich Dein Wrapper Skript `extMerge` anpassen:

	$ cat /usr/local/bin/extMerge
	#!/bin/sh
	/Applications/kdiff3.app/Contents/MacOS/kdiff3 $*

<!--Now, Git will use the KDiff3 tool for diff viewing and merge conflict resolution.-->

Ab jetzt verwendet Git KDiff3 zur Anzeige von Diffs und zur Auflösung von Merge Konflikten.

<!--Git comes preset to use a number of other merge-resolution tools without your having to set up the cmd configuration. You can set your merge tool to kdiff3, opendiff, tkdiff, meld, xxdiff, emerge, vimdiff, or gvimdiff. If you’re not interested in using KDiff3 for diff but rather want to use it just for merge resolution, and the kdiff3 command is in your path, then you can run-->

Git wird bereits mit Standard-Einstellungen für verschiedene Merge-Auflösungswerkzeuge ausgeliefert, sodass Du diese nicht extra konfigurieren musst. Als Merge-Werkzeug kann Du kdiff3, opendiff, tkdiff, meld, xxdiff, emerge, vimdiff oder gvimdiff einstellen. Wenn Du KDiff3 nur zum Auflösen von Konflikten und nicht für einen Diff verwenden willst, kannst Du den folgenden Befehl ausführen (vorausgesetzt KDiff3 befindet sich im Standard-Pfad):

	$ git config --global merge.tool kdiff3

<!--If you run this instead of setting up the `extMerge` and `extDiff` files, Git will use KDiff3 for merge resolution and the normal Git diff tool for diffs.-->

Wenn Du diesen Befehl ausführst, anstatt die `extMerge` und `extDiff` Skripte zu erstellen, dann wird Git KDiff3 zum Auflösen von Merge Konflikten verwenden. Für einen Vergleich verwendet Git weiterhin das integrierte Diff-Werkzeug.

<!--### Formatting and Whitespace ###-->
### Formatierungen und Leerzeichen ###

<!--Formatting and whitespace issues are some of the more frustrating and subtle problems that many developers encounter when collaborating, especially cross-platform. It’s very easy for patches or other collaborated work to introduce subtle whitespace changes because editors silently introduce them or Windows programmers add carriage returns at the end of lines they touch in cross-platform projects. Git has a few configuration options to help with these issues.-->

Bei der Zusammenarbeit mit anderen Entwicklern sind Probleme mit Formatierungen und Leerzeichen einige der frustrierendsten und heikelsten Themen denen viele Entwickler begegnen, vor allem bei plattformübergreifenden Projekten. Es kann sehr leicht passieren, dass durch Patches oder andere gemeinsame Arbeit fast unmerklich Leerzeichen Änderungen eingeführt werden, z.B. weil ein Editor sie stillschweigend einfügt. Beim Programmieren unter Windows können durch Änderungen an einer Zeile auch leicht Wagenrückläufe (CR) am Zeilenende eingefügt werden (relevant bei plattformübergreifenden Projekten). Git kann mit ein paar Einstellungen hierbei unterstützend eingreifen.

<!--#### core.autocrlf ####-->
#### core.autocrlf ####

<!--If you’re programming on Windows or using another system but working with people who are programming on Windows, you’ll probably run into line-ending issues at some point. This is because Windows uses both a carriage-return character and a linefeed character for newlines in its files, whereas Mac and Linux systems use only the linefeed character. This is a subtle but incredibly annoying fact of cross-platform work.-->

Falls Du unter Windows programmierst oder ein anderes System benutzt und mit anderen zusammenarbeitest, die unter Windows programmieren, wirst Du sehr wahrscheinlich irgendwann Problemen mit Zeilenenden begegnen. Dies liegt daran, dass Windows sowohl ein CR Zeichen, als auch ein LF Zeichen zum Signalisieren einer neuen Zeile in Dateien verwendet. Mac und Linux nutzen stattdessen nur ein LF Zeichen (Mac OS bis Version 9 verwendet ein einzelnes CR Zeichen). Dies ist eine kleine, aber extrem störende Tatsache beim Arbeiten über Plattformgrenzen hinweg.

<!--Git can handle this by auto-converting CRLF line endings into LF when you commit, and vice versa when it checks out code onto your filesystem. You can turn on this functionality with the `core.autocrlf` setting. If you’re on a Windows machine, set it to `true` — this converts LF endings into CRLF when you check out code:-->

Git kann dies vermeiden, indem es CRLF am Zeilenende automatisch zu LF konvertiert, wenn Du ein Commit durchführst, und umgekehrt wenn es Code in Dein lokales Dateisystem auscheckt. Du kannst diese Funktionalität mittels der Option `core.autocrlf` aktivieren. Falls Du auf einem Windows System arbeitest, setze sie auf `true` — dies konvertiert LF zu CRLF, wenn Du Code auscheckst:

	$ git config --global core.autocrlf true

<!--If you’re on a Linux or Mac system that uses LF line endings, then you don’t want Git to automatically convert them when you check out files; however, if a file with CRLF endings accidentally gets introduced, then you may want Git to fix it. You can tell Git to convert CRLF to LF on commit but not the other way around by setting `core.autocrlf` to input:-->

Falls Du auf einem Linux oder Mac System arbeitest, welches LF Zeilenenden verwendet, dann soll Git keine Datei automatisch konvertieren, wenn sie ausgecheckt wird. Wenn allerdings versehentlich eine Datei mit CRLF in das Repository eingeführt wurde, dann möchtest Du vielleicht, dass Git dies automatisch für Dich repariert. Wenn Du den Parameter `core.autocrlf` auf input setzt, wird Git bei einem Commit automatisch CRLF in LF umwandeln. Allerdings nicht in die andere Richtung bei einem Checkout:

	$ git config --global core.autocrlf input

<!--This setup should leave you with CRLF endings in Windows checkouts but LF endings on Mac and Linux systems and in the repository.-->

Mit dieser Einstellung solltest Du CRLF Zeilenenden in unter Windows ausgecheckten Dateien haben und LF Zeilenenden auf Mac und Linux Sytemen und im Repository.

<!--If you’re a Windows programmer doing a Windows-only project, then you can turn off this functionality, recording the carriage returns in the repository by setting the config value to `false`:-->

Falls Du ein Windows Programmierer bist und an einem Projekt arbeitest, welches nur unter Windows entwickelt wird, dann kannst Du diese Funktionalität auch deaktivieren. In diesem Fall werden Zeilenenden mit CRLF im Repository gespeichert. Dazu setzt Du die Option auf `false`:

	$ git config --global core.autocrlf false

<!--#### core.whitespace ####-->
#### core.whitespace ####

<!--Git comes preset to detect and fix some whitespace issues. It can look for four primary whitespace issues — two are enabled by default and can be turned off, and two aren’t enabled by default but can be activated.-->

Git ist so voreingestellt, dass es einige Leerzeichen Probleme erkennen und beheben kann. Es kann nach vier grundlegenden Problemen mit Leerzeichen suchen — Zwei davon sind standardmässig aktiviert und können deaktiviert werden. Die anderen beiden sind inaktiv, können aber aktiviert werden.

<!--The two that are turned on by default are `trailing-space`, which looks for spaces at the end of a line, and `space-before-tab`, which looks for spaces before tabs at the beginning of a line.-->

Die zwei standardmäßig aktiven Optionen sind `trailing-space`, das nach Leerzeichen am Ende einer Zeile sucht, und `space-before-tab`, das nach Leerzeichen vor Tabulatoren am Anfang einer Zeile sucht.

<!--The two that are disabled by default but can be turned on are `indent-with-non-tab`, which looks for lines that begin with eight or more spaces instead of tabs, and `cr-at-eol`, which tells Git that carriage returns at the end of lines are OK.-->

Die beiden aktivierbaren, aber normalerweise deaktivierten Optionen sind `indent-with-non-tab`, welches nach Zeilen sucht, die mit acht oder mehr Leerzeichen anstelle von Tabulatoren beginnen, und `cr-at-eol`, wodurch Git angewiesen wird, dass CR Zeichen am Zeilenende in Ordnung sind.

<!--You can tell Git which of these you want enabled by setting `core.whitespace` to the values you want on or off, separated by commas. You can disable settings by either leaving them out of the setting string or prepending a `-` in front of the value. For example, if you want all but `cr-at-eol` to be set, you can do this:-->

Du kannst Git mitteilen, welche dieser Optionen es aktivieren soll, indem Du `core.whitespace` auf die Werte setzt, die Du an- oder abgeschaltet haben möchtest. Die jeweiligen Werte werden mit einem Komma getrennt. Du kannst Optionen deaktivieren, indem Du sie entweder aus der Parameterliste entfernst, oder ihnen ein `-` Zeichen voranstellst. Wenn Du zum Beispiel alle Optionen außer `cr-at-eol` aktivieren willst, kannst Du folgenden Befehl ausführen:

	$ git config --global core.whitespace \
	    trailing-space,space-before-tab,indent-with-non-tab

<!--Git will detect these issues when you run a `git diff` command and try to color them so you can possibly fix them before you commit. It will also use these values to help you when you apply patches with `git apply`. When you’re applying patches, you can ask Git to warn you if it’s applying patches with the specified whitespace issues:-->

Git wird die möglichen Problemstellen erkennen, wenn Du den `git diff` Befehl ausführst, und es wird versuchen, sie farblich hervorzuheben, damit Du sie vor einem Commit beheben kannst. Git wird diese Einstellungen auch benutzen, um Dir zu helfen, wenn Du mit `git apply` Patches anwendest. Wenn Du Patches anwendest, kannst Du Git anweisen eine Warnung auszugeben, falls es beim Patchen die spezifizierten Leerzeichenprobleme erkennt:

	$ git apply --whitespace=warn <patch>

<!--Or you can have Git try to automatically fix the issue before applying the patch:-->

Oder Du kannst Git versuchen lassen, diese Probleme automatisch zu beheben, bevor es den Patch anwendet:

	$ git apply --whitespace=fix <patch>

<!--These options apply to the `git rebase` command as well. If you’ve committed whitespace issues but haven’t yet pushed upstream, you can run a `rebase` with the `-\-whitespace=fix` option to have Git automatically fix whitespace issues as it’s rewriting the patches.-->

Diese Optionen gelten auch für den Rebase Befehl. Falls Du einen Commit gemacht hast, der problematische Leerzeichen enthält, aber Du die Änderungen noch nicht auf den Server gepusht hast, kannst Du ein `rebase` mit dem Parameter `--whitespace=fix` ausführen. Damit behebt Git automatisch die Leerzeichenfehler während des Rebase-Vorgangs.

<!--### Server Configuration ###-->
### Server Konfiguration ###

<!--Not nearly as many configuration options are available for the server side of Git, but there are a few interesting ones you may want to take note of.-->

Es gibt nicht annähernd so viele Konfigurationsmöglichkeiten für die Serverfunktionalitäten von Git, aber es gibt dabei einige interessante Parameter, die Du Dir anschauen solltest.

<!--#### receive.fsckObjects ####-->
#### receive.fsckObjects ####

<!--By default, Git doesn’t check for consistency all the objects it receives during a push. Although Git can check to make sure each object still matches its SHA-1 checksum and points to valid objects, it doesn’t do that by default on every push. This is a relatively expensive operation and may add a lot of time to each push, depending on the size of the repository or the push. If you want Git to check object consistency on every push, you can force it to do so by setting `receive.fsckObjects` to true:-->

Die Objekte, die Git durch einen Push empfängt, werden von Haus aus nicht auf Konsistenz geprüft. Auch wenn Git sicherstellen kann, dass jedes Objekt mit dessen SHA-1 Checksumme übereinstimmt und auf gültige Objekte verweist, so wird dies standardmäßig nicht bei jedem Push durchgeführt. Das ist eine aufwändige Operation und kann abhängig von der Größe des Repositorys oder dem Push eine Menge Zeit kosten. Wenn Du die Objektkonsistenz bei jedem Push durch Git prüfen lassen willst, so kannst Du das erzwingen, indem Du `receive.fsckObjects` auf ‚true‘ setzt:

	$ git config --system receive.fsckObjects true

<!--Now, Git will check the integrity of your repository before each push is accepted to make sure faulty clients aren’t introducing corrupt data.-->

Ab jetzt prüft Git die Integrität des Repositorys bevor der Push akzeptiert wird. Damit ist sichergestellt, dass kein Client korrupte Daten einspeist.

<!--#### receive.denyNonFastForwards ####-->
#### receive.denyNonFastForwards ####

<!--If you rebase commits that you’ve already pushed and then try to push again, or otherwise try to push a commit to a remote branch that doesn’t contain the commit that the remote branch currently points to, you’ll be denied. This is generally good policy; but in the case of the rebase, you may determine that you know what you’re doing and can force-update the remote branch with a `-f` flag to your push command.-->

Falls Du auf Commits, die bereits gepusht sind, einen Rebase anwendest, und diese dann versuchst zu pushen, wird Git dies mit einer Fehlermeldung zurückweisen. Wenn der Remote Branch auf einen Commit zeigt, welcher nicht in Deinem lokalen Branch enthalten ist und Du versuchst diesen Branch zu pushen, wird sich Git genau gleich verhalten und den Push verweigern. Das ist in den meisten Fällen eine gute Richtlinie, aber im Falle eines Rebase ist eventuell ein anderes Verhalten gewünscht (vorausgesetzt Du weißt was Du tust). Dann kannst Du den Push auch erzwingen, indem Du den Parameter `-f` zu dem Push Kommando hinzufügst.

<!--To disable the ability to force-update remote branches to non-fast-forward references, set `receive.denyNonFastForwards`:-->

Aktualisierungen auf dem Remote Branch, welche nicht einem Fast-Forward entsprechen können durch Setzen des Parameters `receive.denyNonFastForward` auf den Wert ‚true‘ deaktiviert werden:

	$ git config --system receive.denyNonFastForwards true

<!--The other way you can do this is via server-side receive hooks, which I’ll cover in a bit. That approach lets you do more complex things like deny non-fast-forwards to a certain subset of users.-->

Eine andere Möglichkeit ist die Einrichtung von serverseitigen Hooks, die ich etwas später noch beschreiben werde. Dieser Ansatz erlaubt noch komplexere Szenarien. Man kann z.B. die Pushes, welche nicht einem Fast-Forward entsprechen nur für bestimmte Benutzergruppen verweigern.

<!--#### receive.denyDeletes ####-->
#### receive.denyDeletes ####

<!--One of the workarounds to the `denyNonFastForwards` policy is for the user to delete the branch and then push it back up with the new reference. In newer versions of Git (beginning with version 1.6.1), you can set `receive.denyDeletes` to true:-->

Es ist möglich die Option `denyNonFastForwards` zu umgehen, indem man den Remote Branch zuerst löscht und dann mit einer neuen Referenz pusht. In neueren Versionen von Git (ab Version 1.6.1) kann man den Parameter  `receive.denyDeletes` auf ‚true‘ setzen:

	$ git config --system receive.denyDeletes true

<!--This denies branch and tag deletion over a push across the board — no user can do it. To remove remote branches, you must remove the ref files from the server manually. There are also more interesting ways to do this on a per-user basis via ACLs, as you’ll learn at the end of this chapter.-->

Dies verbietet grundsätzlich jedem Benutzer das Löschen eines Branches oder Tags. Um einen Remote Branch zu löschen müssen die ref Dateien manuell vom Server entfernt werden. Es gibt aber auch noch andere interessantere Wege dies auf Benutzerbasis über Zugriffssteuerungslisten (ACL) durchzuführen. Ich werde dies am Ende dieses Kapitel noch vorstellen.

<!--## Git Attributes ##-->
## Git Attribute ##

<!--Some of these settings can also be specified for a path, so that Git applies those settings only for a subdirectory or subset of files. These path-specific settings are called Git attributes and are set either in a `.gitattributes` file in one of your directories (normally the root of your project) or in the `.git/info/attributes` file if you don’t want the attributes file committed with your project.-->

Einige dieser Einstellungen können auch auf einen Pfad beschränkt werden, sodass sie nur für bestimmte Unterverzeichnisse oder eine Gruppe von Dateien gültig sind. Diese Einstellungen werden Git Attribute genannt und werden in der Datei `.gitattributes` in einem der Projektverzeichnisse verwaltet (üblicherweise im Root-Verzeichnis Deines Projekts). Alternativ kannst Du diese auch unter `.git/info/attributes` ablegen. In diesem Fall werden die Attribute nicht in das Repository eingecheckt und gelten nur für dieses einzelne, lokale Repository.

<!--Using attributes, you can do things like specify separate merge strategies for individual files or directories in your project, tell Git how to diff non-text files, or have Git filter content before you check it into or out of Git. In this section, you’ll learn about some of the attributes you can set on your paths in your Git project and see a few examples of using this feature in practice.-->

Mittels den Attributen ist es zum Beispiel möglich, verschiedene Merge Strategien für einzelne Dateien oder Verzeichnisse innerhalb Deines Projekts vorzugeben. Ebenso kannst Du Git anweisen, wie ein Vergleich von Binärdateien durchzuführen ist. Oder Du konfigurierst Git so, dass der Inhalt von Dateien vorgefiltert wird, wenn Du ein Commit oder Checkout durchführst. In diesem Abschnitt wirst Du einiger der Attribute kennenlernen, die Du für die einzelnen Verzeichnisse in Deinem Git Projekt vorgeben kannst. Außerdem werde ich einige Beispiele aus der Praxis näher erläutern.

<!--### Binary Files ###-->
### Binärdateien ###

<!--One cool trick for which you can use Git attributes is telling Git which files are binary (in cases it otherwise may not be able to figure out) and giving Git special instructions about how to handle those files. For instance, some text files may be machine generated and not diffable, whereas some binary files can be diffed — you’ll see how to tell Git which is which.-->

Mit Hilfe der Git Attribute ist es Dir möglich, Git mitzuteilen, welche Dateien binär sind (für den Fall, dass Git nicht in der Lage ist, dies selbst feszustellen) und wie Git diese behandeln soll. Es kann zum Beispiel sein, dass automatisiert, erstellte Textdateien nicht einfach verglichen werden können. Oder umgekehrt können manche Binärdateien leicht von einem Menschen verglichen werden. Ich werde jetzt aufzeigen, wie Du Git konfigurierst damit es solche Dateien unterscheiden kann.

<!--#### Identifying Binary Files ####-->
#### Binärdateien erkennen ####

<!--Some files look like text files but for all intents and purposes are to be treated as binary data. For instance, Xcode projects on the Mac contain a file that ends in `.pbxproj`, which is basically a JSON (plain text javascript data format) dataset written out to disk by the IDE that records your build settings and so on. Although it’s technically a text file, because it’s all ASCII, you don’t want to treat it as such because it’s really a lightweight database — you can’t merge the contents if two people changed it, and diffs generally aren’t helpful. The file is meant to be consumed by a machine. In essence, you want to treat it like a binary file.-->

Manche Dateien sehen zwar wie Textdateien aus, sollten aber streng genommen als Binärdateien behandelt werden. So enthalten zum Beispiel Xcode Projekte auf dem Mac eine Datei mit der Endung `.pbxproj`. Die Datei ist eigentlich nur ein JSON-Datensatz (ein Klartext Javascript Datenformat), der von der IDE gespeichert wird und unter anderem die Build Einstellungen enthält. Obwohl sie nur ASCII Zeichen enthält und damit technisch gesehen eine Textdatei ist, sollte man diese nicht als solche behandeln. In Wirklichkeit ist diese Datei eine kleine Datenbank, deren Inhalt nicht zusammengeführt werden kann, wenn zwei Leute sie geändert haben. Das Vergleichen der Datei ist ebenso selten hilfreich. Die Datei ist für die Verarbeitung durch einen Computer gedacht. Kurz gesagt, Du willst, dass man sie als Binärdatei behandelt.

<!--To tell Git to treat all `pbxproj` files as binary data, add the following line to your `.gitattributes` file:-->

Um Git anzuweisen alle `pbxproj` Dateien als Binärdateien zu behandeln, kannst Du die folgende Zeile zu Deiner `.gitattributes` Datei hinzufügen:

	*.pbxproj -crlf -diff

<!--Now, Git won’t try to convert or fix CRLF issues; nor will it try to compute or print a diff for changes in this file when you run `git show` or `git diff` on your project. You can also use a built-in macro `binary` that means `-crlf -diff`:-->

Ab jetzt wird Git nicht mehr versuchen CRLF Probleme zu lösen oder die Datei beim Commit oder Checkout zu ändern. Außerdem ermittelt Git keine Dateiunterschiede mehr und gibt diese auch nicht aus, wenn Du den Befehl `git show` oder `git diff` ausführst. Alternativ gibt es auch ein integriertes Makro `binary`, welches den Parametern `-crlf -diff` entspricht:

	*.pbxproj binary

<!--#### Diffing Binary Files ####-->
#### Diff bei Binärdateien ####

<!--In Git, you can use the attributes functionality to effectively diff binary files. You do this by telling Git how to convert your binary data to a text format that can be compared via the normal diff. But the question is how do you convert *binary* data to a text? The best solution is to find some tool that does conversion for your binary format to a text representation. Unfortunately, very few binary formats can be represented as human readable text (imagine trying to convert audio data to a text). If this is the case and you failed to get a text presentation of your file's contents, it's often relatively easy to get a human readable description of that content, or metadata. Metadata won't give you a full representation of your file's content, but in any case it's better than nothing.-->

Mit Hilfe der Git Attribute können Unterschiede in binären Dateien effektiv und leicht angezeigt werden. Du kannst Git so konfigurieren, dass es automatisch Binärdateien in Textdateien umwandelt, damit sie mit einem normalen Diff verglichen werden können. Meist stellt sich aber die Frage, wie man binäre Daten in Text konvertieren soll. Wenn man ein Werkzeug findet, welches einem diese Konvertierung abnimmt und die binäre Daten in ein Textformat umwandelt, ist dies meist die beste Lösung. Leider gibt es nur sehr wenige binäre Formate, die sich dafür eignen, dass man sie in lesbare Textformate umwandelt (Ich denke dabei zum Beispiel an Audio-Daten). Wenn dies der Fall ist und Du keine geeignete Möglichkeit gefunden hast, die Daten in lesbare Form zu wandeln, dann ist es oft relativ einfach eine entsprechende Beschreibung des eigentlichen Inhalts zu erhalten. Alternativ gibt es noch Metadaten, wobei Metadaten einem nicht ein vollständiges Abbild vom Dateiinhalt liefern können, aber in diesem Fall ist das besser als gar nichts.

<!--We'll make use of the both described approaches to get usable diffs for some widely used binary formats.-->

Im folgenden Abschnitt werden wir beide Möglichkeiten besprechen, wie man für weit verbreitete binäre Formate eine lesbare Form für einen Vergleich erhält.

<!--Side note: There are different kinds of binary formats with a text content, which are hard to find usable converter for. In such a case you could try to extract a text from your file with the `strings` program. Some of these files may use an UTF-16 encoding or other "codepages" and `strings` won’t find anything useful in there. Your mileage may vary. However, `strings` is available on most Mac and Linux systems, so it may be a good first try to do this with many binary formats.-->

Anmerkung: Es gibt verschiedene Arten von binären Formaten, welche Text beinhalten, und es ist meist sehr schwierig einen passenden Konverter zu finden. In solchen Fällen kann man sein Glück mit dem `strings`-Programm versuchen. Manche der Formate verwenden ein UTF-16 Encoding oder andere Zeichentabellen. In diesem Fall wird es mit dem Programm `strings` meist nicht funktionieren. Da `strings` jedoch auf den meisten Mac- und Linux-Systemen verfügbar ist, sollte man es durchaus auf einen Versuch ankommen lassen.

<!--##### MS Word files #####-->
##### MS Word files #####

<!--First, you’ll use the described technique to solve one of the most annoying problems known to humanity: version-controlling Word documents. Everyone knows that Word is the most horrific editor around; but, oddly, everyone uses it. If you want to version-control Word documents, you can stick them in a Git repository and commit every once in a while; but what good does that do? If you run `git diff` normally, you only see something like this:-->

Als erstes werden wir die beschriebene Technik benutzen um eines der lästigsten Probleme der Menschheit zu lösen: Versionskontrolle von Word Dokumenten. Jeder weiß, dass Word der schrecklichste Editor der Welt ist, aber trotzdem benutzt ihn jeder. Wenn Du Word Dokumente versionieren willst, kannst Du sie in Dein Repository packen und ab und zu einen Commit durchführen. Aber wozu ist das nützlich? Wenn Du einen Vergleich mit `git diff` ausführst, erhälst Du ähnliche Ausgabe wie diese:

	$ git diff
	diff --git a/chapter1.doc b/chapter1.doc
	index 88839c4..4afcb7c 100644
	Binary files a/chapter1.doc and b/chapter1.doc differ

<!--You can’t directly compare two versions unless you check them out and scan them manually, right? It turns out you can do this fairly well using Git attributes. Put the following line in your `.gitattributes` file:-->

Du kannst zwei Versionen nicht direkt vergleichen, außer Du checkst sie aus und prüfst sie manuell, richtig? Es stellt sich heraus, dass dies recht gut mittels Git Attributen möglich ist. Füge dazu die folgende Zeile in Deine `.gitattributes` Datei ein:

	*.doc diff=word

<!--This tells Git that any file that matches this pattern (.doc) should use the "word" filter when you try to view a diff that contains changes. What is the "word" filter? You have to set it up. Here you’ll configure Git to use the `catdoc` program, which was written specifically for extracting text from a binary MS Word documents (you can get it from `http://www.wagner.pp.ru/~vitus/software/catdoc/`), to convert Word documents into readable text files, which it will then diff properly:-->

Dies weist Git an, dass auf jede Datei, die diesem Dateimuster (.doc) entspricht, der „word“ Filter angewandt werden soll, wenn Du versuchst, einen Diff mit Dateiunterschieden anzusehen. Was ist nun der „word“ Filter? Dieser muss von Dir noch konfiguriert werden. Du kannst Git so konfigurieren, dass es das `catdoc` Programm verwendet um Word Dokumente in lesbare Textdateien zu konvertieren. `catdoc` wurde speziell dafür entwickelt um lesbaren Text aus binären MS Word Dokumenten zu extrahieren (du erhälst es unter `http://www.wagner.pp.ru/~vitus/software/catdoc/`).   Bei jedem Diff wird Git diese Konvertierung durchführen:

	$ git config diff.word.textconv catdoc

<!--This command adds a section to your `.git/config` that looks like this:-->

Dieser Befehl fügt in der Datei `.git/config` eine Sektion mit folgendem Aufbau hinzu:

	[diff "word"]
		textconv = catdoc

<!--Now Git knows that if it tries to do a diff between two snapshots, and any of the files end in `.doc`, it should run those files through the "word" filter, which is defined as the `catdoc` program. This effectively makes nice text-based versions of your Word files before attempting to diff them.-->

Bei jedem Vergleich von zwei Schnappschüssen wird Git Dateien mit der Dateiendung `.doc` durch den „word“ Filter jagen, welcher durch das `catdoc` Programm definiert ist. Das erzeugt gut lesbare Textversionen Deiner Word Dateien, die für den Vergleich herangezogen werden.

<!--Here’s an example. I put Chapter 1 of this book into Git, added some text to a paragraph, and saved the document. Then, I ran `git diff` to see what changed:-->

Dazu ein Beispiel. Ich habe Kapitel 1 des Buches in ein Word-Dokument einfgefügt und in Git gespeichert. Danach habe ich etwas Text in einem Absatz geändert, die Datei gespeichert und den Befehl `git diff` ausgeführt um zu prüfen, was sich geändert hat:

	$ git diff
	diff --git a/chapter1.doc b/chapter1.doc
	index c1c8a0a..b93c9e4 100644
	--- a/chapter1.doc
	+++ b/chapter1.doc
	@@ -128,7 +128,7 @@ and data size)
	 Since its birth in 2005, Git has evolved and matured to be easy to use
	 and yet retain these initial qualities. It’s incredibly fast, it’s
	 very efficient with large projects, and it has an incredible branching
	-system for non-linear development.
	+system for non-linear development (See Chapter 3).

<!--Git successfully and succinctly tells me that I added the string "(See Chapter 3)", which is correct. Works perfectly!-->

Git war erfolgreich und zeigt nun kurz und bündig an, dass ich den Text „(See Chapter 3)“ hinzugefügt habe, was korrekt ist. Wie du siehst, funktioniert perfekt.

<!--##### OpenDocument Text files #####-->
##### OpenDocument Textdateien #####

<!--The same approach that we used for MS Word files (`*.doc`) can be used for OpenDocument Text files (`*.odt`) created by OpenOffice.org.-->

Bei OpenDocument Textdateien (`*.odt`), die mit OpenOffice erstellt wurden, können wir die gleiche Herangehensweise wie bei MS Word Dateien (`*.odt`) anwenden.

<!--Add the following line to your `.gitattributes` file:-->

Füge die folgende Zeile zu der `.gitattributes` Datei hinzu:

	*.odt diff=odt

<!--Now set up the `odt` diff filter in `.git/config`:-->

Jetzt müssen wir noch den `odt` Diff Filter in der `.git/config` hinzufügen:

	[diff "odt"]
		binary = true
		textconv = /usr/local/bin/odt-to-txt

<!--OpenDocument files are actually zip’ped directories containing multiple files (the content in an XML format, stylesheets, images, etc.). We’ll need to write a script to extract the content and return it as plain text. Create a file `/usr/local/bin/odt-to-txt` (you are free to put it into a different directory) with the following content:-->

OpenDocument Dateien sind eigentlich komprimierte Zip Verzeichnisse, die mehrere Dateien enthalten (der Inhalt: XML-Dateien, Stylesheets, Bilder, usw.). Wir müssen ein Skript schreiben um den Inhalt zu extrahieren und das Ergebnis als reinen Text zurückliefern. Erzeuge dazu eine Datei `/usr/local/bin/odt-to-txt` (die Datei kann in einem beliebigen Verzeichnis abgelegt werden) mit dem folgenden Inhalt:

	#! /usr/bin/env perl
	# Simplistic OpenDocument Text (.odt) to plain text converter.
	# Author: Philipp Kempgen

	if (! defined($ARGV[0])) {
		print STDERR "No filename given!\n";
		print STDERR "Usage: $0 filename\n";
		exit 1;
	}

	my $content = '';
	open my $fh, '-|', 'unzip', '-qq', '-p', $ARGV[0], 'content.xml' or die $!;
	{
		local $/ = undef;  # slurp mode
		$content = <$fh>;
	}
	close $fh;
	$_ = $content;
	s/<text:span\b[^>]*>//g;           # remove spans
	s/<text:h\b[^>]*>/\n\n*****  /g;   # headers
	s/<text:list-item\b[^>]*>\s*<text:p\b[^>]*>/\n    --  /g;  # list items
	s/<text:list\b[^>]*>/\n\n/g;       # lists
	s/<text:p\b[^>]*>/\n  /g;          # paragraphs
	s/<[^>]+>//g;                      # remove all XML tags
	s/\n{2,}/\n\n/g;                   # remove multiple blank lines
	s/\A\n+//;                         # remove leading blank lines
	print "\n", $_, "\n\n";

<!--And make it executable-->

Nun musst Du diese Datei noch ausführbar machen:

	chmod +x /usr/local/bin/odt-to-txt

<!--Now `git diff` will be able to tell you what changed in `.odt` files.-->

Jetzt kann Dir `git diff` aufzeigen, was sich in `.odt` Dateien geändert hat.

<!--##### Image files #####-->
##### Bilddateien #####

<!--Another interesting problem you can solve this way involves diffing image files. One way to do this is to run PNG files through a filter that extracts their EXIF information — metadata that is recorded with most image formats. If you download and install the `exiftool` program, you can use it to convert your images into text about the metadata, so at least the diff will show you a textual representation of any changes that happened:-->

Auf diese Art und Weise kann man ein weiteres, interessantes Problem lösen. Das Vergleichen von Bilddateien. Eine Möglichkeit dies zu tun, ist es, JPEG Dateien durch einen Filter zu schicken, der ihre EXIF Bildinformationen extrahiert. EXIF Bildinformationen sind Metadaten, die den meisten Bilddateien beigefügt werden. Wenn Du das Programm `exiftool` herunterlädst und installierst, kannst Du es benutzen um Deine Bilder in einen Text mit diesen Metainformationen umzuwandeln. Damit kann Dir ein Diff zumindest eine textuelle Repräsentation aller Veränderungen an der Datei anzeigen:

	$ echo '*.png diff=exif' >> .gitattributes
	$ git config diff.exif.textconv exiftool

<!--If you replace an image in your project and run `git diff`, you see something like this:-->

Wenn Du nun ein Bild in Deinem Projekt ersetzt und `git diff` ausführst, erhälst Du in etwa folgende Ausgabe:

	diff --git a/image.png b/image.png
	index 88839c4..4afcb7c 100644
	--- a/image.png
	+++ b/image.png
	@@ -1,12 +1,12 @@
	 ExifTool Version Number         : 7.74
	-File Size                       : 70 kB
	-File Modification Date/Time     : 2009:04:17 10:12:35-07:00
	+File Size                       : 94 kB
	+File Modification Date/Time     : 2009:04:21 07:02:43-07:00
	 File Type                       : PNG
	 MIME Type                       : image/png
	-Image Width                     : 1058
	-Image Height                    : 889
	+Image Width                     : 1056
	+Image Height                    : 827
	 Bit Depth                       : 8
	 Color Type                      : RGB with Alpha

<!--You can easily see that the file size and image dimensions have both changed.-->

Man sieht auf einen Blick, dass sowohl Dateigröße als auch die Bildabmessungen verändert wurden.

<!--### Keyword Expansion ###-->
### Schlüsselworterweiterung ###

<!--SVN- or CVS-style keyword expansion is often requested by developers used to those systems. The main problem with this in Git is that you can’t modify a file with information about the commit after you’ve committed, because Git checksums the file first. However, you can inject text into a file when it’s checked out and remove it again before it’s added to a commit. Git attributes offers you two ways to do this.-->

Entwickler, die an SVN- oder CVS-ähnliche Systeme gewöhnt sind, fragen oft nach der Möglichkeit Schlüsselwörter zu erweitern oder zu ersetzen. Mit Git ist dies nicht so einfach möglich, da eine Datei nach einem durchgeführten Commit nicht mehr verändert werden kann. Die Information über den Commit kann also nicht zur Datei hinzugefügt werden, da Git bereits bereits vor dem Commit die Prüfsumme berechnet. Jedoch hast Du die Möglichkeit Text einzufügen, wenn die Datei ausgecheckt wird und diesen dann wieder entfernen, wenn die Datei zu einem Commit hinzugefügt wird. Die Git Attribute bieten hierfür zwei Möglichkeiten an.

<!--First, you can inject the SHA-1 checksum of a blob into an `$Id$` field in the file automatically. If you set this attribute on a file or set of files, then the next time you check out that branch, Git will replace that field with the SHA-1 of the blob. It’s important to notice that it isn’t the SHA of the commit, but of the blob itself:-->

Zunächst kannst Du die SHA-1 Prüfsumme eines Blobs automatisch in ein `$Id$` Feld einer Datei einfügen. Wenn Du das folgende Attribut für eine oder eine Gruppe von Dateien einstellst, wird Git dieses Feld beim nächsten Checkout mit dem SHA-1 Wert dessen Blobs ersetzen. Hierbei ist es wichtig zu beachten, dass es die Prüfsumme des Blobs selbst ist, und nicht die des Commits:

	$ echo '*.txt ident' >> .gitattributes
	$ echo '$Id$' > test.txt

<!--The next time you check out this file, Git injects the SHA of the blob:-->

Wenn Du diese Datei das nächste Mal auscheckst, wird Git den SHA Wert des Blobs einfügen:

	$ rm test.txt
	$ git checkout -- test.txt
	$ cat test.txt
	$Id: 42812b7653c7b88933f8a9d6cad0ca16714b9bb3 $

<!--However, that result is of limited use. If you’ve used keyword substitution in CVS or Subversion, you can include a datestamp — the SHA isn’t all that helpful, because it’s fairly random and you can’t tell if one SHA is older or newer than another.-->

Allerdings ist das Ergebnis nur beschränkt verwertbar. Die SHA Werte als solches sind nicht sehr hilfreich, da sie recht zufällig sind und nicht festgestellt werden kann ob ein SHA Wert älter oder neuer ist, als der andere. In anderen Systemen, wie CVS oder Subversion kann man mit Hilfe der Keyword Expansion Datum- und Zeitstempel einfügen.

<!--It turns out that you can write your own filters for doing substitutions in files on commit/checkout. These are the "clean" and "smudge" filters. In the `.gitattributes` file, you can set a filter for particular paths and then set up scripts that will process files just before they’re checked out ("smudge", see Figure 7-2) and just before they’re committed ("clean", see Figure 7-3). These filters can be set to do all sorts of fun things.-->

Wie sich herausstellt, kann man aber seine eigenen Filter schreiben, um bei Commits oder Checkouts Schlüsselwörter in Dateien zu ersetzen. In der `.gitattributes` Datei kann man einen Filter für bestimmte Pfade angeben und dann Skripte einrichten, die Dateien kurz vor einem Checkout („smudge“, siehe Abbildung 7-2) und kurz vor einem Commit („clean“, siehe Abbildung 7-3) modifizieren. Diese Filter können eingerichtet werden, um alle möglichen witzigen Dinge zu machen.

<!--Figure 7-2. The “smudge” filter is run on checkout.-->

Insert 18333fig0702.png
Abbildung 7-2. Der „smudge“ Filter wird beim Checkout ausgeführt.

<!--Figure 7-3. The “clean” filter is run when files are staged.-->

Insert 18333fig0703.png
Abbildung 7-3. Der „clean“ Filter wird beim Transfer in die Staging Area ausgeführt.

<!--The original commit message for this functionality gives a simple example of running all your C source code through the `indent` program before committing. You can set it up by setting the filter attribute in your `.gitattributes` file to filter `*.c` files with the "indent" filter:-->

Die Beschreibung des ersten Commits dieser Funktionalität enthält ein einfaches Beispiel, wie man all seinen C Quellcode durch das `indent` Programm leiten lassen kann, bevor ein Commit gemacht wird. Du kannst dies einrichten, indem Du das entsprechende Filterattribut in der `.gitattributes` Datei auflistest, damit `*.c` Dateien mit dem „indent“ Programm gefiltert werden:

	*.c     filter=indent

<!--Then, tell Git what the "indent" filter does on smudge and clean:-->

Dann muss Git noch gesagt werden, was der „indent“ Filter bei „smudge“ und „clean“ zu tun hat:

	$ git config --global filter.indent.clean indent
	$ git config --global filter.indent.smudge cat

<!--In this case, when you commit files that match `*.c`, Git will run them through the indent program before it commits them and then run them through the `cat` program before it checks them back out onto disk. The `cat` program is basically a no-op: it spits out the same data that it gets in. This combination effectively filters all C source code files through `indent` before committing.-->

Wenn ein Commit Dateien umfasst, die dem Muster `*.c` entspechen, wird Git diese Dateien vor Ausführung des Commits durch das `indent` Programm leiten. Werden sie wieder ausgecheckt, so schickt Git sie durch das `cat` Programm. `cat` ist im Grunde genommen eine Null-Operation: es gibt genau die Daten wieder aus, die hereinkommen. Diese Einstellung bewirkt also tatsächlich nur, dass alle C Quellcode Dateien vor einem Commit durch den `indent` Filter bearbeitet werden.

<!--Another interesting example gets `$Date$` keyword expansion, RCS style. To do this properly, you need a small script that takes a filename, figures out the last commit date for this project, and inserts the date into the file. Here is a small Ruby script that does that:-->

Ein weiteres interessantes Beispiel ermöglicht im Stile von RCS die Schlüsselworterweiterung `$Date$`. Damit dies vernünftig funktioniert, brauchst Du ein kleines Skript, welches mit Hilfe des Dateinamen das letzte Commitdatum in diesem Projekt herausfindet und dieses Datum in die Datei einfügt. Hierzu ein kleines Beispiel als Ruby Skript:

	#! /usr/bin/env ruby
	data = STDIN.read
	last_date = `git log --pretty=format:"%ad" -1`
	puts data.gsub('$Date$', '$Date: ' + last_date.to_s + '$')

<!--All the script does is get the latest commit date from the `git log` command, stick that into any `$Date$` strings it sees in stdin, and print the results — it should be simple to do in whatever language you’re most comfortable in. You can name this file `expand_date` and put it in your path. Now, you need to set up a filter in Git (call it `dater`) and tell it to use your `expand_date` filter to smudge the files on checkout. You’ll use a Perl expression to clean that up on commit:-->

Das Skript ermittelt das letzte Commitdatum mittels des Befehls `git log`, ersetzt jede Zeichenfolge von `$Date` im Stream stdin mit dem Commitdatum und gibt das Ergebnis wieder aus. Dieses Skript sollte auch in der Skriptsprache Deiner Wahl leicht umzusetzen sein. Am besten nennst Du dieses Skript `expand_date` und legst es in Deinem Standard Suchpfad ab. Nun musst Du noch einen Filter (nennen wir ihn `dater`) in Git einrichten, der Dein `expand_date` Skript benutzt, um die Textdateien beim Checkout zu modifizieren. Zum Säubern der Dateien wird beim Commit ein Perl Ausdruck verwendet:

	$ git config filter.dater.smudge expand_date
	$ git config filter.dater.clean 'perl -pe "s/\\\$Date[^\\\$]*\\\$/\\\$Date\\\$/"'

<!--This Perl snippet strips out anything it sees in a `$Date$` string, to get back to where you started. Now that your filter is ready, you can test it by setting up a file with your `$Date$` keyword and then setting up a Git attribute for that file that engages the new filter:-->

Um wieder zum Ursprungszustand zurückzukehren entfernt dieses kurze Perl Schnipsel alles was es in einer `$Date$` Zeichenfolge findet. Jetzt da Dein Filter fertig ist, kannst Du ihn testen indem Du eine Datei mit dem `$Date$` Schlüsselwort erstellst und das entsprechende Git Attribut für diese Datei einrichtest:

	$ echo '# $Date$' > date_test.txt
	$ echo 'date*.txt filter=dater' >> .gitattributes

<!--If you commit those changes and check out the file again, you see the keyword properly substituted:-->

Wenn Du diese Änderungen eincheckst und wieder erneut auscheckst, sollte Dein Schlüsselwort korrekt ersetzt worden sein:

	$ git add date_test.txt .gitattributes
	$ git commit -m "Testing date expansion in Git"
	$ rm date_test.txt
	$ git checkout date_test.txt
	$ cat date_test.txt
	# $Date: Tue Apr 21 07:26:52 2009 -0700$

<!--You can see how powerful this technique can be for customized applications. You have to be careful, though, because the `.gitattributes` file is committed and passed around with the project but the driver (in this case, `dater`) isn’t; so, it won’t work everywhere. When you design these filters, they should be able to fail gracefully and have the project still work properly.-->

Man kann sehen wie mächtig diese Technik für Deinen Entwickleralltag sein kann. Da die `.gitattributes` Datei ebenfalls im Git Repository verwaltet wird und damit an alle Benutzer weitergeben wird, solltest Du vorsichtig mit Filtern umgehen. Denn Dein Filterskript (in diesem Fall das Skript `dater`) liegt nicht unter Versionskontrolle. Deshalb kann es passieren, dass die Schlüsselwortersetzung beziehungsweise das Arbeiten mit dem Repository nicht bei jedem funktioniert. Beim Entwickeln von Filtern solltest Du deshalb darauf achten, dass das Projekt weiterhin benutzt werden kann, auch wenn ein Filter einmal fehlschlägt.

<!--### Exporting Your Repository ###-->
### Exportieren von Repositorys ###

<!--Git attribute data also allows you to do some interesting things when exporting an archive of your project.-->

Git Attribute erlauben auch einige interessante Dinge, wenn Du Dein Projekt in ein Archiv exportierst.

<!--#### export-ignore ####-->
#### export-ignore ####

<!--You can tell Git not to export certain files or directories when generating an archive. If there is a subdirectory or file that you don’t want to include in your archive file but that you do want checked into your project, you can determine those files via the `export-ignore` attribute.-->

Du kannst Git anweisen gewisse Dateien oder Verzeichnisse nicht zu exportieren, wenn es ein Archiv erzeugt. Falls es Unterverzeichnisse oder Dateien gibt, die Du nicht in Deiner Archivdatei haben willst, aber in Deinem Projektrepository, so kannst Du diese Datein mit Hilfe des `export-ignore` Attributes festlegen.

<!--For example, say you have some test files in a `test/` subdirectory, and it doesn’t make sense to include them in the tarball export of your project. You can add the following line to your Git attributes file:-->

Nehmen wir zum Beispiel an, Du hast einige Testdateien in einem `test/` Unterverzeichnis und es macht keinen Sinn, dass diese in einem Tarball Export Deines Projekts enthalten sind. In diesem Fall kannst Du die folgende Zeile in Deine Git Attribute aufnehmen:

	test/ export-ignore

<!--Now, when you run git archive to create a tarball of your project, that directory won’t be included in the archive.-->

Wenn Du jetzt `git archive` ausführst, um einen Tarball Deines Projekts zu erstellen, wird das Verzeichnis nicht mit in das Archiv aufgenommen.

<!--#### export-subst ####-->
#### export-subst ####

<!--Another thing you can do for your archives is some simple keyword substitution. Git lets you put the string `$Format:$` in any file with any of the `-\-pretty=format` formatting shortcodes, many of which you saw in Chapter 2. For instance, if you want to include a file named `LAST_COMMIT` in your project, and the last commit date was automatically injected into it when `git archive` ran, you can set up the file like this:-->

Auch das einfache Ersetzen von Schlüsselwörtern ist bei einem Archivierungsvorgang möglich. Git erlaubt die Zeichenfolge `$Format:$` mit allen Formatierungsoptionen des Parameters `--pretty=format` in jeglichen Dateien. Viele der Optionen hast Du bereits in Kapitel 2 kennengelernt. Wenn Du zum Beispiel eine Datei namens `LAST_COMMIT` zu Deinem  Projekt hinzufügen willst, welche das Datum des letzten  Commits enthalten soll, dann kannst Du die folgenden Befehle ausführen:

	$ echo 'Last commit date: $Format:%cd$' > LAST_COMMIT
	$ echo "LAST_COMMIT export-subst" >> .gitattributes
	$ git add LAST_COMMIT .gitattributes
	$ git commit -am 'adding LAST_COMMIT file for archives'

<!--When you run `git archive`, the contents of that file when people open the archive file will look like this:-->

Nach Ausführung des Befehls `git archive`, wird die Datei `LAST_COMMIT` in Deinem Archiv in etwa folgendermaßen aussehen:

	$ cat LAST_COMMIT
	Last commit date: $Format:Tue Apr 21 08:38:48 2009 -0700$

<!--### Merge Strategies ###-->
### Merge Strategien ###

<!--You can also use Git attributes to tell Git to use different merge strategies for specific files in your project. One very useful option is to tell Git to not try to merge specific files when they have conflicts, but rather to use your side of the merge over someone else’s.-->

Die Git Attribute ermöglichen es ebenso verschiedene Regeln für das Zusammenführen bestimmter Dateien innerhalb Deines Projekts festzulegen. Eine besonders nützliche Option ist es, Git so einzustellen, dass es bei bestimmten Dateien kein Zusammenführen von Konfliktstellen versucht, sondern einfach Deine Version übernimmt und die des anderen verwirft.

<!--This is helpful if a branch in your project has diverged or is specialized, but you want to be able to merge changes back in from it, and you want to ignore certain files. Say you have a database settings file called database.xml that is different in two branches, and you want to merge in your other branch without messing up the database file. You can set up an attribute like this:-->

Dies ist hilfreich, falls ein Zweig Deines Projekts sehr weit vom Hauptzweig abgewichen oder sehr speziell ist, aber Du weiterhin in der Lage sein willst, Änderungen daran zurückzuführen und dabei gewisse Dateien zu ignorieren. Nehmen wir an Du hast eine Konfigurationsdatei einer Datenbank namens database.xml, welche sich in zwei Zweigen unterscheidet. Wenn Du jetzt einen Merge von dem anderen Zweig machen möchtest ohne Deine Datenbankdatei unbrauchbar zu machen, dann kannst Du folgendes Attribut einrichten:

	database.xml merge=ours

<!--If you merge in the other branch, instead of having merge conflicts with the database.xml file, you see something like this:-->

Wenn Du ein Merge des anderen Zweiges machst, werden für die Datei database.xml keine Merge-Konflikte auftreten, sondern es wird folgendes ausgegeben:

	$ git merge topic
	Auto-merging database.xml
	Merge made by recursive.

<!--In this case, database.xml stays at whatever version you originally had.-->

In diesem Fall wird die Datei database.xml aus dem anderen Zweig ignoriert und in Deinem Zweig bleibt die Datei im gleichen Zustand wie vor dem Merge.

<!--## Git Hooks ##-->
## Git Hooks ##

<!--Like many other Version Control Systems, Git has a way to fire off custom scripts when certain important actions occur. There are two groups of these hooks: client side and server side. The client-side hooks are for client operations such as committing and merging. The server-side hooks are for Git server operations such as receiving pushed commits. You can use these hooks for all sorts of reasons, and you’ll learn about a few of them here.-->

Genau wie bei vielen anderen Versionskontrollsystemen gibt es auch bei Git die Möglichkeit eigene Skripte zu starten, wenn bestimmte, wichtige Ereignisse auftreten. Es gibt zwei Gruppen dieser Einschubmethoden: Hooks für den Client und Hooks für den Server. Die Hooks für den Client können bei Ereignissen, wie zum Beispiel einem Commit oder Merge, eingerichtet werden. Die Hooks für den Server können bei Operationen wie den Empfang von hochgeladenen Commits, ausgeführt werden. Es gibt viele Möglichkeiten diese Hooks sinnvoll einzusetzen. Einige davon werde ich hier vorstellen.

<!--### Installing a Hook ###-->
### Installieren eines Hooks ###

<!--The hooks are all stored in the `hooks` subdirectory of the Git directory. In most projects, that’s `.git/hooks`. By default, Git populates this directory with a bunch of example scripts, many of which are useful by themselves; but they also document the input values of each script. All the examples are written as shell scripts, with some Perl thrown in, but any properly named executable scripts will work fine — you can write them in Ruby or Python or what have you. These example hook files end with .sample; you’ll need to rename them.-->

Sämtliche Hooks werden im `hooks` Unterverzeichnis des Git Verzeichnisses gespeichert. In den meisten Projekten wird das `.git/hooks` sein. Git installiert in dieses Verzeichnis standardmäßig Beispielskripte. Einige davon sind auch ohne Änderung nützlich und sofort einsetzbar. Zusätzlich dokumentieren diese Beispiele die Eingabewerte des jeweiligen Skripts. Alle Beispiele sind Shellskripte, die hier und da ein Paar Zeilen Perl Code enthalten. Prinzipiell sollte aber jedes ausführbare Skript funktionieren, wenn es korrekt benannt wird. Du kannst also die Skriptsprache Deiner Wahl verwenden, z.B. Ruby oder Python. Die Beispieldateien haben die Endung .sample, sie müssen also nur noch umbenannt werden.

<!--To enable a hook script, put a file in the `hooks` subdirectory of your Git directory that is named appropriately and is executable. From that point forward, it should be called. I’ll cover most of the major hook filenames here.-->

Um ein Hook-Skript zu aktivieren, speichere eine entsprechend benannte und ausführbare Datei im `hooks` Unterverzeichnis Deines Git Verzeichnisses. Von diesem Augenblick an sollte es ausgeführt werden. Ich werde hier die meisten der wichtigen Hook Dateinamen besprechen.

<!--### Client-Side Hooks ###-->
### Hooks für den Client ###

<!--There are a lot of client-side hooks. This section splits them into committing-workflow hooks, e-mail-workflow scripts, and the rest of the client-side scripts.-->

Es gibt eine Menge Hooks auf Seiten des Clients. Der folgende Abschnitt teilt die Hooks in drei Gruppen auf: Skripte für den Commit Vorgang, Skripte für den Arbeitsablauf mit E-Mails und den Rest der Client Skripte.

<!--#### Committing-Workflow Hooks ####-->
#### Hooks für den Commit Vorgang ####

<!--The first four hooks have to do with the committing process. The `pre-commit` hook is run first, before you even type in a commit message. It’s used to inspect the snapshot that’s about to be committed, to see if you’ve forgotten something, to make sure tests run, or to examine whatever you need to inspect in the code. Exiting non-zero from this hook aborts the commit, although you can bypass it with `git commit -\-no-verify`. You can do things like check for code style (run lint or something equivalent), check for trailing whitespace (the default hook does exactly that), or check for appropriate documentation on new methods.-->

Die ersten vier Hooks hängen mit dem Commit Prozess zusammen. Der `pre-commit` Hook wird zuerst ausgeführt, schon bevor Du die Commit Nachricht eingegeben hast. Der Hook wird oft benutzt, um den zu versionierenden Zustand des Arbeitsverzeichnisses zu prüfen, um festzustellen ob etwas vergessen wurde, um sicherzustellen das Tests ausgeführt wurden oder aus irgendeinem anderen Grund, der es nötig macht, den Code vor dem Commit zu inspizieren. Wenn das entsprechende Skript einen Wert ungleich Null zurückgibt, wird der Commit abgebrochen. Auch für die Prüfung, ob Kodierrichtlinien eingehalten wurden oder für eine statische Codeanalyse (z.B. mit lint oder einem entsprechenden Programm) kann dieses Skript verwendet werden. Das von Git installierte Beispielskript prüft zum Beispiel, ob am Zeilenende Leerzeichen vorhanden sind. Der Hook kann mit `git commit --no-verify` auch umgangen werden.

<!--The `prepare-commit-msg` hook is run before the commit message editor is fired up but after the default message is created. It lets you edit the default message before the commit author sees it. This hook takes a few options: the path to the file that holds the commit message so far, the type of commit, and the commit SHA-1 if this is an amended commit. This hook generally isn’t useful for normal commits; rather, it’s good for commits where the default message is auto-generated, such as templated commit messages, merge commits, squashed commits, and amended commits. You may use it in conjunction with a commit template to programmatically insert information.-->

Der `prepare-commit-msg` Hook wird ausgeführt, bevor der Editor für die Commit Nachricht geöffnet wird, aber nachdem die Standardnachricht erstellt wurde. Er erlaubt es die Standardnachricht zu modifizieren, bevor der Autor des Commits sie sieht. Dieser Hook akzeptiert diverse Optionen: den Pfad der Datei, die die bisherige Commit Nachricht enthält, den Typ des Commit und den SHA-1 Hash des Commit, falls es sich um ein Korrektur-Commit handelt. Dieser Hook ist üblicherweise nicht sehr nützlich bei normalen Commits; er ist eher für solche Commits gedacht, bei denen die Standardnachricht automatisch generiert wird, wie zum Beispiel vorlagenbasierte Commit Nachrichten, Commits nach einem Merge, Commits, die zusammengeführt werden und Korrektur-Commits. Du kannst diesen Hook mit einer Commit Vorlage kombinieren, um automatisiert Informationen einzufügen.

<!--The `commit-msg` hook takes one parameter, which again is the path to a temporary file that contains the current commit message. If this script exits non-zero, Git aborts the commit process, so you can use it to validate your project state or commit message before allowing a commit to go through. In the last section of this chapter, I’ll demonstrate using this hook to check that your commit message is conformant to a required pattern.-->

Der `commit-msg` Hook akzeptiert einen Parameter, der wiederum der Pfad zu der temporären Datei ist, die die momentane Commit Nachricht enthält. Falls dieses Skript nicht Null zurückgibt, so wird der Commit abgebrochen. Damit kannst Du die Gültigkeit des Projekstatus oder die Commit Nachricht prüfen, bevor ein Commit akzeptiert wird. Im letzten Abschnitt dieses Kapitels werde ich beschreiben, wie man diesen Hook benutzt, um sicherzustellen, dass Commit Nachrichten einem bestimmten Muster entsprechen.

<!--After the entire commit process is completed, the `post-commit` hook runs. It doesn’t take any parameters, but you can easily get the last commit by running `git log -1 HEAD`. Generally, this script is used for notification or something similar.-->

Wenn ein Commit komplett abgeschlossen wurde, wird der `post-commit` Hook ausgeführt. Er akzeptiert keine Parameter, aber Du kannst den letzten Commit einfach mit dem Befehl `git log -1 HEAD` abfragen. Dieses Skript wird üblicherweise für das Senden von Benachrichtigungen oder ähnlichem benutzt.

<!--The committing-workflow client-side scripts can be used in just about any workflow. They’re often used to enforce certain policies, although it’s important to note that these scripts aren’t transferred during a clone. You can enforce policy on the server side to reject pushes of commits that don’t conform to some policy, but it’s entirely up to the developer to use these scripts on the client side. So, these are scripts to help developers, and they must be set up and maintained by them, although they can be overridden or modified by them at any time.-->

Diese Skripte für den Commit Prozess können für jeden anderen Arbeitsablauf entsprechend angepasst werden. Oft werden sie benutzt um bestimmte Regeln zu erzwingen. Dabei ist es wichtig zu wissen, dass diese Skripte beim Klonen eines Repositorys nicht mit übertragen werden. Du kannst auf Seiten des Servers die Einhaltung von bestimmten Regeln erzwingen indem die hochgeladenen Commits abgelehnt werden, wenn sie diesen Prinzipien nicht entsprechen. Auf dem Client entscheidet aber der Anwender selber, ob er diese Skripte verwendet oder nicht. Dies sind also Skripte, die den Entwicklern helfen sollen, und sie müssen von ihnen erstellt und gepflegt werden. Aber sie können auch von ihnen jederzeit verändert oder umgangen werden.

<!--#### E-mail Workflow Hooks ####-->
#### Hooks für den Arbeitsablauf mit E-Mails ####

<!--You can set up three client-side hooks for an e-mail-based workflow. They’re all invoked by the `git am` command, so if you aren’t using that command in your workflow, you can safely skip to the next section. If you’re taking patches over e-mail prepared by `git format-patch`, then some of these may be helpful to you.-->

Für einen E-Mail basierten Arbeitsablauf kannst Du drei Hooks auf dem Client einrichten. Sie werden alle bei Ausführung des Befehls `git am` aufgerufen. Wenn Du also diesen Befehl in Deinem normalen Arbeitsablauf nicht verwendest, kann Du guten Gewissens zum nächsten Abschnitt springen. Falls Du aber Patches per E-Mail erhälst, die mit `git format-patch` erstellt wurden, könnten trotzdem einige dieser Skripte nützlich für Dich sein.

<!--The first hook that is run is `applypatch-msg`. It takes a single argument: the name of the temporary file that contains the proposed commit message. Git aborts the patch if this script exits non-zero. You can use this to make sure a commit message is properly formatted or to normalize the message by having the script edit it in place.-->

Der erste Hook, der ausgeführt wird, ist `applypatch-msg`. Er akzeptiert genau einen Parameter: den Namen der temporären Datei, die die vorgegebene Commit Nachricht enthält. Git bricht den Patch ab, falls dieses Skript nicht Null zurückgibt. Du kannst dies benutzen um sicherzustellen, dass die Commit Nachricht richtig formatiert ist, oder um die Nachricht zu standardisieren, indem das Skript sie direkt editiert.

<!--The next hook to run when applying patches via `git am` is `pre-applypatch`. It takes no arguments and is run after the patch is applied, so you can use it to inspect the snapshot before making the commit. You can run tests or otherwise inspect the working tree with this script. If something is missing or the tests don’t pass, exiting non-zero also aborts the `git am` script without committing the patch.-->

Der nächste Hook, der beim Anwenden von Patches via `git am` ausgeführt wird, ist `pre-applypatch`. Er benötigt keine Parameter und wird direkt nach Anwendung des Patches ausgeführt. Damit kannst Du den Zustand Deines Projektes noch vor dem eigentlich Commit inspizieren. Du kannst mit diesem Skript Tests ablaufen lassen oder das Arbeitsverzeichnis anderweitig untersuchen. Falls etwas fehlt oder ein Test fehlschlägt, sorgt eine Beenden des Skripts mit einem Wert ungleich Null ebenfalls für das Abbrechen des `git am` Skripts. Es wird also auch kein Commit ausgeführt.

<!--The last hook to run during a `git am` operation is `post-applypatch`. You can use it to notify a group or the author of the patch you pulled in that you’ve done so. You can’t stop the patching process with this script.-->

Der letzte Hook, der während der `git am` Operation ausgeführt wird, ist `post-applypatch`. Du kannst dies verwenden, um eine Benutzergruppe oder den Autoren des Patches darueber zu informieren, dass der Patch übernommen wurde. Der eigentliche Patch Vorgang kann mit diesem Skript aber nicht mehr abgebrochen werden.

<!--#### Other Client Hooks ####-->
#### Weitere Hooks für den Client ####

<!--The `pre-rebase` hook runs before you rebase anything and can halt the process by exiting non-zero. You can use this hook to disallow rebasing any commits that have already been pushed. The example `pre-rebase` hook that Git installs does this, although it assumes that next is the name of the branch you publish. You’ll likely need to change that to whatever your stable, published branch is.-->

Der `pre-rebase` Hook wird ausgeführt, bevor ein Rebase gestartet wird. Durch einen Rückgabewert ungleich Null kann der Rebase Vorgang abgebrochen werden. Du kannst diesen Hook dazu verwenden um beispielsweise zu verhindern, dass auf bereits gepushte Commits ein Rebase durchgeführt wird. Der von Git installierte Beispiel-Hook für `pre-rebase` macht genau das. Allerdings nimmt dieser an, dass der Name des veröffentlichten Branches ‚next‘ ist. Du musst wahrscheinlich den Namen durch den Deines stabilen, öffentlichen Branches ersetzen.

<!--After you run a successful `git checkout`, the `post-checkout` hook runs; you can use it to set up your working directory properly for your project environment. This may mean moving in large binary files that you don’t want source controlled, auto-generating documentation, or something along those lines.-->

Nach jedem erfolgreichen `git-checkout` wird der `post-checkout` Hook ausgeführt. Du kannst ihn verwenden, um Dein Arbeitsverzeichnis  für Deine Arbeitsumgebung einzurichten. Das kann das Hinzukopieren großer Binärdateien bedeuten, die Du nicht unter Versionskontrolle stellen möchtest, das automatisierte Generieren von Dokumentation, oder entsprechend ähnliche Aktionen.

<!--Finally, the `post-merge` hook runs after a successful `merge` command. You can use it to restore data in the working tree that Git can’t track, such as permissions data. This hook can likewise validate the presence of files external to Git control that you may want copied in when the working tree changes.-->

Der letzte Hook, den ich vorstellen möchte, ist der `post-merge` Hook. Er wird nach jedem erfolgreichen Aufruf von `merge` ausgeführt. Du kannst diesen benutzen, um Daten in Deinem Arbeitsverzeichnis wiederherzustellen, die Git nicht unter Versionskontrolle stellen kann. Das sind zum Beispiel Berechtigungsdaten. Dieser Hook kann genauso überprüfen, ob Dateien, die nicht unter Versionskontrolle stehen, entsprechend in das Arbeitsverzeichnis kopiert worden sind, wenn sich dieses ändert.

<!--### Server-Side Hooks ###-->
### Serverseitige Hooks ###

<!--In addition to the client-side hooks, you can use a couple of important server-side hooks as a system administrator to enforce nearly any kind of policy for your project. These scripts run before and after pushes to the server. The pre hooks can exit non-zero at any time to reject the push as well as print an error message back to the client; you can set up a push policy that’s as complex as you wish.-->

Neben den Hooks für den Client, kannst Du als Systemadministrator auch einige wichtige Hooks auf Seiten des Servers installieren. Damit kannst Du nahezu jede Art von Richtlinie für Dein Projekt erzwingen. Die Skripte werden ausgeführt bevor und nachdem ein Push auf den Server durchgeführt wurde. Das Skript für den vorgelagerten Hook kann den Push jederzeit abbrechen indem es einen Wert ungleich Null zurückgibt. Zusätzlich kann dem Client eine Fehlermeldung zurückgeliefert werden. Mit diesen Hooks kannst Du eine beliebig komplexe Push Richtlinie umsetzen.

<!--#### pre-receive and post-receive ####-->
#### pre-receive und post-receive ####

<!--The first script to run when handling a push from a client is `pre-receive`. It takes a list of references that are being pushed from stdin; if it exits non-zero, none of them are accepted. You can use this hook to do things like make sure none of the updated references are non-fast-forwards; or to check that the user doing the pushing has create, delete, or push access or access to push updates to all the files they’re modifying with the push.-->

Das erste Skript, dass ausgeführt wird, wenn ein Push von einem Client empfangen wird, ist `pre-receive`. Es akzeptiert eine Liste von Referenzen, die über ‚stdin‘ hochgeladen werden. Wird es mit einem Wert ungleich Null beendet, so wird keine von ihnen akzeptiert. Du kannst diesen Hook benutzen, um sicherzustellen, dass keine Pushes durchgeführt werden können, welche nicht einem Fast-Forward entsprechen. Ebenso ist es möglich zu Prüfen, ob der Client, die entsprechende Berechtigung zum Erstellen, Löschen oder Aktualisieren eines Branches hat oder ob er die Berechtigung hat, die jeweiligen Dateien zu ändern, die mit dem Push hochgeladen werden.

<!--The `post-receive` hook runs after the entire process is completed and can be used to update other services or notify users. It takes the same stdin data as the `pre-receive` hook. Examples include e-mailing a list, notifying a continuous integration server, or updating a ticket-tracking system — you can even parse the commit messages to see if any tickets need to be opened, modified, or closed. This script can’t stop the push process, but the client doesn’t disconnect until it has completed; so, be careful when you try to do anything that may take a long time.-->

Der `post-receive` Hook wird aufgerufen, nachdem der komplette Prozess abgeschlossen ist und kann zum Aktualisieren anderer Dienste oder zum Benachrichtigen von Benutzern verwendet werden. Er erwartet die gleichen ‚stdin‘ Daten wie `pre-receive`. Beispielsweise können folgende Aktionen ausgeführt werden: Versand von E-Mails an eine vorgefertigte Liste von Personen, Benachrichtigen eins Continuous Integration Servers oder Aktualisieren eines Issue-Tracking-Werkzeugs (Du kannst sogar die Commit Nachrichten parsen um zu prüfen, ob bestimmte Tickets geöffnet, aktualisiert oder geschlossen werden müssen). Das Skript kann allerdings den Push Prozess nicht abbrechen und der Client bleibt bis zum Abschluss des Skripts mit dem Server verbunden. Du solltest deshalb darauf achten, dass Du keinen Vorgang ausführst, der zu viel Zeit in Anspruch nimmt.

<!--#### update ####-->
#### update ####

<!--The update script is very similar to the `pre-receive` script, except that it’s run once for each branch the pusher is trying to update. If the pusher is trying to push to multiple branches, `pre-receive` runs only once, whereas update runs once per branch they’re pushing to. Instead of reading from stdin, this script takes three arguments: the name of the reference (branch), the SHA-1 that reference pointed to before the push, and the SHA-1 the user is trying to push. If the update script exits non-zero, only that reference is rejected; other references can still be updated.-->

Das Update Skript ist dem `pre-receive` Skript sehr ähnlich, außer dass es für jeden Branch, den der Client aktualisieren will, ausgeführt wird. Wenn der Benutzer des Clients versucht mehrere Branches zu pushen, wird `pre-receive` nur einmalig aufgerufen, wohingegen das Update Skript für jeden einzelnen Branch ausgeführt wird. Anstatt von dem Stream stdin zu lesen, akzeptiert dieses Skript drei Argumente: der Name der Referenz (Branch), die SHA-1 Prüfsumme auf die die Referenz vor dem Push zeigt und die SHA-1 Prüfsumme, die der Anwender versucht zu pushen. Wenn das Update Skript einen Wert ungleich Null zurückgibt, wird der Vorgang nur für diese Referenz abgebrochen, die anderen Referenzen werden weiterhin aktualisiert.

<!--## An Example Git-Enforced Policy ##-->
## Beispiel für die Durchsetzung von Richtlinien mit Hilfe von Git ##

<!--In this section, you’ll use what you’ve learned to establish a Git workflow that checks for a custom commit message format, enforces fast-forward-only pushes, and allows only certain users to modify certain subdirectories in a project. You’ll build client scripts that help the developer know if their push will be rejected and server scripts that actually enforce the policies.-->

In diesem Abschnitt werden wir die gelernten Dinge verwenden um einen Git Arbeitsablauf umzusetzen, der das Format der Commit Nachrichten prüft, nur Pushes zulässt, die einem Fast-Forward entsprechen und der es nur einem beschränkten Kreis von Nutzern ermöglicht einzelne Unterverzeichnisse innerhalb eines Projekts zu modifizieren. Wir werden Client Skripte erstellen, die für den Entwickler prüfen, ob seine Pushes abgelehnt werden würden und wir werden Server Skripte erstellen, die diese Richtlinien um- bzw. durchsetzen.

<!--I used Ruby to write these, both because it’s my preferred scripting language and because I feel it’s the most pseudocode-looking of the scripting languages; thus you should be able to roughly follow the code even if you don’t use Ruby. However, any language will work fine. All the sample hook scripts distributed with Git are in either Perl or Bash scripting, so you can also see plenty of examples of hooks in those languages by looking at the samples.-->

Ich habe für diese Hooks Ruby verwendet, weil es einerseits meine bevorzugte Skriptsprache ist und andererseits weil der resultierende Code nahezu einem leicht zu lesenden Pseudo-Code entspricht. Auch wenn Du Ruby normalerweise nicht einsetzt, solltest Du deshalb in der Lage sein, meinen Ausführungen zu folgen. Jede andere Sprache sollte aber genauso funktionieren. Alle Beispielskripte, die standardmäßig in Git enthalten sind, sind entweder Perl oder Bash Skripte. Für diese Sprache findest Du also auch genügend Beispiele.

<!--### Server-Side Hook ###-->
### Server Hooks ###

<!--All the server-side work will go into the update file in your hooks directory. The update file runs once per branch being pushed and takes the reference being pushed to, the old revision where that branch was, and the new revision being pushed. You also have access to the user doing the pushing if the push is being run over SSH. If you’ve allowed everyone to connect with a single user (like "git") via public-key authentication, you may have to give that user a shell wrapper that determines which user is connecting based on the public key, and set an environment variable specifying that user. Here I assume the connecting user is in the `$USER` environment variable, so your update script begins by gathering all the information you need:-->

Die gesamten Skripte für den Server gehören in die Update Datei in Deinem Hooks Verzeichnis. Die Update Datei wird für jeden Branch, der gepusht wird, gestartet und erhält als Parameter die Referenz, die gepusht wird, die alte Revision auf der der Branch stand und die neue Revision, die gepusht wird. Wenn der Push über SSH ausgeführt wird, hat es auch Zugriff auf den Benutzer mit dem der Push durchgeführt wird. Wenn Du den Server so konfiguriert hast, dass jeder über einen einzelnen Benutzer (zum Beispiel „git“) über das Public-Key Verfahren zugreifen kann, dann wäre es sinnvoll diesem Benutzer einen Shell Wrapper einzurichten, der über den öffentlichen Schlüssel die Identität feststellt und damit die Umgebungsvariablen für den jeweiligen Benutzer setzen kann. In dem Beispiel setze ich voraus, dass der Benutzer, der sich verbinden will, in der Umgebungsvariable `$USER` enthalten ist. Deshalb sammelt das Update Skript erstmal alle benötigten Informationen:

	#!/usr/bin/env ruby

	$refname = ARGV[0]
	$oldrev  = ARGV[1]
	$newrev  = ARGV[2]
	$user    = ENV['USER']

	puts "Enforcing Policies... \n(#{$refname}) (#{$oldrev[0,6]}) (#{$newrev[0,6]})"

<!--Yes, I’m using global variables. Don’t judge me — it’s easier to demonstrate in this manner.-->

Ja, ich verwende globale Variablen. Bitte steinigt mich dafür nicht. Auf diese Art und Weise ist es für mich einfacher das Ganze zu demonstrieren.

<!--#### Enforcing a Specific Commit-Message Format ####-->
#### Format der Commit Nachricht erzwingen ####

<!--Your first challenge is to enforce that each commit message must adhere to a particular format. Just to have a target, assume that each message has to include a string that looks like "ref: 1234" because you want each commit to link to a work item in your ticketing system. You must look at each commit being pushed up, see if that string is in the commit message, and, if the string is absent from any of the commits, exit non-zero so the push is rejected.-->

Deine erste Herausforderung wird es sein, sicherzustellen, dass jede Commit Nachricht einem bestimmten Format entspricht. Nehmen wir zum Beispiel an, dass jeder Commit mit einem Ticket in Deinem Issue-Tracking-System verknüpft sein soll. Deshalb soll jede Commit Nachricht diese Referenz in etwa dem Format „ref: 1234“ enthalten. Dazu musst Du jeden Commit, der gepusht werden soll, prüfen, ob der entsprechende Text enthalten ist. Ist er es nicht, so musst Du das entsprechende Skripte mit einem Rückgabewert ungleich Null beenden, damit der Push abgelehnt beziehungsweise abgebrochen wird.

<!--You can get a list of the SHA-1 values of all the commits that are being pushed by taking the `$newrev` and `$oldrev` values and passing them to a Git plumbing command called `git rev-list`. This is basically the `git log` command, but by default it prints out only the SHA-1 values and no other information. So, to get a list of all the commit SHAs introduced between one commit SHA and another, you can run something like this:-->

Eine Liste aller SHA-1 Prüfsummen, die gepusht werden sollen, erhälst Du, indem Du die Werte `$newrev` und `$oldrev` an das Git Kommando `git rev-list` übergibst (Dieser Befehl gehört zu den Low-Level Funktionen von Git. Im Englischen werden diese auch als „plumbing“ Befehle bezeichnet). Der Befehl entspricht dem `git log` Kommando, gibt aber im Gegensatz zu diesem nur die SHA-1 Prüfsummen und keine weitere Informationen aus. Um eine Liste aller SHA-1 Prüfsummen zwischen zwei Commits zu erhalten, musst Du in etwa folgendes eingeben:

	$ git rev-list 538c33..d14fc7
	d14fc7c847ab946ec39590d87783c69b031bdfb7
	9f585da4401b0a3999e84113824d15245c13f0be
	234071a1be950e2a8d078e6141f5cd20c1e61ad3
	dfa04c9ef3d5197182f13fb5b9b1fb7717d2222a
	17716ec0f1ff5c77eff40b7fe912f9f6cfd0e475

<!--You can take that output, loop through each of those commit SHAs, grab the message for it, and test that message against a regular expression that looks for a pattern.-->

Du kannst nun durch diese Liste iterieren und für jeden SHA-1 Commit die entsprechende Commit Nachricht anfordern und diese mit Hilfe eines regulären Ausdrucks auf das jeweilige Format prüfen.

<!--You have to figure out how to get the commit message from each of these commits to test. To get the raw commit data, you can use another plumbing command called `git cat-file`. I’ll go over all these plumbing commands in detail in Chapter 9; but for now, here’s what that command gives you:-->

Um dies durchführen zu können, benötigst Du das Wissen, wie man an die Commit Nachricht eines einzelnen Commits herankommt. Um die Rohdaten eines Commits zu erhalten, kannst Du eine andere Low-Level Funktion von Git verwenden, nämlich `git cat-file`. Weitere Low-Level Funktionen werde ich in Kapitel 9 näher erläutern, aber hier reicht es erst einmal, wenn Du das Kommando einfach mal ausprobierst:

	$ git cat-file commit ca82a6
	tree cfda3bf379e4f8dba8717dee55aab78aef7f4daf
	parent 085bb3bcb608e1e8451d4b2432f8ecbe6306e7e7
	author Scott Chacon <schacon@gmail.com> 1205815931 -0700
	committer Scott Chacon <schacon@gmail.com> 1240030591 -0700

	changed the version number

<!--A simple way to get the commit message from a commit when you have the SHA-1 value is to go to the first blank line and take everything after that. You can do so with the `sed` command on Unix systems:-->

Um die Commit Nachricht auf Basis der SHA-1 Prüfsumme zu extrahieren, gibt es eine einfache Möglichkeit. Dazu musst Du die Position der ersten leeren Zeile bestimmen. Der gesamte Text nach dieser leeren Zeile entspricht der Commit Nachricht. Mit dem `sed` Befehl funktioniert das unter Unix Systemen ganz einfach:

	$ git cat-file commit ca82a6 | sed '1,/^$/d'
	changed the version number

<!--You can use that incantation to grab the commit message from each commit that is trying to be pushed and exit if you see anything that doesn’t match. To exit the script and reject the push, exit non-zero. The whole method looks like this:-->

Damit sollte es Dir auf einfache Art und Weise möglich sein, jede einzelne Commit Nachricht eines Commits, welcher gepusht werden soll, zu prüfen. Du kannst den Push abbrechen, sollte einer der Nachrichten nicht dem gewünschten Format entsprechen. Um ihn abzubrechen reicht es, wenn der Rückgabewert des Skripts ungleich Null ist. Zusammengefasst ergibt sich die folgende Methode:

	$regex = /\[ref: (\d+)\]/

	# enforced custom commit message format
	def check_message_format
	  missed_revs = `git rev-list #{$oldrev}..#{$newrev}`.split("\n")
	  missed_revs.each do |rev|
	    message = `git cat-file commit #{rev} | sed '1,/^$/d'`
	    if !$regex.match(message)
	      puts "[POLICY] Your message is not formatted correctly"
	      exit 1
	    end
	  end
	end
	check_message_format

<!--Putting that in your `update` script will reject updates that contain commits that have messages that don’t adhere to your rule.-->

Wenn Du diesen Auszug in Dein `update` Skript einbaust, wird jeder Push abgelehnt, der eine Commit Nachricht enthält, die nicht Deinen Regeln entspricht.

<!--#### Enforcing a User-Based ACL System ####-->
#### Einrichten eines benutzerspezifischen ACL-Systems ####

<!--Suppose you want to add a mechanism that uses an access control list (ACL) that specifies which users are allowed to push changes to which parts of your projects. Some people have full access, and others only have access to push changes to certain subdirectories or specific files. To enforce this, you’ll write those rules to a file named `acl` that lives in your bare Git repository on the server. You’ll have the `update` hook look at those rules, see what files are being introduced for all the commits being pushed, and determine whether the user doing the push has access to update all those files.-->

Nehmen wir einmal an, dass Du für Deine Projekte ein Mechanismus einrichten willst, der festlegt, wer auf welche Teile Deines Projekts pushen kann. Mit Hilfe einer Zugriffssteuerungsliste (ACL – Access Control List) ist so etwas möglich. Manche Benutzer sollen vollen Zugriff auf das gesamte Repository haben, andere widerrum dürfen nur auf bestimmte Unterverzeichnisse oder spezielle Dateien pushen. Um diese Regeln durchzusetzen werden wir eine Datei mit dem Namen `acl` erstellen und diese im Bare Repository auf Deinem Git Server ablegen. Außerdem werden wir den `update` Hook so anpassen, dass dieser die erstellten Regeln prüft und bestimmt, ob die jeweilige Aktion vom jeweiligen Benutzer ausgeführt werden darf. Dazu muss der Hook alle Commits, die gepusht werden, prüfen.

<!--The first thing you’ll do is write your ACL. Here you’ll use a format very much like the CVS ACL mechanism: it uses a series of lines, where the first field is `avail` or `unavail`, the next field is a comma-delimited list of the users to which the rule applies, and the last field is the path to which the rule applies (blank meaning open access). All of these fields are delimited by a pipe (`|`) character.-->

Der erste Schritt ist das Erstellen einer ACL. In unserem Beispiel verwenden wir ein Format, welches der CVS ACL sehr ähnlich ist. Jede Zeile ist nach dem selben Format aufgebaut. Das erste Feld einer Zeile enthält entweder `avail` oder `unavail`. Das nächste Feld ist ein kommaseparierte Liste aller User, auf die die Regel zutrifft. Das letzte Feld enthält den Pfad auf welche die Regel zutrifft (ein leeres Feld bedeutet in diesem Fall freien Zugriff). Alle Felder werden durch einen senkrechten Strich (`|`, auch Pipe genannt) getrennt.

<!--In this case, you have a couple of administrators, some documentation writers with access to the `doc` directory, and one developer who only has access to the `lib` and `tests` directories, and your ACL file looks like this:-->

In unserem Beispiel gibt es ein paar Administratoren, ein paar Leute, die sich um die Dokumentation im Verzeichnis `doc` kümmern, und einen Entwickler, der nur auf das `lib` und das `test` Verzeichnis zugreifen darf. In diesem Fall sollte die ACL Datei etwa folgendermaßen aussehen:

	avail|nickh,pjhyett,defunkt,tpw
	avail|usinclair,cdickens,ebronte|doc
	avail|schacon|lib
	avail|schacon|tests

<!--You begin by reading this data into a structure that you can use. In this case, to keep the example simple, you’ll only enforce the `avail` directives. Here is a method that gives you an associative array where the key is the user name and the value is an array of paths to which the user has write access:-->

Als erstes müssen wir die Daten in eine Struktur bringen, die wir einfach weiterverwenden können. Um das ganze Beispiel einfach zu halten, erzwingen wir hier nur die `avail` Direktive. Die folgende Funktion erzeugt ein assoziatives Array, in dem der Benutzername als Schlüssel verwendet wird. Der jeweilige Wert ist ein Array von Dateipfaden, auf die der Benutzer Zugriffsrechte besitzt.

	def get_acl_access_data(acl_file)
	  # read in ACL data
	  acl_file = File.read(acl_file).split("\n").reject { |line| line == '' }
	  access = {}
	  acl_file.each do |line|
	    avail, users, path = line.split('|')
	    next unless avail == 'avail'
	    users.split(',').each do |user|
	      access[user] ||= []
	      access[user] << path
	    end
	  end
	  access
	end

<!--On the ACL file you looked at earlier, this `get_acl_access_data` method returns a data structure that looks like this:-->

Übergibt man der Funktion `get_acl_access_data` die oben overgestellte ACL wird eine Datenstruktur zurückgegeben, die etwa folgendermaßen aussieht:

	{"defunkt"=>[nil],
	 "tpw"=>[nil],
	 "nickh"=>[nil],
	 "pjhyett"=>[nil],
	 "schacon"=>["lib", "tests"],
	 "cdickens"=>["doc"],
	 "usinclair"=>["doc"],
	 "ebronte"=>["doc"]}

<!--Now that you have the permissions sorted out, you need to determine what paths the commits being pushed have modified, so you can make sure the user who’s pushing has access to all of them.-->

Nachdem wir auf diese Weise die jeweiligen Zugriffsrechte bestimmt haben, müssen wir noch rausfinden, welche Verzeichnisse bei den gepushten Commits geändert werden. Nur so können wir sicherstellen, dass ein Benutzer die entsprechenden Zugriffsrechte für das jeweilige Verzeichnis hat.

<!--You can pretty easily see what files have been modified in a single commit with the `-\-name-only` option to the `git log` command (mentioned briefly in Chapter 2):-->

Mit Hilfe des `git log` Befehls und der Option `--name-only` findet man sehr leicht heraus, welche Dateien in einem einzelnen Commit geändert wurden (dies haben wir bereits im Kapitel 2 vorgestellt):

	$ git log -1 --name-only --pretty=format:'' 9f585d

	README
	lib/test.rb

<!--If you use the ACL structure returned from the `get_acl_access_data` method and check it against the listed files in each of the commits, you can determine whether the user has access to push all of their commits:-->

Wenn wir nun die Liste der geänderten Dateien, mit der ACL Struktur, die `get_acl_access_data` zurückliefert, vergleichen, kann man ganz einfach herausfinden, ob der Benutzer das Recht hat, alle seine Commits zu pushen:

	# only allows certain users to modify certain subdirectories in a project
	def check_directory_perms
	  access = get_acl_access_data('acl')

	  # see if anyone is trying to push something they can't
	  new_commits = `git rev-list #{$oldrev}..#{$newrev}`.split("\n")
	  new_commits.each do |rev|
	    files_modified = `git log -1 --name-only --pretty=format:'' #{rev}`.split("\n")
	    files_modified.each do |path|
	      next if path.size == 0
	      has_file_access = false
	      access[$user].each do |access_path|
	        if !access_path || # user has access to everything
	          (path.index(access_path) == 0) # access to this path
	          has_file_access = true
	        end
	      end
	      if !has_file_access
	        puts "[POLICY] You do not have access to push to #{path}"
	        exit 1
	      end
	    end
	  end
	end

	check_directory_perms

<!--Most of that should be easy to follow. You get a list of new commits being pushed to your server with `git rev-list`. Then, for each of those, you find which files are modified and make sure the user who’s pushing has access to all the paths being modified. One Rubyism that may not be clear is `path.index(access_path) == 0`, which is true if path begins with `access_path` — this ensures that `access_path` is not just in one of the allowed paths, but an allowed path begins with each accessed path.-->

Ich hoffe Du kannst dem Skript leicht folgen. Mit dem Befehl `git rev-list` erhälst Du eine Liste aller Dateien, die gepusht werden. Danach bestimmen wir für jeden Commit, welche Dateien geändert wurden und prüfen, ob der Benutzer auf diese Pfade zugreifen darf. Die Ruby-Zeile `path.index(access_path) == 0`, die vielleicht nicht so einfach zu verstehen ist, liefert true zurück, wenn path mit der gleichen Zeichenfolge beginnt, wie `access_path`. Das stellt sicher, dass `access_path` nicht nur innerhalb eines erlaubten Pfads als Zeichenfolge enthalten ist, sondern das wirklich der Anfang der Zeichenketten verglichen wird.

<!--Now your users can’t push any commits with badly formed messages or with modified files outside of their designated paths.-->

Ab jetzt haben alle Benutzer nur für die jeweils freigegebenen Verzeichnisse Zugriffsrechte und es ist sichergestellt, dass keine falsch formatierten Commit-Nachrichten gepusht werden können.

<!--#### Enforcing Fast-Forward-Only Pushes ####-->
#### Verweigern von Pushes, welche nicht einem Fast-Forward entsprechen ####

<!--The only thing left is to enforce fast-forward-only pushes. To do so, you can simply set the `receive.denyDeletes` and `receive.denyNonFastForwards` settings. But enforcing this with a hook will also work, and you can modify it to do so only for certain users or whatever else you come up with later.-->

Nun müssen wir unser System nur noch so einrichten, dass es nur Fast-Forward Push-Operationen zulässt. Man verwendet dafür die `receive.denyDeletes` und `receive.denyNonFastForwards` Konfigurationsparameter. Das gleiche Ergebnis kann man aber auch über einen Hook erreichen und diesen kann man dann so konfigurieren, dass die Regeln nur für bestimmte Benutzer gelten.

<!--The logic for checking this is to see if any commits are reachable from the older revision that aren’t reachable from the newer one. If there are none, then it was a fast-forward push; otherwise, you deny it:-->

Um herauszufinden, ob es sich um einen Fast-Forward handelt, müssen wir prüfen, ob alle Commits, die ausgehend von der letzten Revision erreichbar sind, auch von der neuen Revision aus erreichbar sind. Gibt es einen Commit auf den das nicht zutrifft, so war der Push kein Fast-Forward und wir verweigern ihn:

	# enforces fast-forward only pushes
	def check_fast_forward
	  missed_refs = `git rev-list #{$newrev}..#{$oldrev}`
	  missed_ref_count = missed_refs.split("\n").size
	  if missed_ref_count > 0
	    puts "[POLICY] Cannot push a non fast-forward reference"
	    exit 1
	  end
	end

	check_fast_forward

<!--Everything is set up. If you run `chmod u+x .git/hooks/update`, which is the file into which you should have put all this code, and then try to push a non-fast-forward reference, you’ll get something like this:-->

Das war es. Jetzt sollte alles eingerichtet sein. Wenn Du jetzt noch den Befehl `chmod u+x .git/hooks/update` für die Datei ausführst, in die Du den obigen Code eingefügt hast, und dann einen Push ausführst, welcher keinem Fast-Forward entspricht, erhälst Du in etwa folgende Ausgabe:

	$ git push -f origin master
	Counting objects: 5, done.
	Compressing objects: 100% (3/3), done.
	Writing objects: 100% (3/3), 323 bytes, done.
	Total 3 (delta 1), reused 0 (delta 0)
	Unpacking objects: 100% (3/3), done.
	Enforcing Policies...
	(refs/heads/master) (8338c5) (c5b616)
	[POLICY] Cannot push a non fast-forward reference
	error: hooks/update exited with error code 1
	error: hook declined to update refs/heads/master
	To git@gitserver:project.git
	 ! [remote rejected] master -> master (hook declined)
	error: failed to push some refs to 'git@gitserver:project.git'

<!--There are a couple of interesting things here. First, you see this where the hook starts running.-->

Lass uns die Ausgabe etwas genauer anschauen, denn sie enthält ein paar interessante Dinge. An Hand der folgenden Zeile erkennst Du, wenn der Hook gestartet wird.

	Enforcing Policies...
	(refs/heads/master) (8338c5) (c5b616)

<!--Notice that you printed that out to stdout at the very beginning of your update script. It’s important to note that anything your script prints to stdout will be transferred to the client.-->

Bitte beachte, dass wir diesen Text beim Start des `update`-Skripts auf stdout ausgegeben haben. Es ist wichtig zu wissen, dass alles was Dein Skript auf stdout ausgibt, auf den Client übertragen wird und dort ausgegeben wird.

<!--The next thing you’ll notice is the error message.-->

Als nächstes haben wir da noch die folgende Fehlermeldung.

	[POLICY] Cannot push a non fast-forward reference
	error: hooks/update exited with error code 1
	error: hook declined to update refs/heads/master

<!--The first line was printed out by you, the other two were Git telling you that the update script exited non-zero and that is what is declining your push. Lastly, you have this:-->

Die erste Zeile hast Du innerhalb des Skripts ausgegeben. Die anderen zwei stammen von Git und teilen Dir mit, dass Dein `update`-Skript einen Rückgabewert ungleich Null zurückgegeben hat und das der Push verweigert wird. Als Letztes schauen wir uns noch die folgenden Zeilen an:

	To git@gitserver:project.git
	 ! [remote rejected] master -> master (hook declined)
	error: failed to push some refs to 'git@gitserver:project.git'

<!--You’ll see a remote rejected message for each reference that your hook declined, and it tells you that it was declined specifically because of a hook failure.-->

Du siehst dort eine „remote rejected“ Nachricht für jede Referenz, die Dein Hook verweigert hat. Zusätzlich wird dort angegeben, aus welchem Grund der Push verweigert wurde. In diesem Fall hat der Hook den Push verweigert.

<!--Furthermore, if the ref marker isn’t there in any of your commits, you’ll see the error message you’re printing out for that.-->

Wenn in einem Deiner Commits die Refernez zu dem Issue-Tracking-System fehlt, wird die folgende von Dir festgelegte Fehlermeldung ausgegeben.

	[POLICY] Your message is not formatted correctly

<!--Or if someone tries to edit a file they don’t have access to and push a commit containing it, they will see something similar. For instance, if a documentation author tries to push a commit modifying something in the `lib` directory, they see-->

Auch wenn jemand in einem Commit eine Datei geändert hat, die er eigentlich nicht ändern hätte dürfen, und dann versucht diesen Commit zu pushen, wird eine ähnliche Fehlermeldung ausgegeben. Wenn zum Beispiel einer der Jungs und Mädels aus dem Dokumentationsteam versucht einen Commit zu pushen, der irgendeine Änderung im Verzeichnis `lib` enthält, wird diesen die folgende Meldung angezeigt:

	[POLICY] You do not have access to push to lib/test.rb

<!--That’s all. From now on, as long as that `update` script is there and executable, your repository will never be rewound and will never have a commit message without your pattern in it, and your users will be sandboxed.-->

Von nun an wird Dein Repository immer in einem ordentlichen Zustand sein. Niemand kann Dein Repository durcheinanderbringen oder eine Commit-Nachricht einbringen, die nicht Deinen Vorgaben entspricht. Vorausgesetzt das `update`-Skript ist vorhanden und ausführbar.

<!--### Client-Side Hooks ###-->
### Client Hooks ###

<!--The downside to this approach is the whining that will inevitably result when your users’ commit pushes are rejected. Having their carefully crafted work rejected at the last minute can be extremely frustrating and confusing; and furthermore, they will have to edit their history to correct it, which isn’t always for the faint of heart.-->

Allerdings hat unser strenger `update`-Hook auch einen Nachteil. Du kannst Dich schon mal auf das unvermeidliche Jammern Deiner Mitarbeiter einstellen, wenn diese ihre Commits nicht pushen können, weil sie verweigert werden. Wenn Du deren mit viel Mühe erstellte Arbeit in letzter Minute ablehnst, kann das für die Benutzer extrem frustrierend und verwirrend sein. Dazu kommt noch, dass diese ihre Historie ändern müssen um das ganze zu korrigieren. Und das ist nicht immer etwas für schwache Nerven.

<!--The answer to this dilemma is to provide some client-side hooks that users can use to notify them when they’re doing something that the server is likely to reject. That way, they can correct any problems before committing and before those issues become more difficult to fix. Because hooks aren’t transferred with a clone of a project, you must distribute these scripts some other way and then have your users copy them to their `.git/hooks` directory and make them executable. You can distribute these hooks within the project or in a separate project, but there is no way to set them up automatically.-->

Um dieses Dilemma zu vermeiden, ist es sinnvoll Deinen Mitarbeiter eine Handvoll Client Hooks zur Verfügung zu stellen, die darauf hinweisen, dass der gerade durchgeführte Commit wahrscheinlich vom Server verweigert wird. Auf diese Art und Weise können Deine Mitarbeiter ihre Arbeit noch korrigieren bevor sie sie einchecken. Zu diesem Zeitpunkt sind die Probleme meistens noch einfacher zu lösen. Da die Hooks während des Klonvorgangs nicht mitübertragen werden, musst Du diese auf andere Weise zur Verfügung stellen. Die Benutzer müssen diese Hooks dann auch noch in ihr `.git/hooks`-Verzeichnis kopieren und ausführbar machen. Du kannst die Hooks auch in Deinem Projekt oder in einem separaten Projekt verwalten und verteilen. Allerdings gibt es keine Möglichkeit, dass diese automatisch eingerichtet werden. Dies muss vom Nutzer selber durchgeführt werden.

<!--To begin, you should check your commit message just before each commit is recorded, so you know the server won’t reject your changes due to badly formatted commit messages. To do this, you can add the `commit-msg` hook. If you have it read the message from the file passed as the first argument and compare that to the pattern, you can force Git to abort the commit if there is no match:-->

Als erstes fangen wir damit an, die Commit-Nachrichten beim Einchecken zu prüfen. Damit ist sichergestellt, dass Dein Server die Commits und damit die Änderungen nicht ablehnt, weil sie eine falsch formatierte Commit-Nachricht enthalten. Um dies sicherzustellen, kannst Du den `commit-msg`-Hook einrichten. Wenn Du in diesem die Nachricht aus der im ersten Argument übergebenen Datei ausliest und mit Deinem Muster vergleichst, kannst Du Git dazu bringen, dass der Commit abgebrochen wird, wenn das Muster nicht passt:

	#!/usr/bin/env ruby
	message_file = ARGV[0]
	message = File.read(message_file)

	$regex = /\[ref: (\d+)\]/

	if !$regex.match(message)
	  puts "[POLICY] Your message is not formatted correctly"
	  exit 1
	end

<!--If that script is in place (in `.git/hooks/commit-msg`) and executable, and you commit with a message that isn’t properly formatted, you see this:-->

Wenn dieses Skript an der richtigen Stelle (`.git/hooks/commit-msg`) liegt und ausführbar ist und ein Commit durchgeführt wird, welcher nicht korrekt formatiert ist, wirst Du folgende Ausgabe sehen:

	$ git commit -am 'test'
	[POLICY] Your message is not formatted correctly

<!--No commit was completed in that instance. However, if your message contains the proper pattern, Git allows you to commit:-->

In diesem Fall wurde der Commit nicht durchgeführt. Wenn die Commit-Nachricht allerdings richtig formatiert ist, erlaubt Git den Commit:

	$ git commit -am 'test [ref: 132]'
	[master e05c914] test [ref: 132]
	 1 files changed, 1 insertions(+), 0 deletions(-)

<!--Next, you want to make sure you aren’t modifying files that are outside your ACL scope. If your project’s `.git` directory contains a copy of the ACL file you used previously, then the following `pre-commit` script will enforce those constraints for you:-->

Als nächstes möchten wir sicherstellen, dass Dateien nur von den Personen geändert werden, die diese auch ändern dürfen. Dazu verwenden wir wieder die Zugriffssteuerungsliste. Wenn Dein lokales `.git`-Verzeichnis eine Kopie der ACL Datei enthält, die wir vorher erstellt haben, kann das folgende `pre-commit`-Skript dafür sorgen, dass die Regeln eingehalten werden.

	#!/usr/bin/env ruby

	$user    = ENV['USER']

	# [ insert acl_access_data method from above ]

	# only allows certain users to modify certain subdirectories in a project
	def check_directory_perms
	  access = get_acl_access_data('.git/acl')

	  files_modified = `git diff-index --cached --name-only HEAD`.split("\n")
	  files_modified.each do |path|
	    next if path.size == 0
	    has_file_access = false
	    access[$user].each do |access_path|
	    if !access_path || (path.index(access_path) == 0)
	      has_file_access = true
	    end
	    if !has_file_access
	      puts "[POLICY] You do not have access to push to #{path}"
	      exit 1
	    end
	  end
	end

	check_directory_perms

<!--This is roughly the same script as the server-side part, but with two important differences. First, the ACL file is in a different place, because this script runs from your working directory, not from your Git directory. You have to change the path to the ACL file from this-->

Das vorgestellte Skript entspricht nahezu dem Skript, welches wir für den Server erstellt haben. Bis auf zwei wichtige Ausnahmen. Erstens, die ACL Datei befindet sich an einem anderen Speicherort, da das Skript ausgehend von Deinem Arbeitsverzeichnis und nicht ausgehend von Deinem Git-Verzeichnis ausgeführt wird. Aus diesem Grund muss der Pfad zu der ACL Datei von

	access = get_acl_access_data('acl')

<!--to this:-->

nach

	access = get_acl_access_data('.git/acl')

geändert werden.

<!--The other important difference is the way you get a listing of the files that have been changed. Because the server-side method looks at the log of commits, and, at this point, the commit hasn’t been recorded yet, you must get your file listing from the staging area instead. Instead of-->

Der andere wichtige Unterschied besteht darin, auf welche Art und Weise Du eine Liste der geänderten Dateien erhälst. Auf dem Server haben wir die Möglichkeit die Commits zu durchsuchen. Diese Möglichkeit haben wir beim Client nicht, da der Commit noch gar nicht ausgeführt wurde. Deswegen müssen wir die Dateien aus der Staging Area prüfen. Statt

	files_modified = `git log -1 --name-only --pretty=format:'' #{ref}`

<!--you have to use-->

musst Du folgende Zeile verwenden:

	files_modified = `git diff-index --cached --name-only HEAD`

<!--But those are the only two differences — otherwise, the script works the same way. One caveat is that it expects you to be running locally as the same user you push as to the remote machine. If that is different, you must set the `$user` variable manually.-->

Das sind die einzigen Unterschiede, ansonsten funktioniert das Skript auf die gleiche Art und Weise. Ein Nachteil besteht darin, dass davon ausgegangen wird, dass das Skript mit dem gleichen Benutzer ausgeführt wird, wie die Commits auf den Remote gepusht werden. Wenn sich diese unterscheiden, muss die `$user`-Variable manuell angepasst werden.

<!--The last thing you have to do is check that you’re not trying to push non-fast-forwarded references, but that is a bit less common. To get a reference that isn’t a fast-forward, you either have to rebase past a commit you’ve already pushed up or try pushing a different local branch up to the same remote branch.-->

Im letzten Schritt müssen wir noch prüfen, ob versucht wird einen Push durchzuführen, der keinem Fast-Forward entspricht. Das kommt normalerweise aber nicht so oft vor. Dazu muss entweder ein Rebase für Commits durchgeführt werden, die bereits gepusht wurden oder es muss ein lokaler Branch gepusht werden, dessen Name bereits auf dem Remote vorhanden ist und eine andere Historie aufweist.

<!--Because the server will tell you that you can’t push a non-fast-forward anyway, and the hook prevents forced pushes, the only accidental thing you can try to catch is rebasing commits that have already been pushed.-->

Da der Server bereits jeden Push ablehnt, der nicht einem Fast-Forward entspricht und alle Push verweigert werden, die die Historie ändern würden, kann man jetzt nur noch prüfen, ob der Benutzer einen Rebase für bereits gepushte Commits durchführt.

<!--Here is an example pre-rebase script that checks for that. It gets a list of all the commits you’re about to rewrite and checks whether they exist in any of your remote references. If it sees one that is reachable from one of your remote references, it aborts the rebase:-->

Hier möchte ich ein Beispiel `pre-rebase`-Skript vorstellen, welches diese Prüfung vornimmt. Es bestimmt eine Liste aller Commits, die neu geschrieben werden und prüft, ob diese bereits auf irgendeinem Remote vorhanden sind. Wenn dies der Fall ist, wird der Rebase abgebrochen:

	#!/usr/bin/env ruby

	base_branch = ARGV[0]
	if ARGV[1]
	  topic_branch = ARGV[1]
	else
	  topic_branch = "HEAD"
	end

	target_shas = `git rev-list #{base_branch}..#{topic_branch}`.split("\n")
	remote_refs = `git branch -r`.split("\n").map { |r| r.strip }

	target_shas.each do |sha|
	  remote_refs.each do |remote_ref|
	    shas_pushed = `git rev-list ^#{sha}^@ refs/remotes/#{remote_ref}`
	    if shas_pushed.split("\n").include?(sha)
	      puts "[POLICY] Commit #{sha} has already been pushed to #{remote_ref}"
	      exit 1
	    end
	  end
	end

<!--This script uses a syntax that wasn’t covered in the Revision Selection section of Chapter 6. You get a list of commits that have already been pushed up by running this:-->

Das Skript verwendet eine Syntax, die wir bereits im Kapitel 6.1 verwendet haben. Man erhält eine Liste aller Commits, die bereits gepusht wurden, wenn folgender Befehl ausgeführt wird:

	git rev-list ^#{sha}^@ refs/remotes/#{remote_ref}

<!--The `SHA^@` syntax resolves to all the parents of that commit. You’re looking for any commit that is reachable from the last commit on the remote and that isn’t reachable from any parent of any of the SHAs you’re trying to push up — meaning it’s a fast-forward.-->

Die `SHA^@`-Syntax gibt an, dass alle Eltern-Commits miteinbezogen werden sollen. Man sucht auf diese Art und Weise nach allen Commits, die ausgehend vom letzten auf dem Server vorhandenen Commit, erreichbar sind und nach allen Commits, die ausgehend von dem letzten zu pushenden Commit, nicht erreichbar sind.

<!--The main drawback to this approach is that it can be very slow and is often unnecessary — if you don’t try to force the push with `-f`, the server will warn you and not accept the push. However, it’s an interesting exercise and can in theory help you avoid a rebase that you might later have to go back and fix.-->

Diese Methode ist allerdings auch sehr langsam und meistens auch unnötig. Wenn ein Push ohne die Option `-f` ausgeführt wird und es sich um einen Push handelt, der keinem Fast-Forward entspricht, wird der Server eine Warnung ausgeben und den Push nicht akzeptieren. Allerdings ist diese Methode eine interessante Übung und kann zumindest in der Theorie verhindern, dass ein Rebase durchgeführt wird, der später wieder rückgängig gemacht werden müsste.

<!--## Summary ##-->
## Zusammenfassung ##

<!--You’ve covered most of the major ways that you can customize your Git client and server to best fit your workflow and projects. You’ve learned about all sorts of configuration settings, file-based attributes, and event hooks, and you’ve built an example policy-enforcing server. You should now be able to make Git fit nearly any workflow you can dream up.-->

In diesem Kapitel hast Du die wichtigsten Möglichkeiten kennengelernt, wie Du Deinen Git Client und Git Server an Deine gewohnte Arbeitsweise und Projekte anpassen kannst. Wir haben eine große Auswahl an Konfigurationsparametern, dateibasierten Attributen und Hooks vorgestellt. Außerdem haben wir einen Server eingerichtet, der dafür sorgt, dass Deine vorgegebenen Richtlinien eingehalten werden. Du solltest jetzt in der Lage sein, Git an nahezu jeden Workflow anzupassen, den Du Dir vorstellen kannst.
<!--# Git and Other Systems #-->
# Git und andere Versionsverwaltungen #

<!--The world isn’t perfect. Usually, you can’t immediately switch every project you come in contact with to Git. Sometimes you’re stuck on a project using another VCS, and many times that system is Subversion. You’ll spend the first part of this chapter learning about `git svn`, the bidirectional Subversion gateway tool in Git.-->

Leider ist die Welt nicht perfekt. Normalerweise kannst Du nicht bei jedem Deiner Projekte sofort auf Git umsteigen. Manchmal musst Du in einem Deiner Projekte irgendeine andere Versionsverwaltung nutzen, ziemlich oft ist das Subversion. Im ersten Teil dieses Kapitels werden wir das bidirektionale Gateway zwischen Git und Subversion kennenlernen: `git svn`.

<!--At some point, you may want to convert your existing project to Git. The second part of this chapter covers how to migrate your project into Git: first from Subversion, then from Perforce, and finally via a custom import script for a nonstandard importing case.-->

Manchmal kommst Du an den Zeitpunkt, zu dem Du ein bestehendes Projekt zu Git konvertieren willst. Der zweite Teil dieses Kapitels zeigt Dir, wie Du Dein Projekt zu Git migrieren kannst. Zunächst behandeln wir Subversion, dann Perforce und zum Schluss verwenden wir ein angepasstes Import-Skript, um einen nicht standard-mäßigen Import abzudecken.

<!--## Git and Subversion ##-->
## Git und Subversion ##

<!--Currently, the majority of open source development projects and a large number of corporate projects use Subversion to manage their source code. It’s the most popular open source VCS and has been around for nearly a decade. It’s also very similar in many ways to CVS, which was the big boy of the source-control world before that.-->

Gegenwärtig verwenden die meisten Open-Source-Entwicklungsprojekte und eine große Anzahl von Projekten in Unternehmen Subversion, um ihren Quellcode zu verwalten. Es ist die populärste Open-Source-Versionsverwaltung und wird seit fast einem Jahrzehnt eingesetzt. In vielen Bereichen ähnelt es CVS, das vorher der König der Versionsverwaltungen war.

<!--One of Git’s great features is a bidirectional bridge to Subversion called `git svn`. This tool allows you to use Git as a valid client to a Subversion server, so you can use all the local features of Git and then push to a Subversion server as if you were using Subversion locally. This means you can do local branching and merging, use the staging area, use rebasing and cherry-picking, and so on, while your collaborators continue to work in their dark and ancient ways. It’s a good way to sneak Git into the corporate environment and help your fellow developers become more efficient while you lobby to get the infrastructure changed to support Git fully. The Subversion bridge is the gateway drug to the DVCS world.-->

Eines der großartigen Features von Git ist die bi-direktionale Brücke zu Subversion: `git svn`. Dieses Tool ermöglicht es, Git als ganz normalen Client für einen Subversion-Server zu benutzen, sodass Du alle lokalen Features von Git nutzen kannst und Deine Änderungen dann auf einen Subversion-Server pushen kannst, so als ob Du Subversion lokal nutzen würdest. Das bedeutet, dass Du lokale Branches anlegen kannst, mergen, die staging area, rebasing, cherry-picking etc. verwenden, während Deine Kollegen weiterhin auf ihre angestaubte Art und Weise arbeiten. Das ist eine gute Gelegenheit, um Git in einem Unternehmen einzuführen und Deinen Entwickler-Kollegen dabei zu helfen, effizienter zu werden während Du an der Unterstützung arbeitest, die Infrastruktur so umzubauen, dass Git voll unterstützt wird. Die Subversion-Bridge von Git ist quasi die Einstiegsdroge in die Welt der verteilten Versionsverwaltungssysteme (distributed version control systems, DVCS).

<!--### git svn ###-->
### git svn ###

<!--The base command in Git for all the Subversion bridging commands is `git svn`. You preface everything with that. It takes quite a few commands, so you’ll learn about the common ones while going through a few small workflows.-->

Das Haupt-Kommando in Git für alle Kommandos der Subversion Bridge ist `git svn`. Dieser Befehl wird allen anderen vorangestellt. Er kennt zahlreiche Optionen, daher werden wir jetzt die gebräuchlichsten zusammen anhand von ein paar Beispielen durchspielen.

<!--It’s important to note that when you’re using `git svn`, you’re interacting with Subversion, which is a system that is far less sophisticated than Git. Although you can easily do local branching and merging, it’s generally best to keep your history as linear as possible by rebasing your work and avoiding doing things like simultaneously interacting with a Git remote repository.-->

Es ist wichtig, dass Du im Hinterkopf behältst, dass Du mit dem Befehl `git svn` mit Subversion interagierst, einem System, das nicht ganz so fortschrittlich ist wie Git. Obwohl Du auch dort ganz einfach Branches erstellen und wieder zusammenführen kannst, ist es üblicherweise am einfachsten, wenn Du die History so geradlinig wie möglich gestaltest, indem Du ein rebase für Deine Arbeit durchfürst und es vermeidest, zum Beispiel mit einem entfernten Git-Repository zu interagieren.

<!--Don’t rewrite your history and try to push again, and don’t push to a parallel Git repository to collaborate with fellow Git developers at the same time. Subversion can have only a single linear history, and confusing it is very easy. If you’re working with a team, and some are using SVN and others are using Git, make sure everyone is using the SVN server to collaborate — doing so will make your life easier.-->

Du solltest auf keinen Fall Deine History neu schreiben und dann versuchen, die Änderungen zu publizieren. Bitte schicke Deine Änderungen auch nicht zeitgleich dazu an ein anderes Git-Repository, in dem Du mit Deinen Kollegen zusammenarbeitest, die bereits Git nutzen. Subversion kennt nur eine einzige lineare History für das gesamte Repository und da kommt man schnell mal durcheinander. Wenn Du in einem Team arbeitest, in dem manche Deiner Kollegen SVN und andere schon Git nutzen, dann solltest Du sicherstellen, dass Ihr alle einen SVN-Server zur Zusammenarbeit nutzt, das macht Dein Leben deutlich einfacher.

<!--### Setting Up ###-->
### Installation ###

<!--To demonstrate this functionality, you need a typical SVN repository that you have write access to. If you want to copy these examples, you’ll have to make a writeable copy of my test repository. In order to do that easily, you can use a tool called `svnsync` that comes with more recent versions of Subversion — it should be distributed with at least 1.4. For these tests, I created a new Subversion repository on Google code that was a partial copy of the `protobuf` project, which is a tool that encodes structured data for network transmission.-->

Um dieses Feature zu demonstrieren, brauchst Du zunächst ein typisches SVN-Repository, in dem Du Schreibzugriff hast. Wenn Du die folgenden Beispiele selbst ausprobieren willst, brauchst Du eine beschreibbare Kopie meines Test-Repositories. Das geht ganz einfach mit einem kleinen Tool namens `svnsync`, das mit den letzten Subversion-Versionen (ab Version 1.4) mitgeliefert wird. Für diese Test habe ich ein neues Subversion Repository auf Google Code angelegt, das einen Teil aus dem `protobuf`-Projekts kopiert, einem Tool, das Datenstrukturen für die Übertragung über ein Netzwerk umwandelt.

<!--To follow along, you first need to create a new local Subversion repository:-->

Zunächst einmal musst Du ein neues lokales Subversion-Repository anlegen:

	$ mkdir /tmp/test-svn
	$ svnadmin create /tmp/test-svn

<!--Then, enable all users to change revprops — the easy way is to add a pre-revprop-change script that always exits 0:-->

Anschließend gibst Du allen Usern die Möglichkeit, die revprops zu ändern. Am einfachsten geht das, indem wir ein pre-revprop-change Skript erstellen, das immer 0 zurückgibt:

	$ cat /tmp/test-svn/hooks/pre-revprop-change
	#!/bin/sh
	exit 0;
	$ chmod +x /tmp/test-svn/hooks/pre-revprop-change

<!--You can now sync this project to your local machine by calling `svnsync init` with the to and from repositories.-->

Jetzt kannst Du dieses Projekt auf Deinen lokalen Rechner mit einem Aufruf von `svnsync init` synchronisieren. Als Optionen gibst Du das Ziel- und das Quell-Repository an.

	$ svnsync init file:///tmp/test-svn http://progit-example.googlecode.com/svn/

<!--This sets up the properties to run the sync. You can then clone the code by running-->

Das richtet die Properties ein, um die Synchronisierung laufen zu lassen. Nun kannst Du den Code klonen:

	$ svnsync sync file:///tmp/test-svn
	Committed revision 1.
	Copied properties for revision 1.
	Committed revision 2.
	Copied properties for revision 2.
	Committed revision 3.
	...

<!--Although this operation may take only a few minutes, if you try to copy the original repository to another remote repository instead of a local one, the process will take nearly an hour, even though there are fewer than 100 commits. Subversion has to clone one revision at a time and then push it back into another repository — it’s ridiculously inefficient, but it’s the only easy way to do this.-->

Obwohl diese Operation möglicherweise nur ein paar wenige Minuten in Anspruch nimmt, wird das Kopieren des Quell-Repositories von einem entfernten Repository in ein lokales fast eine Stunde dauern, auch wenn weniger als 100 Commits getätigt wurden. Subversion muss jede Revision einzeln klonen und sie dann in ein anderes Repository schieben – das ist zwar ziemlich ineffizient, aber für uns der einfachste Weg.

<!--### Getting Started ###-->
### Die ersten Schritte (Getting Started) ###

<!--Now that you have a Subversion repository to which you have write access, you can go through a typical workflow. You’ll start with the `git svn clone` command, which imports an entire Subversion repository into a local Git repository. Remember that if you’re importing from a real hosted Subversion repository, you should replace the `file:///tmp/test-svn` here with the URL of your Subversion repository:-->

Jetzt, da wir ein beschreibbares Subversion Repository haben, können wir mit einem typischen Workflow loslegen. Du beginnst mit dem `git svn clone`Kommando, das ein komplettes Subversion-Repository in ein lokales Git-Repository importiert. Denk daran, dass Du `file:///tmp/test-svn` im folgenden Beispiel mit der URL Deines eigenen Subversion-Repositorys ersetzt, wenn Du den Import für ein real existierendes Subversion-Repository durchführen willst.

	$ git svn clone file:///tmp/test-svn -T trunk -b branches -t tags
	Initialized empty Git repository in /Users/schacon/projects/testsvnsync/svn/.git/
	r1 = b4e387bc68740b5af56c2a5faf4003ae42bd135c (trunk)
	      A    m4/acx_pthread.m4
	      A    m4/stl_hash.m4
	...
	r75 = d1957f3b307922124eec6314e15bcda59e3d9610 (trunk)
	Found possible branch point: file:///tmp/test-svn/trunk => \
	    file:///tmp/test-svn /branches/my-calc-branch, 75
	Found branch parent: (my-calc-branch) d1957f3b307922124eec6314e15bcda59e3d9610
	Following parent with do_switch
	Successfully followed parent
	r76 = 8624824ecc0badd73f40ea2f01fce51894189b01 (my-calc-branch)
	Checked out HEAD:
	 file:///tmp/test-svn/branches/my-calc-branch r76

<!--This runs the equivalent of two commands — `git svn init` followed by `git svn fetch` — on the URL you provide. This can take a while. The test project has only about 75 commits and the codebase isn’t that big, so it takes just a few minutes. However, Git has to check out each version, one at a time, and commit it individually. For a project with hundreds or thousands of commits, this can literally take hours or even days to finish.-->

Hier werden für die angegebene URL eigentlich zwei Befehle ausgeführt, `git svn init` und anschließend `git svn fetch`. Das kann auch eine Weile dauern. Das Testprojekt hat nur etwa 75 Commits und die Codebase ist nicht so groß, daher benötigen wir nur ein paar Minuten. Da Git aber jede Version einzeln auschecken muss, kann es unter Umständen Stunden oder gar Tage dauern, bis die Ausführung des Befehls fertig ist.

<!--The `-T trunk -b branches -t tags` part tells Git that this Subversion repository follows the basic branching and tagging conventions. If you name your trunk, branches, or tags differently, you can change these options. Because this is so common, you can replace this entire part with `-s`, which means standard layout and implies all those options. The following command is equivalent:-->

Die Parameter `-T trunk -b branches -t tags` teilen Git mit, dass das Subversion-Repository den normalen Konventionen bezüglich Branching und Tagging folgt. Wenn Du Deinen Trunk, Deine Branches oder Deine Tags anders benannt hast, kannst Du diese hier anpassen. Da die Angabe aus dem Beispiel für die meisten Repositories gängig ist, kannst Du das ganze auch mit `-s` abkürzen. Diese Option steht für das Standard-Repository-Layount und umfasst die oben genannten Parameter. Der folgende Befehl ist äquivalent zum zuvor genannten:

	$ git svn clone file:///tmp/test-svn -s

<!--At this point, you should have a valid Git repository that has imported your branches and tags:-->

Jetzt solltest Du ein Git-Repository erzeugt haben, in das Deine Branches und Tags übernommen wurden:

	$ git branch -a
	* master
	  my-calc-branch
	  tags/2.0.2
	  tags/release-2.0.1
	  tags/release-2.0.2
	  tags/release-2.0.2rc1
	  trunk

<!--It’s important to note how this tool namespaces your remote references differently. When you’re cloning a normal Git repository, you get all the branches on that remote server available locally as something like `origin/[branch]` - namespaced by the name of the remote. However, `git svn` assumes that you won’t have multiple remotes and saves all its references to points on the remote server with no namespacing. You can use the Git plumbing command `show-ref` to look at all your full reference names:-->

An dieser Stelle soll die wichtige Anmerkung nicht fehlen, dass dieses Tool die Namespaces Deiner entfernten Referenzen unterschiedlich behandelt. Wenn Du ein normales Git-Repository klonst, werden alle Branches auf jenem entfernten Server für Dich lokal verfügbar gemacht, zum Beispiel als `origin/[branch]`, der Namespace entspricht dem Namen des entfernten Branches. `git svn` get allerdings davon aus, dass es nicht mehrere entfernte Repositorys gibt und speichert daher seie Referenzen auf die Bereiche entfernter Server ohne Namespaces. Du kannst das Git-Kommando `show-ref` verwenden, um Dir die vollständigen Namen aller Referenzen anzeigen zu lassen:

	$ git show-ref
	1cbd4904d9982f386d87f88fce1c24ad7c0f0471 refs/heads/master
	aee1ecc26318164f355a883f5d99cff0c852d3c4 refs/remotes/my-calc-branch
	03d09b0e2aad427e34a6d50ff147128e76c0e0f5 refs/remotes/tags/2.0.2
	50d02cc0adc9da4319eeba0900430ba219b9c376 refs/remotes/tags/release-2.0.1
	4caaa711a50c77879a91b8b90380060f672745cb refs/remotes/tags/release-2.0.2
	1c4cb508144c513ff1214c3488abe66dcb92916f refs/remotes/tags/release-2.0.2rc1
	1cbd4904d9982f386d87f88fce1c24ad7c0f0471 refs/remotes/trunk

<!--A normal Git repository looks more like this:-->

Ein normales Git-Repository sieht dagegen eher so aus:

	$ git show-ref
	83e38c7a0af325a9722f2fdc56b10188806d83a1 refs/heads/master
	3e15e38c198baac84223acfc6224bb8b99ff2281 refs/remotes/gitserver/master
	0a30dd3b0c795b80212ae723640d4e5d48cabdff refs/remotes/origin/master
	25812380387fdd55f916652be4881c6f11600d6f refs/remotes/origin/testing

<!--You have two remote servers: one named `gitserver` with a `master` branch; and another named `origin` with two branches, `master` and `testing`.-->

Du hast zwei entfernte Server: einen, der `gitserver` heißt und einen `master`-Branch beinhaltet, und einen weiteren, der `origin` heißt und zwei Branches (`master` und `testing`) enthält.

<!--Notice how in the example of remote references imported from `git svn`, tags are added as remote branches, not as real Git tags. Your Subversion import looks like it has a remote named tags with branches under it.-->

Hast Du bemerkt, dass die entfernten Referenzen, die von `git svn` im Beispiel importiert wurden, nicht als echte Git-Tags importiert wurden, sondern als entfernte Branches? Dein Subversion-Import sieht aus als besäße er einen eigenen Remote-Bereich namens `tags` und unterhalb davon einzelne Branches.

<!--### Committing Back to Subversion ###-->
### Änderungen ins Subversion-Repository committen ###

<!--Now that you have a working repository, you can do some work on the project and push your commits back upstream, using Git effectively as a SVN client. If you edit one of the files and commit it, you have a commit that exists in Git locally that doesn’t exist on the Subversion server:-->

Mit unserem funktionierenden Repository können wir nun am Projekt arbeiten und unsere Änderungen commiten; dabei nutzen wir Git als SVN-Client. Wenn Du eine der Dateien bearbeitest und sie commitest, hast Du lokal einen Commit in Git, der auf dem Subversion-Server (noch) nicht vorhanden ist:

	$ git commit -am 'Adding git-svn instructions to the README'
	[master 97031e5] Adding git-svn instructions to the README
	 1 files changed, 1 insertions(+), 1 deletions(-)

<!--Next, you need to push your change upstream. Notice how this changes the way you work with Subversion — you can do several commits offline and then push them all at once to the Subversion server. To push to a Subversion server, you run the `git svn dcommit` command:-->

Als nächsten Schritt wirst Du Deine Änderungen einchecken wollen. Dein Umgang mit Subversion wird sich dabei verändern -- Du kannst eine Vielzahl an Commits lokal durchführen und dann alle zusammen an den Subversion-Server schicken. Um Deine Änderungen auf den Subversion-Server zu pushen, verwendest Du das `git svn dcommit` Kommando:

	$ git svn dcommit
	Committing to file:///tmp/test-svn/trunk ...
	       M      README.txt
	Committed r79
	       M      README.txt
	r79 = 938b1a547c2cc92033b74d32030e86468294a5c8 (trunk)
	No changes between current HEAD and refs/remotes/trunk
	Resetting to the latest refs/remotes/trunk

<!--This takes all the commits you’ve made on top of the Subversion server code, does a Subversion commit for each, and then rewrites your local Git commit to include a unique identifier. This is important because it means that all the SHA-1 checksums for your commits change. Partly for this reason, working with Git-based remote versions of your projects concurrently with a Subversion server isn’t a good idea. If you look at the last commit, you can see the new `git-svn-id` that was added:-->

Das bündelt alle Commits, die Du auf Basis des Codes im Subversion-Server durchgeführt hast und und führt für jede Änderung ein Subversion-Commit durch. Anschließend werden Deine lokalen Git-Commits angepasst und jeder von ihnen bekommt einen eindeutigen Identifier. Das bedeutet, dass alle SHA-1 Checksums Deiner Commits verändert werden. Dies ist einer der Gründe, warum das Arbeiten mit Git-basierten entfernten Versionen Deines Projekts und zeitgleich mit einem Subversion-Server keine gute Idee ist. Wenn Du Dir den letzten Commit ansiehst, wirst Du feststellen, dass eine neue `git-svn-id` hinzugefügt wurde.

	$ git log -1
	commit 938b1a547c2cc92033b74d32030e86468294a5c8
	Author: schacon <schacon@4c93b258-373f-11de-be05-5f7a86268029>
	Date:   Sat May 2 22:06:44 2009 +0000

	    Adding git-svn instructions to the README

	    git-svn-id: file:///tmp/test-svn/trunk@79 4c93b258-373f-11de-be05-5f7a86268029

<!--Notice that the SHA checksum that originally started with `97031e5` when you committed now begins with `938b1a5`. If you want to push to both a Git server and a Subversion server, you have to push (`dcommit`) to the Subversion server first, because that action changes your commit data.-->

Die SHA-Checksum Deines ursprünglichen Commits begann mit `97031e5`, jetzt fängt sie mit `938b1a5` an. Wenn Du zugleich auf einen Git- und einen Subversion-Server pushen willst, solltest Du zunächst an den Subversion-Server pushen (`dcommit`), da diese Aktion Deine Commit-Daten verändert.

<!--### Pulling in New Changes ###-->
### Änderungen ins lokale Repository übernehmen ###

<!--If you’re working with other developers, then at some point one of you will push, and then the other one will try to push a change that conflicts. That change will be rejected until you merge in their work. In `git svn`, it looks like this:-->

Wenn Du mit anderen Entwicklern zusammenarbeitest, wirst Du irgendwann an den Punkt gelangen an dem einer von Euch Änderungen ins Repository pusht und jemand anderes versuchen wird, ebenfalls seine Änderungen zu pushen und damit einen Konflikt erzeugt. Diese Änderung wird solange zurückgewiesen bis Du die Arbeit des anderen Entwicklers mergt. Mit `git svn` sieht das so aus:

	$ git svn dcommit
	Committing to file:///tmp/test-svn/trunk ...
	Merge conflict during commit: Your file or directory 'README.txt' is probably \
	out-of-date: resource out of date; try updating at /Users/schacon/libexec/git-\
	core/git-svn line 482

<!--To resolve this situation, you can run `git svn rebase`, which pulls down any changes on the server that you don’t have yet and rebases any work you have on top of what is on the server:-->

Um diese Situation zu lösen, kannst Du `git svn rebase` laufen lassen. Das zieht alle Änderungen vom Server, die Dir noch fehlen und führt ein rebase Deiner lokalen Kopie durch (auf Basis dessen, was auf dem Server vorhanden ist).

	$ git svn rebase
	       M      README.txt
	r80 = ff829ab914e8775c7c025d741beb3d523ee30bc4 (trunk)
	First, rewinding head to replay your work on top of it...
	Applying: first user change

<!--Now, all your work is on top of what is on the Subversion server, so you can successfully `dcommit`:-->

Jetzt sind alle Deine Arbeiten auf der gleichen Ebene wie die auf dem Subversion-Server und nun kannst Du erfolgreich ein `dcommit` absetzen:

	$ git svn dcommit
	Committing to file:///tmp/test-svn/trunk ...
	       M      README.txt
	Committed r81
	       M      README.txt
	r81 = 456cbe6337abe49154db70106d1836bc1332deed (trunk)
	No changes between current HEAD and refs/remotes/trunk
	Resetting to the latest refs/remotes/trunk

<!--It’s important to remember that unlike Git, which requires you to merge upstream work you don’t yet have locally before you can push, `git svn` makes you do that only if the changes conflict. If someone else pushes a change to one file and then you push a change to another file, your `dcommit` will work fine:-->

Es ist wichtig, im Hinterkopf zu behalten, dass `git svn` sich an dieser Stelle anders als Git verhält. Git erwartet von Dir, dass Du upstream-Arbeiten, die Du lokal noch nicht hast, zunächst mergst, bevor Du pushen kannst. Dieses Vorgehen ist bei `git svn` nur nötig, wenn es Konflikte bei den Änderungen gibt. Wenn jemand anderes eine geänderte Datei gepusht hat und Du eine andere geänderte Datei pushst, wird Dein `dcommit` problemlos funktionieren:

	$ git svn dcommit
	Committing to file:///tmp/test-svn/trunk ...
	       M      configure.ac
	Committed r84
	       M      autogen.sh
	r83 = 8aa54a74d452f82eee10076ab2584c1fc424853b (trunk)
	       M      configure.ac
	r84 = cdbac939211ccb18aa744e581e46563af5d962d0 (trunk)
	W: d2f23b80f67aaaa1f6f5aaef48fce3263ac71a92 and refs/remotes/trunk differ, \
	  using rebase:
	:100755 100755 efa5a59965fbbb5b2b0a12890f1b351bb5493c18 \
	  015e4c98c482f0fa71e4d5434338014530b37fa6 M   autogen.sh
	First, rewinding head to replay your work on top of it...
	Nothing to do.

<!--This is important to remember, because the outcome is a project state that didn’t exist on either of your computers when you pushed. If the changes are incompatible but don’t conflict, you may get issues that are difficult to diagnose. This is different than using a Git server — in Git, you can fully test the state on your client system before publishing it, whereas in SVN, you can’t ever be certain that the states immediately before commit and after commit are identical.-->

Das ist darum wichtig, weil der daraus resultierende Projekt-Status auf keinem der Computer existierte als Du die Änderungen gepusht hast. Wenn die Änderungen nicht zueinander kompatibel sind aber keinen Konflikt ergeben, wirst Du Probleme bekommen, die schwer zu diagnostizieren sind. Das ist der Unterschied zu einem Git-Server — mit Git kannst Du den Zustand Deines Client-Systems komplett testen bevor Du ihn veröffentlichst, während Du bei Subversion nie sicher sein kannst, dass der Zustand direkt vor und direkt nach dem Commit identisch sind.

<!--You should also run this command to pull in changes from the Subversion server, even if you’re not ready to commit yourself. You can run `git svn fetch` to grab the new data, but `git svn rebase` does the fetch and then updates your local commits.-->

Du solltest Dieses Kommando auch ausführen um Änderungen vom Subversion-Server zu ziehen, selbst wenn Du noch nicht so weit bist, einen Commit durchzuführen. Du kannst `git svn fetch` ausführen um die neuen Daten zu besorgen aber `git svn rebase` zieht die Daten ebenfalls und aktualisiert Deine lokalen Commits.

	$ git svn rebase
	       M      generate_descriptor_proto.sh
	r82 = bd16df9173e424c6f52c337ab6efa7f7643282f1 (trunk)
	First, rewinding head to replay your work on top of it...
	Fast-forwarded master to refs/remotes/trunk.

<!--Running `git svn rebase` every once in a while makes sure your code is always up to date. You need to be sure your working directory is clean when you run this, though. If you have local changes, you must either stash your work or temporarily commit it before running `git svn rebase` — otherwise, the command will stop if it sees that the rebase will result in a merge conflict.-->

Wenn Du `git svn rebase`ab und an ausführst, stellst Du sicher, dass Dein Code immer up-to-date ist. Du musst Dir aber sicher sein, dass Dein Arbeitsverzeichnis „sauber“ ist, bevor Du den Befehl ausführst. Wenn Du lokale Änderungen hast, musst Du Deine Arbeit vor dem `git svn rebase` entweder stashen oder temporär commiten. Anderenfalls wird die Ausführung des Befehls angehalten, wenn das Rebase in einem Merge-Konflikt enden würde.

<!--### Git Branching Issues ###-->
### Probleme beim Benutzen von Branches ###

<!--When you’ve become comfortable with a Git workflow, you’ll likely create topic branches, do work on them, and then merge them in. If you’re pushing to a Subversion server via git svn, you may want to rebase your work onto a single branch each time instead of merging branches together. The reason to prefer rebasing is that Subversion has a linear history and doesn’t deal with merges like Git does, so git svn follows only the first parent when converting the snapshots into Subversion commits.-->

Wenn Du Dich an den Git-Workflow gewöhnt hast, wirst Du höchstwahrscheinlich Zweige für Deine Arbeitspakete anlegen, mit ihnen arbeiten und sie anschließend wieder mergen. Wenn Du mit `git svn` auf einen Subversion-Server pushst, führst Du am besten eine Rebase-Operation in einen einzigen Zweig durch anstatt alle Zweige zusammenzufügen. Der Grund dafür, das Rebase zu bevorzugen, liegt darin, dass Subversion eine lineare Historie hat und mit dem Merge-Operationen nicht wie Git es tut. Daher folgt `git svn` nur dem ersten Elternelement, wenn es Snapshots in Subversion-Commits umwandelt.

<!--Suppose your history looks like the following: you created an `experiment` branch, did two commits, and then merged them back into `master`. When you `dcommit`, you see output like this:-->

Angenommen, Deine Historie sieht wie folgt aus: Du hast einen Zweig mit dem Namen `experiment` angelegt, zwei Commits durchgeführt und diese dann anschließen mit `master`zusammengeführt. Führst Du nun ein `dcommit` durch, sieht die Ausgabe wie folgt aus:

	$ git svn dcommit
	Committing to file:///tmp/test-svn/trunk ...
	       M      CHANGES.txt
	Committed r85
	       M      CHANGES.txt
	r85 = 4bfebeec434d156c36f2bcd18f4e3d97dc3269a2 (trunk)
	No changes between current HEAD and refs/remotes/trunk
	Resetting to the latest refs/remotes/trunk
	COPYING.txt: locally modified
	INSTALL.txt: locally modified
	       M      COPYING.txt
	       M      INSTALL.txt
	Committed r86
	       M      INSTALL.txt
	       M      COPYING.txt
	r86 = 2647f6b86ccfcaad4ec58c520e369ec81f7c283c (trunk)
	No changes between current HEAD and refs/remotes/trunk
	Resetting to the latest refs/remotes/trunk

<!--Running `dcommit` on a branch with merged history works fine, except that when you look at your Git project history, it hasn’t rewritten either of the commits you made on the `experiment` branch — instead, all those changes appear in the SVN version of the single merge commit.-->

Das Ausführen von `dcommit` in einem Zweig mit zusammengeführter Historie funktioniert wunderbar, mit der folgenden Ausnahme: wenn Du Deinen Git-Projekthistorie anschaust, wurden die Commits, die Du im `experiment`-Zweig gemacht hast, nicht neu geschrieben — stattdessen tauchen alle diese Änderungen in der SVN-Version des einzelnen Merge-Commits auf.

<!--When someone else clones that work, all they see is the merge commit with all the work squashed into it; they don’t see the commit data about where it came from or when it was committed.-->

Wenn nun jemand anderes Deine Arbeit klont, ist alles, was er oder sie sieht, der Merge-Commit mit all Deinen Änderungen insgesamt; die Daten zu den einzelnen Commits (den Urpsrung und die Zeit, wann die Commits stattfanden) sehen sie nicht.

<!--### Subversion Branching ###-->
### Subversion-Zweige ###

<!--Branching in Subversion isn’t the same as branching in Git; if you can avoid using it much, that’s probably best. However, you can create and commit to branches in Subversion using git svn.-->

Das Arbeiten mit Zweigen in Subversion ist nicht das gleiche wie mit Zweigen in Git arbeiten; es ist wohl das beste, wenn Du es vermeiden kannst, viel damit zu arbeiten. Dennoch kannst Du mit `git svn` Zweige in Subversion anlegen und Commits darin durchführen.

<!--#### Creating a New SVN Branch ####-->
#### Einen neuen SVN-Zweig anlegen ####

<!--To create a new branch in Subversion, you run `git svn branch [branchname]`:-->

Um einen neuen Zweig in Subversion anzulegen, führst Du `git svn branch [branchname]` aus:

	$ git svn branch opera
	Copying file:///tmp/test-svn/trunk at r87 to file:///tmp/test-svn/branches/opera...
	Found possible branch point: file:///tmp/test-svn/trunk => \
	  file:///tmp/test-svn/branches/opera, 87
	Found branch parent: (opera) 1f6bfe471083cbca06ac8d4176f7ad4de0d62e5f
	Following parent with do_switch
	Successfully followed parent
	r89 = 9b6fe0b90c5c9adf9165f700897518dbc54a7cbf (opera)

<!--This does the equivalent of the `svn copy trunk branches/opera` command in Subversion and operates on the Subversion server. It’s important to note that it doesn’t check you out into that branch; if you commit at this point, that commit will go to `trunk` on the server, not `opera`.-->

Dieser Befehl macht genau das gleiche wie das `svn copy trunk branches/opera`-Kommando in Subversion und arbeitet auf dem Subversion-Server. Wichtig hierbei ist, dass Du mit Deiner Arbeit nicht automatisch in diesen Zweig wechselst: wenn Du zu diesem Zeitpunkt einen Commit durchführst, wird dieser in `trunk` auf dem Server landen, nicht im `opera`-Zweig.

<!--### Switching Active Branches ###-->
### Den aktiven Zweig wechseln ###

<!--Git figures out what branch your dcommits go to by looking for the tip of any of your Subversion branches in your history — you should have only one, and it should be the last one with a `git-svn-id` in your current branch history.-->

Git findet heraus, in welchem Zweig Deine `dcommits` abgelegt werden, indem es den tip jedes Subversion-Zweiges in Deiner Historie untersucht — Du solltest nur einen einzigen haben und es sollte der letzte mit einer `git-svn-id` in der aktuellen Historie des Zweiges sein.

<!--If you want to work on more than one branch simultaneously, you can set up local branches to `dcommit` to specific Subversion branches by starting them at the imported Subversion commit for that branch. If you want an `opera` branch that you can work on separately, you can run-->

Wenn Du gleichzeitig mit mehr als einem Zweig arbeiten willst, kannst Du lokale Zweige derart anlegen, dass ein `dcommit` für sie in bestimmte Subversion-Zweige durchgeführt wird. Dazu startest Du diese beim importierten Subversion-Commit für diesen Zweig. Wenn Du einen `opera`-Zweig haben willst, an dem Du separat arbeiten kannst, führst Du

	$ git branch opera remotes/opera

<!--Now, if you want to merge your `opera` branch into `trunk` (your `master` branch), you can do so with a normal `git merge`. But you need to provide a descriptive commit message (via `-m`), or the merge will say "Merge branch opera" instead of something useful.-->

aus. Wenn Du jetzt Deinen `opera`-Zweig in `trunk` (Deinen `master`-Zweig) zusammenführen willst , kannst Du das mit einem normalen `git merge` machen. Du solltest allerdings eine aussagekräftige Commit-Beschreibung angeben (mit `-m`) oder sie wird statt etwas Sinnvollem „Merge branch opera“ lauten.

<!--Remember that although you’re using `git merge` to do this operation, and the merge likely will be much easier than it would be in Subversion (because Git will automatically detect the appropriate merge base for you), this isn’t a normal Git merge commit. You have to push this data back to a Subversion server that can’t handle a commit that tracks more than one parent; so, after you push it up, it will look like a single commit that squashed in all the work of another branch under a single commit. After you merge one branch into another, you can’t easily go back and continue working on that branch, as you normally can in Git. The `dcommit` command that you run erases any information that says what branch was merged in, so subsequent merge-base calculations will be wrong — the dcommit makes your `git merge` result look like you ran `git merge -\-squash`. Unfortunately, there’s no good way to avoid this situation — Subversion can’t store this information, so you’ll always be crippled by its limitations while you’re using it as your server. To avoid issues, you should delete the local branch (in this case, `opera`) after you merge it into trunk.-->

Behalte im Hinterkopf, dass dieses Vorgehen kein normaler Git-Merge-Commit ist, auch wenn Du den `git merge`-Befehl verwendes und das Zusammenführen wahrscheinlich wesentlich einfacher ist als es in Subversion gewesen wäre (weil Git automatisch die passende Basis für das Zusammenführen der Zweige für Dich herausfindet=. Du musst diese Daten  zurück auf den Subversion-Server schieben, der nicht mit Commits umgehen kann, die mehr als ein Elternelement haben; all Deine Änderungen aus einem anderen Zweig werden in diesem einen Commit zusammengepresst, wenn Du die Änderungen hochschiebst. Nachdem Du einen Zweig mit einem anderen zusammengefügt hast, kannst Du nicht einfach zurückgehen und mit der Arbeit an diesem Zweig weitermachen, wie Du das normalerweise in Git machen würdest. Wenn Du das `dcommit`-Kommando ausführst, löscht es jegliche Information darüber, welcher Zweig hier hineingefügt wurde und als Folge dessen werden künftige merge-base-Berechnungen falsche sein — die `dcommit`-Operation lässt Dein `git merge`-Ergebnis so aussehen als ob Du `git merge --squash` verwendet hättest.
Unglücklicherweise gibt es kein ideales Mittel, diese Situation zu vermeiden — Subversion kann diese Information einfach nicht speichern, daher wirst Du immer unter seinen beschränkten Möglichkeiten zu leiden haben, so lange Du es als Server verwendest. Um diese Probleme zu vermeiden, solltest Du den lokalen Zweig (in unserem Beispiel `opera`) löschen, nachdem Du ihn mit dem `trunk` zusammengeführt hast.

<!--### Subversion Commands ###-->
### Subversion Befehle ###

<!--The `git svn` toolset provides a number of commands to help ease the transition to Git by providing some functionality that’s similar to what you had in Subversion. Here are a few commands that give you what Subversion used to.-->

Das `git svn`-Werkzeug bietet eine ganze Reihe von Befehlen an, die Dir helfen, den Übergang zu Git zu vereinfachen, indem sie einige Funktionen bereitstellen, die jeden ähneln, die Du bereits in Subversion kanntest. Hier sind ein paar Befehle, die Dir solche Funktionen bereitstellen wie das Subversion früher für Dich tat:

<!--#### SVN Style History ####-->
#### Historie im SVN-Stil ####

<!--If you’re used to Subversion and want to see your history in SVN output style, you can run `git svn log` to view your commit history in SVN formatting:-->

Wenn Du an Subversion gewöhnt bist und Deine Historie so sehen möchtest, wie SVN sie ausgeben würde, kannst Du `git svn log` ausführen, um Deine Commit-Historie in der SVN-Formatierung anzusehen:

	$ git svn log
	------------------------------------------------------------------------
	r87 | schacon | 2009-05-02 16:07:37 -0700 (Sat, 02 May 2009) | 2 lines

	autogen change

	------------------------------------------------------------------------
	r86 | schacon | 2009-05-02 16:00:21 -0700 (Sat, 02 May 2009) | 2 lines

	Merge branch 'experiment'

	------------------------------------------------------------------------
	r85 | schacon | 2009-05-02 16:00:09 -0700 (Sat, 02 May 2009) | 2 lines

	updated the changelog

<!--You should know two important things about `git svn log`. First, it works offline, unlike the real `svn log` command, which asks the Subversion server for the data. Second, it only shows you commits that have been committed up to the Subversion server. Local Git commits that you haven’t dcommited don’t show up; neither do commits that people have made to the Subversion server in the meantime. It’s more like the last known state of the commits on the Subversion server.-->

Du solltest zwei wichtige Dinge über `git svn log` wissen. Erstens: es arbeitet offline, im Gegensatz zum echten `svn log`-Befehl, der den Subversion-Server nach den Daten fragt. Zweitens: es zeigt Dir nur Commits an, die auf den Subversion-Server committet wurden. Lokale Git-Commits die Du nicht mit `dcommit` bestätigt hast, werden nicht aufgeführt, genausowenig wie Commits, die andere in der Zwischenzeit auf dem Subversion-Server gemacht haben. Die Ausgabe zeigt Dir eher den letzten bekannten Zustand der Commits auf dem Subversion-Server.

<!--#### SVN Annotation ####-->
#### SVN Vermerke (SVN Annotation) ####

<!--Much as the `git svn log` command simulates the `svn log` command offline, you can get the equivalent of `svn annotate` by running `git svn blame [FILE]`. The output looks like this:-->

Genauso wie das Kommando `git svn log` den `svn log`-Befehl offline simuliert kannst Du das Pendant zu `svn annotate` mit dem Befehl `git svn blame [FILE]` ausführen. Die Ausgabe sieht so aus:

	$ git svn blame README.txt
	 2   temporal Protocol Buffers - Google's data interchange format
	 2   temporal Copyright 2008 Google Inc.
	 2   temporal http://code.google.com/apis/protocolbuffers/
	 2   temporal
	22   temporal C++ Installation - Unix
	22   temporal =======================
	 2   temporal
	79    schacon Committing in git-svn.
	78    schacon
	 2   temporal To build and install the C++ Protocol Buffer runtime and the Protocol
	 2   temporal Buffer compiler (protoc) execute the following:
	 2   temporal

<!--Again, it doesn’t show commits that you did locally in Git or that have been pushed to Subversion in the meantime.-->

Auch dieser Befehl zeigt die Commits nicht an, die Du lokal getätigt hast, genauso wenig wie jene, die in der Zwischenzeit zum Subversion-Server übertragen wurden.

<!--#### SVN Server Information ####-->
#### SVN-Server-Informationen ####

<!--You can also get the same sort of information that `svn info` gives you by running `git svn info`:-->

Die selben Informationen wie bei `svn info` bekommst Du, wenn Du `git svn info` ausführst:

	$ git svn info
	Path: .
	URL: https://schacon-test.googlecode.com/svn/trunk
	Repository Root: https://schacon-test.googlecode.com/svn
	Repository UUID: 4c93b258-373f-11de-be05-5f7a86268029
	Revision: 87
	Node Kind: directory
	Schedule: normal
	Last Changed Author: schacon
	Last Changed Rev: 87
	Last Changed Date: 2009-05-02 16:07:37 -0700 (Sat, 02 May 2009)

<!--This is like `blame` and `log` in that it runs offline and is up to date only as of the last time you communicated with the Subversion server.-->

Genauso wie `blame` und `log` läuft die Ausführung dieses Befehls offline ab und ist nur so aktuell wie zu dem Zeitpunkt, zu dem Du das letzte Mal mit dem Subversion-Server verbunden warst.

<!--#### Ignoring What Subversion Ignores ####-->
#### Ignorieren, was Subversion ignoriert ####

<!--If you clone a Subversion repository that has `svn:ignore` properties set anywhere, you’ll likely want to set corresponding `.gitignore` files so you don’t accidentally commit files that you shouldn’t. `git svn` has two commands to help with this issue. The first is `git svn create-ignore`, which automatically creates corresponding `.gitignore` files for you so your next commit can include them.-->

Wenn Du ein Subversion-Repository klonst, das irgendwo `svn:ignore` Eigenschaften definiert hat, wirst Du die `.gitignore`-Dateien wahrscheinlich entsprechend setzen, damit Du nicht aus Versehen Dateien committest, bei denen Du das besser nicht tun solltest. `git svn` kennt zwei Befehle, um Dir bei diesem Problem zu helfen. Der erste ist `git svn create-ignore`, der automatisch entsprechende `.gitignore`-Dateien für Dich anlegt, sodass Dein nächster Commit diese beinhalten kann.

<!--The second command is `git svn show-ignore`, which prints to stdout the lines you need to put in a `.gitignore` file so you can redirect the output into your project exclude file:-->

Der zweite Befehl ist `git svn show-ignore`, der Dir diejenigen Zeilen auf stdout ausgibt, die Du in eine `.gitignore`-Datei einfügen musst. So kannst Du die Ausgabe des Befehls direkt in die Ausnahmedatei umleiten:

	$ git svn show-ignore > .git/info/exclude

<!--That way, you don’t litter the project with `.gitignore` files. This is a good option if you’re the only Git user on a Subversion team, and your teammates don’t want `.gitignore` files in the project.-->

Auf diese Weise müllst Du Dein Projekt nicht mit `.gitignore`-Dateien zu. Das ist eine gute Wahl wenn Du der einzige Git-Benutzer in Deinem Team bist (alle anderen benutzen Subversion) und Deine Kollegen keine `.gitignore`-Dateien im Projekt haben wollen.

<!--### Git-Svn Summary ###-->
### Zusammenfassung von Git-Svn ###

<!--The `git svn` tools are useful if you’re stuck with a Subversion server for now or are otherwise in a development environment that necessitates running a Subversion server. You should consider it crippled Git, however, or you’ll hit issues in translation that may confuse you and your collaborators. To stay out of trouble, try to follow these guidelines:-->

Die `git svn`-Werkzeuge sind sehr nützlich, wenn Du derzeit (noch) an einen Subversion-Server gebunden bist oder Dich anderweitig in einer Entwicklungsumgebung befindest, die nicht auf einen Subversion-Server verzichten kann. Wie auch immer: Du solltest es als eine Art gestutztes Git ansehen. Anderenfalls läufst Du Gefahr, Dich und Deine Kollegen durcheinander zu bringen. Um dieses Kliff zu umschiffen, solltest Du folgende Richtlinien befolgen:

<!--* Keep a linear Git history that doesn’t contain merge commits made by `git merge`. Rebase any work you do outside of your mainline branch back onto it; don’t merge it in.-->
<!--* Don’t set up and collaborate on a separate Git server. Possibly have one to speed up clones for new developers, but don’t push anything to it that doesn’t have a `git-svn-id` entry. You may even want to add a `pre-receive` hook that checks each commit message for a `git-svn-id` and rejects pushes that contain commits without it.-->

* Versuch, eine „geradlinige“ Git-Historie zu führen, die keine von `git merge` durchgeführten Merges enthält. Alle Arbeiten, die Du außerhalb des Hauptzweiges durchführst, solltest Du mit `rebase` in ihn aufnehmen anstatt sie zu mit `merge` zusammenzuführen.
* Setz keinen zusätzlichen, externen Git-Server auf, mit dem Du arbeiten möchtest. Du kannst einen aufsetzen um die Klone für neue Entwickler zu beschleunigen, aber Du solltest keine Änderungen dorthin pushen, die keine `git-svn-id`-Einträge haben. Du solltest vielleicht sogar darüber nachdenken, einen `pre-receive`-Hook einzusetzen, der jede Commit-Nachricht auf eine `git-svn-id` prüft und bestimmte Pushes ablehnt, bei denen diese IDs fehlt.

<!--If you follow those guidelines, working with a Subversion server can be more bearable. However, if it’s possible to move to a real Git server, doing so can gain your team a lot more.-->

Wenn Du diese Ratschläge befolgst, werden sie die Arbeit mit dem Subversion-Server erträglich machen. Wenn es Dir irgendwie möglich ist, solltest Du trotzdem zu einem echten Git-Server umziehen, denn davon profitiert Dein Team wesentlich deutlicher.

<!--## Migrating to Git ##-->
## Zu Git umziehen ##

<!--If you have an existing codebase in another VCS but you’ve decided to start using Git, you must migrate your project one way or another. This section goes over some importers that are included with Git for common systems and then demonstrates how to develop your own custom importer.-->

Wenn Du bereits Quellcode in einer anderen Versionsverwaltung abgelegt hast, aber Dich nun entschieden hast, von nun an Git zu benutzen, musst Du Dein Projekt so oder so umziehen. Für geläufige Systeme bringt Git einige Importer mit. Anschließend lernen wir, wie Du Deinen eigenen, angepassten Importer entwickeln kann. All das wird im folgenden Abschnitt behandelt.

<!--### Importing ###-->
### Import ###

<!--You’ll learn how to import data from two of the bigger professionally used SCM systems — Subversion and Perforce — both because they make up the majority of users I hear of who are currently switching, and because high-quality tools for both systems are distributed with Git.-->

Jetzt ist es an der Zeit zu lernen, wie Du Daten aus zwei der am meisten benutzten (professionellen) SCM-Systeme importieren kannst: Subversion und Perforce. Ein Großteil der Benutzer, die gegenwärtig zu Git umziehen, arbeiten mit einem von diesen beiden Systemen. Außerdem liefert Git für beide jeweils hochprofessionelle Werkzeuge für den Import mit.

<!--### Subversion ###-->
### Subversion ###

<!--If you read the previous section about using `git svn`, you can easily use those instructions to `git svn clone` a repository; then, stop using the Subversion server, push to a new Git server, and start using that. If you want the history, you can accomplish that as quickly as you can pull the data out of the Subversion server (which may take a while).-->

Wenn Du die letzten Abschnitte über `git svn` gelesen hast, kannst Du diese Anleitungen ganz einfach benutzen um mit `git svn clone` ein Repository zu klonen. Anschließend stoppst den Subversion-Server, führst einen Push auf den neuen Git-Server durch und beginnst ihn zu benutzen. Wenn Du an die Historie ran willst, kannst Du das genausoschnell erreichen als ob Du die Daten aus dem Subversion-Server beziehen würdest (was eine Weile dauern könnte).

<!--However, the import isn’t perfect; and because it will take so long, you may as well do it right. The first problem is the author information. In Subversion, each person committing has a user on the system who is recorded in the commit information. The examples in the previous section show `schacon` in some places, such as the `blame` output and the `git svn log`. If you want to map this to better Git author data, you need a mapping from the Subversion users to the Git authors. Create a file called `users.txt` that has this mapping in a format like this:-->

Trotzdem ist der Import nicht perfekt. Und weil das ziemlich lange dauern wird, kannst Du es auch gleich richtig machen. Das erste Problem sind die Informationen über die Autoren. In Subversion besitzt jede Person, die mit dem System arbeitet, einen eigenen User-Account, der in den Commit-Informationen aufgezeichnet wird. Die Beispiele in den vorherigen Abschnitten zeigten dafür manchmal `schacon` an, wie beispielsweise bei der Ausgabe von `blame` und bei `git svn log`. Wenn Du dies näher an die Autoren-Daten von Git binden willst, musst Du Mapping-Informationen für die Subversion-Benutzer und die Git-Autoren anlegen. Erstelle eine Datei mit dem Namen `users.txt`, die folgendes Mapping-Format verwendet:

	schacon = Scott Chacon <schacon@geemail.com>
	selse = Someo Nelse <selse@geemail.com>

<!--To get a list of the author names that SVN uses, you can run this:-->

Um eine Liste der Namen der Autoren bekommen, die SVN benutzen, kannst Du folgendes Kommando ausführen:

	$ svn log ^/ --xml | grep -P "^<author" | sort -u | \
	      perl -pe 's/<author>(.*?)<\/author>/$1 = /' > users.txt

<!--That gives you the log output in XML format — you can look for the authors, create a unique list, and then strip out the XML. (Obviously this only works on a machine with `grep`, `sort`, and `perl` installed.) Then, redirect that output into your users.txt file so you can add the equivalent Git user data next to each entry.-->

Dies erzeugt Dir die Log-Ausgabe im XML-Format — Du suchst damit nach den Autoren, erzeugst eine Liste ohne doppelte Einträge und wirfst anschließend das überflüssige XML weg. Leite anschließend die Ausgabe in die Datei `users.txt` um, sodass Du jedem Eintrag den entsprechenden Git-Benutzer zuordnen kannst.

<!--You can provide this file to `git svn` to help it map the author data more accurately. You can also tell `git svn` not to include the metadata that Subversion normally imports, by passing `-\-no-metadata` to the `clone` or `init` command. This makes your `import` command look like this:-->

Du kannst diese Datei dann `git svn` zur Verfügung stellen um das Tool dabei zu unterstützen, die Autoreninformationen besser zu mappen. Du kannst `git svn` ebenfalls mitteilen, dass es die Metadaten nicht einbeziehen soll, die Subversion normalerweise importiert, indem Du dem `clone` oder `init` Kommando die `--no-metadata`-Option mitgibst.

	$ git svn clone http://my-project.googlecode.com/svn/ \
	      --authors-file=users.txt --no-metadata -s my_project

<!--Now you should have a nicer Subversion import in your `my_project` directory. Instead of commits that look like this-->

Jetzt solltest Du einen hübscheren Subversion-Import in Deinem `my_project`-Verzeichnis haben. Statt eines Commit, die so aussehen:

	commit 37efa680e8473b615de980fa935944215428a35a
	Author: schacon <schacon@4c93b258-373f-11de-be05-5f7a86268029>
	Date:   Sun May 3 00:12:22 2009 +0000

	    fixed install - go to trunk

	    git-svn-id: https://my-project.googlecode.com/svn/trunk@94 4c93b258-373f-11de-
	    be05-5f7a86268029

<!--they look like this:-->

sehen sie jetzt so aus:

	commit 03a8785f44c8ea5cdb0e8834b7c8e6c469be2ff2
	Author: Scott Chacon <schacon@geemail.com>
	Date:   Sun May 3 00:12:22 2009 +0000

	    fixed install - go to trunk

<!--Not only does the Author field look a lot better, but the `git-svn-id` is no longer there, either.-->

Nicht nur das Autoren-Feld sieht jetzt wesentlich besser aus. Auch die `git-svn-id` wird jetzt nicht mehr gebraucht.

<!--You need to do a bit of `post-import` cleanup. For one thing, you should clean up the weird references that `git svn` set up. First you’ll move the tags so they’re actual tags rather than strange remote branches, and then you’ll move the rest of the branches so they’re local.-->

Nach dem Import musst Du noch ein wenig aufräumen. Dafür solltest Du all die merkwürdigen Referenzen säubern, die `git svn` angelegt hat. Zuerst verschiebst Du die Tags, damit sie tatsächliche Git-Tags sind statt merkwürdigen Remote-Zweigen. Anschließend verschieben wir den Rest der Zweige, sodass sie lokale Zweige werden.

<!--To move the tags to be proper Git tags, run-->

Um die Tags so zu verschieben, dass sie echte Git-Tags werden, führst Du folgenden Befehl aus:

	$ git for-each-ref refs/remotes/tags | cut -d / -f 4- | grep -v @ | while read tagname; do git tag "$tagname" "tags/$tagname"; git branch -r -d "tags/$tagname"; done

<!--This takes the references that were remote branches that started with `tag/` and makes them real (lightweight) tags.-->

Das nimmt die Referenzen, die vorher Remote-Zweige waren, die mit `tag/` begonnen haben und macht aus ihnen echte (leichtgewichtige) Tags.

<!--Next, move the rest of the references under `refs/remotes` to be local branches:-->

Als nächstes verschieben wir den Rest der Referenzen aus `refs/remotes` und machen lokale Zweige daraus:

	$ git for-each-ref refs/remotes | cut -d / -f 3- | grep -v @ | while read branchname; do git branch "$branchname" "refs/remotes/$branchname"; git branch -r -d "$branchname"; done

<!--Now all the old branches are real Git branches and all the old tags are real Git tags. The last thing to do is add your new Git server as a remote and push to it. Here is an example of adding your server as a remote:-->

Jetzt sind alle alten Zweige richtige Git-Zweige geworden und alle alten Tags sind echte Git-Tags. Als letztes müssen wir den neuen Git-Server noch als entfernten Server einrichten und unsere Änderungen zu ihm pushen. Da wir alle Zweige und Tags einbeziehen wollen, kannst Du diesen Befehl verwenden:

	$ git remote add origin git@my-git-server:myrepository.git

<!--Because you want all your branches and tags to go up, you can now run this:-->

	$ git push origin --all
	$ git push origin --tags

<!--All your branches and tags should be on your new Git server in a nice, clean import.-->

All Deine Zweige und Tags sollten jetzt in Deinem neuen Git-Server in einem schicken, sauberen Import vorhanden sein.

<!--### Perforce ###-->
### Perforce ###

<!--The next system you’ll look at importing from is Perforce. A Perforce importer is also distributed with Git. If you have a version of Git earlier than 1.7.11, then the importer is only available in the `contrib` section of the source code. In that case you must get the Git source code, which you can download from git.kernel.org:-->

Das nächste System, dass wir zum Importieren anschauen werden, ist Perforce. Ein Import-Werkzeug für Perforce wird ebenfalls mit Git mitgeliefert, allerdings nur im `contrib`-Bereich des Quellcodes — es ist nicht wie `git svn` standardmäßig verfügbar. Um es auszuführen, musst Du den Git-Quellcode von `git.kernel.org` herunterladen:

	$ git clone git://git.kernel.org/pub/scm/git/git.git
	$ cd git/contrib/fast-import

<!--In this `fast-import` directory, you should find an executable Python script named `git-p4`. You must have Python and the `p4` tool installed on your machine for this import to work. For example, you’ll import the Jam project from the Perforce Public Depot. To set up your client, you must export the P4PORT environment variable to point to the Perforce depot:-->

In diesem `fast-import` Verzeichnis wirst Du ein ausführbares Python-Skript mit dem Namen `git-p4` finden. Du musst Python sowie das `p4`-Werkzeug auf Deiner Maschine installiert haben, damit der Import klappt. Als Beispiel werden wir das Jam-Projekt aus dem Perforce Public Depot verwenden. Um den Client einzurichten, musst Du die P4PORT-Umgebungsvariable exportieren und sie auf das Perforce-Depot einstellen:

	$ export P4PORT=public.perforce.com:1666

<!--Run the `git-p4 clone` command to import the Jam project from the Perforce server, supplying the depot and project path and the path into which you want to import the project:-->

Führe den `git-p4 clone`-Befehl aus, um das Jam-Projekt aus dem Perforce-Server zu importieren. Dazu gibst Du den Depot- und Projekt-Pfad sowie den Pfad an, in den Du das Projekt importieren willst:

	$ git-p4 clone //public/jam/src@all /opt/p4import
	Importing from //public/jam/src@all into /opt/p4import
	Reinitialized existing Git repository in /opt/p4import/.git/
	Import destination: refs/remotes/p4/master
	Importing revision 4409 (100%)

<!--If you go to the `/opt/p4import` directory and run `git log`, you can see your imported work:-->

Wenn Du zum `/opt/p4import`-Verzeichnis wechselst und dann `git log` ausführst, kannst Du sehen, dass Dein Import funktioniert hat:

	$ git log -2
	commit 1fd4ec126171790efd2db83548b85b1bbbc07dc2
	Author: Perforce staff <support@perforce.com>
	Date:   Thu Aug 19 10:18:45 2004 -0800

	    Drop 'rc3' moniker of jam-2.5.  Folded rc2 and rc3 RELNOTES into
	    the main part of the document.  Built new tar/zip balls.

	    Only 16 months later.

	    [git-p4: depot-paths = "//public/jam/src/": change = 4409]

	commit ca8870db541a23ed867f38847eda65bf4363371d
	Author: Richard Geiger <rmg@perforce.com>
	Date:   Tue Apr 22 20:51:34 2003 -0800

	    Update derived jamgram.c

	    [git-p4: depot-paths = "//public/jam/src/": change = 3108]

<!--You can see the `git-p4` identifier in each commit. It’s fine to keep that identifier there, in case you need to reference the Perforce change number later. However, if you’d like to remove the identifier, now is the time to do so — before you start doing work on the new repository. You can use `git filter-branch` to remove the identifier strings en masse:-->

Du kannst die `git-p4`-ID bei jedem Commit sehen. Es ist OK, diese ID hier zu behalten, falls Du später noch mal einen Bezug zu der Perforce-Änderung herstellen musst. Falls Du die ID entfernen willst, ist jetzt Zeit dazu — bevor Du mit der Arbeit an dem neuen Repository beginnst. Du kannst `git filter-branch` benutzen um all die IDs zu entfernen:

	$ git filter-branch --msg-filter '
	        sed -e "/^\[git-p4:/d"
	'
	Rewrite 1fd4ec126171790efd2db83548b85b1bbbc07dc2 (123/123)
	Ref 'refs/heads/master' was rewritten

<!--If you run `git log`, you can see that all the SHA-1 checksums for the commits have changed, but the `git-p4` strings are no longer in the commit messages:-->

Wenn Du `git log` ausführst, kannst Du alle SHA1-Prüfsummen für jene Commits sehen, die sich geändert haben, aber die `git-p4`-Zeichenketten sind nicht mehr in den Commit-Nachrichten vorhanden.

	$ git log -2
	commit 10a16d60cffca14d454a15c6164378f4082bc5b0
	Author: Perforce staff <support@perforce.com>
	Date:   Thu Aug 19 10:18:45 2004 -0800

	    Drop 'rc3' moniker of jam-2.5.  Folded rc2 and rc3 RELNOTES into
	    the main part of the document.  Built new tar/zip balls.

	    Only 16 months later.

	commit 2b6c6db311dd76c34c66ec1c40a49405e6b527b2
	Author: Richard Geiger <rmg@perforce.com>
	Date:   Tue Apr 22 20:51:34 2003 -0800

	    Update derived jamgram.c

<!--Your import is ready to push up to your new Git server.-->

Dein Import ist jetzt so weit, dass Du ihn auf den neuen Git-Server pushen kannst.

<!--### A Custom Importer ###-->
### Ein Import-Tool im Eigenbau ###

<!--If your system isn’t Subversion or Perforce, you should look for an importer online — quality importers are available for CVS, Clear Case, Visual Source Safe, even a directory of archives. If none of these tools works for you, you have a rarer tool, or you otherwise need a more custom importing process, you should use `git fast-import`. This command reads simple instructions from stdin to write specific Git data. It’s much easier to create Git objects this way than to run the raw Git commands or try to write the raw objects (see Chapter 9 for more information). This way, you can write an import script that reads the necessary information out of the system you’re importing from and prints straightforward instructions to stdout. You can then run this program and pipe its output through `git fast-import`.-->

Wenn die Versionsverwaltung, die Du verwendest, nicht Subversion oder Perforce ist, solltest Du zunächst einmal online nach einem Import-Tool suchen — gute Import-Tools sind für CVS, Clear Case, Visual Source Sage und sogar für ein Verzeichnis mit Archiven verfügbar. Wenn für Deinen Anwendungsfall keines dieser Werkzeuge passt, Du eine fast schon ausgestorbene Versionsverwaltung verwendest oder Du aus irgendeinem anderen Grund ein angepassteres Vorgehen brauchst, dann solltest Du `git fast-import` verwenden. Dieser Befehl nimmt einfache Anweisungen von `stdin` entgegen um entsprechende Git-Daten zu schreiben. Es ist viel einfacher Git-Objekte auf diese Art zu erzeugen als die blanken Git-Kommandos zu verwenden oder zu versuchen, die Roh-Objekte zu schreiben (weitere Informationen findest Du in Kapitel 9).

<!--To quickly demonstrate, you’ll write a simple importer. Suppose you work in current, you back up your project by occasionally copying the directory into a time-stamped `back_YYYY_MM_DD` backup directory, and you want to import this into Git. Your directory structure looks like this:-->

Um das kurz zu zeigen, schreiben wir einen einfachen Importer. Nehmen wir an, Du arbeitest im Verzeichnis `current` und führst ab und an ein Backup durch, indem Du dieses Verzeichnis in ein Backup-Verzeichnis kopierst und ihm einen anderen Namen mit einem Zeitstempel, z. B. `back_YYYY_MM_DD`, verpasst. Diese Struktur wollen wir jetzt in Git importieren. Dein Verzeichnis sieht also so aus:

	$ ls /opt/import_from
	back_2009_01_02
	back_2009_01_04
	back_2009_01_14
	back_2009_02_03
	current

<!--In order to import a Git directory, you need to review how Git stores its data. As you may remember, Git is fundamentally a linked list of commit objects that point to a snapshot of content. All you have to do is tell `fast-import` what the content snapshots are, what commit data points to them, and the order they go in. Your strategy will be to go through the snapshots one at a time and create commits with the contents of each directory, linking each commit back to the previous one.-->

Damit wir ein Git-Verzeichnis importieren können, müssen wir uns zunächst noch einmal anschauen, wie Git seine Daten speichert. Wie Du Dich vielleicht erinnerst, ist Git im Grundsatz eine verlinkte Liste von Commit-Objekten die auf eine Momentaufnahme (Snapshots) des Inhalts zeigt. Jetzt musst Du nur noch `fast-import` mitteilen, was dieses Snapshots sind, welche Commit-Daten zu ihnen zeigen und die Reihenfolge, in die sie gehören. Deine Strategie wir es sein, einen nach dem anderen durch die Snapshots zu gehen und Commits mit dem Inhalt eines jeden Verzeichnisses zu erzeigen und jeden dieser Commits anschließend mit dem vorherigen zu verknüpfen.

<!--As you did in the "An Example Git Enforced Policy" section of Chapter 7, we’ll write this in Ruby, because it’s what I generally work with and it tends to be easy to read. You can write this example pretty easily in anything you’re familiar with — it just needs to print the appropriate information to stdout. And, if you are running on Windows, this means you’ll need to take special care to not introduce carriage returns at the end your lines — git fast-import is very particular about just wanting line feeds (LF) not the carriage return line feeds (CRLF) that Windows uses.-->

Wie wir das schon im Abschnitt „(...)“ in Kapitel 7 getan haben, programmieren wir diese Lösung in Ruby, weil es die Sprache ist, mit der ich normalerweise arbeite und weil sie recht einfach zu lesen ist. Du kannst das Beispiel in so ziemlich jeder Sprache schreiben, mit der Du vertraut bist — es muss nur die passenden Informationen nach `stdout` schreiben.

<!--To begin, you’ll change into the target directory and identify every subdirectory, each of which is a snapshot that you want to import as a commit. You’ll change into each subdirectory and print the commands necessary to export it. Your basic main loop looks like this:-->

Zu Beginn musst Du in das Zielverzeichnis wechseln und jedes Unterverzeichnis identifizieren, das ein Snapshot ist, den Du als Commit importieren willst. Du wirst in jedes dieser Unterverzeichnisse wechseln und den entsprechenden Befehl auszugeben um es zu exportieren. Deine Schleife wird etwa so aussehen:

	last_mark = nil

	# loop through the directories
	Dir.chdir(ARGV[0]) do
	  Dir.glob("*").each do |dir|
	    next if File.file?(dir)

	    # move into the target directory
	    Dir.chdir(dir) do
	      last_mark = print_export(dir, last_mark)
	    end
	  end
	end

<!--You run `print_export` inside each directory, which takes the manifest and mark of the previous snapshot and returns the manifest and mark of this one; that way, you can link them properly. "Mark" is the `fast-import` term for an identifier you give to a commit; as you create commits, you give each one a mark that you can use to link to it from other commits. So, the first thing to do in your `print_export` method is generate a mark from the directory name:-->

Du führst `print_export` für jedes Verzeichnis aus. Das nimmt das Manifest und die Markierung des letzten Snapshots entgegen und gibt das Manifest und die Markierung des aktuellen zurück; auf diese Weise kannst Du sie passend verlinken. „Mark“ ist der `fast-import`-Begriff für eine ID, die Du einem Commit gibst. Während Du Commits anlegst, verpasst Du jedem einzelnen eine Markierung, die Du benutzen kannst, um von anderen Commits zu ihm zu linken. Daher ist das erste, was Deine `print_export`-Methode macht, eine Markierung aus dem Verzeichnisnamen zu erstellen:

	mark = convert_dir_to_mark(dir)

<!--You’ll do this by creating an array of directories and using the index value as the mark, because a mark must be an integer. Your method looks like this:-->

Das erreichst Du, indem Du ein Array von Verzeichnissen anlegst und den Index-Wert als Markierung verwendest (eine Markierung muss vom Typ `integer` sein). Deine Methode sieht so aus:

	$marks = []
	def convert_dir_to_mark(dir)
	  if !$marks.include?(dir)
	    $marks << dir
	  end
	  ($marks.index(dir) + 1).to_s
	end

<!--Now that you have an integer representation of your commit, you need a date for the commit metadata. Because the date is expressed in the name of the directory, you’ll parse it out. The next line in your `print_export` file is-->

Jetzt hast Du eine `integer`-Repräsentation Deines Commits und brauchst nur noch ein Datum für die Commit-Metadaten. Da das Datum im Verzeichnisnamen enthalten ist, parsen wir es einfach daraus. Die nächste Zeile in Deiner `print_export`-Datei lautet

	date = convert_dir_to_date(dir)

<!--where `convert_dir_to_date` is defined as-->

wobei `convert_dir_to_date` so definiert ist:

	def convert_dir_to_date(dir)
	  if dir == 'current'
	    return Time.now().to_i
	  else
	    dir = dir.gsub('back_', '')
	    (year, month, day) = dir.split('_')
	    return Time.local(year, month, day).to_i
	  end
	end

<!--That returns an integer value for the date of each directory. The last piece of meta-information you need for each commit is the committer data, which you hardcode in a global variable:-->

Die Funktion gibt einen Integer-Wert für das Datum eines jeden Verzeichnisses zurück. Das letzte Stück Meta-Information, das wir noch für jeden Commit brauchen, sind die Commit-Daten des Autors, die wir in eine globale Variable packen:

	$author = 'Scott Chacon <schacon@example.com>'

<!--Now you’re ready to begin printing out the commit data for your importer. The initial information states that you’re defining a commit object and what branch it’s on, followed by the mark you’ve generated, the committer information and commit message, and then the previous commit, if any. The code looks like this:-->

Jetzt können wir damit beginnen, die Commit-Daten für den Importer auszugeben. Die ursprüngliche Information gibt an, dass Du ein Commit-Objekt definierst und zu welchem Branch es gehört, gefolgt von der Markierung, die Du angelegt hast, der Committer-Information und der Commit-Nachricht und schließlich der ID des vorhergehenden Commits, falls dieser existiert. Der Code sieht wie folgt aus:

	# print the import information
	puts 'commit refs/heads/master'
	puts 'mark :' + mark
	puts "committer #{$author} #{date} -0700"
	export_data('imported from ' + dir)
	puts 'from :' + last_mark if last_mark

You hardcode the time zone (-0700) because doing so is easy. If you’re importing from another system, you must specify the time zone as an offset.
The commit message must be expressed in a special format:

	data (size)\n(contents)

The format consists of the word data, the size of the data to be read, a newline, and finally the data. Because you need to use the same format to specify the file contents later, you create a helper method, `export_data`:

	def export_data(string)
	  print "data #{string.size}\n#{string}"
	end

<!--All that’s left is to specify the file contents for each snapshot. This is easy, because you have each one in a directory — you can print out the `deleteall` command followed by the contents of each file in the directory. Git will then record each snapshot appropriately:-->

Alles, was jetzt noch übrig bleibt, ist das Feststellen des Dateiinhalts eines jeden Snapshots. Das ist einfach, weil Du jeden davon in einem Verzeichnis hast — Du kannst das `deleteall`-Kommando ausgeben, gefolgt von den Inhalten einer jeden Datei in dem Verzeichnis. Git wird dann jeden Snapshot entsprechend aufzeichnen:

	puts 'deleteall'
	Dir.glob("**/*").each do |file|
	  next if !File.file?(file)
	  inline_data(file)
	end

<!--Note:	Because many systems think of their revisions as changes from one commit to another, fast-import can also take commands with each commit to specify which files have been added, removed, or modified and what the new contents are. You could calculate the differences between snapshots and provide only this data, but doing so is more complex — you may as well give Git all the data and let it figure it out. If this is better suited to your data, check the `fast-import` man page for details about how to provide your data in this manner.-->

Anmerkung:	Da viele Systeme ihre Revisionen als Änderungen von einem Commit zu einem anderen betrachten, kann `fast-import` auch mit jedem Commit Kommandos entgegennehmen, die angeben, welche Dateien geändert, entfernt oder verändert wurden und was der neue Inhalt ist. Wir könnten die Unterschiede zwischen den Snapshots berechnen und nur diese Daten bereitstellen, aber das zu tun ist komplizierter — Du kannst Git auch einfach alle Daten füttern und es kümmert sich dann darum. Wenn dieses Vorgehen eher zu Deinen Daten passt, schau Dir die `fast-import` man-Seite an, um Details darüber zu erfahren, wie diese Daten dafür bereitgestellt werden müssen.

<!--The format for listing the new file contents or specifying a modified file with the new contents is as follows:-->

Das Format, in dem die Inhalte einer neuen Datei oder in eine geänderte Datei mit ihrem neuen Inhalt angegeben werden, sieht wie folgt aus:

	M 644 inline path/to/file
	data (size)
	(file contents)

<!--Here, 644 is the mode (if you have executable files, you need to detect and specify 755 instead), and inline says you’ll list the contents immediately after this line. Your `inline_data` method looks like this:-->

In diesem Beispiel ist 644 der Datei-Modus (wenn Du ausführbare Dateien hast, wirst Du möglicherweise 755 sehen bzw. einstellen), und inline gibt an, dass Du die Inhalte direkt im Anschluss an diese Zeile aufführen wirst. Deine `inline_data`-Methode sieht so aus:

	def inline_data(file, code = 'M', mode = '644')
	  content = File.read(file)
	  puts "#{code} #{mode} inline #{file}"
	  export_data(content)
	end

<!--You reuse the `export_data` method you defined earlier, because it’s the same as the way you specified your commit message data.-->

Du kannst die `export_data`-Methode, die Du vorher definiert hast, wiederverwenden, da wir das auf die gleiche Weise lösen, wie wir die Daten für die Commit-Nachrichten aufbereitet haben.

<!--The last thing you need to do is to return the current mark so it can be passed to the next iteration:-->

Das letzte, das wir jetzt noch machen müssen, ist, die gegenwärtige Marke zurückzugeben, damit sie an den nächsten Durchlauf übergeben werden kann.

	return mark

<!--NOTE: If you are running on Windows you’ll need to make sure that you add one extra step. As mentioned before, Windows uses CRLF for new line characters while git fast-import expects only LF. To get around this problem and make git fast-import happy, you need to tell ruby to use LF instead of CRLF:-->

	$stdout.binmode

<!--That’s it. If you run this script, you’ll get content that looks something like this:-->

Das ist alles. Wenn Du diese Skript laufen lässt, wirst Du eine Ausgabe erhalten, die etwa wie folgt aussieht:

	$ ruby import.rb /opt/import_from
	commit refs/heads/master
	mark :1
	committer Scott Chacon <schacon@geemail.com> 1230883200 -0700
	data 29
	imported from back_2009_01_02deleteall
	M 644 inline file.rb
	data 12
	version two
	commit refs/heads/master
	mark :2
	committer Scott Chacon <schacon@geemail.com> 1231056000 -0700
	data 29
	imported from back_2009_01_04from :1
	deleteall
	M 644 inline file.rb
	data 14
	version three
	M 644 inline new.rb
	data 16
	new version one
	(...)

<!--To run the importer, pipe this output through `git fast-import` while in the Git repository you want to import into. You can create a new directory and then run `git init` in it for a starting point, and then run your script:-->

Um den Importer zu starten, leite die Ausgabe durch eine Pipe zu `git fast-import` weiter — während Du im Git Verzeichnis befindest, in das Du importieren willst. Zum Anfang kannst Du ein neues Verzeichnis anlegen, darin `git init` laufen lassen und anschließend das Skript starten:

	$ git init
	Initialized empty Git repository in /opt/import_to/.git/
	$ ruby import.rb /opt/import_from | git fast-import
	git-fast-import statistics:
	---------------------------------------------------------------------
	Alloc'd objects:       5000
	Total objects:           18 (         1 duplicates                  )
	      blobs  :            7 (         1 duplicates          0 deltas)
	      trees  :            6 (         0 duplicates          1 deltas)
	      commits:            5 (         0 duplicates          0 deltas)
	      tags   :            0 (         0 duplicates          0 deltas)
	Total branches:           1 (         1 loads     )
	      marks:           1024 (         5 unique    )
	      atoms:              3
	Memory total:          2255 KiB
	       pools:          2098 KiB
	     objects:           156 KiB
	---------------------------------------------------------------------
	pack_report: getpagesize()            =       4096
	pack_report: core.packedGitWindowSize =   33554432
	pack_report: core.packedGitLimit      =  268435456
	pack_report: pack_used_ctr            =          9
	pack_report: pack_mmap_calls          =          5
	pack_report: pack_open_windows        =          1 /          1
	pack_report: pack_mapped              =       1356 /       1356
	---------------------------------------------------------------------

<!--As you can see, when it completes successfully, it gives you a bunch of statistics about what it accomplished. In this case, you imported 18 objects total for 5 commits into 1 branch. Now, you can run `git log` to see your new history:-->

Wie Du sehen kannst, werden Dir eine Menge Statistiken über den erreichten Erfolg angezeigt. In diesem Beispiel haben wir insgesamt 18 Objekte für 5 Commits in einen Zweig importiert. Jetzt kannst Du `git log` ausführen, um die neue History einzusehen:

	$ git log -2
	commit 10bfe7d22ce15ee25b60a824c8982157ca593d41
	Author: Scott Chacon <schacon@example.com>
	Date:   Sun May 3 12:57:39 2009 -0700

	    imported from current

	commit 7e519590de754d079dd73b44d695a42c9d2df452
	Author: Scott Chacon <schacon@example.com>
	Date:   Tue Feb 3 01:00:00 2009 -0700

	    imported from back_2009_02_03

<!--There you go — a nice, clean Git repository. It’s important to note that nothing is checked out — you don’t have any files in your working directory at first. To get them, you must reset your branch to where `master` is now:-->

Na also — jetzt haben wir ein schickes, sauberes Git-Repository. Dabei ist es wichtig, dass noch nichts ausgecheckt ist — zu Beginn hast Du keinerlei Dateien in Deinem Arbeitsverzeichnis. Um an sie heran zu kommen, musst Du Deinen Zweig dahin zurücksetzen, wo sich `master` gerade befindet:

	$ ls
	$ git reset --hard master
	HEAD is now at 10bfe7d imported from current
	$ ls
	file.rb  lib

<!--You can do a lot more with the `fast-import` tool — handle different modes, binary data, multiple branches and merging, tags, progress indicators, and more. A number of examples of more complex scenarios are available in the `contrib/fast-import` directory of the Git source code; one of the better ones is the `git-p4` script I just covered.-->

Du kannst noch eine ganze Menge mehr mit dem `fast-import`-Tool anstellen — es kann verschiedene Modi behandeln, Binärdaten, mehrere Zweige sowie Merges, Tags, Fortschrittsbalken und mehr. Eine Reihe von Beispielen komplexerer Szenarios werden im `contrib/fast-import`-Verzeichnis des Git-Quellcodes bereitgestellt; eines der besseren ist das `git-p4` -Skript, das ich gerade behandelt habe.

<!--## Summary ##-->
## Zusammenfassung ##

<!--You should feel comfortable using Git with Subversion or importing nearly any existing repository into a new Git one without losing data. The next chapter will cover the raw internals of Git so you can craft every single byte, if need be.-->

Du solltest Dich jetzt ausreichend sicher fühlen im Umgang mit Git und Subversion bzw. mit dem Import von nahezu jedem existierenden Repository in ein neues Git-Repository, ohne Daten zu verlieren. Das nächste Kapitel wird die Interna von Git behandeln, damit Du jedes einzelne Byte bearbeiten kannst, falls es nötig sein sollte.
<!--# Git Internals #-->
# Git Interna #

<!--You may have skipped to this chapter from a previous chapter, or you may have gotten here after reading the rest of the book — in either case, this is where you’ll go over the inner workings and implementation of Git. I found that learning this information was fundamentally important to understanding how useful and powerful Git is, but others have argued to me that it can be confusing and unnecessarily complex for beginners. Thus, I’ve made this discussion the last chapter in the book so you could read it early or later in your learning process. I leave it up to you to decide.-->

Möglicherweise hast Du dieses Kapitel hier direkt aufgeschlagen, vorherige Kapitel ausgelassen, oder bist hierher gelangt, nachdem Du alle vorherigen Kapitel gelesen hast. Wie auch immer, in diesem Kapitel werden wir uns mit der internen Funktionsweise und der Implementierung von Git befassen. Meine eigene Erfahrung ist, dass das Lernen dieser Dinge unerlässlich ist, um zu verstehen, wie unheimlich mächtig und flexibel Git ist. Allerdings gibt es Leute, die der Ansicht sind, dass sie auch verwirrend und unnötig komplex für Anfänger sein können. Deshalb habe ich mich entschieden, dieses Kapitel an das Ende des Buches zu verlegen. Du kannst es, ganz wie es Dir beliebt, früher oder später einschieben.

<!--Now that you’re here, let’s get started. First, if it isn’t yet clear, Git is fundamentally a content-addressable filesystem with a VCS user interface written on top of it. You’ll learn more about what this means in a bit.-->

Lass uns also loslegen. Zunächst will ich betonen, falls das bisher noch nicht klargeworden ist, dass Git im seinen Grundzügen ein Dateisystem ist, dessen Inhalte addressierbar sind und auf dem eine VCS-Schnittstelle aufgesetzt ist. Wir werden gleich genauer darauf eingehen, was das heißt.

<!--In the early days of Git (mostly pre 1.5), the user interface was much more complex because it emphasized this filesystem rather than a polished VCS. In the last few years, the UI has been refined until it’s as clean and easy to use as any system out there; but often, the stereotype lingers about the early Git UI that was complex and difficult to learn.-->

In den frühen Tagen von Git (d.h. vor Version 1.5) war die Benutzerschnittstelle sehr viel komplexer, weil es die Dateisystem-Eigenschaften stark betonte – im Gegensatz zu einem herkömmlichen VCS. In den letzten Jahren wurde die Benutzerschnittstelle dann stückweise verbessert und verfeinert, sodass es heute so einfach verständlich und einfach zu verwenden ist, wie andere vergleichbare Systeme, die auf dem Markt erhältlich sind. Allerdings besteht scheinbar weiterhin das Vorurteil, das Git-Interface sei komplex und schwer zu erlernen.

<!--The content-addressable filesystem layer is amazingly cool, so I’ll cover that first in this chapter; then, you’ll learn about the transport mechanisms and the repository maintenance tasks that you may eventually have to deal with.-->

Die inhaltsbasiert adressierbare Dateisystem-Ebene ist erstaunlich cool, weshalb ich in diesem Kapitel zuerst darauf eingehen werde. Als Nächstes lernst Du etwas über die Transport-Mechanismen und Repository-Wartungsaufgaben, mit denen Du möglicherweise irgendwann zu tun bekommen wirst.

<!--## Plumbing and Porcelain ##-->
## Plumbing und Porcelain ##

<!--This book covers how to use Git with 30 or so verbs such as `checkout`, `branch`, `remote`, and so on. But because Git was initially a toolkit for a VCS rather than a full user-friendly VCS, it has a bunch of verbs that do low-level work and were designed to be chained together UNIX style or called from scripts. These commands are generally referred to as "plumbing" commands, and the more user-friendly commands are called "porcelain" commands.-->

In diesem Buch haben wir Git besprochen, indem wir vielleicht 30 Befehle wie `checkout`, `branch`, `remote` und so weiter verwendet haben. Weil Git aber ursprünglich als ein Werkzeugkasten konzipiert war und nicht so sehr als ein komplettes, anwenderfreundliches VCS, gibt es auch eine Reihe von Befehlen, die ihre Arbeit auf einer sehr viel grundlegenderen Ebene verrichten. Viele davon sind ursprünglich entwickelt worden, um als UNIX-Befehle miteinander verkettet zu werden oder aus Skripten heraus aufgerufen zu werden. Diese Befehle werden oft als „plumbing“-Befehle (Klempner-Befehle) zusammengefasst, während die eher anwenderfreundlichen Befehle „porcelain“ (d.h. Porzellan) genannt werden.

<!--The book’s first eight chapters deal almost exclusively with porcelain commands. But in this chapter, you’ll be dealing mostly with the lower-level plumbing commands, because they give you access to the inner workings of Git and help demonstrate how and why Git does what it does. These commands aren’t meant to be used manually on the command line, but rather to be used as building blocks for new tools and custom scripts.-->

Die ersten acht Kapitel dieses Buches haben sich fast ausschließlich mit „Porcelain“-Befehlen befasst. In diesem Kapitel gehen wir dagegen auf die zugrundeliegenden „Plumbing“-Befehle ein, u.a. weil sie Dir den Zugriff auf die inneren Abläufe von Git ermöglichen, und weil sie dabei helfen, zu verstehen, warum Git tut, was es tut. Diese Befehle sind nicht dazu gedacht, manuell in der Eingabeaufforderung ausgeführt zu werden, sondern sind als Bausteine für Werkzeuge und Skripts gemeint.

<!--When you run `git init` in a new or existing directory, Git creates the `.git` directory, which is where almost everything that Git stores and manipulates is located. If you want to back up or clone your repository, copying this single directory elsewhere gives you nearly everything you need. This entire chapter basically deals with the stuff in this directory. Here’s what it looks like:-->

Wenn Du `git init` in einem neuen oder bereits bestehenden Verzeichnis ausführst, erzeugt Git das `.git`-Verzeichnis, das fast alle Dateien enthält, die Git intern speichert und ändert. Wenn Du eine Sicherheitskopie Deines Repositorys anlegen oder es duplizieren willst, dann reicht es aus, dieses Verzeichnis zu kopieren. Dieses ganze Kapitel handelt praktisch nur von den Inhalten dieses Verzeichnisses. Schauen wir es uns einmal an:

	$ ls
	HEAD
	branches/
	config
	description
	hooks/
	index
	info/
	objects/
	refs/

<!--You may see some other files in there, but this is a fresh `git init` repository — it’s what you see by default. The `branches` directory isn’t used by newer Git versions, and the `description` file is only used by the GitWeb program, so don’t worry about those. The `config` file contains your project-specific configuration options, and the `info` directory keeps a global exclude file for ignored patterns that you don’t want to track in a .gitignore file. The `hooks` directory contains your client- or server-side hook scripts, which are discussed in detail in Chapter 7.-->

Möglicherweise findest Du darin weitere Dateien. Obiges stammt aus einem mit `git init` neu angelegten Repository – das sind also die Standardinhalte. Der Ordner `branches` wird von neueren Git-Versionen nicht mehr verwendet, und die Datei `descriptions` wird nur vom Programm GitWeb benötigt. Du kannst sie also ignorieren. Die Datei `config` enthält Deine projekt-spezifischen Konfigurationsoptionen, und im Ordner `info` befindet sich eine Datei, die globale Dateiausschlussmuster enthält, die Du nicht in jeder .gitignore-Datei neu spezifizieren willst. Das `hooks`-Verzeichnis enthält die client- oder serverseitigen Hook-Skripte, die wir in Kapitel 7 besprochen haben.

<!--This leaves four important entries: the `HEAD` and `index` files and the `objects` and `refs` directories. These are the core parts of Git. The `objects` directory stores all the content for your database, the `refs` directory stores pointers into commit objects in that data (branches), the `HEAD` file points to the branch you currently have checked out, and the `index` file is where Git stores your staging area information. You’ll now look at each of these sections in detail to see how Git operates.-->

Damit bleiben vier wichtige Einträge übrig: die Dateien `HEAD` und `index` und die Verzeichnisse `objects` und `refs`. Dies sind die Kernkomponenten eines Git-Repositorys. Im `objects`-Verzeichnis befinden sich die Inhalte der Datenbank. Das `refs`-Verzeichnis enthält Referenzen auf Commit-Objekte (Branches) in dieser Datenbank. Die Datei `HEAD` zeigt auf denjeningen Branch, den Du gegenwärtig ausgecheckt hast, und in der Datei `index` verwaltet Git die Informationen der Staging-Area. Wir werden auf diese Elemente jetzt im einzelnen darauf eingehen, sodass Du nachvollziehen kannst, wie Git intern arbeitet.

<!--## Git Objects ##-->
## Git Objekte ##

<!--Git is a content-addressable filesystem. Great. What does that mean?-->
<!--It means that at the core of Git is a simple key-value data store. You can insert any kind of content into it, and it will give you back a key that you can use to retrieve the content again at any time. To demonstrate, you can use the plumbing command `hash-object`, which takes some data, stores it in your `.git` directory, and gives you back the key the data is stored as. First, you initialize a new Git repository and verify that there is nothing in the `objects` directory:-->

Git ist ein Dateisystem, das Inhalte addressieren kann. Prima. Aber was heißt das? Es bedeutet, dass Git im Kern nichts anderes ist als ein einfacher Key-Value-Store („Schlüssel-Wert-Speicher“). Du kannst darin jede Art von Inhalt ablegen und Git wird einen Schlüssel dafür zurückgeben, den Du dann verwenden kannst, um diesen Inhalt jederzeit nachzuschlagen. Um das auszuprobieren, kannst Du den Plumbing-Befehl `hash-object` verwenden. Dieser nimmt Daten entgegen, speichert diese in Deinem `.git`-Verzeichnis und gibt Dir den Schlüssel zurück, unter dem der Inhalt gespeichert wurde. Dazu initialisierst Du als erstes ein neues Git-Repository und verifizierst, dass das `objects`-Verzeichnis leer ist:

	$ mkdir test
	$ cd test
	$ git init
	Initialized empty Git repository in /tmp/test/.git/
	$ find .git/objects
	.git/objects
	.git/objects/info
	.git/objects/pack
	$ find .git/objects -type f
	$

<!--Git has initialized the `objects` directory and created `pack` and `info` subdirectories in it, but there are no regular files. Now, store some text in your Git database:-->

Git hat also ein Verzeichnis `objects` und darin die Unterverzeichnisse `pack` und `info` angelegt, bisher aber keine weiteren Dateien. Als nächsten speichern wir einen Text in dieser Git-Datenbank:

	$ echo 'test content' | git hash-object -w --stdin
	d670460b4b4aece5915caf5c68d12f560a9fe3e4

<!--The `-w` tells `hash-object` to store the object; otherwise, the command simply tells you what the key would be. `-\-stdin` tells the command to read the content from stdin; if you don’t specify this, `hash-object` expects the path to a file. The output from the command is a 40-character checksum hash. This is the SHA-1 hash — a checksum of the content you’re storing plus a header, which you’ll learn about in a bit. Now you can see how Git has stored your data:-->

Die Option `-w` weist `git hash-object` an, das Objekt zu speichern. Andernfalls würde Dir der Befehl lediglich den Schlüssel mitteilen. `--stdin` weist den Befehl an, den Inhalt von der Standardeingabe einzulesen. Wenn Du diese Option weglässt, erwartet der Befehl zusätzlich einen Dateipfad. Die Ausgabe ist ein 40 Zeichen langer SHA-1-Hash, der eine Prüfsumme des gespeicherten Inhaltes darstellt (wir gehen auf diese Hashes gleich noch genauer ein). Git hat jetzt außerdem eine neue Datei in der Datenbank angelegt:

	$ find .git/objects -type f
	.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4

<!--You can see a file in the `objects` directory. This is how Git stores the content initially — as a single file per piece of content, named with the SHA-1 checksum of the content and its header. The subdirectory is named with the first 2 characters of the SHA, and the filename is the remaining 38 characters.-->

Jetzt liegt im Verzeichnis `objects` genau eine Datei. Anfangs speichert Git den Inhalt auf diese Art und Weise: In jeweils einer einzelnen Datei werden die Daten gespeichert, referenziert durch den SHA-1-Hash des Inhaltes und seines Headers. Der Name des Unterverzeichnis `d6` entspricht den ersten zwei Zeichen des SHA-1-Hashes. Die verbleibenden 38 Zeichen werden als Dateiname verwendet.

<!--You can pull the content back out of Git with the `cat-file` command. This command is sort of a Swiss army knife for inspecting Git objects. Passing `-p` to it instructs the `cat-file` command to figure out the type of content and display it nicely for you:-->

Mit dem Befehl `git cat-file` kannst Du den jeweiligen Inhalt nachschlagen. Dieser Befehl ist so etwas wie ein Schweizer Taschenmesser, wenn es um Objekte in der Git-Datenbank geht. Wenn Du die Option `-p` übergibst, versucht `git cat-file`, die Art des Inhaltes herauszufinden und lesbar darzustellen:

	$ git cat-file -p d670460b4b4aece5915caf5c68d12f560a9fe3e4
	test content

<!--Now, you can add content to Git and pull it back out again. You can also do this with content in files. For example, you can do some simple version control on a file. First, create a new file and save its contents in your database:-->

Auf diese Weise kannst Du also Inhalte zu Git hinzufügen und von dort wieder auslesen. Das klappt auch mit Dateiinhalten. Wenn Du beispielsweise eine einzelne Datei versionieren willst, legst Du dazu die Datei zunächst an und speicherst ihren Inhalt in der Datenbank:

	$ echo 'version 1' > test.txt
	$ git hash-object -w test.txt
	83baae61804e65cc73a7201a7252750c76066a30

<!--Then, write some new content to the file, and save it again:-->

Dann kannst Du Änderungen vornehmen und die Datei erneut speichern:

	$ echo 'version 2' > test.txt
	$ git hash-object -w test.txt
	1f7a7a472abf3dd9643fd615f6da379c4acb3e3a

<!--Your database contains the two new versions of the file as well as the first content you stored there:-->

Die Datenbank enthält jetzt zwei weitere Versionen der Datei neben dem ursprünglich gespeicherten Inhalt:

	$ find .git/objects -type f
	.git/objects/1f/7a7a472abf3dd9643fd615f6da379c4acb3e3a
	.git/objects/83/baae61804e65cc73a7201a7252750c76066a30
	.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4

<!--Now you can revert the file back to the first version-->

Jetzt kannst Du die erste Version der Datei mit folgendem Befehl wieder herstellen:

	$ git cat-file -p 83baae61804e65cc73a7201a7252750c76066a30 > test.txt
	$ cat test.txt
	version 1

<!--or the second version:-->

Oder die zweite Version:

	$ git cat-file -p 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a > test.txt
	$ cat test.txt
	version 2

<!--But remembering the SHA-1 key for each version of your file isn’t practical; plus, you aren’t storing the filename in your system — just the content. This object type is called a blob. You can have Git tell you the object type of any object in Git, given its SHA-1 key, with `cat-file -t`:-->

Sich den SHA-1-Hash für jede Version merken zu müssen, ist allerdings nicht sonderlich praktisch. Außerdem speicherst Du nicht den Dateinamen in der Datenbank, sondern lediglich den Inhalt der Datei. Ein solcher Objekttyp wird als „Blob“ bezeichnet. Mit `git cat-file -t` kannst Du Git nach dem Typ eines Objektes in der Datenbank fragen:

	$ git cat-file -t 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a
	blob

<!--### Tree Objects ###-->
### Baum-Objekte ###

<!--The next type you’ll look at is the tree object, which solves the problem of storing the filename and also allows you to store a group of files together. Git stores content in a manner similar to a UNIX filesystem, but a bit simplified. All the content is stored as tree and blob objects, with trees corresponding to UNIX directory entries and blobs corresponding more or less to inodes or file contents. A single tree object contains one or more tree entries, each of which contains a SHA-1 pointer to a blob or subtree with its associated mode, type, and filename. For example, the most recent tree in the simplegit project may look something like this:-->

Als Nächstes schauen wir uns den Objekttyp „Tree“ (Baum) an, der es ermöglicht, Dateinamen zu speichern und Dateien zu gruppieren. Git speichert Inhalte in einer ähnlichen Weise wie das UNIX-Dateisystem, allerdings ein bisschen vereinfacht. Sie werden als Tree- und Blob-Objekte abgelegt, wobei die Trees mit UNIX-Verzeichnis-Einträgen korrespondieren und Blobs mehr oder weniger mit den Inode-Einträgen bzw. Datei-Inhalten. Ein einzelnes Baum-Objekt enthält einen oder mehrere Einträge, von denen jeder ein SHA-1-Hash ist, der wiederum einen Blob oder einen Untertree referenziert. Jeder dieser Einträge verfügt außerdem über einen Modus, Typ und Dateinamen. Beispielsweise sieht das aktuelle Tree-Objekt im simplegit-Projekt möglicherweise so aus:

	$ git cat-file -p master^{tree}
	100644 blob a906cb2a4a904a152e80877d4088654daad0c859      README
	100644 blob 8f94139338f9404f26296befa88755fc2598c289      Rakefile
	040000 tree 99f1a6d12cb4b6f19c8655fca46c3ecf317074e0      lib

<!--The `master^{tree}` syntax specifies the tree object that is pointed to by the last commit on your `master` branch. Notice that the `lib` subdirectory isn’t a blob but a pointer to another tree:-->

Die `master^{tree}`-Syntax spezifiziert, dass wir an dem Tree-Objekt interessiert sind, auf das der letzte Commit des Branches `master` zeigt. Beachte, dass das Unterverzeichnis `lib` nicht auf ein Blob, sondern wiederum auf einen weiteren Baum zeigt.

	$ git cat-file -p 99f1a6d12cb4b6f19c8655fca46c3ecf317074e0
	100644 blob 47c6340d6459e05787f644c2447d2595f5d3a54b      simplegit.rb

<!--Conceptually, the data that Git is storing is something like Figure 9-1.-->

Git speichert Daten also, konzeptuell gesehen, in etwa wie in Bild 9-1 dargestellt.

<!--Figure 9-1. Simple version of the Git data model.-->

Insert 18333fig0901.png
Bild 9-1. Vereinfachte Darstellung des Git-Datenmodels.

<!--You can create your own tree. Git normally creates a tree by taking the state of your staging area or index and writing a tree object from it. So, to create a tree object, you first have to set up an index by staging some files. To create an index with a single entry — the first version of your test.txt file — you can use the plumbing command `update-index`. You use this command to artificially add the earlier version of the test.txt file to a new staging area. You must pass it the `-\-add` option because the file doesn’t yet exist in your staging area (you don’t even have a staging area set up yet) and `-\-cacheinfo` because the file you’re adding isn’t in your directory but is in your database. Then, you specify the mode, SHA-1, and filename:-->

Du kannst auch Deine eigenen Tree-Objekte anlegen. Git erzeugt Trees normalerweise, indem es die Inhalte der Staging-Area nimmt und als ein Tree-Objekt speichert. D.h., um ein Tree-Objekt anzulegen, musst Du zunächst einen Index (d.h. eine Staging-Area) aufbauen, indem Du einige Dateien hinzufügst. Um einen einzelnen Eintrag in den Index zu schreiben – z.B. die erste version der Datei test.txt – kannst Du den Plumbing-Befehl `git update-index` verwenden, der eine frühere Version dieser Datei künstlich zu einer neuen Staging-Area hinzufügt. Du musst ihm die Option `--add` übergeben, weil die Datei bisher noch nicht in der Staging-Area enthalten ist (Du hast ja bisher noch überhaupt keine Staging-Area aufgesetzt), und die Option `--cacheinfo`, weil Du eine Datei hinzufügst, die sich nicht in Deinem Verzeichnis befindet, sondern in der Datenbank. Du gibst außerdem den Modus, SHA-1-Hash und den Dateinamen an:

	$ git update-index --add --cacheinfo 100644 \
	  83baae61804e65cc73a7201a7252750c76066a30 test.txt

<!--In this case, you’re specifying a mode of `100644`, which means it’s a normal file. Other options are `100755`, which means it’s an executable file; and `120000`, which specifies a symbolic link. The mode is taken from normal UNIX modes but is much less flexible — these three modes are the only ones that are valid for files (blobs) in Git (although other modes are used for directories and submodules).-->

In diesem Fall gibst Du als Modus `100644` an, was bedeutet, dass es sich um eine normale Datei handelt. Eine ausführbare Datei wäre dagegen `100755` und ein symbolischer Link `120000`. Der Modus entspricht normalen UNIX-Datei-Modi, ist aber weniger flexibel. Die drei genannten Modi sind die einzigen, die in Git für Dateien (Blobs) verwendet werden (es gibt allerdings noch weitere Modi für Verzeichnisse und Submodule).

<!--Now, you can use the `write-tree` command to write the staging area out to a tree object. No `-w` option is needed — calling `write-tree` automatically creates a tree object from the state of the index if that tree doesn’t yet exist:-->

Jetzt kannst Du den Befehl `git write-tree` verwenden, um die Staging-Area als Tree-Objekt zu schreiben. Dazu brauchst Du die `-w` Option nicht angeben – `git write-tree` schreibt automatisch ein Tree-Objekt für Einträge der Staging-Area, für die es noch keinen Tree gibt:

	$ git write-tree
	d8329fc1cc938780ffdd9f94e0d364e0ea74f579
	$ git cat-file -p d8329fc1cc938780ffdd9f94e0d364e0ea74f579
	100644 blob 83baae61804e65cc73a7201a7252750c76066a30      test.txt

<!--You can also verify that this is a tree object:-->

Um zu überprüfen, ob es sich wirklich um ein Tree-Objekt handelt:

	$ git cat-file -t d8329fc1cc938780ffdd9f94e0d364e0ea74f579
	tree

<!--You’ll now create a new tree with the second version of test.txt and a new file as well:-->

Jetzt erzeugen wir einen neuen Tree mit der zweiten Version der Datei test.txt sowie einer neuen Datei:

	$ echo 'new file' > new.txt
	$ git update-index test.txt
	$ git update-index --add new.txt

<!--Your staging area now has the new version of test.txt as well as the new file new.txt. Write out that tree (recording the state of the staging area or index to a tree object) and see what it looks like:-->

Die Staging-Area enthält jetzt eine neue Version der Datei test.txt sowie die neue Datei new.txt. Speichern wir diesen Tree (d.h. den gegenwärtigen Status der Staging-Area bzw. des Index als Tree-Objekt) und schauen ihn uns an:

	$ git write-tree
	0155eb4229851634a0f03eb265b69f5a2d56f341
	$ git cat-file -p 0155eb4229851634a0f03eb265b69f5a2d56f341
	100644 blob fa49b077972391ad58037050f2a75f74e3671e92      new.txt
	100644 blob 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a      test.txt

<!--Notice that this tree has both file entries and also that the test.txt SHA is the "version 2" SHA from earlier (`1f7a7a`). Just for fun, you’ll add the first tree as a subdirectory into this one. You can read trees into your staging area by calling `read-tree`. In this case, you can read an existing tree into your staging area as a subtree by using the `-\-prefix` option to `read-tree`:-->

Beachte, dass das Tree-Objekt beide Datei-Einträge hat und dass der SHA-1-Hash der Datei test.txt noch derselbe „Version 2“-Hash ist wie zuvor (`1f7a7a`). Fügen wir jetzt den ersten Tree als ein Unterverzeichnis in diesem hier ein. Du kannst einen Tree mit `git read-tree` in die Staging-Area einlesen. In diesem Fall können wir einen bereits existierenden Tree als einen Untertree zur Staging-Area hinzufügen, indem wir die Option `--prefix` verwenden:

	$ git read-tree --prefix=bak d8329fc1cc938780ffdd9f94e0d364e0ea74f579
	$ git write-tree
	3c4e9cd789d88d8d89c1073707c3585e41b0e614
	$ git cat-file -p 3c4e9cd789d88d8d89c1073707c3585e41b0e614
	040000 tree d8329fc1cc938780ffdd9f94e0d364e0ea74f579      bak
	100644 blob fa49b077972391ad58037050f2a75f74e3671e92      new.txt
	100644 blob 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a      test.txt

<!--If you created a working directory from the new tree you just wrote, you would get the two files in the top level of the working directory and a subdirectory named `bak` that contained the first version of the test.txt file. You can think of the data that Git contains for these structures as being like Figure 9-2.-->

Wenn Du ein Arbeitsverzeichnis aus diesem neuen Tree-Objekt auschecken würdest, würdest Du zwei Dateien im Hauptverzeichnis und ein Unterverzeichnis mit dem Namen `bak` erhalten, in dem sich die erste Version der Datei test.txt befindet. Du kannst Dir die Daten, die Git für diese Strukturen speichert, in etwa wie in Bild 9-2 vorstellen.

<!--Figure 9-2. The content structure of your current Git data.-->

Insert 18333fig0902.png
Bild 9-2. Die Datenstruktur des gegenwärtigen Git-Repositorys.

<!--### Commit Objects ###-->
### Objekte committen ###

<!--You have three trees that specify the different snapshots of your project that you want to track, but the earlier problem remains: you must remember all three SHA-1 values in order to recall the snapshots. You also don’t have any information about who saved the snapshots, when they were saved, or why they were saved. This is the basic information that the commit object stores for you.-->

Du hast jetzt drei Trees, die verschiedene Snapshots Deines Projektes spezifizieren, die Du nachverfolgen willst. Das ursprüngliche Problem besteht aber weiterhin: Du musst Dir alle drei SHA-1-Hashwerte merken, um wieder an die Snapshots zu kommen. Ebenso fehlen Dir die Informationen darüber, wer die Snapshots gespeichert hat, wann sie gespeichert wurden und warum. Dies sind die drei Hauptinformationen, die ein Commit-Objekt für uns speichert.

<!--To create a commit object, you call `commit-tree` and specify a single tree SHA-1 and which commit objects, if any, directly preceded it. Start with the first tree you wrote:-->

Um ein Commit-Objekt anzulegen, verwendest Du den Befehl `git commit-tree`, spezifizierst den SHA-1-Hash eines einzelnen Trees und welche Commit-Objekte (sofern vorhanden) die direkten Vorgänger sind. Fangen wir damit an, den ersten Tree, den Du angelegt hast, einzuchecken:

	$ echo 'first commit' | git commit-tree d8329f
	fdf4fc3344e67ab068f836878b6c4951e3b15f3d

<!--Now you can look at your new commit object with `cat-file`:-->

Du kannst Dir dann dieses neue Commit-Objekt mit `cat-file` anschauen:

	$ git cat-file -p fdf4fc3
	tree d8329fc1cc938780ffdd9f94e0d364e0ea74f579
	author Scott Chacon <schacon@gmail.com> 1243040974 -0700
	committer Scott Chacon <schacon@gmail.com> 1243040974 -0700

	first commit

<!--The format for a commit object is simple: it specifies the top-level tree for the snapshot of the project at that point; the author/committer information pulled from your `user.name` and `user.email` configuration settings, with the current timestamp; a blank line, and then the commit message.-->

Das Format für ein Commit-Objekt ist einfach: es besteht aus dem obersten Tree für den Snapshot des Projektes zum gegebenen Zeitpunkt, die Autoren- und ggf. Committer-Information (jeweils entsprechend Deiner `user.name`- und `user.email`-Konfiguration) und dem aktuellen Zeitstempel. Dann folgen eine leere Zeile und die Commit-Nachricht.

<!--Next, you’ll write the other two commit objects, each referencing the commit that came directly before it:-->

Als Nächstes speichern wir die beiden anderen Commit-Objekte und referenzieren jeweils den vorhergehenden Commit:

	$ echo 'second commit' | git commit-tree 0155eb -p fdf4fc3
	cac0cab538b970a37ea1e769cbbde608743bc96d
	$ echo 'third commit'  | git commit-tree 3c4e9c -p cac0cab
	1a410efbd13591db07496601ebc7a059dd55cfe9

<!--Each of the three commit objects points to one of the three snapshot trees you created. Oddly enough, you have a real Git history now that you can view with the `git log` command, if you run it on the last commit SHA-1:-->

Jedes der drei Commit-Objekte zeigt auf einen der drei Snapshot-Trees, die Du zuvor gespeichert hattest. Es mag Dich überraschen, aber Du hast jetzt bereits eine vollständige Git-Historie, die Du mit dem Befehl `git log` inspizieren kannst, indem Du den SHA-1-Hash des letzten Commits angibst:

	$ git log --stat 1a410e
	commit 1a410efbd13591db07496601ebc7a059dd55cfe9
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Fri May 22 18:15:24 2009 -0700

	    third commit

	 bak/test.txt |    1 +
	 1 files changed, 1 insertions(+), 0 deletions(-)

	commit cac0cab538b970a37ea1e769cbbde608743bc96d
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Fri May 22 18:14:29 2009 -0700

	    second commit

	 new.txt  |    1 +
	 test.txt |    2 +-
	 2 files changed, 2 insertions(+), 1 deletions(-)

	commit fdf4fc3344e67ab068f836878b6c4951e3b15f3d
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Fri May 22 18:09:34 2009 -0700

	    first commit

	 test.txt |    1 +
	 1 files changed, 1 insertions(+), 0 deletions(-)

<!--Amazing. You’ve just done the low-level operations to build up a Git history without using any of the front ends. This is essentially what Git does when you run the `git add` and `git commit` commands — it stores blobs for the files that have changed, updates the index, writes out trees, and writes commit objects that reference the top-level trees and the commits that came immediately before them. These three main Git objects — the blob, the tree, and the commit — are initially stored as separate files in your `.git/objects` directory. Here are all the objects in the example directory now, commented with what they store:-->

Fantastisch, oder? Du hast jetzt sämtliche Low-Level-Operationen durchgeführt, die eine vollständige Git-Historie aufbauen, ohne aber irgendwelche Frontend-Befehle von Git zu verwenden. Im Wesentlichen ist das derselbe Prozess, der im Hintergrund stattfindet, wenn Du die Befehle `git add` und `git commit` ausführst. Sie speichern Blobs für die Dateien, die Du hinzugefügt oder geändert hast, aktualisieren den Index (d.h. die Staging-Area), speichern Trees und legen Commit Objekte an, die die obersten Trees sowie die Commits referenzieren, die ihnen unmittelbar vorhergingen. Diese drei Hauptobjekte – Blob, Tree und Commit – werden zunächst als separate Dateien im Verzeichnis `.git/objects` gespeichert. Hier ist eine Liste aller Objekte, die sich in unserem Beispiel-Repository jetzt in der Datenbank befinden – jeweils mit einem Kommentar darüber, was sie speichern:

	$ find .git/objects -type f
	.git/objects/01/55eb4229851634a0f03eb265b69f5a2d56f341 # tree 2
	.git/objects/1a/410efbd13591db07496601ebc7a059dd55cfe9 # commit 3
	.git/objects/1f/7a7a472abf3dd9643fd615f6da379c4acb3e3a # test.txt v2
	.git/objects/3c/4e9cd789d88d8d89c1073707c3585e41b0e614 # tree 3
	.git/objects/83/baae61804e65cc73a7201a7252750c76066a30 # test.txt v1
	.git/objects/ca/c0cab538b970a37ea1e769cbbde608743bc96d # commit 2
	.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4 # 'test content'
	.git/objects/d8/329fc1cc938780ffdd9f94e0d364e0ea74f579 # tree 1
	.git/objects/fa/49b077972391ad58037050f2a75f74e3671e92 # new.txt
	.git/objects/fd/f4fc3344e67ab068f836878b6c4951e3b15f3d # commit 1

<!--If you follow all the internal pointers, you get an object graph something like Figure 9-3.-->

Wenn man all diese internen Zeiger nachverfolgt, erhält man einen Objekt-Graphen wie den folgenden (Bild 9-3).

<!--Figure 9-3. All the objects in your Git directory.-->

Insert 18333fig0903.png
Bild 9-3. Alle Objekte in Deinem Git-Repository.

<!--### Object Storage ###-->
### Objekt-Speicher ###

<!--I mentioned earlier that a header is stored with the content. Let’s take a minute to look at how Git stores its objects. You’ll see how to store a blob object — in this case, the string "what is up, doc?" — interactively in the Ruby scripting language. You can start up interactive Ruby mode with the `irb` command:-->

Ich habe bereits erwähnt, dass zusammen mit dem jeweiligen Inhalt ein Header gespeichert wird. Schauen wir uns also genauer an, wie genau Git Objekte speichert. Du wirst sehen, wie ein Blob-Objekt – in diesem Fall die Zeichenkette „what is up, doc?“ gespeichert wird. Dazu nutzen wir den interaktiven Ruby-Modus, den Du mit dem Befehl `irb` starten kannst:

	$ irb
	>> content = "what is up, doc?"
	=> "what is up, doc?"

<!--Git constructs a header that starts with the type of the object, in this case a blob. Then, it adds a space followed by the size of the content and finally a null byte:-->

Git erzeugt einen Header, der mit dem Objekttyp beginnt, in diesem Fall ist das ein Blob. Dann folgt ein Leerzeichen, die Anzahl der Zeichen des Inhalts und schließlich ein Nullbyte.

	>> header = "blob #{content.length}\0"
	=> "blob 16\000"

<!--Git concatenates the header and the original content and then calculates the SHA-1 checksum of that new content. You can calculate the SHA-1 value of a string in Ruby by including the SHA1 digest library with the `require` command and then calling `Digest::SHA1.hexdigest()` with the string:-->

Git fügt diesen Header mit dem ursprünglichen Inhalt zusammen und kalkuliert aus dem Ergebnis die SHA-1-Prüfsumme. Du kannst einen SHA-1-Hash in Ruby berechnen, indem Du die SHA1-Digest-Bibliothek mit `require` einbindest und dann `Digest::SHA1.hexdigest()` mit der Zeichenkette ausführst:

	>> store = header + content
	=> "blob 16\000what is up, doc?"
	>> require 'digest/sha1'
	=> true
	>> sha1 = Digest::SHA1.hexdigest(store)
	=> "bd9dbf5aae1a3862dd1526723246b20206e5fc37"

<!--Git compresses the new content with zlib, which you can do in Ruby with the zlib library. First, you need to require the library and then run `Zlib::Deflate.deflate()` on the content:-->

Git komprimiert den neuen Inhalt (d.h. inklusive des Headers) mit zlib. In Ruby kannst Du dazu die zlib-Bibliothek verwenden, indem Du wiederum zuerst die Bibliothek mit `require` einbindest und dann `Zlib::Deflate.deflate()` mit dem Inhalt aufrufst:

	>> require 'zlib'
	=> true
	>> zlib_content = Zlib::Deflate.deflate(store)
	=> "x\234K\312\311OR04c(\317H,Q\310,V(-\320QH\311O\266\a\000_\034\a\235"

<!--Finally, you’ll write your zlib-deflated content to an object on disk. You’ll determine the path of the object you want to write out (the first two characters of the SHA-1 value being the subdirectory name, and the last 38 characters being the filename within that directory). In Ruby, you can use the `FileUtils.mkdir_p()` function to create the subdirectory if it doesn’t exist. Then, open the file with `File.open()` and write out the previously zlib-compressed content to the file with a `write()` call on the resulting file handle:-->

Schließlich schreibst Du den zlib-komprimierten Inhalt in eine Datei auf der Festplatte. Dazu bestimmst Du den Pfad, an den die Datei gespeichert wird (die ersten beiden Zeichen für das Unterverzeichnis und die verbleibenden 38 Zeichen für den Dateinamen). In Ruby kannst Du die Funktion `FileUtils.mkdir_p()` verwenden, um Unterverzeichnisse anzulegen, die noch nicht existieren. Dann öffnest Du die Datei mit `File.open()` und schreibst den komprimierten Inhalt mit `write()` in die Datei:

	>> path = '.git/objects/' + sha1[0,2] + '/' + sha1[2,38]
	=> ".git/objects/bd/9dbf5aae1a3862dd1526723246b20206e5fc37"
	>> require 'fileutils'
	=> true
	>> FileUtils.mkdir_p(File.dirname(path))
	=> ".git/objects/bd"
	>> File.open(path, 'w') { |f| f.write zlib_content }
	=> 32

<!--That’s it — you’ve created a valid Git blob object. All Git objects are stored the same way, just with different types — instead of the string blob, the header will begin with commit or tree. Also, although the blob content can be nearly anything, the commit and tree content are very specifically formatted.-->

Das ist alles – Du hast jetzt ein gültiges Blob-Objekt geschrieben. Git-Objekte werden immer in dieser Weise gespeichert, lediglich mit verschiedenen Typen, d.h. anstelle der Zeichenkette „blob“ wird der Header mit „commit“ oder „tree“ anfangen. Außerdem sind Commit- und Tree-Inhalte auf eine sehr spezifische Weise formatiert, während Blobs beliebige Inhalte sein können.

<!--## Git References ##-->
## Git-Referenzen ##

<!--You can run something like `git log 1a410e` to look through your whole history, but you still have to remember that `1a410e` is the last commit in order to walk that history to find all those objects. You need a file in which you can store the SHA-1 value under a simple name so you can use that pointer rather than the raw SHA-1 value.-->

Du kannst Befehle wie `git log 1a410e` ausführen, um die Commit-Historie zu inspizieren, aber dazu musst Du Dir jeweils merken, dass `1a410e` der jeweils letzte Commit ist. Um diese SHA-1-Hashes mit einfacheren, verständlichen Namen zu referenzieren, verwendet Git weitere Dateien, in denen die Namen für Hashes gespeichert sind.

<!--In Git, these are called "references" or "refs"; you can find the files that contain the SHA-1 values in the `.git/refs` directory. In the current project, this directory contains no files, but it does contain a simple structure:-->

Diese Namen werden in Git intern als „references“ oder „refs“ (also Referenz bzw. Verweis) bezeichnet. Du kannst diese Dateien, die SHA-1-Hashes enthalten, im Verzeichnis `.git/refs` finden. In unserem gegenwärtigen Projekt enthält dieses Verzeichnis noch keine Dateien, aber eine simple Verzeichnisstruktur:

	$ find .git/refs
	.git/refs
	.git/refs/heads
	.git/refs/tags
	$ find .git/refs -type f
	$

<!--To create a new reference that will help you remember where your latest commit is, you can technically do something as simple as this:-->

Um jetzt eine neue Referenz anzulegen, die Dir dabei hilft, Dich zu erinnern, wo sich Dein letzter Commit befindet, könntest Du, technisch gesehen, Folgendes tun:

	$ echo "1a410efbd13591db07496601ebc7a059dd55cfe9" > .git/refs/heads/master

<!--Now, you can use the head reference you just created instead of the SHA-1 value in your Git commands:-->

Jetzt kannst Du diese „head“-Referenz anstelle des SHA-1-Wertes in allen möglichen Git-Befehlen verwenden:

	$ git log --pretty=oneline  master
	1a410efbd13591db07496601ebc7a059dd55cfe9 third commit
	cac0cab538b970a37ea1e769cbbde608743bc96d second commit
	fdf4fc3344e67ab068f836878b6c4951e3b15f3d first commit

<!--You aren’t encouraged to directly edit the reference files. Git provides a safer command to do this if you want to update a reference called `update-ref`:-->

Allerdings ist es nicht empfehlenswert, die Referenz-Dateien direkt zu bearbeiten. Git stellt einen sichereren Befehl dafür zur Verfügung, den Befehl `git update-ref`:

	$ git update-ref refs/heads/master 1a410efbd13591db07496601ebc7a059dd55cfe9

<!--That’s basically what a branch in Git is: a simple pointer or reference to the head of a line of work. To create a branch back at the second commit, you can do this:-->

Im Prinzip ist das alles, was einen Branch in Git ausmacht: ein simpler Zeiger oder eine Referenz auf den jeweiligen Head einer Arbeitsreihe. Um einen neuen Branch anzulegen, der vom zweiten Commit aus verzweigt, kannst Du Folgendes tun:

	$ git update-ref refs/heads/test cac0ca

<!--Your branch will contain only work from that commit down:-->

Dein Branch beginnt jetzt beim zweiten Commit:

	$ git log --pretty=oneline test
	cac0cab538b970a37ea1e769cbbde608743bc96d second commit
	fdf4fc3344e67ab068f836878b6c4951e3b15f3d first commit

<!--Now, your Git database conceptually looks something like Figure 9-4.-->

Die Git-Datenbank unseres Beispiel-Repositorys ist jetzt wie folgt strukturiert:

<!--Figure 9-4. Git directory objects with branch head references included.-->

Insert 18333fig0904.png
Bild 9-4. Git-Verzeichnis-Objekte mit Branch-Head-Referenzen.

<!--When you run commands like `git branch (branchname)`, Git basically runs that `update-ref` command to add the SHA-1 of the last commit of the branch you’re on into whatever new reference you want to create.-->

Wenn Du Befehle wie `git branch (Branch-Name)` verwendest, führt Git intern im Wesentlichen den Befehl `update-ref` aus, um den SHA-1-Hash des letzten Commits des jeweils gegenwärtigen Branches mit dem gegebenen Namen zu referenzieren.

<!--### The HEAD ###-->
### Der HEAD ###

<!--The question now is, when you run `git branch (branchname)`, how does Git know the SHA-1 of the last commit? The answer is the HEAD file. The HEAD file is a symbolic reference to the branch you’re currently on. By symbolic reference, I mean that unlike a normal reference, it doesn’t generally contain a SHA-1 value but rather a pointer to another reference. If you look at the file, you’ll normally see something like this:-->

Die Frage ist jetzt: Wenn Du `git branch (Branch-Name)` ausführst, woher weiß Git den SHA-1 des letzten Commits? Die Antwort ist: aus der Datei HEAD. Diese Datei ist eine symbolische Referenz auf den jeweiligen Branch, auf dem Du Dich gerade befindest. Mit „symbolischer Referenz“ meine ich, dass sie (anders als eine „normale“ Referenz) keinen SHA-1-Hash enthält, sondern stattdessen auf eine andere Referenz zeigt. Wenn Du Dir die Datei ansiehst, findest Du normalerweise etwas wie:

	$ cat .git/HEAD
	ref: refs/heads/master

<!--If you run `git checkout test`, Git updates the file to look like this:-->

Wenn Du jetzt `git checkout test` ausführst, wird Git die Datei aktualisieren, sodass sie so aussieht:

	$ cat .git/HEAD
	ref: refs/heads/test

<!--When you run `git commit`, it creates the commit object, specifying the parent of that commit object to be whatever SHA-1 value the reference in HEAD points to.-->

Wenn Du `git commit` ausführst, erzeugt Git das Commit-Objekt und verwendet als Parent des Commit-Objektes den jeweiligen Wert der Referenz, auf die HEAD zeigt.

<!--You can also manually edit this file, but again a safer command exists to do so: `symbolic-ref`. You can read the value of your HEAD via this command:-->

Du kannst diese Datei manuell bearbeiten, aber wiederum verfügt Git über einen sichereren Befehl, um das zu tun: `git symbolic-ref`. Du kannst den Wert des HEAD mit Hilfe des folgenden Befehls lesen:

	$ git symbolic-ref HEAD
	refs/heads/master

<!--You can also set the value of HEAD:-->

Und so kannst Du ihn setzen:

	$ git symbolic-ref HEAD refs/heads/test
	$ cat .git/HEAD
	ref: refs/heads/test

<!--You can’t set a symbolic reference outside of the refs style:-->

Du kannst den Befehl allerdings nicht verwenden, um eine Referenz außerhalb von `refs` zu setzen:

	$ git symbolic-ref HEAD test
	fatal: Refusing to point HEAD outside of refs/

<!--### Tags ###-->
### Tags ###

<!--You’ve just gone over Git’s three main object types, but there is a fourth. The tag object is very much like a commit object — it contains a tagger, a date, a message, and a pointer. The main difference is that a tag object points to a commit rather than a tree. It’s like a branch reference, but it never moves — it always points to the same commit but gives it a friendlier name.-->

Wir haben jetzt Gits drei Haupt-Objekttypen besprochen, aber es gibt noch einen vierten. Das Tag-Objekt ist dem Commit-Objekt sehr ähnlich: es enthält den Autor des Tags, ein Datum, eine Meldung und eine Referenz auf ein anderes Objekt. Der Hauptunterschied besteht darin, dass ein Tag-Objekt auf einen Commit zeigt und nicht auf einen Tree. Ein Tag ist in dieser Hinsicht also ähnlich einem Branch, aber er bewegt sich nie, sondern zeigt immer auf denselben Commit und gibt ihm damit einen netteren Namen.

<!--As discussed in Chapter 2, there are two types of tags: annotated and lightweight. You can make a lightweight tag by running something like this:-->

Wie wir schon in Kapitel 2 besprochen haben, gibt es zwei Typen von Tags: „annotierte“ und „einfache“. Du kannst einen einfachen Tag wie folgt anlegen:

	$ git update-ref refs/tags/v1.0 cac0cab538b970a37ea1e769cbbde608743bc96d

<!--That is all a lightweight tag is — a branch that never moves. An annotated tag is more complex, however. If you create an annotated tag, Git creates a tag object and then writes a reference to point to it rather than directly to the commit. You can see this by creating an annotated tag (`-a` specifies that it’s an annotated tag):-->

Das ist alles, woraus ein einfacher Tag besteht: einem Branch, der sich nie bewegt. Ein annotierter Tag ist komplexer. Wenn Du einen annotierten Tag anlegst, erzeugt Git ein Tag-Objekt und speichert eine Referenz, die darauf zeigt, statt direkt auf den Commit zu zeigen. Du kannst das sehen, wenn Du einen annotierten Tag anlegst (`-a` bewirkt, dass wir einen annotierten Tag erhalten):

	$ git tag -a v1.1 1a410efbd13591db07496601ebc7a059dd55cfe9 -m 'test tag'

<!--Here’s the object SHA-1 value it created:-->

Das erzeugt den folgenden Objekt SHA-1-Hash:

	$ cat .git/refs/tags/v1.1
	9585191f37f7b0fb9444f35a9bf50de191beadc2

<!--Now, run the `cat-file` command on that SHA-1 value:-->

Jetzt wendest Du den Befehl `git cat-file` auf diesen SHA-1-Hash an:

	$ git cat-file -p 9585191f37f7b0fb9444f35a9bf50de191beadc2
	object 1a410efbd13591db07496601ebc7a059dd55cfe9
	type commit
	tag v1.1
	tagger Scott Chacon <schacon@gmail.com> Sat May 23 16:48:58 2009 -0700

	test tag

<!--Notice that the object entry points to the commit SHA-1 value that you tagged. Also notice that it doesn’t need to point to a commit; you can tag any Git object. In the Git source code, for example, the maintainer has added their GPG public key as a blob object and then tagged it. You can view the public key by running-->

Beachte, dass der der Wert `object` auf den SHA-1 des Commits zeigt, den Du getaggt hast. Weiterhin muss der Eintrag nicht auf einen Commit zeigen. In Git kann man jedes beliebige Objekt taggen. Im Git-Quellcode befindet sich beispielweise der öffentliche GPG-Schlüssel des Projektbetreibers als ein Blob-Objekt, sowie ein Tag, der darauf zeigt. Du kannst Dir den öffentlichen Schlüssel anzeigen lassen, indem du den folgenden Befehl im Git-Quellcode-Repository ausführst:

	$ git cat-file blob junio-gpg-pub

<!--in the Git source code repository. The Linux kernel repository also has a non-commit-pointing tag object — the first tag created points to the initial tree of the import of the source code.-->

Der Linux-Kernel hat auch ein Tag-Objekt, das nicht auf einen Commit zeigt – der erste erstellte Tag zeigt auf den anfänglichen Tree des Quelltext-Imports.

<!--### Remotes ###-->
### Externe Referenzen ###

<!--The third type of reference that you’ll see is a remote reference. If you add a remote and push to it, Git stores the value you last pushed to that remote for each branch in the `refs/remotes` directory. For instance, you can add a remote called `origin` and push your `master` branch to it:-->

Der dritte Referenztyp ist die externe Referenz („remote reference“). Wenn Du einen externen Server („remote“) definierst und dorthin pushst, merkt sich Git den zuletzt gepushten Commit für jeden Branch im `refs/remotes` Verzeichnis. Bespielsweise fügst Du einen externen Server `origin` hinzu und pushst Deinen Branch `master` dorthin:

	$ git remote add origin git@github.com:schacon/simplegit-progit.git
	$ git push origin master
	Counting objects: 11, done.
	Compressing objects: 100% (5/5), done.
	Writing objects: 100% (7/7), 716 bytes, done.
	Total 7 (delta 2), reused 4 (delta 1)
	To git@github.com:schacon/simplegit-progit.git
	   a11bef0..ca82a6d  master -> master

<!--Then, you can see what the `master` branch on the `origin` remote was the last time you communicated with the server, by checking the `refs/remotes/origin/master` file:-->

Dann kannst Du herausfinden, in welchem Zustand sich der Branch `master` auf dem Server `origin` zuletzt befand (d.h. als Du das letzte Mal mit ihm kommuniziert hast), indem Du Dir die Datei `refs/remotes/origin/master` anschaust:

	$ cat .git/refs/remotes/origin/master
	ca82a6dff817ec66f44342007202690a93763949

<!--Remote references differ from branches (`refs/heads` references) mainly in that they can’t be checked out. Git moves them around as bookmarks to the last known state of where those branches were on those servers.-->

Externe Referenzen unterscheiden sich von Branches (`refs/heads`) hauptsächlich dadurch, dass man sie nicht auschecken kann. Git verwendet sie quasi als Lesezeichen für den zuletzt bekannten Status, in dem sich die Branches auf externen Servern jeweils befanden.

<!--## Packfiles ##-->
## Pack-Dateien ##

<!--Let’s go back to the objects database for your test Git repository. At this point, you have 11 objects — 4 blobs, 3 trees, 3 commits, and 1 tag:-->

Kommen wir noch einmal auf die Objekt-Datenbank zurück, die Du für Dein Test-Repository angelegt hast. Im Moment müsstest Du 11 Objekte haben: 4 Blobs, 3 Trees, 3 Commits und 1 Tag:

	$ find .git/objects -type f
	.git/objects/01/55eb4229851634a0f03eb265b69f5a2d56f341 # tree 2
	.git/objects/1a/410efbd13591db07496601ebc7a059dd55cfe9 # commit 3
	.git/objects/1f/7a7a472abf3dd9643fd615f6da379c4acb3e3a # test.txt v2
	.git/objects/3c/4e9cd789d88d8d89c1073707c3585e41b0e614 # tree 3
	.git/objects/83/baae61804e65cc73a7201a7252750c76066a30 # test.txt v1
	.git/objects/95/85191f37f7b0fb9444f35a9bf50de191beadc2 # tag
	.git/objects/ca/c0cab538b970a37ea1e769cbbde608743bc96d # commit 2
	.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4 # 'test content'
	.git/objects/d8/329fc1cc938780ffdd9f94e0d364e0ea74f579 # tree 1
	.git/objects/fa/49b077972391ad58037050f2a75f74e3671e92 # new.txt
	.git/objects/fd/f4fc3344e67ab068f836878b6c4951e3b15f3d # commit 1

<!--Git compresses the contents of these files with zlib, and you’re not storing much, so all these files collectively take up only 925 bytes. You’ll add some larger content to the repository to demonstrate an interesting feature of Git. Add the repo.rb file from the Grit library you worked with earlier — this is about a 12K source code file:-->

Git komprimiert die Inhalte dieser Dateien mit zlib und Du hast nicht sonderlich viele davon, sodass die Gesamtgröße der Dateien gerade mal 925 Bytes beträgt. Wir wollen ein anderes interessantes Feature von Git demonstrieren, und dazu müssen wir eine größere Datei hinzufügen, z.B. die `repo.rb` Datei aus der Grit-Bibliothek, die Du schon verwendet hast. Diese Datei ist eine etwa 12K große Quelltext-Datei:

	$ curl -L https://raw.github.com/mojombo/grit/master/lib/grit/repo.rb > repo.rb
	$ git add repo.rb
	$ git commit -m 'added repo.rb'
	[master 484a592] added repo.rb
	 3 files changed, 459 insertions(+), 2 deletions(-)
	 delete mode 100644 bak/test.txt
	 create mode 100644 repo.rb
	 rewrite test.txt (100%)

<!--If you look at the resulting tree, you can see the SHA-1 value your repo.rb file got for the blob object:-->

Wenn Du Dir den resultierenden Tree anschaust, findest Du den SHA-1-Hash, den die Datei `repo.rb` für das Blob-Objekt erhalten hat:

	$ git cat-file -p master^{tree}
	100644 blob fa49b077972391ad58037050f2a75f74e3671e92      new.txt
	100644 blob 9bc1dc421dcd51b4ac296e3e5b6e2a99cf44391e      repo.rb
	100644 blob e3f094f522629ae358806b17daf78246c27c007b      test.txt

<!--You can then check how big is that object on your disk:-->

Jetzt kannst Du mit `git cat-file` sehen, wie groß das Objekt ist:

	$ du -b .git/objects/9b/c1dc421dcd51b4ac296e3e5b6e2a99cf44391e
	4102	.git/objects/9b/c1dc421dcd51b4ac296e3e5b6e2a99cf44391e

<!--Now, modify that file a little, and see what happens:-->

Als Nächstes ändern wir die Datei ein bisschen, um zu sehen, was passiert:

	$ echo '# testing' >> repo.rb
	$ git commit -am 'modified repo a bit'
	[master ab1afef] modified repo a bit
	 1 files changed, 1 insertions(+), 0 deletions(-)

<!--Check the tree created by that commit, and you see something interesting:-->

Wenn Du jetzt den Tree anschaust, der durch den Commit angelegt wurde, findest Du etwas Interessantes:

	$ git cat-file -p master^{tree}
	100644 blob fa49b077972391ad58037050f2a75f74e3671e92      new.txt
	100644 blob 05408d195263d853f09dca71d55116663690c27c      repo.rb
	100644 blob e3f094f522629ae358806b17daf78246c27c007b      test.txt

<!--The blob is now a different blob, which means that although you added only a single line to the end of a 400-line file, Git stored that new content as a completely new object:-->

Das Blob ist ein anderes, d.h. obwohl Du lediglich eine einzige Zeile an das Ende einer 400 Zeilen langen Datei angehängt hast, speichert Git den Inhalt jetzt als ein ganz neues Objekt:

	$ du -b .git/objects/05/408d195263d853f09dca71d55116663690c27c
	4109	.git/objects/05/408d195263d853f09dca71d55116663690c27c

<!--You have two nearly identical 4K objects on your disk. Wouldn’t it be nice if Git could store one of them in full but then the second object only as the delta between it and the first?-->

Du hast jetzt zwei fast identische 12K große Objekte auf Deiner Festplatte. Wäre es nicht besser, wenn Git nur das erste vollständig und das zweite lediglich als ein Delta zwischen dem ersten und dem zweiten speichern würde?

<!--It turns out that it can. The initial format in which Git saves objects on disk is called a loose object format. However, occasionally Git packs up several of these objects into a single binary file called a packfile in order to save space and be more efficient. Git does this if you have too many loose objects around, if you run the `git gc` command manually, or if you push to a remote server. To see what happens, you can manually ask Git to pack up the objects by calling the `git gc` command:-->

Tatsächlich kann Git das. Das ursprüngliche Format, in dem Git Objekte in der Datenbank speichert, wird als „freies Objekt-Format“ („loose object format“) bezeichnet. Hin und wieder packt Git allerdings eine Reihe solcher Objekte in eine einzige binäre Datei zusammen, um Platz zu sparen und effizienter zu arbeiten. Eine solche Datei wird als „packfile“ bezeichnet. Git tut das immer dann, wenn zu viele freie Objekte vorhanden sind, wenn Du den Befehl `git gc` manuell ausführst oder wenn Du auf einen externen Server pushst. Schauen wir uns also an, was passiert, wenn wir manuell `git gc` ausführen:

	$ git gc
	Counting objects: 17, done.
	Delta compression using 2 threads.
	Compressing objects: 100% (13/13), done.
	Writing objects: 100% (17/17), done.
	Total 17 (delta 1), reused 10 (delta 0)

<!--If you look in your objects directory, you’ll find that most of your objects are gone, and a new pair of files has appeared:-->

Wenn Du das Objekt-Verzeichnis anschaust, siehst Du, dass die meisten Objekte jetzt fehlen und dass stattdessen zwei neue Objekte aufgetaucht sind:

	$ find .git/objects -type f
	.git/objects/71/08f7ecb345ee9d0084193f147cdad4d2998293
	.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4
	.git/objects/info/packs
	.git/objects/pack/pack-7a16e4488ae40c7d2bc56ea2bd43e25212a66c45.idx
	.git/objects/pack/pack-7a16e4488ae40c7d2bc56ea2bd43e25212a66c45.pack

<!--The objects that remain are the blobs that aren’t pointed to by any commit — in this case, the "what is up, doc?" example and the "test content" example blobs you created earlier. Because you never added them to any commits, they’re considered dangling and aren’t packed up in your new packfile.-->

Die verbleibenden Objekte sind diejenigen Blobs, die nicht von irgendeinem Commit referenziert werden – in diesem Fall sind das die Beispielblobs „what is up, doc?“ und „test content“, die wir zuvor gespeichert hatten. Weil wir sie nie zu irgendeinem Commit hinzugefügt haben, werden sie als „dangling“ (wörtlich: herumbaumelnd) betrachtet und nicht im Packfile zusammengepackt.

<!--The other files are your new packfile and an index. The packfile is a single file containing the contents of all the objects that were removed from your filesystem. The index is a file that contains offsets into that packfile so you can quickly seek to a specific object. What is cool is that although the objects on disk before you ran the `gc` were collectively about 8K in size, the new packfile is only 4K. You’ve halved your disk usage by packing your objects.-->

Die beiden neuen Dateien sind das Packfile und ein Index. Das Packfile ist eine einzelne Datei, die die Inhalte all der Dateien umfasst, die jetzt aus dem Dateisystem entfernt worden sind. Der Index ist eine Datei, die auf Positionen von Objekten im Packfile zeigt, sodass Git schneller nach einem bestimmten Objekt suchen kann. Obwohl diese Objekte auf der Festplatte ingesamt 12K groß waren, bevor Du `git gc` ausgeführt hast, ist das Packfile jetzt nur 6K groß. D.h., Du hast den Platzverbrauch dadurch um die Hälfte reduziert. Toll, oder?

<!--How does Git do this? When Git packs objects, it looks for files that are named and sized similarly, and stores just the deltas from one version of the file to the next. You can look into the packfile and see what Git did to save space. The `git verify-pack` plumbing command allows you to see what was packed up:-->

Wie stellt Git das genau an? Wenn Git Objekte zusammen packt, sucht es nach Dateien, die ähnlich benannt und ähnlich groß sind, und speichert dann lediglich Deltas von einer Version zur nächsten. Du kannst ein Packfile inspizieren, um zu sehen, wie Git die Objekte gepackt hat. Der Plumbing Befehl `git verify-pack` macht das möglich:

	$ git verify-pack -v \
	  .git/objects/pack/pack-7a16e4488ae40c7d2bc56ea2bd43e25212a66c45.idx
	0155eb4229851634a0f03eb265b69f5a2d56f341 tree   71 76 5400
	05408d195263d853f09dca71d55116663690c27c blob   12908 3478 874
	09f01cea547666f58d6a8d809583841a7c6f0130 tree   106 107 5086
	1a410efbd13591db07496601ebc7a059dd55cfe9 commit 225 151 322
	1f7a7a472abf3dd9643fd615f6da379c4acb3e3a blob   10 19 5381
	3c4e9cd789d88d8d89c1073707c3585e41b0e614 tree   101 105 5211
	484a59275031909e19aadb7c92262719cfcdf19a commit 226 153 169
	83baae61804e65cc73a7201a7252750c76066a30 blob   10 19 5362
	9585191f37f7b0fb9444f35a9bf50de191beadc2 tag    136 127 5476
	9bc1dc421dcd51b4ac296e3e5b6e2a99cf44391e blob   7 18 5193 1 \
	  05408d195263d853f09dca71d55116663690c27c
	ab1afef80fac8e34258ff41fc1b867c702daa24b commit 232 157 12
	cac0cab538b970a37ea1e769cbbde608743bc96d commit 226 154 473
	d8329fc1cc938780ffdd9f94e0d364e0ea74f579 tree   36 46 5316
	e3f094f522629ae358806b17daf78246c27c007b blob   1486 734 4352
	f8f51d7d8a1760462eca26eebafde32087499533 tree   106 107 749
	fa49b077972391ad58037050f2a75f74e3671e92 blob   9 18 856
	fdf4fc3344e67ab068f836878b6c4951e3b15f3d commit 177 122 627
	chain length = 1: 1 object
	pack-7a16e4488ae40c7d2bc56ea2bd43e25212a66c45.pack: ok

<!--Here, the `9bc1d` blob, which if you remember was the first version of your repo.rb file, is referencing the `05408` blob, which was the second version of the file. The third column in the output is the size of the object’s content, so you can see that the content of `05408` takes up 12K, but that of `9bc1d` only takes up 7 bytes. What is also interesting is that the second version of the file is the one that is stored intact, whereas the original version is stored as a delta — this is because you’re most likely to need faster access to the most recent version of the file.-->

Du erinnerst dich, dass der Blob `9bc1d` die erste Version der `repo.rb`-Datei ist. Dieser Blob referenziert jetzt den  Blob`05408`, der die zweite Version der Datei ist. Die dritte Spalte der Ausgabe ist die Größe des Objektes im Packfile. Wir können also sehen, dass `05408` 12K in Anspruch nimmt, `9bc1d` aber nur 7 Bytes. Das bedeutet also, dass die zweite Version diejenige ist, die vollständig, während die ursprüngliche, erste Version als Delta gespeichert wird! Der Grund dafür ist, dass Du höchstwahrscheinlich einen schnelleren Zugriff auf die jeweils neuesten Dateien brauchst.

<!--The really nice thing about this is that it can be repacked at any time. Git will occasionally repack your database automatically, always trying to save more space. You can also manually repack at any time by running `git gc` by hand.-->

Außerdem ist toll, dass ein Repository jederzeit neu gepackt werden kann. Git macht das gelegentlich automatisch, um weniger Platz für die Datenbank zu verbrauchen. Du kannst sie aber auch jederzeit manuell mit `git gc` packen.

<!--## The Refspec ##-->
## Die Refspec ##

<!--Throughout this book, you’ve used simple mappings from remote branches to local references; but they can be more complex.-->
<!--Suppose you add a remote like this:-->

In diesem Buch haben wir bisher einfache Zuweisungen von externen Branches auf lokale Referenzen verwendet. Sie können aber auch durchaus komplex sein. Nehmen wir an, Du hast ein Remote-Repository wie folgt definiert:

	$ git remote add origin git@github.com:schacon/simplegit-progit.git

<!--It adds a section to your `.git/config` file, specifying the name of the remote (`origin`), the URL of the remote repository, and the refspec for fetching:-->

Das fügt eine Sektion in Deine `.git/config`-Datei hinzu, die Deinen lokalen Namen des externen Repositorys (`origin`), dessen URL und die Refspec spezifiziert, mit der neue Daten heruntergeladen werden.

	[remote "origin"]
	       url = git@github.com:schacon/simplegit-progit.git
	       fetch = +refs/heads/*:refs/remotes/origin/*

<!--The format of the refspec is an optional `+`, followed by `<src>:<dst>`, where `<src>` is the pattern for references on the remote side and `<dst>` is where those references will be written locally. The `+` tells Git to update the reference even if it isn’t a fast-forward.-->

Das Format der Refspec besteht aus einem optionalen `+` gefolgt von `<Quelle>:<Ziel>`, wobei `<Quelle>` ein Muster für Referenzen auf der Remote-Seite ist, und `<Ziel>` angibt, wohin diese Referenzen lokal geschrieben werden. Das `+` weist Git an, die Referenz zu mergen, wenn sie nicht mit einem Fast-forward aktualisiert werden kann.

<!--In the default case that is automatically written by a `git remote add` command, Git fetches all the references under `refs/heads/` on the server and writes them to `refs/remotes/origin/` locally. So, if there is a `master` branch on the server, you can access the log of that branch locally via-->

Der Standard, der von `git remote add` automatisch eingerichtet wird, besteht darin, dass Git automatisch alle Referenzen unter `refs/heads/` vom Server holt und sie lokal nach `refs/remotes/origin` speichert. D.h., wenn es auf dem Server einen Branch `master` gibt, kannst Du auf das Log dieses Branches wie folgt zugreifen:

	$ git log origin/master
	$ git log remotes/origin/master
	$ git log refs/remotes/origin/master

<!--They’re all equivalent, because Git expands each of them to `refs/remotes/origin/master`.-->

Diese Varianten sind allesamt äquivalent, weil Git sie jeweils zu `refs/remotes/origin/master` vervollständigt.

<!--If you want Git instead to pull down only the `master` branch each time, and not every other branch on the remote server, you can change the fetch line to-->

Wenn Du stattdessen willst, dass Git jeweils nur den Branch `master` herunterlädt und andere Branches auf dem Server ignoriert, kannst Du die `fetch`-Zeile wie folgt ändern:

	fetch = +refs/heads/master:refs/remotes/origin/master

<!--This is just the default refspec for `git fetch` for that remote. If you want to do something one time, you can specify the refspec on the command line, too. To pull the `master` branch on the remote down to `origin/mymaster` locally, you can run-->

Dies ist allerdings lediglich der Standardwert der Refspec und Du kannst ihn auf der Kommandozeile jederzeit überschreiben. Um zum Beispiel nur den Branch `master` vom Server lokal als `origin/mymaster` zu speichern, kannst Du Folgendes ausführen:

	$ git fetch origin master:refs/remotes/origin/mymaster

<!--You can also specify multiple refspecs. On the command line, you can pull down several branches like so:-->

Du kannst auch mehrere Refspecs gleichzeitig spezifizieren. Um mehrere Branches zu holen kannst du folgenden Befehl in die Kommandozeile eingeben:

	$ git fetch origin master:refs/remotes/origin/mymaster \
	   topic:refs/remotes/origin/topic
	From git@github.com:schacon/simplegit
	 ! [rejected]        master     -> origin/mymaster  (non fast forward)
	 * [new branch]      topic      -> origin/topic

<!--In this case, the master branch pull was rejected because it wasn’t a fast-forward reference. You can override that by specifying the `+` in front of the refspec.-->

In diesem Fall wurde ein Pull zurückgewiesen, weil der Branch nicht mit einem simplen Fast-forward aktualisiert werden konnte. Du kannst einen Merge erzwingen, indem Du der Refspec ein `+` voranstellst.

<!--You can also specify multiple refspecs for fetching in your configuration file. If you want to always fetch the master and experiment branches, add two lines:-->

Du kannst außerdem natürlich auch mehrere Refspecs in Deiner Konfiguration spezifizieren. Wenn Du z.B. immer die Branches `master` und `experiment` holen willst, fügst Du die folgenden Zeilen hinzu:

	[remote "origin"]
	       url = git@github.com:schacon/simplegit-progit.git
	       fetch = +refs/heads/master:refs/remotes/origin/master
	       fetch = +refs/heads/experiment:refs/remotes/origin/experiment

<!--You can’t use partial globs in the pattern, so this would be invalid:-->

Du kannst keine partiellen Glob-Muster verwenden, d.h. Folgendes wäre ungültig:

	fetch = +refs/heads/qa*:refs/remotes/origin/qa*

<!--However, you can use namespacing to accomplish something like that. If you have a QA team that pushes a series of branches, and you want to get the master branch and any of the QA team’s branches but nothing else, you can use a config section like this:-->

Allerdings kannst Du Namensräume verwenden, um etwas Ähnliches zu erreichen. Nehmen wir an, Du hast ein QA-Team, das regelmäßig verschiedene Branches pusht, und Du willst nun den Branch master und sämtliche Branches des QA-Teams, aber keine anderen Branches haben. Dann kannst Du eine Config-Sektion wie die folgende verwenden:

	[remote "origin"]
	       url = git@github.com:schacon/simplegit-progit.git
	       fetch = +refs/heads/master:refs/remotes/origin/master
	       fetch = +refs/heads/qa/*:refs/remotes/origin/qa/*

<!--If you have a complex workflow process that has a QA team pushing branches, developers pushing branches, and integration teams pushing and collaborating on remote branches, you can namespace them easily this way.-->

In einem großen Team mit einem komplexen Workflow, in dem ein QA-Team, Entwickler und ein Integrations-Team jeweils eigene Branches pushen, kann man auf diese Weise Branches einfach in Namensräume einteilen.

<!--### Pushing Refspecs ###-->
### Refspecs pushen ###

<!--It’s nice that you can fetch namespaced references that way, but how does the QA team get their branches into a `qa/` namespace in the first place? You accomplish that by using refspecs to push.-->

Wie aber legt das QA-Team die Branches im `qa/` Namensraum ab? Das geht, indem man mit einer Refspec pusht.

<!--If the QA team wants to push their `master` branch to `qa/master` on the remote server, they can run-->

Wenn das QA-Team seinen Branch `master` in einem externen Repository als `qa/master` speichern will, kann es das wie folgt tun:

	$ git push origin master:refs/heads/qa/master

<!--If they want Git to do that automatically each time they run `git push origin`, they can add a `push` value to their config file:-->

Um Git so zu konfigurieren, dass diese Refspec jedes Mal automatisch für `git push origin` verwendet wird, kann man den `push` Wert in der Config-Datei setzen:

	[remote "origin"]
	       url = git@github.com:schacon/simplegit-progit.git
	       fetch = +refs/heads/*:refs/remotes/origin/*
	       push = refs/heads/master:refs/heads/qa/master

<!--Again, this will cause a `git push origin` to push the local `master` branch to the remote `qa/master` branch by default.-->

Auf diese Weise wird `git push origin` den lokalen Branch `master` als `qa/master` auf dem Server `origin` speichern.

<!--### Deleting References ###-->
### Referenzen löschen ###

<!--You can also use the refspec to delete references from the remote server by running something like this:-->

Man kann Refspecs außerdem verwenden, um Referenzen aus einem externen Repository zu löschen:

	$ git push origin :topic

<!--Because the refspec is `<src>:<dst>`, by leaving off the `<src>` part, this basically says to make the topic branch on the remote nothing, which deletes it.-->

Das Refspec Format ist `<Quelle>:<Ziel>`. Wenn man den Teil `<Quelle>` weglässt, dann heißt das im obigen Beispiel, dass man den Branch `topic` auf dem Server `origin` auf „nichts“ setzt, d.h. also löscht.

<!--## Transfer Protocols ##-->
## Transfer-Protokolle ##

<!--Git can transfer data between two repositories in two major ways: over HTTP and via the so-called smart protocols used in the `file://`, `ssh://`, and `git://` transports. This section will quickly cover how these two main protocols operate.-->

Git kann Daten zwischen zwei Repositorys im Wesentlichen auf zwei Arten transportieren: über HTTP und über sogenannte smarte Protokolle, die mit `file://`, `ssh://` und `git://` verwendet werden. Die folgende Sektion gibt einen kurzen Überblick über diese Protokolle und wie sie funktionieren.

<!--### The Dumb Protocol ###-->
### Das dumme Protokoll ###

<!--Git transport over HTTP is often referred to as the dumb protocol because it requires no Git-specific code on the server side during the transport process. The fetch process is a series of GET requests, where the client can assume the layout of the Git repository on the server. Let’s follow the `http-fetch` process for the simplegit library:-->

Das HTTP-Transfer-Protokoll von Git wird oft auch als „dummes“ Protokoll bezeichnet, weil es auf der Server-Seite keinen Git-spezifischen Code benötigt. Der `fetch`-Prozess besteht aus einer Reihe von GET-Requests, für die der Client Vorannahmen über das Layout des Git-Repositorys auf dem Server machen kann. Schauen wir uns den `http-fetch`-Prozess der Bibliothek `simplegit` an:

	$ git clone http://github.com/schacon/simplegit-progit.git

<!--The first thing this command does is pull down the `info/refs` file. This file is written by the `update-server-info` command, which is why you need to enable that as a `post-receive` hook in order for the HTTP transport to work properly:-->

Der Befehl lädt zunächst die Datei `info/refs` herunter. Diese Datei wird vom Befehl `update-server-info` geschrieben, den man als einen `post-receive`-Hook einrichten muss, damit das HTTP-Protokoll richtig funktionieren kann.

	=> GET info/refs
	ca82a6dff817ec66f44342007202690a93763949     refs/heads/master

<!--Now you have a list of the remote references and SHAs. Next, you look for what the HEAD reference is so you know what to check out when you’re finished:-->

Jetzt hat man eine Liste aller Referenzen und SHA-Prüfsummen in diesem Repository. Als nächstes schaut man die HEAD-Referenz nach, um zu wissen, was ausgecheckt werden muss:

	=> GET HEAD
	ref: refs/heads/master

<!--You need to check out the `master` branch when you’ve completed the process.-->

D.h., wenn wir mit dem Prozess fertig sind, wir müssen den Branch `master` auschecken.

<!--At this point, you’re ready to start the walking process. Because your starting point is the `ca82a6` commit object you saw in the `info/refs` file, you start by fetching that:-->

Wir können jetzt loslegen. Weil in der Datei `info/refs` der Commit `ca82a6` angegeben ist, fangen wir damit an, dieses Objekt herunterzuladen:

	=> GET objects/ca/82a6dff817ec66f44342007202690a93763949
	(179 bytes of binary data)

<!--You get an object back — that object is in loose format on the server, and you fetched it over a static HTTP GET request. You can zlib-uncompress it, strip off the header, and look at the commit content:-->

Wir erhalten also ein Objekt zurück. Dieses Objekt ist im losen Format auf dem Server gespeichert, und wir haben es über einen statischen HTTP-GET-Request herunter geladen. Jetzt können wir es mit zlib dekomprimieren, den Header entfernen und den Inhalt des Commits durchsehen:

	$ git cat-file -p ca82a6dff817ec66f44342007202690a93763949
	tree cfda3bf379e4f8dba8717dee55aab78aef7f4daf
	parent 085bb3bcb608e1e8451d4b2432f8ecbe6306e7e7
	author Scott Chacon <schacon@gmail.com> 1205815931 -0700
	committer Scott Chacon <schacon@gmail.com> 1240030591 -0700

	changed the version number

<!--Next, you have two more objects to retrieve — `cfda3b`, which is the tree of content that the commit we just retrieved points to; and `085bb3`, which is the parent commit:-->

Als Nächstes brauchen wir also zwei weitere Objekte: `cfda3b`, welches der Tree der Inhalte dieses Commits ist, und `085bb3`, welches der übergeordnete Commit ist:

	=> GET objects/08/5bb3bcb608e1e8451d4b2432f8ecbe6306e7e7
	(179 bytes of data)

<!--That gives you your next commit object. Grab the tree object:-->

Das gibt uns das nächste Commit-Objekt. Versuchen wir, das Tree-Objekt zu holen:

	=> GET objects/cf/da3bf379e4f8dba8717dee55aab78aef7f4daf
	(404 - Not Found)

<!--Oops — it looks like that tree object isn’t in loose format on the server, so you get a 404 response back. There are a couple of reasons for this — the object could be in an alternate repository, or it could be in a packfile in this repository. Git checks for any listed alternates first:-->

Huch. Es sieht so aus, als ob der Tree nicht im losen Format auf dem Server gespeichert ist, weshalb wir eine 404-Antwort („Not found“) erhalten. Dafür kann es verschiedene Gründe geben. Das Objekt könnte in einem anderen, alternativen Repository liegen, oder es könnte sich in einem Packfile befinden. Git sucht deshalb zunächst nach alternativen Repositories:

	=> GET objects/info/http-alternates
	(empty file)

	=> GET objects/info/http-alternates
	(leere Datei)

<!--If this comes back with a list of alternate URLs, Git checks for loose files and packfiles there — this is a nice mechanism for projects that are forks of one another to share objects on disk. However, because no alternates are listed in this case, your object must be in a packfile. To see what packfiles are available on this server, you need to get the `objects/info/packs` file, which contains a listing of them (also generated by `update-server-info`):-->

Wenn wir hier eine Liste alternativer URLs erhalten, schaut Git dort nach losen Dateien und Packfiles. Auf diese Weise können Repositorys, die Forks von anderen Repositorys sind, mit diesen Objekte im Dateisystem teilen. In unserem Fall sind allerdings keine Alternativen vorhanden, weshalb sich das gesuchte Objekt in einem Packfile befinden muss. Um die vorhandenen Packfiles nachzuschlagen, holt Git die `objects/info/packs` Datei, die eine entsprechende Auflistung enthält (und ebenfalls mit `update-server-info` erzeugt wird)

	=> GET objects/info/packs
	P pack-816a9b2334da9953e530f27bcac22082a9f5b835.pack

<!--There is only one packfile on the server, so your object is obviously in there, but you’ll check the index file to make sure. This is also useful if you have multiple packfiles on the server, so you can see which packfile contains the object you need:-->

Es gibt nur ein einziges Packfile auf dem Server, weshalb sich unser Objekt darin befinden muss. Aber wir prüfen die Index-Datei, um sicher zu sein. Gäbe es mehrere Packfiles auf dem Server, könnten wir auf diese Weise herausfinden, welches Packfile das gesuchte Objekt enthält:

	=> GET objects/pack/pack-816a9b2334da9953e530f27bcac22082a9f5b835.idx
	(4k of binary data)

	=> GET objects/pack/pack-816a9b2334da9953e530f27bcac22082a9f5b835.idx
	(4k binäre Daten)

<!--Now that you have the packfile index, you can see if your object is in it — because the index lists the SHAs of the objects contained in the packfile and the offsets to those objects. Your object is there, so go ahead and get the whole packfile:-->

Nachdem wir jetzt den Packfile-Index haben, können wir prüfen, ob sich unser Objekt darin befindet: der Index enthält eine Liste der SHA-Hashes der Objekte, die sich im Packfile befinden und die jeweiligen Offsets dieser Objekte. Unser Objekt ist vorhanden, also laden wir das Packfile herunter:

	=> GET objects/pack/pack-816a9b2334da9953e530f27bcac22082a9f5b835.pack
	(13k of binary data)

	=> GET objects/pack/pack-816a9b2334da9953e530f27bcac22082a9f5b835.pack
	(13k binäre Daten)

<!--You have your tree object, so you continue walking your commits. They’re all also within the packfile you just downloaded, so you don’t have to do any more requests to your server. Git checks out a working copy of the `master` branch that was pointed to by the HEAD reference you downloaded at the beginning.-->

Du hast jetzt das Tree-Objekt, also kannst Du jetzt damit fortfahren, über die Commits zu iterieren. Sie sind in unserem Fall allesamt in dem Packfile enthalten, das Du gerade heruntergeladen hast.

<!--The entire output of this process looks like this:-->

Die Ausgabe des ganzen Vorgangs sieht dann in etwa so aus:

	$ git clone http://github.com/schacon/simplegit-progit.git
	Initialized empty Git repository in /private/tmp/simplegit-progit/.git/
	got ca82a6dff817ec66f44342007202690a93763949
	walk ca82a6dff817ec66f44342007202690a93763949
	got 085bb3bcb608e1e8451d4b2432f8ecbe6306e7e7
	Getting alternates list for http://github.com/schacon/simplegit-progit.git
	Getting pack list for http://github.com/schacon/simplegit-progit.git
	Getting index for pack 816a9b2334da9953e530f27bcac22082a9f5b835
	Getting pack 816a9b2334da9953e530f27bcac22082a9f5b835
	 which contains cfda3bf379e4f8dba8717dee55aab78aef7f4daf
	walk 085bb3bcb608e1e8451d4b2432f8ecbe6306e7e7
	walk a11bef06a3f659402fe7563abf99ad00de2209e6

<!--### The Smart Protocol ###-->
### Das schlaue Protokoll ###

<!--The HTTP method is simple but a bit inefficient. Using smart protocols is a more common method of transferring data. These protocols have a process on the remote end that is intelligent about Git — it can read local data and figure out what the client has or needs and generate custom data for it. There are two sets of processes for transferring data: a pair for uploading data and a pair for downloading data.-->

Die HTTP-Methode ist simpel, aber sie ist auch ein bisschen ineffizient. Deshalb ist es üblicher, ein schlaues Protokoll für den Datentransfer zu verwenden. Diese Protokolle umfassen serverseitige Prozesse, die Wissen über Git besitzen. Sie können lokale Daten lesen und herausfinden, was auf dem Client schon vorhanden ist oder fehlt und darauf zugeschnittene Daten generieren. Es gibt zwei Sets von Prozessen für den Datentransfer: ein Paar für den Upload und ein Paar für den Download von Daten.

<!--#### Uploading Data ####-->
#### Daten hochladen ####

<!--To upload data to a remote process, Git uses the `send-pack` and `receive-pack` processes. The `send-pack` process runs on the client and connects to a `receive-pack` process on the remote side.-->

Um Daten an einen serverseitigen Prozess zu schicken, verwendet Git die Prozesse `send-pack` und `receive-pack`. Der Prozess `send-pack` läuft auf dem Client und verbindet sich mit einem `receive-pack`-Prozess auf dem Server.

<!--For example, say you run `git push origin master` in your project, and `origin` is defined as a URL that uses the SSH protocol. Git fires up the `send-pack` process, which initiates a connection over SSH to your server. It tries to run a command on the remote server via an SSH call that looks something like this:-->

Nehmen wir z.B. an, Du führst `git push origin master` in Deinem Projekt aus und `origin` ist als eine URL mit SSH-Protokoll definiert. Git startet dann einen `send-pack`-Prozess, der eine SSH-Verbindung zum Server initiiert. Dieser versucht, via SSH auf dem Server einen Befehl wie den folgenden auszuführen:

	$ ssh -x git@github.com "git-receive-pack 'schacon/simplegit-progit.git'"
	005bca82a6dff817ec66f4437202690a93763949 refs/heads/master report-status delete-refs
	003e085bb3bcb608e1e84b2432f8ecbe6306e7e7 refs/heads/topic
	0000

<!--The `git-receive-pack` command immediately responds with one line for each reference it currently has — in this case, just the `master` branch and its SHA. The first line also has a list of the server’s capabilities (here, `report-status` and `delete-refs`).-->

Der `git-receive-pack`-Befehl antwortet dann mit jeweils einer Zeile pro Referenz, die er kennt – in diesem Fall sind das lediglich der Branch `master` und dessen SHA-Prüfsumme. Die erste Zeile listet außerdem Features, die der Server beherrscht (in unserem Fall `report-status` und `delete-refs`).

<!--Each line starts with a 4-byte hex value specifying how long the rest of the line is. Your first line starts with 005b, which is 91 in hex, meaning that 91 bytes remain on that line. The next line starts with 003e, which is 62, so you read the remaining 62 bytes. The next line is 0000, meaning the server is done with its references listing.-->

Jede Zeile beginnt mit einem 4 Byte Hexadezimalzahl-Wert, der angibt, wie lang der Rest der Zeile ist. Die erste Zeile beginnt mit 005b, d.h. dezimal 91. ALso ist der Rest der Zeile 91 Zeichen lang. Die nächste Zeile fängt mit 003e an, also dezimal 62. Die letzte Zeile ist 0000, was das Ende der Liste anzeigt.

<!--Now that it knows the server’s state, your `send-pack` process determines what commits it has that the server doesn’t. For each reference that this push will update, the `send-pack` process tells the `receive-pack` process that information. For instance, if you’re updating the `master` branch and adding an `experiment` branch, the `send-pack` response may look something like this:-->

Nachdem Dein `send-pack`-Prozess jetzt den Zustand des Servers kennt, kann er als nächstes evaluieren, welche Commits lokal, aber nicht auf dem Server vorhanden sind. der `send-pack`-Prozess schickt diese Information für jede Referenz, auf die sich der `push`-Befehl bezieht, an den `receive-pack` Prozess. Wenn Du beispielsweise den Branch `master` aktualisierst und einen Branch `experiment` hinzufügst, dann könnte die Antwort auf `send-pack` so aussehen:

	0085ca82a6dff817ec66f44342007202690a93763949  15027957951b64cf874c3557a0f3547bd83b3ff6 refs/heads/master report-status
	00670000000000000000000000000000000000000000 cdfdb42577e2506715f8cfeacdbabc092bf63e8d refs/heads/experiment
	0000

<!--The SHA-1 value of all '0's means that nothing was there before — because you’re adding the experiment reference. If you were deleting a reference, you would see the opposite: all '0's on the right side.-->

Der SHA-1-Wert, der nur aus Nullen besteht, heißt, dass dort zuvor nichts war: Du fügst die `experiment`-Referenz ja neu hinzu. Würdest Du eine Referenz löschen, würdest Du das Gegenteil sehen: nur Nullen auf der rechten Seite.

<!--Git sends a line for each reference you’re updating with the old SHA, the new SHA, and the reference that is being updated. The first line also has the client’s capabilities. Next, the client uploads a packfile of all the objects the server doesn’t have yet. Finally, the server responds with a success (or failure) indication:-->

Pro Referenz, die Du aktualisierst, schickt Git eine Zeile mit dem alten SHA, dem neuen SHA und der jeweiligen Referenz, die aktualisiert wird. Die erste Zeile listet zudem die Server-Features auf. Als nächstes lädt der Client ein Packfile aller Objekte hoch, die der Server noch nicht kennt. Abschließend antwortet der Server mit einer Erfolgs- oder Fehlermeldung:

	000Aunpack ok

<!--#### Downloading Data ####-->
#### Daten herunterladen ####

<!--When you download data, the `fetch-pack` and `upload-pack` processes are involved. The client initiates a `fetch-pack` process that connects to an `upload-pack` process on the remote side to negotiate what data will be transferred down.-->

Wenn Du Daten herunterlädst, sind daran die Prozesse `fetch-pack` und `upload-pack` beteiligt. Der Client startet einen `fetch-pack`-Prozess, der sich mit einem `upload-pack`-Prozess auf dem Server verbindet, um auszuhandeln, welche Daten heruntergeladen werden sollen.

<!--There are different ways to initiate the `upload-pack` process on the remote repository. You can run via SSH in the same manner as the `receive-pack` process. You can also initiate the process via the Git daemon, which listens on a server on port 9418 by default. The `fetch-pack` process sends data that looks like this to the daemon after connecting:-->

Es gibt verschiedene Möglichkeiten, den `upload-pack`-Prozess auf dem Server zu starten: einerseits via SSH auf die gleiche Weise wie den `receive-pack`-Prozess. Und andererseits über den Git-Daemon, der standardmäßig auf dem Server auf dem Port 9418 läuft. Der `fetch-pack`-Prozess schickt etwa Folgendes an den Daemon:

	003fgit-upload-pack schacon/simplegit-progit.git\0host=myserver.com\0

<!--It starts with the 4 bytes specifying how much data is following, then the command to run followed by a null byte, and then the server’s hostname followed by a final null byte. The Git daemon checks that the command can be run and that the repository exists and has public permissions. If everything is cool, it fires up the `upload-pack` process and hands off the request to it.-->

Diese Zeile beginnt wiederum mit 4 Bytes, die angeben, wieviel Daten folgen. Dann kommt der auszuführende Befehl und ein Null-Byte, und schließlich der Hostname des Servers und ein weiteres Null-Byte. Der Git-Daemon prüft, ob der Befehl ausgeführt werden kann, das Repository existiert und Schreibzugriff erlaubt. Wenn alles stimmt, startet er den `upload-pack`-Prozess und gibt den Request dorthin weiter.

<!--If you’re doing the fetch over SSH, `fetch-pack` instead runs something like this:-->

Wenn Du den `fetch`-Befehl über SSH verwendest, führt `fetch-pack` stattdessen etwas aus wie:

	$ ssh -x git@github.com "git-upload-pack 'schacon/simplegit-progit.git'"

<!--In either case, after `fetch-pack` connects, `upload-pack` sends back something like this:-->

In beiden Fällen wird, nachdem `fetch-pack` verbunden ist, `upload-pack` eine Antwort wie die folgende zurückschicken:

	0088ca82a6dff817ec66f44342007202690a93763949 HEAD\0multi_ack thin-pack \
	  side-band side-band-64k ofs-delta shallow no-progress include-tag
	003fca82a6dff817ec66f44342007202690a93763949 refs/heads/master
	003e085bb3bcb608e1e8451d4b2432f8ecbe6306e7e7 refs/heads/topic
	0000

<!--This is very similar to what `receive-pack` responds with, but the capabilities are different. In addition, it sends back the HEAD reference so the client knows what to check out if this is a clone.-->

Die Antwort ähnelt der, mit der `receive-pack` antwortet, aber die aufgelisteten Features sind andere. Zusätzlich wird die HEAD-Referenz mitgeschickt, sodass der Client weiß, was er auschecken muss, falls es sich um einen Clone handelt.

<!--At this point, the `fetch-pack` process looks at what objects it has and responds with the objects that it needs by sending "want" and then the SHA it wants. It sends all the objects it already has with "have" and then the SHA. At the end of this list, it writes "done" to initiate the `upload-pack` process to begin sending the packfile of the data it needs:-->

Der `fetch-pack`-Prozess inspiziert jetzt die vorhandenen Objekte und antwortet mit einer Liste von Objekten, wobei er das Schlüsselwort „want“ für Objekte verwendet, die benötigt werden, und „have“ für Objekte, die bereits vorhanden sind. Am Ende der Liste folgt das Schlüsselwort „done“. Der `upload-pack`-Prozess schickt dann ein Packfile mit allen benötigten Objekten:

	0054want ca82a6dff817ec66f44342007202690a93763949 ofs-delta
	0032have 085bb3bcb608e1e8451d4b2432f8ecbe6306e7e7
	0000
	0009done

<!--That is a very basic case of the transfer protocols. In more complex cases, the client supports `multi_ack` or `side-band` capabilities; but this example shows you the basic back and forth used by the smart protocol processes.-->

Das ist ein sehr einfaches Beispiel. In komplexeren Fällen unterstützt der Client die Features `multi_ack` oder `side-band`. Aber obiges Beispiel verdeutlicht den grundlegenden Request-Response Zyklus der Prozesse bei schlauen Protokollen.

<!--## Maintenance and Data Recovery ##-->
## Wartung und Datenwiederherstellung ##

<!--Occasionally, you may have to do some cleanup — make a repository more compact, clean up an imported repository, or recover lost work. This section will cover some of these scenarios.-->

Gelegentlich will man ein bisschen aufräumen – ein Repository komprimieren, ein importiertes Repository aufräumen oder verloren gegangene Daten wiederherstellen. Dieses Kapitel wird sich mit einigen derartigen Szenarien befassen.

<!--### Maintenance ###-->
### Wartung ###

<!--Occasionally, Git automatically runs a command called "auto gc". Most of the time, this command does nothing. However, if there are too many loose objects (objects not in a packfile) or too many packfiles, Git launches a full-fledged `git gc` command. The `gc` stands for garbage collect, and the command does a number of things: it gathers up all the loose objects and places them in packfiles, it consolidates packfiles into one big packfile, and it removes objects that aren’t reachable from any commit and are a few months old.-->

Git führt den Befehl `auto gc` hin und wieder automatisch aus. In den meisten Fällen tut dieser Befehl nichts. Wenn allerdings zu viele lose Objekte (d.h. Objekte, die nicht in einem Packfile gepackt sind) oder zu viele einzelne Packfiles vorhanden sind, führt Git den Befehl `git gc` aus. `gc` steht für „Garbage Collection“. Dieser Befehl führt eine Reihe von Aufgaben durch: er sammelt die losen Objekte und packt sie in ein Packfile, er führt einzelne Packfiles zu einem einzigen, großen Packfile zusammen, und er entfernt Objekte, die mit keinem Commit erreichbar und einige Monate alt sind.

<!--You can run auto gc manually as follows:-->

Du kannst `auto gc` wie folgt manuell ausführen:

	$ git gc --auto

<!--Again, this generally does nothing. You must have around 7,000 loose objects or more than 50 packfiles for Git to fire up a real gc command. You can modify these limits with the `gc.auto` and `gc.autopacklimit` config settings, respectively.-->

Wie schon erwähnt tut dies normalerweise gar nichts. Es müssen sich etwa 7.000 lose Objekte oder mehr als 50 Packfiles angesammelt haben, bevor Git tatsächlich den echten gc-Befehl startet. Du kannst diese Werte mit Hilfe der Konfigurationsvariablen `gc.auto` bzw. `gc.autopacklimit` manuell setzen.

<!--The other thing `gc` will do is pack up your references into a single file. Suppose your repository contains the following branches and tags:-->

`gc` packt außerdem Referenzen in eine einzige Datei zusammen. Nehmen wir an, Dein Repository enthält die folgenden Branches und Tags:

	$ find .git/refs -type f
	.git/refs/heads/experiment
	.git/refs/heads/master
	.git/refs/tags/v1.0
	.git/refs/tags/v1.1

<!--If you run `git gc`, you’ll no longer have these files in the `refs` directory. Git will move them for the sake of efficiency into a file named `.git/packed-refs` that looks like this:-->

Nachdem Du `git gc` ausgeführt hast, werden diese Dateien um der Effizienz willen aus dem Verzeichnis `refs` entfernt und in eine Datei `.git/packed-refs` verschoben, die dann wie folgt aussieht:

	$ cat .git/packed-refs
	# pack-refs with: peeled
	cac0cab538b970a37ea1e769cbbde608743bc96d refs/heads/experiment
	ab1afef80fac8e34258ff41fc1b867c702daa24b refs/heads/master
	cac0cab538b970a37ea1e769cbbde608743bc96d refs/tags/v1.0
	9585191f37f7b0fb9444f35a9bf50de191beadc2 refs/tags/v1.1
	^1a410efbd13591db07496601ebc7a059dd55cfe9

<!--If you update a reference, Git doesn’t edit this file but instead writes a new file to `refs/heads`. To get the appropriate SHA for a given reference, Git checks for that reference in the `refs` directory and then checks the `packed-refs` file as a fallback. However, if you can’t find a reference in the `refs` directory, it’s probably in your `packed-refs` file.-->

Wenn Du eine Referenz bearbeitest, lässt Git diese Datei unverändert und schreibt stattdessen eine neue Datei nach `refs/heads`. Um eine SHA-Prüfsumme für eine Referenz nachzuschlagen, schaut Git zunächst im Verzeichnis `refs` und danach erst in der Datei `packed-refs` nach, falls nötig. Wenn eine Referenz also nicht im Verzeichnis `refs` liegt, befindet sie sich wahrscheinlich in der Datei `packed-refs`.

<!--Notice the last line of the file, which begins with a `^`. This means the tag directly above is an annotated tag and that line is the commit that the annotated tag points to.-->

Beachte, dass die letzte Zeile der Datei mit `^` anfängt. Das bedeutet, dass der Tag darüber ein annotierter Tag ist und diese Zeile zeigt den Commit, auf den der annotierte Tag zeigt.

<!--### Data Recovery ###-->
### Daten-Wiederherstellung ###

<!--At some point in your Git journey, you may accidentally lose a commit. Generally, this happens because you force-delete a branch that had work on it, and it turns out you wanted the branch after all; or you hard-reset a branch, thus abandoning commits that you wanted something from. Assuming this happens, how can you get your commits back?-->

Irgendwann wird es vielleicht mal vorkommen, dass Du während der Arbeit mit Git einen Commit verlierst. Normalerweise passiert das, wenn Du versehentlich einen Branch löschst, an dem Du gearbeitet hattest und den Du noch brauchst. Oder Du führst `git reset --hard` aus und stellst fest, dass Du einige der Commits noch brauchst. Nehmen wir an, Du steckst in einer solchen Situation – wie kannst Du Deine Commits dann wiederherstellen?

<!--Here’s an example that hard-resets the master branch in your test repository to an older commit and then recovers the lost commits. First, let’s review where your repository is at this point:-->

Das folgende Beispiel setzt zuerst den Branch `master` auf einen älteren Commit zurück und stellt die verlorenen Commits dann wieder her. Zunächst schauen wir uns den gegenwärtigen Zustand des Repositorys an:

	$ git log --pretty=oneline
	ab1afef80fac8e34258ff41fc1b867c702daa24b modified repo a bit
	484a59275031909e19aadb7c92262719cfcdf19a added repo.rb
	1a410efbd13591db07496601ebc7a059dd55cfe9 third commit
	cac0cab538b970a37ea1e769cbbde608743bc96d second commit
	fdf4fc3344e67ab068f836878b6c4951e3b15f3d first commit

<!--Now, move the `master` branch back to the middle commit:-->

Jetzt setzen wir den Branch `master` auf den mittleren Commit zurück:

	$ git reset --hard 1a410efbd13591db07496601ebc7a059dd55cfe9
	HEAD is now at 1a410ef third commit
	$ git log --pretty=oneline
	1a410efbd13591db07496601ebc7a059dd55cfe9 third commit
	cac0cab538b970a37ea1e769cbbde608743bc96d second commit
	fdf4fc3344e67ab068f836878b6c4951e3b15f3d first commit

<!--You’ve effectively lost the top two commits — you have no branch from which those commits are reachable. You need to find the latest commit SHA and then add a branch that points to it. The trick is finding that latest commit SHA — it’s not like you’ve memorized it, right?-->

Du hast damit die oberen beiden Commits verloren, d.h. es gibt keinen Branch mehr, von dem aus diese Commits erreichbar wären. Um sie wiederherzustellen, kannst Du einen neuen Branch anlegen, der auf den SHA-Hash des obersten (letzten) Commits zeigt. Der Trick besteht darin, diesen letzten Commit-Hash herauszufinden. Es ist ja nicht so, dass Du Dir jederzeit all die Hashes merken könntest, oder?

<!--Often, the quickest way is to use a tool called `git reflog`. As you’re working, Git silently records what your HEAD is every time you change it. Each time you commit or change branches, the reflog is updated. The reflog is also updated by the `git update-ref` command, which is another reason to use it instead of just writing the SHA value to your ref files, as we covered in the "Git References" section of this chapter earlier.  You can see where you’ve been at any time by running `git reflog`:-->

In der Regel ist der schnellste Weg, solche Hashes zu finden, der Befehl `git reflog`. Während Du mit Git arbeitest, macht Git im Stillen fortlaufende Notizen darüber, was HEAD ist. Jedes Mal, wenn Du einen Commit anlegst oder den Branch wechselst, wird das „Reflog“ aktualisiert. Das Reflog wird außerdem vom Befehl `git update-ref` verwendet – ein weiterer guter Grund, nicht stattdessen einfach den SHA-Wert in eine Referenz-Datei zu schreiben (wie wir das in der Sektion „Git-Referenzen“ zuvor in diesem Kapitel besprochen haben). Du kannst also jederzeit nachschlagen, woran Du jeweils gearbeitet hast, indem Du den Befehl `git reflog` verwendest:

	$ git reflog
	1a410ef HEAD@{0}: 1a410efbd13591db07496601ebc7a059dd55cfe9: updating HEAD
	ab1afef HEAD@{1}: ab1afef80fac8e34258ff41fc1b867c702daa24b: updating HEAD

<!--Here we can see the two commits that we have had checked out, however there is not much information here.  To see the same information in a much more useful way, we can run `git log -g`, which will give you a normal log output for your reflog.-->

Das zeigt also die beiden verloren gegangenen Commits an, die wir zuvor ausgecheckt hatten. Allerdings zeigt es auch nicht viel mehr Information. Um das Reflog in einer anderen, etwas nützlicheren Weise anzuzeigen, kannst Du `git log -g` verwenden. Das gibt das Reflog im gewohnten Log-Format aus:

	$ git log -g
	commit 1a410efbd13591db07496601ebc7a059dd55cfe9
	Reflog: HEAD@{0} (Scott Chacon <schacon@gmail.com>)
	Reflog message: updating HEAD
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Fri May 22 18:22:37 2009 -0700

	    third commit

	commit ab1afef80fac8e34258ff41fc1b867c702daa24b
	Reflog: HEAD@{1} (Scott Chacon <schacon@gmail.com>)
	Reflog message: updating HEAD
	Author: Scott Chacon <schacon@gmail.com>
	Date:   Fri May 22 18:15:24 2009 -0700

	     modified repo a bit

<!--It looks like the bottom commit is the one you lost, so you can recover it by creating a new branch at that commit. For example, you can start a branch named `recover-branch` at that commit (ab1afef):-->

Es sieht also so aus, als sei der untere Commit derjenige, den Du verloren hast, aber noch brauchst. Du kannst ihn jetzt wiederherstellen, indem Du einen neuen Branch erstellst, der auf diesen Commit zeigt. Beispielsweise kannst Du einen Branch `recover-branch` anlegen, der auf den Commit `ab1afef` zeigt:

	$ git branch recover-branch ab1afef
	$ git log --pretty=oneline recover-branch
	ab1afef80fac8e34258ff41fc1b867c702daa24b modified repo a bit
	484a59275031909e19aadb7c92262719cfcdf19a added repo.rb
	1a410efbd13591db07496601ebc7a059dd55cfe9 third commit
	cac0cab538b970a37ea1e769cbbde608743bc96d second commit
	fdf4fc3344e67ab068f836878b6c4951e3b15f3d first commit

<!--Cool — now you have a branch named `recover-branch` that is where your `master` branch used to be, making the first two commits reachable again.-->
<!--Next, suppose your loss was for some reason not in the reflog — you can simulate that by removing `recover-branch` and deleting the reflog. Now the first two commits aren’t reachable by anything:-->

Sehr gut. Du hast jetzt einen neuen Branch `recover-branch`, der diejenigen Commits enthält, die sich zuvor in Deinem Branch `master` befanden. Damit hast Du wieder Zugriff auf die beiden verloren gegangenen Commits. Als nächstes nehmen wir aber außerdem an, dass diese verlorenen Commits aus irgendeinem Grunde nicht im Reflog enthalten sind – Du kannst das z.B. simulieren, indem Du den Branch `recover-branch` und das Reflog löschst. Damit sind die beiden Commits jetzt von nirgendwo her mehr erreichbar:

	$ git branch -D recover-branch
	$ rm -Rf .git/logs/

<!--Because the reflog data is kept in the `.git/logs/` directory, you effectively have no reflog. How can you recover that commit at this point? One way is to use the `git fsck` utility, which checks your database for integrity. If you run it with the `-\-full` option, it shows you all objects that aren’t pointed to by another object:-->

Das Reflog wird im Verzeichnis `.git/logs/` aufbewahrt, d.h. Du hast nun faktisch kein Reflog mehr. Wie kann man einen Commit jetzt noch wiederherstellen? Eine Möglichkeit dazu ist der Befehl `git fsck`, der die Integrität der Git-Datenbank prüft. Wenn Du den Befehl mit der Option `--full` ausführst, zeigt er alle Objekte an, auf die nicht von einem anderen Objekt verwiesen wird:

	$ git fsck --full
	dangling blob d670460b4b4aece5915caf5c68d12f560a9fe3e4
	dangling commit ab1afef80fac8e34258ff41fc1b867c702daa24b
	dangling tree aea790b9a58f6cf6f2804eeac9f0abbe9631e4c9
	dangling blob 7108f7ecb345ee9d0084193f147cdad4d2998293

<!--In this case, you can see your missing commit after the dangling commit. You can recover it the same way, by adding a branch that points to that SHA.-->

In diesem Fall findest Du den verlorenen Commit nach den Worten „dangling commit“. Du kannst ihn dann auf dieselbe Weise wiederherstellen wie zuvor, indem Du einen Branch erstellst, der auf diesen Commit-Hash zeigt.

<!--### Removing Objects ###-->
### Objekte entfernen ###

<!--There are a lot of great things about Git, but one feature that can cause issues is the fact that a `git clone` downloads the entire history of the project, including every version of every file. This is fine if the whole thing is source code, because Git is highly optimized to compress that data efficiently. However, if someone at any point in the history of your project added a single huge file, every clone for all time will be forced to download that large file, even if it was removed from the project in the very next commit. Because it’s reachable from the history, it will always be there.-->

Git ist in vielerlei Hinsicht unschlagbar, aber es gibt auch Features, die Probleme verursachen können. Ein solches Problem kann darin bestehen, dass `git clone` die vollständige Historie eines Projektes herunterlädt, d.h. jede einzelne Version jeder einzelnen Datei. Das ist eine feine Sache, solange es sich um Quellcode handelt, denn Git ist darauf optimiert, diese Art von Daten effizient zu komprimieren. Wenn allerdings irgendwann einmal eine einzelne, sehr große Datei zur Versionskontrolle hinzugefügt wurde, wird jeder Clone dieses Repositorys diese Datei gezwungenermaßen herunterladen müssen – auch dann, wenn die Datei inzwischen aus dem Repository entfernt würde. Weil sie über die Historie erreichbar ist, muss die Datei vorhanden sein.

<!--This can be a huge problem when you’re converting Subversion or Perforce repositories into Git. Because you don’t download the whole history in those systems, this type of addition carries few consequences. If you did an import from another system or otherwise find that your repository is much larger than it should be, here is how you can find and remove large objects.-->

Hierin kann ein großes Problem bestehen, wenn Du Subversion- oder Perforce-Repositorys nach Git konvertierst. Weil Du in diesen Systemen nicht die gesamte Historie herunterlädst, kann diese Art von Hinzufügung einige unangenehme Konsequenzen haben. Wenn Du Dein Repository aus einem anderen System importiert hast oder aus irgendeinem anderen Grunde findest, dass es sehr viel größer ist, als es eigentlich sein sollte, kannst Du große Objekte wie folgt suchen und entfernen.

<!--Be warned: this technique is destructive to your commit history. It rewrites every commit object downstream from the earliest tree you have to modify to remove a large file reference. If you do this immediately after an import, before anyone has started to base work on the commit, you’re fine — otherwise, you have to notify all contributors that they must rebase their work onto your new commits.-->

Sei Dir allerdings bewusst, dass diese Technik die Commit-Historie zerstört. Sie schreibt angefangen beim ursprünglichen Tree jedes einzelne Commit-Objekt neu, um die jeweilige, große Datei zu entfernen. Wenn Du das direkt nach einem Import tust, d.h. bevor jemand angefangen hat, auf der Basis eines Commits zu arbeiten, ist das in Ordnung – andernfalls müssen alle Mitarbeiter ihre Arbeit auf Deinen Commit rebasen.

<!--To demonstrate, you’ll add a large file into your test repository, remove it in the next commit, find it, and remove it permanently from the repository. First, add a large object to your history:-->

Um das zu demonstrieren, werden wir eine große Datei zu Deinem Test-Repository hinzufügen, sie dann im nächsten Commit löschen, in der Datenbank nachschlagen und sie schließlich dauerhaft aus dem Repository entfernen. Als erstes checke also eine große Datei in Dein Repository ein:

	$ curl http://kernel.org/pub/software/scm/git/git-1.6.3.1.tar.bz2 > git.tbz2
	$ git add git.tbz2
	$ git commit -am 'added git tarball'
	[master 6df7640] added git tarball
	 1 files changed, 0 insertions(+), 0 deletions(-)
	 create mode 100644 git.tbz2

<!--Oops — you didn’t want to add a huge tarball to your project. Better get rid of it:-->

Oha. Du wolltest keinen dermaßen großen Tarball in Deinem Projekt. Am besten löschen wir es gleich wieder:

	$ git rm git.tbz2
	rm 'git.tbz2'
	$ git commit -m 'oops - removed large tarball'
	[master da3f30d] oops - removed large tarball
	 1 files changed, 0 insertions(+), 0 deletions(-)
	 delete mode 100644 git.tbz2

<!--Now, `gc` your database and see how much space you’re using:-->

Jetzt lassen wir die Garbage-Collection über die Datenbank laufen und sehen, wieviel Platz sie braucht:

	$ git gc
	Counting objects: 21, done.
	Delta compression using 2 threads.
	Compressing objects: 100% (16/16), done.
	Writing objects: 100% (21/21), done.
	Total 21 (delta 3), reused 15 (delta 1)

<!--You can run the `count-objects` command to quickly see how much space you’re using:-->

Du kannst auch den Befehl `count-objects` laufen lassen, um einen schnellen Überblick darüber zu erhalten, wieviel Platz das Repository einnimmt:

	$ git count-objects -v
	count: 4
	size: 16
	in-pack: 21
	packs: 1
	size-pack: 2016
	prune-packable: 0
	garbage: 0

<!--The `size-pack` entry is the size of your packfiles in kilobytes, so you’re using 2MB. Before the last commit, you were using closer to 2K — clearly, removing the file from the previous commit didn’t remove it from your history. Every time anyone clones this repository, they will have to clone all 2MB just to get this tiny project, because you accidentally added a big file. Let’s get rid of it.-->

Der `size-pack`-Eintrag zeigt die Größe der Packfiles in Kilobytes an, d.h. Dein Repository braucht 2 MB. Vor dem letzten Commit lag dieser Wert eher bei 2 KB. D.h., die Datei wurde im letzten Commit eindeutig nicht aus der History entfernt. Jedes Mal, wenn jemand künftig dieses sehr kleine Repository klont, wird er die 2 MB mit herunterladen müssen – nur weil wir versehentlich diese große Datei hinzugefügt hatten. Also versuchen wir, sie endgültig loszuwerden.

<!--First you have to find it. In this case, you already know what file it is. But suppose you didn’t; how would you identify what file or files were taking up so much space? If you run `git gc`, all the objects are in a packfile; you can identify the big objects by running another plumbing command called `git verify-pack` and sorting on the third field in the output, which is file size. You can also pipe it through the `tail` command because you’re only interested in the last few largest files:-->

Zunächst mal müssen wir sie finden. In diesem Fall wissen wir bereits, um welche Datei es sich handelt. Aber nehmen wir an, wir wüssten es nicht. Wie würdest Du herausfinden, welche Datei (oder welche Dateien) so viel Platz verbrauchen? Wenn Du `git gc` laufen gelassen hast, werden sich alle Objekte in einem Packfile befinden. Du kannst dann große Objekte identifizieren, indem Du einen weiteren Plumbing-Befehl, nämlich `git verify-pack` ausführst und nach dem dritten Feld der Ausgabe sortierst, d.h. der Dateigröße. Du kannst sie außerdem durch den `tail` Befehl leiten, denn Du bist ja nur an den wenigen größten Objekten interessiert:

	$ git verify-pack -v .git/objects/pack/pack-3f8c0...bb.idx | sort -k 3 -n | tail -3
	e3f094f522629ae358806b17daf78246c27c007b blob   1486 734 4667
	05408d195263d853f09dca71d55116663690c27c blob   12908 3478 1189
	7a9eb2fba2b1811321254ac360970fc169ba2330 blob   2056716 2056872 5401

<!--The big object is at the bottom: 2MB. To find out what file it is, you’ll use the `rev-list` command, which you used briefly in Chapter 7. If you pass `-\-objects` to `rev-list`, it lists all the commit SHAs and also the blob SHAs with the file paths associated with them. You can use this to find your blob’s name:-->

Das größte Objekt ist ganz klar das letzte: es ist 2 MB groß. Um herauszufinden, welche Datei das ist, kannst Du den `rev-list` Befehl verwenden, den wir in Kapitel 7 schon einmal kurz verwendet haben. Wenn Du die Optionen `--objects` und `rev-list` verwendest, werden alle Commit- und Blob-SHAs mit den jeweiligen Dateipfaden aufgelistet, die mit ihnen assoziiert sind. Auf diese Weise kannst Du den Namen des Blobs finden:

	$ git rev-list --objects --all | grep 7a9eb2fb
	7a9eb2fba2b1811321254ac360970fc169ba2330 git.tbz2

<!--Now, you need to remove this file from all trees in your past. You can easily see what commits modified this file:-->

Du musst diese Datei jetzt aus allen Trees entfernen, in denen sie sich befindet. Du kannst leicht herausfinden, welche Commits diese Datei verändert haben:

	$ git log --pretty=oneline --branches -- git.tbz2
	da3f30d019005479c99eb4c3406225613985a1db oops - removed large tarball
	6df764092f3e7c8f5f94cbe08ee5cf42e92a0289 added git tarball

<!--You must rewrite all the commits downstream from `6df76` to fully remove this file from your Git history. To do so, you use `filter-branch`, which you used in Chapter 6:-->

Es geht also darum, alle Commits angefangen bei `6df76` neu zu schreiben, sodass der Tarball nicht mehr in Deiner Git-Historie enthalten ist. Um das zu erreichen, verwendest Du den Befehl `git filter-branch`, den wir schon mal in Kapitel 6 verwendet haben:

	$ git filter-branch --index-filter \
	   'git rm --cached --ignore-unmatch git.tbz2' -- 6df7640^..
	Rewrite 6df764092f3e7c8f5f94cbe08ee5cf42e92a0289 (1/2)rm 'git.tbz2'
	Rewrite da3f30d019005479c99eb4c3406225613985a1db (2/2)
	Ref 'refs/heads/master' was rewritten

<!--The `-\-index-filter` option is similar to the `-\-tree-filter` option used in Chapter 6, except that instead of passing a command that modifies files checked out on disk, you’re modifying your staging area or index each time. Rather than remove a specific file with something like `rm file`, you have to remove it with `git rm -\-cached` — you must remove it from the index, not from disk. The reason to do it this way is speed — because Git doesn’t have to check out each revision to disk before running your filter, the process can be much, much faster. You can accomplish the same task with `-\-tree-filter` if you want. The `-\-ignore-unmatch` option to `git rm` tells it not to error out if the pattern you’re trying to remove isn’t there. Finally, you ask `filter-branch` to rewrite your history only from the `6df7640` commit up, because you know that is where this problem started. Otherwise, it will start from the beginning and will unnecessarily take longer.-->

Die Option  `--index-filter` ist ähnlich der Option `--tree-filter` aus Kapitel 6. Allerdings übergibt man in diesem Fall nicht einen Befehl, der die Dateien verändert, die sich jeweils ausgecheckt auf der Festplatte befinden. Stattdessen verändert man jeweils die Staging-Area bzw. den Index. Statt eine bestimmte Datei mit z.B: `rm file` zu entfernen, müssen wir also `git rm --cached` verwenden – denn wir wollen sie aus dem Index, nicht von der Festplatte löschen. Der Grund dafür ist einfach Geschwindigkeit: Git braucht nicht jede einzelne Revision auf die Festplatte auszuchecken, um den Filter anzuwenden. Auf diese Weise läuft der ganze Prozess sehr viel schneller. Man kann dieselbe Aufgabe aber auch mit `--tree-filter` erledigen, wenn man will. Die Option `--ignore-match` weist Git an, nicht mit einer Fehlermeldung abzubrechen, wenn die Datei, die wir löschen wollen, nicht vorhanden ist. Außerdem teilen wir `filter-branch` mit, die Historie nur von dem Commit `6df7640` an umzuschreiben. Andernfalls würde der Befehl von ganz vorn beginnen und unnötig länger brauchen.

<!--Your history no longer contains a reference to that file. However, your reflog and a new set of refs that Git added when you did the `filter-branch` under `.git/refs/original` still do, so you have to remove them and then repack the database. You need to get rid of anything that has a pointer to those old commits before you repack:-->

Deine Historie enthält jetzt nicht länger eine Referenz auf diese Datei. Dein Reflog und ein neues Set an Referenzen, die Git unter `.git/refs/original` angelegt hat, als Du `filter-branch` ausgeführt hast, tun das allerdings immer noch – also entfernen wir sie von dort und packen das Repository erneut. Bevor Du packst, musst Du zuerst alles entfernen, was noch Referenzen auf das alte Objekt enthält:

	$ rm -Rf .git/refs/original
	$ rm -Rf .git/logs/
	$ git gc
	Counting objects: 19, done.
	Delta compression using 2 threads.
	Compressing objects: 100% (14/14), done.
	Writing objects: 100% (19/19), done.
	Total 19 (delta 3), reused 16 (delta 1)

<!--Let’s see how much space you saved.-->

Prüfen wir also, wieviel Platz wir damit eingespart haben:

	$ git count-objects -v
	count: 8
	size: 2040
	in-pack: 19
	packs: 1
	size-pack: 7
	prune-packable: 0
	garbage: 0

<!--The packed repository size is down to 7K, which is much better than 2MB. You can see from the size value that the big object is still in your loose objects, so it’s not gone; but it won’t be transferred on a push or subsequent clone, which is what is important. If you really wanted to, you could remove the object completely by running `git prune -\-expire`.-->

Das gepackte Repository umfasst jetzt nur noch 7K – sehr viel besser als die vorherigen 2MB. Du kannst an dem Wert `size` erkennen, dass sich die große Datei selbst jetzt immer noch in Deinen losen Objekten befindet. Aber sie wird bei einem `git push` oder anschließenden `git clone` nicht übermittelt werden – und das war unser Ziel. Wenn Du das wirklich willst, kannst Du sie jetzt vollständig und endgültig mit `git prune --expire` aus Deinem Repository löschen.

<!--## Summary ##-->
## Zusammenfassung ##

<!--You should have a pretty good understanding of what Git does in the background and, to some degree, how it’s implemented. This chapter has covered a number of plumbing commands — commands that are lower level and simpler than the porcelain commands you’ve learned about in the rest of the book. Understanding how Git works at a lower level should make it easier to understand why it’s doing what it’s doing and also to write your own tools and helping scripts to make your specific workflow work for you.-->

Du solltest jetzt ein gutes Verständnis davon haben, was Git im Hintergrund tut, und in einem gewissen Maße auch davon, wie es implementiert ist. In diesem Kapitel haben wir eine Reihe von Plumbing-Befehlen besprochen, also Befehlen, die grundlegender und einfacher als die Porcelain-Befehle sind, um die es im restlichen Buch ging. Dieses Verständnis sollte Dir helfen, zu verstehen, warum Git tut, was es tut – und natürlich auch dabei, Deine eigenen Werkzeuge und Hilfsskripte zu schreiben, um einen bestimmten Arbeitsablauf für Dich anzupassen.

<!--Git as a content-addressable filesystem is a very powerful tool that you can easily use as more than just a VCS. I hope you can use your newfound knowledge of Git internals to implement your own cool application of this technology and feel more comfortable using Git in more advanced ways.-->

Git als ein Dateisystem, das Inhalte addressieren kann, ist ein äußerst mächtiges Werkzeug, das Du leicht für mehr als „nur“ als VCS einsetzen kannst. Ich hoffe, Du kannst Dein neugewonnenes Wissen der Git Interna nutzen, um Deine eigene tolle Anwendung dieser Technologie zu implementieren und Dich wohler damit zu fühlen, Git auch in fortgeschrittener Weise zu benutzen.
