#!/bin/bash
#sin

RED=$(tput setaf 1) 
GREEN=$(tput setaf 2) 
CYAN=$(tput setaf 6) 
WHITE=$(tput setaf 7) 
begin=$(date +"%s")
echo -e "${RED} "
echo "  Sin . (Hensuki [bash pUrurin downloader])"
echo "${WHITE}"
echo "parameter : ['code/params']"
echo "ie: https://pururin.io/gallery/43639/naganami-sama-no-yowaitoko"
echo "just type: ${CYAN}43639/naganami-sama-no-yowaitoko"
#get
echo "code:"
read -r kode
echo -e "reading /${GREEN}$kode ..
${WHITE}"
kuproy="cat link.txt"
jamet="cat pu.html"

wget -q -nv -O pu.html https://pururin.io/gallery/${kode};
anjing=$(${jamet} | grep -oP '(?<=<title>)[^/*\/]*' | sed 's/ $//' |  sed 's/\.//') #title
count=$(${jamet} | grep -oP '(?<=total=")[^"]*') #counts for loop


echo ${GREEN}${anjing}${WHITE}
echo ${count}


mkdir -pv "${anjing}"
${jamet} | grep -oP '(?<=image" content=")[^"]*' > "${anjing}"/link.txt
rm pu.html
cd "${anjing}";


babi=$(${kuproy} | sed 's/./https:\//' | sed 's/cover//' | sed 's/\/\.jpg//')
echo ${RED}"first match => "${GREEN}${babi}
png='.png'
jpg='.jpg'
for i in $( eval "echo {1..$count}" )
do
  echo "$babi/$i$jpg" 
done > hey.txt
wget -q -nv -i hey.txt
rm hey.txt


termin=$(date +"%s")
difftimelps=$(($termin-$begin))
echo "${WHITE}$(($difftimelps / 60)) minutes and $(($difftimelps % 60)) sec. elapsed ${RED}pUrurin bash"