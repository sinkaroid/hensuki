#!/bin/bash
#sin

#view
RED=$(tput setaf 1) 
GREEN=$(tput setaf 2) 
CYAN=$(tput setaf 6) 
WHITE=$(tput setaf 7) 


begin=$(date +"%s")

echo -e "${RED}
        _                _        _       _     
       | |              | |      (_)     | |    
  _ __ | |__   ___ _ __ | |_ __ _ _   ___| |__  
 | '_ \| '_ \ / _ \ '_ \| __/ _' | | / __| '_ \ 
 | | | | | | |  __/ | | | || (_| | |_\__ \ | | |
 |_| |_|_| |_|\___|_| |_|\__\__._|_(_)___/_| |_|                                                                                                                                          
"
echo "![Nhentai stealer]!"

#get
echo "code:"
read -r kode
echo -e "reading /${GREEN}$kode ..
${WHITE}"

wget -q -nv -O ${kode}.html https://nhentai.net/g/${kode};
json="cat ${kode}.html"


var="$($json | grep -oP '(?<=<h1>)[^<]*')"
tag="$($json | grep -oP '(?<= ">)[^<]*' | sed 's/ //' | sed 's/$/\,/')"
count="$($json | grep -oP '(?<=<div>)[^<]*' | sed 's/Uploaded//')"

echo "Title:" ${GREEN}$var
echo "${WHITE}Tags:" ${RED}$tag
echo "${WHITE}Count:" ${CYAN}$count
echo ${WHITE}
mkdir -pv "${var}" 
echo -e "still doing /${GREEN}$kode ..."

#do

cat ${kode}.html | grep img | grep -Po '<img src="\K.*?(?=")' | sed 's/\?.*//' | sed 's/t.nhentai/i.nhentai/' | sed 's/t.jpg/.jpg/' | sed 's/t.png/.png/' > "${var}"/links.txt | rm ${kode}.html; 
cd "${var}";
wget -q -nv -i links.txt # option -i $argv if necessary show dat process



#length
termin=$(date +"%s")
difftimelps=$(($termin-$begin))
echo "${WHITE}$(($difftimelps / 60)) minutes and $(($difftimelps % 60)) sec. elapsed ${RED}nhentai stealer"

#283616