#!/bin/bash
#
if [ -e galer.html ]
        then echo "galer.html already exists, please remove it and rerun this script"
        exit
fi
#Create Thumbnails
#
#Check if directory exists and create it if not


##Create HTML page
title=$1
cat >> ./galer.html <<HEAD
<!DOCTYPE html>
<html>
<title>Hensukiviewer | $var</title>
<meta name="viewport" content="width=device-width, initial-scale=1">
<link rel="stylesheet" href="https://www.w3schools.com/w3css/4/w3.css">
<font face=Consolas color=white>
<style>
.mySlides {display:none;}
body {
  background-color: #f7f3f3;
}
.memek {
  margin: auto;
  background-color: #292424;
  width: 50%;
  
  padding: 10px;
  -moz-border-radius: 5px;
	-webkit-border-radius: 30px;
}

  img:hover {
  animation: shake 0.5s;
  animation-iteration-count: infinite;
}

@keyframes shake {
  0% { transform: translate(1px, 1px) rotate(0deg); }
  10% { transform: translate(-1px, -2px) rotate(-1deg); }
  20% { transform: translate(-3px, 0px) rotate(1deg); }
  30% { transform: translate(3px, 2px) rotate(0deg); }
  40% { transform: translate(1px, -1px) rotate(1deg); }
  50% { transform: translate(-1px, 2px) rotate(-1deg); }
  60% { transform: translate(-3px, 1px) rotate(0deg); }
  70% { transform: translate(3px, 1px) rotate(-1deg); }
  80% { transform: translate(-1px, -1px) rotate(1deg); }
  90% { transform: translate(1px, 2px) rotate(0deg); }
  100% { transform: translate(1px, -2px) rotate(-1deg); }
}

</style>
  <div class="memek">


title: $var
<br>
tags: $tag

<body>

<div class="w3-container">
<br>$PWD</p>
</div>

<div class="w3-content w3-display-container">

HEAD

for i in $( seq 1 $count )
do 


cat >> ./galer.html <<HTML
   
      <div class="w3-display-container mySlides">
  <img src="$i.jpg" style="width:100%">
  <div class="w3-display-middle w3-large w3-container w3-padding-16 w3-black">
    $i
  </div>
</div>
    
HTML
done

cat >>./galer.html<<FOOT
<button class="w3-button w3-display-left w3-black" onclick="plusDivs(-1)">&#10094;</button>
<button class="w3-button w3-display-right w3-black" onclick="plusDivs(1)">&#10095;</button>

</div>
</div>

<script>
var slideIndex = 1;
showDivs(slideIndex);

function plusDivs(n) {
  showDivs(slideIndex += n);
}

function showDivs(n) {
  var i;
  var x = document.getElementsByClassName("mySlides");
  if (n > x.length) {slideIndex = 1}
  if (n < 1) {slideIndex = x.length}
  for (i = 0; i < x.length; i++) {
     x[i].style.display = "none";  
  }
  x[slideIndex-1].style.display = "block";  
}
</script>

</body>

</html>

FOOT

echo "gallery > galer.html"