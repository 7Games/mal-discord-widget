# MyAnimeList Discord Widgert

## How to setup

LINUX ONLY, if you're on windows I recommend using WSL

### Making the widget

You can make the widget look however you like but make sure it has the following user data fields

#### Widget Top

| Item name | User data name | Type |
| --- | --- | --- |
| Image | avatar | media |
| Title | username | string |
| Subtitle 1 | joindate | string |
| Subtitle 2 | dayswatched | string |

#### Widget Bottom

| Item name | User data name | Type |
| --- | --- | --- |
| Stat #1 | watching | number |
| Stat #2 | plantowatch | number |
| Stat #3 | completed | number |
| Stat #4 | dropped | number |
| Stat #5 | onhold | number |
| Stat #6 | totalanime | number |

#### Mini Profile

You can set `Stat` to be whatever you want but I just chose `watching`

### .env file

Copy `.env.example` to `.env` and replace all the fields with what it asks

### Setup a venv

In the terminal navigate to this folder and type

```bash
python -m venv venv
```

### Installing packages

Activate the venv

```bash
chmod +x venv/bin/activate
. venv/bin/activate
```

Then install the required packages

```bash
pip install jikanpy-v4 dotenv
```

On some systems you may also need to install requests

```bash
pip install requests
```

### Test it

Then while still in the venv enter

```bash
python refresh_mal.py
```

Check your profile, if it's updated then the program works

To leave the venv type

```bash
deactivate
```

Next is to have it run every couple hours. I have modified a bash script called `updater.sh` from [discord-steam-profile-widget](https://github.com/xamionex/discord-steam-profile-widget) that'll run `refresh_mal.py` every 6 hours, on Linux to run it type

```bash
chmod +x updater.sh
./updater.sh & disown
```

however you will need to do this everytime your server launches so I recommend making a cronjob or a systemd service, but I can't be bothered making one for you so figure it our yourself :p
