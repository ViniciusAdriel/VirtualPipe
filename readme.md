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
* Click "Create Pipe" to add a new stereo pipe.  
* That’s it! Your stereo pipe has been created.  
Your virtual speaker will appear among your speakers or headphones.  
Your virtual microphone will appear among your microphones.  
* To make changes, click on the pipe in the list to edit its name or channel type.  



> [!WARNING]
> Virtual devices are temporary and will be removed after reboot.
