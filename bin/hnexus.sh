#!/bin/bash
#sin

RED=$(tput setaf 1) 
GREEN=$(tput setaf 2) 
CYAN=$(tput setaf 6) 
WHITE=$(tput setaf 7) 
begin=$(date +"%s")
echo -e "${RED} "
echo "  Sin . (Hensuki [bash hentainexus downloader])"
echo "${WHITE}"
echo "parameter : ['view/code']"
echo "ie: https://hentainexus.com/view/7390"
echo "just type: ${CYAN}7390"
#get
echo "code:"
read -r kode
echo -e "reading /${GREEN}$kode ..
${WHITE}"

jamet="cat nex.html"

wget -q -nv -O nex.html https://hentainexus.com/view/${kode};
anjing=$(${jamet} | grep -oP '(?<=<h1 class="title">)[^<]*' | sed 's/::/,/g')
echo ${GREEN}${anjing}${WHITE}
mkdir -pv "${anjing}"
${jamet} | grep -Po '<img src="https:\K.*?(?=")' | sed 's/\\\/\\\//https:\/\//' | sed 's/\/\///g' | sed 's/?filter=null//g' > "${anjing}"/link.txt
rm nex.html;
cd "${anjing}";
wget -q -nv -i link.txt 
rm link.txt

#length
termin=$(date +"%s")
difftimelps=$(($termin-$begin))
echo "${WHITE}$(($difftimelps / 60)) minutes and $(($difftimelps % 60)) sec. elapsed ${RED}hentainexus bash"

# cat dog.txt | grep -Po '"https\K.*?(?=")' // link
# cat dog.txt | grep -Po '"https:\K.*?(?=")' | sed 's/\\/dog/' // char "\"
