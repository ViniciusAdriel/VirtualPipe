# VirtualPipe
**A GUI utility to create and manage virtual audio cables in PipeWire.**  
  
> [!IMPORTANT]
> **PIPEWIRE ONLY**  
> Requires **`pw-link`** and **`pactl`**.  
> You can check if they are installed with:
> ```bash
> which pw-link
> which pactl
> ```
> If no path is returned, install them with the following packages:  
>  
> **Debian/Ubuntu**
> ```bash
> apt install pulseaudio-utils # provides pactl
> apt install pipewire         # provides pw-link
> ```
> **Fedora**
> ```bash
> dnf install pulseaudio-utils # provides pactl
> dnf install pipewire-utils   # provides pw-link
> ```
> **Arch**
> ```bash
> dnf install libpulse # provides pactl
> dnf install pipewire # provides pw-link
> ```
> I haven't tested in other distros :p

  
## What it does
Create and remove virtual audio devices — a virtual input paired with a virtual output.

## Why use it
Capture your microphone's audio in OBS, apply processing (like noise filters), monitor audio, and route it to a virtual output.
For apps without built-in noise suppression, you can feed the processed audio through a virtual input.

## How to use it
* In the first text field, enter the name of your virtual output.
* In the second text field, enter the name of your virtual input.
* Choose between stereo and mono in the drop-down menu.
* Click Create.  
  
**Done!**

Your virtual output will appear among your speakers or headphones.  
Your virtual input will appear among your microphones.  

> [!WARNING]
> Virtual devices are temporary and will be removed after reboot.
