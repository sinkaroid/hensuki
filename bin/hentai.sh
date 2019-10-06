#!/bin/bash
#sin

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
#get
read -p "${GREEN}PATH: " kode
echo -e "\n"
read -p "${WHITE}${kode} Continue? (Y/N): " confirm && [[ $confirm == [yY] || $confirm == [yY][eE][sS] ]] || exit 1 
echo -e "reading source for /${GREEN}$kode ..
${WHITE}"

wget -O meki.html https://www.simply-hentai.com/${kode};

#do
mkdir -pv "${kode}" 
echo -e "wait pls, try to get /${GREEN}$kode ..
${WHITE}"
cat meki.html| grep -Po 'full":"\K[^"]+' > ${kode}/asu.txt #dog
cd ${kode};
wget -q -nv -i asu.txt # --user-agent="Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36"
mv * ..
# option -i $argv if necessary show dat process
cd -
rm meki.html;

#length
termin=$(date +"%s")
difftimelps=$(($termin-$begin))
echo "${WHITE}$(($difftimelps / 60)) minutes and $(($difftimelps % 60)) sec. elapsed ${RED}simplynetwork stealer"