#!/bin/bash
# pls full path eg: 43639/naganami-sama-no-yowaitoko

#just color, remove this if not necessary
RED=$(tput setaf 1) 
GREEN=$(tput setaf 2) 
CYAN=$(tput setaf 6) 
WHITE=$(tput setaf 7) 

#awalan
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

# trash
kuproy="cat link.txt"
jamet="cat pu.html"

# do
wget -q -nv -O pu.html https://pururin.io/gallery/${kode};
anjing=$(${jamet} | grep -oP '(?<=<title>)[^/*\/]*' | sed 's/ $//' |  sed 's/\.//')
count=$(${jamet} | grep -oP '(?<=total=")[^"]*')
echo ${GREEN}${anjing}${WHITE}
echo ${count}

# proses
mkdir -pv "../result/[pururin.io] - ${anjing}"
${jamet} | grep -oP '(?<=image" content=")[^"]*' > "../result/[pururin.io] - ${anjing}"/link.txt
rm pu.html
cd "../result/[pururin.io] - ${anjing}";

babi=$(${kuproy} | sed 's/./https:\//' | sed 's/cover//' | sed 's/\/\.jpg//')
echo ${RED}"match => "${GREEN}${babi}
png='.png'
jpg='.jpg'
for i in $( eval "echo {1..$count}" )
do
  echo "$babi/$i$jpg" 
done > hey.txt

# option -i $argv = menampilkan proses
wget -q -nv -i hey.txt
rm hey.txt

# akhiran
termin=$(date +"%s")
difftimelps=$(($termin-$begin))
echo "${WHITE}$(($difftimelps / 60)) minutes and $(($difftimelps % 60)) sec. elapsed ${RED}pUrurin bash"