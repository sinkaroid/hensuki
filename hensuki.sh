#!/bin/bash
#sin

RED=$(tput setaf 1) 
GREEN=$(tput setaf 2) 
CYAN=$(tput setaf 6) 
WHITE=$(tput setaf 7) 
begin=$(date +"%s")

echo -e "${RED}
  _                          _    _ 
 | |                        | |  (_)
 | |__   ___ _ __  ___ _   _| | ___ 
 | '_ \ / _ \ '_ \/ __| | | | |/ / |
 | | | |  __/ | | \__ \ |_| |   <| |
 |_| |_|\___|_| |_|___/\__,_|_|\_\_|"
echo " #[bash NSFW Doujinstealer] -sin#"
echo -e "${GREEN}"
PS3='select your platform or "4" to exit: '
options=("simply-hentai" "hentai2read" "nhentai" "Quit")
select opt in "${options[@]}"
do
    case $opt in
        "simply-hentai")
            cd bin;./hentai.sh
            ;;
        "hentai2read")
            cd bin;./h2r.sh
            ;;
        "nhentai")
            cd bin;./nhen.sh
            ;;
       
        "Quit")
            break
            ;;
        *) echo "invalid option $REPLY";;
    esac
done