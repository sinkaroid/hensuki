#!/bin/bash
#sin

#view
RED=$(tput setaf 1) 
GREEN=$(tput setaf 2) 
CYAN=$(tput setaf 6) 
WHITE=$(tput setaf 7) 


begin=$(date +"%s")

echo "![Nhentai stealer]! - with mirror"

#get
echo "code:"
read -r kode
echo -e "reading /${GREEN}$kode ..
${WHITE}"

wget -q -nv -O ${kode}.html https://hensuki.sinxdr.workers.dev/g/${kode};
json="cat ${kode}.html"
jamet="cat links.txt"

var="$($json | grep -oP '(?<=<span class="pretty">)[^<]*' | sed -z 's/\n//g' | sed 's/\//_/')" ## title > sed fg/o > _
tag="$($json | grep -Po '(?<=<span class="name">)([^<]*)')" ## tags
count="$($json | grep -Po '(?<=<span class="name">)[0-9]([^<]*)')" ## count

hapus="rm ${kode}.html"
iki="rm links.txt dog.txt"

echo "Title:" ${GREEN}$var
echo "${WHITE}Tags:" ${RED}$tag
echo "${WHITE}Count:" ${CYAN}$count


echo ${WHITE}
mkdir -pv "${var:0:225}" 
echo -e "still doing /${GREEN}$kode ..."

#do

${json} | grep img | grep -Po '<img src="\K.*?(?=")' | sed 's/\?.*//' | sed 's/t.nhentai.net/hell.sinxdr.workers.dev/' | sed 's/t.jpg/.jpg/' | sed 's/t.png/.png/' | sed 's/.*.svg//' > "${var:0:225}"/links.txt | ${hapus};
cd "${var:0:225}";
wget -q -nv -i links.txt # option -i $argv if necessary show dat process
${jamet} | sed 's/.*thumb.jpg//' | sed 's/.*cover.jpg//' | sed 's/.*.svg//' | sed -r '/^\s*$/d' > dog.txt

ParDir="$(cd ../ && pwd)/galer.sh"

source $ParDir
${iki};

#length
termin=$(date +"%s")
difftimelps=$(($termin-$begin))
echo "${WHITE}$(($difftimelps / 60)) minutes and $(($difftimelps % 60)) sec. elapsed ${RED}nhentai stealer"

#283616
