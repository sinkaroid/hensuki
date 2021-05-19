#!/bin/bash
# 283616 for example and playground

#just color, remove this if not necessary
RED=$(tput setaf 1) 
GREEN=$(tput setaf 2) 
CYAN=$(tput setaf 6) 
WHITE=$(tput setaf 7) 

#awalan
begin=$(date +"%s")
echo
echo "${WHITE}nHentai stealer started!"

# get
echo -n "${WHITE}code: " # Makes echo and read inline
read -r kode 
echo -e "reading ${GREEN}/g/$kode...
${WHITE}"

# do
wget -q -nv -O ${kode}.html https://nhentai.net/g/${kode};
json="cat ${kode}.html"
jamet="cat links.txt"
var="$($json | grep -oP '(?<=<span class="pretty">)[^<]*' | head -1 | sed -z 's/\n//g' | sed 's/\//_/')" ## 1st Title is shown | title > sed fg/o > _ 
tag="$($json | grep -Po '(?<=<span class="name">)([^<]*)')" ## tags
count="$($json | grep -Po '(?<=<span class="name">)[0-9]([^<]*)')" ## count

# trash
hapus="rm ${kode}.html"
iki="rm links.txt dog.txt"
sniff="rm *.svg"

# proses
echo "${WHITE}Title:" ${GREEN}$var
echo "${WHITE}Tags:" ${RED}$tag
echo "${WHITE}Count:" ${CYAN}$count
echo ${WHITE}
mkdir -pv "../result/[nhentai.net] - ${var}" 
echo -e "Please wait! Processing ${GREEN}/g/$kode..."
${json} | grep img | grep -Po '<img src="\K.*?(?=")' | sed 's/\?.*//' | sed 's/t.nhentai/i.nhentai/' | sed 's/t.jpg/.jpg/' | sed 's/t.png/.png/' | sed 's/t.gif/.gif/' > "../result/[nhentai.net] - ${var}"/links.txt | ${hapus};
cd "../result/[nhentai.net] - ${var}";

# option -i $argv = menampilkan proses
wget -q -nv -i links.txt
${jamet} | sed 's/.*thumb.jpg//' | sed 's/.*cover.jpg//' | sed 's/.*.svg//' | sed -r '/^\s*$/d' > dog.txt
${sniff};
${iki};

# akhiran
termin=$(date +"%s")
difftimelps=$(($termin-$begin))
echo "${WHITE}$(($difftimelps / 60)) minutes and $(($difftimelps % 60)) seconds elapsed to finish ${GREEN}$var!"