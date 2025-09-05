# p5rcbt
A plugin for loading loose files instead of repacking a CPK, powered by [Skyline](https://github.com/skyline-dev/skyline).

## Features
This comes built-in with the following features:

* File logger (use the [following executable](https://github.com/Coolsonickirby/skyline-logger-files/releases/download/1.0.0/skyline_logger_rust.exe) for logging. Use 127.0.0.1 as the IP if you plan to mod on Ryujinx)
* Bind directories and CPKs found in ``sd:/p5r/bind`` alphanumerically. Bind directory files prevail over the ones in CPKs.
* 60 FPS patch
* PC settings menu
* Configuration file
* Patch PUT, PUTS and PUTF script methods (@DeathChaos25)

> [!TIP]
> Make your mods using bind directories but distribute mods in CPK format to keep the game running at a good speed.

> [!WARNING]
> Bind directories do **NOT** use the ``PATCH1, BASE, SOUND, MOVIE`` found in CPKs.

## Configuration
Goes in ``sd:/p5r/cfg/cbt.toml`` and defaults to the following:
```toml
logging = false
uncap_framerate = true
pc_settings = false
```

## Downloads 

Head to the [release](https://github.com/Raytwo/p5rcbt/releases/latest) page to get the latest build!
