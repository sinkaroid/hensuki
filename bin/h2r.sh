#!/bin/bash
#sin

RED=$(tput setaf 1) 
GREEN=$(tput setaf 2) 
CYAN=$(tput setaf 6) 
WHITE=$(tput setaf 7) 
begin=$(date +"%s")
echo -e "${RED} "
echo "  Sin . (Hensuki [bash hentai2read.com downloader])"
echo "${WHITE}"
echo "usage : ['series']/['chapter']"
echo "ie: https://hentai2read.com/lewd_inuyamasenpai/1/"
echo "just type: ${CYAN}lewd_inuyamasenpai/1/"
#get
echo "code:"
read -r kode
echo -e "reading /${GREEN}$kode ..
${WHITE}"

#fundamen
json="cat meki.html"


wget -O meki.html https://hentai2read.com/${kode};

#do
mkdir -pv "${kode}" 
echo -e "still doing for /${GREEN}$kode ...
${WHITE}"
${json} | grep "'images'" | sed 's/","/\nstatic.hentaicdn.com\/hentai/g' | sed "s/\\\//g" | sed 's/"],//g' > ${kode}/asu.txt #dog
cd ${kode};
wget -q -nv -i asu.txt # --user-agent="Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36"
# option -i $argv if necessary show dat process
cd $OLDPWD
rm meki.html;

#length
termin=$(date +"%s")
difftimelps=$(($termin-$begin))
echo "${WHITE}$(($difftimelps / 60)) minutes and $(($difftimelps % 60)) sec. elapsed ${RED}hentai2read stealer"