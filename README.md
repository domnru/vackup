# vackup

[![Build Status](https://img.shields.io/badge/build-unkown-red)](https://github.com/domnru/cnitch)

vackup (Volume Backup)  is an automated solution for compressing and encrypting container volumes and any specified directories. By mounting volumes from other containers to the TOOLNAME `/volumes` directory, the container will compress and encrypt the data using the [sevenz-rust crate]("https://crates.io/crates/sevenz-rust") and store the resulting archives in the `/archives` directory. Subsequently, tools like rclone can be used to securely back up these 7z archives to cloud storage, safeguarding your homelab volumes.


## How It Works

1. **Mapping volumes:** You specify which volumes from other containers should map into the `/volumes` direcotry of the vackup container.
2. **Automated zipping and compressing:** By providing the environment variables specified in the `compose.yaml` you can automate the whole process.
3. **Backup the archives:** Simply use a tool like [rclone]("https://rclone.org/") to automate the process of storing the encrypted archives. All archives are stored in the `/archives` directory. Simply backup these on any methods you would like to use.
4. **Logging:** All processes are logged via rusts `println` macro inside the container.
