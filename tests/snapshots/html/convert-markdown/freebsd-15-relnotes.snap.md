[Donate to FreeBSD](https://www.FreeBSDFoundation.org/donate/)

- [Get FreeBSD](https://www.freebsd.org/where)
- [Release Information](https://www.freebsd.org/releases)
  
  - Production Release:\
    [15.0](https://www.FreeBSD.org/releases/15.0R/announce/)
  - Production Release:\
    [14.3](https://www.FreeBSD.org/releases/14.3R/announce/)
  - Legacy Release:\
    [13.5](https://www.FreeBSD.org/releases/13.5R/announce/)
- [Snapshot Releases](https://www.freebsd.org/snapshots)
  
  - Upcoming Release:\
    [14.4](https://www.FreeBSD.org/releases/14.4R/schedule/)
- [Ported Applications](https://www.freebsd.org/ports)

# FreeBSD 15.0\-RELEASE Release Notes

## Abstract

The release notes for FreeBSD 15.0\-RELEASE contain a summary of the changes
made to the FreeBSD base system on the 15\-STABLE development line. This
document lists applicable security advisories that were issued since the last
release, as well as significant changes to the FreeBSD kernel and userland. Some
brief remarks on upgrading are also presented.

Table of Contents

- [Abstract](#_abstract)
- [Introduction](#intro)
- [Upgrading from Previous Releases of FreeBSD](#upgrade)
- [Upgrading from Existing Pre\-Release Base System Package
  Installs](#upgrade-rc)
- [Included Security Fixes and Errata Patches](#security-errata)
  
  - [Fixed Security Advisories](#security)
  - [Patched Errata Notices](#errata)
- [Architectures](#architectures)
- [Userland](#userland)
  
  - [Userland Configuration Changes](#userland-config)
  - [Userland Application Changes](#userland-programs)
  - [Contributed Software](#userland-contrib)
  - [Runtime Libraries and API](#userland-libraries)
  - [Miscellaneous](#userland-misc)
  - [Deprecated Applications](#userland-deprecated-programs)
- [Cloud Support](#cloud)
- [Kernel](#kernel)
  
  - [General Kernel Changes](#kernel-general)
  - [Architecture\-Specific Changes](#kernel-architecture-specific)
- [Devices and Drivers](#drivers)
  
  - [Device Drivers](#drivers-device)
  - [Deprecated and Removed Drivers](#drivers-removals)
- [Storage](#storage)
  
  - [NFS](#storage-nfs)
  - [UFS](#storage-ufs)
  - [ZFS](#storage-zfs)
  - [GEOM](#storage-geom)
  - [General Storage](#storage-general)
- [Boot Loader Changes](#boot-loader)
- [Networking](#network)
  
  - [General Network](#network-general)
  - [Network Protocols](#network-protocols)
  - [Wireless Networking](#wireless-networking)
- [Hardware Support](#hardware)
  
  - [Virtualization Support](#hardware-virtualization)
  - [Linux Binary Compatibility](#linuxulator)
- [Multimedia](#multimedia)
- [Documentation](#documentation)
  
  - [Manual Pages](#man-pages)
- [Ports Collection and Package Infrastructure](#ports)
  
  - [Installer](#Installer)
  - [Packaging Changes](#ports-packages)
- [General Notes Regarding Future FreeBSD Releases](#future-releases)

## Introduction

This document contains the release notes for FreeBSD 15.0\-RELEASE. It describes
recently added, changed, or deleted features of FreeBSD. It also provides some
notes on upgrading from previous versions of FreeBSD.

The "release" distribution to which these release notes apply represents the
latest point along the 15\-STABLE development branch between 14.0\-RELEASE and
the future 15.1\-RELEASE. Information regarding pre\-built, binary "release"
distributions along this branch can be found at
<https://www.FreeBSD.org/releases/>. More information on obtaining this \(or
other\) "release" distributions of FreeBSD can be found in the [Obtaining
FreeBSD appendix](https://docs.freebsd.org/en/books/handbook//mirrors) to the
[FreeBSD Handbook](https://docs.freebsd.org/en/books/handbook//).

All users are encouraged to consult the release errata before installing
FreeBSD. The errata document is updated with "late\-breaking" information
discovered late in the release cycle or after the release. Typically, it
contains information on known bugs, security advisories, and corrections to
documentation. An up\-to\-date copy of the errata for FreeBSD 15.0\-RELEASE can
be found on the FreeBSD Web site.

This document describes the most user\-visible new or changed features in
FreeBSD since 14.0\-RELEASE. In general, changes described here are unique to
the 15\-STABLE branch unless specifically marked as MERGED features.

Typical release note items document recent security advisories issued after
14.0\-RELEASE, new drivers or hardware support, new commands or options, major
bug fixes, or contributed software upgrades. They may also list changes to major
ports/packages or release engineering practices. Clearly the release notes
cannot list every single change made to FreeBSD between releases; this document
focuses primarily on security advisories, user\-visible changes, and major
architectural improvements.

## Upgrading from Previous Releases of FreeBSD

Binary upgrades between RELEASE versions \(and snapshots of the various security
branches\) are supported using the
[freebsd\-update\(8\)](https://man.freebsd.org/cgi/man.cgi?query=freebsd-update&sektion=8&format=html)
utility. See the release\-specific upgrade procedure, [FreeBSD 15.0\-RELEASE
upgrade information](../installation/#upgrade-binary), with more details in the
FreeBSD handbook [binary upgrade
procedure](https://docs.freebsd.org/en/books/handbook/cutting-edge/#freebsdupdate-upgrade).
This will update unmodified userland utilities, as well as unmodified GENERIC
kernels distributed as a part of an official FreeBSD release. The
[freebsd\-update\(8\)](https://man.freebsd.org/cgi/man.cgi?query=freebsd-update&sektion=8&format=html)
utility requires that the host being upgraded have Internet connectivity.

Source\-based upgrades \(those based on recompiling the FreeBSD base system from
source code\) from previous versions are supported, according to the
instructions in /usr/src/UPDATING.

Upgrading FreeBSD should only be attempted after backing up _all_ data and
configuration files.

## Upgrading from Existing Pre\-Release Base System Package Installs

For users of PRERELEASE, ALPHA, and BETA builds of FreeBSD 15.0, due to
late\-breaking changes in FreeBSD.org infrastructure, it is not possible to
upgrade directly using the
[pkg\-upgrade\(8\)](https://man.freebsd.org/cgi/man.cgi?query=pkg-upgrade&sektion=8&format=html)
utility.

Users should either manually copy the required files from a source tree checkout
of `15.0-RELEASE` tag, or a later commit in `STABLE` or `CURRENT` branches, or
alternatively, force\-install the `FreeBSD-pkg-bootstrap` package from the
official release base system packages.

The recommended, and most secure approach, is using the source tree checkout of
any of head, stable/15, or releng/15.0 branches after 2025\-11\-27 22:00 UTC.

```
# cp /usr/src/usr.sbin/pkg/FreeBSD.conf.quarterly-release \
       /etc/pkg/FreeBSD.conf
# cp -R /usr/src/share/keys/pkgbase-15 /usr/share/keys/pkgbase-15
```


Users who do not have up to date sources installed may use a less secure, but
simpler approach, validating the checksums after installation. As these are
architecture\-independent files, the checksums will match on all platforms.

```
# pkg add -f https://pkg.freebsd.org/FreeBSD:15:$(uname -p)/base_release_0/FreeBSD-pkg-bootstrap-15.0.pkg
# sha256 -r /etc/pkg/FreeBSD.conf /usr/share/keys/pkg/trusted/pkg.freebsd.org.2013102301 \
  /usr/share/keys/pkgbase-15/trusted/awskms-15 /usr/share/keys/pkgbase-15/trusted/backup-signing-15
ab261a3b84ffc11654ac0bafbb7d6b3f1b6afc30bfabab3bcff64259678eac26 /etc/pkg/FreeBSD.conf
036ae4f9c441a3febb41734bbb37227ec3374edd3c6c687e5cb70d580efbea30 /usr/share/keys/pkg/trusted/pkg.freebsd.org.2013102301
529c79e85a6ca152faa9d57ead85fe0111ffada8d0a0fa2f11fc510999fa50df /usr/share/keys/pkgbase-15/trusted/awskms-15
c368ec8d05654bdaad34742c1d75b9b150bfc3892838cef32f6e5b036b0c0605 /usr/share/keys/pkgbase-15/trusted/backup-signing-15
```


Upgrading FreeBSD should only be attempted after backing up _all_ data and
configuration files.

## Included Security Fixes and Errata Patches

This section lists the various Security Advisories and Errata Notices since
14.0\-RELEASE that have been addressed in 15.0\-RELEASE.

### Fixed Security Advisories

Advisory Date Topic

[FreeBSD\-SA\-23:17.pf](https://www.freebsd.org/security/advisories/FreeBSD-SA-23:17.pf.asc)

05 December 2023

TCP spoofing vulnerability in
[pf\(4\)](https://man.freebsd.org/cgi/man.cgi?query=pf&sektion=4&format=html)

[FreeBSD\-SA\-23:18.nfsclient](https://www.freebsd.org/security/advisories/FreeBSD-SA-23:18.nfsclient.asc)

12 December 2023

NFS client data corruption and kernel memory disclosure

[FreeBSD\-SA\-23:19.openssh](https://www.freebsd.org/security/advisories/FreeBSD-SA-23:19.openssh.asc)

19 December 2023

Prefix Truncation Attack in the SSH protocol

[FreeBSD\-SA\-24:01.bhyveload](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:01.bhyveload.asc)

14 February 2024

[bhyveload\(8\)](https://man.freebsd.org/cgi/man.cgi?query=bhyveload&sektion=8&format=html)
host file access

[FreeBSD\-SA\-24:02.tty](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:02.tty.asc)

14 February 2024

[jail\(2\)](https://man.freebsd.org/cgi/man.cgi?query=jail&sektion=2&format=html)
information leak

[FreeBSD\-SA\-24:03.unbound](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:03.unbound.asc)

28 March 2024

Multiple vulnerabilities in unbound

[FreeBSD\-SA\-24:04.openssh](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:04.openssh.asc)

01 July 2024

OpenSSH pre\-authentication remote code execution

[FreeBSD\-SA\-24:05.pf](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:05.pf.asc)

07 August 2024

pf incorrectly matches different ICMPv6 states in the state table

[FreeBSD\-SA\-24:06.ktrace](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:06.ktrace.asc)

07 August 2024

[ktrace\(2\)](https://man.freebsd.org/cgi/man.cgi?query=ktrace&sektion=2&format=html)
fails to detach when executing a setuid binary

[FreeBSD\-SA\-24:07.nfsclient](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:07.nfsclient.asc)

07 August 2024

NFS client accepts file names containing path separators

[FreeBSD\-SA\-24:08.openssh](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:08.openssh.asc)

07 August 2024

OpenSSH pre\-authentication async signal safety issue

[FreeBSD\-SA\-24:09.libnv](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:09.libnv.asc)

04 September 2024

Multiple vulnerabilities in libnv

[FreeBSD\-SA\-24:10.bhyve](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:10.bhyve.asc)

04 September 2024

[bhyve\(8\)](https://man.freebsd.org/cgi/man.cgi?query=bhyve&sektion=8&format=html)
privileged guest escape via TPM device passthrough

[FreeBSD\-SA\-24:11.ctl](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:11.ctl.asc)

04 September 2024

Multiple issues in
[ctl\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ctl&sektion=4&format=html)
CAM Target Layer

[FreeBSD\-SA\-24:12.bhyve](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:12.bhyve.asc)

04 September 2024

[bhyve\(8\)](https://man.freebsd.org/cgi/man.cgi?query=bhyve&sektion=8&format=html)
privileged guest escape via USB controller

[FreeBSD\-SA\-24:13.openssl](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:13.openssl.asc)

04 September 2024

Possible DoS in X.509 name checks in OpenSSL

[FreeBSD\-SA\-24:14.umtx](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:14.umtx.asc)

04 September 2024

umtx Kernel panic or Use\-After\-Free

[FreeBSD\-SA\-24:15.bhyve](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:15.bhyve.asc)

19 September 2024

[bhyve\(8\)](https://man.freebsd.org/cgi/man.cgi?query=bhyve&sektion=8&format=html)
out\-of\-bounds read access via XHCI emulation

[FreeBSD\-SA\-24:16.libnv](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:16.libnv.asc)

19 September 2024

Integer overflow in libnv

[FreeBSD\-SA\-24:17.bhyve](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:17.bhyve.asc)

29 October 2024

Multiple issues in the bhyve hypervisor

[FreeBSD\-SA\-24:18.ctl](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:18.ctl.asc)

29 October 2024

Unbounded allocation in
[ctl\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ctl&sektion=4&format=html)
CAM Target Layer

[FreeBSD\-SA\-24:19.fetch](https://www.freebsd.org/security/advisories/FreeBSD-SA-24:19.fetch.asc)

29 October 2024

Certificate revocation list
[fetch\(1\)](https://man.freebsd.org/cgi/man.cgi?query=fetch&sektion=1&format=html)
option fails

[FreeBSD\-SA\-25:01.openssh](https://www.freebsd.org/security/advisories/FreeBSD-SA-25:01.openssh.asc)

29 January 2025

OpenSSH Keystroke Obfuscation Bypass

[FreeBSD\-SA\-25:02.fs](https://www.freebsd.org/security/advisories/FreeBSD-SA-25:02.fs.asc)

29 January 2025

Buffer overflow in some filesystems via NFS

[FreeBSD\-SA\-25:03.etcupdate](https://www.freebsd.org/security/advisories/FreeBSD-SA-25:03.etcupdate.asc)

29 January 2025

Unprivileged access to system files

[FreeBSD\-SA\-25:04.ktrace](https://www.freebsd.org/security/advisories/FreeBSD-SA-25:04.ktrace.asc)

29 January 2025

Uninitialized kernel memory disclosure via
[ktrace\(2\)](https://man.freebsd.org/cgi/man.cgi?query=ktrace&sektion=2&format=html)

[FreeBSD\-SA\-25:05.openssh](https://www.freebsd.org/security/advisories/FreeBSD-SA-25:05.openssh.asc)

21 February 2025

Multiple vulnerabilities in OpenSSH

[FreeBSD\-SA\-25:06.xz](https://www.freebsd.org/security/advisories/FreeBSD-SA-25:06.xz.asc)

02 July 2025

Use\-after\-free in multi\-threaded xz decoder

[FreeBSD\-SA\-25:07.libarchive](https://www.freebsd.org/security/advisories/FreeBSD-SA-25:07.libarchive.asc)

08 August 2025

Integer overflow in libarchive leading to double free

[FreeBSD\-SA\-25:08.openssl](https://www.freebsd.org/security/advisories/FreeBSD-SA-25:08.openssl.asc)

30 September 2025

Multiple vulnerabilities in OpenSSL

[FreeBSD\-SA\-25:09.netinet](https://www.freebsd.org/security/advisories/FreeBSD-SA-25:09.netinet.asc)

22 October 2025

`SO_REUSEPORT_LB` breaks
[connect\(2\)](https://man.freebsd.org/cgi/man.cgi?query=connect&sektion=2&format=html)
for UDP sockets

[FreeBSD\-SA\-25:10.unbound](https://www.freebsd.org/security/advisories/FreeBSD-SA-25:10.unbound.asc)

26 November 2025

Cache poison in local\-unbound service

### Patched Errata Notices

Errata Date Topic

[FreeBSD\-EN\-23:15:sanitizer](https://www.freebsd.org/security/advisories/FreeBSD-EN-23:15.sanitizer.asc)

01 December 2023

Clang sanitizer failure with ASLR enabled

[FreeBSD\-EN\-23:16:openzfs](https://www.freebsd.org/security/advisories/FreeBSD-EN-23:16.openzfs.asc)

01 December 2023

OpenZFS data corruption

[FreeBSD\-EN\-23:17:ossl](https://www.freebsd.org/security/advisories/FreeBSD-EN-23:17.ossl.asc)

05 December 2023

[ossl\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ossl&sektion=4&format=html)'s
AES\-GCM implementation may give incorrect results

[FreeBSD\-EN\-23:18:openzfs](https://www.freebsd.org/security/advisories/FreeBSD-EN-23:18.openzfs.asc)

05 December 2023

High CPU usage by ZFS kernel threads

[FreeBSD\-EN\-23:19:pkgbase](https://www.freebsd.org/security/advisories/FreeBSD-EN-23:19.pkgbase.asc)

05 December 2023

Incorrect pkgbase version number for FreeBSD 14.0\-RELEASE.

[FreeBSD\-EN\-23:20:vm](https://www.freebsd.org/security/advisories/FreeBSD-EN-23:20.vm.asc)

05 December 2023

Incorrect results from the kernel physical memory allocator

[FreeBSD\-EN\-23:21:tty](https://www.freebsd.org/security/advisories/FreeBSD-EN-23:21.tty.asc)

24 November 2023

[tty\(4\)](https://man.freebsd.org/cgi/man.cgi?query=tty&sektion=4&format=html)
IUTF8 causes a kernel panic

[FreeBSD\-EN\-23:22:vfs](https://www.freebsd.org/security/advisories/FreeBSD-EN-23:22.vfs.asc)

05 December 2023

ZFS snapshot directories not accessible over NFS

[FreeBSD\-EN\-24:01:tzdata](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:01.tzdata.asc)

14 February 2024

Timezone database information update

[FreeBSD\-EN\-24:02:libutil](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:02.libutil.asc)

14 February 2024

Login class resource limits and CPU mask bypass

[FreeBSD\-EN\-24:03:kqueue](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:03.kqueue.asc)

14 February 2024

[kqueue\_close\(2\)](https://man.freebsd.org/cgi/man.cgi?query=kqueue_close&sektion=2&format=html)
page fault on exit using
[rfork\(2\)](https://man.freebsd.org/cgi/man.cgi?query=rfork&sektion=2&format=html)

[FreeBSD\-EN\-24:04:ip](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:04.ip.asc)

14 February 2024

Kernel panic triggered by
[bind\(2\)](https://man.freebsd.org/cgi/man.cgi?query=bind&sektion=2&format=html)

[FreeBSD\-EN\-24:05:tty](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:05.tty.asc)

28 March 2024

TTY Kernel Panic

[FreeBSD\-EN\-24:06:wireguard](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:06.wireguard.asc)

28 March 2024

Insufficient barriers in WireGuard
[if\_wg\(4\)](https://man.freebsd.org/cgi/man.cgi?query=if_wg&sektion=4&format=html)

[FreeBSD\-EN\-24:07:clang](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:07.clang.asc)

28 March 2024

Clang crash when certain optimization is enabled

[FreeBSD\-EN\-24:08:kerberos](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:08.kerberos.asc)

28 March 2024

Kerberos segfaults when using weak crypto

[FreeBSD\-EN\-24:09:zfs](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:09.zfs.asc)

24 April 2024

High CPU usage by kernel threads related to ZFS

[FreeBSD\-EN\-24:10:zfs](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:10.zfs.asc)

19 June 2024

Kernel memory leak in ZFS

[FreeBSD\-EN\-24:11:ldns](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:11.ldns.asc)

19 June 2024

LDNS uses nameserver commented out in resolv.conf

[FreeBSD\-EN\-24:12:killpg](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:12.killpg.asc)

19 June 2024

Lock order reversal in killpg causing livelock

[FreeBSD\-EN\-24:13:libc\+\+](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:13.libc%2B%2B.asc)

19 June 2024

Incorrect size passed to heap allocated std::string delete

[FreeBSD\-EN\-24:14:ifconfig](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:14.ifconfig.asc)

07 August 2024

Incorrect ifconfig netmask assignment

[FreeBSD\-EN\-24:15:calendar](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:15.calendar.asc)

04 September 2024

[cron\(8\)](https://man.freebsd.org/cgi/man.cgi?query=cron&sektion=8&format=html)
/
[periodic\(8\)](https://man.freebsd.org/cgi/man.cgi?query=periodic&sektion=8&format=html)
session login

[FreeBSD\-EN\-24:16:pf](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:16.pf.asc)

19 September 2024

Incorrect ICMPv6 state handling in pf

[FreeBSD\-EN\-24:17:pam\_xdg](https://www.freebsd.org/security/advisories/FreeBSD-EN-24:17.pam_xdg.asc)

20 October 2024

XDG runtime directory’s file descriptor leak at login

[FreeBSD\-EN\-25:01.rpc](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:01.rpc.asc)

29 January 2025

NULL pointer dereference in the NFSv4 client

[FreeBSD\-EN\-25:02.audit](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:02.audit.asc)

29 January 2025

System call auditing disabled by DTrace

[FreeBSD\-EN\-25:03.tzdata](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:03.tzdata.asc)

29 January 2025

Timezone database information update

[FreeBSD\-EN\-25:04.tzdata](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:04.tzdata.asc)

10 April 2025

Timezone database information update

[FreeBSD\-EN\-25:05.expat](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:05.expat.asc)

10 April 2025

Update expat to 2.7.1

[FreeBSD\-EN\-25:06.daemon](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:06.daemon.asc)

10 April 2025

[daemon\(8\)](https://man.freebsd.org/cgi/man.cgi?query=daemon&sektion=8&format=html)
missing signals

[FreeBSD\-EN\-25:07.openssl](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:07.openssl.asc)

10 April 2025

Update OpenSSL to 3.0.16

[FreeBSD\-EN\-25:08.caroot](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:08.caroot.asc)

10 April 2025

Root certificate bundle update

[FreeBSD\-EN\-25:09:libc](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:09.libc.asc)

02 July 2025

Dynamically\-loaded C\+\+ libraries crashing at exit

[FreeBSD\-EN\-25:10:zfs](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:10.zfs.asc)

02 July 2025

Corruption in ZFS replication streams from encrypted datasets

[FreeBSD\-EN\-25:11:ena](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:11.ena.asc)

02 July 2025

`ena` resets and kernel panic on Nitro v4 or newer instances

[FreeBSD\-EN\-25:12:efi](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:12.efi.asc)

08 August 2025

[bsdinstall\(8\)](https://man.freebsd.org/cgi/man.cgi?query=bsdinstall&sektion=8&format=html)
not copying the correct loader on systems with IA32 UEFI firmware.

[FreeBSD\-EN\-25:13:wlan\_tkip](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:13.wlan_tkip.asc)

08 August 2025

net80211 TKIP crypto support fails for some drivers

[FreeBSD\-EN\-25:14:route](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:14.route.asc)

08 August 2025

[route\(8\)](https://man.freebsd.org/cgi/man.cgi?query=route&sektion=8&format=html)
monitor buffers too much when redirected to a file

[FreeBSD\-EN\-25:15:arm64](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:15.arm64.asc)

16 September 2025

arm64
[syscall\(2\)](https://man.freebsd.org/cgi/man.cgi?query=syscall&sektion=2&format=html)
allows unprivileged user to panic kernel

[FreeBSD\-EN\-25:16:vfs](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:16.vfs.asc)

16 September 2025

[copy\_file\_range\(2\)](https://man.freebsd.org/cgi/man.cgi?query=copy_file_range&sektion=2&format=html)
fails to set output parameters

[FreeBSD\-EN\-25:17:bnxt](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:17.bnxt.asc)

16 September 2025

[bnxt\(4\)](https://man.freebsd.org/cgi/man.cgi?query=bnxt&sektion=4&format=html)
fails to set media type in some cases

[FreeBSD\-EN\-25:18:freebsd\-update](https://www.freebsd.org/security/advisories/FreeBSD-EN-25:18.freebsd-update.asc)

30 September 2025

[freebsd\-update\(8\)](https://man.freebsd.org/cgi/man.cgi?query=freebsd-update&sektion=8&format=html)
installs libraries in incorrect order

## Architectures

The venerable 32\-bit hardware platforms i386, armv6, and 32\-bit powerpc have
been retired. 32\-bit application support lives on via the 32\-bit compatibility
mode in their respective 64\-bit platforms. The armv7 platform remains as the
last supported 32\-bit platform. We thank them for their service.

## Userland

This section covers changes and additions to userland applications, contributed
software, and system utilities.

### Userland Configuration Changes

The Kerberos v5 Authentication Service,
[krb5kdc\(8\)](https://man.freebsd.org/cgi/man.cgi?query=krb5kdc&sektion=8&format=html),
has gained a new `kdc_restart` variable under
[daemon\(8\)](https://man.freebsd.org/cgi/man.cgi?query=daemon&sektion=8&format=html).
Set `kdc_restart="YES"` in
[rc.conf\(5\)](https://man.freebsd.org/cgi/man.cgi?query=rc.conf&sektion=5&format=html)
to auto restart kdc on abnormal termination. Set `kdc_restart_delay="N"` to the
number of seconds to delay before restarting the kdc.
[abc4b3088941](https://cgit.freebsd.org/src/commit/?id=abc4b3088941)

The `daily`
[periodic\(8\)](https://man.freebsd.org/cgi/man.cgi?query=periodic&sektion=8&format=html)
scripts now show less context in emails by default to reduce output size. The
behavior can be controlled by the `daily_diff_flags` variable in
[periodic.conf\(5\)](https://man.freebsd.org/cgi/man.cgi?query=periodic.conf&sektion=5&format=html).
Similarly, the changes shown by the security scripts show less context than
previously, controlled by the `security_status_diff_flags` variable in
[periodic.conf\(5\)](https://man.freebsd.org/cgi/man.cgi?query=periodic.conf&sektion=5&format=html).
[538994626b9f](https://cgit.freebsd.org/src/commit/?id=538994626b9f),
[37dc394170a5](https://cgit.freebsd.org/src/commit/?id=37dc394170a5),
[128e78ffb084](https://cgit.freebsd.org/src/commit/?id=128e78ffb084)

The
[bsnmpd\(1\)](https://man.freebsd.org/cgi/man.cgi?query=bsnmpd&sektion=1&format=html)
daemon no longer supports legacy UDP transport. Users, that have not updated
their `/etc/snmpd.config` since 12.0\-RELEASE or older will need to merge in the
new configuration. In particular, the transport definition shall be changed from
`begemotSnmpdPortStatus` OID to `begemotSnmpdTransInetStatus`.
[9ba51cce8bbd](https://cgit.freebsd.org/src/commit/?id=9ba51cce8bbd)

The `FreeBSD-base` repository is now defined in `/etc/pkg/FreeBSD.conf`,
disabled by default. Systems which installed with pkgbase prior to 15.0\-RC1
\(if running `releng/15.0`\) or November 15th \(if running from `stable`/`main`
snapshots\) will need to remove the definition of the `FreeBSD-base` repository
from `/usr/local/etc/pkg/repos/` and replace it with a single line
`FreeBSD-base: { enabled: yes }`.
[5d832135a971](https://cgit.freebsd.org/src/commit/?id=5d832135a971)

The
[powerd\(8\)](https://man.freebsd.org/cgi/man.cgi?query=powerd&sektion=8&format=html)
utility is now enabled in `/etc/rc.conf` by default on images for the arm64
Raspberry Pi’s \(`arm64-aarch64-RPI` files\). This prevents the CPU clock from
running slow all the time.
[4347ef60501f](https://cgit.freebsd.org/src/commit/?id=4347ef60501f)

### Userland Application Changes

The
[adduser\(8\)](https://man.freebsd.org/cgi/man.cgi?query=adduser&sektion=8&format=html)
utility, used by
[bsdinstall\(8\)](https://man.freebsd.org/cgi/man.cgi?query=bsdinstall&sektion=8&format=html),
will now create a ZFS dataset for a new user’s home directory if the parent
directory resides on a ZFS dataset. A command\-line option is available to
disable use of a separate dataset. ZFS encryption is also available.
[516009ce8d38](https://cgit.freebsd.org/src/commit/?id=516009ce8d38)

The
[date\(1\)](https://man.freebsd.org/cgi/man.cgi?query=date&sektion=1&format=html)
program now supports nanoseconds. For example: `date -Ins` prints
"2024\-04\-22T12:20:28,763742224\+02:00" and `date +%N` prints "415050400".
[eeb04a736cb9](https://cgit.freebsd.org/src/commit/?id=eeb04a736cb9) \(Sponsored
by Klara, Inc.\)

The
[dtrace\(1\)](https://man.freebsd.org/cgi/man.cgi?query=dtrace&sektion=1&format=html)
utility can now generate machine\-readable output in JSON, XML, and HTML using
[libxo\(3\)](https://man.freebsd.org/cgi/man.cgi?query=libxo&sektion=3&format=html).
[aef4504139a4](https://cgit.freebsd.org/src/commit/?id=aef4504139a4) \(Sponsored
by Innovate UK\)

The
[lastcomm\(1\)](https://man.freebsd.org/cgi/man.cgi?query=lastcomm&sektion=1&format=html)
utility now displays timestamps with a precision of seconds.
[692c0a2e80c1](https://cgit.freebsd.org/src/commit/?id=692c0a2e80c1) \(Sponsored
by DSS Gmbh\)

The
[ldconfig\(8\)](https://man.freebsd.org/cgi/man.cgi?query=ldconfig&sektion=8&format=html)
utility now supports hints files of either byte order. The default format is the
native byte\-order of the host.
[fa7b31166ddb](https://cgit.freebsd.org/src/commit/?id=fa7b31166ddb)

The
[usbconfig\(8\)](https://man.freebsd.org/cgi/man.cgi?query=usbconfig&sektion=8&format=html)
utility now reads the descriptions of usb vendor and products from
/usr/share/misc/usb\_vendors when available, similar to what
[pciconf\(8\)](https://man.freebsd.org/cgi/man.cgi?query=pciconf&sektion=8&format=html)
does. [7b9a772f9f64](https://cgit.freebsd.org/src/commit/?id=7b9a772f9f64)

The
[env\(1\)](https://man.freebsd.org/cgi/man.cgi?query=env&sektion=1&format=html)
utility has gained an option to change the directory, which closely resembles
the feature in the GNU version of env, although it does not support long
options. [08e8554c4a39](https://cgit.freebsd.org/src/commit/?id=08e8554c4a39)
\(Sponsored by Klara, Inc.\)

The
[ps\(1\)](https://man.freebsd.org/cgi/man.cgi?query=ps&sektion=1&format=html)
utility now automatically removes canned displays' columns that contain same
data as some explicitly\-requested columns. Before this change, if some user
requested to add some "canned display" \(options `-j`, `-l`, `-u` or `-v`\),
columns in it that were duplicates of explicitly\-requested ones earlier on the
command line were omitted, but this did not work the other way around, when a
canned display appears before explicitly\-requested columns. Additionally,
columns with different keywords but which are aliases to the same keyword are
now also considered holding the same data, in addition to columns having the
same keyword.
[cd768a840644](https://cgit.freebsd.org/src/commit/?id=cd768a840644) \(Sponsored
by The FreeBSD Foundation\)

The
[ps\(1\)](https://man.freebsd.org/cgi/man.cgi?query=ps&sektion=1&format=html)
utility’s `-O` option is now more versatile and predictable. The
[ps\(1\)](https://man.freebsd.org/cgi/man.cgi?query=ps&sektion=1&format=html)
display’s list of columns is now first built without taking into account the
`-O` options. In a second step, all columns passed via `-O` are finally inserted
after the built\-so\-far display’s first PID column \(if it exists, else at
start\), in their order of appearance as arguments to the `-O` options.
[5dad61d9b949](https://cgit.freebsd.org/src/commit/?id=5dad61d9b949) \(Sponsored
by The FreeBSD Foundation\)

The
[ps\(1\)](https://man.freebsd.org/cgi/man.cgi?query=ps&sektion=1&format=html)
utility’s `-a` and `-A` options now always show all processes. When combined
with other options affecting the selection of processes, except for `-X` and
`-x`, option `-a` would have no effect \(and `-A` would reduce to just `-x`\).
This was in contradiction with the rule applying to all other selection options
stating that one process is listed as soon as any of these options has been
specified and selects it, which is both mandated by POSIX and arguably a natural
expectation. As a practical consequence, specifying `-a` or `-A` now causes all
processes to be listed regardless of other selection options such as `-U`, `-p`,
`-G`, etc., except for the `-X` and `-x` filter options, which continue to
apply. In particular, to list only processes from specific jails, one must not
use `-a` with `-J`. Option `-J`, contrary to its apparent initial intent, never
worked as a filter in practice, except by accident with only `-a` due to the
bug. [93a94ce731a8](https://cgit.freebsd.org/src/commit/?id=93a94ce731a8)
\(Sponsored by The FreeBSD Foundation\)

The
[ps\(1\)](https://man.freebsd.org/cgi/man.cgi?query=ps&sektion=1&format=html)
utility now matches current user’s processes using the effective user ID.
Previously, we would match using the real user ID. This puts
[ps\(1\)](https://man.freebsd.org/cgi/man.cgi?query=ps&sektion=1&format=html) in
conformance with POSIX on that topic.
[1aabbb25c9f9](https://cgit.freebsd.org/src/commit/?id=1aabbb25c9f9c4372)
\(Sponsored by The FreeBSD Foundation\)

The
[ps\(1\)](https://man.freebsd.org/cgi/man.cgi?query=ps&sektion=1&format=html)
utility’s `-U` flag now selects processes by real user IDs. This is what POSIX
mandates for option `-U` and arguably the behavior that most users actually need
in most cases. Before, `-U` would select processes by their effective user IDs
\(which is the behavior mandated by POSIX for option `-u`\).
[995b690d1398](https://cgit.freebsd.org/src/commit/?id=995b690d1398) \(Sponsored
by The FreeBSD Foundation\)

The
[sysctl\(8\)](https://man.freebsd.org/cgi/man.cgi?query=sysctl&sektion=8&format=html)
utility has gained flags to filter jail prison and vnet variables, so users do
not have to contact the source code to tell whether a variable is a jail prison
/ vnet one or not.
[615c9ce250ee](https://cgit.freebsd.org/src/commit/?id=615c9ce250ee)

The
[grep\(1\)](https://man.freebsd.org/cgi/man.cgi?query=grep&sektion=1&format=html)
utility no longer follows symbolic links by default for recursive searches. This
matches the documented behavior in the manual page.
[fc12c191c087](https://cgit.freebsd.org/src/commit/?id=fc12c191c087)

The
[mdo\(1\)](https://man.freebsd.org/cgi/man.cgi?query=mdo&sektion=1&format=html)
utility now supports fully specifying all users and groups in the target
credentials. As a convenience, in addition to a full explicit specification, it
allows starting from a baseline providing default values for all attributes,
which is either the login credentials from some user in the password database or
the current credentials, and then amending these attributes selectively. The
manual page has been updated to describe the new options and their interactions.
[4ffcb1a4a99c](https://cgit.freebsd.org/src/commit/?id=4ffcb1a4a99c) \(Sponsored
by The FreeBSD Foundation\) \(Sponsored by Google LLC \(GSoC 2025\)\)

When booting in single\-user mode,
[init\(8\)](https://man.freebsd.org/cgi/man.cgi?query=init&sektion=8&format=html)
now changes the working directory to `/root`, using `/` only as a fallback. The
`/.profile` link to `/root/.profile` is no more installed.
[b4b91207ab6f](https://cgit.freebsd.org/src/commit/?id=b4b91207ab6f),
[ca771d7ae527](https://cgit.freebsd.org/src/commit/?id=ca771d7ae527)

The deprecated
[ftpd\(8\)](https://man.freebsd.org/cgi/man.cgi?query=ftpd&sektion=8&format=html)
has been removed from the base system. Users who still need it can install the
`ftp/freebsd-ftpd` port.
[259bb93b80c0](https://cgit.freebsd.org/src/commit/?id=259bb93b80c0)

The Kerberos v5 database administration program learned how to dump the Heimdal
KDC database in a format which can be loaded into the MIT KDC. See
<https://wiki.freebsd.org/Kerberos/Heimdal2MIT_KDC_Migration> for how to use
`kadmin -l dump -f` to transfer/convert the KDC database.
[9fd3b28d4e0d](https://cgit.freebsd.org/src/commit/?id=9fd3b28d4e0d),
[23fbea8cf2f3](https://cgit.freebsd.org/src/commit/?id=23fbea8cf2f3)

The
[bsdconfig\(8\)](https://man.freebsd.org/cgi/man.cgi?query=bsdconfig&sektion=8&format=html)
and
[bsdinstall\(8\)](https://man.freebsd.org/cgi/man.cgi?query=bsdinstall&sektion=8&format=html)
utilities now use
[bsddialog\(1\)](https://man.freebsd.org/cgi/man.cgi?query=bsddialog&sektion=1&format=html)
instead of GNU dialog.
[c36b3dbc99d1](https://cgit.freebsd.org/src/commit/?id=c36b3dbc99d1),
[04b465777a09](https://cgit.freebsd.org/src/commit/?id=04b465777a09)

The
[jail\(8\)](https://man.freebsd.org/cgi/man.cgi?query=jail&sektion=8&format=html)
command now supports the `zfs.dataset` parameter to attach a list of ZFS
datasets to a jail.
[e0dfe185cbca](https://cgit.freebsd.org/src/commit/?id=e0dfe185cbca)

The
[jail\(8\)](https://man.freebsd.org/cgi/man.cgi?query=jail&sektion=8&format=html)
command now supports meta and env parameters, which are arbitrary strings
associated with a jail. These parameters can be used to tag jails with specific
metadata, or to pass information securely to be accessed inside a jail. They can
be added at jail creation, or modified later using
[jail\(8\)](https://man.freebsd.org/cgi/man.cgi?query=jail&sektion=8&format=html).
[30e6e008bc06](https://cgit.freebsd.org/src/commit/?id=30e6e008bc06) \(Sponsored
by SkunkWerks, GmbH\)

The `rc.d/jail` startup script now supports the legacy variable
`jail_${jailname}_zfs_dataset` to allow unmaintained jail managers like `ezjail`
to leverage the new `zfs.dataset` feature \(see above\).
[0b49e504a32d](https://cgit.freebsd.org/src/commit/?id=0b49e504a32d)

The
[newsyslog\(8\)](https://man.freebsd.org/cgi/man.cgi?query=newsyslog&sektion=8&format=html)
utility now supports specifying a global compression method directly at the
beginning of the `newsyslog.conf` file. All historical compression flags \(`J`,
`X`, `Y`, `Z`\) then behave as indicating "treat the file as compressible"
instead of "compress the file with that specific method.". The following methods
are available:

- `none`: Never compress.
- `legacy`: Historical behavior \(`J`=bzip2, `X`=xz, `Y`=zstd, `Z`=gzip\).
- `bzip2`, `xz`, `zstd`, `gzip`: apply the specified compression method.
  [61174ad88e33](https://cgit.freebsd.org/src/commit/?id=61174ad88e33),
  [906748d208d3](https://cgit.freebsd.org/src/commit/?id=906748d208d3),
  [39d668f1e09e](https://cgit.freebsd.org/src/commit/?id=39d668f1e09e)

### Contributed Software

One True Awk
\([awk\(1\)](https://man.freebsd.org/cgi/man.cgi?query=awk&sektion=1&format=html)\)
has been updated to 2nd Edition, with new \-csv support and UTF\-8 support. The
snapshot used is 20250804.
[b45a181a74c8](https://cgit.freebsd.org/src/commit/?id=b45a181a74c8) \(Sponsored
by Netflix\)

The system reference manual toolchain,
[mandoc\(1\)](https://man.freebsd.org/cgi/man.cgi?query=mandoc&sektion=1&format=html),
has been updated to version 1.14.6 snapshot 2025\-09\-26. This version includes
improved compatibility with groff and DocBook, improved html and markdown
output, and the deprecation of the LIBRARY section.
[c1c95add8c80](https://cgit.freebsd.org/src/commit/?id=c1c95add8c80),
[80c12959679a](https://cgit.freebsd.org/src/commit/?id=80c12959679a),
[4c07abdbacf4](https://cgit.freebsd.org/src/commit/?id=4c07abdbacf4),
[06410c1b5163](https://cgit.freebsd.org/src/commit/?id=06410c1b5163),
[59fc2b0166f7](https://cgit.freebsd.org/src/commit/?id=59fc2b0166f7)

The
[jemalloc\(3\)](https://man.freebsd.org/cgi/man.cgi?query=jemalloc&sektion=3&format=html)
library has been updated to version 5.3.0.
[c43cad871720](https://cgit.freebsd.org/src/commit/?id=c43cad871720)

The
[bmake\(1\)](https://man.freebsd.org/cgi/man.cgi?query=bmake&sektion=1&format=html)
build system has been upgraded to 20250804, providing many debugging
improvements, bug fixes such as detecting and rejecting `gmake` syntax, and
feature improvements such as a floating point argument to `-j` being used as a
multiple of the number of cpus available.

The
[sendmail\(8\)](https://man.freebsd.org/cgi/man.cgi?query=sendmail&sektion=8&format=html)
suite has been upgraded to version 8.18.1, addressing CVE\-2023\-51765.
[58ae50f31e95](https://cgit.freebsd.org/src/commit/?id=58ae50f31e95)

The
[bc\(1\)](https://man.freebsd.org/cgi/man.cgi?query=bc&sektion=1&format=html)
calculator has been upgraded to 7.1.0.
[fdc4a7c8012b](https://cgit.freebsd.org/src/commit/?id=fdc4a7c8012b)

The `blacklist` suite has been renamed upstream to `blocklist`. Existing setups
will continue to work emitting a warning. The snapshot used is 20251026.
[4afb96fdd272](https://cgit.freebsd.org/src/commit/?id=4afb96fdd272)

The
[bsddialog\(1\)](https://man.freebsd.org/cgi/man.cgi?query=bsddialog&sektion=1&format=html)
utility has been upgraded to 1.0.5.
[0595e10ec773](https://cgit.freebsd.org/src/commit/?id=0595e10ec773)

The
[byacc\(1\)](https://man.freebsd.org/cgi/man.cgi?query=byacc&sektion=1&format=html)
parser generator has been upgraded to 20240109.
[822ca3276345](https://cgit.freebsd.org/src/commit/?id=822ca3276345)

The `libarchive` library has been upgraded to 3.8.2.
[8a0b57ba54f0](https://cgit.freebsd.org/src/commit/?id=8a0b57ba54f0)

The `libcbor` library has been upgraded to 0.11.0.
[1755b9daa693](https://cgit.freebsd.org/src/commit/?id=1755b9daa693) \(Sponsored
by The FreeBSD Foundation\)

The `libcxxrt` library has been upgraded to vendor snapshot 6f2fdfebcd62.
[d0dcee46d971](https://cgit.freebsd.org/src/commit/?id=d0dcee46d971)

The `libfido2` library has been upgraded to 1.14.0.
[128bace5102e](https://cgit.freebsd.org/src/commit/?id=128bace5102e) \(Sponsored
by The FreeBSD Foundation\)

The `libpcap` library has been upgraded to 1.10.5.
[26f21a6494b4](https://cgit.freebsd.org/src/commit/?id=26f21a6494b4) \(Sponsored
by The FreeBSD Foundation\)

The
[ncurses\(3\)](https://man.freebsd.org/cgi/man.cgi?query=ncurses&sektion=3&format=html)
library has been upgraded to 6.5.
[21817992b331](https://cgit.freebsd.org/src/commit/?id=21817992b331)

The
[tcpdump\(1\)](https://man.freebsd.org/cgi/man.cgi?query=tcpdump&sektion=1&format=html)
utility has been upgraded to 4.99.5.
[ec3da16d8bc1](https://cgit.freebsd.org/src/commit/?id=ec3da16d8bc1) \(Sponsored
by The FreeBSD Foundation\)

The `unbound` DNS validating resolver has been upgraded to 1.24.1.
[a988846174e0](https://cgit.freebsd.org/src/commit/?id=a988846174e0)

The `llvm` compiler infrastructure has been upgraded to
19.1.7\-0\-gcd708029e0b2.
[dc3f24ea8a25](https://cgit.freebsd.org/src/commit/?id=dc3f24ea8a25)

The OpenZFS filesystem has been updated to zfs\-2.4.0\-rc4.
[7b5b0f43eb06](https://cgit.freebsd.org/src/commit/?id=7b5b0f43eb06)

The
[xz\(1\)](https://man.freebsd.org/cgi/man.cgi?query=xz&sektion=1&format=html)
data compressors have been updated to 5.8.1.
[128836d304d9](https://cgit.freebsd.org/src/commit/?id=128836d304d9)

The
[less\(1\)](https://man.freebsd.org/cgi/man.cgi?query=less&sektion=1&format=html)
pager has been updated to v679.
[76bafc906926](https://cgit.freebsd.org/src/commit/?id=76bafc906926)

The
[file\(1\)](https://man.freebsd.org/cgi/man.cgi?query=file&sektion=1&format=html)
identifier has been updated to 5.46.
[ae316d1d1cff](https://cgit.freebsd.org/src/commit/?id=ae316d1d1cff)

The
[zlib\(3\)](https://man.freebsd.org/cgi/man.cgi?query=zlib&sektion=3&format=html)
data compression library has been updated to 1.3.1.
[6255c67c3d1a](https://cgit.freebsd.org/src/commit/?id=6255c67c3d1a)

The Time Zone Database, `tzdata`, has been updated to 2025b.
[475082194ac8](https://cgit.freebsd.org/src/commit/?id=475082194ac8)

OpenSSH has been updated to 10.0p2.
.[8e28d84935f2](https://cgit.freebsd.org/src/commit/?id=8e28d84935f2)
\(Sponsored by The FreeBSD Foundation\)

OpenSSL has been updated to 3.5.4.
[c0366f908ff4](https://cgit.freebsd.org/src/commit/?id=c0366f908ff4)

Lua has been updated to 5.4.8.
[3068d706eabe](https://cgit.freebsd.org/src/commit/?id=3068d706eabe) \(Sponsored
by Netflix\)

The Google Test C testing framework has been updated to 1.15.2. One notable
change is that GoogleTest 1.15.x now officially requires C\-14 \(1.14.x required
C\+\+\-11\).
[1d67cec52542](https://cgit.freebsd.org/src/commit/?id=1d67cec52542)

The `spleen`
[vt\(4\)](https://man.freebsd.org/cgi/man.cgi?query=vt&sektion=4&format=html)
console font has been updated to version 2.1.0.
[26336203d32c](https://cgit.freebsd.org/src/commit/?id=26336203d32c)

MIT KRB5 1.22.1 Kerberos replaces Heimdal 1.5.2 by default. Heimdal 1.5.2 can
still be built using the `WITHOUT_MITKRB5` flag. Heimdal Kerberos will be
entirely removed in FreeBSD 16. See also the note about the `-f` flag to `kadmin
-l dump` under section [Userland Application Changes](#userland-programs).
[ee3960cba106](https://cgit.freebsd.org/src/commit/?id=ee3960cba106),
[0b9a631e0724](https://cgit.freebsd.org/src/commit/?id=0b9a631e0724),
[60f970b85e44](https://cgit.freebsd.org/src/commit/?id=60f970b85e44),
[0d1496f0f1e7](https://cgit.freebsd.org/src/commit/?id=0d1496f0f1e7),
[cbb6e747af98](https://cgit.freebsd.org/src/commit/?id=cbb6e747af98),
[0559f30a882d](https://cgit.freebsd.org/src/commit/?id=0559f30a882d),
[ae07a5805b19](https://cgit.freebsd.org/src/commit/?id=ae07a5805b19),
[f58febc4cefa](https://cgit.freebsd.org/src/commit/?id=f58febc4cefa),
[805498e49ae4](https://cgit.freebsd.org/src/commit/?id=805498e49ae4),
[4cb1baa7d85c](https://cgit.freebsd.org/src/commit/?id=4cb1baa7d85c),
[188138106b9f](https://cgit.freebsd.org/src/commit/?id=188138106b9f),
[4680e7fcc70a](https://cgit.freebsd.org/src/commit/?id=4680e7fcc70a),
[e447c252d0ec](https://cgit.freebsd.org/src/commit/?id=e447c252d0ec),
[5f8493bbf479](https://cgit.freebsd.org/src/commit/?id=5f8493bbf479),
[110111a6cca1](https://cgit.freebsd.org/src/commit/?id=110111a6cca1),
[2a454b05f2c1](https://cgit.freebsd.org/src/commit/?id=2a454b05f2c1),
[98d46e05ab08](https://cgit.freebsd.org/src/commit/?id=98d46e05ab08),
[6b28571cb6ba](https://cgit.freebsd.org/src/commit/?id=6b28571cb6ba),
[ca9ccf0ce9ad](https://cgit.freebsd.org/src/commit/?id=ca9ccf0ce9ad),
[b98d0566b2bd](https://cgit.freebsd.org/src/commit/?id=b98d0566b2bd),
[fb1ccc04adfe](https://cgit.freebsd.org/src/commit/?id=fb1ccc04adfe),
[dd0ec030f8fd](https://cgit.freebsd.org/src/commit/?id=dd0ec030f8fd),
[6c4771c73470](https://cgit.freebsd.org/src/commit/?id=6c4771c73470),
[7b68893ffa9b](https://cgit.freebsd.org/src/commit/?id=7b68893ffa9b),
[624b7beed5ac](https://cgit.freebsd.org/src/commit/?id=624b7beed5ac),
[04764f21855a](https://cgit.freebsd.org/src/commit/?id=04764f21855a),
[73ed0c7992fd](https://cgit.freebsd.org/src/commit/?id=73ed0c7992fd),
[40a5abfc3f66](https://cgit.freebsd.org/src/commit/?id=40a5abfc3f66),
[543b875a8ee4](https://cgit.freebsd.org/src/commit/?id=543b875a8ee4),
[c791ea80b5f7](https://cgit.freebsd.org/src/commit/?id=c791ea80b5f7),
[383e7290c0b5](https://cgit.freebsd.org/src/commit/?id=383e7290c0b5),
[9a726ef24134](https://cgit.freebsd.org/src/commit/?id=9a726ef24134),
[a245dc5d68c7](https://cgit.freebsd.org/src/commit/?id=a245dc5d68c7),
[e26259f48afe](https://cgit.freebsd.org/src/commit/?id=e26259f48afe),
[7d2cfb27d62f](https://cgit.freebsd.org/src/commit/?id=7d2cfb27d62f),
[619feb9dd00e](https://cgit.freebsd.org/src/commit/?id=619feb9dd00e),
[10eecc467f32](https://cgit.freebsd.org/src/commit/?id=10eecc467f32),
[0c13e9c3c464](https://cgit.freebsd.org/src/commit/?id=0c13e9c3c464),
[89c82750da1a](https://cgit.freebsd.org/src/commit/?id=89c82750da1a),
[18a870751b03](https://cgit.freebsd.org/src/commit/?id=18a870751b03),
[ce9c325a2e92](https://cgit.freebsd.org/src/commit/?id=ce9c325a2e92),
[cb3eac927b5d](https://cgit.freebsd.org/src/commit/?id=cb3eac927b5d),
[5105e1ebecc7](https://cgit.freebsd.org/src/commit/?id=5105e1ebecc7),
[b9b0e105c357](https://cgit.freebsd.org/src/commit/?id=b9b0e105c357),
[929f5966a9fd](https://cgit.freebsd.org/src/commit/?id=929f5966a9fd) \(Sponsored
by The FreeBSD Foundation\)

The
[rtw88\(4\)](https://man.freebsd.org/cgi/man.cgi?query=rtw88&sektion=4&format=html)
driver has been updated to Linux v6.17. A possible issue that devices cannot
authenticate is still being investigated.
[c1d365f39e08](https://cgit.freebsd.org/src/commit/?id=c1d365f39e08) \(Sponsored
by The FreeBSD Foundation\)

The
[rtw89\(4\)](https://man.freebsd.org/cgi/man.cgi?query=rtw89&sektion=4&format=html)
driver has been updated to Linux v6.17. The driver is under\-tested and may
still have issues.
[b35044b38f74](https://cgit.freebsd.org/src/commit/?id=b35044b38f74) \(Sponsored
by The FreeBSD Foundation\)

The
[iwlwifi\(4\)](https://man.freebsd.org/cgi/man.cgi?query=iwlwifi&sektion=4&format=html)
driver has been updated to Linux v6.17. The BE200 based chipsets will need newer
firmware requiring further driver fixes which are not in this release.
[69caa1cf3ce5](https://cgit.freebsd.org/src/commit/?id=69caa1cf3ce5) \(Sponsored
by The FreeBSD Foundation\)

### Runtime Libraries and API

The
[setusercontext\(3\)](https://man.freebsd.org/cgi/man.cgi?query=setusercontext&sektion=3&format=html)
routine in `libutil` will now set the process priority \(nice\) from the
.login.conf file from the home directory under appropriate conditions, as well
as the system
[login.conf\(5\)](https://man.freebsd.org/cgi/man.cgi?query=login.conf&sektion=5&format=html).
The priority can now have the value `inherit`, indicating that the priority
should be unchanged from that of the parent process. Similarly, the umask can
have the value `inherit`.
[c328e6c6ccaa](https://cgit.freebsd.org/src/commit/?id=c328e6c6ccaa),
[d162d7e2ad32](https://cgit.freebsd.org/src/commit/?id=d162d7e2ad32),
[f2a0277d3e51](https://cgit.freebsd.org/src/commit/?id=f2a0277d3e51) \(Sponsored
by Kumacom SAS\)

Many string and memory operations in the C library now use SIMD \(single
instruction multiple data\) extensions for improved performance when available
on amd64 systems; see
[simd\(7\)](https://man.freebsd.org/cgi/man.cgi?query=simd&sektion=7&format=html).
\(Sponsored by The FreeBSD Foundation\)

There is now a much better implementation of the 128\-bit `tgammal` function in
the math library,
[math\(3\)](https://man.freebsd.org/cgi/man.cgi?query=math&sektion=3&format=html),
on platforms that support it.
[8df6c930c151](https://cgit.freebsd.org/src/commit/?id=8df6c930c151)

[fma\(3\)](https://man.freebsd.org/cgi/man.cgi?query=fma&sektion=3&format=html)
now returns correctly\-signed zero when provided certain small inputs \(as
observed in the Python test suite\).
[dc39004bc670](https://cgit.freebsd.org/src/commit/?id=dc39004bc670) \(Sponsored
by The FreeBSD Foundation\)

The `cap_rights_is_empty` function has been added. It reports whether a
`cap_rights_t` has no rights set.
[e77813f7e4a3](https://cgit.freebsd.org/src/commit/?id=e77813f7e4a3) \(Sponsored
by The FreeBSD Foundation\)

`libcxxrt` has been updated to upstream 6f2fdfebcd62.
[d9901a23bd2f](https://cgit.freebsd.org/src/commit/?id=d9901a23bd2f)

The accuracy of
[asinf\(3\)](https://man.freebsd.org/cgi/man.cgi?query=asinf&sektion=3&format=html)
and
[acosf\(3\)](https://man.freebsd.org/cgi/man.cgi?query=acosf&sektion=3&format=html)
has improved.
[33c82f11c267](https://cgit.freebsd.org/src/commit/?id=33c82f11c267)

The
[setgroups\(2\)](https://man.freebsd.org/cgi/man.cgi?query=setgroups&sektion=2&format=html)
and
[getgroups\(2\)](https://man.freebsd.org/cgi/man.cgi?query=getgroups&sektion=2&format=html)
system calls and the
[initgroups\(3\)](https://man.freebsd.org/cgi/man.cgi?query=initgroups&sektion=3&format=html)
library function have been changed to avoid setting or reporting the effective
group ID, now only concerning themselves with the supplementary groups. The main
purpose of this change is to avoid security issues going forward by becoming
compatible with Linux/glibc, OpenBSD, NetBSD and illumos\-based systems.
Consequently, almost all portable applications should already be compliant with
this new behavior and will continue to work correctly or even get fixed in the
process \(see, e.g.,
[239e8c98636a](https://cgit.freebsd.org/src/commit/?id=239e8c98636a) for an
example affecting OpenSSH\). However, out of caution, porters, system
administrators and users are advised to audit their applications using
[setgroups\(2\)](https://man.freebsd.org/cgi/man.cgi?query=setgroups&sektion=2&format=html),
[getgroups\(2\)](https://man.freebsd.org/cgi/man.cgi?query=getgroups&sektion=2&format=html)
and
[initgroups\(3\)](https://man.freebsd.org/cgi/man.cgi?query=initgroups&sektion=3&format=html),
watching out for the following points. Applications must be using
[setgid\(2\)](https://man.freebsd.org/cgi/man.cgi?query=setgid&sektion=2&format=html)
or
[setegid\(2\)](https://man.freebsd.org/cgi/man.cgi?query=setegid&sektion=2&format=html)
in addition to
[setgroups\(2\)](https://man.freebsd.org/cgi/man.cgi?query=setgroups&sektion=2&format=html)
or
[initgroups\(3\)](https://man.freebsd.org/cgi/man.cgi?query=initgroups&sektion=3&format=html)
to set the effective group ID. They must not treat the first element of the
array returned by
[getgroups\(2\)](https://man.freebsd.org/cgi/man.cgi?query=getgroups&sektion=2&format=html)
specially, but instead as any other supplementary group. For more information,
please consult the SECURITY CONSIDERATIONS sections that have been added to the
[setgroups\(2\)](https://man.freebsd.org/cgi/man.cgi?query=setgroups&sektion=2&format=html),
[getgroups\(2\)](https://man.freebsd.org/cgi/man.cgi?query=getgroups&sektion=2&format=html)
and
[initgroups\(3\)](https://man.freebsd.org/cgi/man.cgi?query=initgroups&sektion=3&format=html)
manual pages. Compatibility system calls and library functions have been
provided so that binaries and libraries compiled on FreeBSD 14 systems or
earlier will continue to work exactly as before.
[9da2fe96ff2e](https://cgit.freebsd.org/src/commit/?id=9da2fe96ff2e),
[8878569103a3](https://cgit.freebsd.org/src/commit/?id=8878569103a3),
[7132fb5edbc9](https://cgit.freebsd.org/src/commit/?id=7132fb5edbc9),
[2932e6f59bff](https://cgit.freebsd.org/src/commit/?id=2932e6f59bff),
[8878569103a3](https://cgit.freebsd.org/src/commit/?id=8878569103a3) \(Sponsored
by The FreeBSD Foundation\)

`libc` contains compatibility functions enabling running executables/libraries
compiled for older versions of FreeBSD. Those that are themselves using
compatibility system calls would not reference them correctly, causing
misbehavior at runtime. This has been fixed.
[47f5f89dbd27](https://cgit.freebsd.org/src/commit/?id=47f5f89dbd27) \(Sponsored
by The FreeBSD Foundation\)

The
[readdir\_r\(3\)](https://man.freebsd.org/cgi/man.cgi?query=readdir_r&sektion=3&format=html)
function is deprecated and may be removed in future releases. Using it in a
program will result in compile\-time and link\-time warnings.
[2bd157bc732a](https://cgit.freebsd.org/src/commit/?id=2bd157bc732a) \(Sponsored
by Klara, Inc.\)

The runtime linker
[rtld\(1\)](https://man.freebsd.org/cgi/man.cgi?query=rtld&sektion=1&format=html)
has grown support for the static linker flag specified by `-z initfirst`.
[78aaab9f1cf3](https://cgit.freebsd.org/src/commit/?id=78aaab9f1cf359f)
\(Sponsored by The FreeBSD Foundation\)

### Miscellaneous

The Gallant font for
[vt\(4\)](https://man.freebsd.org/cgi/man.cgi?query=vt&sektion=4&format=html)
has been updated with more than 4300 new glyphs, including support for Greek,
Cyrillic, International Phonetic Association Extensions, Extended Latin
characters, Zapf Dingbats, Tons of arrows, Tons of mathematical symbols,
Letterlike symbols and enclosed alphanumerics, Pixel\-perfect box drawing,
Currency symbols, More punctuation, Just enough Katakana to say コンニチハ, Powerline
glyphs in the Private Use Area at U\+e0a0.
[9e8c1ab0976c](https://cgit.freebsd.org/src/commit/?id=9e8c1ab0976c)

Unicode support has been updated to 16.0.0 and CLDR to 45.0.0.
[ddfc6f84f242](https://cgit.freebsd.org/src/commit/?id=ddfc6f84f242)

### Deprecated Applications

[fdisk\(8\)](https://man.freebsd.org/cgi/man.cgi?query=fdisk&sektion=8&format=html)
has been deprecated in favor of
[gpart\(8\)](https://man.freebsd.org/cgi/man.cgi?query=gpart&sektion=8&format=html)
for a long time but has not been removed, running this application will show a
warning to migrate to
[gpart\(8\)](https://man.freebsd.org/cgi/man.cgi?query=gpart&sektion=8&format=html).
[3958be5c29da](https://cgit.freebsd.org/src/commit/?id=3958be5c29da) \(Sponsored
by The FreeBSD Foundation\)

Deprecation notice for
[syscons\(4\)](https://man.freebsd.org/cgi/man.cgi?query=syscons&sektion=4&format=html)
has been added.
[syscons\(4\)](https://man.freebsd.org/cgi/man.cgi?query=syscons&sektion=4&format=html)
is not compatible with UEFI, does not support UTF\-8, and is Giant\-locked.
There is no specific timeline yet for removing it, but support for the Giant
lock is expected to go away in one or two major release cycles.
[8c922db4f3d9](https://cgit.freebsd.org/src/commit/?id=8c922db4f3d9) \(Sponsored
by The FreeBSD Foundation\)

The `shar` utility has been removed. It lives on as a port at
[sysutils/freebsd\-shar](https://cgit.freebsd.org/ports/tree/sysutils/freebsd-shar/).
[3fde39073c72](https://cgit.freebsd.org/src/commit/?id=3fde39073c72)

The cryptographically weak DSA signature algorithm was removed from OpenSSH,
following upstream.

The
[publickey\(5\)](https://man.freebsd.org/cgi/man.cgi?query=publickey&sektion=5&format=html)
database has been removed, This uses DES and we hope that nobody uses that in
2025. [9197c04a251b](https://cgit.freebsd.org/src/commit/?id=9197c04a251b)

## Cloud Support

This section covers changes in support for cloud environments.

15.0\-RELEASE supports cloudinit, including the `nuageinit` startup script and
support for a `config-drive` partition. It is compatible with OpenStack and many
hosting facilities. See the [cloud\-init](https://cloud-init.io) web site and
the commit messages,
[16a6da44e28d](https://cgit.freebsd.org/src/commit/?id=16a6da44e28d)
[227e7a205edf](https://cgit.freebsd.org/src/commit/?id=227e7a205edf) \(Sponsored
by OVHcloud\)

Basic Cloudinit images no longer generate RSA host keys by default for SSH.
[b22be3bbb2de](https://cgit.freebsd.org/src/commit/?id=b22be3bbb2de)

The FreeBSD project is now publishing OCI\-compatible container images.
[8a688fcc242e](https://cgit.freebsd.org/src/commit/?id=8a688fcc242e)

The FreeBSD project is now publishing Oracle Cloud Infrastructure images. See
the [Oracle Cloud Infrastructure FreeBSD
Listing](https://cloudmarketplace.oracle.com/marketplace/app/freebsd-release)
for more information.
[77b296a2582b](https://cgit.freebsd.org/src/commit/?id=77b296a2582b)

The "shutdown" and "reboot" API in the Amazon EC2 cloud now work for arm64
\("Graviton"\) instances.
[28b881840df7](https://cgit.freebsd.org/src/commit/?id=28b881840df7) \(Sponsored
by Amazon\)

Several bug fixes and configuration changes collectively allow device hotplug on
both x86 and arm64 \("Graviton"\) EC2 instances.
[ce9a34b1614e](https://cgit.freebsd.org/src/commit/?id=ce9a34b1614e)
[55c3348ed78f](https://cgit.freebsd.org/src/commit/?id=55c3348ed78f)
[d70bac252d30](https://cgit.freebsd.org/src/commit/?id=d70bac252d30) \(Sponsored
by Amazon\)

Users upgrading EC2 instances from earlier FreeBSD releases should set
`hw.pci.intx_reroute=0` and `debug.acpi.quirks="56"` in `/boot/loader.conf`.

The FreeBSD project now publishes "small" EC2 images; these are the "base"
images minus debug symbols, tests, 32\-bit libraries, the LLDB debugger, the
Amazon SSM Agent, and the AWS CLI.
[953142d6baf3](https://cgit.freebsd.org/src/commit/?id=953142d6baf3) \(Sponsored
by Amazon\)

The FreeBSD project now publishes "builder" EC2 images; these boot into a memory
disk and extract a clean "base" image onto the root disk \(mounted at `/mnt`\)
to be customized before creating an AMI.
[584265890303](https://cgit.freebsd.org/src/commit/?id=584265890303) \(Sponsored
by Amazon\)

FreeBSD "base" EC2 images now boot up to 76% faster than corresponding
14.0\-RELEASE images, with the largest improvements found on arm64
\("Graviton"\) instances.

EC2 AMIs no longer generate RSA host keys by default for SSH. RSA host key
generation can be re\-enabled by setting `sshd_rsa_enable="YES"` in
`/etc/rc.conf` if it is necessary to support very old SSH clients.
[0aabcd75dbc2](https://cgit.freebsd.org/src/commit/?id=0aabcd75dbc2) \(Sponsored
by Amazon\)

FreeBSD 15.0\-RELEASE now supports Google Cloud Compute Engine C4 machines.
[7b32f4f0a7fe](https://cgit.freebsd.org/src/commit/?id=7b32f4f0a7fe) \(Sponsored
by Google\)

## Kernel

This section covers changes to kernel configurations, system tuning, and system
control parameters that are not otherwise categorized.

### General Kernel Changes

ktrace\(2\) will now record detailed information about capability mode
violations. The kdump\(1\) utility has been updated to display such information.
[9bec84131215](https://cgit.freebsd.org/src/commit/?id=9bec84131215),
[96c8b3e50988](https://cgit.freebsd.org/src/commit/?id=96c8b3e50988),
[05296a0ff616](https://cgit.freebsd.org/src/commit/?id=05296a0ff616),
[6a4616a529c1](https://cgit.freebsd.org/src/commit/?id=6a4616a529c1),
[0cd9cde767c3](https://cgit.freebsd.org/src/commit/?id=0cd9cde767c3),
[aa32d7cbc92c](https://cgit.freebsd.org/src/commit/?id=aa32d7cbc92c)

FreeBSD now natively implements the Linux
[inotify\(2\)](https://man.freebsd.org/cgi/man.cgi?query=inotify&sektion=2&format=html)
interface. The system calls themselves are not API\-compatible, but libc
provides an API\-compatible interface, so software which relies on inotify can
be run unmodified.
[f1f230439fa4](https://cgit.freebsd.org/src/commit/?id=f1f230439fa4),
\(Sponsored by Klara, Inc.\)

The `fpu_kern_enter` and `fpu_kern_leave` routines have been implemented for
powerpc, allowing the use of
[ossl\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ossl&sektion=4&format=html)
crypto functions in the kernel that use floating point and vector registers.
[91e53779b4fc](https://cgit.freebsd.org/src/commit/?id=91e53779b4fc)

Support legacy PCI hotplug on arm64.
[355f02cddbf0](https://cgit.freebsd.org/src/commit/?id=355f02cddbf0).
\(Sponsored by Arm Ltd\)

Jails can now be accessed via jail descriptors in
[jail\_set\(2\)](https://man.freebsd.org/cgi/man.cgi?query=jail_set&sektion=2&format=html)
and
[jail\_get\(2\)](https://man.freebsd.org/cgi/man.cgi?query=jail_get&sektion=2&format=html),
as well as the new `jail_attach_jd(2)` and `jail_remove_jd(2)` syscalls. They
allow manipulation of jails through the file descriptor interface without the
race conditions inherent in jail IDs, and can also optionally control jail
lifetime. [851dc7f859c2](https://cgit.freebsd.org/src/commit/?id=851dc7f859c2)

Jails and jail descriptors now have associated
[kevent\(2\)](https://man.freebsd.org/cgi/man.cgi?query=kevent&sektion=2&format=html)
filters that allow tracking jail creation, changes, attachment, and removal.
[1bd74d201a53](https://cgit.freebsd.org/src/commit/?id=1bd74d201a53)
[9d7f89ef2607](https://cgit.freebsd.org/src/commit/?id=9d7f89ef2607)

A new common 'mac' node for MAC modules' jail parameters has been created. All
future MAC modules' jail parameters will appear under this node. See
[mac\(4\)](https://man.freebsd.org/cgi/man.cgi?query=mac&sektion=4&format=html)
for an introduction to MAC. First consumer is
[mac\_do\(4\)](https://man.freebsd.org/cgi/man.cgi?query=mac_do&sektion=4&format=html).
[5041b20503db](https://cgit.freebsd.org/src/commit/?id=5041b20503db),
[f3a06ced2568](https://cgit.freebsd.org/src/commit/?id=f3a06ced2568) \(Sponsored
by The FreeBSD Foundation\)

[mac\_do\(4\)](https://man.freebsd.org/cgi/man.cgi?query=mac_do&sektion=4&format=html)
is now considered production\-ready, after a number of important fixes.
[bbf8af664dc9](https://cgit.freebsd.org/src/commit/?id=bbf8af664dc9),
[292c814931d9](https://cgit.freebsd.org/src/commit/?id=292c814931d9),
[53d2e0d48549](https://cgit.freebsd.org/src/commit/?id=53d2e0d48549),
[add521c1a5d2](https://cgit.freebsd.org/src/commit/?id=add521c1a5d2),
[2a20ce91dc29](https://cgit.freebsd.org/src/commit/?id=2a20ce91dc29),
[fa4352b74580](https://cgit.freebsd.org/src/commit/?id=fa4352b74580),
[3d8d91a5b32c](https://cgit.freebsd.org/src/commit/?id=3d8d91a5b32c),
[8f7e8726e3f5](https://cgit.freebsd.org/src/commit/?id=8f7e8726e3f5),
[89958992b618](https://cgit.freebsd.org/src/commit/?id=89958992b618) \(Sponsored
by The FreeBSD Foundation\)

[mac\_do\(4\)](https://man.freebsd.org/cgi/man.cgi?query=mac_do&sektion=4&format=html)
now supports changing rules within jails with the `security.mac.do.rules`
[sysctl\(8\)](https://man.freebsd.org/cgi/man.cgi?query=sysctl&sektion=8&format=html)
knob. [b3f93680e39b](https://cgit.freebsd.org/src/commit/?id=b3f93680e39b)
\(Sponsored by The FreeBSD Foundation\)

Introduce the
[setcred\(2\)](https://man.freebsd.org/cgi/man.cgi?query=setcred&sektion=2&format=html)
system call and associated MAC hooks. This new system call allows to set all
necessary credentials of a process in one go: Effective, real and saved user
IDs, effective, real and saved group IDs, supplementary groups and the MAC
label. Besides providing atomicity, its advantage over standard
credentials\-setting system calls, such as `setuid()`, `seteuid()`, etc., is
that it enables MAC modules, such as
[mac\_do\(4\)](https://man.freebsd.org/cgi/man.cgi?query=mac_do&sektion=4&format=html),
to restrict the set of credentials some process may gain in a fine\-grained
manner, as they can now see the final desired state and compare it with the
initial one.
[ddb3eb4efe55](https://cgit.freebsd.org/src/commit/?id=ddb3eb4efe55) \(Sponsored
by The FreeBSD Foundation\)

Support multiple users and groups as single rule’s targets in
[mac\_do\(4\)](https://man.freebsd.org/cgi/man.cgi?query=mac_do&sektion=4&format=html).
Supporting group targets is a requirement for
[mac\_do\(4\)](https://man.freebsd.org/cgi/man.cgi?query=mac_do&sektion=4&format=html)
to be able to enforce a limited set of valid new groups in the target
credentials and to allow group\-only credentials transitions. The allowed groups
are tied to one or multiple user IDs. Multiple users and groups in a rule’s
target part are treated as alternatives \(inclusive disjunction\), except for
the clauses expressing the mandatory presence or absence of a supplementary
group. The rules syntax has been changed incompatibly, but migrating existing
rules is just a matter of adding `uid=` in front of the target part,
substituting commas \(`,`\) with semi\-colons \(`;`\) and colons \(`:`\) with
greater\-than signs \(`>`\). Please consult the
[mac\_do\(4\)](https://man.freebsd.org/cgi/man.cgi?query=mac_do&sektion=4&format=html)
manual page for more information.
[83ffc412b2e9](https://cgit.freebsd.org/src/commit/?id=83ffc412b2e9),
[8f7e8726e3f5](https://cgit.freebsd.org/src/commit/?id=8f7e8726e3f5),
[f01d26dec67f](https://cgit.freebsd.org/src/commit/?id=f01d26dec67f) \(Sponsored
by The FreeBSD Foundation\)

Teach
[sysctl\(8\)](https://man.freebsd.org/cgi/man.cgi?query=sysctl&sektion=8&format=html)
to attach and run itself in a jail. This allows the parent jail to retrieve or
set kernel state when child does not have
[sysctl\(8\)](https://man.freebsd.org/cgi/man.cgi?query=sysctl&sektion=8&format=html)
installed \(for example light weighted OCI containers or slim jails\). This is
especially useful when manipulating jail prison or vnet sysctls. For example,
`sysctl -j foo -Ja` or `sysctl -j foo net.fibs=2`.
[8d5d7e2ba3a6](https://cgit.freebsd.org/src/commit/?id=8d5d7e2ba3a6).

Enable vnet
[sysctl\(9\)](https://man.freebsd.org/cgi/man.cgi?query=sysctl&sektion=9&format=html)
variables to be loader tunable. In
[3da1cf1e88f8](https://cgit.freebsd.org/src/commit/?id=3da1cf1e88f8), the
meaning of the flag `CTLFLAG_TUN` is extended to automatically check if there is
a kernel environment variable which shall initialize the `SYSCTL` during early
boot. It works for all `SYSCTL` types both statically and dynamically created
ones, except for the `SYSCTLs` which belong to VNETs. Note that the
implementation has a limitation. It behaves the same way as that of non\-vnet
loader tunables. That is, after the kernel or modules being initialized, any
changes \(for example via `kenv`\) to kernel environment variable will not
affect the corresponding vnet variable of subsequently created VNETs. To
overcome it, `TUNABLE_XXX_FETCH` can be used to fetch the kernel environment
variable into those vnet variables during vnet constructing.
[894efae09de4](https://cgit.freebsd.org/src/commit/?id=894efae09de4)

[sound\(4\)](https://man.freebsd.org/cgi/man.cgi?query=sound&sektion=4&format=html):
Allocate vchans on\-demand. Refactor `pcm_chnalloc()` and merge with parts of
`vchan_setnew()` \(now removed\) and `dsp_open()`’s channel creation into a `new
dsp_chn_alloc()` function. The function is responsible for either using a free
HW channel \(if `vchans` are disabled\), or allocating a new vchan.
`hw.snd.vchans_enable` \(previously `hw.snd.maxautovchans`\) and
`dev.pcm.X.{play|rec}.vchans` now work as tunables to only enable/disable
`vchans`, as opposed to setting their number and/or \(de\-\)allocating vchans.
Since these sysctls do not trigger any \(de\-\)allocations anymore, their effect
is instantaneous, whereas before it could have frozen the machine \(when trying
to allocate new vchans\) when setting `dev.pcm.X.{play|rec}.vchans` to a very
large value.
[960ee8094913](https://cgit.freebsd.org/src/commit/?id=960ee8094913).
\(Sponsored by The FreeBSD Foundation\)

The `hw.snd.version`
[sysctl\(8\)](https://man.freebsd.org/cgi/man.cgi?query=sysctl&sektion=8&format=html)
knob was removed.
[7398d1ece5cf](https://cgit.freebsd.org/src/commit/?id=7398d1ece5cf) \(Sponsored
by The FreeBSD Foundation\)

The `unit.*` code in
[sound\(4\)](https://man.freebsd.org/cgi/man.cgi?query=sound&sektion=4&format=html)
was retired, and as part of that the `hw.snd.maxunit`
[loader\(8\)](https://man.freebsd.org/cgi/man.cgi?query=loader&sektion=8&format=html)
tunable was removed.
[25723d66369f](https://cgit.freebsd.org/src/commit/?id=25723d66369f) \(Sponsored
by The FreeBSD Foundation\)

Gradual slowdowns and freezes experienced by owners of some AMD GPUs using the
amdgpu DRM driver from the `drm-kmod` ports, starting with v5.15
\(`graphics/drm-515-kmod` port\), have been fixed. In particular, owners of
graphics cards with Green Sardine, Polaris 10 and 20 and Vega chips were known
to be affected. Recent Intel\-based GPUs \(gen 13\+\) may also have been
affected. [718d1928f874](https://cgit.freebsd.org/src/commit/?id=718d1928f874),
[4ca9190251bb](https://cgit.freebsd.org/src/commit/?id=4ca9190251bb),
[986edb19a49c](https://cgit.freebsd.org/src/commit/?id=986edb19a49c),
[9d1f3ce79d85](https://cgit.freebsd.org/src/commit/?id=9d1f3ce79d85),
[da257e519bc0](https://cgit.freebsd.org/src/commit/?id=da257e519bc0) \(Sponsored
by The FreeBSD Foundation\)

The code iterating over memory domains \(NUMA\) was improved and fixed in a
number of ways, resulting in particular in decreased latency for some graphical
operations with DRM drivers.
[da257e519bc0](https://cgit.freebsd.org/src/commit/?id=da257e519bc0),
[83ad6d8d8eee](https://cgit.freebsd.org/src/commit/?id=83ad6d8d8eee),
[b15ff7214020](https://cgit.freebsd.org/src/commit/?id=b15ff7214020) \(Sponsored
by The FreeBSD Foundation\)

The effective group ID is now stored in the new `cr_gid` field of `struct cred`
and has been removed as the first element of `cr_groups[]`, which now only
contains the supplementary groups. All downstream and out\-of\-tree modules
using `cr_groups[0]` must be fixed to use `cr_gid` instead, and surrounding code
that loops on `cr_groups[]` elements excluding `cr_groups[0]`, i.e., that
intends to act on supplementary groups only, also needs to be adjusted as now
supplementary groups start at `&cr_groups[0]` instead of `&cr_groups[1]`. Code
that needs to be portable to both 15.0 and earlier versions can use `cr_gid`,
which existed also previously as a macro, and can test the truth value of
`&cr_groups[0] != &cr_gid` to know how to browse the supplementary groups
adequately.
[be1f7435ef21](https://cgit.freebsd.org/src/commit/?id=be1f7435ef218b1df35)
\(Sponsored by the FreeBSD Foundation\)

### Architecture\-Specific Changes

On amd64, FreeBSD now supports more than 4TB of RAM on modern machines that have
the LA57 CPU feature.
[d390633cf8cf](https://cgit.freebsd.org/src/commit/?id=d390633cf8cf) \(Sponsored
by the FreeBSD Foundation\)

On amd64, handling of the `%fsbase`/`%gsbase` registers and tls base were
reworked, making it more useful for apps that directly manipulate CPU context.
[68ba38dad3](https://cgit.freebsd.org/src/commit/?id=68ba38dad3) \(Sponsored by
the FreeBSD Foundation\)

## Devices and Drivers

This section covers changes and additions to devices and device drivers since
14.0\-RELEASE.

### Device Drivers

The
[tty\(4\)](https://man.freebsd.org/cgi/man.cgi?query=tty&sektion=4&format=html)
terminal interface now has the `IUTF8` flag, which enables proper UTF\-8
backspacing handling, set by default, suiting the default UTF\-8 locale.
[bb830e346bd5](https://cgit.freebsd.org/src/commit/?id=bb830e346bd5)

A driver is available for
[ice\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ice&sektion=4&format=html)
Ethernet network controllers in the Intel E800 series, which support 100 Gb/s
operation. It was upgraded to version 1.43.2\-k.
[38a1655adcb3](https://cgit.freebsd.org/src/commit/?id=38a1655adcb3) \(Sponsored
by Intel Corporation\)

Numerous stability improvements have been in the
[iwlwifi\(4\)](https://man.freebsd.org/cgi/man.cgi?query=iwlwifi&sektion=4&format=html)
driver for Intel Wi\-Fi devices. \(Sponsored by The FreeBSD Foundation\)

Multiple PCI MCFG regions are now supported on amd64, allowing PCI configuration
space access for domains \(segments\) other than 0.
[4b5f64408804](https://cgit.freebsd.org/src/commit/?id=4b5f64408804)

The
[smsc\(4\)](https://man.freebsd.org/cgi/man.cgi?query=smsc&sektion=4&format=html)
Ethernet driver can now fetch the value of `smsc95xx.macaddr` passed by some
Raspberry Pi models and use it for the MAC address. It always uses a stable MAC
address even if there is no address in EEPROM.
[028e4c6548e4](https://cgit.freebsd.org/src/commit/?id=028e4c6548e4)

The `snd_clone` framework has been removed from the sound subsystem, including
related sysctls, simplifying the system. The per\-channel nodes \(/dev/dspX.Y\)
are no longer created, just the primary device \(/dev/dspX\).
[e6c51f6db8d7](https://cgit.freebsd.org/src/commit/?id=e6c51f6db8d7) \(Sponsored
by The FreeBSD Foundation\)

Audio now supports asynchronous device detach. This greatly simplifies hot
plugging and unplugging of things such as USB headsets, and eases use of
PulseAudio in cases that require operating system sleep and wake \(suspend and
resume\). [d692c314d29a](https://cgit.freebsd.org/src/commit/?id=d692c314d29a)
\(Sponsored by The FreeBSD Foundation\)

`ice_ddp` has been upgraded to 1.3.41.0.
[a9d78bb714e3](https://cgit.freebsd.org/src/commit/?id=a9d78bb714e3) \(Sponsored
by Intel Corporation\)

Tiger Lake\-H support has been added to the
[hda\(4\)](https://man.freebsd.org/cgi/man.cgi?query=hda&sektion=4&format=html)
driver. [dbb6f488df6e](https://cgit.freebsd.org/src/commit/?id=dbb6f488df6e)

Meteor Lake support has been added to the
[ichsmb\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ichsmb&sektion=4&format=html)
driver. [14c22e28e4ee](https://cgit.freebsd.org/src/commit/?id=14c22e28e4ee)
\(Sponsored by Framework Computer Inc\) \(Sponsored by The FreeBSD Foundation\)

Meteor Lake support has been added to the
[ig4\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ig4&sektion=4&format=html)
driver. [56f0fc0011c2](https://cgit.freebsd.org/src/commit/?id=56f0fc0011c2)

Support for Realtek 8156/8156B has been moved from
[cdce\(4\)](https://man.freebsd.org/cgi/man.cgi?query=cdce&sektion=4&format=html)
to
[ure\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ure&sektion=4&format=html)
for improved performance and reliability.
[630077a84186](https://cgit.freebsd.org/src/commit/?id=630077a84186) \(Sponsored
by The FreeBSD Foundation\)

Support for ACPI GPIO \_AEI objects has been added.
[1db6ffb2a482](https://cgit.freebsd.org/src/commit/?id=1db6ffb2a482) \(Sponsored
by Amazon\)

[nvme\(4\)](https://man.freebsd.org/cgi/man.cgi?query=nvme&sektion=4&format=html)
and
[nvmecontrol\(8\)](https://man.freebsd.org/cgi/man.cgi?query=nvmecontrol&sektion=8&format=html)
have been enabled on all architectures.
[24687a65dd7f](https://cgit.freebsd.org/src/commit/?id=24687a65dd7f),
[aba2d7f89dcf](https://cgit.freebsd.org/src/commit/?id=aba2d7f89dcf) \(Sponsored
by Chelsio Communications and Netflix\)

[mpi3mr\(4\)](https://man.freebsd.org/cgi/man.cgi?query=mpi3mr&sektion=4&format=html)
driver version has been updated to 8.14.0.2.0.
[e6d4b221ba7c](https://cgit.freebsd.org/src/commit/?id=e6d4b221ba7c)

[mpi3mr\(4\)](https://man.freebsd.org/cgi/man.cgi?query=mpi3mr&sektion=4&format=html)
MPI Header has been updated to Version 36. This aligns with the latest MPI
specification. This includes updated structures, field definitions, and
constants required for compatibility with updated firmware.
[60cf1576501d](https://cgit.freebsd.org/src/commit/?id=60cf1576501d)

The
[mpi3mr\(4\)](https://man.freebsd.org/cgi/man.cgi?query=mpi3mr&sektion=4&format=html)
driver is now in GENERIC.
[e2b8fb2202c2](https://cgit.freebsd.org/src/commit/?id=e2b8fb2202c2)

[iwmbtfw\(4\)](https://man.freebsd.org/cgi/man.cgi?query=iwmbtfw&sektion=4&format=html):
Add support for 9260/9560 bluetooth adapters. Required firmware files are
already included in to
[comms/iwmbt\-firmware](https://cgit.freebsd.org/ports/tree/comms/iwmbt-firmware/)
port. [8e62ae9693bd](https://cgit.freebsd.org/src/commit/?id=8e62ae9693bd)

[ena\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ena&sektion=4&format=html)
driver version has been updated to v2.8.1.
[a1685d25601e](https://cgit.freebsd.org/src/commit/?id=a1685d25601e) \(Sponsored
by Amazon, Inc.\)

[bnxt\(4\)](https://man.freebsd.org/cgi/man.cgi?query=bnxt&sektion=4&format=html):
Enable NPAR support on BCM57504 10/25GbE NICs.
[54f842ed8897](https://cgit.freebsd.org/src/commit/?id=54f842ed8897)

[bnxt\(4\)](https://man.freebsd.org/cgi/man.cgi?query=bnxt&sektion=4&format=html):
Add 5760X \(Thor2\) PCI IDs support. Add Thor2 PCI IDs.
[45e161020c2d](https://cgit.freebsd.org/src/commit/?id=45e161020c2d)

[bnxt\(4\)](https://man.freebsd.org/cgi/man.cgi?query=bnxt&sektion=4&format=html):
Add support for 400G speed modules.
[32fdad17f060](https://cgit.freebsd.org/src/commit/?id=32fdad17f060)

[ix\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ix&sektion=4&format=html):
Add support for 1000BASE\-BX SFP modules. Add support for 1Gbit BiDi modules.
Add support for Intel Ethernet Network Adapter E610.
[89d4096950c4](https://cgit.freebsd.org/src/commit/?id=89d4096950c4)
[dea5f973d0c8](https://cgit.freebsd.org/src/commit/?id=dea5f973d0c8)

[igc\(4\)](https://man.freebsd.org/cgi/man.cgi?query=igc&sektion=4&format=html):
Fix attach for I226\-K and LMVP devices. The device IDs for these were in the
driver’s list of PCI ids to attach to, but `igc_set_mac_type()` had never been
setup to set the correct mac type for these devices. Fix this by adding these
IDs to the switch block in order for them to be recognized by the driver instead
of returning an error. This fixes the
[igc\(4\)](https://man.freebsd.org/cgi/man.cgi?query=igc&sektion=4&format=html)
attach for the I226\-K LOM on the ASRock Z790 PG\-ITX/TB4 motherboard, allowing
it to be recognized and used.
[f034ddd2fa38](https://cgit.freebsd.org/src/commit/?id=f034ddd2fa38).

Remove old itr sysctl handler from
[em\(4\)](https://man.freebsd.org/cgi/man.cgi?query=em&sektion=4&format=html).
This implementation had various bugs. The unit conversion/scaling was wrong, and
it also did not handle 82574L or
[igb\(4\)](https://man.freebsd.org/cgi/man.cgi?query=igb&sektion=4&format=html)
devices correctly. With the new AIM code, it is expected most users will not
need to manually tune this.
[edf50670e215](https://cgit.freebsd.org/src/commit/?id=edf50670e215) \(Sponsored
by BBOX.io\)

Added support for Brainboxes USB\-to\-Serial adapters in
[uftdi\(4\)](https://man.freebsd.org/cgi/man.cgi?query=uftdi&sektion=4&format=html).
[47db906375b5](https://cgit.freebsd.org/src/commit/?id=47db906375b5)

The
[iwx\(4\)](https://man.freebsd.org/cgi/man.cgi?query=iwx&sektion=4&format=html)
driver has been added, supporting the Intel Wi\-Fi 6 series of M.2 wireless
network adapters.
[2ad0f7e91582](https://cgit.freebsd.org/src/commit/?id=2ad0f7e91582) \(Sponsored
by The FreeBSD Foundation\)

A new cellular modem driver supports USB network devices implementing the Mobile
Broadband Interface Model \(MBIM\):
[umb\(4\)](https://man.freebsd.org/cgi/man.cgi?query=umb&sektion=4&format=html).
The accompanying
[umbctl\(8\)](https://man.freebsd.org/cgi/man.cgi?query=umbctl&sektion=8&format=html)
tool is used to display or set MBIM cellular modem interface parameters
\(4G/LTE\). [0f1bf1c22a0c](https://cgit.freebsd.org/src/commit/?id=0f1bf1c22a0c)
\(Sponsored by The FreeBSD Foundation\)

[smbios\(4\)](https://man.freebsd.org/cgi/man.cgi?query=smbios&sektion=4&format=html)
now searches for the SMBIOS v3 \(64\-bit\) entry point first also if booted from
BIOS. This allows to detect and report the proper SMBIOS version with BIOSes
that only provide the v3 table, as happens on Hetzner virtual machines. For
machines that provide both, leverage the v3 table in priority consistently with
the EFI case.
[bc7f6508363c](https://cgit.freebsd.org/src/commit/?id=bc7f6508363c) \(Sponsored
by The FreeBSD Foundation\)

The
[usbhid\(4\)](https://man.freebsd.org/cgi/man.cgi?query=usbhid&sektion=4&format=html)
driver is now enabled by default, and is used in preference to other USB HID
drivers like
[ukbd\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ukbd&sektion=4&format=html),
[ums\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ums&sektion=4&format=html),
and
[uhid\(4\)](https://man.freebsd.org/cgi/man.cgi?query=uhid&sektion=4&format=html).
Supported device classes now include:

- Absolute‐positioning mice in virtualized environments via
  [hms\(4\)](https://man.freebsd.org/cgi/man.cgi?query=hms&sektion=4&format=html)
- Digitizers and stylus devices via
  [hpen\(4\)](https://man.freebsd.org/cgi/man.cgi?query=hpen&sektion=4&format=html)
- Compound HID devices, such as keyboards and mice that share a single USB
  interface
- Special keyboard function keys \(volume, brightness, etc.\) via
  [hcons\(4\)](https://man.freebsd.org/cgi/man.cgi?query=hcons&sektion=4&format=html)
  and
  [hsctrl\(4\)](https://man.freebsd.org/cgi/man.cgi?query=hsctrl&sektion=4&format=html)
- Game controllers, including Xbox 360 and PS4 gamepads via
  [xb360gp\(4\)](https://man.freebsd.org/cgi/man.cgi?query=xb360gp&sektion=4&format=html)
  and
  [ps4dshock\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ps4dshock&sektion=4&format=html),
  and generic controllers via
  [hgame\(4\)](https://man.freebsd.org/cgi/man.cgi?query=hgame&sektion=4&format=html)
- Raw HID devices via
  [hidraw\(4\)](https://man.freebsd.org/cgi/man.cgi?query=hidraw&sektion=4&format=html)

FIDO/U2F security tokens continue to be supported through the autoloaded
[u2f\(4\)](https://man.freebsd.org/cgi/man.cgi?query=u2f&sektion=4&format=html)
driver. Device names and protocol handling for these devices are unchanged.
[74072e9f16c1](https://cgit.freebsd.org/src/commit/?id=74072e9f16c1) \(Sponsored
by The FreeBSD Foundation\)

The
[udbc\(4\)](https://man.freebsd.org/cgi/man.cgi?query=udbc&sektion=4&format=html)
driver has been added enabling host side debugging of targets using xHC debug.
[d566b6a70bcb](https://cgit.freebsd.org/src/commit/?id=d566b6a70bcb) \(Sponsored
by The FreeBSD Foundation\)

The
[ufshci\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ufshci&sektion=4&format=html)
driver has been added, supporting Universal Flash Storage \(UFS\) host
controllers.
[1349a733cf28](https://cgit.freebsd.org/src/commit/?id=1349a733cf28) \(Sponsored
by Samsung Electronics\)

The
[mlx5\(4\)](https://man.freebsd.org/cgi/man.cgi?query=mlx5&sektion=4&format=html)
driver now supports inline IPSEC offload on Nvidia ConnectX\-6\+ network cards,
leveraging the new in\-kernel IPSEC offload infrastructure.
[e23731db48ef](https://cgit.freebsd.org/src/commit/?id=e23731db48ef) \(Sponsored
by NVIDIA networking\)

Support for the watchdog timer in Intel 6300ESB I/O controller hub has been
included in the
[ichwd\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ichwd&sektion=4&format=html)
driver. This is intended primarily for QEMU users, where that watchdog timer
serves as the default and only one for x86 virtual machines.
[2b74ff5fceb6](https://cgit.freebsd.org/src/commit/?id=2b74ff5fceb6623f6)

The
[qat\(4\)](https://man.freebsd.org/cgi/man.cgi?query=qat&sektion=4&format=html)
driver has grown support for the 402xx device with ID 0x4944/0x4945.
[138e36514fe8](https://cgit.freebsd.org/src/commit/?id=138e36514fe8) \(Sponsored
by Intel Corporation\)

### Deprecated and Removed Drivers

The
[agp\(4\)](https://man.freebsd.org/cgi/man.cgi?query=agp&sektion=4&format=html)
bus driver has been deprecated and planned for removal in FreeBSD 16.0.
[92af7c97e197](https://cgit.freebsd.org/src/commit/?id=92af7c97e197)
[cadadd1a0398](https://cgit.freebsd.org/src/commit/?id=cadadd1a0398)

The IBM PC floppy disk controller,
[fdc\(4\)](https://man.freebsd.org/cgi/man.cgi?query=fdc&sektion=4&format=html),
and related utilities have been deprecated and planned for removal in FreeBSD
16.0. [4c736cfc69a7](https://cgit.freebsd.org/src/commit/?id=4c736cfc69a7)
\(Sponsored by The FreeBSD Foundation\)

The
[firewire\(4\)](https://man.freebsd.org/cgi/man.cgi?query=firewire&sektion=4&format=html)
bus and related drivers have been deprecated and planned for removal in FreeBSD
16.0. [fc889167c319](https://cgit.freebsd.org/src/commit/?id=fc889167c319)
\(Sponsored by The FreeBSD Foundation\)

The
[le\(4\)](https://man.freebsd.org/cgi/man.cgi?query=le&sektion=4&format=html)
Ethernet driver has been deprecated and planned for removal in FreeBSD 16.0.
[e4d6433e9c03](https://cgit.freebsd.org/src/commit/?id=e4d6433e9c03) \(Sponsored
by The FreeBSD Foundation\)

[syscons\(4\)](https://man.freebsd.org/cgi/man.cgi?query=syscons&sektion=4&format=html)
has been planned for removal in future releases, and has been noted as
deprecated in the manual pages to notify users to migrate to
[vt\(4\)](https://man.freebsd.org/cgi/man.cgi?query=vt&sektion=4&format=html).
[2bc5b1d60512](https://cgit.freebsd.org/src/commit/?id=2bc5b1d60512) \(Sponsored
by The FreeBSD Foundation\)

The
[upgt\(4\)](https://man.freebsd.org/cgi/man.cgi?query=upgt&sektion=4&format=html)
USB 802.11g driver has been deprecated and planned for removal in FreeBSD 16.0.
[7f8a5c5a1585](https://cgit.freebsd.org/src/commit/?id=7f8a5c5a1585) \(Sponsored
by The FreeBSD Foundation\)

## Storage

This section covers changes and additions to file systems and other storage
subsystems, both local and networked.

### NFS

The default value of the `nfs_reserved_port_only`
[rc.conf\(5\)](https://man.freebsd.org/cgi/man.cgi?query=rc.conf&sektion=5&format=html)
setting has changed. The FreeBSD NFS server now requires the source port of
requests to be in the privileged port range \(i.e., ≤ 1023\), which generally
requires the client to have elevated privileges on their local system. The
previous behavior can be restored by setting `nfs_reserved_port_only=NO` in
[rc.conf\(5\)](https://man.freebsd.org/cgi/man.cgi?query=rc.conf&sektion=5&format=html).
[6d5ce2bb6344](https://cgit.freebsd.org/src/commit/?id=6d5ce2bb6344) \(Sponsored
by The FreeBSD Foundation\)

Define a new `-a` command line option
[mountd\(8\)](https://man.freebsd.org/cgi/man.cgi?query=mountd&sektion=8&format=html)
that prevents exporting a file system with the `-alldirs` flag if the directory
path is not a server file system mount point.
[07cd69e272da](https://cgit.freebsd.org/src/commit/?id=07cd69e272da)

The layout of NFS file handles for the
[tarfs\(4\)](https://man.freebsd.org/cgi/man.cgi?query=tarfs&sektion=4&format=html),
[tmpfs\(4\)](https://man.freebsd.org/cgi/man.cgi?query=tmpfs&sektion=4&format=html),
[cd9660\(4\)](https://man.freebsd.org/cgi/man.cgi?query=cd9660&sektion=4&format=html),
and
[ext2fs\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ext2fs&sektion=4&format=html)
file systems has changed. An NFS server that exports any of these file systems
will need its clients to unmount and remount the exports.
[4db1b113b151](https://cgit.freebsd.org/src/commit/?id=4db1b113b151),
[1ccbdf561f41](https://cgit.freebsd.org/src/commit/?id=1ccbdf561f41),
[205659c43d87](https://cgit.freebsd.org/src/commit/?id=205659c43d87),
[cf0ede720391](https://cgit.freebsd.org/src/commit/?id=cf0ede720391),
[8ae6247aa966](https://cgit.freebsd.org/src/commit/?id=8ae6247aa966) \(Sponsored
by The FreeBSD Foundation\)

The
[mountd\(8\)](https://man.freebsd.org/cgi/man.cgi?query=mountd&sektion=8&format=html)
server has been modified to use
[strunvis\(3\)](https://man.freebsd.org/cgi/man.cgi?query=strunvis&sektion=3&format=html)
to decode directory names in
[exports\(5\)](https://man.freebsd.org/cgi/man.cgi?query=exports&sektion=5&format=html)
file\(s\). This allows special characters, such as blanks, to be embedded in the
directory name. `vis -M` may be used to encode such directory names; see
[vis\(1\)](https://man.freebsd.org/cgi/man.cgi?query=vis&sektion=1&format=html).
[2c83f1ada435](https://cgit.freebsd.org/src/commit/?id=2c83f1ada435)

New
[sysctl\(8\)](https://man.freebsd.org/cgi/man.cgi?query=sysctl&sektion=8&format=html)
variables have been added under `kern.rpc.unenc` and `kern.rpc.tls`, which allow
an NFS server administrator to determine how much NFS\-over\-TLS is being used.
A large number of failed handshakes might indicate an NFS configuration problem.
[b8e137d8d32d](https://cgit.freebsd.org/src/commit/?id=b8e137d8d32d)

The utilization of NFSv4.1/4.2 delegations was improved when the `nocto` mount
option is used. This requires an up\-to\-date NFSv4.1/4.2 server with
delegations enabled. For example, when building a FreeBSD kernel with both `src`
and `obj` NFSv4 mounted, the total RPC count drops from 5461286 to 945643, with
a 20% drop in elapsed time.
[171f66b0c2ca](https://cgit.freebsd.org/src/commit/?id=171f66b0c2ca),
[50e733f19b37](https://cgit.freebsd.org/src/commit/?id=50e733f19b37)

New support for the NFSv4.2 Clone operation, which uses block cloning to "copy
on write" files on an NFS server. This only works for exported ZFS file systems
that have block cloning enabled, at this time.
[cce64f2e6851](https://cgit.freebsd.org/src/commit/?id=cce64f2e6851)

### UFS

Soft updates are now enabled by default when creating a new UFS file system with
[newfs\(8\)](https://man.freebsd.org/cgi/man.cgi?query=newfs&sektion=8&format=html).
[6b2af2d88ffd](https://cgit.freebsd.org/src/commit/?id=6b2af2d88ffd)

Reliability of UFS on volumes with more than 2G of inodes is significantly
improved. The underlying issue was the invalid interpretation of the 32\-bit
inode number as signed, which got sign\-extended into `ino_t`.
[c069ca085bd1](https://cgit.freebsd.org/src/commit/?id=c069ca085bd1),
[e36f069ecb47](https://cgit.freebsd.org/src/commit/?id=e36f069ecb47) \(Sponsored
by The FreeBSD Foundation\)

Defer the January 19, 2038 date limit in UFS1 filesystems to February 7, 2106.
This affects only filesystems with old UFS1 format. See the commit message for
details. [1111a44301da](https://cgit.freebsd.org/src/commit/?id=1111a44301da)

### ZFS

Add support to `VOP_COPY_FILE_RANGE()` for block cloning. At this time, ZFS is
the only local file system that supports this and only if block cloning is
enabled. NFSv4.2 also supports it. See `pathconf(2)` and `copy_file_range(2)`
for more information.
[37b2cb5ecb0f](https://cgit.freebsd.org/src/commit/?id=37b2cb5ecb0f)

### GEOM

Support for vinum volumes has been removed.
[f87bb5967670](https://cgit.freebsd.org/src/commit/?id=f87bb5967670),
[e51036fbf3f8](https://cgit.freebsd.org/src/commit/?id=e51036fbf3f8)

### General Storage

Add Solaris style extended attributes \(called named attributes in NFSv4\). At
this time, only ZFS when the ZFS property called xattr=dir and NFSv4 support
them. The attributes are presented in a directory as regular files. See
named\_attribute\(7\) for more information.
[2ec2ba7e232d](https://cgit.freebsd.org/src/commit/?id=2ec2ba7e232d),
[df58e8b1506f](https://cgit.freebsd.org/src/commit/?id=df58e8b1506f),
[f61844833ee8](https://cgit.freebsd.org/src/commit/?id=f61844833ee8),
[b1b607bd200f](https://cgit.freebsd.org/src/commit/?id=b1b607bd200f),
[ee95e4d02dbd](https://cgit.freebsd.org/src/commit/?id=ee95e4d02dbd)

Allow to specify as many groups as configured to be supported by the system in
`-maproot` or `-mapall` options in
[exports\(5\)](https://man.freebsd.org/cgi/man.cgi?query=exports&sektion=5&format=html).
Previously, the cap was `NGROUPS_MAX + 1`, where `NGROUPS_MAX` is just the
minimum maximum of the number of allowed supplementary groups. Now use the
proper `{NGROUPS_MAX} + 1` value, with `{NGROUPS_MAX}` being fetched at runtime
via
[sysconf\(3\)](https://man.freebsd.org/cgi/man.cgi?query=sysconf&sektion=3&format=html).
[e87848a8150e](https://cgit.freebsd.org/src/commit/?id=e87848a8150e) \(Sponsored
by The FreeBSD Foundation\)

Add support for accessing remote NVMe over Fabrics controllers over the TCP
transport. New commands added to
[nvmecontrol\(8\)](https://man.freebsd.org/cgi/man.cgi?query=nvmecontrol&sektion=8&format=html)
are used to establish connections to remote controllers. Once connections are
established they are handed off to the
[nvmf\(4\)](https://man.freebsd.org/cgi/man.cgi?query=nvmf&sektion=4&format=html)
kernel module which creates `nvmeX` devices and exports remote namespaces as
[nda\(4\)](https://man.freebsd.org/cgi/man.cgi?query=nda&sektion=4&format=html)
disks. [a1eda74167b5](https://cgit.freebsd.org/src/commit/?id=a1eda74167b5),
[1058c12197ab](https://cgit.freebsd.org/src/commit/?id=1058c12197ab) \(Sponsored
by Chelsio Communications\)

Add support for exporting namespaces to remote NVMe over Fabrics hosts over the
TCP transport. The
[nvmft\(4\)](https://man.freebsd.org/cgi/man.cgi?query=nvmft&sektion=4&format=html)
kernel module adds a new frontend to the CAM target layer which exports
[ctl\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ctl&sektion=4&format=html)
LUNs as NVMe namespaces to remote hosts. The
[ctld\(8\)](https://man.freebsd.org/cgi/man.cgi?query=ctld&sektion=8&format=html)
daemon now supports NVMe controllers in addition to iSCSI targets and is
responsible for accepting incoming connection requests and handing off connected
queue pairs to
[nvmft\(4\)](https://man.freebsd.org/cgi/man.cgi?query=nvmft&sektion=4&format=html).
[a15f7c96a276](https://cgit.freebsd.org/src/commit/?id=a15f7c96a276),
[66b5296f1b29](https://cgit.freebsd.org/src/commit/?id=66b5296f1b29) \(Sponsored
by Chelsio Communications\)

Add support for dynamically resizing NVMe namespaces. The
[nvd\(4\)](https://man.freebsd.org/cgi/man.cgi?query=nvd&sektion=4&format=html)
and
[nda\(4\)](https://man.freebsd.org/cgi/man.cgi?query=nda&sektion=4&format=html)
drivers now notify geom of sizes changes in real time.
[86d3ec359a56](https://cgit.freebsd.org/src/commit/?id=86d3ec359a56) \(Sponsored
by Netflix\)

## Boot Loader Changes

This section covers the boot loader, boot menu, and other boot\-related changes.

The ASCII
[loader\(8\)](https://man.freebsd.org/cgi/man.cgi?query=loader&sektion=8&format=html)
art may once again be enabled on graphical systems via an optional `loader_gfx`
variable in
[loader.conf\(5\)](https://man.freebsd.org/cgi/man.cgi?query=loader.conf&sektion=5&format=html).
[bef6d85b6de5](https://cgit.freebsd.org/src/commit/?id=bef6d85b6de5)

The
[loader\(8\)](https://man.freebsd.org/cgi/man.cgi?query=loader&sektion=8&format=html)
now reads local configuration files listed in the variable
`local_loader_conf_files` after other configuration files, defaulting to
/boot/loader.conf.local.
[a25531db0fc2](https://cgit.freebsd.org/src/commit/?id=a25531db0fc2)

The
[loader\(8\)](https://man.freebsd.org/cgi/man.cgi?query=loader&sektion=8&format=html)
can now be configured to read specific configuration files based on the planar
maker, planar product, system product and uboot m\_product variables from the
SMBIOS. For the moment, the best documentation is the git commit message,
[3eb3a802a31b](https://cgit.freebsd.org/src/commit/?id=3eb3a802a31b).

Console detection in
[loader\(8\)](https://man.freebsd.org/cgi/man.cgi?query=loader&sektion=8&format=html)
has been improved on EFI systems. If there is no ConOut variable, ConIn is
checked. If multiple devices are found, serial is preferred.
[20a6f4779ac6](https://cgit.freebsd.org/src/commit/?id=20a6f4779ac6) \(Sponsored
by Netflix\)

Frame buffer support in
[loader\(8\)](https://man.freebsd.org/cgi/man.cgi?query=loader&sektion=8&format=html)
can now use a text\-only video driver, resulting in space savings.
[57ca2848c0aa](https://cgit.freebsd.org/src/commit/?id=57ca2848c0aa) \(Sponsored
by Netflix\)

The detection of ACPI is now done earlier in
[loader.efi\(8\)](https://man.freebsd.org/cgi/man.cgi?query=loader.efi&sektion=8&format=html)
on arm64 systems. The copy of loader.efi on the EFI partition should be updated
on arm64 systems using ACPI.
[05cf4dda599a](https://cgit.freebsd.org/src/commit/?id=05cf4dda599a)
[16c09de80135](https://cgit.freebsd.org/src/commit/?id=16c09de80135)

The LinuxBoot loader can be used to boot FreeBSD from Linux on aarch64 and
amd64. [46010641267](https://cgit.freebsd.org/src/commit/?id=46010641267)
\(Sponsored by Netflix\)

The BIOS boot loader added back support for gzip and bzip2, but removed support
for graphics mode \(by default\) to address size problems. \(The EFI boot loader
is unchanged with support for all of those.\)
[4d3b05a8530e](https://cgit.freebsd.org/src/commit/?id=4d3b05a8530e) \(Sponsored
by Netflix\)

The BIOS boot loader can now use the SMBIOS v3 \(64\-bit\) entry point if its
table is below 4GB. The BIOS boot loader is compiled 32\-bit as a client of BTX
even on amd64, so cannot access addresses beyond 4GB. However, the 64\-bit entry
point may refer to a structure table below 4GB, which can be used if the BIOS
does not provide a 32\-bit entry point, as happens on Hetzner virtual machines.
[7f005c6699f4](https://cgit.freebsd.org/src/commit/?id=7f005c6699f4) \(Sponsored
by The FreeBSD Foundation\)

The BIOS boot loader now favors the SMBIOS v3 \(64\-bit\) entry point. When both
the 32\-bit and 64\-bit entry points are present, the SMBIOS specification says
that the 64\-bit entry point always has at least all the structures the 32\-bit
entry point refers to. In other words, the 32\-bit entry point is provided for
compatibility, so it is assumed the 64\-bit one has more chances to be filled
with adequate values.
[3f744fb8b2c5](https://cgit.freebsd.org/src/commit/?id=3f744fb8b2c5) \(Sponsored
by The FreeBSD Foundation\)

The EFI boot loader now favors the SMBIOS v3 \(64\-bit\) entry point.
Consistently with what is done with BIOS boot. There is a difference though: As
the EFI loader runs in 64\-bit mode on 64\-bit platforms, there is no
restriction that the v3 entry point’s structure table should be below 4GB.
[96f77576e9ea](https://cgit.freebsd.org/src/commit/?id=96f77576e9ea) \(Sponsored
by The FreeBSD Foundation\)

## Networking

This section describes changes that affect networking in FreeBSD.

### General Network

FreeBSD now implements the `SO_SPLICE` interface, originally from OpenBSD. This
features allows userspace applications to splice two connected TCP sockets
together, after which data arriving on one socket is automatically forwarded
through the socket to which it is spliced, instead of being delivered to the
application.
[a1da7dc1cdad](https://cgit.freebsd.org/src/commit/?id=a1da7dc1cdad) \(Sponsored
by Klara, Inc.\) \(Sponsored by Stormshield\)

The
[ifconfig\(8\)](https://man.freebsd.org/cgi/man.cgi?query=ifconfig&sektion=8&format=html)
utility will no longer accept assigning IP addresses to the underlying member
interfaces of a
[bridge\(4\)](https://man.freebsd.org/cgi/man.cgi?query=bridge&sektion=4&format=html).
To temporarily bypass this safeguard, use the `net.link.bridge.member_ifaddrs`
[sysctl\(8\)](https://man.freebsd.org/cgi/man.cgi?query=sysctl&sektion=8&format=html).
This sysctl is expected to be removed in FreeBSD 16.
[b61850c4e6f6](https://cgit.freebsd.org/src/commit/?id=b61850c4e6f6)

ARP
\([arp\(4\)](https://man.freebsd.org/cgi/man.cgi?query=arp&sektion=4&format=html)\)
support for 802\-standard networks has been restored; it had been accidentally
removed with FDDI support. \(This is different than the Ethernet standard
encapsulation.\)
[d776dd5fbd48](https://cgit.freebsd.org/src/commit/?id=d776dd5fbd48)

It is possible to build a kernel with IPv6 support \(INET6\) without IPv4
\(INET\). [6df9fa1c6b83](https://cgit.freebsd.org/src/commit/?id=6df9fa1c6b83)
and others

The netgraph
[ng\_ipfw\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ng_ipfw&sektion=4&format=html)
module no longer truncates cookies to 16 bits, allowing a full 32 bits.
[dadf64c5586e](https://cgit.freebsd.org/src/commit/?id=dadf64c5586e)

AIM \(Adaptive Interrupt Moderation\) support has been added to the
[igc\(4\)](https://man.freebsd.org/cgi/man.cgi?query=igc&sektion=4&format=html)
driver. [472a0ccf847a](https://cgit.freebsd.org/src/commit/?id=472a0ccf847a)
\(Sponsored by Rubicon Communications, LLC \("Netgate"\) and BBOX.io\)

This feature has also been added to the
[lem\(4\)](https://man.freebsd.org/cgi/man.cgi?query=lem&sektion=4&format=html),
[em\(4\)](https://man.freebsd.org/cgi/man.cgi?query=em&sektion=4&format=html)
and
[igb\(4\)](https://man.freebsd.org/cgi/man.cgi?query=igb&sektion=4&format=html)
drivers. A major regression in UDP performance introduced in FreeBSD 12.0,
including NFS over UDP, is believed to be fixed with this change.
[49f12d5b38f6](https://cgit.freebsd.org/src/commit/?id=49f12d5b38f6) \(Sponsored
by Rubicon Communications, LLC \("Netgate"\) and BBOX.io\)

Teach
[ip6addrctl\(8\)](https://man.freebsd.org/cgi/man.cgi?query=ip6addrctl&sektion=8&format=html)
to attach and run itself in a jail. This will make it easier to manage address
selection policies of vnet jails, especially for those light weighted OCI
containers or slim jails.
[b709f7b38cc4](https://cgit.freebsd.org/src/commit/?id=b709f7b38cc4)

The
[pf\(4\)](https://man.freebsd.org/cgi/man.cgi?query=pf&sektion=4&format=html)
packet filter has learned a new runtime
[loader.conf\(5\)](https://man.freebsd.org/cgi/man.cgi?query=loader.conf&sektion=5&format=html)
tunable, 'net.pf.default\_to\_drop', as well as a compile time option,
`PF_DEFAULT_TO_DROP`, making the default rule to drop.
[7f7ef494f11d](https://cgit.freebsd.org/src/commit/?id=7f7ef494f11d),
[3965be101c43](https://cgit.freebsd.org/src/commit/?id=3965be101c43)

A new
[pf\(4\)](https://man.freebsd.org/cgi/man.cgi?query=pf&sektion=4&format=html)
route\-to pool option "prefer\-ipv6\-nexthop" allows for routing IPv4 packets
over IPv6 gateways.
[65c318630123](https://cgit.freebsd.org/src/commit/?id=65c318630123)
[d2761422eb0a](https://cgit.freebsd.org/src/commit/?id=d2761422eb0a) \(Sponsored
by InnoGames GmbH\)

[pf\(4\)](https://man.freebsd.org/cgi/man.cgi?query=pf&sektion=4&format=html)
now supports the OpenBSD style NAT syntax. It is possible to use "nat\-to",
"rdr\-to" and "binat\-to" on "pass" and "match" rules. The old "nat on …​"
syntax can still be used.
[e0fe26691fc9](https://cgit.freebsd.org/src/commit/?id=e0fe26691fc9) \(Sponsored
by InnoGames GmbH\)

The
[pfsync\(4\)](https://man.freebsd.org/cgi/man.cgi?query=pfsync&sektion=4&format=html)
protocol has been updated to synchronize multiple missing attributes. This fixes
synchronizing of states with route\-to, af\-to, rtable, dummynet, tags, and
scrub options. If synchronization with an older version of FreeBSD is needed the
protocol version can be configured with `ifconfig pfsync0 version $VERSION`
where $VERSION is 1301 for 13.X relases or 1400 for 14.X. It defaults to 1500
for synchronization between hosts running FreeBSD 15.0.
[99475087d63b](https://cgit.freebsd.org/src/commit/?id=99475087d63b) \(Sponsored
by InnoGames GmbH\)

Kernel TLS support is now enabled by default in `GENERIC` \(default\) kernels
for aarch64, amd64, powerpc64, and powerpc64le.
[b2f7c53430c3](https://cgit.freebsd.org/src/commit/?id=b2f7c53430c3) \(Sponsored
by Chelsio Communications\)

The `net.inet.{tcp,udp,raw}.bind_all_fibs` tunables have been added. They
default to 1 for backwards compatibility. Setting them to 0 modifies the
corresponding protocol’s socket behavior such that packets not originating from
an interface in the same FIB as the socket are ignored. In this case, TCP and
UDP sockets belonging to different FIBs may also be bound to the same address.
The default behavior is unmodified.
[5dc99e9bb985](https://cgit.freebsd.org/src/commit/?id=5dc99e9bb985),
[08e638c089ab](https://cgit.freebsd.org/src/commit/?id=08e638c089ab),
[4009a98fe80b](https://cgit.freebsd.org/src/commit/?id=4009a98fe80b) \(Sponsored
by Klara, Inc.\) \(Sponsored by Stormshield\)

Making a connection to `INADDR_ANY`, i.e., using it as an alias for `localhost`,
is now disabled by default. This functionality can be re\-enabled by setting the
`net.inet.ip.connect_inaddr_wild` sysctl to 1.
[cd240957d7ba](https://cgit.freebsd.org/src/commit/?id=cd240957d7ba) \(Sponsored
by The FreeBSD Foundation\)

New in\-kernel inline IPSEC offload infrastructure. See also the note about the
[mlx5\(4\)](https://man.freebsd.org/cgi/man.cgi?query=mlx5&sektion=4&format=html)
driver supporting it.
[ef2a572bf6](https://cgit.freebsd.org/src/commit/?id=ef2a572bf6) \(Sponsored by
NVIDIA networking\)

A new
[ngctl\(8\)](https://man.freebsd.org/cgi/man.cgi?query=ngctl&sektion=8&format=html)
flag, `-j`, allows it to attach and run inside a jail, making it possible to
manipulate netgraph nodes in a jail even if
[ngctl\(8\)](https://man.freebsd.org/cgi/man.cgi?query=ngctl&sektion=8&format=html)
is not installed inside it.
[72d01e62b082](https://cgit.freebsd.org/src/commit/?id=72d01e62b082)

[sockstat\(4\)](https://man.freebsd.org/cgi/man.cgi?query=sockstat&sektion=4&format=html)
will show UDP\-Lite endpoints by default.
[978615d7bf7c](https://cgit.freebsd.org/src/commit/?id=978615d7bf7c)

Kernel compatibility code supporting
[ipfw\(8\)](https://man.freebsd.org/cgi/man.cgi?query=ipfw&sektion=8&format=html)
binaries from FreeBSD 7 and 8 has been removed.
[660255be1ed9](https://cgit.freebsd.org/src/commit/?id=660255be1ed9) \(Sponsored
by The FreeBSD Foundation\)

### Network Protocols

Lots of improvements to the network stack, including performance improvements
and bug fixes for the
[sctp\(4\)](https://man.freebsd.org/cgi/man.cgi?query=sctp&sektion=4&format=html)
stack.

Descriptors returned by
[sctp\_peeloff\(2\)](https://man.freebsd.org/cgi/man.cgi?query=sctp_peeloff&sektion=2&format=html)
now inherit Capsicum capability
[rights\(4\)](https://man.freebsd.org/cgi/man.cgi?query=rights&sektion=4&format=html)
from the parent socket.
[ae3d7e27abc9](https://cgit.freebsd.org/src/commit/?id=ae3d7e27abc9) \(Sponsored
by The FreeBSD Foundation\)

The default value of the sysctl variable `net.inet.tcp.nolocaltimewait` has
changed from 1 to 0. This means that FreeBSD does not skip the `TIME_WAIT` state
anymore for endpoints for which the remote address is local. The new sysctl
variable `net.inet.tcp.msl_local` can be used to control the time these
endpoints stay in the `TIME_WAIT` state. The sysctl variable
`net.inet.tcp.nolocaltimewait` is deprecated and intended to be removed in
FreeBSD 16. [c3fc0db3bc50](https://cgit.freebsd.org/src/commit/?id=c3fc0db3bc50)
\(Sponsored by Netflix\)

The local stream \(AF\_UNIX/SOCK\_STREAM\) and sequenced packet stream
\(AF\_UNIX/SOCK\_SEQPACKET\) sockets have been improved for better bulk transfer
and round trip times. The SOCK\_SEQPACKET socket has been brought to the
specification and now behaves as a true stream socket, while in previous FreeBSD
releases it could exhibit features of a datagram socket. Applications that were
using SOCK\_SEQPACKET incorrectly and relied on old implementation bugs may need
to be adjusted.
[d15792780760](https://cgit.freebsd.org/src/commit/?id=d15792780760)

### Wireless Networking

The LinuxKPI 802.11 compatibility layer
[linuxkpi\_wlan\(4\)](https://man.freebsd.org/cgi/man.cgi?query=linuxkpi_wlan&sektion=4&format=html)
gained support for the Galois/Counter Mode Protocol \(GCMP\) from
[wlan\_gcmp\(4\)](https://man.freebsd.org/cgi/man.cgi?query=wlan_gcmp&sektion=4&format=html).
\(Sponsored by The FreeBSD Foundation\)

Following other drivers
[iwlwififw\(4\)](https://man.freebsd.org/cgi/man.cgi?query=iwlwififw&sektion=4&format=html)
firmware was removed from the base system in favor of the ports based solution
and
[fwget\(8\)](https://man.freebsd.org/cgi/man.cgi?query=fwget&sektion=8&format=html)
support. In case of updating from earlier releases, users must install the
firmware packages upfront. \(Sponsored by The FreeBSD Foundation\)

The
[iwlwifi\(4\)](https://man.freebsd.org/cgi/man.cgi?query=iwlwifi&sektion=4&format=html)
wireless driver supports 802.11ac \(VHT\) for some Intel Wi\-Fi 5, and all of
Intel Wi\-Fi 6 and Wi\-Fi 7 hardware. \(Sponsored by The FreeBSD Foundation\)
The
[iwx\(4\)](https://man.freebsd.org/cgi/man.cgi?query=iwx&sektion=4&format=html)
wireless driver supports 802.11ac \(VHT\) for Intel Wi\-Fi 6 hardware.
\(Sponsored by The FreeBSD Foundation\) The
[rtwn\(4\)](https://man.freebsd.org/cgi/man.cgi?query=rtwn&sektion=4&format=html)
wireless driver supports 802.11ac \(VHT\) for the RTL8812A and RTL8821A
chipsets. The
[rtw89\(4\)](https://man.freebsd.org/cgi/man.cgi?query=rtw89&sektion=4&format=html)
wireless driver supports 802.11g for some Realtek Wi\-Fi 6 and Wi\-Fi 7
hardware. [a2d1e07f6451](https://cgit.freebsd.org/src/commit/?id=a2d1e07f6451)
\(Sponsored by The FreeBSD Foundation\)

## Hardware Support

This section covers general hardware support for physical machines, hypervisors,
and virtualization environments, as well as hardware changes and updates that do
not otherwise fit in other sections of this document.

Please see [the list of
hardware](https://www.freebsd.org/releases/15.0R/hardware) supported by
15.0\-RELEASE, as well as [the platforms
page](https://www.freebsd.org/platforms/) for the complete list of supported CPU
architectures.

### Virtualization Support

[bhyve\(8\)](https://man.freebsd.org/cgi/man.cgi?query=bhyve&sektion=8&format=html)
and
[vmm\(4\)](https://man.freebsd.org/cgi/man.cgi?query=vmm&sektion=4&format=html)
now support the arm64 and riscv platforms. The `sysutils/u-boot-bhyve-arm64` and
`sysutils/u-boot-bhyve-riscv` ports provide boot loaders for use on these
platforms. [47e073941f4e](https://cgit.freebsd.org/src/commit/?id=47e073941f4e)
[d3916eace506](https://cgit.freebsd.org/src/commit/?id=d3916eace506) \(Sponsored
by Arm Ltd\) \(Sponsored by Innovate UK\) \(Sponsored by The FreeBSD
Foundation\) \(Sponsored by University Politehnica of Bucharest\)

[bhyve\(4\)](https://man.freebsd.org/cgi/man.cgi?query=bhyve&sektion=4&format=html)
now supports a "slirp" networking backend, which enables unprivileged user
networking. Currently only inbound connections to the guest are supported,
outbound connections from the guest are not. This feature requires the
`net/libslirp` port.
[c5359e2af5ab](https://cgit.freebsd.org/src/commit/?id=c5359e2af5ab) \(Sponsored
by Innovate UK\)

[bhyve\(4\)](https://man.freebsd.org/cgi/man.cgi?query=bhyve&sektion=4&format=html)
now may configure a NUMA topology for guest memory. Furthermore, it is possible
to define a
[domainset\(9\)](https://man.freebsd.org/cgi/man.cgi?query=domainset&sektion=9&format=html)
policy for each guest NUMA domain, wherein the host memory used to back the
guest physical memory of each guest NUMA domain can be specified, akin to
[cpuset\(1\)](https://man.freebsd.org/cgi/man.cgi?query=cpuset&sektion=1&format=html)'s
`-n` option. This is supported only for amd64 guests for now.
[f1d705d4f431](https://cgit.freebsd.org/src/commit/?id=f1d705d4f431)

The VNC server in
[bhyve\(8\)](https://man.freebsd.org/cgi/man.cgi?query=bhyve&sektion=8&format=html)
will now show the correct colors when using the
[www/novnc](https://cgit.freebsd.org/ports/tree/www/novnc/) client.
[f9e09dc5b1d5](https://cgit.freebsd.org/src/commit/?id=f9e09dc5b1d5)

When running
[bhyve\(8\)](https://man.freebsd.org/cgi/man.cgi?query=bhyve&sektion=8&format=html)
guests with a boot ROM, i.e., bhyveload\(8\) is not used, bhyve now assumes that
the boot ROM will enable PCI BAR decoding. This is incompatible with some boot
ROMs, particularly outdated builds of `edk2-bhyve`. To restore the old behavior,
add `pci.enable_bars='true'` to your bhyve configuration. Note that the
`uefi-edk2-bhyve` package has been renamed to `edk2-bhyve`.
[e962b37bf0ff](https://cgit.freebsd.org/src/commit/?id=e962b37bf0ff) \(Sponsored
by Innovate UK\)

amd64
[bhyve\(8\)](https://man.freebsd.org/cgi/man.cgi?query=bhyve&sektion=8&format=html)'s
`lpc.bootrom` and `lpc.bootvars` options are deprecated. Use the top\-level
`bootrom` and `bootvars` options instead.
[43caa2e805c2](https://cgit.freebsd.org/src/commit/?id=43caa2e805c2) \(Sponsored
by Innovate UK\)

The NVMM hypervisor is now detected.
[34f40baca641](https://cgit.freebsd.org/src/commit/?id=34f40baca641)

Under Hyper\-V, TLB flushes are now performed using hypercalls rather than IPIs,
providing up to a 40% improvement in TLB performance.
[7ece5993b787](https://cgit.freebsd.org/src/commit/?id=7ece5993b787) \(Sponsored
by Microsoft\)

### Linux Binary Compatibility

The `AT_NO_AUTOMOUNT` flag is now ignored for all Linuxulator stat\(\) variants
\(as the behavior specified by the flag already matches FreeBSD’s\), improving
Linux application compatibility.
[99d3ce80ba07](https://cgit.freebsd.org/src/commit/?id=99d3ce80ba07) \(Sponsored
by The FreeBSD Foundation\)

The Linux
[inotify\(2\)](https://man.freebsd.org/cgi/man.cgi?query=inotify&sektion=2&format=html)
system calls are now implemented in the Linuxulator. \(Sponsored by Klara,
Inc.\)

## Multimedia

Many improvements to the audio stack including support for hot\-swapping in
[mixer\(8\)](https://man.freebsd.org/cgi/man.cgi?query=mixer&sektion=8&format=html),
and the addition of
[mididump\(1\)](https://man.freebsd.org/cgi/man.cgi?query=mididump&sektion=1&format=html).
[cf9d2fb18433](https://cgit.freebsd.org/src/commit/?id=cf9d2fb18433) \(Sponsored
by The FreeBSD Foundation\)
[7224e9f2d4af](https://cgit.freebsd.org/src/commit/?id=7224e9f2d4af) \(Sponsored
by The FreeBSD Foundation\)

A new utility
[sndctl\(8\)](https://man.freebsd.org/cgi/man.cgi?query=sndctl&sektion=8&format=html)
has been added to concentrate the various interfaces for viewing and
manipulating audio device settings \(sysctls, `/dev/sndstat`\), into a single
utility with a similar control\-driven interface to that of `mixer(8)`.
[44e5a0150835](https://cgit.freebsd.org/src/commit/?id=44e5a0150835),
[9a37f1024ceb](https://cgit.freebsd.org/src/commit/?id=9a37f1024ceb) \(Sponsored
by The FreeBSD Foundation\)

`virtual_oss` is imported to base. The `audio/virtual_oss` port will stop being
built from FreeBSD 15.0 onwards. Regarding user\-facing changes, the only
practical difference is the installation process. Everything is provided by the
base system, except for the following optional components, which can be
installed from ports:

- sndio backend support: `audio/virtual_oss_sndio`
- bluetooth backend support: `audio/virtual_oss_bluetooth`
- `virtual_equalizer(8)`: `audio/virtual_oss_equalizer`

Apart from that, `virtual_oss` should work as expected. Users of `virtual_oss`
can uninstall `audio/virtual_oss` and instead use the base system version from
now on. [5a31c623143f](https://cgit.freebsd.org/src/commit/?id=5a31c623143f)
\(Sponsored by The FreeBSD Foundation\)

## Documentation

This section covers changes to manual
\([man\(1\)](https://man.freebsd.org/cgi/man.cgi?query=man&sektion=1&format=html)\)
pages and other documentation shipped with the base system.

### Manual Pages

A new
[freebsd\-base\(7\)](https://man.freebsd.org/cgi/man.cgi?query=freebsd-base&sektion=7&format=html)
manual provides details on the layout of base system packages and how to update
a system with them.
[e1632b827b1a](https://cgit.freebsd.org/src/commit/?id=e1632b827b1a)

Manual pages on filesystems have been moved to section four, the Kernel
Interfaces Manual.
[1687d77197c0](https://cgit.freebsd.org/src/commit/?id=1687d77197c0)

The
[builtin\(1\)](https://man.freebsd.org/cgi/man.cgi?query=builtin&sektion=1&format=html)
manual has been rewritten featuring streamlined information and a new section on
keybindings that are built into the FreeBSD CLI.
[42df4faf7004](https://cgit.freebsd.org/src/commit/?id=42df4faf7004)

A new
[networking\(7\)](https://man.freebsd.org/cgi/man.cgi?query=networking&sektion=7&format=html)
manual page provides a quickstart guide to connecting the system to networks
including Wi\-Fi, and links to other manual pages and the handbook.
[39f92a4c4c49](https://cgit.freebsd.org/src/commit/?id=39f92a4c4c49)

The
[build\(7\)](https://man.freebsd.org/cgi/man.cgi?query=build&sektion=7&format=html)
manual has been revised to incorporate instructions on building the system from
source. [275f61111f43](https://cgit.freebsd.org/src/commit/?id=275f61111f435)

Refer to
[graid\(8\)](https://man.freebsd.org/cgi/man.cgi?query=graid&sektion=8&format=html)
and
[zfs\(8\)](https://man.freebsd.org/cgi/man.cgi?query=zfs&sektion=8&format=html)
instead of
[gvinum\(8\)](https://man.freebsd.org/cgi/man.cgi?query=gvinum&sektion=8&format=html)
in
[ccdconfig\(8\)](https://man.freebsd.org/cgi/man.cgi?query=ccdconfig&sektion=8&format=html).
[55cb3a33d920](https://cgit.freebsd.org/src/commit/?id=55cb3a33d920)

The
[ps\(1\)](https://man.freebsd.org/cgi/man.cgi?query=ps&sektion=1&format=html)
manual page has been revamped to explain the general principles, and
descriptions in there have been updated to match reality. The preamble has been
revamped to give a thorough overview of the different aspects of the
[ps\(1\)](https://man.freebsd.org/cgi/man.cgi?query=ps&sektion=1&format=html)
command. The description of several options and some keywords have been fixed to
match their actual behavior and/or expanded. The STANDARDS and BUGS sections
have been expanded.
[ddf144a04b53](https://cgit.freebsd.org/src/commit/?id=ddf144a04b53) \(Sponsored
by The FreeBSD Foundation\)

The
[mac\_do\(4\)](https://man.freebsd.org/cgi/man.cgi?query=mac_do&sektion=4&format=html)
manual page has been revamped as part of adding support for multiple users and
groups as single rule’s targets, which lead to changing the rules syntax. In
particular, it has grown a JAIL SUPPORT and SECURITY CONSIDERATIONS sections.
[bc201841d139](https://cgit.freebsd.org/src/commit/?id=bc201841d139) \(Sponsored
by The FreeBSD Foundation\)

The existing content of the
[mdo\(1\)](https://man.freebsd.org/cgi/man.cgi?query=mdo&sektion=1&format=html)
manual page has been enriched as part of documenting the new support for fully
specifying all users and groups in the target credentials. It has now a longer
introduction and a new SECURITY CONSIDERATIONS section.
[20ebb6ec5ac0](https://cgit.freebsd.org/src/commit/?id=20ebb6ec5ac0) \(Sponsored
by The FreeBSD Foundation\) \(Sponsored by Google LLC \(GSoC 2025\)\)

The ethernet switch controllers,
[mtkswitch\(4\)](https://man.freebsd.org/cgi/man.cgi?query=mtkswitch&sektion=4&format=html),
[ip17x\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ip17x&sektion=4&format=html),
[ar40xx\(4\)](https://man.freebsd.org/cgi/man.cgi?query=ar40xx&sektion=4&format=html),
and
[e6000sw\(4\)](https://man.freebsd.org/cgi/man.cgi?query=e6000sw&sektion=4&format=html)
have gained initial manual pages.
[37f00bc257d](https://cgit.freebsd.org/src/commit/?id=37f00bc257d),
[f750a114d2c](https://cgit.freebsd.org/src/commit/?id=f750a114d2c),
[91c975c3913](https://cgit.freebsd.org/src/commit/?id=91c975c3913),
[6da793a8caa](https://cgit.freebsd.org/src/commit/?id=6da793a8caa)

[mount\(8\)](https://man.freebsd.org/cgi/man.cgi?query=mount&sektion=8&format=html)
has gained an example for remounting all filesystems read/write in single\-user
mode. [c3e06b23b417](https://cgit.freebsd.org/src/commit/?id=c3e06b23b417)

Manual pages for the lua
[loader\(8\)](https://man.freebsd.org/cgi/man.cgi?query=loader&sektion=8&format=html)
modules have had their descriptions reworded to optimize
[apropos\(1\)](https://man.freebsd.org/cgi/man.cgi?query=apropos&sektion=1&format=html)
results. [5d59c1b4f14e](https://cgit.freebsd.org/src/commit/?id=5d59c1b4f14e)

The manual pages style guide,
[style.mdoc\(5\)](https://man.freebsd.org/cgi/man.cgi?query=style.mdoc&sektion=5&format=html),
has gained a section for listing supported hardware. When listed this way, the
supported hardware will be listed in [the supported hardware
notes](https://www.freebsd.org/releases/15.0R/hardware). Many manuals have had
this section added or reworded in this release.

Much work has gone into adding
[sysctl\(8\)](https://man.freebsd.org/cgi/man.cgi?query=sysctl&sektion=8&format=html)s
and environment variables to the manual. Try searching for them with `apropos
Va=here.is.the.sysctl` or `apropos Ev=here_is_the_environment_variable`.

The
[intro\(1\)](https://man.freebsd.org/cgi/man.cgi?query=intro&sektion=1&format=html)
to the General Commands manual has been revised, incorporating a statement about
installing additional commands, and a listing of cannonical command directories.
[cc0af6d5a6c2](https://cgit.freebsd.org/src/commit/?id=cc0af6d5a6c2)

The
[intro\(2\)](https://man.freebsd.org/cgi/man.cgi?query=intro&sektion=2&format=html)
to the System Calls manual has been revised, incorporating links and a HISTORY
section from OpenBSD.
[9a62cdc01327](https://cgit.freebsd.org/src/commit/?id=9a62cdc01327),
[69ff2d754c1c](https://cgit.freebsd.org/src/commit/?id=69ff2d754c1c),
[6dfbe695c322](https://cgit.freebsd.org/src/commit/?id=6dfbe695c322),
[de525c502a3a](https://cgit.freebsd.org/src/commit/?id=de525c502a3a),
[d846f33bb6d4](https://cgit.freebsd.org/src/commit/?id=d846f33bb6d4),
[4696ca7baf2f](https://cgit.freebsd.org/src/commit/?id=4696ca7baf2f),
[9e8df7900f52](https://cgit.freebsd.org/src/commit/?id=9e8df7900f52),
[bcc57e971597](https://cgit.freebsd.org/src/commit/?id=bcc57e971597)

The
[intro\(5\)](https://man.freebsd.org/cgi/man.cgi?query=intro&sektion=5&format=html)
to the File Formats manual has been revised, incorporating improvements from
OpenBSD. [8d65152cbfc8](https://cgit.freebsd.org/src/commit/?id=8d65152cbfc8),
[26ec37653662](https://cgit.freebsd.org/src/commit/?id=26ec37653662),
[37508388d066](https://cgit.freebsd.org/src/commit/?id=37508388d066),
[a6175f28da70](https://cgit.freebsd.org/src/commit/?id=a6175f28da70)

The filesystem hierarchy index manual,
[hier\(7\)](https://man.freebsd.org/cgi/man.cgi?query=hier&sektion=7&format=html),
has been revised, incorporating a great deal of crossreferences, and increased
detail on `/usr/local`.

## Ports Collection and Package Infrastructure

This section covers changes to the FreeBSD Ports Collection, package
infrastructure, and package maintenance and installation tools.

A new `FreeBSD-kmods` repository is included in the default
`/etc/pkg/FreeBSD.conf`
[pkg\(8\)](https://man.freebsd.org/cgi/man.cgi?query=pkg&sektion=8&format=html)
configuration file. This repository contains kernel modules compiled
specifically for 15.0\-RELEASE rather than for the 15\-STABLE branch. Installing
kernel modules from this repository allows drivers with unstable kernel
interfaces, in particular graphics drivers, to work even when the main
15\-STABLE repository has packages build on a previous release.
[a47542f71511](https://cgit.freebsd.org/src/commit/?id=a47542f71511)

The `FreeBSD` and `FreeBSD-kmods` repositories defined in
`/etc/pkg/FreeBSD.conf` have been renamed to `FreeBSD-ports` and
`FreeBSD-ports-kmods` respectively. Users who override these in
`/usr/local/etc/pkg/repos` will need to adjust their configuration to match the
new names.

### Installer

The FreeBSD installer,
[bsdinstall\(8\)](https://man.freebsd.org/cgi/man.cgi?query=bsdinstall&sektion=8&format=html),
now supports downloading and installing firmware packages after the FreeBSD base
system installation is complete.
[03c07bdc8b31](https://cgit.freebsd.org/src/commit/?id=03c07bdc8b31) \(Sponsored
by The FreeBSD Foundation\)

### Packaging Changes

The bootonly ISO and mini\-memstick image now include the
[net/wifi\-firmware\-iwlwifi\-kmod](https://cgit.freebsd.org/ports/tree/net/wifi-firmware-iwlwifi-kmod/)
and
[net/wifi\-firmware\-rtw88\-kmod](https://cgit.freebsd.org/ports/tree/net/wifi-firmware-rtw88-kmod/)
packages, making installations possible over a wireless connection \(on systems
supported by these firmware packages\).
[655fcdde1aff](https://cgit.freebsd.org/src/commit/?id=655fcdde1aff) \(Sponsored
by The FreeBSD Foundation\)

The
[net/wifi\-firmware\-kmod@release](https://cgit.freebsd.org/ports/tree/net/wifi-firmware-kmod/)
package has been added to the DVD ISO, providing firmware for a broader set of
Wi\-Fi drivers.
[8c6df7ead19c](https://cgit.freebsd.org/src/commit/?id=8c6df7ead19c) \(Sponsored
by The FreeBSD Foundation\)

## General Notes Regarding Future FreeBSD Releases

**Last modified on**: December 1, 2025 by [Alexander
Ziaee](https://cgit.freebsd.org/doc/commit/?id=4105335197)

[Legal Notices](https://www.freebsd.org/copyright/) \| © 1995\-2025 The FreeBSD
Project All rights reserved. The mark FreeBSD is a registered trademark of The
FreeBSD Foundation and is used by The FreeBSD Project with the permission of
[The FreeBSD
Foundation](https://www.freebsdfoundation.org/legal/trademark-usage-terms-and-conditions/).
[Contact](https://www.freebsd.org/mailto/)
