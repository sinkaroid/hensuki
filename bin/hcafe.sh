#!/bin/bash
#sin

RED=$(tput setaf 1) 
GREEN=$(tput setaf 2) 
CYAN=$(tput setaf 6) 
WHITE=$(tput setaf 7) 
begin=$(date +"%s")
echo -e "${RED} "
echo "  Sin . (Hensuki [bash hcafe downloader])"
echo "${WHITE}"
echo "usage : ['code']"
echo "ie: https://hentai.cafe/hc.fyi/14287"
echo "just type: ${CYAN}14287"
#get
echo "code:"
read -r kode
echo -e "reading /${GREEN}$kode ..
${WHITE}"

link="cat link.txt"
meki="cat meki.html"

wget -O meki.html https://hentai.cafe/hc.fyi/${kode};
anjing=$(${meki} | grep -Po 'href="\K.*?(?=" title="Read")')
echo ${GREEN}${anjing}${WHITE}
title="$(${meki} | grep -oP '(?<=<h3>)[^<]*')"
rm meki.html;

mkdir -pv "${title}" 
echo -e "still doing for /${GREEN}$kode ...
${WHITE}"
cd "${title}";
wget -O link.txt ${anjing}
${link} | grep thumb_url | grep -Po 'thumb_url":"\K.*?(?=")' | sed 's:\\\/:/:g' > asu.txt #dog

wget -q -nv -i asu.txt # --user-agent="Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36"
# option -i $argv if necessary show dat process
rm asu.txt link.txt


#length
termin=$(date +"%s")
difftimelps=$(($termin-$begin))
echo "${WHITE}$(($difftimelps / 60)) minutes and $(($difftimelps % 60)) sec. elapsed ${RED}hcafe stealer"