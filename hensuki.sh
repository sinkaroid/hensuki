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
 |_| |_|\___|_| |_|___/\__,_|_|\_\_|
             [!] [bash NSFW Doujinstealer] 2019 -sin [!]"
echo -e "${GREEN}"

PS3='type your vendor number or "6" to exit: '
options=("simply-hentai" "hentai2read" "nhentai" "hentaicafe" "hentainexus" "Quit")
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
        "hentaicafe")
            cd bin;./hcafe.sh
            ;;
       "hentainexus")
            cd bin;./hnexus.sh
            ;;
        "Quit")
            break
            ;;
        *) echo "invalid option $REPLY";;
    esac
done