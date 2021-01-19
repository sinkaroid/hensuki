#!/bin/bash
#sin

#view
RED=$(tput setaf 1) 
GREEN=$(tput setaf 2) 
CYAN=$(tput setaf 6) 
WHITE=$(tput setaf 7) 


begin=$(date +"%s")


echo "![Nhentai stealer]!"

#get
echo "code:"
read -r kode
echo -e "reading /${GREEN}$kode ..
${WHITE}"

wget -q -nv -O ${kode}.html https://nhentai.net/g/${kode};
json="cat ${kode}.html"
jamet="cat links.txt"

var="$($json | grep -oP '(?<=<title>)[^<]*' | sed -z 's/\n//g' | sed 's/&raquo;/-/')" ## title > sed fg/o > _
tag="$($json | grep -Po '(?<=<span class="name">)([^<]*)')" ## tags
count="$($json | grep -Po '(?<=<span class="name">)[0-9]([^<]*)')" ## count


hapus="rm ${kode}.html"
iki="rm links.txt dog.txt"
sniff="rm *.svg"

echo "Title:" ${GREEN}$var
echo "${WHITE}Tags:" ${RED}$tag
echo "${WHITE}Count:" ${CYAN}$count


echo ${WHITE}
mkdir -pv "${var:0:225}" 
echo -e "still doing /${GREEN}$kode ..."

#do

${json} | grep img | grep -Po '<img src="\K.*?(?=")' | sed 's/\?.*//' | sed 's/t.nhentai/i.nhentai/' | sed 's/t.jpg/.jpg/' | sed 's/t.png/.png/' > "${var}"/links.txt | ${hapus};
cd "${var}";
wget -q -nv -i links.txt # option -i $argv if necessary show dat process
${jamet} | sed 's/.*thumb.jpg//' | sed 's/.*cover.jpg//' | sed 's/.*.svg//' | sed -r '/^\s*$/d' > dog.txt
##for img in *.png; do
   ## filename=${img%.*}
    ##convert "$filename.png" "$filename.jpg" && rm "$img"; ## hell
##done
ParDir="$(cd ../ && pwd)/galer.sh"

source $ParDir
${sniff};
${iki};



#length
termin=$(date +"%s")
difftimelps=$(($termin-$begin))
echo "${WHITE}$(($difftimelps / 60)) minutes and $(($difftimelps % 60)) sec. elapsed ${RED}nhentai stealer"

#283616
