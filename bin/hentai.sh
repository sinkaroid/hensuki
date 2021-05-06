#!/bin/bash
# fullpath all-pages pls eg: original-work/kawaisou-na-ko-shiawase-na-ko/all-pages
# for https://www.simply-hentai.com/original-work/kawaisou-na-ko-shiawase-na-ko
# non alphabet(cn,jp) url still unstable

RED=$(tput setaf 1) 
GREEN=$(tput setaf 2) 
CYAN=$(tput setaf 6) 
WHITE=$(tput setaf 7) 
begin=$(date +"%s")
echo -e "${RED} "
echo "  Sin . (Hensuki [bash simply-hentai.com downloader])"
echo "${GREEN}"
echo "usage : ['artwork']/['series']/[all-pages]"
echo "${WHITE}"
echo "ie: https://www.simply-hentai.com/original-work/kawaisou-na-ko-shiawase-na-ko"
echo "just type: ${CYAN}original-work/kawaisou-na-ko-shiawase-na-ko/all-pages"

# get
echo "code:"
read -r kode
echo -e "reading /${GREEN}$kode ..
${WHITE}"
meki="cat meki.html"
wget -O meki.html https://www.simply-hentai.com/${kode};

# do
mkdir -pv "../result/[simply-hentai.com] - ${kode}" 
echo -e "Processing, pls wait /${GREEN}$kode ..
${WHITE}"
${meki} | grep -Po 'full":"\K[^"]+' > "../result/[simply-hentai.com] - ${kode}"/asu.txt
cd "../result/[simply-hentai.com] - ${kode}";
wget -q -nv -i asu.txt # --user-agent="Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36"
rm asu.txt
# option -i $argv if necessary show dat process
cd $OLDPWD
rm meki.html;

# akhiran
termin=$(date +"%s")
difftimelps=$(($termin-$begin))
echo "${WHITE}$(($difftimelps / 60)) minutes and $(($difftimelps % 60)) sec. elapsed ${RED}simplynetwork stealer"