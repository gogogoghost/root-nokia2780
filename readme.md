## Root nokia 2780

This repo provide a way to root nokia 2780

### Requirements

- Follow [weeknd-toolbox](https://git.abscue.de/affe_null/weeknd-toolbox/) to make system editable. And enable adb
- ndk-build

### Get started

Reboot to recovery and mount the whole emmc to your PC

### Replace kernel

Patched boot partition has been replaced the kernel cmdline from **androidboot.selinux=enforcing** to **androidboot.selinux=permissive**

```bash
sudo dd if=patched/boot of=/dev/sda13
```

### Replace init

Eject the whole emmc and mount system partion to your PC

Patched init has been edit to disable selinux

```bash
sudo cp patched/init $(systemRoot)/system/bin/init
```

### build su-daemon/su

```bash
ndk-build
```

Then copy the su and su-daemon to **$(systemRoot)/system/xbin**

```bash
sudo cp libs/armeabi-v7a/su $(systemRoot)/system/xbin/su
sudo cp libs/armeabi-v7a/su-daemon $(systemRoot)/system/xbin/su-daemon
```

### Edit init.rc

Add su-daemon service to **/init.rc**

```text
service su-daemon /system/xbin/su-daemon
    class core
    user root
    group root
    oneshot
    socket su-daemon seqpacket 666 root root
    seclabel u:r:su:s0
```

Reboot your device. Then you can execute su to get access root

### Known issues

- su will print some error message first
- su cannot exit normally. you should use Ctrl+C
